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

        pub use __pgx_internals::PgxSql;
        mod __pgx_internals {
            use core::convert::TryFrom;
            use pgx::{once_cell::sync::Lazy, inventory, ControlFile};

            #[derive(Debug)]
            pub struct PgxSql {
                pub control: ControlFile,
                pub schemas: Vec<&'static Schema>,
                pub extension_sql: Vec<&'static ExtensionSql>,
                pub externs: Vec<&'static PgExtern>,
                pub types: Vec<&'static PostgresType>,
                pub enums: Vec<&'static PostgresEnum>,
                pub ords: Vec<&'static PostgresOrd>,
                pub hashes: Vec<&'static PostgresHash>
            }

            impl PgxSql {
                pub fn generate() -> Self {
                    use std::fmt::Write;
                    let mut generated = Self {
                        control: ControlFile::try_from(
                            include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", env!("CARGO_CRATE_NAME"), ".control"))
                        ).expect("Invalid .control file"),
                        schemas: inventory::iter::<Schema>().collect(),
                        extension_sql: inventory::iter::<ExtensionSql>().collect(),
                        externs: inventory::iter::<PgExtern>().collect(),
                        types: inventory::iter::<PostgresType>().collect(),
                        enums: inventory::iter::<PostgresEnum>().collect(),
                        hashes: inventory::iter::<PostgresHash>().collect(),
                        ords: inventory::iter::<PostgresOrd>().collect(),
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

                pub fn schema_alias_of(&self, module_path: &'static str) -> Option<String> {
                    let mut needle = None;
                    for Schema(schema) in &self.schemas {
                        if schema.module_path.starts_with(module_path) {
                            needle = Some(schema.name.to_string());
                            break;
                        }
                    }
                    needle
                }

                pub fn schema_prefix_for(&self, module_path: &'static str) -> String {
                    self.schema_alias_of(module_path).or_else(|| {
                        self.control.schema.clone()
                    }).map(|v| (v + ".").to_string()).unwrap_or_else(|| "".to_string())
                }

                pub fn to_sql(&self) -> String {
                    format!("\
                            -- This file is auto generated by pgx.\n\
                            -- `extension_sql!()` defined SQL.\n\
                            {extension_sql}\n\
                            -- Schemas defined by `#[pg_schema] mod {}` blocks (except `public` & `pg_catalog`)\n\
                            {schemas}\n\
                            -- Enums derived via `#[derive(PostgresEnum)]`\n\
                            {enums}\n\
                            -- Shell types for types defined by `#[derive(PostgresType)]`\n\
                            {shell_types}\n\
                            -- Functions defined by `#[pg_extern]`\n\
                            {externs_with_operators}\n\
                            -- Types defined by `#[derive(PostgresType)]`\n\
                            {materialized_types}\n\
                            -- Operator classes defined by `#[derive(PostgresHash, PostgresOrd)]`\n\
                            {operator_classes}\n\
                        ",
                        extension_sql = self.extension_sql(),
                        schemas = self.schemas(),
                        enums = self.enums(),
                        shell_types = self.shell_types(),
                        externs_with_operators = self.externs_with_operators(),
                        materialized_types = self.materialized_types(),
                        operator_classes = self.operator_classes(),
                    )
                }

                fn extension_sql(&self) -> String {
                    let mut buf = String::new();
                    for &ExtensionSql(ref item) in &self.extension_sql {
                        buf.push_str(&format!("\
                                -- {file}:{line}\n\
                                {sql}\
                            ",
                            file = item.file,
                            line = item.line,
                            sql = item.sql,
                        ))
                    }
                    buf
                }

                fn schemas(&self) -> String {
                    let mut buf = String::new();
                    if let Some(schema) = &self.control.schema {
                        buf.push_str(&format!("CREATE SCHEMA IF NOT EXISTS {};\n", schema));
                    }
                    for &Schema(ref item) in &self.schemas {
                        match item.name {
                            "pg_catalog" | "public" =>  (),
                            name => buf.push_str(&format!("\
                                    CREATE SCHEMA IF NOT EXISTS {name}; /* {module_path} */\n\
                                ",
                                name = name,
                                module_path = item.module_path,
                            )),
                        };
                    }
                    buf
                }

                fn enums(&self) -> String {
                    let mut buf = String::new();
                    for &PostgresEnum(ref item) in &self.enums {
                        buf.push_str(&format!("\
                                -- {file}:{line}\n\
                                -- {full_path} - {id:?}\n\
                                CREATE TYPE {schema}{name} AS ENUM (\n\
                                    {variants}\
                                );\n\
                            ",
                            schema = self.schema_prefix_for(item.module_path),
                            full_path = item.full_path,
                            file = item.file,
                            line = item.line,
                            id = item.id,
                            name = item.name,
                            variants = item.variants.iter().map(|variant| format!("\t'{}',\n", variant)).collect::<String>(),
                        ));
                    }
                    buf
                }

                fn shell_types(&self) -> String {
                    let mut buf = String::new();
                    for &PostgresType(ref item) in &self.types {
                        buf.push_str(&format!("\n\
                                -- {file}:{line}\n\
                                -- {full_path}\n\
                                CREATE TYPE {schema}{name};\n\
                            ",
                            schema = self.schema_prefix_for(item.module_path),
                            full_path = item.full_path,
                            file = item.file,
                            line = item.line,
                            name = item.name,
                        ))
                    }
                    buf
                }

                fn externs_with_operators(&self) -> String {
                    let mut buf = String::new();
                    for &PgExtern(ref item) in &self.externs {
                        let mut extern_attrs = item.extern_attrs.clone();
                        let mut strict_upgrade = true;
                        if !extern_attrs.iter().any(|i| i == &pgx_utils::ExternArgs::Strict) {
                            for arg in &item.fn_args {
                                if arg.is_optional {
                                    strict_upgrade = false;
                                }
                            }
                        }
                        if strict_upgrade {
                            extern_attrs.push(pgx_utils::ExternArgs::Strict);
                        }

                        let fn_sql = format!("\
                                CREATE OR REPLACE FUNCTION {schema}\"{name}\"({arguments}) {returns}\n\
                                {extern_attrs}\
                                {search_path}\
                                LANGUAGE c /* Rust */\n\
                                AS 'MODULE_PATHNAME', '{name}_wrapper';\
                            ",
                            schema = self.schema_prefix_for(item.module_path),
                            name = item.name,
                            arguments = if !item.fn_args.is_empty() {
                                String::from("\n") + &item.fn_args.iter().enumerate().map(|(idx, arg)| {
                                    let needs_comma = idx < (item.fn_args.len() - 1);
                                    format!("\
                                            \t\"{pattern}\" {sql_type} {default}{maybe_comma}/* {ty_name} */\
                                        ",
                                        pattern = arg.pattern,
                                        sql_type = pgx::type_id_to_sql_type(arg.ty_id).unwrap_or_else(|| arg.ty_name.to_string()),
                                        default = if let Some(def) = arg.default { format!("DEFAULT {} ", def) } else { String::from("") },
                                        maybe_comma = if needs_comma { "," } else { "" },
                                        ty_name = arg.ty_name,
                                    )
                                }).collect::<Vec<_>>().join("\n") + "\n"
                            } else { Default::default() },
                            returns = match &item.fn_return {
                                pgx_utils::pg_inventory::InventoryPgExternReturn::None => String::from("RETURNS void"),
                                pgx_utils::pg_inventory::InventoryPgExternReturn::Type { id, name } => format!("RETURNS {} /* {} */", pgx::type_id_to_sql_type(*id).unwrap_or_else(|| name.to_string()), name),
                                pgx_utils::pg_inventory::InventoryPgExternReturn::SetOf { id, name } => format!("RETURNS SETOF {} /* {} */", pgx::type_id_to_sql_type(*id).unwrap_or_else(|| name.to_string()), name),
                                pgx_utils::pg_inventory::InventoryPgExternReturn::Iterated(vec) => format!("RETURNS TABLE ({}\n)",
                                    vec.iter().map(|(id, ty_name, col_name)| format!("\n\t\"{}\" {} /* {} */", col_name.unwrap(), pgx::type_id_to_sql_type(*id).unwrap_or_else(|| ty_name.to_string()), ty_name)).collect::<Vec<_>>().join(",")
                                ),
                            },
                            search_path = if let Some(search_path) = &item.search_path {
                                let retval = format!("SET search_path TO {}", search_path.join(", "));
                                retval + "\n"
                            } else { Default::default() },
                            extern_attrs = if extern_attrs.is_empty() {
                                String::default()
                            } else {
                                let mut retval = extern_attrs.iter().map(|attr| format!("{}", attr).to_uppercase()).collect::<Vec<_>>().join(" ");
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
                            name = item.name,
                            module_path = item.module_path,
                            file = item.file,
                            line = item.line,
                            fn_sql = if item.overridden.is_some() {
                                let mut inner = fn_sql.lines().map(|f| format!("-- {}", f)).collect::<Vec<_>>().join("\n");
                                inner.push_str("\n--\n-- Overridden as (due to a `//` comment with a `sql` code block):");
                                inner
                            } else {
                                fn_sql
                            },
                            overridden = item.overridden.map(|f| f.to_owned() + "\n").unwrap_or_default(),
                        );

                        let rendered = match (item.overridden, &item.operator) {
                            (None, Some(op)) => {
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
                                            \tPROCEDURE=\"{name}\",\n\
                                            \tLEFTARG={left_arg}, /* {left_name} */\n\
                                            \tRIGHTARG={right_arg}, /* {right_name} */\n\
                                            {optionals}\
                                        );
                                    ",
                                    opname = op.opname.unwrap(),
                                    file = item.file,
                                    line = item.line,
                                    name = item.name,
                                    module_path = item.module_path,
                                    left_name = item.fn_args.get(0).unwrap().ty_name,
                                    right_name = item.fn_args.get(1).unwrap().ty_name,
                                    left_arg = pgx::type_id_to_sql_type(item.fn_args.get(0).unwrap().ty_id).unwrap_or_else(|| item.fn_args.get(0).unwrap().ty_name.to_string()),
                                    right_arg = pgx::type_id_to_sql_type(item.fn_args.get(1).unwrap().ty_id).unwrap_or_else(|| item.fn_args.get(1).unwrap().ty_name.to_string()),
                                    optionals = optionals.join(",\n") + "\n"
                                );
                                ext_sql + &operator_sql
                            },
                            (None, None) | (Some(_), Some(_)) | (Some(_), None) => ext_sql,
                        };
                        buf.push_str(&rendered)
                    }
                    buf
                }

                fn materialized_types(&self) -> String {
                    let mut buf = String::new();
                    for &PostgresType(ref item) in &self.types {
                        buf.push_str(&format!("\n\
                                -- {file}:{line}\n\
                                -- {full_path} - {id:?}\n\
                                CREATE TYPE {schema}{name} (\n\
                                    \tINTERNALLENGTH = variable,\n\
                                    \tINPUT = {in_fn},\n\
                                    \tOUTPUT = {out_fn},\n\
                                    \tSTORAGE = extended\n\
                                );
                            ",
                            full_path = item.full_path,
                            file = item.file,
                            line = item.line,
                            schema = self.schema_prefix_for(item.module_path),
                            id = item.id,
                            name = item.name,
                            in_fn = item.in_fn,
                            out_fn = item.out_fn,
                        ));
                    }
                    buf
                }

                fn operator_classes(&self) -> String {
                    let mut buf = String::new();
                    for &PostgresHash(ref item) in &self.hashes {
                        buf.push_str(&format!("\n\
                            -- {file}:{line}\n\
                            -- {full_path}\n\
                            -- {id:?}\n\
                            CREATE OPERATOR FAMILY {name}_hash_ops USING hash;\n\
                            CREATE OPERATOR CLASS {name}_hash_ops DEFAULT FOR TYPE {name} USING hash FAMILY {name}_hash_ops AS\n\
                                \tOPERATOR    1   =  ({name}, {name}),\n\
                                \tFUNCTION    1   {name}_hash({name});\
                            ",
                            name = item.name,
                            full_path = item.full_path,
                            file = item.file,
                            line = item.line,
                            id = item.id,
                        ));
                    }
                    for &PostgresOrd(ref item) in &self.ords {
                        buf.push_str(&format!("\n\
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
                            name = item.name,
                            full_path = item.full_path,
                            file = item.file,
                            line = item.line,
                            id = item.id,
                        ))
                    }
                    buf
                }

                pub fn register_types(&self) {
                    for &PostgresEnum(ref item) in &self.enums {
                        pgx::map_type_id_to_sql_type(item.id, item.name);
                        pgx::map_type_id_to_sql_type(item.option_id, item.name);
                        pgx::map_type_id_to_sql_type(item.vec_id, format!("{}[]", item.name));
                    }
                    for &PostgresType(ref item) in &self.types {
                        pgx::map_type_id_to_sql_type(item.id, item.name);
                        pgx::map_type_id_to_sql_type(item.option_id, item.name);
                        pgx::map_type_id_to_sql_type(item.vec_id, format!("{}[]", item.name));
                    }
                }
            }

            #[derive(Debug)]
            pub struct ExtensionSql(pub pgx_utils::pg_inventory::InventoryExtensionSql);
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
use std::convert::TryFrom;

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

#[derive(Debug)]
pub struct ControlFile {
    pub comment: String,
    pub default_version: String,
    pub module_pathname: String,
    pub relocatable: bool,
    pub superuser: bool,
    pub schema: Option<String>,
}

#[derive(Debug)]
pub enum ControlFileError {
    MissingField(&'static str),
}

impl std::fmt::Display for ControlFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ControlFileError::MissingField(field) => write!(f, "Missing field in control file! Please add `{}`", field),
        }
    }
}

impl std::error::Error for ControlFileError {}

impl TryFrom<&str> for ControlFile {
    type Error = Box<dyn std::error::Error>;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut temp = HashMap::new();
        for line in input.lines() {
            let parts: Vec<&str> = line.split('=').collect();

            if parts.len() != 2 {
                continue;
            }

            let (k, v) = (parts.get(0).unwrap().trim(), parts.get(1).unwrap().trim());

            let v = v.trim_start_matches('\'');
            let v = v.trim_end_matches('\'');

            temp.insert(k, v);
        }
        Ok(ControlFile {
            comment: temp.get("comment").ok_or(ControlFileError::MissingField("comment"))?.to_string(),
            default_version: temp.get("default_version").ok_or(ControlFileError::MissingField("default_version"))?.to_string(),
            module_pathname: temp.get("module_pathname").ok_or(ControlFileError::MissingField("module_pathname"))?.to_string(),
            relocatable: temp.get("relocatable").ok_or(ControlFileError::MissingField("relocatable"))? == &"true",
            superuser: temp.get("superuser").ok_or(ControlFileError::MissingField("superuser"))? == &"true",
            schema: temp.get("schema").map(|v| v.to_string()),
        })
    }
}
