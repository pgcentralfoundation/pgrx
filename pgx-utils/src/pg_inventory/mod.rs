mod control_file;
mod extension_sql;
mod pg_extern;
mod pg_schema;
mod pgx_sql;
mod postgres_enum;
mod postgres_hash;
mod postgres_ord;
mod postgres_type;
mod sql_graph_entity;

pub use control_file::{ControlFile, ControlFileError};
pub use extension_sql::{
    ExtensionSql, ExtensionSqlFile, InventoryExtensionSql, InventoryExtensionSqlPositioningRef,
};
pub use pg_extern::{
    InventoryPgExtern, InventoryPgExternInput, InventoryPgExternReturn, InventoryPgOperator,
    PgExtern,
};
pub use pg_schema::{InventorySchema, Schema};
pub use pgx_sql::PgxSql;
pub use postgres_enum::{InventoryPostgresEnum, PostgresEnum};
pub use postgres_hash::{InventoryPostgresHash, PostgresHash};
pub use postgres_ord::{InventoryPostgresOrd, PostgresOrd};
pub use postgres_type::{InventoryPostgresType, PostgresType};
pub use sql_graph_entity::SqlGraphEntity;

// Reexports for the pgx extension inventory builders.
#[doc(hidden)]
pub use color_eyre;
#[doc(hidden)]
pub use eyre;
#[doc(hidden)]
pub use inventory;
#[doc(hidden)]
pub use once_cell;
#[doc(hidden)]
pub use tracing;
#[doc(hidden)]
pub use tracing_error;
#[doc(hidden)]
pub use tracing_subscriber;

use core::{any::TypeId, fmt::Debug};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RustSqlMapping {
    pub rust: String,
    pub sql: String,
    pub id: TypeId,
}

/// Able to produce a GraphViz DOT format identifier.
pub trait DotIdentifier {
    /// An identifier for the entity.
    ///
    /// Typically this is the result of [`std::module_path`], [`core::any::type_name`],
    /// or some combination of [`std::file`] and [`std::line`].
    fn dot_identifier(&self) -> String;
}

/// Able to be transformed into to SQL.
pub trait ToSql {
    /// Attempt to transform this type into SQL.
    ///
    /// Some entities require additional context from a [`PgxSql`], such as
    /// `#[derive(PostgresType)]` which must include it's relevant in/out functions.
    fn to_sql(&self, context: &PgxSql) -> eyre::Result<String>;
}
