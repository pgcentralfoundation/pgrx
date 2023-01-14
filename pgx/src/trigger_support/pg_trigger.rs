use crate::heap_tuple::PgHeapTuple;
use crate::pg_sys;
use crate::pgbox::{AllocatedByPostgres, PgBox};
use crate::rel::PgRelation;
use crate::trigger_support::{
    called_as_trigger, PgTriggerError, PgTriggerLevel, PgTriggerOperation, PgTriggerSafe,
    PgTriggerWhen, TriggerEvent, TriggerTuple,
};
use cstr_core::c_char;
use std::borrow::Borrow;

/**
The datatype accepted by a trigger

A safe structure providing the an API similar to the constants provided in a PL/pgSQL function.

Usage examples exist in the module level docs.
*/
pub struct PgTrigger {
    trigger: pg_sys::Trigger,
    trigger_data: PgBox<pgx_pg_sys::TriggerData>,
    relation_data: pg_sys::RelationData,
    #[allow(dead_code)]
    fcinfo: pg_sys::FunctionCallInfo,
}

impl PgTrigger {
    /// Construct a new [`PgTrigger`] from a [`FunctionCallInfo`][pg_sys::FunctionCallInfo]
    ///
    /// Generally this would be automatically done for the user in a [`#[pg_trigger]`][crate::pg_trigger].
    ///
    /// # Safety
    ///
    /// This constructor attempts to do some checks for validity, but it is ultimately unsafe
    /// because it must dereference several raw pointers.
    ///
    /// Users should ensure the provided `fcinfo` is:
    ///
    /// * one provided by PostgreSQL during a trigger invocation,
    /// * unharmed (the user has not mutated it since PostgreSQL provided it),
    ///
    /// If any of these conditions are untrue, this or any other function on this type is
    /// undefined behavior, hopefully panicking.
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

        let trigger_ptr = trigger_data.tg_trigger;
        if trigger_ptr.is_null() {
            return Err(PgTriggerError::NullTrigger);
        }
        let trigger = *trigger_ptr;

        let relation_data_ptr = trigger_data.tg_relation;
        if relation_data_ptr.is_null() {
            return Err(PgTriggerError::NullRelation);
        }
        let relation_data = *relation_data_ptr;

