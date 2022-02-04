mod pgx_sql;
pub use pgx_sql::PgxSql;

pub mod aggregate;

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

/// The signature of a function that can transform a SqlGraphEntity to a SQL string
///
/// This is used to provide a facility for overriding the default SQL generator behavior using
/// the `#[to_sql(path::to::function)]` attribute in circumstances where the default behavior is
/// not desirable.
///
/// Implementations can invoke `ToSql::to_sql(entity, context)` on the unwrapped SqlGraphEntity
/// type should they wish to delegate to the default behavior for any reason.
pub type ToSqlFn =
    fn(
        &SqlGraphEntity,
        &PgxSql,
    ) -> std::result::Result<String, Box<dyn std::error::Error + Send + Sync + 'static>>;

/// Represents configuration options for tuning the SQL generator.
///
/// When an item that can be rendered to SQL has these options at hand, they should be
/// respected. If an item does not have them, then it is not expected that the SQL generation
/// for those items can be modified.
///
/// The default configuration has `enabled` set to `true`, and `callback` to `None`, which indicates
/// that the default SQL generation behavior will be used. These are intended to be mutually exclusive
/// options, so `callback` should only be set if generation is enabled.
///
/// When `enabled` is false, no SQL is generated for the item being configured.
///
/// When `callback` has a value, the corresponding `ToSql` implementation should invoke the
/// callback instead of performing their default behavior.
#[derive(Default, Clone)]
pub struct ToSqlConfigEntity {
    pub enabled: bool,
    pub callback: Option<ToSqlFn>,
    pub content: Option<&'static str>,
}
impl ToSqlConfigEntity {
    /// Given a SqlGraphEntity, this function converts it to SQL based on the current configuration.
    ///
    /// If the config overrides the default behavior (i.e. using the `ToSql` trait), then `Some(eyre::Result)`
    /// is returned. If the config does not override the default behavior, then `None` is returned. This can
    /// be used to dispatch SQL generation in a single line, e.g.:
    ///
    /// ```rust,ignore
    /// config.to_sql(entity, context).unwrap_or_else(|| entity.to_sql(context))?
    /// ```
    pub fn to_sql(
        &self,
        entity: &SqlGraphEntity,
        context: &PgxSql,
    ) -> Option<eyre::Result<String>> {
        use eyre::{eyre, WrapErr};

        if !self.enabled {
            return Some(Ok(String::default()));
        }

        if let Some(content) = self.content {
            return Some(Ok("\n".to_owned() + content));
        }

        if let Some(callback) = self.callback {
            return Some(
                callback(entity, context)
                    .map_err(|e| eyre!(e))
                    .wrap_err("Failed to run specified `#[pgx(sql = path)] function`"),
            );
        }

        None
    }
}
impl std::cmp::PartialEq for ToSqlConfigEntity {
    fn eq(&self, other: &Self) -> bool {
        if self.enabled != other.enabled {
            return false;
        }
        match (self.callback, other.callback) {
            (None, None) => match (self.content, other.content) {
                (None, None) => true,
                (Some(a), Some(b)) => a == b,
                _ => false,
            },
            (Some(a), Some(b)) => std::ptr::eq(std::ptr::addr_of!(a), std::ptr::addr_of!(b)),
            _ => false,
        }
    }
}
impl std::cmp::Eq for ToSqlConfigEntity {}
impl std::hash::Hash for ToSqlConfigEntity {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.enabled.hash(state);
        self.callback.map(|cb| std::ptr::addr_of!(cb)).hash(state);
        self.content.hash(state);
    }
}
impl std::fmt::Debug for ToSqlConfigEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let callback = self.callback.map(|cb| std::ptr::addr_of!(cb));
        f.debug_struct("ToSqlConfigEntity")
            .field("enabled", &self.enabled)
            .field("callback", &format_args!("{:?}", &callback))
            .field("content", &self.content)
            .finish()
    }
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
