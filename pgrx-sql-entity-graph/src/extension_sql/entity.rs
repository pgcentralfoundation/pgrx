//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
/*!

`pgrx::extension_sql!()` related entities for Rust to SQL translation

> Like all of the [`sql_entity_graph`][crate] APIs, this is considered **internal**
> to the `pgrx` framework and very subject to change between versions. While you may use this, please do it with caution.


*/
use crate::extension_sql::SqlDeclared;
use crate::metadata::{SqlMapping, SqlTranslatable, TypeOrigin};
use crate::pgrx_sql::PgrxSql;
use crate::positioning_ref::PositioningRef;
use crate::to_sql::ToSql;
use crate::{SqlGraphEntity, SqlGraphIdentifier};

use std::fmt::Display;

/// The output of a [`ExtensionSql`](crate::ExtensionSql) from `quote::ToTokens::to_tokens`.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExtensionSqlEntity<'a> {
    pub module_path: &'a str,
    pub full_path: &'a str,
    pub sql: &'a str,
    pub file: &'a str,
    pub line: u32,
    pub name: &'a str,
    pub bootstrap: bool,
    pub finalize: bool,
    pub requires: Vec<PositioningRef>,
    pub creates: Vec<SqlDeclaredEntity>,
}

impl ExtensionSqlEntity<'_> {
    pub fn has_sql_declared_entity(&self, identifier: &SqlDeclared) -> Option<&SqlDeclaredEntity> {
        self.creates.iter().find(|created| created.has_sql_declared_entity(identifier))
    }
}

impl<'a> From<ExtensionSqlEntity<'a>> for SqlGraphEntity<'a> {
    fn from(val: ExtensionSqlEntity<'a>) -> Self {
        SqlGraphEntity::CustomSql(val)
    }
}

impl SqlGraphIdentifier for ExtensionSqlEntity<'_> {
    fn dot_identifier(&self) -> String {
        format!("sql {}", self.name)
    }
    fn rust_identifier(&self) -> String {
        self.name.to_string()
    }

    fn file(&self) -> Option<&str> {
        Some(self.file)
    }

    fn line(&self) -> Option<u32> {
        Some(self.line)
    }
}

impl ToSql for ExtensionSqlEntity<'_> {
    fn to_sql(&self, _context: &PgrxSql) -> eyre::Result<String> {
        let ExtensionSqlEntity { file, line, sql, creates, requires, .. } = self;
        let creates = if !creates.is_empty() {
            let joined = creates.iter().map(|i| format!("--   {i}")).collect::<Vec<_>>().join("\n");
            format!(
                "\
                -- creates:\n\
                {joined}\n\n"
            )
        } else {
            "".to_string()
        };
        let requires = if !requires.is_empty() {
            let joined =
                requires.iter().map(|i| format!("--   {i}")).collect::<Vec<_>>().join("\n");
            format!(
                "\
               -- requires:\n\
                {joined}\n\n"
            )
        } else {
            "".to_string()
        };
        let sql = format!(
            "\n\
                -- {file}:{line}\n\
                {bootstrap}\
                {creates}\
                {requires}\
                {finalize}\
                {sql}\
                ",
            bootstrap = if self.bootstrap { "-- bootstrap\n" } else { "" },
            finalize = if self.finalize { "-- finalize\n" } else { "" },
        );
        Ok(sql)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct SqlDeclaredEntityData {
    pub(crate) sql: String,
    pub(crate) name: String,
    pub(crate) schema_key: String,
    pub(crate) type_origin: Option<TypeOrigin>,
}
#[derive(Debug, Clone, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub enum SqlDeclaredEntity {
    Type(SqlDeclaredEntityData),
    Enum(SqlDeclaredEntityData),
    Function(SqlDeclaredEntityData),
}

impl Display for SqlDeclaredEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SqlDeclaredEntity::Type(data) => {
                write!(f, "Type({})", data.name)
            }
            SqlDeclaredEntity::Enum(data) => {
                write!(f, "Enum({})", data.name)
            }
            SqlDeclaredEntity::Function(data) => {
                write!(f, "Function({})", data.name)
            }
        }
    }
}

impl SqlDeclaredEntity {
    pub fn build(variant: &str, name: &str) -> eyre::Result<Self> {
        let data = SqlDeclaredEntityData {
            sql: name
                .split("::")
                .last()
                .ok_or_else(|| eyre::eyre!("Did not get SQL for `{}`", name))?
                .to_string(),
            name: name.to_string(),
            schema_key: name.to_string(),
            type_origin: None,
        };
        let retval = match variant {
            "Type" => Self::Type(data),
            "Enum" => Self::Enum(data),
            "Function" => Self::Function(data),
            _ => {
                return Err(eyre::eyre!(
                    "Can only declare `Type(Ident)`, `Enum(Ident)` or `Function(Ident)`"
                ));
            }
        };
        Ok(retval)
    }

    pub fn build_type<T: SqlTranslatable>(variant: &str, name: &str) -> eyre::Result<Self> {
        let sql = match T::argument_sql() {
            Ok(SqlMapping::As(sql)) => sql,
            Ok(SqlMapping::Composite { .. }) => {
                return Err(eyre::eyre!(
                    "`creates = [{variant}(...)]` requires a concrete SQL type name"
                ));
            }
            Ok(SqlMapping::Skip) => {
                return Err(eyre::eyre!(
                    "`creates = [{variant}(...)]` cannot use a skipped SQL type"
                ));
            }
            Err(err) => return Err(err.into()),
        };
        let data = SqlDeclaredEntityData {
            sql,
            name: name.to_string(),
            schema_key: T::SCHEMA_KEY.to_string(),
            type_origin: Some(T::TYPE_ORIGIN),
        };
        let retval = match variant {
            "Type" => Self::Type(data),
            "Enum" => Self::Enum(data),
            _ => {
                return Err(eyre::eyre!(
                    "Can only declare `Type(Ident)` or `Enum(Ident)` with type metadata"
                ));
            }
        };
        Ok(retval)
    }

    pub fn sql(&self) -> String {
        match self {
            SqlDeclaredEntity::Type(data) => data.sql.clone(),
            SqlDeclaredEntity::Enum(data) => data.sql.clone(),
            SqlDeclaredEntity::Function(data) => data.sql.clone(),
        }
    }

    pub fn schema_key(&self) -> Option<&str> {
        match self {
            SqlDeclaredEntity::Type(data) | SqlDeclaredEntity::Enum(data) => {
                Some(data.schema_key.as_str())
            }
            SqlDeclaredEntity::Function(_) => None,
        }
    }

    pub fn type_origin(&self) -> Option<TypeOrigin> {
        match self {
            SqlDeclaredEntity::Type(data) | SqlDeclaredEntity::Enum(data) => data.type_origin,
            SqlDeclaredEntity::Function(_) => None,
        }
    }

    pub fn matches_schema_key(&self, schema_key: &str) -> bool {
        matches!(self.schema_key(), Some(value) if value == schema_key)
    }

    pub fn has_sql_declared_entity(&self, identifier: &SqlDeclared) -> bool {
        match (&identifier, &self) {
            (SqlDeclared::Type(ident_name), &SqlDeclaredEntity::Type(data))
            | (SqlDeclared::Enum(ident_name), &SqlDeclaredEntity::Enum(data))
            | (SqlDeclared::Function(ident_name), &SqlDeclaredEntity::Function(data)) => {
                if ident_name == &data.name || ident_name == &data.schema_key {
                    return true;
                }
                false
            }
            _ => false,
        }
    }
}
