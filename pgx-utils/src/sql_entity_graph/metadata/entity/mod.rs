mod r#type;
pub use r#type::FunctionMetadataTypeEntity;

#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct FunctionMetadataEntity {
    pub arguments: Vec<FunctionMetadataTypeEntity>,
    pub retval: Option<FunctionMetadataTypeEntity>,
    pub path: &'static str,
}

impl FunctionMetadataEntity {
    pub fn function_name(&self) -> &'static str {
        self.path
            .split("::")
            .last()
            .expect("Expected path to contain at least one item")
    }
    pub fn module_path(&self) -> &'static str {
        if let Some(end) = self.path.rfind("::") {
            &self.path[..end]
        } else {
            self.path
        }
    }
}
