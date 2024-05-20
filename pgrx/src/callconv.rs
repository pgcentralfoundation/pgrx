//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
#![doc(hidden)]
#![deny(unsafe_op_in_unsafe_fn)]
//! Helper implementations for returning sets and tables from `#[pg_extern]`-style functions
use crate::heap_tuple::PgHeapTuple;
use crate::{
    pg_return_null, pg_sys, AnyNumeric, Date, Inet, Internal, Interval, IntoDatum, Json, PgBox,
    PgVarlena, Time, TimeWithTimeZone, Timestamp, TimestampWithTimeZone, Uuid,
};
use std::ffi::{CStr, CString};

/// How to return a value from Rust to Postgres
///
/// This bound is necessary to distinguish things which can be returned from a `#[pg_extern] fn`.
/// This bound is not accurately described by IntoDatum or similar traits, as value conversions are
/// handled in a special way at function return boundaries, and may require mutating multiple fields
/// behind the FunctionCallInfo. The most exceptional case are set-returning functions, which
/// require special handling for the fcinfo and also for certain inner types.
///
/// This trait is exposed to external code so macro-generated wrapper fn may expand to calls to it.
/// The number of invariants implementers must uphold is unlikely to be adequately documented.
/// Prefer to use RetAbi as a trait bound instead of implementing it, or even calling it, yourself.
pub unsafe trait RetAbi: Sized {
    /// Type returned to Postgres
    type Item: Sized;
    /// Driver for complex returns
    type Ret;

    /// Initialize the FunctionCallInfo for returns
    ///
    /// The implementer must pick the correct memory context for the wrapped fn's allocations.
    /// # Safety
    /// Requires a valid FunctionCallInfo.
    unsafe fn check_fcinfo_and_prepare(_fcinfo: pg_sys::FunctionCallInfo) -> CallCx {
        CallCx::WrappedFn(unsafe { pg_sys::CurrentMemoryContext })
    }

    /// answer what kind and how many returns happen from this type
    fn to_ret(self) -> Self::Ret;

    /// box the return value
    /// # Safety
    /// must be called with a valid fcinfo
    unsafe fn box_ret_in_fcinfo(fcinfo: pg_sys::FunctionCallInfo, ret: Self::Ret) -> pg_sys::Datum;

    /// Multi-call types want to be in the fcinfo so they can be restored
    /// # Safety
    /// must be called with a valid fcinfo
    unsafe fn move_into_fcinfo_fcx(self, _fcinfo: pg_sys::FunctionCallInfo);

    /// Other types want to add metadata to the fcinfo
    /// # Safety
    /// must be called with a valid fcinfo
    unsafe fn fill_fcinfo_fcx(&self, _fcinfo: pg_sys::FunctionCallInfo);

    /// for multi-call types, how to restore them from the multi-call context
    ///
    /// for all others: panic
    /// # Safety
    /// must be called with a valid fcinfo
    unsafe fn ret_from_fcinfo_fcx(_fcinfo: pg_sys::FunctionCallInfo) -> Self::Ret {
        unimplemented!()
    }

    /// must be called with a valid fcinfo
    unsafe fn finish_call_fcinfo(_fcinfo: pg_sys::FunctionCallInfo) {}
}

