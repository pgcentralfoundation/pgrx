/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

//! Helper functions for working with custom Rust trigger functions

use cstr_core::c_char;

use crate::{PgRelation, is_a, pg_sys, heap_tuple::PgHeapTuple, pgbox::{PgBox, AllocatedByPostgres}};
use std::borrow::Borrow;

/**
The datatype accepted by a trigger

A safe structure providing the an API similar to the constants provided in a PL/pgSQL function.

A "no-op" trigger that gets the current [`PgHeapTuple`][crate::PgHeapTuple],
panicking (into a PostgreSQL error) if it doesn't exist:

```rust,no_run
use pgx::{pg_trigger, pg_sys, PgHeapTuple, WhoAllocated, PgHeapTupleError, PgTrigger};

#[pg_trigger]
fn example_trigger(trigger: &PgTrigger) -> Result<
    PgHeapTuple<'_, impl WhoAllocated<pg_sys::HeapTupleData>>,
    PgHeapTupleError,
> {
    Ok(unsafe { trigger.current() }.expect("No current HeapTuple"))
}
```

Trigger functions only accept one argument, a [`PgTrigger`], and they return a [`Result`][std::result::Result] containing 
either a [`PgHeapTuple`][crate::PgHeapTuple] or any error that implements [`impl std::error::Error`][std::error::Error].

## Working with [WhoAllocated][crate::WhoAllocated]

Trigger functions can return [`PgHeapTuple`][crate::PgHeapTuple]s which are [`AllocatedByRust`][crate::AllocatedByRust]
or [`AllocatedByPostgres`][crate::AllocatedByPostgres]. In most cases, it can be inferred by the compiler using
[`impl WhoAllocated<pg_sys::HeapTupleData>>`][crate::WhoAllocated].

When it can't, the function definition permits for it to be specified:

```rust,no_run
use pgx::{pg_trigger, pg_sys, PgHeapTuple, AllocatedByRust, AllocatedByPostgres, PgHeapTupleError, PgTrigger};

#[pg_trigger]
fn example_allocated_by_rust(trigger: &PgTrigger) -> Result<
    PgHeapTuple<'_, AllocatedByRust>,
    PgHeapTupleError,
> {
    let current = unsafe { trigger.current() }.expect("No current HeapTuple");
    Ok(current.into_owned())
}

#[pg_trigger]
fn example_allocated_by_postgres(trigger: &PgTrigger) -> Result<
    PgHeapTuple<'_, AllocatedByPostgres>,
    PgHeapTupleError,
> {
    let current = unsafe { trigger.current() }.expect("No current HeapTuple");
    Ok(current)
}
```

## Error Handling

Trigger functions can return any [`impl std::error::Error`][std::error::Error]. Returned errors
become PostgreSQL errors.


```rust,no_run
use pgx::{pg_trigger, pg_sys, PgHeapTuple, WhoAllocated, PgHeapTupleError, PgTrigger};

#[derive(thiserror::Error, Debug)]
enum CustomTriggerError {
    #[error("No current HeapTuple")]
    NoCurrentHeapTuple,
    #[error("pgx::PgHeapTupleError: {0}")]
    PgHeapTuple(PgHeapTupleError),
}

#[pg_trigger]
fn example_trigger(trigger: &PgTrigger) -> Result<
    PgHeapTuple<'_, impl WhoAllocated<pg_sys::HeapTupleData>>,
    CustomTriggerError,
> {
    unsafe { trigger.current() }.ok_or(CustomTriggerError::NoCurrentHeapTuple)
}
```

## Lifetimes

Triggers are free to use lifetimes to hone their code, the generated wrapper is as generous as possible.

```rust,no_run
use pgx::{pg_trigger, pg_sys, PgHeapTuple, AllocatedByRust, PgHeapTupleError, PgTrigger};

#[derive(thiserror::Error, Debug)]
enum CustomTriggerError<'a> {
    #[error("No current HeapTuple")]
    NoCurrentHeapTuple,
    #[error("pgx::PgHeapTupleError: {0}")]
    PgHeapTuple(PgHeapTupleError),
    #[error("A borrowed error variant: {0}")]
    SomeStr(&'a str),
}

#[pg_trigger]
fn example_trigger<'a, 'b>(trigger: &'a PgTrigger) -> Result<
    PgHeapTuple<'a, AllocatedByRust>,
    CustomTriggerError<'b>,
> {
    return Err(CustomTriggerError::SomeStr("Oopsie"))
}
```

## Unsafe escape hatches

Unsafe [`pgx::pg_sys::FunctionCallInfo`][crate::pg_sys::FunctionCallInfo] and
[`pgx::pg_sys::TriggerData`][crate::pg_sys::TriggerData] (include its contained
[`pgx::pg_sys::Trigger`][crate::pg_sys::Trigger]) accessors are available..

*/
pub struct PgTrigger {
    trigger_data: PgBox<pgx_pg_sys::TriggerData>,
    #[allow(dead_code)]
    fcinfo: pg_sys::FunctionCallInfo,
}

