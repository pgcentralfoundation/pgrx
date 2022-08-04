use std::error::Error;

use super::sql_translatable::SqlVariant;

#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum ReturnVariant {
    Plain(SqlVariant),
    SetOf(SqlVariant),
    Table(Vec<SqlVariant>),
}

#[derive(Clone, Copy, Debug, Hash, Ord, PartialOrd, PartialEq, Eq)]
pub enum ReturnVariantError {
    NestedSetOf,
    NestedTable,
    SetOfContainingTable,
    TableContainingSetOf,
    SetOfInArray,
    TableInArray,
    BareU8,
    SkipInArray,
}

impl std::fmt::Display for ReturnVariantError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReturnVariantError::NestedSetOf => {
                write!(f, "Nested SetofIterator in return type")
            }
            ReturnVariantError::NestedTable => {
                write!(f, "Nested TableIterator in return type")
            }
            ReturnVariantError::SetOfContainingTable => {
                write!(f, "SetofIterator containing TableIterator in return type")
            }
            ReturnVariantError::TableContainingSetOf => {
                write!(f, "TableIterator containing SetofIterator in return type")
            }
            ReturnVariantError::SetOfInArray => {
                write!(f, "SetofIterator inside Array is not valid")
            }
            ReturnVariantError::TableInArray => {
                write!(f, "TableIterator inside Array is not valid")
            }
            ReturnVariantError::SkipInArray => {
                write!(f, "A SqlVariant::Skip inside Array is not valid")
            }
            ReturnVariantError::BareU8 => {
                write!(f, "Canot use bare u8")
            }
        }
    }
}

impl Error for ReturnVariantError {}
