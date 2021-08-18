use super::{DotIdentifier, SqlGraphEntity, ToSql};
use std::{
    io::Write,
    fs::{create_dir_all, File}
};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct InventorySchema {
    pub module_path: &'static str,
    pub name: &'static str,
    pub file: &'static str,
    pub line: u32,
}

impl<'a> Into<SqlGraphEntity<'a>> for &'a InventorySchema {
    fn into(self) -> SqlGraphEntity<'a> {
        SqlGraphEntity::Schema(self)
    }
}

impl DotIdentifier for InventorySchema {
    fn dot_identifier(&self) -> String {
        format!("schema {}", self.module_path.to_string())
    }
}

impl ToSql for InventorySchema {
    #[tracing::instrument(level = "debug", err, skip(self, _context), fields(identifier = self.module_path))]
    fn to_sql(&self, _context: &super::PgxSql) -> eyre::Result<String> {
        let sql = format!(
            "\n\
                    -- {file}:{line}\n\
                    CREATE SCHEMA IF NOT EXISTS {name}; /* {module_path} */\
                ",
            name = self.name,
            file = self.file,
            line = self.line,
            module_path = self.module_path,
        );
        tracing::debug!(%sql);
        Ok(sql)
    }
}
