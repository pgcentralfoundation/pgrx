//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
//! `pgrx` is a framework for creating Postgres extensions in 100% Rust
//!
//! ## Example
//!
//! ```rust
//! use pgrx::prelude::*;
//!
//! // Convert the input string to lowercase and return
//! #[pg_extern]
//! fn my_to_lowercase(input: &str) -> String {
//!     input.to_lowercase()
//! }
//!
//! ```
#![cfg_attr(feature = "nightly", feature(allocator_api))]

#[macro_use]
extern crate bitflags;
extern crate alloc;

use std::sync::LazyLock as Lazy;
// expose our various derive macros
pub use pgrx_macros;
pub use pgrx_macros::*;
#[doc(inline)]
pub use pgrx_sql_entity_graph::metadata::impl_sql_translatable;
pub use pgrx_sql_entity_graph::pgrx_resolved_type;

/// The PGRX prelude includes necessary imports to make extensions work.
pub mod prelude;

pub mod aggregate;
pub mod array;
#[cfg(not(feature = "pgrust"))]
pub mod atomics;
#[cfg(not(feature = "pgrust"))]
pub mod bgworkers;
#[cfg(not(feature = "pgrust"))]
pub mod callbacks;
pub mod callconv;
pub mod datetime;
pub mod datum;
#[cfg(not(feature = "pgrust"))]
pub mod enum_helper;
pub mod fcinfo;
pub mod ffi;
#[cfg(not(feature = "pgrust"))]
pub mod fn_call;
pub mod guc;
pub mod heap_tuple;
#[cfg(not(feature = "pgrust"))]
pub mod htup;
#[cfg(feature = "pgrust")]
mod htup_pgrust;
pub mod inoutfuncs;
pub mod itemptr;
pub mod iter;
pub mod layout;
#[cfg(not(feature = "pgrust"))]
pub mod list;
#[cfg(not(feature = "pgrust"))]
pub mod lwlock;
pub mod memcx;
pub mod memcxt;
pub mod misc;
#[cfg(feature = "cshim")]
pub mod namespace;
#[cfg(not(feature = "pgrust"))]
pub mod nodes;
pub mod nullable;
pub mod palloc;
#[cfg(not(feature = "pgrust"))]
pub mod pg_catalog;
pub mod pgbox;
#[cfg(not(feature = "pgrust"))]
pub mod rel;
#[cfg(not(feature = "pgrust"))]
pub mod shmem;
pub mod spi;
#[cfg(feature = "cshim")]
pub mod spinlock;
pub mod stringinfo;
#[cfg(not(feature = "pgrust"))]
pub mod trigger_support;
pub mod tupdesc;
pub mod varlena;
pub mod wrappers;
#[cfg(not(feature = "pgrust"))]
pub mod xid;

/// Not ready for public exposure.
mod ptr;
mod slice;
mod toast;

pub use aggregate::*;
#[cfg(not(feature = "pgrust"))]
pub use atomics::*;
#[cfg(not(feature = "pgrust"))]
pub use callbacks::*;
pub use datum::{
    AnyArray, AnyElement, AnyNumeric, Array, FromDatum, Inet, Internal, IntoDatum, Json, JsonB,
    Numeric, Range, Uuid, VariadicArray, geo, numeric,
};
#[cfg(not(feature = "pgrust"))]
pub use enum_helper::*;
pub use fcinfo::*;
pub use guc::*;
#[cfg(not(feature = "pgrust"))]
pub use htup::*;
#[cfg(feature = "pgrust")]
pub use htup_pgrust::*;
pub use inoutfuncs::*;
#[cfg(feature = "cshim")]
pub use list::old_list::*;
#[cfg(not(feature = "pgrust"))]
pub use lwlock::*;
pub use memcxt::*;
#[cfg(feature = "cshim")]
pub use namespace::*;
#[cfg(not(feature = "pgrust"))]
pub use nodes::*;
pub use pgbox::*;
#[cfg(not(feature = "pgrust"))]
pub use rel::*;
#[cfg(not(feature = "pgrust"))]
pub use shmem::*;
pub use spi::Spi; // only Spi.  We don't want the top-level namespace polluted with spi::Result and spi::Error
pub use stringinfo::*;
#[cfg(not(feature = "pgrust"))]
pub use trigger_support::*;
pub use tupdesc::*;
pub use varlena::*;
pub use wrappers::*;
#[cfg(not(feature = "pgrust"))]
pub use xid::*;

