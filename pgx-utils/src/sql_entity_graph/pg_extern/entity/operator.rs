use serde::{Deserialize, Serialize};

/// The output of a [`PgOperator`](crate::sql_entity_graph::PgOperator) from `quote::ToTokens::to_tokens`.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PgOperatorEntity {
    pub opname: Option<&'static str>,
    pub commutator: Option<&'static str>,
    pub negator: Option<&'static str>,
    pub restrict: Option<&'static str>,
    pub join: Option<&'static str>,
    pub hashes: bool,
    pub merges: bool,
}
