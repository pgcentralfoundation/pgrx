/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use crate::{
    pg_sys, void_mut_ptr, Date, FromDatum, IntoDatum, Numeric, Timestamp, TimestampWithTimeZone,
};
use pgx_pg_sys::{Oid, RangeBound};
use std::default::Default;
use std::marker::PhantomData;
// use std::ops::Range;

pub trait RangeSubType {
    fn range_type_oid() -> Oid;
}

pub struct PgRange<T: FromDatum + IntoDatum + RangeSubType> {
    ptr: *mut pg_sys::varlena,
    range_type: *mut pg_sys::RangeType,
    pub lower: RangeBound,
    pub upper: RangeBound,
    pub is_empty: bool,
    __marker: PhantomData<T>,
}

impl<T: FromDatum + IntoDatum + RangeSubType> PgRange<T> {
    unsafe fn from_pg(
        ptr: *mut pg_sys::varlena,
        range_type: *mut pg_sys::RangeType,
        lower_bound: pg_sys::RangeBound,
        upper_bound: pg_sys::RangeBound,
        is_empty: bool,
    ) -> Self {
        PgRange {
            ptr,
            range_type,
            lower: lower_bound,
            upper: upper_bound,
            is_empty,
            __marker: PhantomData,
        }
    }

    #[inline]
    pub fn lower_val(&self) -> Option<T> {
        if self.is_empty || self.lower.infinite {
            None
        } else {
            unsafe { T::from_datum(self.lower.val.clone(), false) }
        }
    }

    #[inline]
    pub fn upper_val(&self) -> Option<T> {
        if self.is_empty || self.upper.infinite {
            None
        } else {
            unsafe { T::from_datum(self.upper.val.clone(), false) }
        }
    }

    pub fn empty_range() -> Self {
        Self::from_values_internal(None, None, true)
    }

    pub fn from_values(lower: Option<T>, upper: Option<T>) -> Self {
        Self::from_values_internal(lower, upper, false)
    }

    pub fn from_bounds(lower_bound: RangeBound, upper_bound: RangeBound) {
        unsafe {
            Self::from_bounds_internal(lower_bound, upper_bound, false);
        }
    }

    unsafe fn from_bounds_internal(
        mut lower_bound: RangeBound,
        mut upper_bound: RangeBound,
        is_empty: bool,
    ) -> Self {
        let typecache =
            pg_sys::lookup_type_cache(T::range_type_oid(), pg_sys::TYPECACHE_RANGE_INFO as i32);

        let range_type =
            pg_sys::make_range(typecache, &mut lower_bound, &mut upper_bound, is_empty);

        Self::from_pg(
            range_type as *mut pg_sys::varlena,
            range_type,
            lower_bound,
            upper_bound,
            is_empty,
        )
    }

    fn from_values_internal(lower: Option<T>, upper: Option<T>, is_empty: bool) -> Self {
        let mut lower_bound = RangeBound::default();
        lower_bound.lower = true;
        lower_bound.inclusive = true;
        let mut upper_bound = RangeBound::default();
        upper_bound.lower = false;
        upper_bound.inclusive = false;

        if !is_empty {
            match lower {
                Some(lower_val) => {
                    lower_bound.val = lower_val.into_datum().expect("lower value datum was null");
                    lower_bound.infinite = false;
                }
                None => {
                    lower_bound.infinite = true;
                }
            }

            match upper {
                Some(upper_val) => {
                    upper_bound.val = upper_val.into_datum().expect("upper value datum was null");
                    upper_bound.infinite = false;
                }
                None => {
                    upper_bound.infinite = true;
                }
            }
        }
        unsafe { Self::from_bounds_internal(lower_bound, upper_bound, is_empty) }
    }
}

