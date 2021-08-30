use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct InventoryPgOperator {
    pub opname: Option<&'static str>,
    pub commutator: Option<&'static str>,
    pub negator: Option<&'static str>,
    pub restrict: Option<&'static str>,
    pub join: Option<&'static str>,
    pub hashes: bool,
    pub merges: bool,
}
