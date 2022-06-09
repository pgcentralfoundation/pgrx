/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
mod argument;
mod operator;
mod returning;

pub use argument::PgExternArgumentEntity;
pub use operator::PgOperatorEntity;
use quote::{ToTokens, TokenStreamExt};
pub use returning::{PgExternReturnEntity, PgExternReturnEntityIteratedItem};

use crate::{
    sql_entity_graph::{
        extension_sql::SqlDeclared,
        pgx_sql::PgxSql,
        to_sql::{entity::ToSqlConfigEntity, ToSql},
        SqlGraphEntity, SqlGraphIdentifier,
    },
    ExternArgs,
};

use eyre::eyre;
use std::cmp::Ordering;

/// The output of a [`PgExtern`](crate::sql_entity_graph::pg_extern::PgExtern) from `quote::ToTokens::to_tokens`.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct PgExternEntity {
    pub name: &'static str,
    pub unaliased_name: &'static str,
    pub schema: Option<&'static str>,
    pub file: &'static str,
    pub line: u32,
    pub module_path: &'static str,
    pub full_path: &'static str,
    pub extern_attrs: Vec<ExternArgs>,
    pub search_path: Option<Vec<&'static str>>,
    pub fn_args: Vec<PgExternArgumentEntity>,
    pub fn_return: PgExternReturnEntity,
    pub operator: Option<PgOperatorEntity>,
    pub to_sql_config: ToSqlConfigEntity,
}

impl Ord for PgExternEntity {
    fn cmp(&self, other: &Self) -> Ordering {
        self.file
            .cmp(other.file)
            .then_with(|| self.file.cmp(other.file))
    }
}

impl PartialOrd for PgExternEntity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Into<SqlGraphEntity> for PgExternEntity {
    fn into(self) -> SqlGraphEntity {
        SqlGraphEntity::Function(self)
    }
}

impl SqlGraphIdentifier for PgExternEntity {
    fn dot_identifier(&self) -> String {
        format!("fn {}", self.full_path)
    }
    fn rust_identifier(&self) -> String {
        self.full_path.to_string()
    }

    fn file(&self) -> Option<&'static str> {
        Some(self.file)
    }

    fn line(&self) -> Option<u32> {
        Some(self.line)
    }
}

