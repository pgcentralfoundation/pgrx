use super::{DotIdentifier, SqlGraphEntity, ToSql};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct InventoryPostgresOrd {
    pub name: &'static str,
    pub file: &'static str,
    pub line: u32,
    pub full_path: &'static str,
    pub module_path: &'static str,
    pub id: core::any::TypeId,
}

impl Into<SqlGraphEntity> for InventoryPostgresOrd {
    fn into(self) -> SqlGraphEntity {
        SqlGraphEntity::Ord(self)
    }
}

impl DotIdentifier for InventoryPostgresOrd {
    fn dot_identifier(&self) -> String {
        format!("ord {}", self.full_path.to_string())
    }
}

impl ToSql for InventoryPostgresOrd {
    #[tracing::instrument(level = "debug", err, skip(self, _context), fields(identifier = self.full_path))]
    fn to_sql(&self, _context: &super::PgxSql) -> eyre::Result<String> {
        let sql = format!("\n\
                            -- {file}:{line}\n\
                            -- {full_path}\n\
                            CREATE OPERATOR FAMILY {name}_btree_ops USING btree;\n\
                            CREATE OPERATOR CLASS {name}_btree_ops DEFAULT FOR TYPE {name} USING btree FAMILY {name}_btree_ops AS\n\
                                  \tOPERATOR 1 <,\n\
                                  \tOPERATOR 2 <=,\n\
                                  \tOPERATOR 3 =,\n\
                                  \tOPERATOR 4 >=,\n\
                                  \tOPERATOR 5 >,\n\
                                  \tFUNCTION 1 {name}_cmp({name}, {name});\
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
