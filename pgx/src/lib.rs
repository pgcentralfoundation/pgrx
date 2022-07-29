/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
//! `pgx` is a framework for creating Postgres extensions in 100% Rust
//!
//! ## Example
//!
//! ```rust
//! use pgx::*;
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

pub mod aggregate;
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
pub mod heap_tuple;
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
pub use once_cell;

pub use aggregate::*;
pub use atomics::*;
pub use callbacks::*;
pub use datum::*;
pub use enum_helper::*;
pub use fcinfo::*;
pub use guc::*;
pub use heap_tuple::*;
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
use utils::sql_entity_graph::metadata::SqlVariant;
use utils::sql_entity_graph::metadata::{ArgumentError, ReturnVariant, ReturnVariantError};
pub use varlena::*;
pub use wrappers::*;
pub use xid::*;

pub use pgx_pg_sys as pg_sys; // the module only, not its contents
pub use pgx_pg_sys::submodules::*;
pub use pgx_pg_sys::PgBuiltInOids; // reexport this so it looks like it comes from here

pub use cstr_core;
pub use pgx_utils as utils;

use core::any::TypeId;
use once_cell::sync::Lazy;
use std::collections::HashSet;
use std::panic::UnwindSafe;

use pgx_utils::sql_entity_graph::{RustSourceOnlySqlMapping, RustSqlMapping};

macro_rules! map_source_only {
    ($map:ident, $rust:ty, $sql:expr) => {{
        let ty = stringify!($rust).to_string().replace(" ", "");
        assert_eq!(
            $map.insert(RustSourceOnlySqlMapping::new(ty.clone(), $sql.to_string(),)),
            true,
            "Cannot map {} twice",
            ty
        );

        let ty = stringify!(Option<$rust>).to_string().replace(" ", "");
        assert_eq!(
            $map.insert(RustSourceOnlySqlMapping::new(ty.clone(), $sql.to_string(),)),
            true,
            "Cannot map {} twice",
            ty
        );

        let ty = stringify!(Vec<$rust>).to_string().replace(" ", "");
        assert_eq!(
            $map.insert(RustSourceOnlySqlMapping::new(
                ty.clone(),
                format!("{}[]", $sql),
            )),
            true,
            "Cannot map {} twice",
            ty
        );

        let ty = stringify!(Array<$rust>).to_string().replace(" ", "");
        assert_eq!(
            $map.insert(RustSourceOnlySqlMapping::new(
                ty.clone(),
                format!("{}[]", $sql),
            )),
            true,
            "Cannot map {} twice",
            ty
        );
    }};
}

pub static DEFAULT_RUST_SOURCE_TO_SQL: Lazy<HashSet<RustSourceOnlySqlMapping>> = Lazy::new(|| {
    let mut m = HashSet::new();

    map_source_only!(m, pg_sys::Oid, "Oid");
    map_source_only!(m, pg_sys::TimestampTz, "timestamp with time zone");

    m
});

macro_rules! map_type {
    ($map:ident, $rust:ty, $sql:expr) => {{
        <$rust as WithTypeIds>::register_with_refs(&mut $map, $sql.to_string());
        WithSizedTypeIds::<$rust>::register_sized_with_refs(&mut $map, $sql.to_string());
        WithArrayTypeIds::<$rust>::register_array_with_refs(&mut $map, $sql.to_string());
        WithVarlenaTypeIds::<$rust>::register_varlena_with_refs(&mut $map, $sql.to_string());
    }};
}