impl<T: FromDatum + IntoDatum + RangeSubType> FromDatum for PgRange<T> {
    #[inline]
    unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool) -> Option<PgRange<T>> {
        if is_null {
            None
        } else {
            let ptr = datum.ptr_cast();
            let range_type = pg_sys::pg_detoast_datum(datum.ptr_cast()) as *mut pg_sys::RangeType;
            let _ = range_type.as_ref().expect("RangeType * was NULL");

            let typecache = pg_sys::lookup_type_cache(
                (*range_type).rangetypid,
                pg_sys::TYPECACHE_RANGE_INFO as i32,
            );

            let mut lower_bound: RangeBound = Default::default();
            let mut upper_bound: RangeBound = Default::default();
            let mut is_empty = false;

            pg_sys::range_deserialize(
                typecache,
                range_type,
                &mut lower_bound,
                &mut upper_bound,
                &mut is_empty,
            );

            Some(PgRange::from_pg(
                ptr,
                range_type,
                lower_bound,
                upper_bound,
                is_empty,
            ))
        }
    }
}

impl<T: FromDatum + IntoDatum + RangeSubType> Drop for PgRange<T> {
    fn drop(&mut self) {
        if !self.range_type.is_null() && self.range_type as *mut pg_sys::varlena != self.ptr {
            unsafe {
                pg_sys::pfree(self.range_type as void_mut_ptr);
            }
        }
    }
}

impl<T: FromDatum + IntoDatum + RangeSubType> IntoDatum for PgRange<T> {
    fn into_datum(self) -> Option<pg_sys::Datum> {
        Some(self.range_type.into())
    }

    fn type_oid() -> u32 {
        unsafe { pg_sys::get_range_subtype(T::type_oid()) }
    }
}

// impl<T: FromDatum + RangeSubType> FromDatum for Range<T> {
//     #[inline]
//     unsafe fn from_datum(datum: pg_sys::Datum, is_null: bool) -> Option<Self> {
//         if is_null {
//             None
//         } else {
//             let range = pg_sys::pg_detoast_datum(datum.ptr_cast()) as *mut pg_sys::RangeType;
//             let _ = range.as_ref().expect("RangeType * was NULL");

//             let typecache =
//                 pg_sys::lookup_type_cache((*range).rangetypid, pg_sys::TYPECACHE_RANGE_INFO as i32);

//             let mut lower_bound: RangeBound = Default::default();
//             let mut upper_bound: RangeBound = Default::default();
//             let mut is_empty = false;
//             pg_sys::range_deserialize(
//                 typecache,
//                 range,
//                 &mut lower_bound,
//                 &mut upper_bound,
//                 &mut is_empty,
//             );

//             if is_empty {
//                 return None;
//             }

//             match (
//                 T::from_datum(lower_bound.val, false),
//                 T::from_datum(upper_bound.val, false),
//             ) {
//                 (Some(lower_val), Some(upper_val)) => Some(lower_val..upper_val),
//                 _ => None,
//             }
//         }
//     }
// }

// impl<T: IntoDatum + RangeSubType> IntoDatum for Range<T> {
//     fn into_datum(self) -> Option<pg_sys::Datum> {
//         unsafe {
//             crate::log::elog(crate::log::PgLogLevel::INFO, "here");
//             let is_empty = false;

//             let typecache =
//                 pg_sys::lookup_type_cache(T::range_type_oid(), pg_sys::TYPECACHE_RANGE_INFO as i32);

//             let mut lower_bound = RangeBound {
//                 inclusive: true,
//                 infinite: false,
//                 lower: true,
//                 val: T::into_datum(self.start).unwrap(),
//                 ..Default::default()
//             };

//             let mut upper_bound = RangeBound {
//                 inclusive: false,
//                 infinite: false,
//                 lower: false,
//                 val: T::into_datum(self.end).unwrap(),
//                 ..Default::default()
//             };

//             crate::log::elog(crate::log::PgLogLevel::INFO, "here2");

//             let range_type =
//                 pg_sys::make_range(typecache, &mut lower_bound, &mut upper_bound, is_empty);

//             crate::log::elog(crate::log::PgLogLevel::INFO, "here3");
//             Some(range_type.into())
//         }
//     }
//     fn type_oid() -> pg_sys::Oid {
//         T::range_type_oid()
//     }
// }

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
