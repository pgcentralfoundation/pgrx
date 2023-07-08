pub mod datum;
#[macro_use]
pub mod elog;
pub mod errcodes;
pub mod ffi;
pub mod htup;
pub mod oids;
pub mod panic;
pub mod pg_try;
pub mod polyfill;
pub(crate) mod thread_check;
pub mod tupdesc;

pub mod utils;

// Various SqlTranslatable mappings for SQL generation
mod sql_translatable;

pub use datum::Datum;
// OnDrop(feature = "pg11"): remove this cfg if all supported versions of Postgres
// now include NullableDatum.
#[cfg(any(
    feature = "pg12",
    feature = "pg13",
    feature = "pg14",
    feature = "pg15",
    feature = "pg16"
))]
pub use datum::NullableDatum;

pub use oids::*;
pub use pg_try::*;
pub use polyfill::*;
pub use tupdesc::*;
pub use utils::*;

#[cfg(target_os = "linux")]
extern "C" {
    #[link_name = "__sigsetjmp"]
    pub(crate) fn sigsetjmp(
        env: *mut crate::sigjmp_buf,
        savemask: std::os::raw::c_int,
    ) -> std::os::raw::c_int;
}

#[cfg(any(target_os = "macos", target_os = "freebsd", target_os = "openbsd"))]
extern "C" {
    pub(crate) fn sigsetjmp(
        env: *mut crate::sigjmp_buf,
        savemask: std::os::raw::c_int,
    ) -> std::os::raw::c_int;
}
