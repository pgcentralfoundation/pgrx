use core::any::TypeId;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum PgExternReturnEntity {
    None,
    Type {
        id: TypeId,
        source: &'static str,
        full_path: &'static str,
        module_path: String,
    },
    SetOf {
        id: TypeId,
        source: &'static str,
        full_path: &'static str,
        module_path: String,
    },
    Iterated(
        Vec<(
            TypeId,
            &'static str,         // Source
            &'static str,         // Full path
            String,               // Module path
            Option<&'static str>, // Name
        )>,
    ),
    Trigger,
}
