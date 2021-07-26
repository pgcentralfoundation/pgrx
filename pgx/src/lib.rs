// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

//! `pgx` is a framework for creating Postgres extensions in 100% Rust
//!
//! ## Example
//!
//! ```rust,ignore
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
use pgx_utils::pg_inventory::RustSqlMapping;
use std::collections::HashMap;

/// Top-level initialization function.  This is called automatically by the `pg_module_magic!()`
/// macro and need not be called directly
#[allow(unused)]
pub fn initialize() {
    register_pg_guard_panic_handler();
}

macro_rules! map_type {
    ($map:ident, $rust:ty, $sql:expr) => {{
        <$rust as WithTypeIds>::register_with_refs(&mut $map, $sql.to_string());
        WithSizedTypeIds::<$rust>::register_sized_with_refs(&mut $map, $sql.to_string());
        WithArrayTypeIds::<$rust>::register_array_with_refs(&mut $map, $sql.to_string());
        WithVarlenaTypeIds::<$rust>::register_varlena_with_refs(&mut $map, $sql.to_string());
    }};
}

pub static DEFAULT_TYPEID_SQL_MAPPING: Lazy<HashMap<TypeId, RustSqlMapping>> = Lazy::new(|| {
    let mut m = HashMap::new();

    // `str` isn't sized, so we can't lean on the macro.
    <str as WithTypeIds>::register(&mut m, "text".to_string());
    map_type!(m, &str, "text");

    // Bytea is a special case, notice how it has no `bytea[]`.
    m.insert(
        TypeId::of::<&[u8]>(),
        RustSqlMapping {
            sql: String::from("bytea"),
            id: TypeId::of::<&[u8]>(),
            rust: core::any::type_name::<&[u8]>().to_string(),
        },
    );
    m.insert(
        TypeId::of::<Option<&[u8]>>(),
        RustSqlMapping {
            sql: String::from("bytea"),
            id: TypeId::of::<Option<&[u8]>>(),
            rust: core::any::type_name::<Option<&[u8]>>().to_string(),
        },
    );
    m.insert(
        TypeId::of::<Vec<u8>>(),
        RustSqlMapping {
            sql: String::from("bytea"),
            id: TypeId::of::<Vec<u8>>(),
            rust: core::any::type_name::<Vec<u8>>().to_string(),
        },
    );
    m.insert(
        TypeId::of::<Option<Vec<u8>>>(),
        RustSqlMapping {
            sql: String::from("bytea"),
            id: TypeId::of::<Option<Vec<u8>>>(),
            rust: core::any::type_name::<Option<Vec<u8>>>().to_string(),
        },
    );

    map_type!(m, String, "text");
    map_type!(m, &std::ffi::CStr, "cstring");
    map_type!(m, (), "void");
    map_type!(m, i8, "\"char\"");
    map_type!(m, i16, "smallint");
    map_type!(m, i32, "integer");
    map_type!(m, i64, "bigint");
    map_type!(m, bool, "bool");
    map_type!(m, char, "varchar");
    map_type!(m, f32, "real");
    map_type!(m, f64, "double precision");
    map_type!(m, datum::JsonB, "jsonb");
    map_type!(m, datum::Json, "json");
    map_type!(m, pgx_pg_sys::ItemPointerData, "tid");
    map_type!(m, pgx_pg_sys::Point, "point");
    map_type!(m, pgx_pg_sys::BOX, "box");
    map_type!(m, Date, "date");
    map_type!(m, Time, "time");
    map_type!(m, TimeWithTimeZone, "time with time zone");
    map_type!(m, Timestamp, "timestamp");
    map_type!(m, TimestampWithTimeZone, "timestamp with time zone");
    map_type!(m, pgx_pg_sys::PlannerInfo, "internal");
    map_type!(m, datum::Internal<pgx_pg_sys::PlannerInfo>, "internal");
    map_type!(m, datum::Internal<pgx_pg_sys::List>, "internal");
    map_type!(m, pgbox::PgBox<pgx_pg_sys::IndexAmRoutine>, "internal");
    map_type!(m, rel::PgRelation, "regclass");
    map_type!(m, datum::Numeric, "numeric");
    map_type!(m, pg_sys::Oid, "oid");
    map_type!(m, datum::AnyElement, "anyelement");
    map_type!(m, datum::AnyArray, "anyarray");
    map_type!(m, datum::Inet, "inet");

    m
});

