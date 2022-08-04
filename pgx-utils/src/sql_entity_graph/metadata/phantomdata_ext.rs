use core::marker::PhantomData;

use super::{
    return_variant::ReturnVariantError, ArgumentError, FunctionMetadataTypeEntity, ReturnVariant,
    SqlTranslatable, SqlVariant,
};

pub trait PhantomDataExt {
    fn type_name(&self) -> &'static str;
    fn argument_sql(&self) -> Result<SqlVariant, ArgumentError>;
    fn return_sql(&self) -> Result<ReturnVariant, ReturnVariantError>;
    fn variadic(&self) -> bool;
    fn optional(&self) -> bool;
    fn entity(&self) -> FunctionMetadataTypeEntity;
}

impl<T> PhantomDataExt for PhantomData<T>
where
    T: SqlTranslatable,
{
    fn type_name(&self) -> &'static str {
        T::type_name()
    }
    fn argument_sql(&self) -> Result<SqlVariant, ArgumentError> {
        T::argument_sql()
    }
    fn return_sql(&self) -> Result<ReturnVariant, ReturnVariantError> {
        T::return_sql()
    }
    fn variadic(&self) -> bool {
        T::variadic()
    }
    fn optional(&self) -> bool {
        T::optional()
    }
    fn entity(&self) -> FunctionMetadataTypeEntity {
        T::entity()
    }
}
