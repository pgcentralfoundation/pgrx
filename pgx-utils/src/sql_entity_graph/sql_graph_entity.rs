use super::{
    aggregate::PgAggregateEntity, ControlFile, ExtensionSqlEntity, PgExternEntity,
    PostgresEnumEntity, PostgresHashEntity, PostgresOrdEntity, PostgresTypeEntity, SchemaEntity,
    SqlGraphIdentifier, ToSql,
};

/// An entity corresponding to some SQL required by the extension.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum SqlGraphEntity {
    ExtensionRoot(ControlFile),
    Schema(SchemaEntity),
    CustomSql(ExtensionSqlEntity),
    Function(PgExternEntity),
    Type(PostgresTypeEntity),
    BuiltinType(String),
    Enum(PostgresEnumEntity),
    Ord(PostgresOrdEntity),
    Hash(PostgresHashEntity),
    Aggregate(PgAggregateEntity),
}

impl SqlGraphEntity {
    pub fn sql_anchor_comment(&self) -> String {
        let maybe_file_and_line = if let (Some(file), Some(line)) = (self.file(), self.line()) {
            format!("-- {file}:{line}\n", file = file, line = line)
        } else {
            String::default()
        };
        format!("\
            {maybe_file_and_line}\
            -- {rust_identifier}\
        ",
            maybe_file_and_line = maybe_file_and_line,
            rust_identifier = self.rust_identifier(),
        )
    }
}

impl SqlGraphIdentifier for SqlGraphEntity {
    fn dot_identifier(&self) -> String {
        match self {
            SqlGraphEntity::Schema(item) => item.dot_identifier(),
            SqlGraphEntity::CustomSql(item) => item.dot_identifier(),
            SqlGraphEntity::Function(item) => item.dot_identifier(),
            SqlGraphEntity::Type(item) => item.dot_identifier(),
            SqlGraphEntity::BuiltinType(item) => format!("preexisting type {}", item),
            SqlGraphEntity::Enum(item) => item.dot_identifier(),
            SqlGraphEntity::Ord(item) => item.dot_identifier(),
            SqlGraphEntity::Hash(item) => item.dot_identifier(),
            SqlGraphEntity::Aggregate(item) => item.dot_identifier(),
            SqlGraphEntity::ExtensionRoot(item) => item.dot_identifier(),
        }
    }

    fn rust_identifier(&self) -> String {
        match self {
            SqlGraphEntity::Schema(item) => item.rust_identifier(),
            SqlGraphEntity::CustomSql(item) => item.rust_identifier(),
            SqlGraphEntity::Function(item) => item.rust_identifier(),
            SqlGraphEntity::Type(item) => item.rust_identifier(),
            SqlGraphEntity::BuiltinType(item) => item.to_string(),
            SqlGraphEntity::Enum(item) => item.rust_identifier(),
            SqlGraphEntity::Ord(item) => item.rust_identifier(),
            SqlGraphEntity::Hash(item) => item.rust_identifier(),
            SqlGraphEntity::Aggregate(item) => item.rust_identifier(),
            SqlGraphEntity::ExtensionRoot(item) => item.rust_identifier(),
        }
    }

    fn file(&self) -> Option<&'static str> {
        match self {
            SqlGraphEntity::Schema(item) => item.file(),
            SqlGraphEntity::CustomSql(item) => item.file(),
            SqlGraphEntity::Function(item) => item.file(),
            SqlGraphEntity::Type(item) => item.file(),
            SqlGraphEntity::BuiltinType(_item) => None,
            SqlGraphEntity::Enum(item) => item.file(),
            SqlGraphEntity::Ord(item) => item.file(),
            SqlGraphEntity::Hash(item) => item.file(),
            SqlGraphEntity::Aggregate(item) => item.file(),
            SqlGraphEntity::ExtensionRoot(item) => item.file(),
        }
    }

    fn line(&self) -> Option<u32> {
        match self {
            SqlGraphEntity::Schema(item) => item.line(),
            SqlGraphEntity::CustomSql(item) => item.line(),
            SqlGraphEntity::Function(item) => item.line(),
            SqlGraphEntity::Type(item) => item.line(),
            SqlGraphEntity::BuiltinType(_item) => None,
            SqlGraphEntity::Enum(item) => item.line(),
            SqlGraphEntity::Ord(item) => item.line(),
            SqlGraphEntity::Hash(item) => item.line(),
            SqlGraphEntity::Aggregate(item) => item.line(),
            SqlGraphEntity::ExtensionRoot(item) => item.line(),
        }
    }
}

impl ToSql for SqlGraphEntity {
    #[tracing::instrument(level = "debug", skip(self, context), fields(identifier = %self.rust_identifier()))]
    fn to_sql(&self, context: &super::PgxSql) -> eyre::Result<String> {
        match self {
            SqlGraphEntity::Schema(item) => {
                if item.name != "public" && item.name != "pg_catalog" {
                    item.to_sql(context)
                } else {
                    Ok(String::default())
                }
            }
            SqlGraphEntity::CustomSql(item) => item.to_sql(context),
            SqlGraphEntity::Function(item) => {
                if let Some(result) = item.to_sql_config.to_sql(self, context) {
                    return result;
                }
                if context.graph.neighbors_undirected(context.externs.get(item).unwrap().clone()).any(|neighbor| {
                    let neighbor_item = &context.graph[neighbor];
                    match neighbor_item {
                        SqlGraphEntity::Type(PostgresTypeEntity { in_fn, in_fn_module_path, out_fn, out_fn_module_path, .. }) => {
                            let is_in_fn = item.full_path.starts_with(in_fn_module_path) && item.full_path.ends_with(in_fn);
                            if is_in_fn {
                                tracing::trace!(r#type = %neighbor_item.dot_identifier(), "Skipping, is an in_fn.");
                            }
                            let is_out_fn = item.full_path.starts_with(out_fn_module_path) && item.full_path.ends_with(out_fn);
                            if is_out_fn {
                                tracing::trace!(r#type = %neighbor_item.dot_identifier(), "Skipping, is an out_fn.");
                            }
                            is_in_fn || is_out_fn
                        },
                        _ => false,
                    }
                }) {
                    Ok(String::default())
                } else {
                    item.to_sql(context)
                }
            }
            SqlGraphEntity::Type(item) => item
                .to_sql_config
                .to_sql(self, context)
                .unwrap_or_else(|| item.to_sql(context)),
            SqlGraphEntity::BuiltinType(_) => Ok(String::default()),
            SqlGraphEntity::Enum(item) => item
                .to_sql_config
                .to_sql(self, context)
                .unwrap_or_else(|| item.to_sql(context)),
            SqlGraphEntity::Ord(item) => item
                .to_sql_config
                .to_sql(self, context)
                .unwrap_or_else(|| item.to_sql(context)),
            SqlGraphEntity::Hash(item) => item
                .to_sql_config
                .to_sql(self, context)
                .unwrap_or_else(|| item.to_sql(context)),
            SqlGraphEntity::Aggregate(item) => item
                .to_sql_config
                .to_sql(self, context)
                .unwrap_or_else(|| item.to_sql(context)),
            SqlGraphEntity::ExtensionRoot(item) => item.to_sql(context),
        }
    }
}