/// A macro for marking a library compatible with [`pgx`][crate].
/// 
/// <div class="example-wrap" style="display:inline-block">
/// <pre class="ignore" style="white-space:normal;font:inherit;">
///
/// **Note**: Every [`pgx`][crate] extension **must** have this macro called at top level (usually `src/lib.rs`) to be valid.
///
/// </pre></div>
///
/// This calls both [`pg_magic_func!()`](pg_magic_func) and [`pg_inventory_magic!()`](pg_inventory_magic).
#[macro_export]
macro_rules! pg_module_magic {
    () => {

        $crate::pg_magic_func!();
        $crate::pg_inventory_magic!();
    };
}

/// Create the `Pg_magic_func` required by PGX in extensions.
///
/// <div class="example-wrap" style="display:inline-block">
/// <pre class="ignore" style="white-space:normal;font:inherit;">
///
/// **Note**: Using [`pg_module_magic!()`](pg_module_magic) results in this macro being called.
/// Generally this macro should only be directly called in advanced use cases.
///
/// </pre></div>
///
/// This macro was initially inspired from the `pg_module` macro in [`thehydroimpulse/postgres-extension.rs`](https://github.com/thehydroimpulse/postgres-extension.rs)
///
/// Shamelessly cribbed from [`bluejekyll/pg-extend-rs`](https://github.com/bluejekyll/pg-extend-rs).
#[macro_export]
macro_rules! pg_magic_func {
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
    }
}

/// Create neccessary extension-local internal types for use with SQL generation.
///
/// <div class="example-wrap" style="display:inline-block">
/// <pre class="ignore" style="white-space:normal;font:inherit;">
///
/// **Note**: Using [`pg_module_magic`] results in this macro being called.
/// Generally this macro should only be directly called in advanced use cases.
///
/// </pre></div>
#[macro_export]
macro_rules! pg_inventory_magic {
    () => {
        /// A module containing [`pgx`] internals.
        ///
        /// This is created by [`macro@pgx::pg_module_magic`] (or, in rare cases,
        /// [`macro@pgx::pg_inventory_magic`].)
        ///
        /// Most often, these are used by the [`macro@pgx::pg_binary_magic`] inside a
        /// `src/bin/sql-generator.rs`.
        ///
        /// <div class="example-wrap" style="display:inline-block">
        /// <pre class="ignore" style="white-space:normal;font:inherit;">
        ///
        /// **Note**: These should be considered [`pgx`] **internals**, they may
        /// change between versions without warning or documentation. While you 
        /// *may* use them, you are signing up for pain later. Please, open an
        /// issue about what you need instead.
        ///
        /// </pre></div>
        pub mod __pgx_internals {
            use ::core::convert::TryFrom;
            use ::pgx_utils::pg_inventory::*;

            /// The contents of the `*.control` file of the crate.
            static CONTROL_FILE: &str = include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/",
                env!("CARGO_CRATE_NAME"),
                ".control"
            ));

            /// A wrapper type used by [`pgx::extension_sql`] and [`pgx::extension_sql_file`].
            ///
            /// Required inside the extension so that we can use [`inventory`] and collect the 
            /// [`pgx_utils::pg_inventory::InventoryExtensionSql`] used in SQL generation.
            #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
            pub struct ExtensionSql(pub pgx_utils::pg_inventory::InventoryExtensionSql);
            inventory::collect!(ExtensionSql);

            /// A wrapper type used by [`#[derive(PostgresType)]`](derive@pgx::PostgresType).
            ///
            /// Required inside the extension so that we can use [`inventory`] and collect the 
            /// [`pgx_utils::pg_inventory::InventoryPostgresType`] used in SQL generation.
            #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
            pub struct PostgresType(pub pgx_utils::pg_inventory::InventoryPostgresType);
            inventory::collect!(PostgresType);

            /// A wrapper type used by [`#[pg_extern]`](pgx::pg_extern).
            ///
            /// Required inside the extension so that we can use [`inventory`] and collect the 
            /// [`pgx_utils::pg_inventory::InventoryPgExtern`] used in SQL generation.
            #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
            pub struct PgExtern(pub pgx_utils::pg_inventory::InventoryPgExtern);
            inventory::collect!(PgExtern);

            /// A wrapper type used by [`#[derive(PostgresEnum)]`](derive@pgx::PostgresEnum).
            ///
            /// Required inside the extension so that we can use [`inventory`] and collect the 
            /// [`pgx_utils::pg_inventory::InventoryPostgresEnum`] used in SQL generation.
            #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
            pub struct PostgresEnum(pub pgx_utils::pg_inventory::InventoryPostgresEnum);
            inventory::collect!(PostgresEnum);

            /// A wrapper type used by [`#[derive(PostgresHash)]`](derive@pgx::PostgresHash).
            ///
            /// Required inside the extension so that we can use [`inventory`] and collect the 
            /// [`pgx_utils::pg_inventory::InventoryPostgresHash`] used in SQL generation.
            #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
            pub struct PostgresHash(pub pgx_utils::pg_inventory::InventoryPostgresHash);
            inventory::collect!(PostgresHash);

            /// A wrapper type used by [`#[derive(PostgresOrd)]`](derive@pgx::PostgresOrd).
            ///
            /// Required inside the extension so that we can use [`inventory`] and collect the 
            /// [`pgx_utils::pg_inventory::InventoryPostgresOrd`] used in SQL generation.
            #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
            pub struct PostgresOrd(pub pgx_utils::pg_inventory::InventoryPostgresOrd);
            inventory::collect!(PostgresOrd);

            /// A wrapper type used by [`#[pg_schema]`](pgx::pg_schema).
            ///
            /// Required inside the extension so that we can use [`inventory`] and collect the 
            /// [`pgx_utils::pg_inventory::InventorySchema`] used in SQL generation.
            #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
            pub struct Schema(pub pgx_utils::pg_inventory::InventorySchema);
            inventory::collect!(Schema);

            /// Build the SQL generator using the inventories of the wrappers in this module.
            pub fn generate_sql<'a>() -> pgx_utils::pg_inventory::eyre::Result<PgxSql<'a>> {
                let generated = PgxSql::build(
                    ControlFile::try_from(CONTROL_FILE)?,
                    (*$crate::DEFAULT_TYPEID_SQL_MAPPING)
                        .iter()
                        .map(|(x, y)| (x.clone(), y.clone())),
                    {
                        let mut set = inventory::iter::<Schema>().collect::<Vec<_>>();
                        set.sort();
                        set.into_iter().map(|x| &x.0)
                    },
                    {
                        let mut set = inventory::iter::<ExtensionSql>().collect::<Vec<_>>();
                        set.sort();
                        set.into_iter().map(|x| &x.0)
                    },
                    {
                        let mut set = inventory::iter::<PgExtern>().collect::<Vec<_>>();
                        set.sort();
                        set.into_iter().map(|x| &x.0)
                    },
                    {
                        let mut set = inventory::iter::<PostgresType>().collect::<Vec<_>>();
                        set.sort();
                        set.into_iter().map(|x| &x.0)
                    },
                    {
                        let mut set = inventory::iter::<PostgresEnum>().collect::<Vec<_>>();
                        set.sort();
                        set.into_iter().map(|x| &x.0)
                    },
                    {
                        let mut set = inventory::iter::<PostgresOrd>().collect::<Vec<_>>();
                        set.sort();
                        set.into_iter().map(|x| &x.0)
                    },
                    {
                        let mut set = inventory::iter::<PostgresHash>().collect::<Vec<_>>();
                        set.sort();
                        set.into_iter().map(|x| &x.0)
                    },
                )?;

                Ok(generated)
            }
        }
    };
}

