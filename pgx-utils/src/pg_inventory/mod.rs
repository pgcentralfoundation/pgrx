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

pub trait DotFormat {
    fn dot_format(&self) -> String;
}