/// The default lookup for [`TypeId`]s to both Rust and SQL types via a [`RustSqlMapping`].
///
/// This only contains types known to [`pgx`](crate), so it will not include types defined by things
/// like [`derive@PostgresType`] in the local extension.
pub static DEFAULT_RUST_TYPE_ID_TO_SQL: Lazy<HashSet<RustSqlMapping>> = Lazy::new(|| {
    let mut m = HashSet::new();

    // `str` isn't sized, so we can't lean on the macro.
    <str as WithTypeIds>::register(&mut m, "text".to_string());
    map_type!(m, &str, "text");

    // Bytea is a special case, notice how it has no `bytea[]`.
    m.insert(RustSqlMapping {
        sql: String::from("bytea"),
        id: TypeId::of::<&[u8]>(),
        rust: core::any::type_name::<&[u8]>().to_string(),
    });
    m.insert(RustSqlMapping {
        sql: String::from("bytea"),
        id: TypeId::of::<Option<&[u8]>>(),
        rust: core::any::type_name::<Option<&[u8]>>().to_string(),
    });
    m.insert(RustSqlMapping {
        sql: String::from("bytea"),
        id: TypeId::of::<Vec<u8>>(),
        rust: core::any::type_name::<Vec<u8>>().to_string(),
    });
    m.insert(RustSqlMapping {
        sql: String::from("bytea"),
        id: TypeId::of::<Option<Vec<u8>>>(),
        rust: core::any::type_name::<Option<Vec<u8>>>().to_string(),
    });

    map_type!(m, String, "text");
    map_type!(m, &std::ffi::CStr, "cstring");
    map_type!(m, &crate::cstr_core::CStr, "cstring");
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
    map_type!(m, datum::Internal, "internal");
    map_type!(m, pgbox::PgBox<pgx_pg_sys::IndexAmRoutine>, "internal");
    map_type!(m, rel::PgRelation, "regclass");
    map_type!(m, datum::Numeric, "numeric");
    map_type!(m, datum::AnyElement, "anyelement");
    map_type!(m, datum::AnyArray, "anyarray");
    map_type!(m, datum::Inet, "inet");
    map_type!(m, datum::Uuid, "uuid");
    map_type!(m, pgx_pg_sys::FdwRoutine, "fdw_handler");

    m
});

/// The default lookup for if composite types are a collection or not
///
/// This is primarily used to determine if they need `[]` in SQL generation.
pub static DEFAULT_COMPOSITE_TYPE_COLLECTIONS: Lazy<std::collections::HashMap<TypeId, bool>> =
    Lazy::new(|| {
        let mut m = std::collections::HashMap::new();

        m.insert(TypeId::of::<PgHeapTuple<'static, AllocatedByRust>>(), false);
        m.insert(
            TypeId::of::<Vec<PgHeapTuple<'static, AllocatedByRust>>>(),
            true,
        );
        m.insert(
            TypeId::of::<Vec<Option<PgHeapTuple<'static, AllocatedByRust>>>>(),
            true,
        );
        m.insert(
            TypeId::of::<Array<PgHeapTuple<'static, AllocatedByRust>>>(),
            true,
        );
        m.insert(
            TypeId::of::<Array<Option<PgHeapTuple<'static, AllocatedByRust>>>>(),
            true,
        );
        m.insert(
            TypeId::of::<VariadicArray<PgHeapTuple<'static, AllocatedByRust>>>(),
            true,
        );
        m.insert(
            TypeId::of::<VariadicArray<Option<PgHeapTuple<'static, AllocatedByRust>>>>(),
            true,
        );
        m.insert(
            TypeId::of::<Option<PgHeapTuple<'static, AllocatedByRust>>>(),
            false,
        );
        m.insert(
            TypeId::of::<Option<Vec<PgHeapTuple<'static, AllocatedByRust>>>>(),
            true,
        );
        m.insert(
            TypeId::of::<Option<Vec<Option<PgHeapTuple<'static, AllocatedByRust>>>>>(),
            true,
        );
        m.insert(
            TypeId::of::<Option<Array<PgHeapTuple<'static, AllocatedByRust>>>>(),
            true,
        );
        m.insert(
            TypeId::of::<Option<Array<Option<PgHeapTuple<'static, AllocatedByRust>>>>>(),
            true,
        );
        m.insert(
            TypeId::of::<Option<VariadicArray<PgHeapTuple<'static, AllocatedByRust>>>>(),
            true,
        );
        m.insert(
            TypeId::of::<Option<VariadicArray<Option<PgHeapTuple<'static, AllocatedByRust>>>>>(),
            true,
        );

        m
    });
use pgx_utils::sql_entity_graph::metadata::SqlTranslatable;

pub struct SetOfIterator<'a, T>
where
    T: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    iter: Box<dyn Iterator<Item = T> + std::panic::UnwindSafe + std::panic::RefUnwindSafe + 'a>,
}

