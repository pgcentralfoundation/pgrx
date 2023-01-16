use std::{
    marker::PhantomData,
    ptr::{slice_from_raw_parts, NonNull},
};

use pgx_pg_sys::{ArrayType, Datum};
use pgx_sql_entity_graph::metadata::{
    ArgumentError, Returns, ReturnsError, SqlMapping, SqlTranslatable,
};

use crate::{
    array::{RawArray, ARR_DATA_PTR, ARR_DIMS, ARR_LBOUND, ARR_OVERHEAD_NONULLS},
    set_varsize, FromDatum, IntoDatum, PgMemoryContexts,
};

pub unsafe trait IntArrayElemType {}
unsafe impl IntArrayElemType for i16 {}
unsafe impl IntArrayElemType for i32 {}
unsafe impl IntArrayElemType for i64 {}

/// An Array of an non-nullable integer type (int2, int4, int8) optimized for speed.
pub struct IntArray<T = i32>
where
    T: FromDatum + IntArrayElemType,
{
    raw_array: RawArray,
    _marker: PhantomData<T>,
}

impl<T> IntArray<T>
where
    T: IntoDatum + FromDatum + IntArrayElemType,
{
    #[inline]
    pub fn as_array_type(&self) -> *mut ArrayType {
        self.raw_array.as_ptr().as_ptr()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.raw_array.len()
    }

    #[inline]
    pub fn into_slice(&self) -> &[T] {
        unsafe {
            slice_from_raw_parts(
                ARR_DATA_PTR(self.raw_array.as_ptr().as_ptr()).cast(),
                self.raw_array.len(),
            )
            .as_ref()
            .expect("Couldn't ARR_DATA_PTR the *ArrayType")
        }
    }

    #[inline]
    pub fn into_mut_slice(&mut self) -> &mut [T] {
        unsafe { self.raw_array.data::<T>().as_mut() }
    }

    /// Creates an IntArray<T> with zero-elements
    /// Slightly faster than new_array_with_len(0)
    pub fn new_empty_array() -> Self {
        let array_datum = unsafe {
            let array_type = pgx_pg_sys::construct_empty_array(T::type_oid());
            let datum: Datum = array_type.into();
            Self::from_polymorphic_datum(datum, false, T::array_type_oid())
                .expect("couldn't make datum!")
        };
        array_datum
    }

    /// Creates an IntArray<T> of a fixed len, with 0 for all elements
    /// Uses a single PG allocation rather than
    /// Rustified version of new_intArrayType(int num) from https://github.com/postgres/postgres/blob/master/contrib/intarray/_int_tool.c#L219
    pub fn new_array_with_len(len: usize) -> Self {
        if len == 0 {
            return Self::new_empty_array();
        }
        let elem_size = std::mem::size_of::<T>();
        let nbytes: usize = ARR_OVERHEAD_NONULLS(1) + elem_size * len;

        unsafe {
            let array_type = PgMemoryContexts::For(pgx_pg_sys::CurrentMemoryContext).palloc0(nbytes)
                as *mut ArrayType;
            set_varsize(array_type as *mut pgx_pg_sys::varlena, nbytes as i32);
            (*array_type).ndim = 1;
            (*array_type).dataoffset = 0; /* marker for no null bitmap */
            (*array_type).elemtype = T::type_oid();

            let ndims = ARR_DIMS(array_type);
            *ndims = len as i32; // equivalent of ARR_DIMS(r)[0] = num;
            let arr_lbound = ARR_LBOUND(array_type);
            *arr_lbound = 1;

            let datum: Datum = array_type.into();
            Self::from_polymorphic_datum(datum, false, T::array_type_oid())
                .expect("Couldn't convert *ArrayType Datum to IntArray<T>")
        }
    }
}

impl<T> FromDatum for IntArray<T>
where
    T: FromDatum + IntArrayElemType,
{
    unsafe fn from_polymorphic_datum(
        datum: pgx_pg_sys::Datum,
        is_null: bool,
        _: pgx_pg_sys::Oid,
    ) -> Option<Self> {
        if is_null || datum.is_null() {
            None
        } else {
            unsafe {
                let array_type =
                    pgx_pg_sys::pg_detoast_datum(datum.cast_mut_ptr()) as *mut ArrayType;
                let raw_array =
                    RawArray::from_ptr(NonNull::new(array_type).expect("ArrayType was null!"));

                // nullable arrays are not allowed for IntArray<T>
                if raw_array.nullable() {
                    pgx_pg_sys::error!("array must not contain nulls");
                }
                Some(IntArray { raw_array: raw_array, _marker: PhantomData })
            }
        }
    }
}

impl<T> IntoDatum for IntArray<T>
where
    T: IntoDatum + FromDatum + IntArrayElemType,
{
    fn into_datum(self) -> Option<pgx_pg_sys::Datum> {
        Some(self.raw_array.into_ptr().as_ptr().into())
    }

    fn type_oid() -> pgx_pg_sys::Oid {
        T::array_type_oid()
    }
}

unsafe impl<T> SqlTranslatable for IntArray<T>
where
    T: SqlTranslatable + FromDatum + IntArrayElemType,
{
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        match T::argument_sql()? {
            SqlMapping::As(sql) => Ok(SqlMapping::As(format!("{sql}[]"))),
            SqlMapping::Skip => Err(ArgumentError::SkipInArray),
            SqlMapping::Composite { .. } => Ok(SqlMapping::Composite { array_brackets: true }),
            SqlMapping::Source { .. } => Ok(SqlMapping::Source { array_brackets: true }),
        }
    }

    fn return_sql() -> Result<Returns, ReturnsError> {
        match T::return_sql()? {
            Returns::One(SqlMapping::As(sql)) => {
                Ok(Returns::One(SqlMapping::As(format!("{sql}[]"))))
            }
            Returns::One(SqlMapping::Composite { array_brackets: _ }) => {
                Ok(Returns::One(SqlMapping::Composite { array_brackets: true }))
            }
            Returns::One(SqlMapping::Source { array_brackets: _ }) => {
                Ok(Returns::One(SqlMapping::Source { array_brackets: true }))
            }
            Returns::One(SqlMapping::Skip) => Err(ReturnsError::SkipInArray),
            Returns::SetOf(_) => Err(ReturnsError::SetOfInArray),
            Returns::Table(_) => Err(ReturnsError::TableInArray),
        }
    }
}