/// A simplified blanket RetAbi
pub unsafe trait BoxRet: Sized {
    unsafe fn box_in_fcinfo(self, fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum;
}

unsafe impl<T> RetAbi for T
where
    T: BoxRet,
{
    type Item = Self;
    type Ret = Self;

    fn to_ret(self) -> Self::Ret {
        self
    }

    unsafe fn box_ret_in_fcinfo(fcinfo: pg_sys::FunctionCallInfo, ret: Self::Ret) -> pg_sys::Datum {
        unsafe { ret.box_in_fcinfo(fcinfo) }
    }

    unsafe fn check_fcinfo_and_prepare(_fcinfo: pg_sys::FunctionCallInfo) -> CallCx {
        CallCx::WrappedFn(unsafe { pg_sys::CurrentMemoryContext })
    }

    unsafe fn fill_fcinfo_fcx(&self, _fcinfo: pg_sys::FunctionCallInfo) {}
    unsafe fn move_into_fcinfo_fcx(self, _fcinfo: pg_sys::FunctionCallInfo) {}
    unsafe fn ret_from_fcinfo_fcx(_fcinfo: pg_sys::FunctionCallInfo) -> Self::Ret {
        unimplemented!()
    }
    unsafe fn finish_call_fcinfo(_fcinfo: pg_sys::FunctionCallInfo) {}
}

/// Control flow for RetAbi
pub enum CallCx {
    RestoreCx,
    WrappedFn(pg_sys::MemoryContext),
}

unsafe impl<T> BoxRet for Option<T>
where
    T: BoxRet,
{
    unsafe fn box_in_fcinfo(self, fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
        unsafe {
            match self {
                None => pg_return_null(fcinfo),
                Some(value) => value.box_in_fcinfo(fcinfo),
            }
        }
    }
}

unsafe impl<T, E> RetAbi for Result<T, E>
where
    T: RetAbi,
    T::Item: RetAbi,
    E: core::any::Any + core::fmt::Display,
{
    type Item = T::Item;
    type Ret = T::Ret;

    unsafe fn check_fcinfo_and_prepare(fcinfo: pg_sys::FunctionCallInfo) -> CallCx {
        unsafe { T::check_fcinfo_and_prepare(fcinfo) }
    }

    fn to_ret(self) -> Self::Ret {
        let value = pg_sys::panic::ErrorReportable::unwrap_or_report(self);
        value.to_ret()
    }

    unsafe fn box_ret_in_fcinfo(fcinfo: pg_sys::FunctionCallInfo, ret: Self::Ret) -> pg_sys::Datum {
        unsafe { T::box_ret_in_fcinfo(fcinfo, ret) }
    }

    unsafe fn fill_fcinfo_fcx(&self, fcinfo: pg_sys::FunctionCallInfo) {
        match self {
            Ok(value) => unsafe { value.fill_fcinfo_fcx(fcinfo) },
            Err(_) => (),
        }
    }

    unsafe fn move_into_fcinfo_fcx(self, fcinfo: pg_sys::FunctionCallInfo) {
        match self {
            Ok(value) => unsafe { value.move_into_fcinfo_fcx(fcinfo) },
            Err(_) => (),
        }
    }

    unsafe fn ret_from_fcinfo_fcx(fcinfo: pg_sys::FunctionCallInfo) -> Self::Ret {
        unsafe { T::ret_from_fcinfo_fcx(fcinfo) }
    }

    unsafe fn finish_call_fcinfo(fcinfo: pg_sys::FunctionCallInfo) {
        unsafe { T::finish_call_fcinfo(fcinfo) }
    }
}

macro_rules! return_packaging_for_primitives {
    ($($scalar:ty),*) => {
        $(unsafe impl BoxRet for $scalar {
              unsafe fn box_in_fcinfo(self, _fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
                  $crate::pg_sys::Datum::from(self)
              }
        })*
    }
}

return_packaging_for_primitives!(i8, i16, i32, i64, bool);

unsafe impl BoxRet for () {
    unsafe fn box_in_fcinfo(self, _fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
        pg_sys::Datum::from(0)
    }
}

unsafe impl BoxRet for f32 {
    unsafe fn box_in_fcinfo(self, _fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
        pg_sys::Datum::from(self.to_bits())
    }
}

unsafe impl BoxRet for f64 {
    unsafe fn box_in_fcinfo(self, _fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
        pg_sys::Datum::from(self.to_bits())
    }
}

unsafe impl<'a> BoxRet for &'a [u8] {
    unsafe fn box_in_fcinfo(self, fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
        self.into_datum().unwrap_or_else(|| unsafe { pg_return_null(fcinfo) })
    }
}

unsafe impl<'a> BoxRet for &'a str {
    unsafe fn box_in_fcinfo(self, fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
        self.into_datum().unwrap_or_else(|| unsafe { pg_return_null(fcinfo) })
    }
}

unsafe impl<'a> BoxRet for &'a CStr {
    unsafe fn box_in_fcinfo(self, fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
        self.into_datum().unwrap_or_else(|| unsafe { pg_return_null(fcinfo) })
    }
}

macro_rules! impl_repackage_into_datum {
    ($($boxable:ty),*) => {
        $(unsafe impl BoxRet for $boxable {
              unsafe fn box_in_fcinfo(self, fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
                  self.into_datum().unwrap_or_else(|| unsafe { pg_return_null(fcinfo) })
              }
          })*
    };
}

impl_repackage_into_datum! {
    String, CString, Vec<u8>, char,
    Json, Inet, Uuid, AnyNumeric, Internal,
    Date, Interval, Time, TimeWithTimeZone, Timestamp, TimestampWithTimeZone,
    pg_sys::Oid, pg_sys::BOX, pg_sys::Point
}

unsafe impl<const P: u32, const S: u32> BoxRet for crate::Numeric<P, S> {
    unsafe fn box_in_fcinfo(self, fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
        self.into_datum().unwrap_or_else(|| unsafe { pg_return_null(fcinfo) })
    }
}

unsafe impl<T> BoxRet for crate::Range<T>
where
    T: IntoDatum + crate::RangeSubType,
{
    unsafe fn box_in_fcinfo(self, fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
        self.into_datum().unwrap_or_else(|| unsafe { pg_return_null(fcinfo) })
    }
}

unsafe impl<T> BoxRet for Vec<T>
where
    T: IntoDatum,
{
    unsafe fn box_in_fcinfo(self, fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
        self.into_datum().unwrap_or_else(|| unsafe { pg_return_null(fcinfo) })
    }
}

unsafe impl<T: Copy> BoxRet for PgVarlena<T> {
    unsafe fn box_in_fcinfo(self, fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
        self.into_datum().unwrap_or_else(|| unsafe { pg_return_null(fcinfo) })
    }
}

unsafe impl<'mcx, A> BoxRet for PgHeapTuple<'mcx, A>
where
    A: crate::WhoAllocated,
{
    unsafe fn box_in_fcinfo(self, fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
        self.into_datum().unwrap_or_else(|| unsafe { pg_return_null(fcinfo) })
    }
}

unsafe impl<T, A> BoxRet for PgBox<T, A>
where
    A: crate::WhoAllocated,
{
    unsafe fn box_in_fcinfo(self, fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
        self.into_datum().unwrap_or_else(|| unsafe { pg_return_null(fcinfo) })
    }
}