impl ToSql for PgExternEntity {
    #[tracing::instrument(
        level = "error",
        skip(self, context),
        fields(identifier = %self.rust_identifier()),
    )]
    fn to_sql(&self, context: &PgxSql) -> eyre::Result<String> {
        let self_index = context.externs[self];
        let mut extern_attrs = self.extern_attrs.clone();
        // if we already have a STRICT marker we do not need to add it
        let mut strict_upgrade = !extern_attrs.iter().any(|i| i == &ExternArgs::Strict);
        if strict_upgrade {
            for arg in &self.fn_args {
                if arg.is_optional {
                    strict_upgrade = false;
                }
            }
        }

        if strict_upgrade {
            extern_attrs.push(ExternArgs::Strict);
        }

        let module_pathname = &context.get_module_pathname();

        let fn_sql = format!(
            "\
                                CREATE FUNCTION {schema}\"{name}\"({arguments}) {returns}\n\
                                {extern_attrs}\
                                {search_path}\
                                LANGUAGE c /* Rust */\n\
                                AS '{module_pathname}', '{unaliased_name}_wrapper';\
                            ",
            schema = self
                .schema
                .map(|schema| format!("{}.", schema))
                .unwrap_or_else(|| context.schema_prefix_for(&self_index)),
            name = self.name,
            unaliased_name = self.unaliased_name,
            module_pathname = module_pathname,
            arguments = if !self.fn_args.is_empty() {
                let mut args = Vec::new();
                for (idx, arg) in self.fn_args.iter().enumerate() {
                    match &arg.ty {
                        TypeEntity::Type {
                            ty_source,
                            ty_id,
                            full_path,
                            module_path: _,
                        } => {
                            let graph_index = context
                                .graph
                                .neighbors_undirected(self_index)
                                .find(|neighbor| match &context.graph[*neighbor] {
                                    SqlGraphEntity::Type(ty) => ty.id_matches(&ty_id),
                                    SqlGraphEntity::Enum(en) => en.id_matches(&ty_id),
                                    SqlGraphEntity::BuiltinType(defined) => defined == full_path,
                                    _ => false,
                                })
                                .ok_or_else(|| {
                                    eyre!("Could not find arg type in graph. Got: {:?}", arg)
                                })?;
                            let needs_comma = idx < (self.fn_args.len() - 1);
                            let buf = format!("\
                                                    \t\"{pattern}\" {variadic}{schema_prefix}{sql_type}{default}{maybe_comma}/* {full_path} */\
                                                ",
                                                    pattern = arg.pattern,
                                                    schema_prefix = context.schema_prefix_for(&graph_index),
                                                    // First try to match on [`TypeId`] since it's most reliable.
                                                    sql_type = context.rust_to_sql(ty_id.clone(), ty_source, full_path).ok_or_else(|| eyre!(
                                                        "Failed to map argument `{}` type `{}` to SQL type while building function `{}`.",
                                                        arg.pattern,
                                                        full_path,
                                                        self.name
                                                    ))?,
                                                    default = if let Some(def) = arg.default { format!(" DEFAULT {}", def) } else { String::from("") },
                                                    variadic = if arg.is_variadic { "VARIADIC " } else { "" },
                                                    maybe_comma = if needs_comma { ", " } else { " " },
                                                    full_path = full_path,
                                            );
                            args.push(buf);
                        }
                        TypeEntity::CompositeType { sql, wrapper } => {
                            let buf = format!("\
                                                \t\"{pattern}\" {variadic}{sql}{maybe_square_braces}{maybe_default}{maybe_comma} /* {with_wrapper} */\
                                            ",
                                                pattern = arg.pattern,
                                                maybe_square_braces = if wrapper.square_brackets() {
                                                    "[]"
                                                } else { "" },
                                                variadic = if arg.is_variadic { "VARIADIC " } else { "" },
                                                with_wrapper = match wrapper {
                                                    CompositeTypeWrapper::None => format!("::pgx::composite_type!(..)"),
                                                    CompositeTypeWrapper::Option => format!("Option<::pgx::composite_type!(..)>"),
                                                    CompositeTypeWrapper::OptionVec => format!("Option<Vec::pgx::composite_type!(..)>"),
                                                    CompositeTypeWrapper::OptionVecOption => format!("Option<Vec<Option::pgx::composite_type!(..)>>"),
                                                    CompositeTypeWrapper::OptionArray => format!("Option<Array::pgx::composite_type!(..)>"),
                                                    CompositeTypeWrapper::OptionArrayOption => format!("Option<Array<Option::pgx::composite_type!(..)>>>"),
                                                    CompositeTypeWrapper::OptionVariadicArray => format!("Option<VariadicArray::pgx::composite_type!(..)>>"),
                                                    CompositeTypeWrapper::OptionVariadicArrayOption => format!("Option<VariadicArray<Option::pgx::composite_type!(..)>>>"),
                                                    CompositeTypeWrapper::Vec => format!("Vec<::pgx::composite_type!(..)>"),
                                                    CompositeTypeWrapper::VecOption => format!("Vec<Option<::pgx::composite_type!(..)>>"),
                                                    CompositeTypeWrapper::Array => format!("::pgx::Array<composite_type!(..)>"),
                                                    CompositeTypeWrapper::ArrayOption => format!("::pgx::Array<Option<::pgx::composite_type!(..)>>"),
                                                    CompositeTypeWrapper::VariadicArray => format!("::pgx::VariadicArray<::pgx::composite_type!(..)>"),
                                                    CompositeTypeWrapper::VariadicArrayOption => format!("::pgx::VariadicArray<Option<::pgx::composite_type!(..)>>"),
                                                },
                                                maybe_default = if let Some(default) = arg.default {
                                                    format!(" DEFAULT {default}")
                                                } else { String::from("") },
                                                maybe_comma = if idx < (self.fn_args.len() - 1) { ", " } else { "" },
                                            );
                            args.push(buf);
                        }
                    }
                }
                String::from("\n") + &args.join("\n") + "\n"
            } else {
                Default::default()
            },
            returns = match &self.fn_return {
                PgExternReturnEntity::None => String::from("RETURNS void"),
                PgExternReturnEntity::Type { ty } => {
                    match ty {
                        TypeEntity::Type {
                            ty_source,
                            ty_id,
                            full_path,
                            module_path: _,
                        } => {
                            let graph_index = context
                                .graph
                                .neighbors_undirected(self_index)
                                .find(|neighbor| match &context.graph[*neighbor] {
                                    SqlGraphEntity::Type(ty) => ty.id_matches(&ty_id),
                                    SqlGraphEntity::Enum(en) => en.id_matches(&ty_id),
                                    SqlGraphEntity::BuiltinType(defined) => &*defined == full_path,
                                    _ => false,
                                })
                                .ok_or_else(|| eyre!("Could not find return type in graph."))?;
                            format!("RETURNS {schema_prefix}{sql_type} /* {full_path} */",
                                                    sql_type = context.source_only_to_sql_type(ty_source).or_else(|| {
                                                        context.type_id_to_sql_type(*ty_id)
                                                    }).or_else(|| {
                                                           let pat = full_path.to_string();
                                                           if let Some(found) = context.has_sql_declared_entity(&SqlDeclared::Type(pat.clone())) {
                                                               Some(found.sql())
                                                           }  else if let Some(found) = context.has_sql_declared_entity(&SqlDeclared::Enum(pat.clone())) {
                                                               Some(found.sql())
                                                           } else {
                                                               None
                                                           }
                                                       }).ok_or_else(|| eyre!("Failed to map return type `{}` to SQL type while building function `{}`.", full_path, self.full_path))?,
                                                    schema_prefix = context.schema_prefix_for(&graph_index),
                                                    full_path = full_path
                                            )
                        }
                        TypeEntity::CompositeType { sql, wrapper } => match wrapper {
                            CompositeTypeWrapper::None => format!("RETURNS {sql} /* ::pgx::composite_type!(..) */"),
                            CompositeTypeWrapper::Option => format!("RETURNS {sql}[] /* Option<::pgx::composite_type!(..)> */"),
                            CompositeTypeWrapper::OptionVec => format!("RETURNS {sql}[] /* Option<Vec::pgx::composite_type!(..)> */"),
                            CompositeTypeWrapper::OptionVecOption => format!("RETURNS {sql}[] /* Option<Vec<Option::pgx::composite_type!(..)>> */"),
                            CompositeTypeWrapper::OptionArray => format!("RETURNS {sql}[] /* Option<Array::pgx::composite_type!(..)> */"),
                            CompositeTypeWrapper::OptionArrayOption => format!("RETURNS {sql}[] /* Option<Array<Option::pgx::composite_type!(..)>>> */"),
                            CompositeTypeWrapper::OptionVariadicArray => format!("RETURNS {sql}[] /* Option<VariadicArray::pgx::composite_type!(..)>> */"),
                            CompositeTypeWrapper::OptionVariadicArrayOption => format!("RETURNS {sql}[] /* Option<VariadicArray<Option::pgx::composite_type!(..)>>> */"),
                            CompositeTypeWrapper::Vec => format!("RETURNS {sql}[] /* Vec<::pgx::composite_type!(..)> */"),
                            CompositeTypeWrapper::VecOption => format!("RETURNS {sql}[] /* Vec<Option<::pgx::composite_type!(..)>> */"),
                            CompositeTypeWrapper::Array => format!("RETURNS {sql}[] /* ::pgx::Array<::pgx::composite_type!(..)> */"),
                            CompositeTypeWrapper::ArrayOption => format!("RETURNS {sql}[] /* ::pgx::Array<Option<::pgx::composite_type!(..)>> */"),
                            CompositeTypeWrapper::VariadicArray => format!("RETURNS {sql}[] /* ::pgx::VariadicArray<::pgx::composite_type!(..)> */"),
                            CompositeTypeWrapper::VariadicArrayOption => format!("RETURNS {sql}[] /* ::pgx::VariadicArray<Option<::pgx::composite_type!(..)>> */"),
                        }
                    }
                }
                PgExternReturnEntity::SetOf { ty } => {
                    match ty {
                        TypeEntity::Type {
                            ty_source,
                            ty_id,
                            full_path,
                            module_path: _,
                        } => {
                            let graph_index = context
                                .graph
                                .neighbors_undirected(self_index)
                                .find(|neighbor| match &context.graph[*neighbor] {
                                    SqlGraphEntity::Type(ty) => ty.id_matches(&ty_id),
                                    SqlGraphEntity::Enum(en) => en.id_matches(&ty_id),
                                    SqlGraphEntity::BuiltinType(defined) => defined == full_path,
                                    _ => false,
                                })
                                .ok_or_else(|| eyre!("Could not find return type in graph."))?;
                            format!("RETURNS SETOF {schema_prefix}{sql_type} /* {full_path} */",
                                                    sql_type = context.source_only_to_sql_type(ty_source).or_else(|| {
                                                        context.type_id_to_sql_type(*ty_id)
                                                    }).or_else(|| {
                                                           let pat = full_path.to_string();
                                                           if let Some(found) = context.has_sql_declared_entity(&SqlDeclared::Type(pat.clone())) {
                                                               Some(found.sql())
                                                           }  else if let Some(found) = context.has_sql_declared_entity(&SqlDeclared::Enum(pat.clone())) {
                                                               Some(found.sql())
                                                           } else {
                                                               None
                                                           }
                                                       }).ok_or_else(|| eyre!("Failed to map return type `{}` to SQL type while building function `{}`.", full_path, self.full_path))?,
                                                    schema_prefix = context.schema_prefix_for(&graph_index),
                                                    full_path = full_path
                                            )
                        }
                        TypeEntity::CompositeType { sql, wrapper } => match wrapper {
                            CompositeTypeWrapper::None => format!("RETURNS SETOF {sql} /* ::pgx::composite_type!(..) */"),
                            CompositeTypeWrapper::Option => format!("RETURNS SETOF {sql}[] /* Option<::pgx::composite_type!(..)> */"),
                            CompositeTypeWrapper::OptionVec => format!("RETURNS SETOF {sql}[] /* Option<Vec::pgx::composite_type!(..)> */"),
                            CompositeTypeWrapper::OptionVecOption => format!("RETURNS SETOF {sql}[] /* Option<Vec<Option::pgx::composite_type!(..)>> */"),
                            CompositeTypeWrapper::OptionArray => format!("RETURNS SETOF {sql}[] /* Option<Array::pgx::composite_type!(..)> */"),
                            CompositeTypeWrapper::OptionArrayOption => format!("RETURNS SETOF {sql}[] /* Option<Array<Option::pgx::composite_type!(..)>>> */"),
                            CompositeTypeWrapper::OptionVariadicArray => format!("RETURNS SETOF {sql}[] /* Option<VariadicArray::pgx::composite_type!(..)>> */"),
                            CompositeTypeWrapper::OptionVariadicArrayOption => format!("RETURNS SETOF {sql}[] /* Option<VariadicArray<Option::pgx::composite_type!(..)>>> */"),
                            CompositeTypeWrapper::Vec => format!("RETURNS SETOF {sql}[] /* Vec<::pgx::composite_type!(..)> */"),
                            CompositeTypeWrapper::VecOption => format!("RETURNS SETOF {sql}[] /* Vec<Option<::pgx::composite_type!(..)>> */"),
                            CompositeTypeWrapper::Array => format!("RETURNS SETOF {sql}[] /* ::pgx::Array<composite_type!(..)> */"),
                            CompositeTypeWrapper::ArrayOption => format!("RETURNS SETOF {sql}[] /* ::pgx::Array<Option<::pgx::composite_type!(..)>> */"),
                            CompositeTypeWrapper::VariadicArray => format!("RETURNS SETOF {sql}[] /* ::pgx::VariadicArray<::pgx::composite_type!(..)> */"),
                            CompositeTypeWrapper::VariadicArrayOption => format!("RETURNS SETOF {sql}[] /* ::pgx::VariadicArray<Option<::pgx::composite_type!(..)>> */"),
                        }
                    }
                }
                PgExternReturnEntity::Iterated(table_items) => {
                    let mut items = String::new();
                    for (idx, returning::PgExternReturnEntityIteratedItem { ty, name: col_name }) in
                        table_items.iter().enumerate()
                    {
                        match ty {
                            TypeEntity::Type {
                                ty_source,
                                ty_id,
                                full_path,
                                module_path: _,
                            } => {
                                let graph_index = context
                                    .graph
                                    .neighbors_undirected(self_index)
                                    .find(|neighbor| match &context.graph[*neighbor] {
                                        SqlGraphEntity::Type(ty) => ty.id_matches(&ty_id),
                                        SqlGraphEntity::Enum(en) => en.id_matches(&ty_id),
                                        SqlGraphEntity::BuiltinType(defined) => {
                                            defined == ty_source
                                        }
                                        _ => false,
                                    });
                                let needs_comma = idx < (table_items.len() - 1);
                                let item = format!("\n\t{col_name} {schema_prefix}{ty_resolved}{needs_comma} /* {ty_name} */",
                                                                   col_name = col_name.expect("An iterator of tuples should have `named!()` macro declarations."),
                                                                   schema_prefix = if let Some(graph_index) = graph_index {
                                                                       context.schema_prefix_for(&graph_index)
                                                                   } else { "".into() },
                                                                   ty_resolved = {
                                                                        context.source_only_to_sql_type(ty_source).or_else(|| {
                                                                            context.type_id_to_sql_type(*ty_id)
                                                                        }).or_else(|| {
                                                                            let pat = ty_source.to_string();
                                                                            if let Some(found) = context.has_sql_declared_entity(&SqlDeclared::Type(pat.clone())) {
                                                                                Some(found.sql())
                                                                            }  else if let Some(found) = context.has_sql_declared_entity(&SqlDeclared::Enum(pat.clone())) {
                                                                                Some(found.sql())
                                                                            } else {
                                                                                None
                                                                            }
                                                                        }).ok_or_else(|| eyre!("Failed to map return type `{}` to SQL type while building function `{}`.", full_path, self.name))?
                                                                    },
                                                                   needs_comma = if needs_comma { ", " } else { " " },
                                                                   ty_name = full_path
                                                );
                                items.push_str(&item);
                            }
                            TypeEntity::CompositeType { sql, wrapper } => {
                                match wrapper.square_brackets() {
                                    false => items.push_str(sql),
                                    true => items.push_str(&format!("{sql}[]")),
                                }
                            }
                        }
                    }
                    format!("RETURNS TABLE ({}\n)", items)
                }
                PgExternReturnEntity::Trigger => String::from("RETURNS trigger"),
            },
            search_path = if let Some(search_path) = &self.search_path {
                let retval = format!("SET search_path TO {}", search_path.join(", "));
                retval + "\n"
            } else {
                Default::default()
            },
            extern_attrs = if extern_attrs.is_empty() {
                String::default()
            } else {
                let mut retval = extern_attrs
                    .iter()
                    .map(|attr| format!("{}", attr).to_uppercase())
                    .collect::<Vec<_>>()
                    .join(" ");
                retval.push('\n');
                retval
            },
        );

        let ext_sql = format!(
            "\n\
                                -- {file}:{line}\n\
                                -- {module_path}::{name}\n\
                                {requires}\
                                {fn_sql}\
                            ",
            name = self.name,
            module_path = self.module_path,
            file = self.file,
            line = self.line,
            fn_sql = fn_sql,
            requires = {
                let requires_attrs = self
                    .extern_attrs
                    .iter()
                    .filter_map(|x| match x {
                        ExternArgs::Requires(requirements) => Some(requirements),
                        _ => None,
                    })
                    .flatten()
                    .collect::<Vec<_>>();
                if !requires_attrs.is_empty() {
                    format!(
                        "\
                       -- requires:\n\
                        {}\n\
                    ",
                        requires_attrs
                            .iter()
                            .map(|i| format!("--   {}", i))
                            .collect::<Vec<_>>()
                            .join("\n")
                    )
                } else {
                    "".to_string()
                }
            },
        );
        tracing::trace!(sql = %ext_sql);

        let rendered = if let Some(op) = &self.operator {
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

            let left_arg = self
                .fn_args
                .get(0)
                .ok_or_else(|| eyre!("Did not find `left_arg` for operator `{}`.", self.name))?;
            let (left_arg_ty_id, left_arg_full_path) = match left_arg.ty {
                TypeEntity::Type {
                    ty_id, full_path, ..
                } => (ty_id, full_path),
                TypeEntity::CompositeType { .. } => {
                    Err(eyre!("Cannot create operators for composite types"))?
                }
            };
            let left_arg_graph_index = context
                .graph
                .neighbors_undirected(self_index)
                .find(|neighbor| match &context.graph[*neighbor] {
                    SqlGraphEntity::Type(ty) => ty.id_matches(&left_arg_ty_id),
                    _ => false,
                })
                .ok_or_else(|| eyre!("Could not find left arg function in graph."))?;

            let right_arg = self
                .fn_args
                .get(1)
                .ok_or_else(|| eyre!("Did not find `left_arg` for operator `{}`.", self.name))?;
            let (right_arg_ty_id, right_arg_full_path) = match left_arg.ty {
                TypeEntity::Type {
                    ty_id, full_path, ..
                } => (ty_id, full_path),
                TypeEntity::CompositeType { .. } => {
                    Err(eyre!("Cannot create operators for composite types"))?
                }
            };
            let right_arg_graph_index = context
                .graph
                .neighbors_undirected(self_index)
                .find(|neighbor| match &context.graph[*neighbor] {
                    SqlGraphEntity::Type(ty) => ty.id_matches(&right_arg_ty_id),
                    _ => false,
                })
                .ok_or_else(|| eyre!("Could not find right arg function in graph."))?;

            let operator_sql = format!("\n\n\
                                        -- {file}:{line}\n\
                                        -- {module_path}::{unaliased_name}\n\
                                        CREATE OPERATOR {opname} (\n\
                                            \tPROCEDURE=\"{name}\",\n\
                                            \tLEFTARG={schema_prefix_left}{left_arg}, /* {left_name} */\n\
                                            \tRIGHTARG={schema_prefix_right}{right_arg}{maybe_comma} /* {right_name} */\n\
                                            {optionals}\
                                        );\
                                        ",
                                        opname = op.opname.unwrap(),
                                        file = self.file,
                                        line = self.line,
                                        name = self.name,
                                        unaliased_name = self.unaliased_name,
                                        module_path = self.module_path,
                                        left_name = left_arg_full_path,
                                        right_name = right_arg_full_path,
                                        schema_prefix_left = context.schema_prefix_for(&left_arg_graph_index),
                                        left_arg = context.type_id_to_sql_type(left_arg_ty_id).ok_or_else(||
                                            eyre!(
                                                "Failed to map argument `{}` type `{}` to SQL type while building operator `{}`.",
                                                left_arg.pattern,
                                                left_arg_full_path,
                                                self.name
                                            )
                                        )?,
                                        schema_prefix_right = context.schema_prefix_for(&right_arg_graph_index),
                                        right_arg = context.type_id_to_sql_type(right_arg_ty_id).ok_or_else(||
                                            eyre!(
                                                "Failed to map argument `{}` type `{}` to SQL type while building operator `{}`.",
                                                right_arg.pattern,
                                                right_arg_full_path,
                                                self.name
                                            )
                                        )?,
                                        maybe_comma = if optionals.len() >= 1 { "," } else { "" },
                                        optionals = if !optionals.is_empty() { optionals.join(",\n") + "\n" } else { "".to_string() },
                                );
            tracing::trace!(sql = %operator_sql);
            ext_sql + &operator_sql
        } else {
            ext_sql
        };
        Ok(rendered)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TypeEntity {
    Type {
        ty_source: &'static str,
        ty_id: core::any::TypeId,
        full_path: &'static str,
        module_path: String,
    },
    CompositeType {
        sql: &'static str,
        wrapper: CompositeTypeWrapper,
    },
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum CompositeTypeWrapper {
    None,
    Option,
    OptionVec,
    OptionVecOption,
    OptionArray,
    OptionArrayOption,
    OptionVariadicArray,
    OptionVariadicArrayOption,
    Vec,
    VecOption,
    Array,
    ArrayOption,
    VariadicArray,
    VariadicArrayOption,
}

impl CompositeTypeWrapper {
    fn square_brackets(&self) -> bool {
        match self {
            CompositeTypeWrapper::None | CompositeTypeWrapper::Option => false,
            CompositeTypeWrapper::Vec
            | CompositeTypeWrapper::VecOption
            | CompositeTypeWrapper::Array
            | CompositeTypeWrapper::ArrayOption
            | CompositeTypeWrapper::VariadicArray
            | CompositeTypeWrapper::OptionVec
            | CompositeTypeWrapper::OptionVecOption
            | CompositeTypeWrapper::OptionArray
            | CompositeTypeWrapper::OptionArrayOption
            | CompositeTypeWrapper::OptionVariadicArray
            | CompositeTypeWrapper::OptionVariadicArrayOption
            | CompositeTypeWrapper::VariadicArrayOption => true,
        }
    }
}

impl ToTokens for CompositeTypeWrapper {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let self_tokens = match self {
            CompositeTypeWrapper::None => {
                quote::quote! { ::pgx::utils::sql_entity_graph::CompositeTypeWrapper::None }
            }
            CompositeTypeWrapper::Option => {
                quote::quote! { ::pgx::utils::sql_entity_graph::CompositeTypeWrapper::Option }
            }
            CompositeTypeWrapper::OptionVec => {
                quote::quote! { ::pgx::utils::sql_entity_graph::CompositeTypeWrapper::OptionVec }
            }
            CompositeTypeWrapper::OptionVecOption => {
                quote::quote! { ::pgx::utils::sql_entity_graph::CompositeTypeWrapper::OptionVecOption }
            }
            CompositeTypeWrapper::OptionArray => {
                quote::quote! { ::pgx::utils::sql_entity_graph::CompositeTypeWrapper::OptionArray }
            }
            CompositeTypeWrapper::OptionArrayOption => {
                quote::quote! { ::pgx::utils::sql_entity_graph::CompositeTypeWrapper::OptionArrayOption }
            }
            CompositeTypeWrapper::OptionVariadicArray => {
                quote::quote! { ::pgx::utils::sql_entity_graph::CompositeTypeWrapper::OptionVariadicArray }
            }
            CompositeTypeWrapper::OptionVariadicArrayOption => {
                quote::quote! { ::pgx::utils::sql_entity_graph::CompositeTypeWrapper::OptionVariadicArrayOption }
            }
            CompositeTypeWrapper::Vec => {
                quote::quote! { ::pgx::utils::sql_entity_graph::CompositeTypeWrapper::Vec }
            }
            CompositeTypeWrapper::VecOption => {
                quote::quote! { ::pgx::utils::sql_entity_graph::CompositeTypeWrapper::VecOption }
            }
            CompositeTypeWrapper::Array => {
                quote::quote! { ::pgx::utils::sql_entity_graph::CompositeTypeWrapper::Array }
            }
            CompositeTypeWrapper::ArrayOption => {
                quote::quote! { ::pgx::utils::sql_entity_graph::CompositeTypeWrapper::ArrayOption }
            }
            CompositeTypeWrapper::VariadicArray => {
                quote::quote! { ::pgx::utils::sql_entity_graph::CompositeTypeWrapper::VariadicArray }
            }
            CompositeTypeWrapper::VariadicArrayOption => {
                quote::quote! { ::pgx::utils::sql_entity_graph::CompositeTypeWrapper::VariadicArrayOption }
            }
        };
        tokens.append_all(self_tokens);
    }
}
