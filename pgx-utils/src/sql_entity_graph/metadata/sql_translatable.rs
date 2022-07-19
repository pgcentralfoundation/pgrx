use core::{any::TypeId, marker::PhantomData};

use super::ReturnVariant;

pub trait SqlTranslatable: 'static {
    fn type_id() -> TypeId {
        TypeId::of::<Self>()
    }
    fn type_name() -> &'static str {
        core::any::type_name::<Self>()
    }
    fn sql_type() -> String;
    fn return_variant() -> ReturnVariant {
        ReturnVariant::Plain
    }
    fn variadic() -> bool {
        false
    }
    fn optional() -> bool {
        false
    }
}

impl<T> SqlTranslatable for Option<T>
where
    T: SqlTranslatable,
{
    fn sql_type() -> String {
        T::sql_type()
    }
    fn optional() -> bool {
        true
    }
}

impl<T, E> SqlTranslatable for Result<T, E>
where
    T: SqlTranslatable,
    E: std::error::Error + 'static,
{
    fn sql_type() -> String {
        T::sql_type()
    }
}

impl<T> SqlTranslatable for Vec<T>
where
    T: SqlTranslatable,
{
    fn sql_type() -> String {
        format!("{}[]", T::sql_type())
    }
}

impl SqlTranslatable for i32 {
    fn sql_type() -> String {
        String::from("INT")
    }
}

impl SqlTranslatable for String {
    fn sql_type() -> String {
        String::from("TEXT")
    }
}

impl SqlTranslatable for &'static str {
    fn sql_type() -> String {
        String::from("TEXT")
    }
}

impl SqlTranslatable for &'static [u8] {
    fn sql_type() -> String {
        String::from("bytea")
    }
}

impl SqlTranslatable for i8 {
    fn sql_type() -> String {
        String::from("char")
    }
}

impl SqlTranslatable for i16 {
    fn sql_type() -> String {
        String::from("smallint")
    }
}

impl SqlTranslatable for i64 {
    fn sql_type() -> String {
        String::from("bigint")
    }
}

impl SqlTranslatable for bool {
    fn sql_type() -> String {
        String::from("bool")
    }
}

impl SqlTranslatable for char {
    fn sql_type() -> String {
        String::from("varchar")
    }
}

impl SqlTranslatable for f32 {
    fn sql_type() -> String {
        String::from("real")
    }
}

impl SqlTranslatable for f64 {
    fn sql_type() -> String {
        String::from("double precision")
    }
}

impl SqlTranslatable for std::ffi::CStr {
    fn sql_type() -> String {
        String::from("cstring")
    }
}