pub mod pg_sys;

// and re-export these
pub use pg_sys::PgBuiltInOids;
pub use pg_sys::elog::PgLogLevel;
pub use pg_sys::errcodes::PgSqlErrorCode;
pub use pg_sys::oids::PgOid;
pub use pg_sys::panic::pgrx_extern_c_guard;
pub use pg_sys::pg_try::PgTryBuilder;
pub use pg_sys::utils::name_data_to_str;
pub use pg_sys::{
    FATAL, PANIC, check_for_interrupts, debug1, debug2, debug3, debug4, debug5, ereport,
    ereport_domain, error, function_name, info, log, notice, warning,
};

#[doc(hidden)]
pub use pgrx_sql_entity_graph;

mod seal {
    /// A trait you can reference but can't impl externally
    pub trait Sealed {}
}

#[doc(hidden)]
pub mod pg_magic_func_support {
    use core::ffi::CStr;

    use crate::pg_sys;

    pub const fn default_magic() -> pg_sys::Pg_magic_struct {
        let len = ::core::mem::size_of::<pg_sys::Pg_magic_struct>() as i32;
        let version = pg_sys::PG_VERSION_NUM as i32 / 100;
        let funcmaxargs = pg_sys::FUNC_MAX_ARGS as i32;
        let indexmaxkeys = pg_sys::INDEX_MAX_KEYS as i32;
        let namedatalen = pg_sys::NAMEDATALEN as i32;
        let float8byval = cfg!(target_pointer_width = "64") as i32;
        #[cfg(any(
            feature = "pg15",
            feature = "pg16",
            feature = "pg17",
            feature = "pg18",
            feature = "pg19", feature = "pgrust"
        ))]
        let abi_extra = abi_extra();

        pg_sys::Pg_magic_struct {
            len,
            #[cfg(not(any(feature = "pg18", feature = "pg19", feature = "pgrust", feature = "pgrust")))]
            version,
            #[cfg(not(any(feature = "pg18", feature = "pg19", feature = "pgrust", feature = "pgrust")))]
            funcmaxargs,
            #[cfg(not(any(feature = "pg18", feature = "pg19", feature = "pgrust", feature = "pgrust")))]
            indexmaxkeys,
            #[cfg(not(any(feature = "pg18", feature = "pg19", feature = "pgrust", feature = "pgrust")))]
            namedatalen,
            #[cfg(not(any(feature = "pg18", feature = "pg19", feature = "pgrust", feature = "pgrust")))]
            float8byval,
            #[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
            abi_extra,
            #[cfg(any(feature = "pg18", feature = "pg19", feature = "pgrust", feature = "pgrust"))]
            abi_fields: pg_sys::Pg_abi_values {
                version,
                funcmaxargs,
                indexmaxkeys,
                namedatalen,
                float8byval,
                abi_extra,
            },
            #[cfg(any(feature = "pg18", feature = "pg19", feature = "pgrust", feature = "pgrust"))]
            name: ::core::ptr::null(),
            #[cfg(any(feature = "pg18", feature = "pg19", feature = "pgrust", feature = "pgrust"))]
            version: ::core::ptr::null(),
        }
    }

    #[cfg(any(
        feature = "pg15",
        feature = "pg16",
        feature = "pg17",
        feature = "pg18",
        feature = "pg19", feature = "pgrust"
    ))]
    const fn abi_extra() -> [::core::ffi::c_char; 32] {
        // We'll use what the bindings tell us, but if it ain't "PostgreSQL" then we'll
        // raise a compilation error unless the `unsafe-postgres` feature is set.
        let magic = pg_sys::FMGR_ABI_EXTRA.to_bytes_with_nul();
        let mut abi = [0 as ::core::ffi::c_char; 32];
        let mut i = 0;
        while i < magic.len() {
            abi[i] = magic[i] as ::core::ffi::c_char;
            i += 1;
        }
        abi
    }

    #[allow(unused_mut, unused_variables)]
    pub const fn with_name(
        mut magic: pg_sys::Pg_magic_struct,
        name: &'static CStr,
    ) -> pg_sys::Pg_magic_struct {
        #[cfg(any(feature = "pg18", feature = "pg19", feature = "pgrust", feature = "pgrust"))]
        {
            magic.name = CStr::as_ptr(name);
        }
        magic
    }

    #[allow(unused_mut, unused_variables)]
    pub const fn with_version(
        mut magic: pg_sys::Pg_magic_struct,
        version: &'static CStr,
    ) -> pg_sys::Pg_magic_struct {
        #[cfg(any(feature = "pg18", feature = "pg19", feature = "pgrust", feature = "pgrust"))]
        {
            magic.version = CStr::as_ptr(version);
        }
        magic
    }
}

