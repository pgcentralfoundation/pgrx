use core::any::TypeId;

use crate::sql_entity_graph::metadata::ReturnVariant;

#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct FunctionMetadataTypeEntity {
    pub type_id: TypeId,
    pub type_name: &'static str,
    pub sql_type: String,
    pub return_variant: ReturnVariant,
    pub variadic: bool,
    pub optional: bool,
}
