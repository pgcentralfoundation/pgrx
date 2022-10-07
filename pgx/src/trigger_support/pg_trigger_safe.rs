use crate::heap_tuple::PgHeapTuple;
use crate::pg_sys;
use crate::pgbox::AllocatedByPostgres;
use crate::trigger_support::{PgTriggerLevel, PgTriggerOperation, PgTriggerWhen, TriggerEvent};

pub struct PgTriggerSafe<'a> {
    pub name: &'a str,
    pub new: Option<PgHeapTuple<'a, AllocatedByPostgres>>,
    pub current: Option<PgHeapTuple<'a, AllocatedByPostgres>>,
    pub event: TriggerEvent,
    pub when: PgTriggerWhen,
    pub level: PgTriggerLevel,
    pub op: PgTriggerOperation,
    pub relid: pg_sys::Oid,
    pub old_transition_table_name: Option<&'a str>,
    pub new_transition_table_name: Option<&'a str>,
    pub relation: crate::PgRelation,
    pub table_name: String,
    pub table_schema: String,
    pub extra_args: Vec<String>,
}
