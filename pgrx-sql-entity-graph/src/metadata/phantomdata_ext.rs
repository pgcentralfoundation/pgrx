/*!

Zero sized type marker metadata for Rust to SQL translation

> Like all of the [`sql_entity_graph`][crate::pgrx_sql_entity_graph] APIs, this is considered **internal**
to the `pgrx` framework and very subject to change between versions. While you may use this, please do it with caution.

*/
use core::marker::PhantomData;

use super::return_variant::ReturnsError;
use super::{ArgumentError, FunctionMetadataTypeEntity, Returns, SqlMapping, SqlTranslatable};

/**
An extension trait for [`PhantomData`][core::marker::PhantomData] offering SQL generation related info

Since we don't actually want to construct values during SQL generation, we use a [`PhantomData`][core::marker::PhantomData].
 */
pub trait PhantomDataExt {
    fn type_name(&self) -> &'static str;
    fn argument_sql(&self) -> Result<SqlMapping, ArgumentError>;
    fn return_sql(&self) -> Result<Returns, ReturnsError>;
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
    fn argument_sql(&self) -> Result<SqlMapping, ArgumentError> {
        T::argument_sql()
    }
    fn return_sql(&self) -> Result<Returns, ReturnsError> {
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
