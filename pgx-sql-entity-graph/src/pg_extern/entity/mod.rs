/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
/*!

`#[pg_extern]` related entities for Rust to SQL translation

> Like all of the [`sql_entity_graph`][crate::pgx_sql_entity_graph] APIs, this is considered **internal**
to the `pgx` framework and very subject to change between versions. While you may use this, please do it with caution.

*/
mod argument;
mod operator;
mod returning;

pub use argument::PgExternArgumentEntity;
pub use operator::PgOperatorEntity;
pub use returning::{PgExternReturnEntity, PgExternReturnEntityIteratedItem};

use crate::metadata::{Returns, SqlMapping};
use crate::pgx_sql::PgxSql;
use crate::to_sql::entity::ToSqlConfigEntity;
use crate::to_sql::ToSql;
use crate::ExternArgs;
use crate::{SqlGraphEntity, SqlGraphIdentifier};

use eyre::{eyre, WrapErr};

/// The output of a [`PgExtern`](crate::pg_extern::PgExtern) from `quote::ToTokens::to_tokens`.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct PgExternEntity {
    pub name: &'static str,
    pub unaliased_name: &'static str,
    pub module_path: &'static str,
    pub full_path: &'static str,
    pub metadata: crate::metadata::FunctionMetadataEntity,
    pub fn_args: Vec<PgExternArgumentEntity>,
    pub fn_return: PgExternReturnEntity,
    pub schema: Option<&'static str>,
    pub file: &'static str,
    pub line: u32,
    pub extern_attrs: Vec<ExternArgs>,
    pub search_path: Option<Vec<&'static str>>,
    pub operator: Option<PgOperatorEntity>,
    pub to_sql_config: ToSqlConfigEntity,
}

impl From<PgExternEntity> for SqlGraphEntity {
    fn from(val: PgExternEntity) -> Self {
        SqlGraphEntity::Function(val)
    }
}

