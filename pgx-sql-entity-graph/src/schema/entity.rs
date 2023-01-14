/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
/*!

`#[pg_schema]` related entities for Rust to SQL translation

> Like all of the [`sql_entity_graph`][crate::pgx_sql_entity_graph] APIs, this is considered **internal**
to the `pgx` framework and very subject to change between versions. While you may use this, please do it with caution.

*/
use crate::pgx_sql::PgxSql;
use crate::to_sql::ToSql;
use crate::{SqlGraphEntity, SqlGraphIdentifier};

/// The output of a [`Schema`](crate::schema::Schema) from `quote::ToTokens::to_tokens`.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct SchemaEntity {
    pub module_path: &'static str,
    pub name: &'static str,
    pub file: &'static str,
    pub line: u32,
}

impl From<SchemaEntity> for SqlGraphEntity {
    fn from(val: SchemaEntity) -> Self {
        SqlGraphEntity::Schema(val)
    }
}

impl SqlGraphIdentifier for SchemaEntity {
    fn dot_identifier(&self) -> String {
        format!("schema {}", self.module_path)
    }
    fn rust_identifier(&self) -> String {
        self.module_path.to_string()
    }

    fn file(&self) -> Option<&'static str> {
        Some(self.file)
    }

    fn line(&self) -> Option<u32> {
        Some(self.line)
    }
}

impl ToSql for SchemaEntity {
    #[tracing::instrument(level = "debug", err, skip(self, _context), fields(identifier = %self.rust_identifier()))]
    fn to_sql(&self, _context: &PgxSql) -> eyre::Result<String> {
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
        tracing::trace!(%sql);
        Ok(sql)
    }
}