        Ok(Self { relation_data, trigger, trigger_data, fcinfo })
    }

    /// The new HeapTuple
    // Derived from `pgx_pg_sys::TriggerData.tg_newtuple` and `pgx_pg_sys::TriggerData.tg_newslot.tts_tupleDescriptor`
    pub fn new(&self) -> Option<PgHeapTuple<'_, AllocatedByPostgres>> {
        // Safety: Given that we have a known good `FunctionCallInfo`, which PostgreSQL has checked is indeed a trigger,
        // containing a known good `TriggerData` which also contains a known good `Trigger`... and the user agreed to
        // our `unsafe` constructor safety rules, we choose to trust this is indeed a valid pointer offered to us by
        // PostgreSQL, and that it trusts it.
        unsafe { PgHeapTuple::from_trigger_data(&*self.trigger_data, TriggerTuple::New) }
    }
    /// The current HeapTuple
    // Derived from `pgx_pg_sys::TriggerData.tg_trigtuple` and `pgx_pg_sys::TriggerData.tg_trigslot.tts_tupleDescriptor`
    pub fn current(&self) -> Option<PgHeapTuple<'_, AllocatedByPostgres>> {
        // Safety: Given that we have a known good `FunctionCallInfo`, which PostgreSQL has checked is indeed a trigger,
        // containing a known good `TriggerData` which also contains a known good `Trigger`... and the user agreed to
        // our `unsafe` constructor safety rules, we choose to trust this is indeed a valid pointer offered to us by
        // PostgreSQL, and that it trusts it.
        unsafe { PgHeapTuple::from_trigger_data(&*self.trigger_data, TriggerTuple::Current) }
    }
    /// Variable that contains the name of the trigger actually fired
    pub fn name(&self) -> Result<&str, PgTriggerError> {
        let name_ptr = self.trigger.tgname as *mut c_char;
        // Safety: Given that we have a known good `FunctionCallInfo`, which PostgreSQL has checked is indeed a trigger,
        // containing a known good `TriggerData` which also contains a known good `Trigger`... and the user agreed to
        // our `unsafe` constructor safety rules, we choose to trust this is indeed a valid pointer offered to us by
        // PostgreSQL, and that it trusts it.
        let name_cstr = unsafe { cstr_core::CStr::from_ptr(name_ptr) };
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
    pub fn relid(&self) -> Result<pg_sys::Oid, PgTriggerError> {
        Ok(self.relation_data.rd_id)
    }
    // #[deprecated = "The name of the table that caused the trigger invocation. This is now deprecated, and could disappear in a future release. Use TG_TABLE_NAME instead."]
    // tg_relname: &'a str,

    /// The name of the old transition table of this trigger invocation
    // Derived from `pgx_pg_sys::TriggerData.trigger.tgoldtable`
    pub fn old_transition_table_name(&self) -> Result<Option<&str>, PgTriggerError> {
        let tgoldtable = self.trigger.tgoldtable;
        if !tgoldtable.is_null() {
            // Safety: Given that we have a known good `FunctionCallInfo`, which PostgreSQL has checked is indeed a trigger,
            // containing a known good `TriggerData` which also contains a known good `Trigger`... and the user agreed to
            // our `unsafe` constructor safety rules, we choose to trust this is indeed a valid pointer offered to us by
            // PostgreSQL, and that it trusts it.
            let table_name_cstr = unsafe { cstr_core::CStr::from_ptr(tgoldtable) };
            let table_name_str = table_name_cstr.to_str()?;
            Ok(Some(table_name_str))
        } else {
            Ok(None)
        }
    }
    /// The name of the new transition table of this trigger invocation
    // Derived from `pgx_pg_sys::TriggerData.trigger.tgoldtable`
    pub fn new_transition_table_name(&self) -> Result<Option<&str>, PgTriggerError> {
        let tgnewtable = self.trigger.tgnewtable;
        if !tgnewtable.is_null() {
            // Safety: Given that we have a known good `FunctionCallInfo`, which PostgreSQL has checked is indeed a trigger,
            // containing a known good `TriggerData` which also contains a known good `Trigger`... and the user agreed to
            // our `unsafe` constructor safety rules, we choose to trust this is indeed a valid pointer offered to us by
            // PostgreSQL, and that it trusts it.
            let table_name_cstr = unsafe { cstr_core::CStr::from_ptr(tgnewtable) };
            let table_name_str = table_name_cstr.to_str()?;
            Ok(Some(table_name_str))
        } else {
            Ok(None)
        }
    }
    /// The `PgRelation` corresponding to the trigger.
    ///
    /// # Panics
    ///
    /// If the relation was recently deleted, this function will panic.
    ///
    /// # Safety
    ///
    /// The caller should already have at least AccessShareLock on the relation ID, else there are nasty race conditions.
    ///
    /// As such, this function is unsafe as we cannot guarantee that this requirement is true.
    pub unsafe fn relation(&self) -> Result<crate::PgRelation, PgTriggerError> {
        let relation = PgRelation::open(self.relation_data.rd_id);
        Ok(relation)
    }
    /// The name of the schema of the table that caused the trigger invocation
    ///
    /// # Panics
    ///
    /// If the relation was recently deleted, this function will panic.
    ///
    /// # Safety
    ///
    /// The caller should already have at least AccessShareLock on the relation ID, else there are nasty race conditions.
    ///
    /// As such, this function is unsafe as we cannot guarantee that this requirement is true.
    pub unsafe fn table_name(&self) -> Result<String, PgTriggerError> {
        let relation = self.relation()?;
        Ok(relation.name().to_string())
    }
    /// The name of the schema of the table that caused the trigger invocation
    ///
    /// # Panics
    ///
    /// If the relation was recently deleted, this function will panic.
    ///
    /// # Safety
    ///
    /// The caller should already have at least AccessShareLock on the relation ID, else there are nasty race conditions.
    ///
    /// As such, this function is unsafe as we cannot guarantee that this requirement is true.
    pub unsafe fn table_schema(&self) -> Result<String, PgTriggerError> {
        let relation = self.relation()?;
        Ok(relation.namespace().to_string())
    }
    /// The arguments from the CREATE TRIGGER statement
    // Derived from `pgx_pg_sys::TriggerData.trigger.tgargs`
    pub fn extra_args(&self) -> Result<Vec<String>, PgTriggerError> {
        let tgargs = self.trigger.tgargs;
        let tgnargs = self.trigger.tgnargs;
        // Safety: Given that we have a known good `FunctionCallInfo`, which PostgreSQL has checked is indeed a trigger,
        // containing a known good `TriggerData` which also contains a known good `Trigger`... and the user agreed to
        // our `unsafe` constructor safety rules, we choose to trust this is indeed a valid pointer offered to us by
        // PostgreSQL, and that it trusts it.
        let slice: &[*mut c_char] =
            unsafe { core::slice::from_raw_parts(tgargs, tgnargs.try_into()?) };
        let args = slice
            .into_iter()
            .map(|v| {
                // Safety: Given that we have a known good `FunctionCallInfo`, which PostgreSQL has checked is indeed a trigger,
                // containing a known good `TriggerData` which also contains a known good `Trigger`... and the user agreed to
                // our `unsafe` constructor safety rules, we choose to trust this is indeed a valid pointer offered to us by
                // PostgreSQL, and that it trusts it.
                unsafe { cstr_core::CStr::from_ptr(*v) }.to_str().map(ToString::to_string)
            })
            .collect::<Result<_, core::str::Utf8Error>>()?;
        Ok(args)
    }

    /// A reference to the underlying [`RelationData`][pgx_pg_sys::RelationData]
    pub fn relation_data(&self) -> &pgx_pg_sys::RelationData {
        self.relation_data.borrow()
    }

    /// A reference to the underlying [`Trigger`][pgx_pg_sys::Trigger]
    pub fn trigger(&self) -> &pgx_pg_sys::Trigger {
        self.trigger.borrow()
    }

    /// A reference to the underlying [`TriggerData`][pgx_pg_sys::TriggerData]
    pub fn trigger_data(&self) -> &pgx_pg_sys::TriggerData {
        self.trigger_data.borrow()
    }

    /// A reference to the underlying fcinfo
    pub fn fcinfo(&self) -> &pg_sys::FunctionCallInfo {
        self.fcinfo.borrow()
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
