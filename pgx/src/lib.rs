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

        mod __pgx_internals {
            #[derive(Debug)]
            pub struct PgxSchema {
                pub externs: Vec<&'static PgxExtern>,
                pub types: Vec<&'static PgxPostgresType>,
                pub enums: Vec<&'static PgxPostgresEnum>,
                pub ords: Vec<&'static PgxPostgresOrd>,
                pub hashes: Vec<&'static PgxPostgresHash>
            }

            impl PgxSchema {
                pub fn to_sql(&self) -> String {
                    format!("\
                            {enums}\n\
                            {shell_types}\n\
                            {externs_with_operators}\n\
                            {materialized_types}\n\
                            {operator_classes}\n\
                        ",
                        enums = self.enums(),
                        shell_types = self.shell_types(),
                        externs_with_operators = self.externs_with_operators(),
                        materialized_types = self.materialized_types(),
                        operator_classes = self.operator_classes(),
                    )
                }

                fn enums(&self) -> String {
                    self.enums.iter().map(|en| {
                        format!("\n\
                                -- {file}\n\
                                -- {full_path}\n\
                                -- {id:?}\n\
                                CREATE TYPE {name} AS ENUM (\n\
                                    {variants}\
                                )\n\
                            ",
                            full_path = en.full_path,
                            file = en.file,
                            id = en.id,
                            name = en.name,
                            variants = en.variants.iter().map(|variant| format!("\t'{}',\n", variant)).collect::<String>(),
                        )
                    }).by_ref().collect()
                }

                fn shell_types(&self) -> String {
                    self.types.iter().map(|ty| {
                        format!("\n\
                                -- {file}\n\
                                -- {full_path}\n\
                                -- {id:?}\n\
                                CREATE TYPE {name};\n\
                            ",
                            full_path = ty.full_path,
                            file = ty.file,
                            id = ty.id,
                            name = ty.name,
                        )
                    }).by_ref().collect()
                }

                fn externs_with_operators(&self) -> String {
                    use crate::__pgx_internals::PgxExternReturn;
                    self.externs.iter().map(|ext| {
                        let ext_sql = format!("\n\
                                -- {file}\n\
                                -- {module_path}::{name}\n\
                                CREATE OR REPLACE FUNCTION \"{name}\"({arguments}) {returns} {extern_attrs} LANGUAGE c AS 'MODULE_PATHNAME', '{name}';\n\
                            ",
                            name = ext.name,
                            module_path = ext.module_path,
                            file = ext.file,
                            arguments = ext.fn_args.iter().map(|arg|
                                format!("\"{}\" {}", arg.pattern, pgx::type_id_to_sql_type(arg.ty_id).unwrap_or(arg.ty_name))
                            ).collect::<Vec<_>>().join(", "),
                            returns = match &ext.fn_return {
                                PgxExternReturn::None => String::default(),
                                PgxExternReturn::Type { id, name } => format!("RETURNS {}", pgx::type_id_to_sql_type(*id).unwrap_or(name)),
                                PgxExternReturn::Iterated(vec) => format!("RETURNS TABLE ({})",
                                    vec.iter().map(|(id, ty_name, col_name)| format!("\"{}\" {}", col_name.unwrap(), pgx::type_id_to_sql_type(*id).unwrap_or(ty_name))).collect::<Vec<_>>().join(", ")
                                ),
                            },
                            extern_attrs = ext.extern_attrs.iter().map(|attr| format!(" {:?} ", attr)).collect::<String>(),
                        );
                        match &ext.operator {
                            Some(op) => {
                                let mut optionals = vec![];
                                if let Some(it) = op.commutator {
                                    optionals.push(format!("\tCOMMUTATOR = {}", it));
                                };
                                if let Some(it) = op.negator {
                                    optionals.push(format!("\tNEGATOR = {}", it));
                                };
                                if let Some(it) = op.restrict {
                                    optionals.push(format!("\tRESTRICT = {}", it));
                                };
                                if let Some(it) = op.join {
                                    optionals.push(format!("\tJOIN = {}", it));
                                };
                                if op.hashes {
                                    optionals.push(String::from("\tHASHES"));
                                };
                                if op.merges {
                                    optionals.push(String::from("\tMERGES"));
                                };
                                let operator_sql = format!("\n\
                                        -- {file}\n\
                                        -- {module_path}::{name}\n\
                                        CREATE OPERATOR {opname} (\n\
                                            \tPROCEDURE=\"{name},\"\n\
                                            \tLEFTARG={left_arg},\n\
                                            \tRIGHTARG={right_arg}\n\
                                            {optionals}\n\
                                        )\n\
                                    ",
                                    opname = op.opname.unwrap(),
                                    file = ext.file,
                                    name = ext.name,
                                    module_path = ext.module_path,
                                    left_arg = ext.fn_args.get(0).unwrap().ty_name,
                                    right_arg = ext.fn_args.get(1).unwrap().ty_name,
                                    optionals = optionals.join(",\n")
                                );
                                ext_sql + &operator_sql
                            },
                            None => ext_sql,
                        }
                    }).by_ref().collect()
                }

                fn materialized_types(&self) -> String {
                    self.types.iter().map(|ty| {
                        format!("\n\
                                -- {file}\n\
                                -- {full_path}\n\
                                -- {id:?}\n\
                                CREATE TYPE {name} (\n\
                                    \tINTERNALLENGTH = variable,\n\
                                    \tINPUT = {in_fn},\n\
                                    \tOUTPUT = {out_fn},\n\
                                    \tSTORAGE = extended\n\
                                );\n\
                            ",
                            full_path = ty.full_path,
                            file = ty.file,
                            id = ty.id,
                            name = ty.name,
                            in_fn = ty.in_fn,
                            out_fn = ty.out_fn,
                        )
                    }).by_ref().collect()
                }

                fn operator_classes(&self) -> String {
                    let hashes = self.hashes.iter().map(|hash_derive| {
                        format!("\n\
                            -- {file}\n\
                            -- {full_path}\n\
                            -- {id:?}\n\
                            CREATE OPERATOR FAMILY {name}_hash_ops USING hash;\n\
                            CREATE OPERATOR CLASS {name}_hash_ops DEFAULT FOR TYPE {name} USING hash FAMILY {name}_hash_ops AS\n\
                                \tOPERATOR    1   =  ({name}, {name}),\n\
                                \tFUNCTION    1   {name}_hash({name});\n\
                            ",
                            name = hash_derive.name,
                            full_path = hash_derive.full_path,
                            file = hash_derive.file,
                            id = hash_derive.id,
                        )
                    }).collect::<String>();
                    let ords = self.ords.iter().map(|ord_derive| {
                        format!("\n\
                            -- {file}\n\
                            -- {full_path}\n\
                            -- {id:?}\n\
                            CREATE OPERATOR FAMILY {name}_btree_ops USING btree;\n\
                            CREATE OPERATOR CLASS {name}_btree_ops DEFAULT FOR TYPE {name} USING btree FAMILY {name}_btree_ops AS\n\
                                  \tOPERATOR 1 < ,\n\
                                  \tOPERATOR 2 <= ,\n\
                                  \tOPERATOR 3 = ,\n\
                                  \tOPERATOR 4 >= ,\n\
                                  \tOPERATOR 5 > ,\n\
                                  \tFUNCTION 1 {name}_cmp({name}, {name});\n\
                            ",
                            name = ord_derive.name,
                            full_path = ord_derive.full_path,
                            file = ord_derive.file,
                            id = ord_derive.id,
                        )
                    }).collect::<String>();
                    hashes + &ords
                }
            }

            #[derive(Debug)]
            pub struct PgxPostgresType {
                pub name: &'static str,
                pub file: &'static str,
                pub full_path: &'static str,
                pub id: core::any::TypeId,
                pub in_fn: &'static str,
                pub out_fn: &'static str,
            }
            pgx::inventory::collect!(PgxPostgresType);

            #[derive(Debug)]
            pub struct PgxOperator {
                pub opname: Option<&'static str>,
                pub commutator: Option<&'static str>,
                pub negator: Option<&'static str>,
                pub restrict: Option<&'static str>,
                pub join: Option<&'static str>,
                pub hashes: bool,
                pub merges: bool,
            }

            #[derive(Debug)]
            pub struct PgxExtern {
                pub name: &'static str,
                pub file: &'static str,
                pub module_path: &'static str,
                pub extern_attrs: Vec<pgx_utils::ExternArgs>,
                pub search_path: Option<Vec<&'static str>>,
                pub fn_args: Vec<PgxExternInputs>,
                pub fn_return: PgxExternReturn,
                pub operator: Option<PgxOperator>,
            }
            pgx::inventory::collect!(PgxExtern);

            #[derive(Debug)]
            pub struct PgxPostgresEnum {
                pub name: &'static str,
                pub file: &'static str,
                pub full_path: &'static str,
                pub id: core::any::TypeId,
                pub variants: Vec<&'static str>,
            }
            pgx::inventory::collect!(PgxPostgresEnum);

            #[derive(Debug)]
            pub enum PgxExternReturn {
                None,
                Type {
                    id: core::any::TypeId,
                    name: &'static str,
                },
                Iterated(Vec<(core::any::TypeId, &'static str, Option<&'static str>)>),
            }

            #[derive(Debug)]
            pub struct PgxExternInputs {
                pub pattern: &'static str,
                pub ty_id: core::any::TypeId,
                pub ty_name: &'static str,
                pub default: Option<&'static str>,
            }

            #[derive(Debug)]
            pub struct PgxPostgresHash {
                pub name: &'static str,
                pub file: &'static str,
                pub full_path: &'static str,
                pub id: core::any::TypeId,
            }
            pgx::inventory::collect!(PgxPostgresHash);

            #[derive(Debug)]
            pub struct PgxPostgresOrd {
                pub name: &'static str,
                pub file: &'static str,
                pub full_path: &'static str,
                pub id: core::any::TypeId,
            }
            pgx::inventory::collect!(PgxPostgresOrd);
        }

        pub fn generate_meta() -> crate::__pgx_internals::PgxSchema {
            use std::fmt::Write;
            let mut generated_sql = crate::__pgx_internals::PgxSchema {
                externs: pgx::inventory::iter::<crate::__pgx_internals::PgxExtern>().collect(),
                types: pgx::inventory::iter::<crate::__pgx_internals::PgxPostgresType>().collect(),
                enums: pgx::inventory::iter::<crate::__pgx_internals::PgxPostgresEnum>().collect(),
                hashes: pgx::inventory::iter::<crate::__pgx_internals::PgxPostgresHash>().collect(),
                ords: pgx::inventory::iter::<crate::__pgx_internals::PgxPostgresOrd>().collect(),
            };

            generated_sql
        }

        #[no_mangle]
        pub extern "C" fn alloc_meta() -> *mut std::os::raw::c_char {
            let schema = self::generate_meta();
            let schema_str = format!("{:?}", schema);
            let c_str = std::ffi::CString::new(schema_str).expect("Could not build CString.");
            c_str.into_raw()
        }
    };
}

