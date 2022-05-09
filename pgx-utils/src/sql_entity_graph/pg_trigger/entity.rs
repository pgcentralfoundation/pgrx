use crate::sql_entity_graph::{SqlGraphEntity, ToSql, PgxSql, SqlGraphIdentifier, ToSqlConfigEntity};
use core::{cmp::{Eq, PartialEq, Ord, PartialOrd, Ordering}, hash::Hash, fmt::Debug};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct PgTriggerEntity {
    pub function_name: &'static str,
    pub to_sql_config: ToSqlConfigEntity,
    pub file: &'static str,
    pub line: u32,
    pub module_path: &'static str,
    pub full_path: &'static str,
}

impl PgTriggerEntity {
    fn wrapper_function_name(&self) -> String {
        self.function_name.to_string() + "_wrapper"
    }
}

impl Ord for PgTriggerEntity {
    fn cmp(&self, other: &Self) -> Ordering {
        self.full_path
            .cmp(other.full_path)
    }
}

impl PartialOrd for PgTriggerEntity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Into<SqlGraphEntity> for PgTriggerEntity {
    fn into(self) -> SqlGraphEntity {
        SqlGraphEntity::Trigger(self)
    }
}


impl ToSql for PgTriggerEntity {
    #[tracing::instrument(
        level = "error",
        skip(self, context),
        fields(identifier = %self.rust_identifier()),
    )]
    fn to_sql(&self, context: &PgxSql) -> eyre::Result<String> {
        let self_index = context.triggers[self];
        let schema = context.schema_prefix_for(&self_index);

        let sql = format!("\n\
            -- {file}:{line}\n\
            -- {full_path}\n\
            CREATE FUNCTION {schema}\"{function_name}\"()\n\
                \tRETURNS TRIGGER\n\
                \tLANGUAGE c\n\
                \tAS 'MODULE_PATHNAME', '{wrapper_function_name}';\
        ",
            schema = schema,
            file = self.file,
            line = self.line,
            full_path = self.full_path,
            function_name = self.function_name,
            wrapper_function_name = self.wrapper_function_name(),
        );
        Ok(sql)
    }
}

impl SqlGraphIdentifier for PgTriggerEntity {
    fn dot_identifier(&self) -> String {
        format!("trigger fn {}", self.full_path)
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