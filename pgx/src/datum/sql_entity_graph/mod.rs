mod pgx_sql;
pub use pgx_sql::PgxSql;

mod control_file;
pub use control_file::ControlFile;

mod schema;
pub use schema::SchemaEntity;

mod pg_extern;
pub use pg_extern::{
    PgExternArgumentEntity, PgExternEntity, PgExternReturnEntity, PgOperatorEntity,
};

mod extension_sql;
pub use extension_sql::{ExtensionSqlEntity, SqlDeclaredEntity};

mod postgres_enum;
pub use postgres_enum::PostgresEnumEntity;

mod postgres_type;
pub use postgres_type::PostgresTypeEntity;

mod postgres_ord;
pub use postgres_ord::PostgresOrdEntity;

mod postgres_hash;
pub use postgres_hash::PostgresHashEntity;

mod sql_graph_entity;
pub use sql_graph_entity::SqlGraphEntity;

use core::any::TypeId;
pub use pgx_utils::sql_entity_graph::*;

/// Able to produce a GraphViz DOT format identifier.
pub trait SqlGraphIdentifier {
    /// A dot style identifier for the entity.
    ///
    /// Typically this is a 'archetype' prefix (eg `fn` or `type`) then result of
    /// [`std::module_path`], [`core::any::type_name`], or some combination of [`std::file`] and
    /// [`std::line`].
    fn dot_identifier(&self) -> String;

    /// A Rust identifier for the entity.
    ///
    /// Typically this is the result of [`std::module_path`], [`core::any::type_name`],
    /// or some combination of [`std::file`] and [`std::line`].
    fn rust_identifier(&self) -> String;

    fn file(&self) -> Option<&'static str>;

    fn line(&self) -> Option<u32>;
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
/// use pgx::datum::sql_entity_graph::RustSqlMapping;
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
/// use pgx::datum::sql_entity_graph::RustSourceOnlySqlMapping;
///
/// let constructed = RustSourceOnlySqlMapping::new(
///     String::from("Oid"),
///     String::from("int"),
/// );
/// ```
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct RustSourceOnlySqlMapping {
    pub rust: String,
    pub sql: String,
}

impl RustSourceOnlySqlMapping {
    pub fn new(rust: String, sql: String) -> Self {
        Self {
            rust: rust.to_string(),
            sql: sql.to_string(),
        }
    }
}