impl PgTrigger {
    pub unsafe fn from_fcinfo(fcinfo: pg_sys::FunctionCallInfo) -> Result<Self, PgTriggerError> {
        if fcinfo.is_null() {
            return Err(PgTriggerError::NullFunctionCallInfo);
        }
        if !called_as_trigger(fcinfo) {
            return Err(PgTriggerError::NotTrigger)
        }
        let fcinfo_data = &*fcinfo;

        if fcinfo_data.context.is_null() {
            return Err(PgTriggerError::NullTriggerData);
        }
        let trigger_data: PgBox<pg_sys::TriggerData> = PgBox::from_pg(
            fcinfo_data.context as *mut pg_sys::TriggerData,
        );

        Ok(Self {
            trigger_data,
            fcinfo,
        })
    }

    /// A reference to the underlaying trigger data
    pub fn trigger_data(&self) -> &pgx_pg_sys::TriggerData {
        self.trigger_data.borrow()
    }

    /// A reference to the underlaying fcinfo
    pub fn fcinfo(&self) -> &pg_sys::FunctionCallInfo {
        self.fcinfo.borrow()
    }

    /// The new HeapTuple
    // Derived from `pgx_pg_sys::TriggerData.tg_newtuple` and `pgx_pg_sys::TriggerData.tg_newslot.tts_tupleDescriptor`
    pub unsafe fn new(&self) -> Option<PgHeapTuple<'_, AllocatedByPostgres>> {
        PgHeapTuple::from_trigger_data(&*self.trigger_data, TriggerTuple::New)
    }
    /// The current HeapTuple
    // Derived from `pgx_pg_sys::TriggerData.tg_trigtuple` and `pgx_pg_sys::TriggerData.tg_trigslot.tts_tupleDescriptor`
    pub unsafe fn current(&self) -> Option<PgHeapTuple<'_, AllocatedByPostgres>> {
        PgHeapTuple::from_trigger_data(&*self.trigger_data, TriggerTuple::Current)
    }
    /// Variable that contains the name of the trigger actually fired
    pub unsafe fn name(&self) -> Result<&str, PgTriggerError> {
        let trigger_ptr = self.trigger_data.tg_trigger;
        if trigger_ptr.is_null() {
            return Err(PgTriggerError::NullTrigger);
        }
        let trigger = *trigger_ptr;
        let name_ptr = trigger.tgname as *mut c_char;
        let name_cstr = cstr_core::CStr::from_ptr(name_ptr);
        let name_str = name_cstr.to_str()?;
        Ok(name_str)
    }
    /// When the trigger was triggered (`BEFORE`, `AFTER`, `INSTEAD OF`)
    // Derived from `pgx_pg_sys::TriggerData.tg_event`
    pub fn when(&self) -> Result<PgTriggerWhen, PgTriggerError> {
        PgTriggerWhen::try_from(TriggerEvent(self.trigger_data.tg_event))
    }
    /// The level, from the trigger definition (`ROW`, `STATEMENT`)
    // Derived from `pgx_pg_sys::TriggerData.tg_event`
    pub fn level(&self) -> PgTriggerLevel {
        PgTriggerLevel::from(TriggerEvent(self.trigger_data.tg_event))
    }
    /// The operation for which the trigger was fired
    // Derived from `pgx_pg_sys::TriggerData.tg_event`
    pub fn op(&self) -> Result<PgTriggerOperation, PgTriggerError> {
        PgTriggerOperation::try_from(TriggerEvent(self.trigger_data.tg_event))
    }
    /// the object ID of the table that caused the trigger invocation
    // Derived from `pgx_pg_sys::TriggerData.tg_relation.rd_id`
    pub unsafe fn relid(&self) -> Result<pg_sys::Oid, PgTriggerError> {
        let relation_data_ptr = self.trigger_data.tg_relation;
        if relation_data_ptr.is_null() {
            return Err(PgTriggerError::NullRelation);
        }
        let relation_data = *relation_data_ptr;
        Ok(relation_data.rd_id)
    }
    // #[deprecated = "The name of the table that caused the trigger invocation. This is now deprecated, and could disappear in a future release. Use TG_TABLE_NAME instead."]
    // tg_relname: &'a str,

    /// The name of the old transition table of this trigger invocation
    // Derived from `pgx_pg_sys::TriggerData.trigger.tgoldtable`
    pub unsafe fn old_transition_table_name(&self) -> Result<Option<&str>, PgTriggerError>  {
        let trigger_ptr = self.trigger_data.tg_trigger;
        if trigger_ptr.is_null() {
            return Err(PgTriggerError::NullTrigger);
        }
        let trigger = *trigger_ptr;
        let tgoldtable = trigger.tgoldtable;
        if !tgoldtable.is_null() {
            let table_name_cstr = cstr_core::CStr::from_ptr(tgoldtable);
            let table_name_str = table_name_cstr.to_str()?;
            Ok(Some(table_name_str))
        } else {
            Ok(None)
        }
    }
    /// The name of the new transition table of this trigger invocation
    // Derived from `pgx_pg_sys::TriggerData.trigger.tgoldtable`
    pub unsafe fn new_transition_table_name(&self) -> Result<Option<&str>, PgTriggerError>  {
        let trigger_ptr = self.trigger_data.tg_trigger;
        if trigger_ptr.is_null() {
            return Err(PgTriggerError::NullTrigger);
        }
        let trigger = *trigger_ptr;
        let tgnewtable = trigger.tgnewtable;
        if !tgnewtable.is_null() {
            let table_name_cstr = cstr_core::CStr::from_ptr(tgnewtable);
            let table_name_str = table_name_cstr.to_str()?;
            Ok(Some(table_name_str))
        } else {
            Ok(None)
        }
    }
    /// The `PgRelation` corresponding to the trigger.
    pub unsafe fn relation(&self) -> Result<crate::PgRelation, PgTriggerError> {
        let relation_data_ptr = self.trigger_data.tg_relation;
        if relation_data_ptr.is_null() {
            return Err(PgTriggerError::NullRelation);
        }
        let relation_data = *relation_data_ptr;
        Ok(PgRelation::open(relation_data.rd_id))
    }
    /// The name of the schema of the table that caused the trigger invocation
    pub unsafe fn table_name(&self) -> Result<String, PgTriggerError> {
        let relation = self.relation()?;
        Ok(relation.name().to_string())
    }
    /// The name of the schema of the table that caused the trigger invocation
    pub unsafe fn table_schema(&self) -> Result<String, PgTriggerError> {
        let relation = self.relation()?;
        Ok(relation.namespace().to_string())
    }
    /// The arguments from the CREATE TRIGGER statement
    // Derived from `pgx_pg_sys::TriggerData.trigger.tgargs`
    pub unsafe fn extra_args(&self) -> Result<Vec<String>, PgTriggerError> {
        let trigger_ptr = self.trigger_data.tg_trigger;
        if trigger_ptr.is_null() {
            return Err(PgTriggerError::NullTrigger);
        }
        let trigger = *trigger_ptr;
        let tgargs = trigger.tgargs;
        let tgnargs = trigger.tgnargs;
        let slice: &[*mut c_char] = core::slice::from_raw_parts(tgargs, tgnargs.try_into()?);
        let args = slice.into_iter()
            .map(|v| cstr_core::CStr::from_ptr(*v).to_str().map(ToString::to_string))
            .collect::<Result<_, core::str::Utf8Error>>()?;
        Ok(args)
    }
}

