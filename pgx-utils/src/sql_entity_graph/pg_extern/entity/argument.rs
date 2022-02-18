use crate::sql_entity_graph::SqlGraphIdentifier;

/// The output of a [`PgExternArgument`](crate::sql_entity_graph::PgExternArgument) from `quote::ToTokens::to_tokens`.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PgExternArgumentEntity {
    pub pattern: &'static str,
    pub ty_source: &'static str,
    pub ty_id: core::any::TypeId,
    pub full_path: &'static str,
    pub module_path: String,
    pub is_optional: bool,
    pub is_variadic: bool,
    pub default: Option<&'static str>,
}

impl SqlGraphIdentifier for PgExternArgumentEntity {
    fn dot_identifier(&self) -> String {
        format!("arg {}", self.full_path)
    }
    fn rust_identifier(&self) -> String {
        self.full_path.to_string()
    }

    fn file(&self) -> Option<&'static str> {
        None
    }

    fn line(&self) -> Option<u32> {
        None
    }
}
