/*!

A trait denoting a type can possibly be mapped to an SQL type

> Like all of the [`sql_entity_graph`][crate::sql_entity_graph] APIs, this is considered **internal**
to the `pgx` framework and very subject to change between versions. While you may use this, please do it with caution.

*/
use std::error::Error;

use super::{return_variant::ReturnVariantError, FunctionMetadataTypeEntity, ReturnVariant};

#[derive(Clone, Copy, Debug, Hash, Ord, PartialOrd, PartialEq, Eq)]
pub enum ArgumentError {
    SetOf,
    Table,
    BareU8,
    SkipInArray,
    Datum,
}

impl std::fmt::Display for ArgumentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgumentError::SetOf => {
                write!(f, "Cannot use SetOfIterator as an argument")
            }
            ArgumentError::Table => {
                write!(f, "Canot use TableIterator as an argument")
            }
            ArgumentError::BareU8 => {
                write!(f, "Canot use bare u8")
            }
            ArgumentError::SkipInArray => {
                write!(f, "A SqlMapping::Skip inside Array is not valid")
            }
            ArgumentError::Datum => {
                write!(f, "A Datum as an argument means that `sql = \"...\"` must be set in the declaration")
            }
        }
    }
}

/// Describes ways that Rust types are mapped into SQL
#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum SqlMapping {
    /// Explicit mappings provided by PGX
    As(String),
    Composite {
        array_brackets: bool,
    },
    /// Some types are still directly from source
    Source {
        array_brackets: bool,
    },
    /// Placeholder for some types with no simple translation
    Skip,
}

impl SqlMapping {
    pub fn literal(s: &'static str) -> SqlMapping {
        SqlMapping::As(String::from(s))
    }
}

impl Error for ArgumentError {}

/**
A value which can be represented in SQL
 */
pub trait SqlTranslatable {
    fn type_name() -> &'static str {
        core::any::type_name::<Self>()
    }
    fn argument_sql() -> Result<SqlMapping, ArgumentError>;
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError>;
    fn variadic() -> bool {
        false
    }
    fn optional() -> bool {
        false
    }
    fn entity() -> FunctionMetadataTypeEntity {
        FunctionMetadataTypeEntity {
            type_name: Self::type_name(),
            argument_sql: Self::argument_sql(),
            return_sql: Self::return_sql(),
            variadic: Self::variadic(),
            optional: Self::optional(),
        }
    }
}

impl<T> SqlTranslatable for Option<T>
where
    T: SqlTranslatable,
{
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        T::argument_sql()
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        T::return_sql()
    }
    fn optional() -> bool {
        true
    }
}

impl<T> SqlTranslatable for *mut T
where
    T: SqlTranslatable,
{
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        T::argument_sql()
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        T::return_sql()
    }
    fn optional() -> bool {
        T::optional()
    }
}

impl<T, E> SqlTranslatable for Result<T, E>
where
    T: SqlTranslatable,
    E: std::error::Error + 'static,
{
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        T::argument_sql()
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        T::return_sql()
    }
    fn optional() -> bool {
        true
    }
}

impl<T> SqlTranslatable for Vec<T>
where
    T: SqlTranslatable,
{
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        match T::type_name() {
            id if id == u8::type_name() => Ok(SqlMapping::As(format!("bytea"))),
            _ => match T::argument_sql() {
                Ok(SqlMapping::As(val)) => Ok(SqlMapping::As(format!("{val}[]"))),
                Ok(SqlMapping::Composite { array_brackets: _ }) => Ok(SqlMapping::Composite {
                    array_brackets: true,
                }),
                Ok(SqlMapping::Source { array_brackets: _ }) => Ok(SqlMapping::Source {
                    array_brackets: true,
                }),
                Ok(SqlMapping::Skip) => Ok(SqlMapping::Skip),
                err @ Err(_) => err,
            },
        }
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        match T::type_name() {
            id if id == u8::type_name() => {
                Ok(ReturnVariant::Plain(SqlMapping::As(format!("bytea"))))
            }
            _ => match T::return_sql() {
                Ok(ReturnVariant::Plain(SqlMapping::As(val))) => {
                    Ok(ReturnVariant::Plain(SqlMapping::As(format!("{val}[]"))))
                }
                Ok(ReturnVariant::Plain(SqlMapping::Composite { array_brackets: _ })) => {
                    Ok(ReturnVariant::Plain(SqlMapping::Composite {
                        array_brackets: true,
                    }))
                }
                Ok(ReturnVariant::Plain(SqlMapping::Source { array_brackets: _ })) => {
                    Ok(ReturnVariant::Plain(SqlMapping::Source {
                        array_brackets: true,
                    }))
                }
                Ok(ReturnVariant::Plain(SqlMapping::Skip)) => {
                    Ok(ReturnVariant::Plain(SqlMapping::Skip))
                }
                Ok(ReturnVariant::SetOf(_)) => Err(ReturnVariantError::SetOfInArray),
                Ok(ReturnVariant::Table(_)) => Err(ReturnVariantError::TableInArray),
                err @ Err(_) => err,
            },
        }
    }
    fn optional() -> bool {
        T::optional()
    }
}

impl SqlTranslatable for u8 {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Err(ArgumentError::BareU8)
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Err(ReturnVariantError::BareU8)
    }
}

impl SqlTranslatable for i32 {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("INT"))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlMapping::literal("INT")))
    }
}

impl SqlTranslatable for u32 {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::Source {
            array_brackets: false,
        })
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlMapping::Source {
            array_brackets: false,
        }))
    }
}

impl SqlTranslatable for String {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("TEXT"))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlMapping::literal("TEXT")))
    }
}

impl<T> SqlTranslatable for &T
where
    T: SqlTranslatable,
{
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        T::argument_sql()
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        T::return_sql()
    }
}

impl<'a> SqlTranslatable for &'a str {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("TEXT"))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlMapping::literal("TEXT")))
    }
}

impl<'a> SqlTranslatable for &'a [u8] {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("bytea"))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlMapping::literal("bytea")))
    }
}

impl SqlTranslatable for i8 {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::As(String::from("\"char\"")))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlMapping::As(String::from(
            "\"char\"",
        ))))
    }
}

impl SqlTranslatable for i16 {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("smallint"))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlMapping::literal("smallint")))
    }
}

impl SqlTranslatable for i64 {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("bigint"))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlMapping::literal("bigint")))
    }
}

impl SqlTranslatable for bool {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("bool"))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlMapping::literal("bool")))
    }
}

impl SqlTranslatable for char {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("varchar"))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlMapping::literal("varchar")))
    }
}

impl SqlTranslatable for f32 {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("real"))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlMapping::literal("real")))
    }
}

impl SqlTranslatable for f64 {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("double precision"))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlMapping::literal(
            "double precision",
        )))
    }
}

impl SqlTranslatable for std::ffi::CStr {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("cstring"))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlMapping::literal("cstring")))
    }
}

impl SqlTranslatable for &'static std::ffi::CStr {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("cstring"))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlMapping::literal("cstring")))
    }
}

impl SqlTranslatable for &'static cstr_core::CStr {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("cstring"))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlMapping::literal("cstring")))
    }
}

impl SqlTranslatable for cstr_core::CStr {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("cstring"))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlMapping::literal("cstring")))
    }
}
