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
//! Helper implementations for returning sets and tables from `#[pg_extern]`-style functions
use std::ffi::{CStr, CString};

use crate::heap_tuple::PgHeapTuple;
use crate::{
    pg_return_null, pg_sys, AnyNumeric, Date, Inet, Internal, Interval, IntoDatum, Json, PgBox,
    PgVarlena, Time, TimeWithTimeZone, Timestamp, TimestampWithTimeZone, Uuid,
};

/// How to return a value from Rust to Postgres
///
/// This bound is necessary to distinguish things which can be passed in/out of `#[pg_extern] fn`.
/// This bound is not accurately described by IntoDatum or similar traits, as value conversions are
/// handled in a special way at function return boundaries, and may require mutating multiple fields
/// behind the FunctionCallInfo. The most exceptional case are set-returning functions.
pub unsafe trait RetAbi: Sized {
    /// The actual type returned from the call
    type Item: Sized;

    /// check the fcinfo state, initialize if necessary, and pick calling the wrapped fn or restoring Self
    ///
    /// the implementer must pick the correct memory context for the wrapped fn's allocations
    /// # safety
    /// must be called with a valid fcinfo
    unsafe fn check_fcinfo_and_prepare(_fcinfo: pg_sys::FunctionCallInfo) -> CallCx {
        CallCx::WrappedFn(unsafe { pg_sys::CurrentMemoryContext })
    }

    /// answer what kind and how many returns happen from this type
    fn label_ret(self) -> Ret<Self>;

    /// box the return value
    /// # Safety
    /// must be called with a valid fcinfo
    unsafe fn box_ret_in_fcinfo(fcinfo: pg_sys::FunctionCallInfo, ret: Ret<Self>) -> pg_sys::Datum;

    /// Some types require filling the fcinfo so they can later be restored
    /// # Safety
    /// must be called with a valid fcinfo
    unsafe fn move_into_fcinfo_fcx(self, _fcinfo: pg_sys::FunctionCallInfo);

    /// Some types require filling the fcinfo so they can later be restored
    /// # Safety
    /// must be called with a valid fcinfo
    unsafe fn fill_fcinfo_fcx(&self, _fcinfo: pg_sys::FunctionCallInfo);

    /// for multi-call types, how to restore them from the multi-call context
    ///
    /// for all others: panic
    /// # Safety
    /// must be called with a valid fcinfo
    unsafe fn ret_from_fcinfo_fcx(_fcinfo: pg_sys::FunctionCallInfo) -> Ret<Self> {
        unimplemented!()
    }

    /// must be called with a valid fcinfo
    unsafe fn finish_call_fcinfo(_fcinfo: pg_sys::FunctionCallInfo) {}
}

pub unsafe trait BoxRet: Sized {
    unsafe fn box_in_fcinfo(self, fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum;
}

unsafe impl<T> RetAbi for T
where
    T: BoxRet,
{
    type Item = Self;

    fn label_ret(self) -> Ret<Self> {
        Ret::Once(self)
    }

    unsafe fn box_ret_in_fcinfo(fcinfo: pg_sys::FunctionCallInfo, ret: Ret<Self>) -> pg_sys::Datum {
        match ret {
            Ret::Zero => unsafe { pg_return_null(fcinfo) },
            Ret::Once(ret) => ret.box_in_fcinfo(fcinfo),
            Ret::Many(_, _) => unreachable!(),
        }
    }

    unsafe fn check_fcinfo_and_prepare(_fcinfo: pg_sys::FunctionCallInfo) -> CallCx {
        CallCx::WrappedFn(unsafe { pg_sys::CurrentMemoryContext })
    }

    unsafe fn fill_fcinfo_fcx(&self, _fcinfo: pg_sys::FunctionCallInfo) {}
    unsafe fn move_into_fcinfo_fcx(self, _fcinfo: pg_sys::FunctionCallInfo) {}
    unsafe fn ret_from_fcinfo_fcx(_fcinfo: pg_sys::FunctionCallInfo) -> Ret<Self> {
        unimplemented!()
    }
    unsafe fn finish_call_fcinfo(_fcinfo: pg_sys::FunctionCallInfo) {}
}

pub enum CallCx {
    RestoreCx,
    WrappedFn(pg_sys::MemoryContext),
}

pub enum Ret<T: RetAbi> {
    Zero,
    Once(T::Item),
    Many(T, T::Item),
}

unsafe impl<T> BoxRet for Option<T>
where
    T: BoxRet,
{
    unsafe fn box_in_fcinfo(self, fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
        match self {
            None => unsafe { pg_return_null(fcinfo) },
            Some(value) => value.box_in_fcinfo(fcinfo),
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

    unsafe fn check_fcinfo_and_prepare(fcinfo: pg_sys::FunctionCallInfo) -> CallCx {
        T::check_fcinfo_and_prepare(fcinfo)
    }

    fn label_ret(self) -> Ret<Self> {
        let value = pg_sys::panic::ErrorReportable::unwrap_or_report(self);
        match T::label_ret(value) {
            Ret::Zero => Ret::Zero,
            Ret::Once(value) => Ret::Once(value),
            Ret::Many(iter, value) => Ret::Many(Ok(iter), value),
        }
    }

    unsafe fn box_ret_in_fcinfo(fcinfo: pg_sys::FunctionCallInfo, ret: Ret<Self>) -> pg_sys::Datum {
        let ret = match ret {
            Ret::Zero => Ret::Zero,
            Ret::Once(value) => Ret::Once(value),
            Ret::Many(iter, value) => {
                let iter = pg_sys::panic::ErrorReportable::unwrap_or_report(iter);
                Ret::Many(iter, value)
            }
        };

        T::box_ret_in_fcinfo(fcinfo, ret)
    }
    unsafe fn fill_fcinfo_fcx(&self, fcinfo: pg_sys::FunctionCallInfo) {
        match self {
            Ok(value) => value.fill_fcinfo_fcx(fcinfo),
            Err(_) => (),
        }
    }

    unsafe fn move_into_fcinfo_fcx(self, fcinfo: pg_sys::FunctionCallInfo) {
        match self {
            Ok(value) => value.move_into_fcinfo_fcx(fcinfo),
            Err(_) => (),
        }
    }

    unsafe fn ret_from_fcinfo_fcx(fcinfo: pg_sys::FunctionCallInfo) -> Ret<Self> {
        match T::ret_from_fcinfo_fcx(fcinfo) {
            Ret::Many(iter, value) => Ret::Many(Ok(iter), value),
            Ret::Once(value) => Ret::Once(value),
            Ret::Zero => Ret::Zero,
        }
    }

    unsafe fn finish_call_fcinfo(fcinfo: pg_sys::FunctionCallInfo) {
        T::finish_call_fcinfo(fcinfo)
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
