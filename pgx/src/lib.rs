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

        pub use __pgx_internals::PgxSchema;
        mod __pgx_internals {
            #[derive(Debug)]
            pub struct PgxSchema {
                pub extension_sql: Vec<&'static PgxExtensionSql>,
                pub externs: Vec<&'static PgxExtern>,
                pub types: Vec<&'static PgxPostgresType>,
                pub enums: Vec<&'static PgxPostgresEnum>,
                pub ords: Vec<&'static PgxPostgresOrd>,
                pub hashes: Vec<&'static PgxPostgresHash>
            }

            impl PgxSchema {
                pub fn generate() -> Self {
                    use std::fmt::Write;
                    let mut generated = Self {
                        extension_sql: pgx::inventory::iter::<PgxExtensionSql>().collect(),
                        externs: pgx::inventory::iter::<PgxExtern>().collect(),
                        types: pgx::inventory::iter::<PgxPostgresType>().collect(),
                        enums: pgx::inventory::iter::<PgxPostgresEnum>().collect(),
                        hashes: pgx::inventory::iter::<PgxPostgresHash>().collect(),
                        ords: pgx::inventory::iter::<PgxPostgresOrd>().collect(),
                    };
                    generated.register_types();

                    generated
                }

                pub fn to_file(&self, file: impl AsRef<str>) -> Result<(), Box<dyn std::error::Error>> {
                    use std::{fs::{File, create_dir_all}, path::Path, io::Write};
                    let generated = self.to_sql();
                    let path = Path::new(file.as_ref());
                    let parent = path.parent();
                    if let Some(parent) = parent {
                        create_dir_all(parent)?;
                    }
                    let mut out = File::create(path)?;
                    write!(out, "{}", generated)?;
                    Ok(())
                }

                pub fn to_sql(&self) -> String {
                    format!("\
                            -- This file is auto generated by pgx.
                            {extension_sql}\n\
                            {enums}\n\
                            {shell_types}\n\
                            {externs_with_operators}\n\
                            {materialized_types}\n\
                            {operator_classes}\n\
                        ",
                        extension_sql = self.extension_sql(),
                        enums = self.enums(),
                        shell_types = self.shell_types(),
                        externs_with_operators = self.externs_with_operators(),
                        materialized_types = self.materialized_types(),
                        operator_classes = self.operator_classes(),
                    )
                }

                fn extension_sql(&self) -> String {
                    self.extension_sql.iter().map(|fragment| {
                        format!("\
                                -- {file}:{line}\n\
                                {sql}\
                            ",
                            file = fragment.file,
                            line = fragment.line,
                            sql = fragment.sql,
                        )
                    }).collect::<Vec<_>>().join("\n")
                }

                fn enums(&self) -> String {
                    self.enums.iter().map(|en| {
                        format!("\
                                -- {file}:{line}\n\
                                -- {full_path} - {id:?}\n\
                                -- Option<{full_path}> - {option_id:?}\n\
                                -- Vec<{full_path}> - {vec_id:?}\n\
                                CREATE TYPE {name} AS ENUM (\n\
                                    {variants}\
                                );\
                            ",
                            full_path = en.full_path,
                            file = en.file,
                            line = en.line,
                            id = en.id,
                            option_id = en.option_id,
                            vec_id = en.vec_id,
                            name = en.name,
                            variants = en.variants.iter().map(|variant| format!("\t'{}',\n", variant)).collect::<String>(),
                        )
                    }).by_ref().collect()
                }

                fn shell_types(&self) -> String {
                    self.types.iter().map(|ty| {
                        format!("\n\
                                -- {file}:{line}\n\
                                -- {full_path}\n\
                                -- {full_path} - {id:?}\n\
                                -- Option<{full_path}> - {option_id:?}\n\
                                -- Vec<{full_path}> - {vec_id:?}\n\
                                CREATE TYPE {name};\
                            ",
                            full_path = ty.full_path,
                            file = ty.file,
                            line = ty.line,
                            id = ty.id,
                            option_id = ty.option_id,
                            vec_id = ty.vec_id,
                            name = ty.name,
                        )
                    }).by_ref().collect()
                }

                fn externs_with_operators(&self) -> String {
                    use crate::__pgx_internals::PgxExternReturn;
                    self.externs.iter().map(|ext| {
                        let fn_sql = format!("\
                                CREATE OR REPLACE FUNCTION \"{name}\"({arguments}) {returns}\n{extern_attrs}LANGUAGE c /* Rust */\nAS 'MODULE_PATHNAME', '{name}';\
                                \
                            ",
                            name = ext.name,
                            arguments = if !ext.fn_args.is_empty() {
                                String::from("\n") + &ext.fn_args.iter().map(|arg|
                                    format!("\
                                            \t\"{pattern}\" {sql_type} {default}/* {ty_name} */\
                                        ",
                                        pattern = arg.pattern,
                                        sql_type = pgx::type_id_to_sql_type(arg.ty_id).unwrap_or_else(|| arg.ty_name.to_string()),
                                        default = if let Some(def) = arg.default { format!("DEFAULT {} ", def) } else { String::from("") },
                                        ty_name = arg.ty_name,
                                    )
                                ).collect::<Vec<_>>().join(",\n") + "\n"
                            } else { Default::default() },
                            returns = match &ext.fn_return {
                                PgxExternReturn::None => String::default(),
                                PgxExternReturn::Type { id, name } => format!("RETURNS {} /* {} */", pgx::type_id_to_sql_type(*id).unwrap_or_else(|| name.to_string()), name),
                                PgxExternReturn::SetOf { id, name } => format!("RETURNS SETOF {} /* {} */", pgx::type_id_to_sql_type(*id).unwrap_or_else(|| name.to_string()), name),
                                PgxExternReturn::Iterated(vec) => format!("RETURNS TABLE ({}\n)",
                                    vec.iter().map(|(id, ty_name, col_name)| format!("\n\t\"{}\" {} /* {} */", col_name.unwrap(), pgx::type_id_to_sql_type(*id).unwrap_or_else(|| ty_name.to_string()), ty_name)).collect::<Vec<_>>().join(",")
                                ),
                            },
                            // TODO: Search Path
                            extern_attrs = if ext.extern_attrs.is_empty() {
                                String::default()
                            } else {
                                let mut retval = ext.extern_attrs.iter().map(|attr| format!("{}", attr).to_uppercase()).collect::<Vec<_>>().join(" ");
                                retval.push('\n');
                                retval
                            },
                        );

                        let ext_sql = format!("\n\
                                -- {file}:{line}\n\
                                -- {module_path}::{name}\n\
                                {fn_sql}\n\
                                {overridden}\
                            ",
                            name = ext.name,
                            module_path = ext.module_path,
                            file = ext.file,
                            line = ext.line,
                            fn_sql = if ext.overridden.is_some() {
                                let mut inner = fn_sql.lines().map(|f| format!("-- {}", f)).collect::<Vec<_>>().join("\n");
                                inner.push_str("\n--\n-- Overridden as (due to a `//` comment with a `sql` code block):");
                                inner
                            } else {
                                fn_sql
                            },
                            overridden = ext.overridden.map(|f| f.to_owned() + "\n").unwrap_or_default(),
                        );
                        match (ext.overridden, &ext.operator) {
                            (Some(_overridden), Some(op)) => {
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
                                        -- {file}:{line}\n\
                                        -- {module_path}::{name}\n\
                                        CREATE OPERATOR {opname} (\n\
                                            \tPROCEDURE=\"{name},\"\n\
                                            \tLEFTARG={left_arg} /* {left_name} */,\n\
                                            \tRIGHTARG={right_arg} /* {right_name} */\n\
                                            {optionals}\
                                        );
                                    ",
                                    opname = op.opname.unwrap(),
                                    file = ext.file,
                                    line = ext.line,
                                    name = ext.name,
                                    module_path = ext.module_path,
                                    left_name = ext.fn_args.get(0).unwrap().ty_name,
                                    right_name = ext.fn_args.get(1).unwrap().ty_name,
                                    left_arg = pgx::type_id_to_sql_type(ext.fn_args.get(0).unwrap().ty_id).unwrap_or_else(|| ext.fn_args.get(0).unwrap().ty_name.to_string()),
                                    right_arg = pgx::type_id_to_sql_type(ext.fn_args.get(1).unwrap().ty_id).unwrap_or_else(|| ext.fn_args.get(1).unwrap().ty_name.to_string()),
                                    optionals = optionals.join(",\n")
                                );
                                ext_sql + &operator_sql
                            },
                            (None, None) | (None, Some(_)) | (Some(_), None) => ext_sql,
                        }
                    }).by_ref().collect()
                }

                fn materialized_types(&self) -> String {
                    self.types.iter().map(|ty| {
                        format!("\n\
                                -- {file}:{line}\n\
                                -- {full_path} - {id:?}\n\
                                -- Option<{full_path}> - {option_id:?}\n\
                                -- Vec<{full_path}> - {vec_id:?}\n\
                                CREATE TYPE {name} (\n\
                                    \tINTERNALLENGTH = variable,\n\
                                    \tINPUT = {in_fn},\n\
                                    \tOUTPUT = {out_fn},\n\
                                    \tSTORAGE = extended\n\
                                );
                            ",
                            full_path = ty.full_path,
                            file = ty.file,
                            line = ty.line,
                            id = ty.id,
                            option_id = ty.option_id,
                            vec_id = ty.vec_id,
                            name = ty.name,
                            in_fn = ty.in_fn,
                            out_fn = ty.out_fn,
                        )
                    }).by_ref().collect()
                }

                fn operator_classes(&self) -> String {
                    let hashes = self.hashes.iter().map(|hash_derive| {
                        format!("\n\
                            -- {file}:{line}\n\
                            -- {full_path}\n\
                            -- {id:?}\n\
                            CREATE OPERATOR FAMILY {name}_hash_ops USING hash;\n\
                            CREATE OPERATOR CLASS {name}_hash_ops DEFAULT FOR TYPE {name} USING hash FAMILY {name}_hash_ops AS\n\
                                \tOPERATOR    1   =  ({name}, {name}),\n\
                                \tFUNCTION    1   {name}_hash({name});\
                            ",
                            name = hash_derive.name,
                            full_path = hash_derive.full_path,
                            file = hash_derive.file,
                            line = hash_derive.line,
                            id = hash_derive.id,
                        )
                    }).collect::<String>();
                    let ords = self.ords.iter().map(|ord_derive| {
                        format!("\n\
                            -- {file}:{line}\n\
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
                            line = ord_derive.line,
                            id = ord_derive.id,
                        )
                    }).collect::<String>();
                    hashes + &ords
                }

                pub fn register_types(&self) {
                    for en in &self.enums {
                        pgx::map_type_id_to_sql_type(en.id, en.name);
                        pgx::map_type_id_to_sql_type(en.option_id, en.name);
                        pgx::map_type_id_to_sql_type(en.vec_id, format!("{}[]", en.name));
                    }
                    for ty in &self.types {
                        pgx::map_type_id_to_sql_type(ty.id, ty.name);
                        pgx::map_type_id_to_sql_type(ty.option_id, ty.name);
                        pgx::map_type_id_to_sql_type(ty.vec_id, format!("{}[]", ty.name));
                    }
                }
            }

            #[derive(Debug)]
            pub struct PgxExtensionSql {
                pub sql: &'static str,
                pub file: &'static str,
                pub line: u32,
            }
            pgx::inventory::collect!(PgxExtensionSql);

            #[derive(Debug)]
            pub struct PgxPostgresType {
                pub name: &'static str,
                pub file: &'static str,
                pub line: u32,
                pub full_path: &'static str,
                pub id: core::any::TypeId,
                pub option_id: core::any::TypeId,
                pub vec_id: core::any::TypeId,
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
                pub line: u32,
                pub module_path: &'static str,
                pub extern_attrs: Vec<pgx_utils::ExternArgs>,
                pub search_path: Option<Vec<&'static str>>,
                pub fn_args: Vec<PgxExternInputs>,
                pub fn_return: PgxExternReturn,
                pub operator: Option<PgxOperator>,
                pub overridden: Option<&'static str>,
            }
            pgx::inventory::collect!(PgxExtern);

            #[derive(Debug)]
            pub struct PgxPostgresEnum {
                pub name: &'static str,
                pub file: &'static str,
                pub line: u32,
                pub full_path: &'static str,
                pub id: core::any::TypeId,
                pub option_id: core::any::TypeId,
                pub vec_id: core::any::TypeId,
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
                SetOf {
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
                pub line: u32,
                pub full_path: &'static str,
                pub id: core::any::TypeId,
            }
            pgx::inventory::collect!(PgxPostgresHash);

            #[derive(Debug)]
            pub struct PgxPostgresOrd {
                pub name: &'static str,
                pub file: &'static str,
                pub line: u32,
                pub full_path: &'static str,
                pub id: core::any::TypeId,
            }
            pgx::inventory::collect!(PgxPostgresOrd);
        }
    };
}