/// A newtype'd wrapper around a `pg_sys::TriggerData.tg_event` to prevent accidental misuse
#[derive(Debug)]
pub struct TriggerEvent(u32);

/// When a trigger happened
/// 
/// Maps from a `TEXT` of `BEFORE`, `AFTER`, or `INSTEAD OF`.
/// 
/// Can be calculated from a `pgx_pg_sys::TriggerEvent`.
// Postgres constants: https://cs.github.com/postgres/postgres/blob/36d4efe779bfc7190ea1c1cf8deb0d945b726663/src/include/commands/trigger.h?q=TRIGGER_FIRED_BEFORE#L100-L102
// Postgres defines: https://cs.github.com/postgres/postgres/blob/36d4efe779bfc7190ea1c1cf8deb0d945b726663/src/include/commands/trigger.h?q=TRIGGER_FIRED_BEFORE#L128-L135
pub enum PgTriggerWhen {
    /// `BEFORE`
    Before,
    /// `AFTER`
    After,
    /// `INSTEAD OF`
    InsteadOf,
}

impl TryFrom<TriggerEvent> for PgTriggerWhen {
    type Error = PgTriggerError;
    fn try_from(event: TriggerEvent) -> Result<Self, Self::Error> {
        match event.0 & pg_sys::TRIGGER_EVENT_TIMINGMASK {
            pg_sys::TRIGGER_EVENT_BEFORE => Ok(Self::Before),
            pg_sys::TRIGGER_EVENT_AFTER => Ok(Self::After),
            pg_sys::TRIGGER_EVENT_INSTEAD => Ok(Self::InsteadOf),
            v => Err(PgTriggerError::InvalidPgTriggerWhen(v))
        }
    }
}

