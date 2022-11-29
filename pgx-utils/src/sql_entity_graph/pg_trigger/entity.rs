/*!

`#[pg_trigger]` related entities for Rust to SQL translation

> Like all of the [`sql_entity_graph`][crate::sql_entity_graph] APIs, this is considered **internal**
to the `pgx` framework and very subject to change between versions. While you may use this, please do it with caution.

*/
use crate::sql_entity_graph::{
    PgxSql, SqlGraphEntity, SqlGraphIdentifier, ToSql, ToSqlConfigEntity,
};

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
pub struct PgTriggerEntity {
    pub function_name: String,
    pub to_sql_config: ToSqlConfigEntity,
    pub file: String,
    pub line: u32,
    pub module_path: String,
    pub full_path: String,
}

impl PgTriggerEntity {
    fn wrapper_function_name(&self) -> String {
        self.function_name.to_string() + "_wrapper"
    }
}

impl From<PgTriggerEntity> for SqlGraphEntity {
    fn from(val: PgTriggerEntity) -> Self {
        SqlGraphEntity::Trigger(val)
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

        let sql = format!(
            "\n\
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
        self.full_path.clone()
    }

    fn file(&self) -> Option<&str> {
        Some(&self.file)
    }

    fn line(&self) -> Option<u32> {
        Some(self.line)
    }
}
