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
#[derive(Debug, Clone)]
pub struct PgExternEntity {
    pub metadata: crate::sql_entity_graph::metadata::FunctionMetadataEntity,
    pub arg_patterns: Vec<&'static str>,
    pub arg_defaults: Vec<Option<&'static str>>,
    pub schema: Option<&'static str>,
    pub file: &'static str,
    pub line: u32,
    pub extern_attrs: Vec<ExternArgs>,
    pub search_path: Option<Vec<&'static str>>,
    pub operator: Option<PgOperatorEntity>,
    pub to_sql_config: ToSqlConfigEntity,
}

impl std::hash::Hash for PgExternEntity {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.metadata.hash(state);
    }
}

impl PartialEq for PgExternEntity {
    fn eq(&self, other: &Self) -> bool {
        self.metadata.eq(&other.metadata)
    }
}

impl Eq for PgExternEntity {}

impl Ord for PgExternEntity {
    fn cmp(&self, other: &Self) -> Ordering {
        self.metadata.cmp(&other.metadata)
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
        format!("fn {}", self.metadata.function_name())
    }
    fn rust_identifier(&self) -> String {
        self.metadata.path.to_string()
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
            for arg in &self.metadata.arguments {
                if arg.optional || arg.type_id == context.internal_type {
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
                                AS '{module_pathname}', '{name}_wrapper';\
                            ",
            schema = self
                .schema
                .map(|schema| format!("{}.", schema))
                .unwrap_or_else(|| context.schema_prefix_for(&self_index)),
            name = self.metadata.function_name(),
            module_pathname = module_pathname,
            arguments = if !self.metadata.arguments.is_empty() {
                let mut args = Vec::new();
                for (idx, arg) in self.metadata.arguments.iter().enumerate() {
                    let arg_pattern = self.arg_patterns[idx];
                    let arg_default = self.arg_defaults[idx];
                    let graph_index = context
                        .graph
                        .neighbors_undirected(self_index)
                        .find(|neighbor| match &context.graph[*neighbor] {
                            SqlGraphEntity::Type(ty) => ty.id_matches(&arg.type_id),
                            SqlGraphEntity::Enum(en) => en.id_matches(&arg.type_id),
                            SqlGraphEntity::BuiltinType(defined) => defined == &arg.type_name,
                            _ => false,
                        })
                        .ok_or_else(|| eyre!("Could not find arg type in graph. Got: {:?}", arg))?;
                    let needs_comma = idx < (self.metadata.arguments.len() - 1);
                    let buf = format!("\
                                            \t\"{pattern}\" {variadic}{schema_prefix}{sql_type}{default}{maybe_comma}/* {type_name} */\
                                        ",
                                            pattern = arg_pattern,
                                            schema_prefix = context.schema_prefix_for(&graph_index),
                                            // First try to match on [`TypeId`] since it's most reliable.
                                            sql_type = self.metadata.arguments[0].sql_type,
                                            default = if let Some(def) = arg_default { format!(" DEFAULT {}", def) } else { String::from("") },
                                            variadic = if arg.variadic { "VARIADIC " } else { "" },
                                            maybe_comma = if needs_comma { ", " } else { " " },
                                            type_name = arg.type_name,
                                     );
                    args.push(buf);
                }
                String::from("\n") + &args.join("\n") + "\n"
            } else {
                Default::default()
            },
            returns = match &self.metadata.retval {
                None => String::from("RETURNS void"),
                Some(retval) => {
                    let graph_index = context
                        .graph
                        .neighbors_undirected(self_index)
                        .find(|neighbor| match &context.graph[*neighbor] {
                            SqlGraphEntity::Type(ty) => ty.id_matches(&retval.type_id),
                            SqlGraphEntity::Enum(en) => en.id_matches(&retval.type_id),
                            SqlGraphEntity::BuiltinType(defined) => &*defined == retval.type_name,
                            _ => false,
                        })
                        .ok_or_else(|| eyre!("Could not find return type in graph."))?;
                    format!(
                        "RETURNS {schema_prefix}{sql_type} /* {type_name} */",
                        sql_type = self.metadata.retval.as_ref().unwrap().sql_type,
                        schema_prefix = context.schema_prefix_for(&graph_index),
                        type_name = retval.type_name
                    )
                }
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
            name = self.metadata.function_name(),
            module_path = self.metadata.module_path(),
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

            let left_arg = self.metadata.arguments.get(0).ok_or_else(|| {
                eyre!(
                    "Did not find `left_arg` for operator `{}`.",
                    self.metadata.function_name()
                )
            })?;
            let left_arg_graph_index = context
                .graph
                .neighbors_undirected(self_index)
                .find(|neighbor| match &context.graph[*neighbor] {
                    SqlGraphEntity::Type(ty) => ty.id_matches(&left_arg.type_id),
                    _ => false,
                })
                .ok_or_else(|| eyre!("Could not find left arg function in graph."))?;
            let right_arg = self.metadata.arguments.get(1).ok_or_else(|| {
                eyre!(
                    "Did not find `left_arg` for operator `{}`.",
                    self.metadata.function_name()
                )
            })?;
            let right_arg_graph_index = context
                .graph
                .neighbors_undirected(self_index)
                .find(|neighbor| match &context.graph[*neighbor] {
                    SqlGraphEntity::Type(ty) => ty.id_matches(&right_arg.type_id),
                    _ => false,
                })
                .ok_or_else(|| eyre!("Could not find right arg function in graph."))?;

            let operator_sql = format!("\n\n\
                                                    -- {file}:{line}\n\
                                                    -- {module_path}::{name}\n\
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
                                                    name = self.metadata.function_name(),
                                                    module_path = self.metadata.module_path(),
                                                    left_name = left_arg.type_name,
                                                    right_name = right_arg.type_name,
                                                    schema_prefix_left = context.schema_prefix_for(&left_arg_graph_index),
                                                    left_arg = context.type_id_to_sql_type(left_arg.type_id)
                                                        .ok_or_else(|| eyre!("Failed to map left argument type `{}` to SQL type while building operator `{}`.", left_arg.type_name, self.metadata.function_name()))?,
                                                    schema_prefix_right = context.schema_prefix_for(&right_arg_graph_index),
                                                    right_arg = context.type_id_to_sql_type(right_arg.type_id)
                                                        .ok_or_else(|| eyre!("Failed to map right argument type `{}` to SQL type while building operator `{}`.", right_arg.type_name, self.metadata.function_name()))?,
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
