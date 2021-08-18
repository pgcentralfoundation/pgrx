use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct InventoryPgExternInput {
    pub pattern: &'static str,
    pub ty_source: &'static str,
    pub ty_id: String, // This is the Debug output of a core::any::TypeId
    pub full_path: &'static str,
    pub module_path: String,
    pub is_optional: bool,
    pub is_variadic: bool,
    pub default: Option<&'static str>,
}
