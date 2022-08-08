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
                write!(f, "A SqlVariant::Skip inside Array is not valid")
            }
        }
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum SqlVariant {
    Mapped(String),
    Composite { requires_array_brackets: bool },
    SourceOnly { requires_array_brackets: bool },
    Skip,
}

impl Error for ArgumentError {}

/**
A value which can be represented in SQL
 */
pub trait SqlTranslatable {
    fn type_name() -> &'static str {
        core::any::type_name::<Self>()
    }
    fn argument_sql() -> Result<SqlVariant, ArgumentError>;
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
    fn argument_sql() -> Result<SqlVariant, ArgumentError> {
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
    fn argument_sql() -> Result<SqlVariant, ArgumentError> {
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
    fn argument_sql() -> Result<SqlVariant, ArgumentError> {
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
    fn argument_sql() -> Result<SqlVariant, ArgumentError> {
        match T::type_name() {
            id if id == u8::type_name() => Ok(SqlVariant::Mapped(format!("bytea"))),
            _ => match T::argument_sql() {
                Ok(SqlVariant::Mapped(val)) => Ok(SqlVariant::Mapped(format!("{val}[]"))),
                Ok(SqlVariant::Composite {
                    requires_array_brackets: _,
                }) => Ok(SqlVariant::Composite {
                    requires_array_brackets: true,
                }),
                Ok(SqlVariant::SourceOnly {
                    requires_array_brackets: _,
                }) => Ok(SqlVariant::SourceOnly {
                    requires_array_brackets: true,
                }),
                Ok(SqlVariant::Skip) => Ok(SqlVariant::Skip),
                err @ Err(_) => err,
            },
        }
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        match T::type_name() {
            id if id == u8::type_name() => {
                Ok(ReturnVariant::Plain(SqlVariant::Mapped(format!("bytea"))))
            }
            _ => match T::return_sql() {
                Ok(ReturnVariant::Plain(SqlVariant::Mapped(val))) => {
                    Ok(ReturnVariant::Plain(SqlVariant::Mapped(format!("{val}[]"))))
                }
                Ok(ReturnVariant::Plain(SqlVariant::Composite {
                    requires_array_brackets: _,
                })) => Ok(ReturnVariant::Plain(SqlVariant::Composite {
                    requires_array_brackets: true,
                })),
                Ok(ReturnVariant::Plain(SqlVariant::SourceOnly {
                    requires_array_brackets: _,
                })) => Ok(ReturnVariant::Plain(SqlVariant::SourceOnly {
                    requires_array_brackets: true,
                })),
                Ok(ReturnVariant::Plain(SqlVariant::Skip)) => {
                    Ok(ReturnVariant::Plain(SqlVariant::Skip))
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
    fn argument_sql() -> Result<SqlVariant, ArgumentError> {
        Err(ArgumentError::BareU8)
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Err(ReturnVariantError::BareU8)
    }
}

impl SqlTranslatable for i32 {
    fn argument_sql() -> Result<SqlVariant, ArgumentError> {
        Ok(SqlVariant::Mapped(String::from("INT")))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlVariant::Mapped(String::from(
            "INT",
        ))))
    }
}

impl SqlTranslatable for u32 {
    fn argument_sql() -> Result<SqlVariant, ArgumentError> {
        Ok(SqlVariant::SourceOnly {
            requires_array_brackets: false,
        })
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlVariant::SourceOnly {
            requires_array_brackets: false,
        }))
    }
}

impl SqlTranslatable for String {
    fn argument_sql() -> Result<SqlVariant, ArgumentError> {
        Ok(SqlVariant::Mapped(String::from("TEXT")))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlVariant::Mapped(String::from(
            "TEXT",
        ))))
    }
}

impl<'a> SqlTranslatable for &'a str {
    fn argument_sql() -> Result<SqlVariant, ArgumentError> {
        Ok(SqlVariant::Mapped(String::from("TEXT")))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlVariant::Mapped(String::from(
            "TEXT",
        ))))
    }
}

impl<'a> SqlTranslatable for &'a [u8] {
    fn argument_sql() -> Result<SqlVariant, ArgumentError> {
        Ok(SqlVariant::Mapped(String::from("bytea")))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlVariant::Mapped(String::from(
            "bytea",
        ))))
    }
}

impl SqlTranslatable for i8 {
    fn argument_sql() -> Result<SqlVariant, ArgumentError> {
        Ok(SqlVariant::Mapped(String::from("\"char\"")))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlVariant::Mapped(String::from(
            "\"char\"",
        ))))
    }
}

impl SqlTranslatable for i16 {
    fn argument_sql() -> Result<SqlVariant, ArgumentError> {
        Ok(SqlVariant::Mapped(String::from("smallint")))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlVariant::Mapped(String::from(
            "smallint",
        ))))
    }
}

impl SqlTranslatable for i64 {
    fn argument_sql() -> Result<SqlVariant, ArgumentError> {
        Ok(SqlVariant::Mapped(String::from("bigint")))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlVariant::Mapped(String::from(
            "bigint",
        ))))
    }
}

impl SqlTranslatable for bool {
    fn argument_sql() -> Result<SqlVariant, ArgumentError> {
        Ok(SqlVariant::Mapped(String::from("bool")))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlVariant::Mapped(String::from(
            "bool",
        ))))
    }
}

impl SqlTranslatable for char {
    fn argument_sql() -> Result<SqlVariant, ArgumentError> {
        Ok(SqlVariant::Mapped(String::from("varchar")))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlVariant::Mapped(String::from(
            "varchar",
        ))))
    }
}

impl SqlTranslatable for f32 {
    fn argument_sql() -> Result<SqlVariant, ArgumentError> {
        Ok(SqlVariant::Mapped(String::from("real")))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlVariant::Mapped(String::from(
            "real",
        ))))
    }
}

impl SqlTranslatable for f64 {
    fn argument_sql() -> Result<SqlVariant, ArgumentError> {
        Ok(SqlVariant::Mapped(String::from("double precision")))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlVariant::Mapped(String::from(
            "double precision",
        ))))
    }
}

impl SqlTranslatable for std::ffi::CStr {
    fn argument_sql() -> Result<SqlVariant, ArgumentError> {
        Ok(SqlVariant::Mapped(String::from("cstring")))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlVariant::Mapped(String::from(
            "cstring",
        ))))
    }
}

impl SqlTranslatable for &'static std::ffi::CStr {
    fn argument_sql() -> Result<SqlVariant, ArgumentError> {
        Ok(SqlVariant::Mapped(String::from("cstring")))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlVariant::Mapped(String::from(
            "cstring",
        ))))
    }
}

impl SqlTranslatable for &'static cstr_core::CStr {
    fn argument_sql() -> Result<SqlVariant, ArgumentError> {
        Ok(SqlVariant::Mapped(String::from("cstring")))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlVariant::Mapped(String::from(
            "cstring",
        ))))
    }
}

impl SqlTranslatable for cstr_core::CStr {
    fn argument_sql() -> Result<SqlVariant, ArgumentError> {
        Ok(SqlVariant::Mapped(String::from("cstring")))
    }
    fn return_sql() -> Result<ReturnVariant, ReturnVariantError> {
        Ok(ReturnVariant::Plain(SqlVariant::Mapped(String::from(
            "cstring",
        ))))
    }
}
