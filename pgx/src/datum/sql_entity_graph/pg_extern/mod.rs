mod argument;
mod operator;
mod returning;

use eyre::eyre;

pub use argument::PgExternArgumentEntity;
pub use operator::PgOperatorEntity;
pub use returning::PgExternReturnEntity;

use pgx_utils::ExternArgs;

use super::{SqlGraphEntity, SqlGraphIdentifier, ToSql, ToSqlConfigEntity};
use pgx_utils::sql_entity_graph::SqlDeclared;
use std::cmp::Ordering;

/// The output of a [`Schema`](crate::datum::sql_entity_graph::Schema) from `quote::ToTokens::to_tokens`.
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
    fn to_sql(&self, context: &super::PgxSql) -> eyre::Result<String> {
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

        let fn_sql = format!("\
                                CREATE OR REPLACE FUNCTION {schema}\"{name}\"({arguments}) {returns}\n\
                                {extern_attrs}\
                                {search_path}\
                                LANGUAGE c /* Rust */\n\
                                AS 'MODULE_PATHNAME', '{unaliased_name}_wrapper';\
                            ",
                             schema = self.schema.map(|schema| format!("{}.", schema)).unwrap_or_else(|| context.schema_prefix_for(&self_index)),
                             name = self.name,
                             unaliased_name = self.unaliased_name,
                             arguments = if !self.fn_args.is_empty() {
                                 let mut args = Vec::new();
                                 for (idx, arg) in self.fn_args.iter().enumerate() {
                                     let graph_index = context.graph.neighbors_undirected(self_index).find(|neighbor| match &context.graph[*neighbor] {
                                         SqlGraphEntity::Type(ty) => ty.id_matches(&arg.ty_id),
                                         SqlGraphEntity::Enum(en) => en.id_matches(&arg.ty_id),
                                         SqlGraphEntity::BuiltinType(defined) => defined == &arg.full_path,
                                         _ => false,
                                     }).ok_or_else(|| eyre!("Could not find arg type in graph. Got: {:?}", arg))?;
                                     let needs_comma = idx < (self.fn_args.len() - 1);
                                     let buf = format!("\
                                            \t\"{pattern}\" {variadic}{schema_prefix}{sql_type}{default}{maybe_comma}/* {full_path} */\
                                        ",
                                            pattern = arg.pattern,
                                            schema_prefix = context.schema_prefix_for(&graph_index),
                                            // First try to match on [`TypeId`] since it's most reliable.
                                            sql_type = context.rust_to_sql(arg.ty_id, arg.ty_source, arg.full_path).ok_or_else(|| eyre!(
                                                "Failed to map argument `{}` type `{}` to SQL type while building function `{}`.",
                                                arg.pattern,
                                                arg.full_path,
                                                self.name
                                            ))?,
                                            default = if let Some(def) = arg.default { format!(" DEFAULT {}", def) } else { String::from("") },
                                            variadic = if arg.is_variadic { "VARIADIC " } else { "" },
                                            maybe_comma = if needs_comma { ", " } else { " " },
                                            full_path = arg.full_path,
                                     );
                                     args.push(buf);
                                 };
                                 String::from("\n") + &args.join("\n") + "\n"
                             } else { Default::default() },
                             returns = match &self.fn_return {
                                 PgExternReturnEntity::None => String::from("RETURNS void"),
                                 PgExternReturnEntity::Type { id, source, full_path, .. } => {
                                     let graph_index = context.graph.neighbors_undirected(self_index).find(|neighbor| match &context.graph[*neighbor] {
                                         SqlGraphEntity::Type(ty) => ty.id_matches(&id),
                                         SqlGraphEntity::Enum(en) => en.id_matches(&id),
                                         SqlGraphEntity::BuiltinType(defined) => &*defined == full_path,
                                         _ => false,
                                     }).ok_or_else(|| eyre!("Could not find return type in graph."))?;
                                     format!("RETURNS {schema_prefix}{sql_type} /* {full_path} */",
                                             sql_type = context.source_only_to_sql_type(source).or_else(|| {
                                                 context.type_id_to_sql_type(*id)
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
                                 },
                                 PgExternReturnEntity::SetOf { id, source, full_path, .. } => {
                                     let graph_index = context.graph.neighbors_undirected(self_index).find(|neighbor| match &context.graph[*neighbor] {
                                         SqlGraphEntity::Type(ty) => ty.id_matches(&id),
                                         SqlGraphEntity::Enum(en) => en.id_matches(&id),
                                         SqlGraphEntity::BuiltinType(defined) => defined == full_path,
                                         _ => false,
                                     }).ok_or_else(|| eyre!("Could not find return type in graph."))?;
                                     format!("RETURNS SETOF {schema_prefix}{sql_type} /* {full_path} */",
                                             sql_type = context.source_only_to_sql_type(source).or_else(|| {
                                                 context.type_id_to_sql_type(*id)
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
                                 },
                                 PgExternReturnEntity::Iterated(table_items) => {
                                     let mut items = String::new();
                                     for (idx, (id, source, ty_name, _module_path, col_name)) in table_items.iter().enumerate() {
                                         let graph_index = context.graph.neighbors_undirected(self_index).find(|neighbor| match &context.graph[*neighbor] {
                                             SqlGraphEntity::Type(ty) => ty.id_matches(&id),
                                             SqlGraphEntity::Enum(en) => en.id_matches(&id),
                                             SqlGraphEntity::BuiltinType(defined) => defined == ty_name,
                                             _ => false,
                                         });
                                         let needs_comma = idx < (table_items.len() - 1);
                                         let item = format!("\n\t{col_name} {schema_prefix}{ty_resolved}{needs_comma} /* {ty_name} */",
                                                            col_name = col_name.expect("An iterator of tuples should have `named!()` macro declarations."),
                                                            schema_prefix = if let Some(graph_index) = graph_index {
                                                                context.schema_prefix_for(&graph_index)
                                                            } else { "".into() },
                                                            ty_resolved = context.source_only_to_sql_type(source).or_else(|| {
                                                                context.type_id_to_sql_type(*id)
                                                            }).or_else(|| {
                                                                let pat = ty_name.to_string();
                                                                if let Some(found) = context.has_sql_declared_entity(&SqlDeclared::Type(pat.clone())) {
                                                                    Some(found.sql())
                                                                }  else if let Some(found) = context.has_sql_declared_entity(&SqlDeclared::Enum(pat.clone())) {
                                                                    Some(found.sql())
                                                                } else {
                                                                    None
                                                                }
                                                            }).ok_or_else(|| eyre!("Failed to map return type `{}` to SQL type while building function `{}`.", ty_name, self.name))?,
                                                            needs_comma = if needs_comma { ", " } else { " " },
                                                            ty_name = ty_name
                                         );
                                         items.push_str(&item);
                                     }
                                     format!("RETURNS TABLE ({}\n)", items)
                                 },
                                 PgExternReturnEntity::Trigger => String::from("RETURNS trigger"),
                             },
                             search_path = if let Some(search_path) = &self.search_path {
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
            let left_arg_graph_index = context
                .graph
                .neighbors_undirected(self_index)
                .find(|neighbor| match &context.graph[*neighbor] {
                    SqlGraphEntity::Type(ty) => ty.id_matches(&left_arg.ty_id),
                    _ => false,
                })
                .ok_or_else(|| eyre!("Could not find left arg function in graph."))?;
            let right_arg = self
                .fn_args
                .get(1)
                .ok_or_else(|| eyre!("Did not find `left_arg` for operator `{}`.", self.name))?;
            let right_arg_graph_index = context
                .graph
                .neighbors_undirected(self_index)
                .find(|neighbor| match &context.graph[*neighbor] {
                    SqlGraphEntity::Type(ty) => ty.id_matches(&right_arg.ty_id),
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
                                        left_name = left_arg.full_path,
                                        right_name = right_arg.full_path,
                                        schema_prefix_left = context.schema_prefix_for(&left_arg_graph_index),
                                        left_arg = context.type_id_to_sql_type(left_arg.ty_id).ok_or_else(|| eyre!("Failed to map argument `{}` type `{}` to SQL type while building operator `{}`.", left_arg.pattern, left_arg.full_path, self.name))?,
                                        schema_prefix_right = context.schema_prefix_for(&right_arg_graph_index),
                                        right_arg = context.type_id_to_sql_type(right_arg.ty_id).ok_or_else(|| eyre!("Failed to map argument `{}` type `{}` to SQL type while building operator `{}`.", right_arg.pattern, right_arg.full_path, self.name))?,
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
