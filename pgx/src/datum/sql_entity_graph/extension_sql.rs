use std::fmt::Display;

use super::{PositioningRef, SqlGraphEntity, SqlGraphIdentifier, ToSql};
use pgx_utils::sql_entity_graph::SqlDeclared;

/// The output of a [`ExtensionSql`](crate::datum::sql_entity_graph::ExtensionSql) from `quote::ToTokens::to_tokens`.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct ExtensionSqlEntity {
    pub module_path: &'static str,
    pub full_path: &'static str,
    pub sql: &'static str,
    pub file: &'static str,
    pub line: u32,
    pub name: &'static str,
    pub bootstrap: bool,
    pub finalize: bool,
    pub requires: Vec<PositioningRef>,
    pub creates: Vec<SqlDeclaredEntity>,
}

impl ExtensionSqlEntity {
    pub fn has_sql_declared_entity(
        &self,
        identifier: &pgx_utils::sql_entity_graph::SqlDeclared,
    ) -> Option<&SqlDeclaredEntity> {
        self.creates
            .iter()
            .find(|created| created.has_sql_declared_entity(identifier))
    }
}

impl Into<SqlGraphEntity> for ExtensionSqlEntity {
    fn into(self) -> SqlGraphEntity {
        SqlGraphEntity::CustomSql(self)
    }
}

impl SqlGraphIdentifier for ExtensionSqlEntity {
    fn dot_identifier(&self) -> String {
        format!("sql {}", self.name)
    }
    fn rust_identifier(&self) -> String {
        self.name.to_string()
    }

    fn file(&self) -> Option<&'static str> {
        Some(self.file)
    }

    fn line(&self) -> Option<u32> {
        Some(self.line)
    }
}

impl ToSql for ExtensionSqlEntity {
    #[tracing::instrument(level = "debug", skip(self, _context), fields(identifier = self.full_path))]
    fn to_sql(&self, _context: &super::PgxSql) -> eyre::Result<String> {
        let sql = format!(
            "\n\
                -- {file}:{line}\n\
                {bootstrap}\
                {creates}\
                {requires}\
                {finalize}\
                {sql}\
                ",
            file = self.file,
            line = self.line,
            bootstrap = if self.bootstrap { "-- bootstrap\n" } else { "" },
            creates = if !self.creates.is_empty() {
                format!(
                    "\
                    -- creates:\n\
                    {}\n\
                ",
                    self.creates
                        .iter()
                        .map(|i| format!("--   {}", i))
                        .collect::<Vec<_>>()
                        .join("\n")
                ) + "\n"
            } else {
                "".to_string()
            },
            requires = if !self.requires.is_empty() {
                format!(
                    "\
                   -- requires:\n\
                    {}\n\
                ",
                    self.requires
                        .iter()
                        .map(|i| format!("--   {}", i))
                        .collect::<Vec<_>>()
                        .join("\n")
                ) + "\n"
            } else {
                "".to_string()
            },
            finalize = if self.finalize { "-- finalize\n" } else { "" },
            sql = self.sql,
        );
        tracing::debug!(%sql);
        Ok(sql)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub enum SqlDeclaredEntity {
    Type {
        sql: String,
        name: String,
        option: String,
        vec: String,
        vec_option: String,
        option_vec: String,
        option_vec_option: String,
        array: String,
        option_array: String,
        varlena: String,
        pg_box: String,
    },
    Enum {
        sql: String,
        name: String,
        option: String,
        vec: String,
        vec_option: String,
        option_vec: String,
        option_vec_option: String,
        array: String,
        option_array: String,
        varlena: String,
        pg_box: String,
    },
    Function {
        sql: String,
        name: String,
        option: String,
        vec: String,
        vec_option: String,
        option_vec: String,
        option_vec_option: String,
        array: String,
        option_array: String,
        varlena: String,
        pg_box: String,
    },
}

impl Display for SqlDeclaredEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SqlDeclaredEntity::Type { name, .. } => {
                f.write_str(&(String::from("Type(") + name + ")"))
            }
            SqlDeclaredEntity::Enum { name, .. } => {
                f.write_str(&(String::from("Enum(") + name + ")"))
            }
            SqlDeclaredEntity::Function { name, .. } => {
                f.write_str(&(String::from("Function ") + name + ")"))
            }
        }
    }
}

