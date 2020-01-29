extern crate pgx_macros;

extern crate num_traits;

// expose the #[derive(DatumCompatible)] trait
pub use pgx_macros::*;

// we need this publicly too
pub use std::convert::TryInto;

pub mod callbacks;
pub mod datum;
pub mod enum_helper;
pub mod fcinfo;
pub mod htup;
pub mod inoutfuncs;
pub mod itemptr;
pub mod list;
pub mod log;
pub mod memcxt;
pub mod namespace;
pub mod nodes;
pub mod oids;
pub mod pgbox;
pub mod spi;
pub mod stringinfo;
pub mod tupdesc;
pub mod utils;
pub mod varlena;
pub mod xid;

pub use callbacks::*;
pub use datum::*;
pub use enum_helper::*;
pub use fcinfo::*;
pub use htup::*;
pub use inoutfuncs::*;
pub use itemptr::*;
pub use list::*;
pub use log::*;
pub use memcxt::*;
pub use namespace::*;
pub use nodes::{is_a, PgNode, PgNodeFactory}; // be specific since we have multiple versions of these things behind feature gates
pub use oids::*;
pub use pgbox::*;
pub use spi::*;
pub use stringinfo::*;
pub use tupdesc::*;
pub use utils::*;
pub use varlena::*;
pub use xid::*;

pub use pgx_pg_sys as pg_sys; // the module only, not its contents
pub use pgx_pg_sys::guard;
pub use pgx_pg_sys::guard::*;

/// A macro for marking a library compatible with the Postgres extension framework.
///
/// This macro was initially inspired from the `pg_module` macro in https://github.com/thehydroimpulse/postgres-extension.rs
///
/// Shameless;y cribbed from https://github.com/bluejekyll/pg-extend-rs
#[macro_export]
macro_rules! pg_module_magic {
    () => {
        #[no_mangle]
        #[allow(non_snake_case)]
        #[allow(unused)]
        #[link_name = "Pg_magic_func"]
        pub extern "C" fn Pg_magic_func() -> &'static pgx::pg_sys::Pg_magic_struct {
            use pgx;
            use std::mem::size_of;
            use std::os::raw::c_int;

            const MY_MAGIC: pgx::pg_sys::Pg_magic_struct = pgx::pg_sys::Pg_magic_struct {
                len: size_of::<pgx::pg_sys::Pg_magic_struct>() as c_int,
                version: pgx::pg_sys::PG_VERSION_NUM as std::os::raw::c_int / 100,
                funcmaxargs: pgx::pg_sys::FUNC_MAX_ARGS as c_int,
                indexmaxkeys: pgx::pg_sys::INDEX_MAX_KEYS as c_int,
                namedatalen: pgx::pg_sys::NAMEDATALEN as c_int,
                float4byval: pgx::pg_sys::USE_FLOAT4_BYVAL as c_int,
                float8byval: pgx::pg_sys::USE_FLOAT8_BYVAL as c_int,
            };

            // go ahead and register our panic handler since Postgres
            // calls this function first
            pgx::initialize();

            // return the magic
            &MY_MAGIC
        }
    };
}

/// Top-level initialization function
///
/// C-based Postgres extensions should call this in their _PG_init() function
#[allow(unused)]
#[no_mangle]
pub extern "C" fn initialize() {
    register_pg_guard_panic_handler();
}
