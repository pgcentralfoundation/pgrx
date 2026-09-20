//! The `pgrust` target of `pgrx-pg-sys`.
//!
//! On Postgres, `pgrx-pg-sys` is a bindgen dump of the C headers and every
//! function is a dlsym'd C symbol. Under pgrust there is no C ABI: the backend
//! is Rust, errors are `PgResult` values, memory contexts are passed
//! explicitly, and extensions are statically linked crates registered with
//! `dfmgr`. This module presents the *same names* the pgrx runtime uses, but
//! implemented over the pgrust crates:
//!
//! * [`types`] — C-layout structs and constants harvested from the PG18
//!   bindings (on-datum layouts are identical in pgrust by construction);
//! * [`mem`] — `palloc`/`pfree` and the memory-context globals, over a
//!   per-thread "current context" stack that the call wrapper arms;
//! * [`error`] — the bridge between pgrust `PgError` values and pgrx's
//!   panic-based error propagation;
//! * [`fmgr`] — the C-shaped `FunctionCallInfoBaseData`/`FmgrInfo` views a
//!   wrapper builds around the native pgrust call frame;
//! * [`bridge`] — SQL-callable builtins (`jsonb_in`, `numeric_add`, ...)
//!   exposed with the C `fn(FunctionCallInfo) -> Datum` signature;
//! * [`funcs`] — hand-written shims for the non-fmgr backend functions;
//! * [`registry`] — `linkme` slices the macros fill with wrapper functions,
//!   schema entities and the extension descriptor, and the registration
//!   into pgrust's builtin-library and embedded-extension registries;
//! * [`srf`] — set-returning functions via pgrust's materialize protocol.

pub mod bridge;
pub mod error;
pub mod fmgr;
pub mod funcs;
pub mod guc;
pub mod mem;
pub mod port;
pub mod registry;
pub mod srf;
pub mod tuples;
pub mod types;

/// Backend structs pgrx names only as opaque handles under pgrust.
macro_rules! opaque {
    ($($name:ident),* $(,)?) => {
        $(
            #[repr(C)]
            #[derive(Debug, Copy, Clone)]
            pub struct $name {
                _opaque: [u8; 0],
            }
        )*
    };
}
opaque!(
    FdwRoutine,
    IndexAmRoutine,
    TableAmRoutine,
    PlannerInfo,
    TypeCacheEntry,
    pg_tz,
    TriggerData
);

#[path = "oids_table.rs"]
pub(crate) mod oids_table;

pub use bridge::*;
pub use fmgr::*;
pub use funcs::*;
pub use guc::*;
pub use mem::*;
pub use port::*;
pub use registry::*;
pub use tuples::*;
pub use types::*;

// The linkme crate, re-exported so macro-generated code in extension crates
// can name it without adding a dependency.
pub use linkme;

/// pgrust's SPI crate, for the Spi arm of the runtime.
pub mod spi_native {
    pub use ::pgr_spi::*;
}
pub type TupleDescNative<'mcx> = ::pgr_types_tuple::TupleDescData<'mcx>;
pub type HeapTupleNative<'tup> = ::pgr_types_tuple::HeapTupleData<'tup>;

// Native pgrust types the generated wrappers name.
pub type NativeFmgrInfo = ::pgr_fmgr::FmgrInfo;
pub type NativeFcinfo = ::pgr_fmgr::FunctionCallInfoBaseData;
pub type NativeDatum = ::pgr_datum::Datum;
pub type NativeResult = ::pgr_types_error::PgResult<NativeDatum>;
pub type NativeUnitResult = ::pgr_types_error::PgResult<()>;
pub type NativePGFunction = ::pgr_fmgr::PGFunction;

pub use error::{PgrustError, raise, unwrap_pg};
pub use fmgr::call_v1;
