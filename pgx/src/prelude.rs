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

pub use crate::pg_sys::oids::PgOid;
pub use crate::pg_sys::pg_try::PgTryBuilder;
pub use crate::pg_sys::utils::name_data_to_str;
pub use crate::pg_sys::PgBuiltInOids;

// It's a database, gotta query it somehow.
pub use crate::spi::Spi;

// Logging and Error support
pub use crate::pg_sys::elog::PgLogLevel;
pub use crate::pg_sys::errcodes::PgSqlErrorCode;
pub use crate::pg_sys::{
    check_for_interrupts, debug1, debug2, debug3, debug4, debug5, ereport, error, function_name,
    info, log, notice, warning, FATAL, PANIC,
};
