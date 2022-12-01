/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
/*!

`#[derive(PostgresType)]` related entities for Rust to SQL translation

> Like all of the [`sql_entity_graph`][crate::sql_entity_graph] APIs, this is considered **internal**
to the `pgx` framework and very subject to change between versions. While you may use this, please do it with caution.

*/
use crate::sql_entity_graph::mapping::RustSqlMapping;
use crate::sql_entity_graph::pgx_sql::PgxSql;
use crate::sql_entity_graph::to_sql::entity::ToSqlConfigEntity;
use crate::sql_entity_graph::to_sql::ToSql;
use crate::sql_entity_graph::{SqlGraphEntity, SqlGraphIdentifier};

use eyre::eyre;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

/// The output of a [`PostgresType`](crate::sql_entity_graph::postgres_type::PostgresType) from `quote::ToTokens::to_tokens`.
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
    pub send_fn: Option<&'static str>,
    pub send_fn_module_path: String,
    pub recv_fn: Option<&'static str>,
    pub recv_fn_module_path: String,
    pub to_sql_config: ToSqlConfigEntity,
}

impl Hash for PostgresTypeEntity {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.full_path.hash(state);
    }
}

impl Ord for PostgresTypeEntity {
    fn cmp(&self, other: &Self) -> Ordering {
        self.file.cmp(other.file).then_with(|| self.file.cmp(other.file))
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

impl From<PostgresTypeEntity> for SqlGraphEntity {
    fn from(val: PostgresTypeEntity) -> Self {
        SqlGraphEntity::Type(val)
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
    fn to_sql(&self, context: &PgxSql) -> eyre::Result<String> {
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
        // - CREATE FUNCTION _send; (optional)
        // - CREATE FUNCTION _recv; (optional)
        // - CREATE TYPE (...);

        let mut functions = String::new();

        let in_fn_module_path = if !item.in_fn_module_path.is_empty() {
            item.in_fn_module_path.clone()
        } else {
            item.module_path.to_string() // Presume a local
        };
        let in_fn_path = format!(
            "{module_path}{maybe_colons}{in_fn}",
            module_path = in_fn_module_path,
            maybe_colons = if !in_fn_module_path.is_empty() { "::" } else { "" },
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
        functions.push_str(in_fn_sql.as_str());
        functions.push('\n');

        let out_fn_module_path = if !item.out_fn_module_path.is_empty() {
            item.out_fn_module_path.clone()
        } else {
            item.module_path.to_string() // Presume a local
        };
        let out_fn_path = format!(
            "{module_path}{maybe_colons}{out_fn}",
            module_path = out_fn_module_path,
            maybe_colons = if !out_fn_module_path.is_empty() { "::" } else { "" },
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
        functions.push_str(out_fn_sql.as_str());
        functions.push('\n');

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

        let full_path = item.full_path;
        let file = item.file;
        let line = item.line;
        let schema = context.schema_prefix_for(&self_index);
        let name = item.name;
        let schema_prefix_in_fn = context.schema_prefix_for(&in_fn_graph_index);
        let in_fn = item.in_fn;
        let in_fn_path = in_fn_path;
        let schema_prefix_out_fn = context.schema_prefix_for(&out_fn_graph_index);
        let out_fn = item.out_fn;
        let out_fn_path = out_fn_path;

        let materialized_type = match (item.send_fn, item.recv_fn) {
            (Some(send_fn), Some(recv_fn)) => {
                let send_fn_module_path = if !item.send_fn_module_path.is_empty() {
                    item.send_fn_module_path.clone()
                } else {
                    item.module_path.to_string() // Presume a local
                };
                let send_fn_path = format!(
                    "{module_path}{maybe_colons}{send_fn}",
                    module_path = send_fn_module_path,
                    maybe_colons = if !send_fn_module_path.is_empty() { "::" } else { "" },
                );
                let (_, _index) = context
                    .externs
                    .iter()
                    .find(|(k, _v)| (**k).full_path == send_fn_path.as_str())
                    .ok_or_else(|| eyre::eyre!("Did not find `send_fn: {}`.", send_fn_path))?;
                let (send_fn_graph_index, send_fn) = context
                    .graph
                    .neighbors_undirected(self_index)
                    .find_map(|neighbor| match &context.graph[neighbor] {
                        SqlGraphEntity::Function(func) if func.full_path == send_fn_path => {
                            Some((neighbor, func))
                        }
                        _ => None,
                    })
                    .ok_or_else(|| eyre!("Could not find send_fn graph entity."))?;
                tracing::trace!(send_fn = ?send_fn_path, "Found matching `send_fn`");
                let send_fn_sql = send_fn.to_sql(context)?;
                tracing::trace!(%send_fn_sql);
                functions.push_str(send_fn_sql.as_str());
                functions.push('\n');

                let recv_fn_module_path = if !item.recv_fn_module_path.is_empty() {
                    item.recv_fn_module_path.clone()
                } else {
                    item.module_path.to_string() // Presume a local
                };
                let recv_fn_path = format!(
                    "{module_path}{maybe_colons}{recv_fn}",
                    module_path = recv_fn_module_path,
                    maybe_colons = if !recv_fn_module_path.is_empty() { "::" } else { "" },
                );
                let (_, _index) = context
                    .externs
                    .iter()
                    .find(|(k, _v)| (**k).full_path == recv_fn_path.as_str())
                    .ok_or_else(|| eyre::eyre!("Did not find `recv_fn: {}`.", recv_fn_path))?;
                let (recv_fn_graph_index, recv_fn) = context
                    .graph
                    .neighbors_undirected(self_index)
                    .find_map(|neighbor| match &context.graph[neighbor] {
                        SqlGraphEntity::Function(func) if func.full_path == recv_fn_path => {
                            Some((neighbor, func))
                        }
                        _ => None,
                    })
                    .ok_or_else(|| eyre!("Could not find recv_fn graph entity."))?;
                tracing::trace!(recv_fn = ?recv_fn_path, "Found matching `recv_fn`");
                let recv_fn_sql = recv_fn.to_sql(context)?;
                tracing::trace!(%recv_fn_sql);
                functions.push_str(recv_fn_sql.as_str());
                functions.push('\n');

                let schema_prefix_send_fn = context.schema_prefix_for(&send_fn_graph_index);
                let send_fn = item.send_fn.unwrap();
                let send_fn_path = send_fn_path;
                let schema_prefix_recv_fn = context.schema_prefix_for(&recv_fn_graph_index);
                let recv_fn = item.recv_fn.unwrap();
                let recv_fn_path = recv_fn_path;
                format!("\n\
                                -- {file}:{line}\n\
                                -- {full_path}\n\
                                CREATE TYPE {schema}{name} (\n\
                                    \tINTERNALLENGTH = variable,\n\
                                    \tINPUT = {schema_prefix_in_fn}{in_fn}, /* {in_fn_path} */\n\
                                    \tOUTPUT = {schema_prefix_out_fn}{out_fn}, /* {out_fn_path} */\n\
                                    \tSEND = {schema_prefix_send_fn}{send_fn}, /* {send_fn_path} */\n\
                                    \tRECEIVE = {schema_prefix_recv_fn}{recv_fn}, /* {recv_fn_path} */\n\
                                    \tSTORAGE = extended\n\
                                );\
                            "
                )
            }
            _ => {
                format!("\n\
                                -- {file}:{line}\n\
                                -- {full_path}\n\
                                CREATE TYPE {schema}{name} (\n\
                                    \tINTERNALLENGTH = variable,\n\
                                    \tINPUT = {schema_prefix_in_fn}{in_fn}, /* {in_fn_path} */\n\
                                    \tOUTPUT = {schema_prefix_out_fn}{out_fn}, /* {out_fn_path} */\n\
                                    \tSTORAGE = extended\n\
                                );\
                            "
                )
            }
        };

        tracing::trace!(sql = %materialized_type);

        Ok(shell_type + "\n" + &functions + &materialized_type)
    }
}
