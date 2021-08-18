mod pgx_sql;
pub use pgx_sql::PgxSql;

mod control_file;
pub use control_file::ControlFile;

mod schema;
pub use schema::InventorySchema;

mod pg_extern;
pub use pg_extern::{InventoryPgExtern, InventoryPgExternReturn, InventoryPgExternInput, InventoryPgOperator};

mod extension_sql;
pub use extension_sql::{InventoryExtensionSql, InventorySqlDeclaredEntity, InventoryExtensionSqlPositioningRef};

mod postgres_enum;
pub use postgres_enum::InventoryPostgresEnum;

mod postgres_type;
pub use postgres_type::InventoryPostgresType;

mod postgres_ord;
pub use postgres_ord::InventoryPostgresOrd;

mod postgres_hash;
pub use postgres_hash::InventoryPostgresHash;

mod sql_graph_entity;
pub use sql_graph_entity::SqlGraphEntity;

use serde::{Serialize, Deserialize};

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
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct RustSqlMapping {
    // This is the **resolved** type, not the raw source. This means a Type Aliase of `type Foo = u32` would appear as `u32`.
    pub rust: String,
    pub sql: String,
    // This is actually the Debug format of a TypeId!
    //
    // This is not a good idea, but without a stable way to create or serialize TypeIds, we have to.
    pub id: String,
}

impl RustSqlMapping {
    pub fn of<T: 'static>(sql: String) -> Self {
        Self {
            rust: core::any::type_name::<T>().to_string(),
            sql: sql.to_string(),
            id: format!("{:?}", core::any::TypeId::of::<T>()),
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