impl SqlGraphIdentifier for PgExternEntity {
    fn dot_identifier(&self) -> String {
        format!("fn {}", self.name)
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
        // presume we can upgrade, then disprove it
        let mut strict_upgrade = !extern_attrs.iter().any(|i| i == &ExternArgs::Strict);
        if strict_upgrade {
            // It may be possible to infer a `STRICT` marker though.
            // But we can only do that if the user hasn't used `Option<T>` or `pgx::Internal`
            for arg in &self.metadata.arguments {
                if arg.optional {
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
                CREATE {or_replace} FUNCTION {schema}\"{name}\"({arguments}) {returns}\n\
                {extern_attrs}\
                {search_path}\
                LANGUAGE c /* Rust */\n\
                AS '{module_pathname}', '{unaliased_name}_wrapper';\
            ",
            or_replace =
                if extern_attrs.contains(&ExternArgs::CreateOrReplace) { "OR REPLACE" } else { "" },
            schema = self
                .schema
                .map(|schema| format!("{}.", schema))
                .unwrap_or_else(|| context.schema_prefix_for(&self_index)),
            name = self.name,
            module_pathname = module_pathname,
            arguments = if !self.fn_args.is_empty() {
                let mut args = Vec::new();
                let metadata_without_arg_skips = &self
                    .metadata
                    .arguments
                    .iter()
                    .filter(|v| v.argument_sql != Ok(SqlMapping::Skip))
                    .collect::<Vec<_>>();
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
                    let needs_comma = idx < (metadata_without_arg_skips.len().saturating_sub(1));
                    let metadata_argument = &self.metadata.arguments[idx];
                    match metadata_argument.argument_sql {
                        Ok(SqlMapping::As(ref argument_sql)) => {
                            let buf = format!("\
                                                \t\"{pattern}\" {variadic}{schema_prefix}{sql_type}{default}{maybe_comma}/* {type_name} */\
                                            ",
                                                pattern = arg.pattern,
                                                schema_prefix = context.schema_prefix_for(&graph_index),
                                                // First try to match on [`TypeId`] since it's most reliable.
                                                sql_type = argument_sql,
                                                default = if let Some(def) = arg.used_ty.default { format!(" DEFAULT {}", def) } else { String::from("") },
                                                variadic = if metadata_argument.variadic { "VARIADIC " } else { "" },
                                                maybe_comma = if needs_comma { ", " } else { " " },
                                                type_name = metadata_argument.type_name,
                                        );
                            args.push(buf);
                        }
                        Ok(SqlMapping::Composite { array_brackets }) => {
                            let sql =
                                self.fn_args[idx]
                                    .used_ty
                                    .composite_type
                                    .map(|v| {
                                        if array_brackets {
                                            format!("{v}[]")
                                        } else {
                                            format!("{v}")
                                        }
                                    })
                                    .ok_or_else(|| {
                                        eyre!(
                                    "Macro expansion time suggested a composite_type!() in return"
                                )
                                    })?;
                            let buf = format!("\
                                \t\"{pattern}\" {variadic}{schema_prefix}{sql_type}{default}{maybe_comma}/* {type_name} */\
                            ",
                                pattern = arg.pattern,
                                schema_prefix = context.schema_prefix_for(&graph_index),
                                // First try to match on [`TypeId`] since it's most reliable.
                                sql_type = sql,
                                default = if let Some(def) = arg.used_ty.default { format!(" DEFAULT {}", def) } else { String::from("") },
                                variadic = if metadata_argument.variadic { "VARIADIC " } else { "" },
                                maybe_comma = if needs_comma { ", " } else { " " },
                                type_name = metadata_argument.type_name,
                        );
                            args.push(buf);
                        }
                        Ok(SqlMapping::Source { array_brackets }) => {
                            let sql =
                                context
                                    .source_only_to_sql_type(arg.used_ty.ty_source)
                                    .map(|v| {
                                        if array_brackets {
                                            format!("{v}[]")
                                        } else {
                                            format!("{v}")
                                        }
                                    })
                                    .ok_or_else(|| {
                                        eyre!(
                                    "Macro expansion time suggested a source only mapping in return"
                                )
                                    })?;
                            let buf = format!("\
                                \t\"{pattern}\" {variadic}{schema_prefix}{sql_type}{default}{maybe_comma}/* {type_name} */\
                            ",
                                pattern = arg.pattern,
                                schema_prefix = context.schema_prefix_for(&graph_index),
                                // First try to match on [`TypeId`] since it's most reliable.
                                sql_type = sql,
                                default = if let Some(def) = arg.used_ty.default { format!(" DEFAULT {}", def) } else { String::from("") },
                                variadic = if metadata_argument.variadic { "VARIADIC " } else { "" },
                                maybe_comma = if needs_comma { ", " } else { " " },
                                type_name = metadata_argument.type_name,
                        );
                            args.push(buf);
                        }
                        Ok(SqlMapping::Skip) => (),
                        Err(err) => {
                            match context.source_only_to_sql_type(arg.used_ty.ty_source) {
                                Some(source_only_mapping) => {
                                    let buf = format!("\
                                            \t\"{pattern}\" {variadic}{schema_prefix}{sql_type}{default}{maybe_comma}/* {type_name} */\
                                        ",
                                            pattern = arg.pattern,
                                            schema_prefix = context.schema_prefix_for(&graph_index),
                                            // First try to match on [`TypeId`] since it's most reliable.
                                            sql_type = source_only_mapping,
                                            default = if let Some(def) = arg.used_ty.default { format!(" DEFAULT {}", def) } else { String::from("") },
                                            variadic = if metadata_argument.variadic { "VARIADIC " } else { "" },
                                            maybe_comma = if needs_comma { ", " } else { " " },
                                            type_name = metadata_argument.type_name,
                                    );
                                    args.push(buf);
                                }
                                None => return Err(err).wrap_err("While mapping argument"),
                            }
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
                    let metadata_retval = self.metadata.retval.clone().ok_or_else(|| eyre!("Macro expansion time and SQL resolution time had differing opinions about the return value existing"))?;
                    let metadata_retval_sql = match metadata_retval.return_sql {
                        Ok(Returns::One(SqlMapping::As(ref sql))) => sql.clone(),
                        Ok(Returns::One(SqlMapping::Composite { array_brackets })) => ty.composite_type.unwrap().to_string()
                        + if array_brackets {
                            "[]"
                        } else {
                            ""
                        },
                        Ok(Returns::SetOf(SqlMapping::Source { array_brackets })) =>
                            context.source_only_to_sql_type(ty.ty_source).unwrap().to_string() + if array_brackets {
                                "[]"
                            } else {
                                ""
                            },
                        Ok(other) => return Err(eyre!("Got non-plain mapped/composite return variant SQL in what macro-expansion thought was a type, got: {other:?}")),
                        Err(err) => {
                            match context.source_only_to_sql_type(ty.ty_source) {
                                Some(source_only_mapping) => source_only_mapping,
                                None => return Err(err).wrap_err("Error mapping return SQL")
                            }
                        },
                    };
                    format!(
                        "RETURNS {schema_prefix}{sql_type} /* {full_path} */",
                        sql_type = metadata_retval_sql,
                        schema_prefix = context.schema_prefix_for(&graph_index),
                        full_path = ty.full_path
                    )
                }
                PgExternReturnEntity::SetOf { ty, optional: _ } => {
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
                    let metadata_retval = self.metadata.retval.clone().ok_or_else(|| eyre!("Macro expansion time and SQL resolution time had differing opinions about the return value existing"))?;
                    let metadata_retval_sql = match metadata_retval.return_sql {
                            Ok(Returns::SetOf(SqlMapping::As(ref sql))) => sql.clone(),
                            Ok(Returns::SetOf(SqlMapping::Composite { array_brackets })) =>
                                ty.composite_type.unwrap().to_string() + if array_brackets {
                                    "[]"
                                } else {
                                    ""
                                },
                            Ok(Returns::SetOf(SqlMapping::Source { array_brackets })) =>
                                context.source_only_to_sql_type(ty.ty_source).unwrap().to_string() + if array_brackets {
                                    "[]"
                                } else {
                                    ""
                                },
                            Ok(_other) => return Err(eyre!("Got non-setof mapped/composite return variant SQL in what macro-expansion thought was a setof")),
                            Err(err) => return Err(err).wrap_err("Error mapping return SQL"),
                        };
                    format!(
                        "RETURNS SETOF {schema_prefix}{sql_type} /* {full_path} */",
                        sql_type = metadata_retval_sql,
                        schema_prefix = context.schema_prefix_for(&graph_index),
                        full_path = ty.full_path
                    )
                }
                PgExternReturnEntity::Iterated { tys: table_items, optional: _ } => {
                    let mut items = String::new();
                    let metadata_retval = self.metadata.retval.clone().ok_or_else(|| eyre!("Macro expansion time and SQL resolution time had differing opinions about the return value existing"))?;
                    let metadata_retval_sqls = match metadata_retval.return_sql {
                            Ok(Returns::Table(variants)) => {
                                let mut retval_sqls = vec![];
                                for (idx, variant) in variants.iter().enumerate() {
                                    let sql = match variant {
                                        SqlMapping::As(sql) => sql.clone(),
                                        SqlMapping::Composite { array_brackets } => {
                                            let composite = table_items[idx].ty.composite_type.unwrap().to_string();
                                            composite  + if *array_brackets {
                                                "[]"
                                            } else {
                                                ""
                                            }
                                        },
                                        SqlMapping::Source { array_brackets } =>
                                            context.source_only_to_sql_type(table_items[idx].ty.ty_source).unwrap() + if *array_brackets {
                                                "[]"
                                            } else {
                                                ""
                                            },
                                        SqlMapping::Skip => todo!(),
                                    };
                                    retval_sqls.push(sql)
                                }
                                retval_sqls
                            },
                            Ok(_other) => return Err(eyre!("Got non-table return variant SQL in what macro-expansion thought was a table")),
                            Err(err) => return Err(err).wrap_err("Error mapping return SQL"),
                        };

                    for (idx, returning::PgExternReturnEntityIteratedItem { ty, name: col_name }) in
                        table_items.iter().enumerate()
                    {
                        let graph_index =
                            context.graph.neighbors_undirected(self_index).find(|neighbor| {
                                match &context.graph[*neighbor] {
                                    SqlGraphEntity::Type(neightbor_ty) => {
                                        neightbor_ty.id_matches(&ty.ty_id)
                                    }
                                    SqlGraphEntity::Enum(neightbor_en) => {
                                        neightbor_en.id_matches(&ty.ty_id)
                                    }
                                    SqlGraphEntity::BuiltinType(defined) => defined == ty.ty_source,
                                    _ => false,
                                }
                            });

                        let needs_comma = idx < (table_items.len() - 1);
                        let item = format!(
                                "\n\t{col_name} {schema_prefix}{ty_resolved}{needs_comma} /* {ty_name} */",
                                col_name = col_name.expect("An iterator of tuples should have `named!()` macro declarations."),
                                schema_prefix = if let Some(graph_index) = graph_index {
                                    context.schema_prefix_for(&graph_index)
                                } else { "".into() },
                                ty_resolved = metadata_retval_sqls[idx],
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
                    .filter(|attr| **attr != ExternArgs::CreateOrReplace)
                    .map(|attr| format!("{}", attr).to_uppercase())
                    .collect::<Vec<_>>()
                    .join(" ");
                retval.push('\n');
                retval
            },
            unaliased_name = self.unaliased_name,
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
                        "-- requires:\n{}\n",
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

            let left_arg =
                self.metadata.arguments.get(0).ok_or_else(|| {
                    eyre!("Did not find `left_arg` for operator `{}`.", self.name)
                })?;
            let left_fn_arg = self
                .fn_args
                .get(0)
                .ok_or_else(|| eyre!("Did not find `left_arg` for operator `{}`.", self.name))?;
            let left_arg_graph_index = context
                .graph
                .neighbors_undirected(self_index)
                .find(|neighbor| match &context.graph[*neighbor] {
                    SqlGraphEntity::Type(ty) => ty.id_matches(&left_fn_arg.used_ty.ty_id),
                    SqlGraphEntity::Enum(en) => en.id_matches(&left_fn_arg.used_ty.ty_id),
                    SqlGraphEntity::BuiltinType(defined) => defined == &left_arg.type_name,
                    _ => false,
                })
                .ok_or_else(|| {
                    eyre!("Could not find left arg type in graph. Got: {:?}", left_arg)
                })?;
            let left_arg_sql = match left_arg.argument_sql {
                Ok(SqlMapping::As(ref sql)) => sql.clone(),
                Ok(SqlMapping::Composite { array_brackets }) => {
                    if array_brackets {
                        let composite_type = self.fn_args[0].used_ty.composite_type
                            .ok_or(eyre!("Found a composite type but macro expansion time did not reveal a name, use `pgx::composite_type!()`"))?;
                        format!("{composite_type}[]")
                    } else {
                        self.fn_args[0].used_ty.composite_type
                            .ok_or(eyre!("Found a composite type but macro expansion time did not reveal a name, use `pgx::composite_type!()`"))?.to_string()
                    }
                }
                Ok(SqlMapping::Source { array_brackets }) => {
                    if array_brackets {
                        let composite_type = context
                            .source_only_to_sql_type(self.fn_args[0].used_ty.ty_source)
                            .ok_or(eyre!(
                                "Found a source only mapping but no source mapping exists for this"
                            ))?;
                        format!("{composite_type}[]")
                    } else {
                        context.source_only_to_sql_type(self.fn_args[0].used_ty.ty_source)
                        .ok_or(eyre!("Found a composite type but macro expansion time did not reveal a name, use `pgx::composite_type!()`"))?.to_string()
                    }
                }
                Ok(SqlMapping::Skip) => {
                    return Err(eyre!(
                        "Found an skipped SQL type in an operator, this is not valid"
                    ))
                }
                Err(err) => return Err(err.into()),
            };

            let right_arg =
                self.metadata.arguments.get(1).ok_or_else(|| {
                    eyre!("Did not find `left_arg` for operator `{}`.", self.name)
                })?;
            let right_fn_arg = self
                .fn_args
                .get(1)
                .ok_or_else(|| eyre!("Did not find `left_arg` for operator `{}`.", self.name))?;
            let right_arg_graph_index = context
                .graph
                .neighbors_undirected(self_index)
                .find(|neighbor| match &context.graph[*neighbor] {
                    SqlGraphEntity::Type(ty) => ty.id_matches(&right_fn_arg.used_ty.ty_id),
                    SqlGraphEntity::Enum(en) => en.id_matches(&right_fn_arg.used_ty.ty_id),
                    SqlGraphEntity::BuiltinType(defined) => defined == &right_arg.type_name,
                    _ => false,
                })
                .ok_or_else(|| {
                    eyre!("Could not find right arg type in graph. Got: {:?}", right_arg)
                })?;
            let right_arg_sql = match right_arg.argument_sql {
                Ok(SqlMapping::As(ref sql)) => sql.clone(),
                Ok(SqlMapping::Composite { array_brackets }) => {
                    if array_brackets {
                        let composite_type = self.fn_args[1].used_ty.composite_type
                            .ok_or(eyre!("Found a composite type but macro expansion time did not reveal a name, use `pgx::composite_type!()`"))?;
                        format!("{composite_type}[]")
                    } else {
                        self.fn_args[0].used_ty.composite_type
                            .ok_or(eyre!("Found a composite type but macro expansion time did not reveal a name, use `pgx::composite_type!()`"))?.to_string()
                    }
                }
                Ok(SqlMapping::Source { array_brackets }) => {
                    if array_brackets {
                        let composite_type = context
                            .source_only_to_sql_type(self.fn_args[1].used_ty.ty_source)
                            .ok_or(eyre!(
                                "Found a source only mapping but no source mapping exists for this"
                            ))?;
                        format!("{composite_type}[]")
                    } else {
                        context.source_only_to_sql_type(self.fn_args[1].used_ty.ty_source)
                        .ok_or(eyre!("Found a composite type but macro expansion time did not reveal a name, use `pgx::composite_type!()`"))?.to_string()
                    }
                }
                Ok(SqlMapping::Skip) => {
                    return Err(eyre!(
                        "Found an skipped SQL type in an operator, this is not valid"
                    ))
                }
                Err(err) => return Err(err.into()),
            };

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
                                                    name = self.name,
                                                    module_path = self.module_path,
                                                    left_name = left_arg.type_name,
                                                    right_name = right_arg.type_name,
                                                    schema_prefix_left = context.schema_prefix_for(&left_arg_graph_index),
                                                    left_arg = left_arg_sql,
                                                    schema_prefix_right = context.schema_prefix_for(&right_arg_graph_index),
                                                    right_arg = right_arg_sql,
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
