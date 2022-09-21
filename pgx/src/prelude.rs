// From "external" crates:
pub use ::pgx_macros::{pg_extern, pg_guard};
pub use ::pgx_pg_sys as pg_sys;

// Necessary local macros:
pub use crate::{default, name};

// Needed for variant RETURNS
pub use crate::iter::{SetOfIterator, TableIterator};
