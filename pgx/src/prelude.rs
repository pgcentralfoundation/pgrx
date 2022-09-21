// From "external" crates:
pub use ::pgx_macros::{pg_extern, pg_guard};
pub use ::pgx_pg_sys as pg_sys;

// Necessary local macros:
pub use crate::default;
