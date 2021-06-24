// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

//! `pgx` is a framework for creating Postgres extensions in 100% Rust
//!
//! ## Example
//!
//! ```rust,no_run
//! use pgx::*;
//!
//! pg_module_magic!();
//!
//! // Convert the input string to lowercase and return
//! #[pg_extern]
//! fn my_to_lowercase(input: &'static str) -> String {
//!     input.to_lowercase()
//! }
//!
//! ```
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::cast_ptr_alignment)]
extern crate pgx_macros;

extern crate num_traits;

#[macro_use]
extern crate bitflags;

// expose our various derive macros
pub use pgx_macros::*;

pub mod callbacks;
pub mod datum;
pub mod enum_helper;
pub mod fcinfo;
pub mod guc;
pub mod hooks;
pub mod htup;
pub mod inoutfuncs;
pub mod itemptr;
pub mod list;
#[macro_use]
pub mod log;
pub mod atomics;
pub mod bgworkers;
pub mod lwlock;
pub mod memcxt;
pub mod misc;
pub mod namespace;
pub mod nodes;
pub mod pgbox;
pub mod rel;
pub mod shmem;
pub mod spi;
pub mod stringinfo;
pub mod trigger_support;
pub mod tupdesc;
pub mod varlena;
pub mod wrappers;
pub mod xid;

#[doc(hidden)]
pub use inventory;
#[doc(hidden)]
pub use once_cell;


pub use atomics::*;
pub use callbacks::*;
pub use datum::*;
pub use enum_helper::*;
pub use fcinfo::*;
pub use guc::*;
pub use hooks::*;
pub use htup::*;
pub use inoutfuncs::*;
pub use itemptr::*;
pub use list::*;
pub use log::*;
pub use lwlock::*;
pub use memcxt::*;
pub use namespace::*;
pub use nodes::*;
pub use pgbox::*;
pub use rel::*;
pub use shmem::*;
pub use spi::*;
pub use stringinfo::*;
pub use trigger_support::*;
pub use tupdesc::*;
pub use varlena::*;
pub use wrappers::*;
pub use xid::*;

pub use pgx_pg_sys as pg_sys; // the module only, not its contents
pub use pgx_pg_sys::submodules::*;
pub use pgx_pg_sys::PgBuiltInOids; // reexport this so it looks like it comes from here

use core::any::TypeId;
use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Top-level initialization function.  This is called automatically by the `pg_module_magic!()`
/// macro and need not be called directly
#[allow(unused)]
pub fn initialize() {
    register_pg_guard_panic_handler();
}

