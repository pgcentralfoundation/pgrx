/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

//! Utility functions for working with `pg_sys::RangeType` structs
use crate::{
    pg_sys, void_mut_ptr, AnyNumeric, Date, FromDatum, IntoDatum, Numeric, Timestamp,
    TimestampWithTimeZone,
};
use pgx_pg_sys::{Oid, RangeBound};
use pgx_sql_entity_graph::metadata::{
    ArgumentError, Returns, ReturnsError, SqlMapping, SqlTranslatable,
};
use std::marker::PhantomData;

/// Represents Datum to serialized RangeType PG struct
pub struct Range<T: FromDatum + IntoDatum + RangeSubType> {
    ptr: *mut pg_sys::varlena,
    range_type: *mut pg_sys::RangeType,
    _marker: PhantomData<T>,
}

impl<T> Range<T>
where
    T: FromDatum + IntoDatum + RangeSubType,
{
    /// ## Safety
    /// This function is safe, but requires that
    /// - datum is not null
    /// - datum represents a PG RangeType datum
    #[inline]
    unsafe fn from_pg(datum: pg_sys::Datum) -> Option<Self> {
        unsafe { Self::from_polymorphic_datum(datum, false, T::range_type_oid()) }
    }
}
impl<T> TryFrom<pg_sys::Datum> for Range<T>
where
    T: FromDatum + IntoDatum + RangeSubType,
{
    type Error = RangeConversionError;

    fn try_from(datum: pg_sys::Datum) -> Result<Self, Self::Error> {
        if datum.is_null() {
            Err(RangeConversionError::NullDatum)
        } else {
            match unsafe { Self::from_pg(datum) } {
                Some(range) => Ok(range),
                None => Err(RangeConversionError::InvalidDatum),
            }
        }
    }
}

impl<T> FromDatum for Range<T>
where
    T: FromDatum + IntoDatum + RangeSubType,
{
    /// ## Safety
    /// function requires that
    /// - is_null is true OR datum represents a PG RangeType datum
    #[inline]
    unsafe fn from_polymorphic_datum(
        datum: pg_sys::Datum,
        is_null: bool,
        _: pg_sys::Oid,
    ) -> Option<Self>
    where
        Self: Sized,
    {
        if is_null || datum.is_null() {
            None
        } else {
            let ptr: *mut pg_sys::varlena = datum.cast_mut_ptr();
            // Datum should be non-null and point to PG RangeType
            let range_type =
                unsafe { pg_sys::pg_detoast_datum(datum.cast_mut_ptr()) as *mut pg_sys::RangeType };
            Some(Range { ptr, range_type, _marker: PhantomData })
        }
    }
}

impl<T> IntoDatum for Range<T>
where
    T: FromDatum + IntoDatum + RangeSubType,
{
    #[inline]
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(self.range_type.into())
    }

    #[inline]
    fn type_oid() -> pg_sys::Oid {
        T::range_type_oid()
    }
}

impl<T> Drop for Range<T>
where
    T: FromDatum + IntoDatum + RangeSubType,
{
    fn drop(&mut self) {
        // Detoasting the varlena may have allocated: the toasted varlena cloned as a detoasted RangeType
        // Checking for pointer equivalence is the only way we can truly tell
        if !self.range_type.is_null() && self.range_type as *mut pg_sys::varlena != self.ptr {
            unsafe {
                // SAFETY: if pgx detoasted a clone of this varlena, pfree the clone
                pg_sys::pfree(self.range_type as void_mut_ptr);
            }
        }
    }
}

/// Represents a deserialized state of the RangeType's data
/// <T> indicates the subtype of the lower/upper bounds' datum
pub struct RangeData<T> {
    pub lower: RangeBound,
    pub upper: RangeBound,
    pub is_empty: bool,
    __marker: PhantomData<T>,
}

