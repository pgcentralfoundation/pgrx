use crate::sql_entity_graph::{
    SqlGraphEntity,
    SqlGraphIdentifier,
    to_sql::{ToSql, entity::ToSqlConfigEntity},
    mapping::RustSqlMapping,
    pgx_sql::PgxSql,
};

use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

/// The output of a [`PostgresEnum`](crate::datum::sql_entity_graph::PostgresEnum) from `quote::ToTokens::to_tokens`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PostgresEnumEntity {
    pub name: &'static str,
    pub file: &'static str,
    pub line: u32,
    pub full_path: &'static str,
    pub module_path: &'static str,
    pub mappings: std::collections::HashSet<RustSqlMapping>,
    pub variants: Vec<&'static str>,
    pub to_sql_config: ToSqlConfigEntity,
}

impl Hash for PostgresEnumEntity {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.full_path.hash(state);
    }
}

impl Ord for PostgresEnumEntity {
    fn cmp(&self, other: &Self) -> Ordering {
        self.file
            .cmp(other.file)
            .then_with(|| self.file.cmp(other.file))
    }
}

impl PartialOrd for PostgresEnumEntity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PostgresEnumEntity {
    pub fn id_matches(&self, candidate: &core::any::TypeId) -> bool {
        self.mappings.iter().any(|tester| *candidate == tester.id)
    }
}

impl Into<SqlGraphEntity> for PostgresEnumEntity {
    fn into(self) -> SqlGraphEntity {
        SqlGraphEntity::Enum(self)
    }
}

impl SqlGraphIdentifier for PostgresEnumEntity {
    fn dot_identifier(&self) -> String {
        format!("enum {}", self.full_path)
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

impl ToSql for PostgresEnumEntity {
    #[tracing::instrument(level = "debug", err, skip(self, context), fields(identifier = %self.rust_identifier()))]
    fn to_sql(&self, context: &PgxSql) -> eyre::Result<String> {
        let self_index = context.enums[self];
        let sql = format!(
            "\n\
                    -- {file}:{line}\n\
                    -- {full_path}\n\
                    CREATE TYPE {schema}{name} AS ENUM (\n\
                        {variants}\
                    );\
                ",
            schema = context.schema_prefix_for(&self_index),
            full_path = self.full_path,
            file = self.file,
            line = self.line,
            name = self.name,
            variants = self
                .variants
                .iter()
                .map(|variant| format!("\t'{}'", variant))
                .collect::<Vec<_>>()
                .join(",\n")
                + "\n",
        );
        tracing::trace!(%sql);
        Ok(sql)
    }
}
