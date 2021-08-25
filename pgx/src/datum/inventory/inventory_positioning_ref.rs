
use std::fmt::Display;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum InventoryPositioningRef<'a> {
    FullPath(&'a str),
    Name(&'a str),
}

impl<'a> Display for InventoryPositioningRef<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InventoryPositioningRef::FullPath(i) => f.write_str(i),
            InventoryPositioningRef::Name(i) => f.write_str(i),
        }
    }
}
