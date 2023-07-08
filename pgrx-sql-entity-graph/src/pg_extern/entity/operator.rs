/*!

`#[pg_extern]` related operator entities for Rust to SQL translation

> Like all of the [`sql_entity_graph`][crate::pgrx_sql_entity_graph] APIs, this is considered **internal**
to the `pgrx` framework and very subject to change between versions. While you may use this, please do it with caution.

*/

/// The output of a [`PgOperator`](crate::PgOperator) from `quote::ToTokens::to_tokens`.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PgOperatorEntity {
    pub opname: Option<&'static str>,
    pub commutator: Option<&'static str>,
    pub negator: Option<&'static str>,
    pub restrict: Option<&'static str>,
    pub join: Option<&'static str>,
    pub hashes: bool,
    pub merges: bool,
}
