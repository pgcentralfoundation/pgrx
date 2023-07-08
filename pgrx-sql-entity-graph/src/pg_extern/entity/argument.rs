/*!

`#[pg_extern]` related argument entities for Rust to SQL translation

> Like all of the [`sql_entity_graph`][crate::pgrx_sql_entity_graph] APIs, this is considered **internal**
to the `pgrx` framework and very subject to change between versions. While you may use this, please do it with caution.

*/
use crate::{SqlGraphIdentifier, UsedTypeEntity};

/// The output of a [`PgExternArgument`](crate::PgExternArgument) from `quote::ToTokens::to_tokens`.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PgExternArgumentEntity {
    pub pattern: &'static str,
    pub used_ty: UsedTypeEntity,
}

impl SqlGraphIdentifier for PgExternArgumentEntity {
    fn dot_identifier(&self) -> String {
        format!("arg {}", self.rust_identifier())
    }
    fn rust_identifier(&self) -> String {
        self.used_ty.full_path.to_string()
    }

    fn file(&self) -> Option<&'static str> {
        None
    }

    fn line(&self) -> Option<u32> {
        None
    }
}
