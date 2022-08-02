use crate::sql_entity_graph::metadata::{
    return_variant::ReturnVariantError, ArgumentError, ReturnVariant, SqlVariant,
};

#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct FunctionMetadataTypeEntity {
    pub type_name: &'static str,
    pub argument_sql: Result<SqlVariant, ArgumentError>,
    pub return_sql: Result<ReturnVariant, ReturnVariantError>,
    pub variadic: bool,
    pub optional: bool,
}
