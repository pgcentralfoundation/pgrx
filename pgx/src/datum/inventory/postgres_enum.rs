use std::{
    hash::{Hash, Hasher},
    io::Write,
    fs::{create_dir_all, File},
};
use serde::{Deserialize, Serialize};

use super::{DotIdentifier, SqlGraphEntity, ToSql};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InventoryPostgresEnum {
    pub name: &'static str,
    pub file: &'static str,
    pub line: u32,
    pub full_path: &'static str,
    pub module_path: &'static str,
    pub mappings: std::collections::HashSet<super::RustSqlMapping>,
    pub variants: Vec<&'static str>,
}

impl crate::PostgresType for InventoryPostgresEnum {}

impl Hash for InventoryPostgresEnum {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.full_path.hash(state);
    }
}

impl PartialOrd for InventoryPostgresEnum {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.full_path.partial_cmp(&other.full_path)
    }
}

impl Ord for InventoryPostgresEnum {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.full_path.cmp(&other.full_path)
    }
}

impl InventoryPostgresEnum {
    pub fn id_matches(&self, candidate: &core::any::TypeId) -> bool {
        self.mappings.iter().any(|tester| format!("{:?}", *candidate) == tester.id)
    }
    pub fn id_str_matches(&self, candidate: &str) -> bool {
        self.mappings.iter().any(|tester| candidate == tester.id)
    }
}

impl<'a> Into<SqlGraphEntity<'a>> for &'a InventoryPostgresEnum {
    fn into(self) -> SqlGraphEntity<'a> {
        SqlGraphEntity::Enum(self)
    }
}

impl DotIdentifier for InventoryPostgresEnum {
    fn dot_identifier(&self) -> String {
        format!("enum {}", self.full_path.to_string())
    }
}

impl ToSql for InventoryPostgresEnum {
    #[tracing::instrument(level = "debug", err, skip(self, context), fields(identifier = self.full_path))]
    fn to_sql(&self, context: &super::PgxSql) -> eyre::Result<String> {
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
        tracing::debug!(%sql);
        Ok(sql)
    }
}