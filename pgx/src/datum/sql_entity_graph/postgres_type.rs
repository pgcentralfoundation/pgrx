use super::RustSqlMapping;
use eyre::eyre;
use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

use super::{SqlGraphEntity, SqlGraphIdentifier, ToSql};

/// The output of a [`PostgresType`](crate::datum::sql_entity_graph::PostgresType) from `quote::ToTokens::to_tokens`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PostgresTypeEntity {
    pub name: &'static str,
    pub file: &'static str,
    pub line: u32,
    pub full_path: &'static str,
    pub module_path: &'static str,
    pub mappings: std::collections::HashSet<RustSqlMapping>,
    pub in_fn: &'static str,
    pub in_fn_module_path: String,
    pub out_fn: &'static str,
    pub out_fn_module_path: String,
}

impl Hash for PostgresTypeEntity {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.full_path.hash(state);
    }
}

impl Ord for PostgresTypeEntity {
    fn cmp(&self, other: &Self) -> Ordering {
        self.file
            .cmp(other.file)
            .then_with(|| self.file.cmp(other.file))
    }
}

impl PartialOrd for PostgresTypeEntity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PostgresTypeEntity {
    pub fn id_matches(&self, candidate: &core::any::TypeId) -> bool {
        self.mappings.iter().any(|tester| *candidate == tester.id)
    }
}

impl Into<SqlGraphEntity> for PostgresTypeEntity {
    fn into(self) -> SqlGraphEntity {
        SqlGraphEntity::Type(self)
    }
}

impl SqlGraphIdentifier for PostgresTypeEntity {
    fn dot_identifier(&self) -> String {
        format!("type {}", self.full_path)
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

impl ToSql for PostgresTypeEntity {
    #[tracing::instrument(level = "debug", err, skip(self, context), fields(identifier = %self.rust_identifier()))]
    fn to_sql(&self, context: &super::PgxSql) -> eyre::Result<String> {
        let self_index = context.types[self];
        let item_node = &context.graph[self_index];
        let item = match item_node {
            SqlGraphEntity::Type(item) => item,
            _ => return Err(eyre!("Was not called on a Type. Got: {:?}", item_node)),
        };

        // The `in_fn`/`out_fn` need to be present in a certain order:
        // - CREATE TYPE;
        // - CREATE FUNCTION _in;
        // - CREATE FUNCTION _out;
        // - CREATE TYPE (...);

        let in_fn_module_path = if !item.in_fn_module_path.is_empty() {
            item.in_fn_module_path.clone()
        } else {
            item.module_path.to_string() // Presume a local
        };
        let in_fn_path = format!(
            "{module_path}{maybe_colons}{in_fn}",
            module_path = in_fn_module_path,
            maybe_colons = if !in_fn_module_path.is_empty() {
                "::"
            } else {
                ""
            },
            in_fn = item.in_fn,
        );
        let (_, _index) = context
            .externs
            .iter()
            .find(|(k, _v)| (**k).full_path == in_fn_path.as_str())
            .ok_or_else(|| eyre::eyre!("Did not find `in_fn: {}`.", in_fn_path))?;
        let (in_fn_graph_index, in_fn) = context
            .graph
            .neighbors_undirected(self_index)
            .find_map(|neighbor| match &context.graph[neighbor] {
                SqlGraphEntity::Function(func) if func.full_path == in_fn_path => {
                    Some((neighbor, func))
                }
                _ => None,
            })
            .ok_or_else(|| eyre!("Could not find in_fn graph entity."))?;
        tracing::trace!(in_fn = ?in_fn_path, "Found matching `in_fn`");
        let in_fn_sql = in_fn.to_sql(context)?;
        tracing::trace!(%in_fn_sql);

        let out_fn_module_path = if !item.out_fn_module_path.is_empty() {
            item.out_fn_module_path.clone()
        } else {
            item.module_path.to_string() // Presume a local
        };
        let out_fn_path = format!(
            "{module_path}{maybe_colons}{out_fn}",
            module_path = out_fn_module_path,
            maybe_colons = if !out_fn_module_path.is_empty() {
                "::"
            } else {
                ""
            },
            out_fn = item.out_fn,
        );
        let (_, _index) = context
            .externs
            .iter()
            .find(|(k, _v)| (**k).full_path == out_fn_path.as_str())
            .ok_or_else(|| eyre::eyre!("Did not find `out_fn: {}`.", out_fn_path))?;
        let (out_fn_graph_index, out_fn) = context
            .graph
            .neighbors_undirected(self_index)
            .find_map(|neighbor| match &context.graph[neighbor] {
                SqlGraphEntity::Function(func) if func.full_path == out_fn_path => {
                    Some((neighbor, func))
                }
                _ => None,
            })
            .ok_or_else(|| eyre!("Could not find out_fn graph entity."))?;
        tracing::trace!(out_fn = ?out_fn_path, "Found matching `out_fn`");
        let out_fn_sql = out_fn.to_sql(context)?;
        tracing::trace!(%out_fn_sql);

        let shell_type = format!(
            "\n\
                                -- {file}:{line}\n\
                                -- {full_path}\n\
                                CREATE TYPE {schema}{name};\
                            ",
            schema = context.schema_prefix_for(&self_index),
            full_path = item.full_path,
            file = item.file,
            line = item.line,
            name = item.name,
        );
        tracing::trace!(sql = %shell_type);

        let materialized_type = format!("\n\
                                -- {file}:{line}\n\
                                -- {full_path}\n\
                                CREATE TYPE {schema}{name} (\n\
                                    \tINTERNALLENGTH = variable,\n\
                                    \tINPUT = {schema_prefix_in_fn}{in_fn}, /* {in_fn_path} */\n\
                                    \tOUTPUT = {schema_prefix_out_fn}{out_fn}, /* {out_fn_path} */\n\
                                    \tSTORAGE = extended\n\
                                );\
                            ",
                                        full_path = item.full_path,
                                        file = item.file,
                                        line = item.line,
                                        schema = context.schema_prefix_for(&self_index),
                                        name = item.name,
                                        schema_prefix_in_fn = context.schema_prefix_for(&in_fn_graph_index),
                                        in_fn = item.in_fn,
                                        in_fn_path = in_fn_path,
                                        schema_prefix_out_fn = context.schema_prefix_for(&out_fn_graph_index),
                                        out_fn = item.out_fn,
                                        out_fn_path = out_fn_path,
        );
        tracing::trace!(sql = %materialized_type);

        Ok(shell_type + "\n" + &in_fn_sql + "\n" + &out_fn_sql + "\n" + &materialized_type)
    }
}