impl<T> RangeData<T>
where
    T: FromDatum + IntoDatum + RangeSubType,
{
    /// The lower bound's datum as Option<T>
    /// Empty ranges or lower infinite bounds will be None
    #[inline]
    pub fn lower_val(&self) -> Option<T> {
        if self.is_empty || self.lower.infinite {
            None
        } else {
            unsafe { T::from_polymorphic_datum(self.lower.val, false, T::type_oid()) }
        }
    }

    /// The upper bound's datum as Option<T>
    /// Empty ranges or upper infinite bounds will be None
    #[inline]
    pub fn upper_val(&self) -> Option<T> {
        if self.is_empty || self.upper.infinite {
            None
        } else {
            unsafe { T::from_polymorphic_datum(self.upper.val, false, T::type_oid()) }
        }
    }

    /// Builds an "empty" range
    pub fn empty_range_data() -> Self {
        let lower_bound = RangeBound { lower: true, ..RangeBound::default() };
        let upper_bound = RangeBound { lower: false, ..RangeBound::default() };
        Self::from_range_bounds_internal(lower_bound, upper_bound, true)
    }

    /// Generate a RangeData<T> from the lower/upper RangeBounds, implies non-empty
    #[inline]
    pub fn from_range_bounds(lower_bound: RangeBound, upper_bound: RangeBound) -> Self {
        Self::from_range_bounds_internal(lower_bound, upper_bound, false)
    }

    pub(crate) fn from_range_bounds_internal(
        lower_bound: RangeBound,
        upper_bound: RangeBound,
        is_empty: bool,
    ) -> Self {
        RangeData { lower: lower_bound, upper: upper_bound, is_empty, __marker: PhantomData }
    }

    /// Generate a RangeData<T> from the T values for lower/upper bounds, lower/upper inclusive
    /// None for lower_val or upper_val will represent lower_inf/upper_inf bounds
    pub fn from_range_values(
        lower_val: Option<T>,
        upper_val: Option<T>,
        lower_inc: bool,
        upper_inc: bool,
    ) -> Self {
        let mut lower_bound =
            RangeBound { lower: true, inclusive: lower_inc, ..Default::default() };

        let mut upper_bound =
            RangeBound { lower: false, inclusive: upper_inc, ..Default::default() };

        match lower_val {
            Some(lower_val) => {
                lower_bound.val =
                    lower_val.into_datum().expect("Couldn't convert lower_val to Datum");
            }
            None => {
                lower_bound.infinite = true;
            }
        }

        match upper_val {
            Some(upper_val) => {
                upper_bound.val =
                    upper_val.into_datum().expect("Couldn't convert upper_val to Datum");
            }
            None => {
                upper_bound.infinite = true;
            }
        }

        RangeData::from_range_bounds(lower_bound, upper_bound)
    }
}

impl<T> From<Range<T>> for RangeData<T>
where
    T: FromDatum + IntoDatum + RangeSubType,
{
    /// ## Safety
    /// Requires that:
    /// - range.range_type is valid pointer to RangeType PG struct
    ///
    /// Only PG will create the range_type, so this should always be valid
    fn from(range: Range<T>) -> Self {
        let mut lower_bound: RangeBound = Default::default();
        let mut upper_bound: RangeBound = Default::default();
        let mut is_empty = false;

        unsafe {
            // range.range_type came from PG, so assume its rangetypid is valid
            let typecache = pg_sys::lookup_type_cache(
                (*(range.range_type)).rangetypid,
                pg_sys::TYPECACHE_RANGE_INFO as i32,
            );

            // PG will deserialize into lower/upper RangeBounds and is_empty
            pg_sys::range_deserialize(
                typecache,
                range.range_type,
                &mut lower_bound,
                &mut upper_bound,
                &mut is_empty,
            );
        }
        // The lower_bound/upper_bound RangeBound value's .val will be a valid Datum of the T type
        // If the range is_empty or either bound is infinite then .val = (Datum) 0
        RangeData::from_range_bounds_internal(lower_bound, upper_bound, is_empty)
    }
}