pub static DEFAULT_TYPEID_SQL_MAPPING: Lazy<HashMap<TypeId, String>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(TypeId::of::<str>(), String::from("text"));
    m.insert(TypeId::of::<&'static str>(), String::from("text"));
    m.insert(TypeId::of::<Option<&'static str>>(), String::from("text"));
    m.insert(TypeId::of::<Vec<&'static str>>(), String::from("text[]"));
    m.insert(TypeId::of::<Option<Vec<&'static str>>>(), String::from("text[]"));
    m.insert(TypeId::of::<Vec<Option<&'static str>>>(), String::from("text[]"));
    m.insert(TypeId::of::<datum::Array<&'static str>>(), String::from("text[]"));
    m.insert(TypeId::of::<Option<datum::Array<&'static str>>>(), String::from("text[]"));

    m.insert(
        TypeId::of::<Vec<Option<&'static str>>>(),
        String::from("text[]"),
    );

    m.insert(TypeId::of::<&std::ffi::CStr>(), String::from("cstring"));
    m.insert(
        TypeId::of::<Option<&std::ffi::CStr>>(),
        String::from("cstring"),
    );

    m.insert(TypeId::of::<String>(), String::from("text"));
    m.insert(TypeId::of::<Option<String>>(), String::from("text"));
    m.insert(TypeId::of::<datum::Array<String>>(), String::from("text[]"));
    m.insert(TypeId::of::<Vec<String>>(), String::from("text[]"));
    m.insert(TypeId::of::<Vec<Option<String>>>(), String::from("text[]"));
    m.insert(TypeId::of::<Option<Vec<String>>>(), String::from("text[]"));
    m.insert(TypeId::of::<Option<Vec<Option<String>>>>(), String::from("text[]"));
    m.insert(TypeId::of::<Option<Vec<Option<&String>>>>(), String::from("text[]"));

    m.insert(TypeId::of::<()>(), String::from("void"));

    m.insert(TypeId::of::<i8>(), String::from("\"char\""));
    m.insert(TypeId::of::<Option<i8>>(), String::from("\"char\""));
    m.insert(TypeId::of::<Vec<i8>>(), String::from("\"char\"[]"));
    m.insert(TypeId::of::<Vec<Option<i8>>>(), String::from("\"char\"[]"));

    m.insert(TypeId::of::<i16>(), String::from("smallint"));
    m.insert(TypeId::of::<Option<i16>>(), String::from("smallint"));
    m.insert(
        TypeId::of::<datum::Array<i16>>(),
        String::from("smallint[]"),
    );
    m.insert(TypeId::of::<Vec<i8>>(), String::from("\"char\""));
    m.insert(TypeId::of::<Vec<Option<i8>>>(), String::from("\"char\""));

    m.insert(TypeId::of::<i32>(), String::from("integer"));
    m.insert(TypeId::of::<Option<i32>>(), String::from("integer"));
    m.insert(TypeId::of::<datum::Array<i32>>(), String::from("integer[]"));
    m.insert(TypeId::of::<Vec<i32>>(), String::from("integer[]"));
    m.insert(TypeId::of::<Vec<Option<i32>>>(), String::from("integer[]"));

    m.insert(TypeId::of::<i64>(), String::from("bigint"));
    m.insert(TypeId::of::<Option<i64>>(), String::from("bigint"));
    m.insert(TypeId::of::<datum::Array<i64>>(), String::from("bigint[]"));
    m.insert(TypeId::of::<Vec<i64>>(), String::from("bigint[]"));
    m.insert(TypeId::of::<Vec<Option<i64>>>(), String::from("bigint[]"));

    m.insert(TypeId::of::<bool>(), String::from("bool"));
    m.insert(TypeId::of::<Option<bool>>(), String::from("bool"));
    m.insert(TypeId::of::<datum::Array<bool>>(), String::from("bool[]"));
    m.insert(TypeId::of::<Vec<bool>>(), String::from("bool[]"));
    m.insert(TypeId::of::<Vec<Option<bool>>>(), String::from("bool[]"));

    m.insert(TypeId::of::<char>(), String::from("varchar"));
    m.insert(TypeId::of::<Option<char>>(), String::from("varchar"));
    m.insert(
        TypeId::of::<datum::Array<char>>(),
        String::from("varchar[]"),
    );
    m.insert(TypeId::of::<Vec<char>>(), String::from("varchar[]"));
    m.insert(TypeId::of::<Vec<Option<char>>>(), String::from("varchar[]"));

    m.insert(TypeId::of::<f32>(), String::from("real"));
    m.insert(TypeId::of::<Option<f32>>(), String::from("real"));
    m.insert(TypeId::of::<datum::Array<f32>>(), String::from("real[]"));
    m.insert(TypeId::of::<Vec<f32>>(), String::from("real[]"));
    m.insert(TypeId::of::<Vec<Option<f32>>>(), String::from("real[]"));

    m.insert(TypeId::of::<datum::JsonB>(), String::from("jsonb"));
    m.insert(TypeId::of::<Option<datum::JsonB>>(), String::from("jsonb"));
    m.insert(TypeId::of::<datum::Array<datum::JsonB>>(), String::from("jsonb[]"));
    m.insert(TypeId::of::<Vec<datum::JsonB>>(), String::from("jsonb[]"));
    m.insert(TypeId::of::<Vec<Option<datum::JsonB>>>(), String::from("jsonb[]"));

    m.insert(TypeId::of::<f64>(), String::from("double precision"));
    m.insert(
        TypeId::of::<Option<f64>>(),
        String::from("double precision"),
    );
    // TODO: Maybe????
    m.insert(
        TypeId::of::<datum::Array<f64>>(),
        String::from("double precision[]"),
    );
    m.insert(TypeId::of::<Vec<f64>>(), String::from("double precision[]"));
    m.insert(
        TypeId::of::<Vec<Option<f64>>>(),
        String::from("double precision[]"),
    );

    m.insert(TypeId::of::<&[u8]>(), String::from("bytea"));
    m.insert(TypeId::of::<Option<&[u8]>>(), String::from("bytea"));
    m.insert(TypeId::of::<Vec<u8>>(), String::from("bytea"));
    m.insert(TypeId::of::<Option<Vec<u8>>>(), String::from("bytea"));

    m.insert(TypeId::of::<pgx_pg_sys::ItemPointerData>(), String::from("tid"));
    m.insert(TypeId::of::<Vec<pgx_pg_sys::ItemPointerData>>(), String::from("tid[]"));

    m.insert(TypeId::of::<pgx_pg_sys::ItemPointerData>(), String::from("tid"));
    m.insert(TypeId::of::<Vec<pgx_pg_sys::ItemPointerData>>(), String::from("tid[]"));
    m.insert(TypeId::of::<datum::Array<pgx_pg_sys::ItemPointerData>>(), String::from("tid[]"));

    m.insert(TypeId::of::<pgx_pg_sys::Point>(), String::from("point"));
    m.insert(TypeId::of::<Vec<pgx_pg_sys::Point>>(), String::from("point[]"));
    m.insert(TypeId::of::<datum::Array<pgx_pg_sys::Point>>(), String::from("point[]"));

    m.insert(TypeId::of::<pgx_pg_sys::BOX>(), String::from("box"));
    m.insert(TypeId::of::<Vec<pgx_pg_sys::BOX>>(), String::from("box[]"));
    m.insert(TypeId::of::<datum::Array<pgx_pg_sys::BOX>>(), String::from("box[]"));

    m.insert(TypeId::of::<Date>(), String::from("date"));
    m.insert(TypeId::of::<Option<Date>>(), String::from("date"));
    m.insert(TypeId::of::<Time>(), String::from("time"));
    m.insert(TypeId::of::<Option<Time>>(), String::from("time"));
    m.insert(TypeId::of::<Timestamp>(), String::from("timestamp"));
    m.insert(TypeId::of::<Option<Timestamp>>(), String::from("timestamp"));
    m.insert(TypeId::of::<TimeWithTimeZone>(), String::from("time with time zone"));
    m.insert(TypeId::of::<Option<TimeWithTimeZone>>(), String::from("time with time zone"));
    m.insert(TypeId::of::<TimestampWithTimeZone>(), String::from("timestamp with time zone"));
    m.insert(TypeId::of::<Option<TimestampWithTimeZone>>(), String::from("timestamp with time zone"));

    m.insert(TypeId::of::<datum::Json>(), String::from("json"));
    m.insert(TypeId::of::<Vec<datum::Json>>(), String::from("json[]"));
    m.insert(TypeId::of::<Option<datum::Json>>(), String::from("json"));
    m.insert(TypeId::of::<Vec<datum::Json>>(), String::from("json[]"));
    m.insert(TypeId::of::<Option<Vec<datum::Json>>>(), String::from("json[]"));
    m.insert(TypeId::of::<Vec<Option<datum::Json>>>(), String::from("json[]"));
    m.insert(TypeId::of::<Option<Vec<Option<datum::Json>>>>(), String::from("json[]"));

    m.insert(TypeId::of::<datum::JsonB>(), String::from("jsonb"));
    m.insert(TypeId::of::<Vec<datum::JsonB>>(), String::from("jsonb[]"));
    m.insert(TypeId::of::<Option<datum::JsonB>>(), String::from("jsonb"));
    m.insert(TypeId::of::<Vec<datum::JsonB>>(), String::from("jsonb[]"));
    m.insert(TypeId::of::<Option<Vec<datum::JsonB>>>(), String::from("jsonb[]"));
    m.insert(TypeId::of::<Vec<Option<datum::JsonB>>>(), String::from("jsonb[]"));
    m.insert(TypeId::of::<Option<Vec<Option<datum::JsonB>>>>(), String::from("jsonb[]"));

    m.insert(TypeId::of::<datum::Internal<pgx_pg_sys::PlannerInfo>>(), String::from("internal"));
    m.insert(TypeId::of::<datum::Internal<pgx_pg_sys::List>>(), String::from("internal"));
    m.insert(TypeId::of::<pgbox::PgBox<pgx_pg_sys::IndexAmRoutine>>(), String::from("internal"));
    m.insert(TypeId::of::<pgx_pg_sys::ItemPointerData>(), String::from("internal"));
    m.insert(TypeId::of::<Option<pgx_pg_sys::ItemPointerData>>(), String::from("internal"));

    m.insert(TypeId::of::<datum::Numeric>(), String::from("pg_catalog.\"numeric\""));
    m.insert(TypeId::of::<Option<datum::Numeric>>(), String::from("pg_catalog.\"numeric\""));
    m.insert(TypeId::of::<Vec<datum::Numeric>>(), String::from("pg_catalog.\"numeric\"[]"));
    m.insert(TypeId::of::<Vec<Option<datum::Numeric>>>(), String::from("pg_catalog.\"numeric\"[]"));
    m.insert(TypeId::of::<Option<Vec<Option<datum::Numeric>>>>(), String::from("pg_catalog.\"numeric\"[]"));


    m.insert(TypeId::of::<pg_sys::Oid>(), String::from("oid"));
    m.insert(TypeId::of::<datum::AnyElement>(), String::from("anyelement"));
    m.insert(TypeId::of::<datum::AnyArray>(), String::from("anyarray"));

    m.insert(TypeId::of::<rel::PgRelation>(), String::from("regclass"));
    m.insert(TypeId::of::<Option<rel::PgRelation>>(), String::from("regclass"));

    m
});

/// A macro for marking a library compatible with the Postgres extension framework.
///
/// This macro was initially inspired from the `pg_module` macro in https://github.com/thehydroimpulse/postgres-extension.rs
///
/// Shamelessly cribbed from https://github.com/bluejekyll/pg-extend-rs
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

            #[cfg(not(feature = "pg13"))]
            const MY_MAGIC: pgx::pg_sys::Pg_magic_struct = pgx::pg_sys::Pg_magic_struct {
                len: size_of::<pgx::pg_sys::Pg_magic_struct>() as c_int,
                version: pgx::pg_sys::PG_VERSION_NUM as std::os::raw::c_int / 100,
                funcmaxargs: pgx::pg_sys::FUNC_MAX_ARGS as c_int,
                indexmaxkeys: pgx::pg_sys::INDEX_MAX_KEYS as c_int,
                namedatalen: pgx::pg_sys::NAMEDATALEN as c_int,
                float4byval: pgx::pg_sys::USE_FLOAT4_BYVAL as c_int,
                float8byval: pgx::pg_sys::USE_FLOAT8_BYVAL as c_int,
            };

            #[cfg(feature = "pg13")]
            const MY_MAGIC: pgx::pg_sys::Pg_magic_struct = pgx::pg_sys::Pg_magic_struct {
                len: size_of::<pgx::pg_sys::Pg_magic_struct>() as c_int,
                version: pgx::pg_sys::PG_VERSION_NUM as std::os::raw::c_int / 100,
                funcmaxargs: pgx::pg_sys::FUNC_MAX_ARGS as c_int,
                indexmaxkeys: pgx::pg_sys::INDEX_MAX_KEYS as c_int,
                namedatalen: pgx::pg_sys::NAMEDATALEN as c_int,
                float8byval: pgx::pg_sys::USE_FLOAT8_BYVAL as c_int,
            };

            // go ahead and register our panic handler since Postgres
            // calls this function first
            pgx::initialize();

            // return the magic
            &MY_MAGIC
        }

        pub use __pgx_internals::generate_sql;
        mod __pgx_internals {
            use ::pgx_utils::pg_inventory::{*, once_cell::sync::Lazy};
            use ::core::{any::TypeId, convert::TryFrom};
            use ::std::collections::HashMap;
            use ::pgx::datum::{FromDatum, PgVarlena, Array};

            static CONTROL_FILE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", env!("CARGO_CRATE_NAME"), ".control"));
            static LOAD_ORDER_FILE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/sql/load-order.txt"));
            static LOAD_ORDER_DIR: include_dir::Dir = include_dir::include_dir!("sql");

            #[derive(Debug)]
            pub struct ExtensionSql(pub pgx_utils::pg_inventory::ExtensionSql);
            inventory::collect!(ExtensionSql);

            #[derive(Debug)]
            pub struct PostgresType(pub pgx_utils::pg_inventory::InventoryPostgresType);
            inventory::collect!(PostgresType);

            #[derive(Debug)]
            pub struct PgExtern(pub pgx_utils::pg_inventory::InventoryPgExtern);
            inventory::collect!(PgExtern);

            #[derive(Debug)]
            pub struct PostgresEnum(pub pgx_utils::pg_inventory::InventoryPostgresEnum);
            inventory::collect!(PostgresEnum);

            #[derive(Debug)]
            pub struct PostgresHash(pub pgx_utils::pg_inventory::InventoryPostgresHash);
            inventory::collect!(PostgresHash);

            #[derive(Debug)]
            pub struct PostgresOrd(pub pgx_utils::pg_inventory::InventoryPostgresOrd);
            inventory::collect!(PostgresOrd);

            #[derive(Debug)]
            pub struct Schema(pub pgx_utils::pg_inventory::InventorySchema);
            inventory::collect!(Schema);

            #[derive(Debug)]
            pub enum LoadOrderError {
                NoListing,
                Missing(&'static str),
            }

            impl ::std::fmt::Display for LoadOrderError {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        LoadOrderError::NoListing => write!(f, "No `load-order.txt` file found or empty."),
                        LoadOrderError::Missing(field) => write!(f, "File from `load-order.txt` does not exist: `{}`.", field),
                    }
                }
            }

            impl ::std::error::Error for LoadOrderError {}

            pub fn generate_sql() -> pgx_utils::pg_inventory::eyre::Result<PgxSql<'static>> {
                use std::fmt::Write;
                let mut generated = PgxSql {
                    load_order: {
                        let mut mapping = HashMap::default();
                        let listing = LOAD_ORDER_DIR.get_file("load-order.txt").ok_or(LoadOrderError::NoListing)?;
                        let listing_str = listing.contents_utf8().ok_or(LoadOrderError::NoListing)?;
                        for item in listing_str.lines() {
                            let item_content = LOAD_ORDER_DIR.get_file(item).ok_or(LoadOrderError::Missing(item))?;
                            let item_str = item_content.contents_utf8().unwrap_or_default();
                            mapping.insert(item, item_str);
                        }
                        mapping
                    },
                    control: ControlFile::try_from(CONTROL_FILE)?,
                    type_mappings: pgx::DEFAULT_TYPEID_SQL_MAPPING.clone(),
                    schemas: inventory::iter::<Schema>().map(|i| (i.0.module_path, &i.0)).collect(),
                    extension_sql: inventory::iter::<ExtensionSql>().map(|i| (i.0.full_path, &i.0)).collect(),
                    externs: inventory::iter::<PgExtern>().map(|i| (i.0.full_path, &i.0)).collect(),
                    types: inventory::iter::<PostgresType>().map(|i| (i.0.full_path, &i.0)).collect(),
                    enums: inventory::iter::<PostgresEnum>().map(|i| (i.0.full_path, &i.0)).collect(),
                    hashes: inventory::iter::<PostgresHash>().map(|i| (i.0.full_path, &i.0)).collect(),
                    ords: inventory::iter::<PostgresOrd>().map(|i| (i.0.full_path, &i.0)).collect(),
                };
                generated.register_types();

                Ok(generated)
            }
        }
    };
}
