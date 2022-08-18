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

use eyre::{eyre, WrapErr};
use std::{any::Any, cmp::Ordering};

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
        let mut strict_upgrade = extern_attrs.iter().any(|i| i == &ExternArgs::Strict);
        if !strict_upgrade {
            // It may be possible to infer a `STRICT` marker though.
            // But we can only do that if the user hasn't used `Option<T>` or `pgx::Internal`
            for arg in &self.fn_args {
                if arg.used_ty.optional || arg.used_ty.type_id() == context.internal_type {
                    strict_upgrade = false;
                }
            }
        }

        if strict_upgrade {
            extern_attrs.push(ExternArgs::Strict);
        }
        extern_attrs.sort();
        extern_attrs.dedup();

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
                    let graph_index = context
                        .graph
                        .neighbors_undirected(self_index)
                        .find(|neighbor| match &context.graph[*neighbor] {
                            SqlGraphEntity::Type(ty) => ty.id_matches(&arg.used_ty.ty_id),
                            SqlGraphEntity::Enum(en) => en.id_matches(&arg.used_ty.ty_id),
                            SqlGraphEntity::BuiltinType(defined) => {
                                defined == arg.used_ty.full_path
                            }
                            _ => false,
                        })
                        .ok_or_else(|| eyre!("Could not find arg type in graph. Got: {:?}", arg))?;
                    let needs_comma = idx < (self.fn_args.len() - 1);
                    let buf = format!("\
                                            \t\"{pattern}\" {variadic}{schema_prefix}{sql_type}{default}{maybe_comma}/* {full_path} */\
                                        ",
                                            pattern = arg.pattern,
                                            schema_prefix = context.schema_prefix_for(&graph_index),
                                            // First try to match on [`TypeId`] since it's most reliable.
                                            sql_type = if let Some(composite_type) = arg.used_ty.composite_type {
                                                composite_type.to_string()
                                                    + if context
                                                        .composite_type_requires_square_brackets(&arg.used_ty.ty_id)
                                                        .wrap_err_with(|| format!("Attempted on `{}`", arg.used_ty.ty_source))?
                                                    {
                                                        "[]"
                                                    } else {
                                                        ""
                                                    }
                                            } else {
                                                context.source_only_to_sql_type(arg.used_ty.ty_source).or_else(|| {
                                                                    context.type_id_to_sql_type(arg.used_ty.ty_id)
                                                                }).or_else(|| {
                                                                    let pat = arg.used_ty.full_path.to_string();
                                                                    if let Some(found) = context.has_sql_declared_entity(&SqlDeclared::Type(pat.clone())) {
                                                                        Some(found.sql())
                                                                    }  else if let Some(found) = context.has_sql_declared_entity(&SqlDeclared::Enum(pat.clone())) {
                                                                        Some(found.sql())
                                                                    } else {
                                                                        None
                                                                    }
                                                                }).ok_or_else(|| eyre!("Failed to map arg type `{}` to SQL type while building function `{}`.", arg.used_ty.full_path, self.full_path))?
                                            },
                                            default = if let Some(def) = arg.used_ty.default { format!(" DEFAULT {}", def) } else { String::from("") },
                                            variadic = if arg.used_ty.variadic { "VARIADIC " } else { "" },
                                            maybe_comma = if needs_comma { ", " } else { " " },
                                            full_path = arg.used_ty.full_path,
                                    );
                    args.push(buf);
                }
                String::from("\n") + &args.join("\n") + "\n"
            } else {
                Default::default()
            },
            returns = match &self.fn_return {
                PgExternReturnEntity::None => String::from("RETURNS void"),
                PgExternReturnEntity::Type { ty } => {
                    let graph_index = context
                        .graph
                        .neighbors_undirected(self_index)
                        .find(|neighbor| match &context.graph[*neighbor] {
                            SqlGraphEntity::Type(neighbor_ty) => neighbor_ty.id_matches(&ty.ty_id),
                            SqlGraphEntity::Enum(neighbor_en) => neighbor_en.id_matches(&ty.ty_id),
                            SqlGraphEntity::BuiltinType(defined) => &*defined == ty.full_path,
                            _ => false,
                        })
                        .ok_or_else(|| eyre!("Could not find return type in graph."))?;
                    format!(
                        "RETURNS {schema_prefix}{sql_type} /* {full_path} */",
                        sql_type = if let Some(composite_type) = ty.composite_type {
                            composite_type.to_string()
                                + if context
                                    .composite_type_requires_square_brackets(&ty.ty_id)
                                    .wrap_err_with(|| format!("Attempted on `{}`", ty.ty_source))?
                                {
                                    "[]"
                                } else {
                                    ""
                                }
                        } else {
                            context.source_only_to_sql_type(ty.ty_source).or_else(|| {
                                                context.type_id_to_sql_type(ty.ty_id)
                                            }).or_else(|| {
                                                let pat = ty.full_path.to_string();
                                                if let Some(found) = context.has_sql_declared_entity(&SqlDeclared::Type(pat.clone())) {
                                                    Some(found.sql())
                                                }  else if let Some(found) = context.has_sql_declared_entity(&SqlDeclared::Enum(pat.clone())) {
                                                    Some(found.sql())
                                                } else {
                                                    None
                                                }
                                            }).ok_or_else(|| eyre!("Failed to map return type `{}` to SQL type while building function `{}`.", ty.full_path, self.full_path))?
                        },
                        schema_prefix = context.schema_prefix_for(&graph_index),
                        full_path = ty.full_path
                    )
                }
                PgExternReturnEntity::SetOf { ty } => {
                    let graph_index = context
                        .graph
                        .neighbors_undirected(self_index)
                        .find(|neighbor| match &context.graph[*neighbor] {
                            SqlGraphEntity::Type(neighbor_ty) => neighbor_ty.id_matches(&ty.ty_id),
                            SqlGraphEntity::Enum(neighbor_en) => neighbor_en.id_matches(&ty.ty_id),
                            SqlGraphEntity::BuiltinType(defined) => defined == ty.full_path,
                            _ => false,
                        })
                        .ok_or_else(|| eyre!("Could not find return type in graph."))?;
                    format!(
                        "RETURNS SETOF {schema_prefix}{sql_type} /* {full_path} */",
                        sql_type = if let Some(composite_type) = ty.composite_type {
                            composite_type.to_string()
                                + if context
                                    .composite_type_requires_square_brackets(&ty.ty_id)
                                    .wrap_err_with(|| format!("Attempted on `{}`", ty.ty_source))?
                                {
                                    "[]"
                                } else {
                                    ""
                                }
                        } else {
                            context.source_only_to_sql_type(ty.ty_source).or_else(|| {
                                                    context.type_id_to_sql_type(ty.ty_id)
                                                }).or_else(|| {
                                                    let pat = ty.full_path.to_string();
                                                    if let Some(found) = context.has_sql_declared_entity(&SqlDeclared::Type(pat.clone())) {
                                                        Some(found.sql())
                                                    }  else if let Some(found) = context.has_sql_declared_entity(&SqlDeclared::Enum(pat.clone())) {
                                                        Some(found.sql())
                                                    } else {
                                                        None
                                                    }
                                                }).ok_or_else(|| eyre!("Failed to map return type `{}` to SQL type while building function `{}`.", ty.full_path, self.full_path))?
                        },
                        schema_prefix = context.schema_prefix_for(&graph_index),
                        full_path = ty.full_path
                    )
                }
                PgExternReturnEntity::Iterated(table_items) => {
                    let mut items = String::new();
                    for (idx, returning::PgExternReturnEntityIteratedItem { ty, name: col_name }) in
                        table_items.iter().enumerate()
                    {
                        let graph_index =
                            context
                                .graph
                                .neighbors_undirected(self_index)
                                .find(|neighbor| match &context.graph[*neighbor] {
                                    SqlGraphEntity::Type(neightbor_ty) => {
                                        neightbor_ty.id_matches(&ty.ty_id)
                                    }
                                    SqlGraphEntity::Enum(neightbor_en) => {
                                        neightbor_en.id_matches(&ty.ty_id)
                                    }
                                    SqlGraphEntity::BuiltinType(defined) => defined == ty.ty_source,
                                    _ => false,
                                });
                        let needs_comma = idx < (table_items.len() - 1);
                        let item = format!(
                                "\n\t{col_name} {schema_prefix}{ty_resolved}{needs_comma} /* {ty_name} */",
                                col_name = col_name.expect("An iterator of tuples should have `named!()` macro declarations."),
                                schema_prefix = if let Some(graph_index) = graph_index {
                                    context.schema_prefix_for(&graph_index)
                                } else { "".into() },
                                ty_resolved = if let Some(composite_type) = ty.composite_type {
                                    composite_type.to_string() + if context.composite_type_requires_square_brackets(&ty.ty_id).wrap_err_with(|| format!("Attempted on `{}`", ty.ty_source))? { "[]" } else { "" }
                                } else {
                                    context.source_only_to_sql_type(ty.ty_source).or_else(|| {
                                        context.type_id_to_sql_type(ty.ty_id)
                                    }).or_else(|| {
                                        let pat = ty.ty_source.to_string();
                                        if let Some(found) = context.has_sql_declared_entity(&SqlDeclared::Type(pat.clone())) {
                                            Some(found.sql())
                                        }  else if let Some(found) = context.has_sql_declared_entity(&SqlDeclared::Enum(pat.clone())) {
                                            Some(found.sql())
                                        } else {
                                            None
                                        }
                                    }).ok_or_else(|| eyre!("Failed to map return type `{}` to SQL type while building function `{}`.", ty.full_path, self.name))?
                                },
                                needs_comma = if needs_comma { ", " } else { " " },
                                ty_name = ty.full_path
                        );
                        items.push_str(&item);
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
            let (left_arg_ty_id, left_arg_full_path) =
                (left_arg.used_ty.ty_id, left_arg.used_ty.full_path);
            let left_arg_graph_index = context
                .graph
                .neighbors_undirected(self_index)
                .find(|neighbor| match &context.graph[*neighbor] {
                    SqlGraphEntity::Type(ty) => ty.id_matches(&left_arg_ty_id),
                    SqlGraphEntity::Enum(en) => en.id_matches(&left_arg_ty_id),
                    SqlGraphEntity::BuiltinType(defined) => defined == left_arg_full_path,
                    _ => false,
                })
                .ok_or_else(|| eyre!("Could not find left arg function in graph."))?;

            let right_arg = self
                .fn_args
                .get(1)
                .ok_or_else(|| eyre!("Did not find `left_arg` for operator `{}`.", self.name))?;
            let (right_arg_ty_id, right_arg_full_path) =
                (right_arg.used_ty.ty_id, right_arg.used_ty.full_path);
            let right_arg_graph_index = context
                .graph
                .neighbors_undirected(self_index)
                .find(|neighbor| match &context.graph[*neighbor] {
                    SqlGraphEntity::Type(ty) => ty.id_matches(&right_arg_ty_id),
                    SqlGraphEntity::Enum(en) => en.id_matches(&right_arg_ty_id),
                    SqlGraphEntity::BuiltinType(defined) => defined == right_arg_full_path,
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
                                        left_arg = if let Some(composite_type) = left_arg.used_ty.composite_type {
                                            composite_type.to_string()
                                                + if context
                                                    .composite_type_requires_square_brackets(&left_arg.used_ty.ty_id)
                                                    .wrap_err_with(|| format!("Attempted on `{}`", left_arg.used_ty.ty_source))?
                                                {
                                                    "[]"
                                                } else {
                                                    ""
                                                }
                                        } else {
                                            context.source_only_to_sql_type(left_arg.used_ty.ty_source).or_else(|| {
                                                                context.type_id_to_sql_type(left_arg.used_ty.ty_id)
                                                            }).or_else(|| {
                                                                let pat = left_arg.used_ty.full_path.to_string();
                                                                if let Some(found) = context.has_sql_declared_entity(&SqlDeclared::Type(pat.clone())) {
                                                                    Some(found.sql())
                                                                }  else if let Some(found) = context.has_sql_declared_entity(&SqlDeclared::Enum(pat.clone())) {
                                                                    Some(found.sql())
                                                                } else {
                                                                    None
                                                                }
                                                            }).ok_or_else(|| eyre!("Failed to map left arg type `{}` to SQL type while building function `{}`.", left_arg.used_ty.full_path, self.full_path))?
                                        },
                                        schema_prefix_right = context.schema_prefix_for(&right_arg_graph_index),
                                        right_arg = if let Some(composite_type) = right_arg.used_ty.composite_type {
                                            composite_type.to_string()
                                                + if context
                                                    .composite_type_requires_square_brackets(&right_arg.used_ty.ty_id)
                                                    .wrap_err_with(|| format!("Attempted on `{}`", right_arg.used_ty.ty_source))?
                                                {
                                                    "[]"
                                                } else {
                                                    ""
                                                }
                                        } else {
                                            context.source_only_to_sql_type(right_arg.used_ty.ty_source).or_else(|| {
                                                                context.type_id_to_sql_type(right_arg.used_ty.ty_id)
                                                            }).or_else(|| {
                                                                let pat = right_arg.used_ty.full_path.to_string();
                                                                if let Some(found) = context.has_sql_declared_entity(&SqlDeclared::Type(pat.clone())) {
                                                                    Some(found.sql())
                                                                }  else if let Some(found) = context.has_sql_declared_entity(&SqlDeclared::Enum(pat.clone())) {
                                                                    Some(found.sql())
                                                                } else {
                                                                    None
                                                                }
                                                            }).ok_or_else(|| eyre!("Failed to map right arg type `{}` to SQL type while building function `{}`.", right_arg.used_ty.full_path, self.full_path))?
                                        },
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