impl<T> From<RangeData<T>> for Range<T>
where
    T: FromDatum + IntoDatum + RangeSubType,
{
    fn from(range_data: RangeData<T>) -> Self {
        let datum: pg_sys::Datum = unsafe {
            // T must have a valid registered "Range" Type ex. int4 -> int4range,
            let typecache =
                pg_sys::lookup_type_cache(T::range_type_oid(), pg_sys::TYPECACHE_RANGE_INFO as i32);

            let mut lower_bound = range_data.lower;
            let mut upper_bound = range_data.upper;

            // PG will serialize these lower/upper RangeBounds to a *RangeType ptr/datum
            let range_type = pg_sys::make_range(
                typecache,
                &mut lower_bound,
                &mut upper_bound,
                range_data.is_empty,
            );

            // *RangeType into Datum
            range_type.into()
        };

        // SAFETY: We expect PG returned us a valid datum, pointing to *mut pg_sys::RangeType
        unsafe { Range::<T>::from_pg(datum) }.expect("Invalid RangeType Datum")
    }
}

/// This trait allows a struct to be a valid subtype for a RangeType
pub unsafe trait RangeSubType {
    fn range_type_oid() -> Oid;
}

/// for int/int4range
unsafe impl RangeSubType for i32 {
    fn range_type_oid() -> Oid {
        pg_sys::INT4RANGEOID
    }
}

/// for bigint/int8range
unsafe impl RangeSubType for i64 {
    fn range_type_oid() -> Oid {
        pg_sys::INT8RANGEOID
    }
}

/// for numeric/numrange
unsafe impl RangeSubType for AnyNumeric {
    fn range_type_oid() -> Oid {
        pg_sys::NUMRANGEOID
    }
}

/// for numeric/numrange
unsafe impl<const P: u32, const S: u32> RangeSubType for Numeric<P, S> {
    fn range_type_oid() -> Oid {
        pg_sys::NUMRANGEOID
    }
}

/// for date/daterange
unsafe impl RangeSubType for Date {
    fn range_type_oid() -> Oid {
        pg_sys::DATERANGEOID
    }
}

/// for Timestamp/tsrange
unsafe impl RangeSubType for Timestamp {
    fn range_type_oid() -> Oid {
        pg_sys::TSRANGEOID
    }
}

/// for Timestamp With Time Zone/tstzrange
unsafe impl RangeSubType for TimestampWithTimeZone {
    fn range_type_oid() -> Oid {
        pg_sys::TSTZRANGEOID
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RangeConversionError {
    #[error("Datum was null, unable to convert to RangeType")]
    NullDatum,
    #[error("Datum was not a valid pg_sys::RangeType, unable to convert to RangeType")]
    InvalidDatum,
}

unsafe impl SqlTranslatable for Range<i32> {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("int4range"))
    }
    fn return_sql() -> Result<Returns, ReturnsError> {
        Ok(Returns::One(SqlMapping::literal("int4range")))
    }
}

unsafe impl SqlTranslatable for Range<i64> {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("int8range"))
    }
    fn return_sql() -> Result<Returns, ReturnsError> {
        Ok(Returns::One(SqlMapping::literal("int8range")))
    }
}

unsafe impl SqlTranslatable for Range<AnyNumeric> {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("numrange"))
    }
    fn return_sql() -> Result<Returns, ReturnsError> {
        Ok(Returns::One(SqlMapping::literal("numrange")))
    }
}

unsafe impl<const P: u32, const S: u32> SqlTranslatable for Range<Numeric<P, S>> {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("numrange"))
    }
    fn return_sql() -> Result<Returns, ReturnsError> {
        Ok(Returns::One(SqlMapping::literal("numrange")))
    }
}

unsafe impl SqlTranslatable for Range<Date> {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("daterange"))
    }
    fn return_sql() -> Result<Returns, ReturnsError> {
        Ok(Returns::One(SqlMapping::literal("daterange")))
    }
}

unsafe impl SqlTranslatable for Range<TimestampWithTimeZone> {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("tstzrange"))
    }
    fn return_sql() -> Result<Returns, ReturnsError> {
        Ok(Returns::One(SqlMapping::literal("tstzrange")))
    }
}

unsafe impl SqlTranslatable for Range<Timestamp> {
    fn argument_sql() -> Result<SqlMapping, ArgumentError> {
        Ok(SqlMapping::literal("tsrange"))
    }
    fn return_sql() -> Result<Returns, ReturnsError> {
        Ok(Returns::One(SqlMapping::literal("tsrange")))
    }
}
