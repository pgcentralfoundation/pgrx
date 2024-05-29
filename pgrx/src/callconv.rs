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
    PgMemoryContexts, PgVarlena, Time, TimeWithTimeZone, Timestamp, TimestampWithTimeZone, Uuid,
};
use core::marker::PhantomData;
use core::panic::{RefUnwindSafe, UnwindSafe};
use std::ffi::{CStr, CString};

type FcinfoData = pg_sys::FunctionCallInfoBaseData;
// FIXME: replace with a real implementation
pub struct Fcinfo<'a>(pub pg_sys::FunctionCallInfo, pub PhantomData<&'a mut FcinfoData>);
impl<'fcx> UnwindSafe for Fcinfo<'fcx> {}
impl<'fcx> RefUnwindSafe for Fcinfo<'fcx> {}

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

type FcInfoData = pg_sys::FunctionCallInfoBaseData;

#[derive(Clone)]
pub struct FcInfo<'fcx>(
    pgrx_pg_sys::FunctionCallInfo,
    std::marker::PhantomData<&'fcx mut FcInfoData>,
);

// when talking about this, there's the lifetime for setreturningfunction, and then there's the current context's lifetime.
// Potentially <'srf, 'curr, 'ret: 'curr + 'srf> -> <'ret> but don't start with that.
// at first <'curr> or <'fcx>
// It's a heap-allocated stack frame for your function.
// 'fcx would apply to the arguments into this whole thing.
// PgHeapTuple is super not ideal and this enables a replacement of that.
// ArgAbi and unbox from fcinfo index
// Just this and the accessors, something that goes from raw_args(&'a self) -> &'fcx [NullableDatum]? &'a [NullableDatum]?
impl<'fcx> FcInfo<'fcx> {
    /// Constructor, used to wrap a raw FunctionCallInfo provided by Postgres.
    ///
    /// # Safety
    ///
    /// This function is unsafe as we cannot ensure the `fcinfo` argument is a valid
    /// [`pg_sys::FunctionCallInfo`] pointer.  This is your responsibility.
    pub unsafe fn assume_valid(fcinfo: pg_sys::FunctionCallInfo) -> FcInfo<'fcx> {
        let _nullptr_check =
            std::ptr::NonNull::new(fcinfo).expect("fcinfo pointer must be non-null");
        Self(fcinfo, std::marker::PhantomData)
    }
    /// Retrieve the arguments to this function call as a slice of [`pgrx_pg_sys::NullableDatum`]
    #[inline]
    pub fn raw_args<'a>(&'a self) -> &'fcx [pgrx_pg_sys::NullableDatum]
    where
        'a: 'fcx,
    {
        // Null pointer check already performed on immutable pointer
        // at construction time.
        unsafe {
            let arg_len = (*self.0).nargs;
            let args_ptr: *const pg_sys::NullableDatum = std::ptr::addr_of!((*self.0).args).cast();
            // A valid FcInfoWrapper constructed from a valid FuntionCallInfo should always have
            // at least nargs elements of NullableDatum.
            std::slice::from_raw_parts(args_ptr, arg_len as _)
        }
    }

    /// Modifies the contained `fcinfo` struct to flag its return value as null.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use pgrx::pg_return_null;
    /// use pgrx::prelude::*;
    /// fn foo(fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
    ///     return unsafe { pg_return_null(fcinfo) };
    /// }
    /// ```
    #[inline]
    pub unsafe fn pg_return_null(&mut self) -> pg_sys::Datum {
        let fcinfo = unsafe { self.0.as_mut() }.unwrap();
        fcinfo.isnull = true;
        pg_sys::Datum::from(0)
    }

    /// Get the collation the function call should use
    #[inline]
    pub unsafe fn pg_get_collation(&self) -> pg_sys::Oid {
        let fcinfo = unsafe { self.0.as_mut() }.unwrap();
        fcinfo.fncollation
    }

    /// Retrieve the type (as an Oid) of argument number `num`.
    /// In other words, the type of `self.raw_args()[num]`
    #[inline]
    pub fn pg_getarg_type(&self, num: usize) -> pg_sys::Oid {
        // Safety: User must supply a valid fcinfo to assume_valid() in order
        // to construct a FcInfo. If that constraint is maintained, this should
        // be safe.
        unsafe {
            pg_sys::get_fn_expr_argtype(self.0.as_ref().unwrap().flinfo, num as std::os::raw::c_int)
        }
    }

    /// Retrieve the `.flinfo.fn_extra` pointer (as a PgBox'd type) from [`pg_sys::FunctionCallInfo`].
    pub fn pg_func_extra<ReturnType, DefaultValue: FnOnce() -> ReturnType>(
        &self,
        default: DefaultValue,
    ) -> PgBox<ReturnType> {
        // Safety: User must supply a valid fcinfo to assume_valid() in order
        // to construct a FcInfo. If that constraint is maintained, this should
        // be safe.
        unsafe {
            let fcinfo = PgBox::from_pg(self.0);
            let mut flinfo = PgBox::from_pg(fcinfo.flinfo);
            if flinfo.fn_extra.is_null() {
                flinfo.fn_extra = PgMemoryContexts::For(flinfo.fn_mcxt)
                    .leak_and_drop_on_delete(default())
                    as crate::void_mut_ptr;
            }

            PgBox::from_pg(flinfo.fn_extra as *mut ReturnType)
        }
    }

    #[inline]
    pub fn srf_is_first_call(&self) -> bool {
        // Safety: User must supply a valid fcinfo to assume_valid() in order
        // to construct a FcInfo. If that constraint is maintained, this should
        // be safe.
        unsafe { (*(*self.0).flinfo).fn_extra.is_null() }
    }

    /// Thin wrapper around [`pg_sys::init_MultiFuncCall`], made necessary
    /// because this structure's FunctionCallInfo is a private field.
    #[inline]
    pub unsafe fn init_multi_func_call(&mut self) -> FcContext<'fcx> {
        unsafe { FcContext::assume_valid(pg_sys::init_MultiFuncCall(self.0)) }
    }

    /// Thin wrapper around [`pg_sys::per_MultiFuncCall`], made necessary
    /// because this structure's FunctionCallInfo is a private field.
    #[inline]
    pub unsafe fn per_multi_func_call(&mut self) -> FcContext<'fcx> {
        unsafe { FcContext::assume_valid(pg_sys::per_MultiFuncCall(self.0)) }
    }
    /// Equivalent to "per_MultiFuncCall" with no FFI cost, and a lifetime
    /// constraint.
    ///
    /// Safety: This is only equivalent to the current (as of Postgres 16.3)
    /// implementation of [`pg_sys::per_MultiFuncCall`], and future changes
    /// to the complexity of the calling convention may break this method.
    pub(crate) unsafe fn deref_fcx(&mut self) -> FcContext<'fcx> {
        unsafe { FcContext::assume_valid((*(*self.0).flinfo).fn_extra.cast()) }
    }

    #[inline]
    pub unsafe fn srf_return_next(&mut self) {
        unsafe {
            let mut fncctx = self.deref_fcx();
            (*fncctx.get_inner()).call_cntr += 1;
            (*((*self.0).resultinfo as *mut pg_sys::ReturnSetInfo)).isDone =
                pg_sys::ExprDoneCond_ExprMultipleResult;
        }
    }

    #[inline]
    pub unsafe fn srf_return_done(&mut self) {
        unsafe {
            let mut fncctx = self.deref_fcx();
            pg_sys::end_MultiFuncCall(self.0, fncctx.get_inner());
            (*((*self.0).resultinfo as *mut pg_sys::ReturnSetInfo)).isDone =
                pg_sys::ExprDoneCond_ExprEndResult;
        }
    }
}

#[derive(Clone)]
pub struct FcContext<'fcx>(
    *mut pgrx_pg_sys::FuncCallContext,
    std::marker::PhantomData<&'fcx mut pgrx_pg_sys::FuncCallContext>,
);

impl<'fcx> FcContext<'fcx> {
    /// Constructor, used to wrap a raw FunctionCallInfo provided by Postgres.
    ///
    /// # Safety
    ///
    /// This function is unsafe as we cannot ensure the `ctx` argument is a valid
    /// [`pg_sys::FunctionCallInfo`] pointer.  This is your responsibility.
    pub(super) unsafe fn assume_valid(ctx: *mut pg_sys::FuncCallContext) -> FcContext<'fcx> {
        Self(ctx, std::marker::PhantomData)
    }
    pub fn get_inner(&mut self) -> *mut pgrx_pg_sys::FuncCallContext {
        self.0
    }
    pub fn srf_memcx(&mut self) -> PgMemoryContexts {
        unsafe { PgMemoryContexts::For((*self.0).multi_call_memory_ctx) }
    }
}