impl<'a, T> SetOfIterator<'a, T>
where
    T: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    pub fn new<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T> + std::panic::UnwindSafe + 'a,
        <I as IntoIterator>::IntoIter: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
    {
        Self {
            iter: Box::new(iter.into_iter()),
        }
    }
}

impl<'a, T> Iterator for SetOfIterator<'a, T>
where
    T: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, T> IntoDatum for SetOfIterator<'a, T>
where
    T: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn into_datum(self) -> Option<pg_sys::Datum> {
        todo!()
    }

    fn type_oid() -> pg_sys::Oid {
        todo!()
    }
}

impl<'a, T> SqlTranslatable for SetOfIterator<'a, T>
where
    T: SqlTranslatable + std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn argument_sql() -> Result<SqlVariant, ArgumentError> {
        T::argument_sql()
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        match T::return_sql() {
            Ok(ReturnVariant::Plain(sql)) => Ok(ReturnVariant::SetOf(sql)),
            Ok(ReturnVariant::SetOf(_)) => Err(ReturnVariantError::NestedSetOf),
            Ok(ReturnVariant::Table(_)) => Err(ReturnVariantError::SetOfContainingTable),
            err @ Err(_) => err,
        }
    }
}

pub struct TableIterator<'a, T>
where
    T: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    iter: Box<dyn Iterator<Item = T> + std::panic::UnwindSafe + std::panic::RefUnwindSafe + 'a>,
}

impl<'a, T> TableIterator<'a, T>
where
    T: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    pub fn new<I>(iter: I) -> Self
    where
        I: Iterator<Item = T> + std::panic::UnwindSafe + std::panic::RefUnwindSafe + 'a,
    {
        Self {
            iter: Box::new(iter),
        }
    }
}

impl<'a, T> Iterator for TableIterator<'a, T>
where
    T: std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a, T> IntoDatum for TableIterator<'a, T>
where
    T: SqlTranslatable + std::panic::UnwindSafe + std::panic::RefUnwindSafe,
{
    fn into_datum(self) -> Option<pg_sys::Datum> {
        todo!()
    }

    fn type_oid() -> pg_sys::Oid {
        todo!()
    }
}

seq_macro::seq!(I in 0..=32 {
    #(
        seq_macro::seq!(N in 0..=I {
            impl<'a, #(Input~N,)*> SqlTranslatable for TableIterator<'a, (#(Input~N,)*)>
            where
                #(
                    Input~N: SqlTranslatable + std::panic::UnwindSafe + std::panic::RefUnwindSafe + 'static,
                )*
            {
                fn argument_sql() -> Result<SqlVariant, ArgumentError> {
                    Err(ArgumentError::Table)
                }
                fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
                    let mut vec = Vec::new();
                    #(
                        vec.push(match Input~N::return_sql() {
                            Ok(ReturnVariant::Plain(sql)) => sql,
                            Ok(ReturnVariant::SetOf(_)) => return Err(ReturnVariantError::TableContainingSetOf),
                            Ok(ReturnVariant::Table(_)) => return Err(ReturnVariantError::NestedTable),
                            Err(err) => return Err(err),
                        });
                    )*
                    Ok(ReturnVariant::Table(vec))
                }
            }
        });
    )*
});

impl<T: SqlTranslatable> SqlTranslatable for crate::pgbox::PgBox<T, AllocatedByPostgres> {
    fn argument_sql() -> Result<SqlVariant, ArgumentError> {
        T::argument_sql()
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        T::return_sql()
    }
}

impl<T: SqlTranslatable> SqlTranslatable for crate::pgbox::PgBox<T, AllocatedByRust> {
    fn argument_sql() -> Result<SqlVariant, ArgumentError> {
        T::argument_sql()
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        T::return_sql()
    }
}

impl SqlTranslatable for crate::datum::Numeric {
    fn argument_sql() -> Result<SqlVariant, ArgumentError> {
        Ok(SqlVariant::Mapped(String::from("NUMERIC")))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlVariant::Mapped(String::from(
            "NUMERIC",
        ))))
    }
}

impl SqlTranslatable for crate::datum::Inet {
    fn argument_sql() -> Result<SqlVariant, ArgumentError> {
        Ok(SqlVariant::Mapped(String::from("inet")))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlVariant::Mapped(String::from(
            "inet",
        ))))
    }
}

