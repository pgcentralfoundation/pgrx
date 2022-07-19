use core::{any::TypeId, marker::PhantomData};

use super::{FunctionMetadataTypeEntity, ReturnVariant, SqlTranslatable};

pub trait PhantomDataExt {
    fn type_id(&self) -> TypeId;
    fn type_name(&self) -> &'static str;
    fn sql_type(&self) -> String;
    fn return_variant(&self) -> ReturnVariant;
    fn variadic(&self) -> bool;
    fn optional(&self) -> bool;
    fn entity(&self) -> FunctionMetadataTypeEntity;
}

impl<T> PhantomDataExt for PhantomData<T>
where
    T: SqlTranslatable + 'static,
{
    fn type_id(&self) -> TypeId {
        T::type_id()
    }
    fn type_name(&self) -> &'static str {
        T::type_name()
    }
    fn sql_type(&self) -> String {
        T::sql_type()
    }
    fn return_variant(&self) -> ReturnVariant {
        T::return_variant()
    }
    fn variadic(&self) -> bool {
        T::variadic()
    }
    fn optional(&self) -> bool {
        T::optional()
    }
    fn entity(&self) -> FunctionMetadataTypeEntity {
        FunctionMetadataTypeEntity {
            type_id: self.type_id(),
            type_name: self.type_name(),
            sql_type: self.sql_type(),
            return_variant: self.return_variant(),
            variadic: self.variadic(),
            optional: self.optional(),
        }
    }
}
