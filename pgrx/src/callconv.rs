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
#![allow(unused)]
//! Helper implementations for returning sets and tables from `#[pg_extern]`-style functions
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::mem::{self, ManuallyDrop};

use crate::heap_tuple::PgHeapTuple;
use crate::iter::{SetOfIterator, TableIterator};
use crate::ptr::PointerExt;
use crate::{
    nonstatic_typeid, pg_return_null, pg_sys, srf_is_first_call, srf_return_done, srf_return_next,
    AnyNumeric, Date, Inet, Internal, Interval, IntoDatum, IntoHeapTuple, Json, PgBox,
    PgMemoryContexts, PgVarlena, Time, TimeWithTimeZone, Timestamp, TimestampWithTimeZone, Uuid,
};
use core::ops::ControlFlow;
use core::ptr;

/// Unboxing for arguments
///
/// This bound is necessary to distinguish things which can be passed into `#[pg_extern] fn`.
/// It is strictly a mistake to use the BorrowDatum/UnboxDatum/DetoastDatum traits for this bound!
/// PGRX allows "phantom arguments" which are not actually present in the C function, and are also
/// omitted in the SQL, but are passed to the Rust function anyways.
pub trait UnboxArg {
    /// indicates min/max number of args that may be consumed if statically known
    fn arg_width(fcinfo: pg_sys::FunctionCallInfo) -> Option<(usize, usize)> {
        todo!()
    }

    /// try to unbox the next argument
    ///
    /// should play into a quasi-iterator somehow?
    fn try_unbox(fcinfo: pg_sys::FunctionCallInfo, current: usize) -> ControlFlow<Self, ()>
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
    type CallRet: Sized;

    /// check the fcinfo state, initialize if necessary, and pick calling the wrapped fn or restoring Self
    ///
    /// the implementer must pick the correct memory context for the wrapped fn's allocations
    /// # safety
    /// must be called with a valid fcinfo
    unsafe fn prepare_call(fcinfo: pg_sys::FunctionCallInfo) -> CallCx {
        CallCx::WrappedFn(unsafe { pg_sys::CurrentMemoryContext })
    }

    /// answer what kind and how many returns happen from this type
    ///
    /// must be overridden if `Self != Self::CallRet`
    fn label_ret(self) -> Ret<Self>;

    // morally I should be allowed to supply a default impl >:(
    /* this default impl would work, but at what cost?
    {
        if nonstatic_typeid::<Self>() == nonstatic_typeid::<Self::CallRet>() {
            // SAFETY: We materialize a copy, then evaporate the original without dropping it,
            // but only when we know that the original Self is the same type as Self::CallRet!
            unsafe {
                let ret = Ret::Once(::core::mem::transmute_copy(&self));
                ::core::mem::forget(self);
                ret
            }
        } else {
            panic!("`BoxRet::into_ret` must be overridden for {}", ::core::any::type_name::<Self>())
        }
    }
    */

    /// box the return value
    /// # Safety
    /// must be called with a valid fcinfo
    unsafe fn box_return(fcinfo: pg_sys::FunctionCallInfo, ret: Ret<Self>) -> pg_sys::Datum;

    /// for multi-call types, how to init them in the multi-call context, for all others: panic
    ///
    /// for all others: panic
    /// # Safety
    /// must be called with a valid fcinfo
    unsafe fn into_context(self, fcinfo: pg_sys::FunctionCallInfo) {
        unimplemented!()
    }