impl ToString for PgTriggerWhen {
    fn to_string(&self) -> String {
        match self {
            PgTriggerWhen::Before => "BEFORE",
            PgTriggerWhen::After => "AFTER",
            PgTriggerWhen::InsteadOf => "INSTEAD OF",
        }.to_string()
    }
}

/// The level of a trigger
/// 
/// Maps from a `TEXT` of `ROW` or `STATEMENT`.
/// 
/// Can be calculated from a `pgx_pg_sys::TriggerEvent`.
// Postgres constants: https://cs.github.com/postgres/postgres/blob/36d4efe779bfc7190ea1c1cf8deb0d945b726663/src/include/commands/trigger.h?q=TRIGGER_FIRED_BEFORE#L98
// Postgres defines: https://cs.github.com/postgres/postgres/blob/36d4efe779bfc7190ea1c1cf8deb0d945b726663/src/include/commands/trigger.h?q=TRIGGER_FIRED_BEFORE#L122-L126
pub enum PgTriggerLevel {
    /// `ROW`
    Row,
    /// `STATEMENT`
    Statement,
}

impl From<TriggerEvent> for PgTriggerLevel {
    fn from(event: TriggerEvent) -> Self {
        match event.0 & pg_sys::TRIGGER_EVENT_ROW {
            0 => PgTriggerLevel::Statement,
            _ => PgTriggerLevel::Row,
        }
    }
}

impl ToString for PgTriggerLevel {
    fn to_string(&self) -> String {
        match self {
            PgTriggerLevel::Statement => "STATEMENT",
            PgTriggerLevel::Row => "ROW",
        }.to_string() 
    }
}

/// The operation for which the trigger was fired
/// 
/// Maps from a `TEXT` of `INSERT`, `UPDATE`, `DELETE`, or `TRUNCATE`.
///
/// Can be calculated from a `pgx_pg_sys::TriggerEvent`.
// Postgres constants: https://cs.github.com/postgres/postgres/blob/36d4efe779bfc7190ea1c1cf8deb0d945b726663/src/include/commands/trigger.h#L92
// Postgres defines: https://cs.github.com/postgres/postgres/blob/36d4efe779bfc7190ea1c1cf8deb0d945b726663/src/include/commands/trigger.h#L92
pub enum PgTriggerOperation {
    /// `INSERT`
    Insert,
    /// `UPDATE`
    Update,
    /// `DELETE`
    Delete,
    /// `TRUNCATE`
    Truncate,
}

impl TryFrom<TriggerEvent> for PgTriggerOperation {
    type Error = PgTriggerError;
    fn try_from(event: TriggerEvent) -> Result<Self, Self::Error> {
        match event.0 & pg_sys::TRIGGER_EVENT_OPMASK {
            pg_sys::TRIGGER_EVENT_INSERT => Ok(Self::Insert),
            pg_sys::TRIGGER_EVENT_DELETE => Ok(Self::Delete),
            pg_sys::TRIGGER_EVENT_UPDATE => Ok(Self::Update),
            pg_sys::TRIGGER_EVENT_TRUNCATE => Ok(Self::Truncate),
            v => Err(PgTriggerError::InvalidPgTriggerOperation(v))
        }
    }
}

impl ToString for PgTriggerOperation {
    fn to_string(&self) -> String {
        match self {
            PgTriggerOperation::Insert => "INSERT",
            PgTriggerOperation::Update => "UPDATE",
            PgTriggerOperation::Delete => "DELETE",
            PgTriggerOperation::Truncate => "TRUNCATE",
        }.to_string() 
    }
}

