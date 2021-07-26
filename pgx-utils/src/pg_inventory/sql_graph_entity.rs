use super::{
    ControlFile, DotFormat, InventoryExtensionSql, InventoryPgExtern, InventoryPostgresEnum,
    InventoryPostgresHash, InventoryPostgresOrd, InventoryPostgresType, InventorySchema,
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

impl<'a> SqlGraphEntity<'a> {
    pub fn dot_format(&self) -> String {
        match self {
            SqlGraphEntity::Schema(item) => item.dot_format(),
            SqlGraphEntity::CustomSql(item) => item.dot_format(),
            SqlGraphEntity::Function(item) => item.dot_format(),
            SqlGraphEntity::Type(item) => item.dot_format(),
            SqlGraphEntity::BuiltinType(item) => format!("preexisting type {}", item),
            SqlGraphEntity::Enum(item) => item.dot_format(),
            SqlGraphEntity::Ord(item) => item.dot_format(),
            SqlGraphEntity::Hash(item) => item.dot_format(),
            SqlGraphEntity::ExtensionRoot(item) => item.dot_format(),
        }
    }
}