    /// for multi-call types, how to restore them from the multi-call context
    ///
    /// for all others: panic
    /// # Safety
    /// must be called with a valid fcinfo
    unsafe fn ret_from_context(fcinfo: pg_sys::FunctionCallInfo) -> Ret<Self> {
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
    type CallRet = Self;
    fn label_ret(self) -> Ret<Self> {
        Ret::Once(self)
    }

    unsafe fn box_return(fcinfo: pg_sys::FunctionCallInfo, ret: Ret<Self>) -> pg_sys::Datum {
        let ret = match ret {
            Ret::Zero => unsafe { return pg_return_null(fcinfo) },
            Ret::Once(ret) => ret,
            Ret::Many(_, _) => unreachable!(),
        };

        ret.package_ret(fcinfo)
    }

    unsafe fn prepare_call(fcinfo: pg_sys::FunctionCallInfo) -> CallCx {
        CallCx::WrappedFn(pg_sys::CurrentMemoryContext)
    }
    unsafe fn into_context(self, fcinfo: pg_sys::FunctionCallInfo) {
        unimplemented!()
    }

    unsafe fn ret_from_context(fcinfo: pg_sys::FunctionCallInfo) -> Ret<Self> {
        unimplemented!()
    }

    unsafe fn finish_call(_fcinfo: pg_sys::FunctionCallInfo) {}
}

pub enum CallCx {
    RestoreCx,
    WrappedFn(pg_sys::MemoryContext),
}

fn prepare_value_per_call_srf(fcinfo: pg_sys::FunctionCallInfo) -> CallCx {
    unsafe {
        if srf_is_first_call(fcinfo) {
            let fn_call_cx = pg_sys::init_MultiFuncCall(fcinfo);
            CallCx::WrappedFn((*fn_call_cx).multi_call_memory_ctx)
        } else {
            CallCx::RestoreCx
        }
    }
}

unsafe impl<'a, T> ReturnShipping for SetOfIterator<'a, T>
where
    T: ReturnShipping,
{
    type CallRet = <Self as Iterator>::Item;
    unsafe fn prepare_call(fcinfo: pg_sys::FunctionCallInfo) -> CallCx {
        prepare_value_per_call_srf(fcinfo)
    }

    fn label_ret(self) -> Ret<Self> {
        let mut iter = self;
        let ret = iter.next();
        match ret {
            None => Ret::Zero,
            Some(value) => Ret::Many(iter, value),
        }
    }

    unsafe fn box_return(fcinfo: pg_sys::FunctionCallInfo, ret: Ret<Self>) -> pg_sys::Datum {
        let fcx = deref_fcx(fcinfo);
        let value = match ret {
            Ret::Zero => return empty_srf(fcinfo),
            Ret::Once(value) => value,
            Ret::Many(iter, value) => {
                iter.into_context(fcinfo);
                value
            }
        };

        unsafe {
            let fcx = deref_fcx(fcinfo);
            srf_return_next(fcinfo, fcx);
            T::box_return(fcinfo, value.label_ret())
        }
    }

    unsafe fn into_context(self, fcinfo: pg_sys::FunctionCallInfo) {
        let fcx = deref_fcx(fcinfo);
        unsafe {
            let ptr = srf_memcx(fcx).leak_and_drop_on_delete(self);
            // it's the first call so we need to finish setting up fcx
            (*fcx).user_fctx = ptr.cast();
        }
    }

    unsafe fn ret_from_context(fcinfo: pg_sys::FunctionCallInfo) -> Ret<Self> {
        let fcx = deref_fcx(fcinfo);
        // SAFETY: fcx.user_fctx was set earlier, immediately before or in a prior call
        let mut iter = &mut *(*fcx).user_fctx.cast::<TableIterator<T>>();
        match iter.next() {
            None => Ret::Zero,
            Some(value) => Ret::Once(value),
        }
    }

    unsafe fn finish_call(fcinfo: pg_sys::FunctionCallInfo) {
        let fcx = deref_fcx(fcinfo);
        unsafe { srf_return_done(fcinfo, fcx) }
    }
}

pub enum Ret<T: ReturnShipping> {
    Zero,
    Once(T::CallRet),
    Many(T, T::CallRet),
}

unsafe impl<'a, T> ReturnShipping for TableIterator<'a, T>
where
    T: ReturnShipping,
{
    type CallRet = <Self as Iterator>::Item;

    unsafe fn prepare_call(fcinfo: pg_sys::FunctionCallInfo) -> CallCx {
        prepare_value_per_call_srf(fcinfo)
    }

    fn label_ret(self) -> Ret<Self> {
        let mut iter = self;
        let ret = iter.next();
        match ret {
            None => Ret::Zero,
            Some(value) => Ret::Many(iter, value),
        }
    }

    unsafe fn box_return(fcinfo: pg_sys::FunctionCallInfo, ret: Ret<Self>) -> pg_sys::Datum {
        let value = match ret {
            Ret::Zero => return empty_srf(fcinfo),
            Ret::Once(value) => value,
            Ret::Many(iter, value) => {
                iter.into_context(fcinfo);
                value
            }
        };

        unsafe {
            let fcx = deref_fcx(fcinfo);
            srf_return_next(fcinfo, fcx);
            T::box_return(fcinfo, value.label_ret())
        }
    }

    unsafe fn into_context(self, fcinfo: pg_sys::FunctionCallInfo) {
        // FIXME: this is assigned here but used in the tuple impl?
        let fcx = deref_fcx(fcinfo);
        unsafe {
            let ptr = srf_memcx(fcx).switch_to(move |mcx| {
                let mut tupdesc = ptr::null_mut();
                let mut oid = pg_sys::Oid::default();
                let ty_class = pg_sys::get_call_result_type(fcinfo, &mut oid, &mut tupdesc);
                if tupdesc.is_non_null() {
                    pg_sys::BlessTupleDesc(tupdesc);
                }
                (*fcx).tuple_desc = tupdesc;
                mcx.leak_and_drop_on_delete(self)
            });
            // it's the first call so we need to finish setting up fcx
            (*fcx).user_fctx = ptr.cast();
        }
    }

    unsafe fn ret_from_context(fcinfo: pg_sys::FunctionCallInfo) -> Ret<Self> {
        let fcx = deref_fcx(fcinfo);
        // SAFETY: fcx.user_fctx was set earlier, immediately before or in a prior call
        let mut iter = &mut *(*fcx).user_fctx.cast::<TableIterator<T>>();
        match iter.next() {
            None => Ret::Zero,
            Some(value) => Ret::Once(value),
        }
    }

    unsafe fn finish_call(fcinfo: pg_sys::FunctionCallInfo) {
        let fcx = deref_fcx(fcinfo);
        unsafe { srf_return_done(fcinfo, fcx) }
    }
}

fn fcx_needs_setup(fcinfo: pg_sys::FunctionCallInfo) -> bool {
    let need = unsafe { srf_is_first_call(fcinfo) };
    if need {
        unsafe { pg_sys::init_MultiFuncCall(fcinfo) };
    }
    need
}

pub(crate) fn empty_srf(fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
    unsafe {
        let fcx = deref_fcx(fcinfo);
        srf_return_done(fcinfo, fcx);
        pg_return_null(fcinfo)
    }
}

/// "per_MultiFuncCall" but no FFI cost
pub(crate) fn deref_fcx(fcinfo: pg_sys::FunctionCallInfo) -> *mut pg_sys::FuncCallContext {
    unsafe { (*(*fcinfo).flinfo).fn_extra.cast() }
}

pub(crate) fn srf_memcx(fcx: *mut pg_sys::FuncCallContext) -> PgMemoryContexts {
    unsafe { PgMemoryContexts::For((*fcx).multi_call_memory_ctx) }
}

fn finish_srf_init<T>(
    arg: Option<T>,
    fcinfo: pg_sys::FunctionCallInfo,
) -> ControlFlow<pg_sys::Datum, ()> {
    match arg {
        // nothing to iterate?
        None => ControlFlow::Break(empty_srf(fcinfo)),
        // must be saved for the next call by leaking it into the multi-call memory context
        Some(value) => {
            let fcx = deref_fcx(fcinfo);
            unsafe {
                let ptr = srf_memcx(fcx).leak_and_drop_on_delete(value);
                // it's the first call so we need to finish setting up fcx
                (*fcx).user_fctx = ptr.cast();
            }
            ControlFlow::Continue(())
        }
    }
}

unsafe impl<T> ReturnShipping for Option<T>
where
    T: ReturnShipping,
    T::CallRet: ReturnShipping,
{
    type CallRet = T::CallRet;

    unsafe fn prepare_call(fcinfo: pg_sys::FunctionCallInfo) -> CallCx {
        T::prepare_call(fcinfo)
    }

    fn label_ret(self) -> Ret<Self> {
        match self {
            None => Ret::Zero,
            Some(value) => match value.label_ret() {
                Ret::Many(iter, value) => Ret::Many(Some(iter), value),
                Ret::Once(value) => Ret::Once(value),
                Ret::Zero => Ret::Zero,
            },
        }
    }

    unsafe fn box_return(fcinfo: pg_sys::FunctionCallInfo, ret: Ret<Self>) -> pg_sys::Datum {
        let inner = match ret {
            Ret::Zero => Ret::Zero,
            Ret::Once(value) => Ret::Once(value),
            Ret::Many(Some(iter), value) => Ret::Many(iter, value),
            Ret::Many(None, _) => Ret::Zero,
        };

        T::box_return(fcinfo, inner)
    }

    unsafe fn into_context(self, fcinfo: pg_sys::FunctionCallInfo) {
        match self {
            None => (),
            Some(value) => value.into_context(fcinfo),
        }
    }

    unsafe fn ret_from_context(fcinfo: pg_sys::FunctionCallInfo) -> Ret<Self> {
        match T::ret_from_context(fcinfo) {
            Ret::Many(iter, value) => Ret::Many(Some(iter), value),
            Ret::Once(value) => Ret::Once(value),
            Ret::Zero => Ret::Zero,
        }
    }

    unsafe fn finish_call(fcinfo: pg_sys::FunctionCallInfo) {
        unsafe { T::finish_call(fcinfo) }
    }
}

unsafe impl<T, E> ReturnShipping for Result<T, E>
where
    T: ReturnShipping,
    T::CallRet: ReturnShipping,
    E: core::any::Any + core::fmt::Display,
{
    type CallRet = T::CallRet;

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

macro_rules! impl_boxret_for_primitives {
    ($($scalar:ty),*) => {
        $(
        unsafe impl RetPackage for $scalar {
            unsafe fn package_ret(self, fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
                $crate::pg_sys::Datum::from(self)
            }
        }
        )*
    }
}

impl_boxret_for_primitives! {
    i8, i16, i32, i64, bool
}

unsafe impl RetPackage for () {
    unsafe fn package_ret(self, fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
        pg_sys::Datum::from(0)
    }
}

unsafe impl RetPackage for f32 {
    unsafe fn package_ret(self, fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
        pg_sys::Datum::from(self.to_bits())
    }
}

unsafe impl RetPackage for f64 {
    unsafe fn package_ret(self, fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
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

macro_rules! impl_boxret_via_intodatum {
    ($($boxable:ty),*) => {
        $(
        unsafe impl RetPackage for $boxable {
            unsafe fn package_ret(self, fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
                repackage_into_datum(fcinfo, self)
            }
        })*
    };
}

impl_boxret_via_intodatum! {
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