impl SqlTranslatable for crate::datum::Json {
    fn argument_sql() -> Result<SqlVariant, ArgumentError> {
        Ok(SqlVariant::Mapped(String::from("json")))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlVariant::Mapped(String::from(
            "json",
        ))))
    }
}

impl SqlTranslatable for crate::datum::JsonB {
    fn argument_sql() -> Result<SqlVariant, ArgumentError> {
        Ok(SqlVariant::Mapped(String::from("jsonb")))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlVariant::Mapped(String::from(
            "jsonb",
        ))))
    }
}

impl SqlTranslatable for crate::datum::AnyArray {
    fn argument_sql() -> Result<SqlVariant, ArgumentError> {
        Ok(SqlVariant::Mapped(String::from("anyarray")))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlVariant::Mapped(String::from(
            "anyarray",
        ))))
    }
}

impl SqlTranslatable for crate::datum::Date {
    fn argument_sql() -> Result<SqlVariant, ArgumentError> {
        Ok(SqlVariant::Mapped(String::from("date")))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlVariant::Mapped(String::from(
            "date",
        ))))
    }
}

impl SqlTranslatable for crate::datum::Time {
    fn argument_sql() -> Result<SqlVariant, ArgumentError> {
        Ok(SqlVariant::Mapped(String::from("time")))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlVariant::Mapped(String::from(
            "time",
        ))))
    }
}

impl SqlTranslatable for crate::datum::TimeWithTimeZone {
    fn argument_sql() -> Result<SqlVariant, ArgumentError> {
        Ok(SqlVariant::Mapped(String::from("time with time zone")))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlVariant::Mapped(String::from(
            "time with time zone",
        ))))
    }
}

impl SqlTranslatable for crate::datum::Timestamp {
    fn argument_sql() -> Result<SqlVariant, ArgumentError> {
        Ok(SqlVariant::Mapped(String::from("timestamp")))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlVariant::Mapped(String::from(
            "timestamp",
        ))))
    }
}

impl SqlTranslatable for crate::datum::TimestampWithTimeZone {
    fn argument_sql() -> Result<SqlVariant, ArgumentError> {
        Ok(SqlVariant::Mapped(String::from("timestamp with time zone")))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlVariant::Mapped(String::from(
            "timestamp with time zone",
        ))))
    }
}

impl SqlTranslatable for crate::datum::Internal {
    fn argument_sql() -> Result<SqlVariant, ArgumentError> {
        Ok(SqlVariant::Mapped(String::from("internal")))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlVariant::Mapped(String::from(
            "internal",
        ))))
    }
    // We don't want to strict upgrade if internal is present.
    fn optional() -> bool {
        true
    }
}

impl<T> SqlTranslatable for crate::datum::PgVarlena<T>
where
    T: SqlTranslatable + Copy,
{
    fn argument_sql() -> Result<SqlVariant, ArgumentError> {
        T::argument_sql()
    }

    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        T::return_sql()
    }
}

// impl<const PRECISION: i32, const SCALE: i32>
//     SqlTranslatable
//     for crate::datum::Numeric<PRECISION, SCALE>
// {
//     fn sql_type() -> String {
//         if PRECISION == 0 && SCALE == 0 {
//             String::from("NUMERIC")
//         } else {
//             format!("NUMERIC({PRECISION}, {SCALE})")
//         }
//     }
// }

impl SqlTranslatable for crate::heap_tuple::PgHeapTuple<'static, AllocatedByPostgres> {
    fn argument_sql() -> Result<SqlVariant, ArgumentError> {
        Ok(SqlVariant::Composite {
            requires_array_brackets: false,
        })
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlVariant::Composite {
            requires_array_brackets: false,
        }))
    }
}

impl SqlTranslatable for crate::heap_tuple::PgHeapTuple<'static, AllocatedByRust> {
    fn argument_sql() -> Result<SqlVariant, ArgumentError> {
        Ok(SqlVariant::Composite {
            requires_array_brackets: false,
        })
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlVariant::Composite {
            requires_array_brackets: false,
        }))
    }
}

