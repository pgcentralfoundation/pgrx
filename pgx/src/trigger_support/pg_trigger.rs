use crate::{
    heap_tuple::PgHeapTuple,
    pg_sys,
    pgbox::{AllocatedByPostgres, PgBox},
    rel::PgRelation,
    trigger_support::{
        called_as_trigger, PgTriggerError, PgTriggerLevel, PgTriggerOperation, PgTriggerSafe,
        PgTriggerWhen, TriggerEvent, TriggerTuple,
    },
};
use cstr_core::c_char;
use std::borrow::Borrow;

/**
The datatype accepted by a trigger

A safe structure providing the an API similar to the constants provided in a PL/pgSQL function.

A "no-op" trigger that gets the current [`PgHeapTuple`][crate::PgHeapTuple],
panicking (into a PostgreSQL error) if it doesn't exist:

```rust,no_run
use pgx::{pg_trigger, pg_sys, PgHeapTuple, WhoAllocated, PgHeapTupleError, PgTrigger};

#[pg_trigger]
fn trigger_example(trigger: &PgTrigger) -> Result<
    PgHeapTuple<'_, impl WhoAllocated<pg_sys::HeapTupleData>>,
    PgHeapTupleError,
> {
    Ok(unsafe { trigger.current() }.expect("No current HeapTuple"))
}
```

Trigger functions only accept one argument, a [`PgTrigger`], and they return a [`Result`][std::result::Result] containing
either a [`PgHeapTuple`][crate::PgHeapTuple] or any error that implements [`impl std::error::Error`][std::error::Error].

## Use from SQL

The `trigger_example` example above would generate something like the following SQL:

```sql
-- pgx-examples/triggers/src/lib.rs:25
-- triggers::trigger_example
CREATE FUNCTION "trigger_example"()
    RETURNS TRIGGER
    LANGUAGE c
    AS 'MODULE_PATHNAME', 'trigger_example_wrapper';
```

Users could then use it like so:

```sql
CREATE TABLE test (
    id serial8 NOT NULL PRIMARY KEY,
    title varchar(50),
    description text,
    payload jsonb
);

CREATE TRIGGER test_trigger
    BEFORE INSERT ON test
    FOR EACH ROW
    EXECUTE PROCEDURE trigger_example();

INSERT INTO test (title, description, payload)
    VALUES ('Fox', 'a description', '{"key": "value"}');
```

This can also be done via the [`extension_sql`][crate::extension_sql] attribute:

```rust,no_run
# use pgx::{pg_trigger, pg_sys, PgHeapTuple, WhoAllocated, PgHeapTupleError, PgTrigger};
#
# #[pg_trigger]
# fn trigger_example(trigger: &PgTrigger) -> Result<
#    PgHeapTuple<'_, impl WhoAllocated<pg_sys::HeapTupleData>>,
#    PgHeapTupleError,
# > {
#     Ok(unsafe { trigger.current() }.expect("No current HeapTuple"))
# }
#
pgx::extension_sql!(
    r#"
CREATE TABLE test (
    id serial8 NOT NULL PRIMARY KEY,
    title varchar(50),
    description text,
    payload jsonb
);

CREATE TRIGGER test_trigger BEFORE INSERT ON test FOR EACH ROW EXECUTE PROCEDURE trigger_example();
INSERT INTO test (title, description, payload) VALUES ('Fox', 'a description', '{"key": "value"}');
"#,
    name = "create_trigger",
    requires = [ trigger_example ]
);
```

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
fn example_custom_error(trigger: &PgTrigger) -> Result<
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
fn example_lifetimes<'a, 'b>(trigger: &'a PgTrigger) -> Result<
    PgHeapTuple<'a, AllocatedByRust>,
    CustomTriggerError<'b>,
> {
    return Err(CustomTriggerError::SomeStr("Oopsie"))
}
```

## Escape hatches

Unsafe [`pgx::pg_sys::FunctionCallInfo`][crate::pg_sys::FunctionCallInfo] and
[`pgx::pg_sys::TriggerData`][crate::pg_sys::TriggerData] (include its contained
[`pgx::pg_sys::Trigger`][crate::pg_sys::Trigger]) accessors are available..


## Getting safe data all at once

Many [`PgTrigger`][PgTrigger] functions are `unsafe` as they dereference pointers inside the
[`TriggerData`][pgx::pg_sys::TriggerData] contained by the [`PgTrigger`][PgTrigger].

In cases where a safe API is desired, the [`PgTriggerSafe`] structure can be retrieved
from [`PgTrigger::to_safe`].

```rust,no_run
use pgx::{pg_trigger, pg_sys, PgHeapTuple, WhoAllocated, PgHeapTupleError, PgTrigger, PgTriggerError};

#[pg_trigger]
fn trigger_safe(trigger: &PgTrigger) -> Result<
    PgHeapTuple<'_, impl WhoAllocated<pg_sys::HeapTupleData>>,
    PgTriggerError,
> {
    let trigger_safe = unsafe { trigger.to_safe() }?;
    Ok(trigger_safe.current.expect("No current HeapTuple"))
}
```

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
            return Err(PgTriggerError::NotTrigger);
        }
        let fcinfo_data = &*fcinfo;

        if fcinfo_data.context.is_null() {
            return Err(PgTriggerError::NullTriggerData);
        }
        let trigger_data: PgBox<pg_sys::TriggerData> =
            PgBox::from_pg(fcinfo_data.context as *mut pg_sys::TriggerData);

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
    /// The trigger event
    pub fn event(&self) -> TriggerEvent {
        TriggerEvent(self.trigger_data.tg_event)
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
    pub unsafe fn old_transition_table_name(&self) -> Result<Option<&str>, PgTriggerError> {
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
    pub unsafe fn new_transition_table_name(&self) -> Result<Option<&str>, PgTriggerError> {
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
        let args = slice
            .into_iter()
            .map(|v| {
                cstr_core::CStr::from_ptr(*v)
                    .to_str()
                    .map(ToString::to_string)
            })
            .collect::<Result<_, core::str::Utf8Error>>()?;
        Ok(args)
    }

    /// Eagerly evaluate the data in this `PgTrigger` and build a safely accessible structure
    /// which mimics the data provided to a PL/pgSQL trigger.
    pub unsafe fn to_safe(&self) -> Result<PgTriggerSafe, PgTriggerError> {
        let trigger_safe = PgTriggerSafe {
            name: self.name()?,
            new: self.new(),
            current: self.current(),
            event: self.event(),
            when: self.when()?,
            level: self.level(),
            op: self.op()?,
            relid: self.relid()?,
            old_transition_table_name: self.old_transition_table_name()?,
            new_transition_table_name: self.new_transition_table_name()?,
            relation: self.relation()?,
            table_name: self.table_name()?,
            table_schema: self.table_schema()?,
            extra_args: self.extra_args()?,
        };

        Ok(trigger_safe)
    }
}
