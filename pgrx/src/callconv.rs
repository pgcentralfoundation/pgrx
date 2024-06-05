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
use std::ptr::NonNull;

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
    #[inline]
    pub unsafe fn from_ptr(fcinfo: pg_sys::FunctionCallInfo) -> FcInfo<'fcx> {
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

    /// Retrieves the internal [`pg_sys::FunctionCallInfo`] wrapped by this type.
    /// A FunctionCallInfo is a mutable pointer to a FunctionCallInfoBaseData already, and so
    /// that type is sufficient.
    #[inline]
    pub unsafe fn as_mut_ptr(&self) -> pg_sys::FunctionCallInfo {
        self.0
    }

    /// Modifies the contained `fcinfo` struct to flag its return value as null.
    ///
    /// Returns a null-pointer Datum for use with the calling function's return value.
    ///
    /// Safety: If this flag is set, regardless of what your function actually returns,
    /// Postgress will presume it is null and discard it. This means that, if you call this
    /// method and then return a value anyway, *your extension will leak memory.* Please
    /// ensure this method is only called for functions which, very definitely, returns null.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use pgrx::callconv::FcInfo;
    /// use pgrx::datum::Datum;
    /// use pgrx::prelude::*;
    /// fn foo<'a>(mut fcinfo: FcInfo<'a>) -> Datum {
    ///     return unsafe { fcinfo.set_return_null() };
    /// }
    /// ```
    #[inline]
    pub unsafe fn set_return_null(&mut self) -> crate::datum::Datum<'fcx> {
        let fcinfo = unsafe { self.0.as_mut() }.unwrap();
        fcinfo.isnull = true;
        crate::datum::Datum::null()
    }

    /// Get the collation the function call should use.
    /// If the OID is 0 (invalid or no-type) this method will return None,
    /// otherwise it will return Some(oid).
    #[inline]
    pub fn get_collation(&self) -> Option<pg_sys::Oid> {
        // SAFETY: see FcInfo::from_ptr
        let fcinfo = unsafe { self.0.as_mut() }.unwrap();
        (fcinfo.fncollation.as_u32() != 0).then_some(fcinfo.fncollation)
    }

    /// Retrieve the type (as an Oid) of argument number `num`.
    /// In other words, the type of `self.raw_args()[num]`
    #[inline]
    pub fn get_arg_type(&self, num: usize) -> Option<pg_sys::Oid> {
        // SAFETY: see FcInfo::from_ptr
        unsafe {
            // bool::then() is lazy-evaluated, then_some is not.
            (num < ((*self.0).nargs as usize)).then(
                #[inline]
                || {
                    pg_sys::get_fn_expr_argtype(
                        self.0.as_ref().unwrap().flinfo,
                        num as std::os::raw::c_int,
                    )
                },
            )
        }
    }

    /// Retrieve the `.flinfo.fn_extra` pointer (as a PgBox'd type) from [`pg_sys::FunctionCallInfo`].
    /// If it was not already initialized, initialize it with `default`
    pub fn get_or_init_func_extra<DefaultValue: FnOnce() -> *mut pg_sys::FuncCallContext>(
        &self,
        default: DefaultValue,
    ) -> NonNull<pg_sys::FuncCallContext> {
        // Safety: User must supply a valid fcinfo to from_ptr() in order
        // to construct a FcInfo. If that constraint is maintained, this should
        // be safe.
        unsafe {
            let mut flinfo = NonNull::new((*self.0).flinfo).unwrap();
            if flinfo.as_ref().fn_extra.is_null() {
                flinfo.as_mut().fn_extra = PgMemoryContexts::For(flinfo.as_ref().fn_mcxt)
                    .leak_and_drop_on_delete(default())
                    as crate::void_mut_ptr;
            }

            // Safety: can new_unchecked() here because we just initialized it.
            NonNull::new_unchecked(flinfo.as_mut().fn_extra as *mut pg_sys::FuncCallContext)
        }
    }

    #[inline]
    pub fn srf_is_initialized(&self) -> bool {
        // Safety: User must supply a valid fcinfo to from_ptr() in order
        // to construct a FcInfo. If that constraint is maintained, this should
        // be safe.
        unsafe { !(*(*self.0).flinfo).fn_extra.is_null() }
    }

    /// Thin wrapper around [`pg_sys::init_MultiFuncCall`], made necessary
    /// because this structure's FunctionCallInfo is a private field.
    ///
    /// This should initialize `self.0.flinfo.fn_extra`
    #[inline]
    pub unsafe fn init_multi_func_call(&mut self) -> &'fcx mut pg_sys::FuncCallContext {
        unsafe {
            let fcx: *mut pg_sys::FuncCallContext = pg_sys::init_MultiFuncCall(self.0);
            debug_assert!(fcx.is_null() == false);
            &mut *fcx
        }
    }

    /// Equivalent to "per_MultiFuncCall" with no FFI cost, and a lifetime
    /// constraint.
    ///
    /// Safety: Assumes `self.0.flinfo.fn_extra` is non-null
    /// i.e. [`FcInfo::srf_is_initialized()`] would be `true`.
    #[inline]
    pub(crate) unsafe fn deref_fcx(&mut self) -> &'fcx mut pg_sys::FuncCallContext {
        unsafe {
            let fcx: *mut pg_sys::FuncCallContext = (*(*self.0).flinfo).fn_extra.cast();
            debug_assert!(fcx.is_null() == false);
            &mut *fcx
        }
    }

    /// Safety: Assumes `self.0.flinfo.fn_extra` is non-null
    /// i.e. [`FcInfo::srf_is_initialized()`] would be `true`.
    #[inline]
    pub unsafe fn srf_return_next(&mut self) {
        unsafe {
            self.deref_fcx().call_cntr += 1;
            self.get_result_info().set_is_done(pg_sys::ExprDoneCond_ExprMultipleResult);
        }
    }

    /// Safety: Assumes `self.0.flinfo.fn_extra` is non-null
    /// i.e. [`FcInfo::srf_is_initialized()`] would be `true`.
    #[inline]
    pub unsafe fn srf_return_done(&mut self) {
        unsafe {
            pg_sys::end_MultiFuncCall(self.0, self.deref_fcx());
            self.get_result_info().set_is_done(pg_sys::ExprDoneCond_ExprEndResult);
        }
    }

    /// # Safety
    /// Do not corrupt the `pg_sys::ReturnSetInfo` struct's data.
    #[inline]
    pub unsafe fn get_result_info(&self) -> ReturnSetInfoWrapper<'fcx> {
        unsafe {
            ReturnSetInfoWrapper::from_ptr((*self.0).resultinfo as *mut pg_sys::ReturnSetInfo)
        }
    }
}