impl SqlTranslatable for crate::datum::Uuid {
    fn argument_sql() -> Result<SqlVariant, ArgumentError> {
        Ok(SqlVariant::Mapped(String::from("uuid")))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlVariant::Mapped(String::from(
            "uuid",
        ))))
    }
}

impl<'a, T> SqlTranslatable for Array<'a, T>
where
    T: SqlTranslatable + FromDatum,
{
    fn argument_sql() -> Result<SqlVariant, ArgumentError> {
        match T::argument_sql() {
            Ok(SqlVariant::Mapped(sql)) => Ok(SqlVariant::Mapped(format!("{sql}[]"))),
            Ok(SqlVariant::Skip) => Err(ArgumentError::SkipInArray),
            Ok(SqlVariant::Composite { .. }) => Ok(SqlVariant::Composite {
                requires_array_brackets: true,
            }),
            err @ Err(_) => err,
        }
    }

    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        match T::return_sql() {
            Ok(ReturnVariant::Plain(SqlVariant::Mapped(sql))) => {
                Ok(ReturnVariant::Plain(SqlVariant::Mapped(format!("{sql}[]"))))
            }
            Ok(ReturnVariant::Plain(SqlVariant::Composite {
                requires_array_brackets: _,
            })) => Ok(ReturnVariant::Plain(SqlVariant::Composite {
                requires_array_brackets: true,
            })),
            Ok(ReturnVariant::Plain(SqlVariant::Skip)) => Err(ReturnVariantError::SkipInArray),
            Ok(ReturnVariant::SetOf(_)) => Err(ReturnVariantError::SetOfInArray),
            Ok(ReturnVariant::Table(_)) => Err(ReturnVariantError::TableInArray),
            err @ Err(_) => err,
        }
    }
}

impl<'a, T> SqlTranslatable for VariadicArray<'a, T>
where
    T: SqlTranslatable + FromDatum,
{
    fn argument_sql() -> Result<SqlVariant, ArgumentError> {
        match T::argument_sql() {
            Ok(SqlVariant::Mapped(sql)) => Ok(SqlVariant::Mapped(format!("{sql}[]"))),
            Ok(SqlVariant::Skip) => Err(ArgumentError::SkipInArray),
            Ok(SqlVariant::Composite { .. }) => Ok(SqlVariant::Composite {
                requires_array_brackets: true,
            }),
            err @ Err(_) => err,
        }
    }

    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        match T::return_sql() {
            Ok(ReturnVariant::Plain(SqlVariant::Mapped(sql))) => {
                Ok(ReturnVariant::Plain(SqlVariant::Mapped(format!("{sql}[]"))))
            }
            Ok(ReturnVariant::Plain(SqlVariant::Composite {
                requires_array_brackets: _,
            })) => Ok(ReturnVariant::Plain(SqlVariant::Composite {
                requires_array_brackets: true,
            })),
            Ok(ReturnVariant::Plain(SqlVariant::Skip)) => Err(ReturnVariantError::SkipInArray),
            Ok(ReturnVariant::SetOf(_)) => Err(ReturnVariantError::SetOfInArray),
            Ok(ReturnVariant::Table(_)) => Err(ReturnVariantError::TableInArray),
            err @ Err(_) => err,
        }
    }

    fn variadic() -> bool {
        true
    }
}

/// A macro for marking a library compatible with [`pgx`][crate].
///
/// <div class="example-wrap" style="display:inline-block">
/// <pre class="ignore" style="white-space:normal;font:inherit;">
///
/// **Note**: Every [`pgx`][crate] extension **must** have this macro called at top level (usually `src/lib.rs`) to be valid.
///
/// </pre></div>
///
/// This calls both [`pg_magic_func!()`](pg_magic_func) and [`pg_sql_graph_magic!()`](pg_sql_graph_magic).
#[macro_export]
macro_rules! pg_module_magic {
    () => {
        $crate::pg_magic_func!();
        $crate::pg_sql_graph_magic!();
    };
}

