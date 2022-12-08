/*!

Return value specific metadata for Rust to SQL translation

> Like all of the [`sql_entity_graph`][crate::pgx_sql_entity_graph] APIs, this is considered **internal**
to the `pgx` framework and very subject to change between versions. While you may use this, please do it with caution.

*/
use std::error::Error;

use super::sql_translatable::SqlMapping;

/// Describes the RETURNS of CREATE FUNCTION ... RETURNS ...
/// See the PostgreSQL documentation for [CREATE FUNCTION]
/// [CREATE FUNCTION]: https://www.postgresql.org/docs/current/sql-createfunction.html
#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Returns {
    One(SqlMapping),
    SetOf(SqlMapping),
    Table(Vec<SqlMapping>),
}

#[derive(Clone, Copy, Debug, Hash, Ord, PartialOrd, PartialEq, Eq)]
pub enum ReturnsError {
    NestedSetOf,
    NestedTable,
    SetOfContainingTable,
    TableContainingSetOf,
    SetOfInArray,
    TableInArray,
    BareU8,
    SkipInArray,
    Datum,
}

impl std::fmt::Display for ReturnsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReturnsError::NestedSetOf => {
                write!(f, "Nested SetOfIterator in return type")
            }
            ReturnsError::NestedTable => {
                write!(f, "Nested TableIterator in return type")
            }
            ReturnsError::SetOfContainingTable => {
                write!(f, "SetOfIterator containing TableIterator in return type")
            }
            ReturnsError::TableContainingSetOf => {
                write!(f, "TableIterator containing SetOfIterator in return type")
            }
            ReturnsError::SetOfInArray => {
                write!(f, "SetofIterator inside Array is not valid")
            }
            ReturnsError::TableInArray => {
                write!(f, "TableIterator inside Array is not valid")
            }
            ReturnsError::SkipInArray => {
                write!(f, "SqlMapping::Skip inside Array is not valid")
            }
            ReturnsError::BareU8 => {
                write!(f, "Cannot use bare u8")
            }
            ReturnsError::Datum => {
                write!(
                    f,
                    "A Datum as a return means that `sql = \"...\"` must be set in the declaration"
                )
            }
        }
    }
}

impl Error for ReturnsError {}
