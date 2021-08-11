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
    InventorySqlDeclaredEntity, SqlDeclaredEntity,
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
pub use super::ExternArgs;

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

/// A mapping from a Rust type to a SQL type, with a `TypeId`.
///
/// ```rust
/// use pgx_utils::pg_inventory::RustSqlMapping;
///
/// let constructed = RustSqlMapping::of::<i32>(String::from("int"));
/// let raw = RustSqlMapping {
///     rust: core::any::type_name::<i32>().to_string(),
///     sql: String::from("int"),
///     id: core::any::TypeId::of::<i32>(),
/// };
///
/// assert_eq!(constructed, raw);
/// ```
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RustSqlMapping {
    // This is the **resolved** type, not the raw source. This means a Type Aliase of `type Foo = u32` would appear as `u32`.
    pub rust: String,
    pub sql: String,
    pub id: TypeId,
}

impl RustSqlMapping {
    pub fn of<T: 'static>(sql: String) -> Self {
        Self {
            rust: core::any::type_name::<T>().to_string(),
            sql: sql.to_string(),
            id: core::any::TypeId::of::<T>(),
        }
    }
}

/// A mapping from a Rust source fragment to a SQL type, typically for type aliases.
///
/// In general, this can only offer a fuzzy matching, as it does not use [`core::any::TypeId`].
///
/// ```rust
/// use pgx_utils::pg_inventory::RustSourceOnlySqlMapping;
///
/// let constructed = RustSourceOnlySqlMapping::new(
///     String::from("Oid"),
///     String::from("int"),
/// );
/// ```
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RustSourceOnlySqlMapping {
    pub rust: String,
    pub sql: String
}

impl RustSourceOnlySqlMapping {
    pub fn new(rust: String, sql: String) -> Self {
        Self {
            rust: rust.to_string(),
            sql: sql.to_string(),
        }
    }
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