/// Create the `Pg_magic_func` required by PGX in extensions.
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
#[macro_export]
macro_rules! pg_magic_func {
    () => {
        #[no_mangle]
        #[allow(non_snake_case)]
        #[allow(unused)]
        #[link_name = "Pg_magic_func"]
        #[doc(hidden)]
        pub extern "C" fn Pg_magic_func() -> &'static pgx::pg_sys::Pg_magic_struct {
            use core::mem::size_of;
            use pgx;

            #[cfg(any(feature = "pg10", feature = "pg11", feature = "pg12"))]
            const MY_MAGIC: pgx::pg_sys::Pg_magic_struct = pgx::pg_sys::Pg_magic_struct {
                len: size_of::<pgx::pg_sys::Pg_magic_struct>() as i32,
                version: pgx::pg_sys::PG_VERSION_NUM as i32 / 100,
                funcmaxargs: pgx::pg_sys::FUNC_MAX_ARGS as i32,
                indexmaxkeys: pgx::pg_sys::INDEX_MAX_KEYS as i32,
                namedatalen: pgx::pg_sys::NAMEDATALEN as i32,
                float4byval: pgx::pg_sys::USE_FLOAT4_BYVAL as i32,
                float8byval: pgx::pg_sys::USE_FLOAT8_BYVAL as i32,
            };

            #[cfg(any(feature = "pg13", feature = "pg14"))]
            const MY_MAGIC: pgx::pg_sys::Pg_magic_struct = pgx::pg_sys::Pg_magic_struct {
                len: size_of::<pgx::pg_sys::Pg_magic_struct>() as i32,
                version: pgx::pg_sys::PG_VERSION_NUM as i32 / 100,
                funcmaxargs: pgx::pg_sys::FUNC_MAX_ARGS as i32,
                indexmaxkeys: pgx::pg_sys::INDEX_MAX_KEYS as i32,
                namedatalen: pgx::pg_sys::NAMEDATALEN as i32,
                float8byval: pgx::pg_sys::USE_FLOAT8_BYVAL as i32,
            };

            // go ahead and register our panic handler since Postgres
            // calls this function first
            pgx::initialize();

            // return the magic
            &MY_MAGIC
        }
    };
}

/// Create neccessary extension-local internal marker for use with SQL generation.
///
/// <div class="example-wrap" style="display:inline-block">
/// <pre class="ignore" style="white-space:normal;font:inherit;">
///
/// **Note**: Generally [`pg_module_magic`] is preferred, and results in this macro being called.
/// This macro should only be directly called in advanced use cases.
///
/// </pre></div>
#[macro_export]
macro_rules! pg_sql_graph_magic {
    () => {
        #[no_mangle]
        #[doc(hidden)]
        pub extern "C" fn __pgx_sql_mappings() -> ::pgx::utils::sql_entity_graph::RustToSqlMapping {
            ::pgx::utils::sql_entity_graph::RustToSqlMapping {
                rust_type_id_to_sql: ::pgx::DEFAULT_RUST_TYPE_ID_TO_SQL.clone(),
                rust_source_to_sql: ::pgx::DEFAULT_RUST_SOURCE_TO_SQL.clone(),
                composite_type_collections: ::pgx::DEFAULT_COMPOSITE_TYPE_COLLECTIONS.clone(),
                internal_type: core::any::TypeId::of::<::pgx::Internal>(),
            }
        }

        // A marker which must exist in the root of the extension.
        #[no_mangle]
        #[doc(hidden)]
        pub extern "C" fn __pgx_marker(
        ) -> ::pgx::utils::__reexports::eyre::Result<::pgx::utils::sql_entity_graph::ControlFile> {
            use ::core::convert::TryFrom;
            use ::pgx::utils::__reexports::eyre::WrapErr;
            let package_version = env!("CARGO_PKG_VERSION");
            let context = include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/",
                env!("CARGO_CRATE_NAME"),
                ".control"
            ))
            .replace("@CARGO_VERSION@", package_version);

            let control_file =
                ::pgx::utils::sql_entity_graph::ControlFile::try_from(context.as_str())
                    .wrap_err_with(|| "Could not parse control file, is it valid?")?;
            Ok(control_file)
        }
    };
}

/// Initialize the extension with Postgres
///
/// Sets up panic handling with [`register_pg_guard_panic_hook()`] to ensure that a crash within
/// the extension does not adversely affect the entire server process.
///
/// ## Note
///
/// This is called automatically by the [`pg_module_magic!()`] macro and need not be called
/// directly.
#[allow(unused)]
pub fn initialize() {
    register_pg_guard_panic_hook();
}