/// Top-level initialization function.  This is called automatically by the `pg_module_magic!()`
/// macro and need not be called directly
#[allow(unused)]
pub fn initialize() {
    register_pg_guard_panic_handler();
}

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{RwLock, Arc};
use core::any::TypeId;

static TYPEID_SQL_BIMAP: Lazy<Arc<RwLock<HashMap<TypeId, &'static str>>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(TypeId::of::<&'static str>(), "text");
    m.insert(TypeId::of::<&std::ffi::CStr>(), "cstring");
    m.insert(TypeId::of::<String>(), "text");
    m.insert(TypeId::of::<()>(), "void");
    m.insert(TypeId::of::<i8>(), "\"char\"");
    m.insert(TypeId::of::<i16>(), "smallint");
    m.insert(TypeId::of::<i32>(), "integer");
    m.insert(TypeId::of::<i64>(), "bigint");
    m.insert(TypeId::of::<bool>(), "bool");
    m.insert(TypeId::of::<char>(), "varchar");
    m.insert(TypeId::of::<f32>(), "real");
    m.insert(TypeId::of::<f64>(), "double precision");
    m.insert(TypeId::of::<&[u8]>(), "bytea");
    m.insert(TypeId::of::<Vec<u8>>(), "bytea");
    Arc::from(RwLock::from(m))
});
pub fn type_id_to_sql_type(id: TypeId) -> Option<&'static str> {
    TYPEID_SQL_BIMAP.read().unwrap().get(&id).map(|f| *f)
}
pub fn insert_mapping(id: TypeId, sql: &'static str) {
    TYPEID_SQL_BIMAP.write().unwrap().insert(id, sql);
}