#[derive(thiserror::Error, Debug, Clone, Copy)]
pub enum PgTriggerError {
    #[error("`PgTrigger`s can only be built from `FunctionCallInfo` instances which `pgx::pg_sys::called_as_trigger(fcinfo)` returns `true`")]
    NotTrigger,
    #[error("`PgTrigger`s cannot be built from `NULL` `pgx::pg_sys::FunctionCallInfo`s")]
    NullFunctionCallInfo,
    #[error("`InvalidPgTriggerWhen` cannot be built from `event & TRIGGER_EVENT_TIMINGMASK` of `{0}")]
    InvalidPgTriggerWhen(u32),
    #[error("`InvalidPgTriggerOperation` cannot be built from `event & TRIGGER_EVENT_OPMASK` of `{0}")]
    InvalidPgTriggerOperation(u32),
    #[error("core::str::Utf8Error: {0}")]
    CoreUtf8(#[from] core::str::Utf8Error),
    #[error("TryFromIntError: {0}")]
    TryFromInt(#[from] core::num::TryFromIntError),
    #[error("The `pgx::pg_sys::TriggerData`'s `tg_trigger` field was a NULL pointer")]
    NullTrigger,
    #[error("The `pgx::pg_sys::FunctionCallInfo`'s `context` field was a NULL pointer")]
    NullTriggerData,
    #[error("The `pgx::pg_sys::TriggerData`'s `tg_relation` field was a NULL pointer")]
    NullRelation,
}

/// Indicates which trigger tuple to convert into a [crate::PgHeapTuple].
#[derive(Debug, Copy, Clone)]
pub enum TriggerTuple {
    /// The row for which the trigger was fired. This is the row being inserted, updated, or deleted.
    /// If this trigger was fired for an INSERT or DELETE then this is what you should return from the
    /// function if you don't want to replace the row with a different one (in the case of INSERT) or
    /// skip the operation.
    Current,

    /// The new version of the row, if the trigger was fired for an UPDATE. This is what you have to
    /// return from the function if the event is an UPDATE and you don't want to replace this row by
    /// a different one or skip the operation.
    New,
}

#[inline]
pub unsafe fn called_as_trigger(fcinfo: pg_sys::FunctionCallInfo) -> bool {
    let fcinfo = fcinfo.as_ref().expect("fcinfo was null");
    !fcinfo.context.is_null() && is_a(fcinfo.context, pg_sys::NodeTag_T_TriggerData)
}

#[inline]
pub fn trigger_fired_by_insert(event: u32) -> bool {
    event & pg_sys::TRIGGER_EVENT_OPMASK == pg_sys::TRIGGER_EVENT_INSERT
}

#[inline]
pub fn trigger_fired_by_delete(event: u32) -> bool {
    event & pg_sys::TRIGGER_EVENT_OPMASK == pg_sys::TRIGGER_EVENT_DELETE
}

#[inline]
pub fn trigger_fired_by_update(event: u32) -> bool {
    event & pg_sys::TRIGGER_EVENT_OPMASK == pg_sys::TRIGGER_EVENT_UPDATE
}

#[inline]
pub fn trigger_fired_by_truncate(event: u32) -> bool {
    event & pg_sys::TRIGGER_EVENT_OPMASK == pg_sys::TRIGGER_EVENT_TRUNCATE
}

#[inline]
pub fn trigger_fired_for_row(event: u32) -> bool {
    event & pg_sys::TRIGGER_EVENT_ROW != 0
}

#[inline]
pub fn trigger_fired_for_statement(event: u32) -> bool {
    !trigger_fired_for_row(event)
}

#[inline]
pub fn trigger_fired_before(event: u32) -> bool {
    event & pg_sys::TRIGGER_EVENT_TIMINGMASK == pg_sys::TRIGGER_EVENT_BEFORE
}

#[inline]
pub fn trigger_fired_after(event: u32) -> bool {
    event & pg_sys::TRIGGER_EVENT_TIMINGMASK == pg_sys::TRIGGER_EVENT_AFTER
}

#[inline]
pub fn trigger_fired_instead(event: u32) -> bool {
    event & pg_sys::TRIGGER_EVENT_TIMINGMASK == pg_sys::TRIGGER_EVENT_INSTEAD
}