/// Top-level initialization function.  This is called automatically by the `pg_module_magic!()`
/// macro and need not be called directly
#[allow(unused)]
pub fn initialize() {
    register_pg_guard_panic_handler();
}

use core::any::TypeId;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

static TYPEID_SQL_MAPPING: Lazy<Arc<RwLock<HashMap<TypeId, String>>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(TypeId::of::<&'static str>(), String::from("text"));
    m.insert(TypeId::of::<Option<&'static str>>(), String::from("text"));
    m.insert(TypeId::of::<Vec<&'static str>>(), String::from("text[]"));
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

    Arc::from(RwLock::from(m))
});
pub fn type_id_to_sql_type(id: TypeId) -> Option<String> {
    TYPEID_SQL_MAPPING
        .read()
        .unwrap()
        .get(&id)
        .map(|f| f.clone())
}
pub fn map_type_to_sql_type<T: 'static>(sql: impl AsRef<str>) {
    let sql = sql.as_ref().to_string();
    TYPEID_SQL_MAPPING
        .write()
        .unwrap()
        .insert(TypeId::of::<T>(), sql.clone());
    TYPEID_SQL_MAPPING
        .write()
        .unwrap()
        .insert(TypeId::of::<Option<T>>(), sql.clone());
    TYPEID_SQL_MAPPING
        .write()
        .unwrap()
        .insert(TypeId::of::<Vec<T>>(), format!("{}[]", sql));
}

pub fn map_type_id_to_sql_type(id: TypeId, sql: impl AsRef<str>) {
    let sql = sql.as_ref().to_string();
    TYPEID_SQL_MAPPING.write().unwrap().insert(id, sql);
}
