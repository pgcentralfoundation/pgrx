// From "external" crates:
pub use ::pgx_macros::{
    extension_sql, extension_sql_file, pg_aggregate, pg_extern, pg_guard, pg_schema, pg_test,
    pg_trigger, search_path, PostgresEnum, PostgresType,
};
pub use ::pgx_pg_sys as pg_sys;

// Necessary local macros:
pub use crate::{default, name};

// Needed for variant RETURNS
pub use crate::iter::{SetOfIterator, TableIterator};

// Needed for complex returns.
pub use crate::heap_tuple::PgHeapTuple;
pub use crate::pgbox::PgBox;

// These could be factored into a temporal type module that could be easily imported for code which works with them.
// However, reexporting them seems fine for now.
pub use crate::datum::{Date, Time, TimeWithTimeZone, Timestamp, TimestampWithTimeZone};

pub use crate::pg_sys::PgBuiltInOids;

// It's a database, gotta query it somehow.
pub use crate::spi::Spi;
