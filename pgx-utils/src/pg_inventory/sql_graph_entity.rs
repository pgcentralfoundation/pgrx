use super::{
    ControlFile, DotIdentifier, InventoryExtensionSql, InventoryPgExtern, InventoryPostgresEnum,
    InventoryPostgresHash, InventoryPostgresOrd, InventoryPostgresType, InventorySchema, ToSql,
};

/// An entity corresponding to some SQL required by the extension.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum SqlGraphEntity<'a> {
    ExtensionRoot(&'a ControlFile),
    Schema(&'a InventorySchema),
    CustomSql(&'a InventoryExtensionSql),
    Function(&'a InventoryPgExtern),
    Type(&'a InventoryPostgresType),
    BuiltinType(&'a str),
    Enum(&'a InventoryPostgresEnum),
    Ord(&'a InventoryPostgresOrd),
    Hash(&'a InventoryPostgresHash),
}

impl<'a> SqlGraphEntity<'a> {}

impl<'a> DotIdentifier for SqlGraphEntity<'a> {
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
            SqlGraphEntity::ExtensionRoot(item) => item.dot_identifier(),
        }
    }
}

impl<'a> ToSql for SqlGraphEntity<'a> {
    fn to_sql(&self, context: &super::PgxSql) -> eyre::Result<String> {
        match self {
            SqlGraphEntity::Schema(item) => if item.name != "public" && item.name != "pg_catalog" {
                item.to_sql(context)
            } else { Ok(String::default()) },
            SqlGraphEntity::CustomSql(item) => {
                item.to_sql(context)
            },
            SqlGraphEntity::Function(item) => if context.graph.neighbors_undirected(context.externs.get(item).unwrap().clone()).any(|neighbor| {
                let neighbor_item = &context.graph[neighbor];
                match neighbor_item {
                    SqlGraphEntity::Type(InventoryPostgresType { in_fn, in_fn_module_path, out_fn, out_fn_module_path, .. }) => {
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
            } else { item.to_sql(context) },
            SqlGraphEntity::Type(item) => item.to_sql(context),
            SqlGraphEntity::BuiltinType(_) => Ok(String::default()),
            SqlGraphEntity::Enum(item) => item.to_sql(context),
            SqlGraphEntity::Ord(item) => item.to_sql(context),
            SqlGraphEntity::Hash(item) => item.to_sql(context),
            SqlGraphEntity::ExtensionRoot(item) => item.to_sql(context),
        }
    }
}
