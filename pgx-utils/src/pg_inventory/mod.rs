mod pg_extern;
mod postgres_enum;
mod postgres_hash;
mod postgres_ord;
mod postgres_type;
mod pg_schema;

pub use pg_extern::{PgExtern, InventoryPgExtern, InventoryPgExternReturn, InventoryPgExternInput, InventoryPgOperator};
pub use postgres_enum::{PostgresEnum, InventoryPostgresEnum};
pub use postgres_hash::{PostgresHash, InventoryPostgresHash};
pub use postgres_ord::{PostgresOrd, InventoryPostgresOrd};
pub use postgres_type::{PostgresType, InventoryPostgresType};
pub use pg_schema::{Schema, InventorySchema};

#[derive(Debug)]
pub struct InventoryExtensionSql {
    pub sql: &'static str,
    pub file: &'static str,
    pub line: u32,
}