use std::marker::PhantomData;

use crate::{
    pg_sys, void_mut_ptr, Date, FromDatum, IntoDatum, Numeric, Timestamp, TimestampWithTimeZone,
};

use pgx_pg_sys::{Oid, RangeBound};

pub struct Range<T: FromDatum + IntoDatum + RangeSubType> {
    ptr: *mut pg_sys::varlena,
    range_type: *mut pg_sys::RangeType,
    __marker: PhantomData<T>,
}

impl<T> Range<T>
where
    T: FromDatum + IntoDatum + RangeSubType,
{
    unsafe fn from_pg(ptr: *mut pg_sys::varlena, range_type: *mut pg_sys::RangeType) -> Self {
        Range {
            ptr,
            range_type,
            __marker: PhantomData,
        }
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
            unsafe {
                let ptr = datum.ptr_cast();
                let range_type =
                    pg_sys::pg_detoast_datum(datum.ptr_cast()) as *mut pg_sys::RangeType;
                let _ = range_type.as_ref().expect("RangeType * was NULL");

                Ok(Range::<T>::from_pg(ptr, range_type))
            }
        }
    }
}

impl<T> FromDatum for Range<T>
where
    T: FromDatum + IntoDatum + RangeSubType,
{
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool) -> Option<Self>
    where
        Self: Sized,
    {
        if is_null {
            None
        } else {
            Some(datum.try_into().expect("Error converting RangeType datum"))
        }
    }
}

impl<T> IntoDatum for Range<T>
where
    T: FromDatum + IntoDatum + RangeSubType,
{
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(self.range_type.into())
    }

    fn type_oid() -> pg_sys::Oid {
        T::range_type_oid()
    }
}

impl<T> Drop for Range<T>
where
    T: FromDatum + IntoDatum + RangeSubType,
{
    fn drop(&mut self) {
        if !self.range_type.is_null() && self.range_type as *mut pg_sys::varlena != self.ptr {
            unsafe {
                pg_sys::pfree(self.range_type as void_mut_ptr);
            }
        }
    }
}

// A deserialized state of the RangeType's data
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
    pub fn lower_val(&self) -> Option<T> {
        if self.is_empty || self.lower.infinite {
            None
        } else {
            unsafe { T::from_datum(self.lower.val, false) }
        }
    }

    pub fn upper_val(&self) -> Option<T> {
        if self.is_empty || self.upper.infinite {
            None
        } else {
            unsafe { T::from_datum(self.upper.val, false) }
        }
    }

    pub fn empty_range_data() -> Self {
        let lower_bound = RangeBound {
            lower: true,
            ..RangeBound::default()
        };
        let upper_bound = RangeBound {
            lower: false,
            ..RangeBound::default()
        };
        Self::from_range_bounds_internal(lower_bound, upper_bound, true)
    }

    pub fn from_range_bounds(lower_bound: RangeBound, upper_bound: RangeBound) -> Self {
        Self::from_range_bounds_internal(lower_bound, upper_bound, false)
    }

    pub(crate) fn from_range_bounds_internal(
        lower_bound: RangeBound,
        upper_bound: RangeBound,
        is_empty: bool,
    ) -> Self {
        RangeData {
            lower: lower_bound,
            upper: upper_bound,
            is_empty,
            __marker: PhantomData,
        }
    }

    pub fn from_range_values(
        lower_val: Option<T>,
        upper_val: Option<T>,
        lower_inc: bool,
        upper_inc: bool,
    ) -> Self {
        let mut lower_bound = RangeBound {
            lower: true,
            inclusive: lower_inc,
            ..Default::default()
        };

        let mut upper_bound = RangeBound {
            lower: false,
            inclusive: upper_inc,
            ..Default::default()
        };

        match lower_val {
            Some(lower_val) => {
                lower_bound.val = lower_val
                    .into_datum()
                    .expect("Couldn't convert lower_val to Datum");
            }
            None => {
                lower_bound.infinite = true;
            }
        }

        match upper_val {
            Some(upper_val) => {
                upper_bound.val = upper_val
                    .into_datum()
                    .expect("Couldn't convert upper_val to Datum");
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
    fn from(range: Range<T>) -> Self {
        let mut lower_bound: RangeBound = Default::default();
        let mut upper_bound: RangeBound = Default::default();
        let mut is_empty = false;

        unsafe {
            let typecache = pg_sys::lookup_type_cache(
                (*(range.range_type)).rangetypid,
                pg_sys::TYPECACHE_RANGE_INFO as i32,
            );

            pg_sys::range_deserialize(
                typecache,
                range.range_type,
                &mut lower_bound,
                &mut upper_bound,
                &mut is_empty,
            );
        }

        RangeData::from_range_bounds_internal(lower_bound, upper_bound, is_empty)
    }
}

impl<T> From<RangeData<T>> for Range<T>
where
    T: FromDatum + IntoDatum + RangeSubType,
{
    fn from(range_data: RangeData<T>) -> Self {
        unsafe {
            let typecache =
                pg_sys::lookup_type_cache(T::range_type_oid(), pg_sys::TYPECACHE_RANGE_INFO as i32);

            let mut lower_bound = range_data.lower;
            let mut upper_bound = range_data.upper;

            let range_type = pg_sys::make_range(
                typecache,
                &mut lower_bound,
                &mut upper_bound,
                range_data.is_empty,
            );

            Range::<T>::from_pg(range_type as *mut pg_sys::varlena, range_type)
        }
    }
}

pub trait RangeSubType {
    fn range_type_oid() -> Oid;
}

impl RangeSubType for i32 {
    fn range_type_oid() -> Oid {
        pg_sys::INT4RANGEOID
    }
}

impl RangeSubType for i64 {
    fn range_type_oid() -> Oid {
        pg_sys::INT8RANGEOID
    }
}

impl RangeSubType for Numeric {
    fn range_type_oid() -> Oid {
        pg_sys::NUMRANGEOID
    }
}

impl RangeSubType for Date {
    fn range_type_oid() -> Oid {
        pg_sys::DATERANGEOID
    }
}

impl RangeSubType for Timestamp {
    fn range_type_oid() -> Oid {
        pg_sys::TSRANGEOID
    }
}

impl RangeSubType for TimestampWithTimeZone {
    fn range_type_oid() -> Oid {
        pg_sys::TSTZRANGEOID
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RangeConversionError {
    #[error("Datum was null, unable to convert to RangeType")]
    NullDatum,
}
