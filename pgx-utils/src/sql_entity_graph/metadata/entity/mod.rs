mod r#type;
pub use r#type::FunctionMetadataTypeEntity;

#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct FunctionMetadataEntity {
    pub arguments: Vec<FunctionMetadataTypeEntity>,
    pub retval: Option<FunctionMetadataTypeEntity>,
    pub path: &'static str,
}
