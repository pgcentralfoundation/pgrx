use std::{io::Write, fs::{create_dir_all, File}};

use super::{DotIdentifier, SqlGraphEntity, ToSql};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct InventoryPostgresHash {
    pub name: &'static str,
    pub file: &'static str,
    pub line: u32,
    pub full_path: &'static str,
    pub module_path: &'static str,
    pub id: core::any::TypeId,
}

impl<'a> Into<SqlGraphEntity<'a>> for &'a InventoryPostgresHash {
    fn into(self) -> SqlGraphEntity<'a> {
        SqlGraphEntity::Hash(self)
    }
}

impl DotIdentifier for InventoryPostgresHash {
    fn dot_identifier(&self) -> String {
        format!("hash {}", self.full_path.to_string())
    }
}

impl ToSql for InventoryPostgresHash {
    #[tracing::instrument(level = "debug", err, skip(self, _context), fields(identifier = self.full_path))]
    fn to_sql(&self, _context: &super::PgxSql) -> eyre::Result<String> {
        let sql = format!("\n\
                            -- {file}:{line}\n\
                            -- {full_path}\n\
                            CREATE OPERATOR FAMILY {name}_hash_ops USING hash;\n\
                            CREATE OPERATOR CLASS {name}_hash_ops DEFAULT FOR TYPE {name} USING hash FAMILY {name}_hash_ops AS\n\
                                \tOPERATOR    1   =  ({name}, {name}),\n\
                                \tFUNCTION    1   {name}_hash({name});\
                            ",
                          name = self.name,
                          full_path = self.full_path,
                          file = self.file,
                          line = self.line,
        );
        tracing::debug!(%sql);
        Ok(sql)
    }
}
