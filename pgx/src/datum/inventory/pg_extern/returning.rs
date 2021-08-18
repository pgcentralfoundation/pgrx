use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum InventoryPgExternReturn {
    None,
    Type {
        id: String, // This is the Debug output of a core::any::TypeId
        source: &'static str,
        full_path: &'static str,
        module_path: String,
    },
    SetOf {
        id: String, // This is the Debug output of a core::any::TypeId
        source: &'static str,
        full_path: &'static str,
        module_path: String,
    },
    Iterated(
        Vec<(
            String, // This is the Debug output of a core::any::TypeId
            &'static str, // Source
            &'static str, // Full path
            String, // Module path
            Option<&'static str>, // Name
        )>,
    ),
    Trigger,
}