// Postgres v15+ has the concept of an ABI "name".  The default is `c"PostgreSQL"` and this is the
// ABI that pgrx extensions expect to be running under.  We will refuse to compile if it is detected
// that we're trying to be built against some other kind of "postgres" that has its own ABI name.
//
// Unless the compiling user explicitly told us that they're aware of this via `--features unsafe-postgres`.
#[cfg(all(
    any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19", feature = "pgrust", feature = "pgrust"),
    not(feature = "unsafe-postgres")
))]
const _: () = {
    use core::ffi::CStr;
    // to appease `const`
    const fn same_cstr(a: &CStr, b: &CStr) -> bool {
        if a.to_bytes().len() != b.to_bytes().len() {
            return false;
        }
        let mut i = 0;
        while i < a.to_bytes().len() {
            if a.to_bytes()[i] != b.to_bytes()[i] {
                return false;
            }
            i += 1;
        }
        true
    }
    assert!(
        same_cstr(pg_sys::FMGR_ABI_EXTRA, c"PostgreSQL"),
        "Unsupported Postgres ABI. Perhaps you need `--features unsafe-postgres`?",
    );
};

/// A macro for marking a library compatible with [`pgrx`][crate].
///
/// <div class="example-wrap" style="display:inline-block">
/// <pre class="ignore" style="white-space:normal;font:inherit;">
///
/// **Note**: Every [`pgrx`][crate] extension **must** have this macro called at top level (usually `src/lib.rs`) to be valid. You can use `pg_module_magic!()`, `pg_module_magic!(name, version)` or `pg_module_magic!(name = c"custom_name", version = c"1.0.0")`
///
/// </pre></div>
///
/// This calls [`pg_magic_func!()`](pg_magic_func).
#[macro_export]
macro_rules! pg_module_magic {
    ($($key:ident $(= $value:expr)?),*) => {
        $crate::pg_magic_func!($($key $(= $value)?),*);
    };
}

