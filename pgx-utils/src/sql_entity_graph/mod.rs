mod extension_sql;
mod pg_extern;
mod pg_schema;
mod positioning_ref;
mod postgres_enum;
mod postgres_hash;
mod postgres_ord;
mod postgres_type;

pub use super::ExternArgs;
pub use extension_sql::{ExtensionSql, ExtensionSqlFile, SqlDeclared};
pub use pg_extern::{Argument, PgExtern, PgOperator};
pub use pg_schema::Schema;
pub use positioning_ref::PositioningRef;
pub use postgres_enum::PostgresEnum;
pub use postgres_hash::PostgresHash;
pub use postgres_ord::PostgresOrd;
pub use postgres_type::PostgresType;

/// Reexports for the pgx SQL generator binaries.
#[doc(hidden)]
pub mod reexports {
    #[doc(hidden)]
    pub use clap;
    #[doc(hidden)]
    pub use color_eyre;
    #[doc(hidden)]
    pub use eyre;
    #[doc(hidden)]
    pub use libloading;
    #[doc(hidden)]
    pub use tracing;
    #[doc(hidden)]
    pub use tracing_error;
    #[doc(hidden)]
    pub use tracing_subscriber;
}