#[macro_export]
macro_rules! pg_binary_magic {
    ($($prelude:ident)::*) => {
        fn main() -> ::pgx_utils::pg_inventory::color_eyre::Result<()> {
            use ::pgx_utils::pg_inventory::{
                tracing_error::ErrorLayer,
                tracing,
                tracing_subscriber::{self, util::SubscriberInitExt, layer::SubscriberExt, EnvFilter},
                color_eyre,
                eyre,
            };
            use std::env;
            use $($prelude :: )*__pgx_internals::generate_sql;

            // Initialize tracing with tracing-error.
            let fmt_layer = tracing_subscriber::fmt::layer().with_target(false);
            let filter_layer = EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("info"))
                .unwrap();
            tracing_subscriber::registry()
                .with(filter_layer)
                .with(fmt_layer)
                .with(ErrorLayer::default())
                .init();

            color_eyre::install()?;

            let mut args = env::args().skip(1);
            let path = args.next().unwrap_or(concat!("./sql/", stringify!($($prelude :: )*), ".sql").into());
            let dot: Option<String> = args.next();
            if args.next().is_some() {
                return Err(eyre::eyre!("Only accepts two arguments, the destination path, and an optional (GraphViz) dot output path."));
            }

            tracing::info!(path = %path, "Writing SQL.");
            let sql = generate_sql()?;
            sql.to_file(path)?;
            if let Some(dot) = dot {
                tracing::info!(dot = %dot, "Writing Graphviz DOT.");
                sql.to_dot(dot)?;
            }
            Ok(())
        }
    };
}