impl SqlDeclaredEntity {
    pub fn build(variant: impl AsRef<str>, name: impl AsRef<str>) -> eyre::Result<Self> {
        let name = name.as_ref();
        let retval = match variant.as_ref() {
            "Type" => Self::Type {
                sql: name
                    .split("::")
                    .last()
                    .ok_or_else(|| eyre::eyre!("Did not get SQL for `{}`", name))?
                    .to_string(),
                name: name.to_string(),
                option: format!("Option<{}>", name),
                vec: format!("Vec<{}>", name),
                vec_option: format!("Vec<Option<{}>>", name),
                option_vec: format!("Option<Vec<{}>>", name),
                option_vec_option: format!("Option<Vec<Option<{}>>", name),
                array: format!("Array<{}>", name),
                option_array: format!("Option<{}>", name),
                varlena: format!("Varlena<{}>", name),
                pg_box: format!("pgx::pgbox::PgBox<{}>", name),
            },
            "Enum" => Self::Enum {
                sql: name
                    .split("::")
                    .last()
                    .ok_or_else(|| eyre::eyre!("Did not get SQL for `{}`", name))?
                    .to_string(),
                name: name.to_string(),
                option: format!("Option<{}>", name),
                vec: format!("Vec<{}>", name),
                vec_option: format!("Vec<Option<{}>>", name),
                option_vec: format!("Option<Vec<{}>>", name),
                option_vec_option: format!("Option<Vec<Option<{}>>", name),
                array: format!("Array<{}>", name),
                option_array: format!("Option<{}>", name),
                varlena: format!("Varlena<{}>", name),
                pg_box: format!("pgx::pgbox::PgBox<{}>", name),
            },
            "function" => Self::Function {
                sql: name
                    .split("::")
                    .last()
                    .ok_or_else(|| eyre::eyre!("Did not get SQL for `{}`", name))?
                    .to_string(),
                name: name.to_string(),
                option: format!("Option<{}>", name),
                vec: format!("Vec<{}>", name),
                vec_option: format!("Vec<Option<{}>>", name),
                option_vec: format!("Option<Vec<{}>>", name),
                option_vec_option: format!("Option<Vec<Option<{}>>", name),
                array: format!("Array<{}>", name),
                option_array: format!("Option<{}>", name),
                varlena: format!("Varlena<{}>", name),
                pg_box: format!("pgx::pgbox::PgBox<{}>", name),
            },
            _ => {
                return Err(eyre::eyre!(
                    "Can only declare `Type(Ident)`, `Enum(Ident)` or `Function(Ident)`"
                ))
            }
        };
        Ok(retval)
    }
    pub fn sql(&self) -> String {
        match self {
            SqlDeclaredEntity::Type { sql, .. } => sql.clone(),
            SqlDeclaredEntity::Enum { sql, .. } => sql.clone(),
            SqlDeclaredEntity::Function { sql, .. } => sql.clone(),
        }
    }

    pub fn has_sql_declared_entity(&self, identifier: &SqlDeclared) -> bool {
        match (&identifier, &self) {
            (
                SqlDeclared::Type(identifier_name),
                &SqlDeclaredEntity::Type {
                    sql: _sql,
                    name,
                    option,
                    vec,
                    vec_option,
                    option_vec,
                    option_vec_option,
                    array,
                    option_array,
                    varlena,
                    pg_box,
                },
            ) => {
                identifier_name == name
                    || identifier_name == option
                    || identifier_name == vec
                    || identifier_name == vec_option
                    || identifier_name == option_vec
                    || identifier_name == option_vec_option
                    || identifier_name == array
                    || identifier_name == option_array
                    || identifier_name == varlena
                    || identifier_name == pg_box
            }
            (
                SqlDeclared::Enum(identifier_name),
                &SqlDeclaredEntity::Enum {
                    sql: _sql,
                    name,
                    option,
                    vec,
                    vec_option,
                    option_vec,
                    option_vec_option,
                    array,
                    option_array,
                    varlena,
                    pg_box,
                },
            ) => {
                identifier_name == name
                    || identifier_name == option
                    || identifier_name == vec
                    || identifier_name == vec_option
                    || identifier_name == option_vec
                    || identifier_name == option_vec_option
                    || identifier_name == array
                    || identifier_name == option_array
                    || identifier_name == varlena
                    || identifier_name == pg_box
            }
            (
                SqlDeclared::Function(identifier_name),
                &SqlDeclaredEntity::Function {
                    sql: _sql,
                    name,
                    option,
                    vec,
                    vec_option,
                    option_vec,
                    option_vec_option,
                    array,
                    option_array,
                    varlena,
                    pg_box,
                },
            ) => {
                identifier_name == name
                    || identifier_name == option
                    || identifier_name == vec
                    || identifier_name == vec_option
                    || identifier_name == option_vec
                    || identifier_name == option_vec_option
                    || identifier_name == array
                    || identifier_name == option_array
                    || identifier_name == varlena
                    || identifier_name == pg_box
            }
            _ => false,
        }
    }
}
