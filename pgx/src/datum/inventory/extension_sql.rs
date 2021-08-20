use std::fmt::Display;

use super::{DotIdentifier, SqlGraphEntity, ToSql};
use pgx_utils::pg_inventory::SqlDeclaredEntity;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct InventoryExtensionSql {
    pub module_path: &'static str,
    pub full_path: &'static str,
    pub sql: &'static str,
    pub file: &'static str,
    pub line: u32,
    pub name: Option<&'static str>,
    pub bootstrap: bool,
    pub finalize: bool,
    pub before: Vec<InventoryExtensionSqlPositioningRef<'static>>,
    pub after: Vec<InventoryExtensionSqlPositioningRef<'static>>,
    pub creates: Vec<InventorySqlDeclaredEntity>,
}

impl InventoryExtensionSql {
    pub fn identifier(&self) -> &str {
        self.name.unwrap_or(self.full_path)
    }

    pub fn has_sql_declared_entity(
        &self,
        identifier: &pgx_utils::pg_inventory::SqlDeclaredEntity,
    ) -> Option<&InventorySqlDeclaredEntity> {
        self.creates
            .iter()
            .find(|created| created.has_sql_declared_entity(identifier))
    }
}

impl Into<SqlGraphEntity> for InventoryExtensionSql {
    fn into(self) -> SqlGraphEntity {
        SqlGraphEntity::CustomSql(self)
    }
}

impl DotIdentifier for InventoryExtensionSql {
    fn dot_identifier(&self) -> String {
        format!("schema {}", self.full_path.to_string())
    }
}

impl ToSql for InventoryExtensionSql {
    #[tracing::instrument(level = "debug", skip(self, _context), fields(identifier = self.full_path))]
    fn to_sql(&self, _context: &super::PgxSql) -> eyre::Result<String> {
        let sql = format!(
            "\n\
                -- {file}:{line}\n\
                {bootstrap}\
                {creates}\
                {before}\
                {after}\
                {finalize}\
                {sql}\
                ",
            file = self.file,
            line = self.line,
            bootstrap = if self.bootstrap {
                "-- bootstrap\n"
            } else { "" },
            creates = if !self.creates.is_empty() {
                format!("\
                    -- creates:\n\
                    {}\n\
                ", self.creates.iter().map(|i| 
                    format!("--   {}", i)
                ).collect::<Vec<_>>().join("\n")) + "\n"
            } else {
                "".to_string()
            },
            before = if !self.before.is_empty() {
                format!("\
                    -- before:\n\
                    {}\n\
                ", self.before.iter().map(|i| 
                    format!("--   {}", i)
                ).collect::<Vec<_>>().join("\n")) + "\n"
            } else {
                "".to_string()
            },
            after = if !self.after.is_empty() {
                format!("\
                   -- after\n\
                    {}\n\
                ", self.after.iter().map(|i| 
                    format!("--   {}", i)
                ).collect::<Vec<_>>().join("\n")) + "\n"
            } else {
                "".to_string()
            },
            finalize = if self.finalize {
                "-- finalize\n"
            } else { "" },
            sql = self.sql,
        );
        tracing::debug!(%sql);
        Ok(sql)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum InventoryExtensionSqlPositioningRef<'a> {
    FullPath(&'a str),
    Name(&'a str),
}

impl<'a> Display for InventoryExtensionSqlPositioningRef<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InventoryExtensionSqlPositioningRef::FullPath(i) => f.write_str(i),
            InventoryExtensionSqlPositioningRef::Name(i) => f.write_str(i),
        }
    }
}


#[derive(Debug, Clone, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub enum InventorySqlDeclaredEntity {
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

impl Display for InventorySqlDeclaredEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InventorySqlDeclaredEntity::Type { name, .. } => f.write_str(&(String::from("Type(") + name + ")")),
            InventorySqlDeclaredEntity::Enum { name, .. } => f.write_str(&(String::from("Enum(") + name + ")")),
            InventorySqlDeclaredEntity::Function { name, .. } => f.write_str(&(String::from("Function ") + name + ")")),
        }
    }
}

impl InventorySqlDeclaredEntity {
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
            InventorySqlDeclaredEntity::Type { sql, .. } => sql.clone(),
            InventorySqlDeclaredEntity::Enum { sql, .. } => sql.clone(),
            InventorySqlDeclaredEntity::Function { sql, .. } => sql.clone(),
        }
    }

    pub fn has_sql_declared_entity(&self, identifier: &SqlDeclaredEntity) -> bool {
        match (&identifier, &self) {
            (
                SqlDeclaredEntity::Type(identifier_name),
                &InventorySqlDeclaredEntity::Type {
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
                SqlDeclaredEntity::Enum(identifier_name),
                &InventorySqlDeclaredEntity::Enum {
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
                SqlDeclaredEntity::Function(identifier_name),
                &InventorySqlDeclaredEntity::Function {
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
