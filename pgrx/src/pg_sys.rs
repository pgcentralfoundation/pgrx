//! PostgreSQL bindings and macros generated from the selected server headers.
//!
//! Where this build translates a static inline function's body, `pg_sys::NAME` runs that
//! translation in Rust rather than calling the C function through a guarded binding. The
//! PostgreSQL functions the body calls keep their guards, and a body that reads or writes a
//! PostgreSQL global first checks for the backend thread. `pgrx_pg_sys::NAME` is always the
//! guarded binding.

pub use pgrx_pg_sys::*;

// Each function this defines shadows the glob-imported binding with its name and signature.
// pgrx-pg-sys cannot define them itself: a crate cannot expand its own `include!`-generated
// exported macros by path (rust-lang/rust#52234), and the functions expand the generated
// macros.
pgrx_pg_sys::__pgrx_c_inline_functions!();