/// Create the `Pg_magic_func` required by PGRX in extensions.
///
/// <div class="example-wrap" style="display:inline-block">
/// <pre class="ignore" style="white-space:normal;font:inherit;">
///
/// **Note**: Generally [`pg_module_magic`] is preferred, and results in this macro being called.
/// This macro should only be directly called in advanced use cases.
///
/// </pre></div>
///
/// Creates a “magic block” that describes the capabilities of the extension to
/// Postgres at runtime. From the [Dynamic Loading] section of the upstream documentation:
///
/// > To ensure that a dynamically loaded object file is not loaded into an incompatible
/// > server, PostgreSQL checks that the file contains a “magic block” with the appropriate
/// > contents. This allows the server to detect obvious incompatibilities, such as code
/// > compiled for a different major version of PostgreSQL. To include a magic block,
/// > write this in one (and only one) of the module source files, after having included
/// > the header `fmgr.h`:
/// >
/// > ```c
/// > PG_MODULE_MAGIC;
/// > ```
///
/// ## Acknowledgements
///
/// This macro was initially inspired from the `pg_module` macro by [Daniel Fagnan]
/// and expanded by [Benjamin Fry].
///
/// [Benjamin Fry]: https://github.com/bluejekyll/pg-extend-rs
/// [Daniel Fagnan]: https://github.com/thehydroimpulse/postgres-extension.rs
/// [Dynamic Loading]: https://www.postgresql.org/docs/current/xfunc-c.html#XFUNC-C-DYNLOAD
/// pgrust: no `Pg_magic_func` (nothing dlopens the crate). Instead the
/// crate declares its extension (name, version, control file) and an
/// `init_seams()` the server calls at startup to register the crate's
/// wrappers with `dfmgr` and its generated SQL as an embedded extension.
#[cfg(feature = "pgrust")]
#[macro_export]
macro_rules! pg_magic_func {
    ($($key:ident $(= $value: expr)?),*) => {
        ::pgrx::pgrx_sql_entity_graph::__pgrx_schema_entry!(
            __PGRX_SCHEMA_SECTION_SENTINEL,
            ::pgrx::pgrx_sql_entity_graph::section::SECTION_SENTINEL_ENTRY_LEN,
            ::pgrx::pgrx_sql_entity_graph::section::schema_section_sentinel_entry()
        );

        #[doc(hidden)]
        pub static __PGRX_EXTENSION: ::pgrx::pg_sys::pgrust::ExtensionDesc =
            ::pgrx::pg_sys::pgrust::ExtensionDesc {
                krate: env!("CARGO_PKG_NAME"),
                name: env!("CARGO_PKG_NAME"),
                version: env!("CARGO_PKG_VERSION"),
                control: include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", env!("CARGO_PKG_NAME"), ".control")),
                pg_init: Some(__pgrx_pg_init),
            };

        #[doc(hidden)]
        fn __pgrx_pg_init() -> ::pgrx::pg_sys::pgrust::NativeUnitResult {
            ::pgrx::pg_sys::pgrust::run_pg_init(env!("CARGO_PKG_NAME"))
        }

        #[doc(hidden)]
        fn __pgrx_lookup(symbol: &str) -> ::core::option::Option<::pgrx::pg_sys::pgrust::NativePGFunction> {
            ::pgrx::pg_sys::pgrust::lookup_in(env!("CARGO_PKG_NAME"), symbol)
        }

        /// Register this extension with the pgrust server (called from seams_init).
        pub fn init_seams() {
            ::pgrx::pg_sys::pgrust::register_extension(&__PGRX_EXTENSION, __pgrx_lookup)
        }
    };
}

