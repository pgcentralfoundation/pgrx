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
use core::ops::ControlFlow;

/// Unboxing for arguments
///
/// This bound is necessary to distinguish things which can be passed into `#[pg_extern] fn`.
/// It is strictly a mistake to use the BorrowDatum/UnboxDatum/DetoastDatum traits for this bound!
/// PGRX allows "phantom arguments" which are not actually present in the C function, and are also
/// omitted in the SQL, but are passed to the Rust function anyways.
pub trait UnboxArg {
    /// indicates min/max number of args that may be consumed if statically known
    fn arg_width(_fcinfo: pg_sys::FunctionCallInfo) -> Option<(usize, usize)> {
        todo!()
    }

    /// try to unbox the next argument
    ///
    /// should play into a quasi-iterator somehow?
    fn try_unbox(_fcinfo: pg_sys::FunctionCallInfo, _current: usize) -> ControlFlow<Self, ()>
    where
        Self: Sized,
    {
        todo!()
    }
}

/// How to return a value from Rust to Postgres
///
/// This bound is necessary to distinguish things which can be passed in/out of `#[pg_extern] fn`.
/// This bound is not accurately described by IntoDatum or similar traits, as value conversions are
/// handled in a special way at function return boundaries, and may require mutating multiple fields
/// behind the FunctionCallInfo. The most exceptional case are set-returning functions.
pub unsafe trait ReturnShipping: Sized {
    /// The actual type returned from the call
    type Item: Sized;

    /// check the fcinfo state, initialize if necessary, and pick calling the wrapped fn or restoring Self
    ///
    /// the implementer must pick the correct memory context for the wrapped fn's allocations
    /// # safety
    /// must be called with a valid fcinfo
    unsafe fn prepare_call(_fcinfo: pg_sys::FunctionCallInfo) -> CallCx {
        CallCx::WrappedFn(unsafe { pg_sys::CurrentMemoryContext })
    }

    /// answer what kind and how many returns happen from this type
    fn label_ret(self) -> Ret<Self>;

    /// box the return value
    /// # Safety
    /// must be called with a valid fcinfo
    unsafe fn box_return(fcinfo: pg_sys::FunctionCallInfo, ret: Ret<Self>) -> pg_sys::Datum;

    /// for multi-call types, how to init them in the multi-call context, for all others: panic
    ///
    /// for all others: panic
    /// # Safety
    /// must be called with a valid fcinfo
    unsafe fn into_context(self, _fcinfo: pg_sys::FunctionCallInfo) {
        unimplemented!()
    }

    /// for multi-call types, how to restore them from the multi-call context
    ///
    /// for all others: panic
    /// # Safety
    /// must be called with a valid fcinfo
    unsafe fn ret_from_context(_fcinfo: pg_sys::FunctionCallInfo) -> Ret<Self> {
        unimplemented!()
    }

    /// must be called with a valid fcinfo
    unsafe fn finish_call(_fcinfo: pg_sys::FunctionCallInfo) {}
}

