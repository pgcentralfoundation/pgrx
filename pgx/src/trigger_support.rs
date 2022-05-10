/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

//! Helper functions for working with custom Rust trigger functions

use cstr_core::c_char;

use crate::{is_a, pg_sys, heap_tuple::{PgHeapTuple, PgHeapTupleError}, pgbox::{PgBox, AllocatedByPostgres}};

/// The datatype accepted by a trigger
/// 
/// A safe structure providing the an API similar to the constants provided in a PL/pgSQL function. The unsafe [`pgx::pg_sys::TriggerData`] (and it's contained [`pgx::pg_sys::Trigger`]) is available in the `trigger_data` field.
pub struct PgTrigger<'a> {
    trigger_data: PgBox<pgx_pg_sys::TriggerData>,
    #[allow(dead_code)]
    fcinfo: &'a pg_sys::FunctionCallInfo,
}

impl<'a> PgTrigger<'a> {
    pub unsafe fn from_fcinfo(fcinfo: &'a pg_sys::FunctionCallInfo) -> Result<Self, PgTriggerError> {
        if !called_as_trigger(*fcinfo) {
            return Err(PgTriggerError::NotTrigger)
        }
        let fcinfo_ref = (*fcinfo).as_ref().ok_or(PgTriggerError::NullFunctionCallInfo)?;

        let trigger_data: PgBox<pg_sys::TriggerData> = PgBox::from_pg(
            fcinfo_ref.context as *mut pg_sys::TriggerData,
        );

        Ok(Self {
            trigger_data,
            fcinfo,
        })
    }

    /// Variable holding the new database row for INSERT/UPDATE operations in row-level triggers. This variable is `None` in statement-level triggers and for DELETE operations
    // Derived from `pgx_pg_sys::TriggerData.tg_newtuple` and `pgx_pg_sys::TriggerData.tg_newslot.tts_tupleDescriptor`
    pub unsafe fn new(&'a self) -> Result<Option<PgHeapTuple<'a, AllocatedByPostgres>>, PgHeapTupleError> {
        PgHeapTuple::from_trigger_data(&*self.trigger_data, TriggerTuple::New)
    }
    /// Variable holding the old database row for UPDATE/DELETE operations in row-level triggers. This variable is `None` in statement-level triggers and for INSERT operations
    // Derived from `pgx_pg_sys::TriggerData.tg_trigtuple` and `pgx_pg_sys::TriggerData.tg_trigslot.tts_tupleDescriptor`
    pub unsafe fn old(&'a self) -> Result<Option<PgHeapTuple<'a, AllocatedByPostgres>>, PgHeapTupleError> {
        PgHeapTuple::from_trigger_data(&*self.trigger_data, TriggerTuple::Current)
    }
    /// Variable that contains the name of the trigger actually fired
    /// 
    // TODO: This maybe should be `unsafe`...
    pub unsafe fn name(&self) -> Result<&str, PgTriggerError> {
        let name_ptr = (*self.trigger_data.tg_trigger).tgname as *mut c_char;
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
    pub unsafe fn relid(&self) -> pg_sys::Oid {
        (*self.trigger_data.tg_relation).rd_id
    }
    // #[deprecated = "The name of the table that caused the trigger invocation. This is now deprecated, and could disappear in a future release. Use TG_TABLE_NAME instead."]
    // tg_relname: &'a str,
    /// The name of the table that caused the trigger invocation
    // Derived from `pgx_pg_sys::TriggerData.trigger.tgoldtable`
    pub fn table_name(&self) -> &str {
        unimplemented!() // TODO
    }
    /// The name of the schema of the table that caused the trigger invocation
    // TODO: Derived from ????
    pub fn table_schema(&self) -> &str {
        unimplemented!() // TODO
    }
    /// The arguments from the CREATE TRIGGER statement
    // Derived from `pgx_pg_sys::TriggerData.trigger.tgargs`
    pub fn extra_args(&self) -> &[&str] {
        unimplemented!() // TODO
    }

    // TODO: What about `pgx_pg_sys::TriggerData.trigger.tgnewtable`?!
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
    #[error("Utf8Error: {0}")]
    Utf8(#[from] core::str::Utf8Error),
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