#[cfg(not(feature = "pgrust"))]
#[macro_export]
macro_rules! pg_magic_func {
    ($($key:ident $(= $value: expr)?),*) => {
        ::pgrx::pgrx_sql_entity_graph::__pgrx_schema_entry!(
            __PGRX_SCHEMA_SECTION_SENTINEL,
            ::pgrx::pgrx_sql_entity_graph::section::SECTION_SENTINEL_ENTRY_LEN,
            ::pgrx::pgrx_sql_entity_graph::section::schema_section_sentinel_entry()
        );

        #[unsafe(no_mangle)]
        #[allow(non_snake_case, unexpected_cfgs)]
        #[doc(hidden)]
        pub extern "C" fn Pg_magic_func() -> &'static ::pgrx::pg_sys::Pg_magic_struct {
            #[repr(transparent)]
            struct AssertSync<T>(T);
            unsafe impl<T> Sync for AssertSync<T> {}

            // since Postgres calls this first, register our panic handler now
            // so we don't unwind into C / Postgres
            ::pgrx::pg_sys::panic::register_pg_guard_panic_hook();

            static MY_MAGIC: AssertSync<::pgrx::pg_sys::Pg_magic_struct> = {
                let mut magic = ::pgrx::pg_magic_func_support::default_magic();
                #[allow(unused_macros)]
                macro_rules! field_update {
                    (name) => {
                        field_update!(name = {
                            const RAW: &str = env!("CARGO_PKG_NAME");
                            const BUFFER: [u8; RAW.len() + 1] = {
                                let mut buffer = [0u8; RAW.len() + 1];
                                let mut i = 0_usize;
                                while i < RAW.len() {
                                    buffer[i] = RAW.as_bytes()[i];
                                    i += 1;
                                }
                                buffer
                            };
                            const STR: &::core::ffi::CStr =
                                if let Ok(s) = ::core::ffi::CStr::from_bytes_with_nul(&BUFFER) {
                                    s
                                } else {
                                    panic!("there are null characters in CARGO_PKG_NAME")
                                };
                            const { STR }
                        });
                    };
                    (version) => {
                        field_update!(version = {
                            const RAW: &str = env!("CARGO_PKG_VERSION");
                            const BUFFER: [u8; RAW.len() + 1] = {
                                let mut buffer = [0u8; RAW.len() + 1];
                                let mut i = 0_usize;
                                while i < RAW.len() {
                                    buffer[i] = RAW.as_bytes()[i];
                                    i += 1;
                                }
                                buffer
                            };
                            const STR: &::core::ffi::CStr =
                                if let Ok(s) = ::core::ffi::CStr::from_bytes_with_nul(&BUFFER) {
                                    s
                                } else {
                                panic!("there are null characters in CARGO_PKG_VERSION")
                             };
                            const { STR }
                        });
                    };
                    (name = $name:expr) => {
                        let name: &'static ::core::ffi::CStr = $name;
                        magic = ::pgrx::pg_magic_func_support::with_name(magic, name);
                    };
                    (version = $version:expr) => {
                        let version: &'static ::core::ffi::CStr = $version;
                        magic = ::pgrx::pg_magic_func_support::with_version(magic, version);
                    };
                }
                $(field_update!($key $(= $value)?);)*
                AssertSync(magic)
            };

            // return the magic
            &MY_MAGIC.0
        }
    };
}

pub(crate) static UTF8DATABASE: Lazy<Utf8Compat> = Lazy::new(|| {
    use pg_sys::pg_enc::*;
    let encoding_int = unsafe { pg_sys::GetDatabaseEncoding() };
    match encoding_int as _ {
        PG_UTF8 => Utf8Compat::Yes,
        // The 0 encoding. It... may be UTF-8
        PG_SQL_ASCII => Utf8Compat::Maybe,
        // Modifies ASCII, and should never be seen as PG doesn't support it as server encoding
        PG_SJIS | PG_SHIFT_JIS_2004
        // Not specified as an ASCII extension, also not a server encoding
        | PG_BIG5
        // Wild vendor differences including non-ASCII are possible, also not a server encoding
        | PG_JOHAB => unreachable!("impossible? unsupported non-ASCII-compatible database encoding is not a server encoding"),
        // Other Postgres encodings either extend US-ASCII or CP437 (which includes US-ASCII)
        // There may be a subtlety that requires us to revisit this later
        1..=41=> Utf8Compat::Ascii,
        // Unfamiliar encoding? Run UTF-8 validation like normal and hope for the best
        _ => Utf8Compat::Maybe,
    }
});

#[derive(Debug, Clone, Copy)]
pub(crate) enum Utf8Compat {
    /// It's UTF-8, so... obviously
    Yes,
    /// This is what is assumed about "SQL_ASCII"
    Maybe,
    /// An "extended ASCII" encoding, so we're fine if we only touch ASCII
    Ascii,
}