#[derive(Clone)]
pub struct ReturnSetInfoWrapper<'fcx>(
    *mut pg_sys::ReturnSetInfo,
    std::marker::PhantomData<&'fcx mut pg_sys::ReturnSetInfo>,
);

impl<'fcx> ReturnSetInfoWrapper<'fcx> {
    /// Constructor, used to wrap a ReturnSetInfo provided by Postgres.
    ///
    /// # Safety
    ///
    /// This function is unsafe as we cannot ensure the `retinfo` argument is a valid
    /// [`pg_sys::ReturnSetInfo`] pointer.  This is your responsibility.
    #[inline]
    pub unsafe fn from_ptr(retinfo: *mut pg_sys::ReturnSetInfo) -> ReturnSetInfoWrapper<'fcx> {
        let _nullptr_check =
            std::ptr::NonNull::new(retinfo).expect("fcinfo pointer must be non-null");
        Self(retinfo, std::marker::PhantomData)
    }
    /*
    /* result status from function (but pre-initialized by caller): */
    SetFunctionReturnMode returnMode;	/* actual return mode */
    ExprDoneCond isDone;		/* status for ValuePerCall mode */
    /* fields filled by function in Materialize return mode: */
    Tuplestorestate *setResult; /* holds the complete returned tuple set */
    TupleDesc	setDesc;		/* actual descriptor for returned tuples */
    */
    // These four fields are, in-practice, owned by the callee.
    /// Status for ValuePerCall mode.
    pub fn set_is_done(&mut self, value: pg_sys::ExprDoneCond) {
        unsafe {
            (*self.0).isDone = value;
        }
    }
    /// Status for ValuePerCall mode.
    pub fn get_is_done(&self) -> pg_sys::ExprDoneCond {
        unsafe { (*self.0).isDone }
    }
    /// Actual return mode.
    pub fn set_return_mode(&mut self, return_mode: pgrx_pg_sys::SetFunctionReturnMode) {
        unsafe {
            (*self.0).returnMode = return_mode;
        }
    }
    /// Actual return mode.
    pub fn get_return_mode(&self) -> pgrx_pg_sys::SetFunctionReturnMode {
        unsafe { (*self.0).returnMode }
    }
    /// Holds the complete returned tuple set.
    pub fn set_tuple_result(&mut self, set_result: *mut pgrx_pg_sys::Tuplestorestate) {
        unsafe {
            (*self.0).setResult = set_result;
        }
    }
    /// Holds the complete returned tuple set.
    ///
    /// Safety: There is no guarantee this has been initialized.
    /// This may be a null pointer.
    pub fn get_tuple_result(&self) -> *mut pgrx_pg_sys::Tuplestorestate {
        unsafe { (*self.0).setResult }
    }

    /// Actual descriptor for returned tuples.
    pub fn set_tuple_desc(&mut self, desc: *mut pgrx_pg_sys::TupleDescData) {
        unsafe {
            (*self.0).setDesc = desc;
        }
    }

    /// Actual descriptor for returned tuples.
    pub fn get_tuple_desc(&mut self) -> *mut pgrx_pg_sys::TupleDescData {
        unsafe { (*self.0).setDesc }
    }
}