pub unsafe trait RetPackage: Sized {
    unsafe fn package_ret(self, fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum;
}

unsafe impl<T> ReturnShipping for T
where
    T: RetPackage,
{
    type Item = Self;

    fn label_ret(self) -> Ret<Self> {
        Ret::Once(self)
    }

    unsafe fn box_return(fcinfo: pg_sys::FunctionCallInfo, ret: Ret<Self>) -> pg_sys::Datum {
        match ret {
            Ret::Zero => unsafe { pg_return_null(fcinfo) },
            Ret::Once(ret) => ret.package_ret(fcinfo),
            Ret::Many(_, _) => unreachable!(),
        }
    }

    unsafe fn prepare_call(_fcinfo: pg_sys::FunctionCallInfo) -> CallCx {
        CallCx::WrappedFn(unsafe { pg_sys::CurrentMemoryContext })
    }

    unsafe fn into_context(self, _fcinfo: pg_sys::FunctionCallInfo) {
        unimplemented!()
    }
    unsafe fn ret_from_context(_fcinfo: pg_sys::FunctionCallInfo) -> Ret<Self> {
        unimplemented!()
    }
    unsafe fn finish_call(_fcinfo: pg_sys::FunctionCallInfo) {}
}

pub enum CallCx {
    RestoreCx,
    WrappedFn(pg_sys::MemoryContext),
}

pub enum Ret<T: ReturnShipping> {
    Zero,
    Once(T::Item),
    Many(T, T::Item),
}

unsafe impl<T> RetPackage for Option<T>
where
    T: RetPackage,
{
    unsafe fn package_ret(self, fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
        match self {
            None => unsafe { pg_return_null(fcinfo) },
            Some(value) => value.package_ret(fcinfo),
        }
    }
}

unsafe impl<T, E> ReturnShipping for Result<T, E>
where
    T: ReturnShipping,
    T::Item: ReturnShipping,
    E: core::any::Any + core::fmt::Display,
{
    type Item = T::Item;

    unsafe fn prepare_call(fcinfo: pg_sys::FunctionCallInfo) -> CallCx {
        T::prepare_call(fcinfo)
    }

    fn label_ret(self) -> Ret<Self> {
        let value = pg_sys::panic::ErrorReportable::unwrap_or_report(self);
        match T::label_ret(value) {
            Ret::Zero => Ret::Zero,
            Ret::Once(value) => Ret::Once(value),
            Ret::Many(iter, value) => Ret::Many(Ok(iter), value),
        }
    }

    unsafe fn box_return(fcinfo: pg_sys::FunctionCallInfo, ret: Ret<Self>) -> pg_sys::Datum {
        let ret = match ret {
            Ret::Zero => Ret::Zero,
            Ret::Once(value) => Ret::Once(value),
            Ret::Many(iter, value) => {
                let iter = pg_sys::panic::ErrorReportable::unwrap_or_report(iter);
                Ret::Many(iter, value)
            }
        };

        T::box_return(fcinfo, ret)
    }

    unsafe fn into_context(self, fcinfo: pg_sys::FunctionCallInfo) {
        match self {
            Err(_) => (),
            Ok(value) => value.into_context(fcinfo),
        }
    }

    unsafe fn ret_from_context(fcinfo: pg_sys::FunctionCallInfo) -> Ret<Self> {
        match T::ret_from_context(fcinfo) {
            Ret::Many(iter, value) => Ret::Many(Ok(iter), value),
            Ret::Once(value) => Ret::Once(value),
            Ret::Zero => Ret::Zero,
        }
    }

    unsafe fn finish_call(fcinfo: pg_sys::FunctionCallInfo) {
        T::finish_call(fcinfo)
    }
}

macro_rules! return_packaging_for_primitives {
    ($($scalar:ty),*) => {
        $(
        unsafe impl RetPackage for $scalar {
            unsafe fn package_ret(self, _fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
                $crate::pg_sys::Datum::from(self)
            }
        }
        )*
    }
}

return_packaging_for_primitives! {
    i8, i16, i32, i64, bool
}

unsafe impl RetPackage for () {
    unsafe fn package_ret(self, _fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
        pg_sys::Datum::from(0)
    }
}

unsafe impl RetPackage for f32 {
    unsafe fn package_ret(self, _fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
        pg_sys::Datum::from(self.to_bits())
    }
}

unsafe impl RetPackage for f64 {
    unsafe fn package_ret(self, _fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
        pg_sys::Datum::from(self.to_bits())
    }
}

fn repackage_into_datum<T>(fcinfo: pg_sys::FunctionCallInfo, ret: T) -> pg_sys::Datum
where
    T: RetPackage + IntoDatum,
{
    match ret.into_datum() {
        None => unsafe { pg_return_null(fcinfo) },
        Some(datum) => datum,
    }
}

unsafe impl<'a> RetPackage for &'a [u8] {
    unsafe fn package_ret(self, fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
        repackage_into_datum(fcinfo, self)
    }
}

unsafe impl<'a> RetPackage for &'a str {
    unsafe fn package_ret(self, fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
        repackage_into_datum(fcinfo, self)
    }
}

unsafe impl<'a> RetPackage for &'a CStr {
    unsafe fn package_ret(self, fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
        repackage_into_datum(fcinfo, self)
    }
}

macro_rules! impl_repackage_into_datum {
    ($($boxable:ty),*) => {
        $(
        unsafe impl RetPackage for $boxable {
            unsafe fn package_ret(self, fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
                repackage_into_datum(fcinfo, self)
            }
        })*
    };
}

impl_repackage_into_datum! {
    String, CString, Json, Inet, Uuid, AnyNumeric, Vec<u8>,
    Date, Interval, Time, TimeWithTimeZone, Timestamp, TimestampWithTimeZone,
    pg_sys::Oid, pg_sys::BOX, pg_sys::Point, char,
    Internal
}

unsafe impl<const P: u32, const S: u32> RetPackage for crate::Numeric<P, S> {
    unsafe fn package_ret(self, fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
        repackage_into_datum(fcinfo, self)
    }
}

unsafe impl<T> RetPackage for crate::Range<T>
where
    T: IntoDatum + crate::RangeSubType,
{
    unsafe fn package_ret(self, fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
        repackage_into_datum(fcinfo, self)
    }
}

unsafe impl<T> RetPackage for Vec<T>
where
    T: IntoDatum,
{
    unsafe fn package_ret(self, fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
        repackage_into_datum(fcinfo, self)
    }
}

unsafe impl<T: Copy> RetPackage for PgVarlena<T> {
    unsafe fn package_ret(self, fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
        repackage_into_datum(fcinfo, self)
    }
}

unsafe impl<'mcx, A> RetPackage for PgHeapTuple<'mcx, A>
where
    A: crate::WhoAllocated,
{
    unsafe fn package_ret(self, fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
        repackage_into_datum(fcinfo, self)
    }
}

unsafe impl<T, A> RetPackage for PgBox<T, A>
where
    A: crate::WhoAllocated,
{
    unsafe fn package_ret(self, fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
        repackage_into_datum(fcinfo, self)
    }
}
