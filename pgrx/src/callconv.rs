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
use std::marker::PhantomData;
use std::mem::{self, ManuallyDrop};

use crate::iter::{SetOfIterator, TableIterator};
use crate::{
    nonstatic_typeid, pg_return_null, pg_sys, srf_first_call_init, srf_is_first_call,
    srf_per_call_setup, srf_return_done, srf_return_next, IntoDatum, IntoHeapTuple,
    PgMemoryContexts,
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

/// Boxing for return values
///
/// This bound is necessary to distinguish things which can be passed in/out of `#[pg_extern] fn`.
/// It is strictly a mistake to use IntoDatum or any derived traits for this bound!
/// PGRX allows complex return values which are nonsensical outside of function boundaries,
/// e.g. for implementing the value-per-call convention for set-returning functions.
pub trait BoxRet {
    /// The actual type returned from the call
    type CallRet: Sized;

    /// check the fcinfo state, initialize if necessary, and pick calling the wrapped fn or restoring Self
    fn prepare_call(fcinfo: pg_sys::FunctionCallInfo) -> CallCx {
        CallCx::WrappedFn(unsafe { pg_sys::CurrentMemoryContext })
    }

    /// answer what kind and how many returns happen from this type
    ///
    /// must be overridden if `Self != Self::CallRet`
    unsafe fn into_ret(self) -> Ret<Self>
    where
        Self: Sized;
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
    fn box_return(fcinfo: pg_sys::FunctionCallInfo, ret: Self::CallRet) -> pg_sys::Datum {
        todo!()
    }

    /// for multi-call types, how to restore them from the multi-call context, for all others: panic
    unsafe fn restore_from_context<'a>(fcinfo: pg_sys::FunctionCallInfo) -> &'a mut Self {
        unimplemented!()
    }
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

impl<'a, T> BoxRet for SetOfIterator<'a, T> {
    type CallRet = Option<<Self as Iterator>::Item>;
    fn prepare_call(fcinfo: pg_sys::FunctionCallInfo) -> CallCx {
        prepare_value_per_call_srf(fcinfo)
    }

    unsafe fn into_ret(self) -> Ret<Self>
    where
        Self: Sized,
    {
        let mut iter = self;
        let ret = iter.next();
        if ret.is_none() {
            Ret::Zero
        } else {
            Ret::Many(iter, ret)
        }
    }
}

pub enum Ret<T: BoxRet> {
    Zero,
    Once(T::CallRet),
    Many(T, T::CallRet),
}

// struct ValuePerCall?
// struct MaterializeTable?

impl<'a, T: IntoDatum> SetOfIterator<'a, T> {
    #[doc(hidden)]
    pub unsafe fn srf_next(
        fcinfo: pg_sys::FunctionCallInfo,
        wrapped_fn: impl FnOnce() -> Option<SetOfIterator<'a, T>>,
    ) -> pg_sys::Datum {
        if fcx_needs_setup(fcinfo) {
            let fcx = deref_fcx(fcinfo);
            // first off, ask the user's function to do the needful and return Option<SetOfIterator<T>>
            let setof_iterator = srf_memcx(fcx).switch_to(|_| wrapped_fn());
            if let ControlFlow::Break(datum) = finish_srf_init(setof_iterator, fcinfo) {
                return datum;
            }
        }

        let fcx = deref_fcx(fcinfo);
        // SAFETY: fcx.user_fctx was set earlier, immediately before or in a prior call
        let setof_iterator = &mut *(*fcx).user_fctx.cast::<SetOfIterator<T>>();

        match setof_iterator.next() {
            Some(datum) => {
                srf_return_next(fcinfo, fcx);
                datum.into_datum().unwrap_or_else(|| pg_return_null(fcinfo))
            }
            None => empty_srf(fcinfo),
        }
    }
}

impl<'a, T> BoxRet for TableIterator<'a, T> {
    type CallRet = Option<<Self as Iterator>::Item>;

    fn prepare_call(fcinfo: pg_sys::FunctionCallInfo) -> CallCx {
        prepare_value_per_call_srf(fcinfo)
    }

    unsafe fn into_ret(self) -> Ret<Self>
    where
        Self: Sized,
    {
        let mut iter = self;
        let ret = iter.next();
        if ret.is_none() {
            Ret::Zero
        } else {
            Ret::Many(iter, ret)
        }
    }
}

impl<'a, T: IntoHeapTuple> TableIterator<'a, T> {
    #[doc(hidden)]
    pub unsafe fn srf_next(
        fcinfo: pg_sys::FunctionCallInfo,
        wrapped_fn: impl FnOnce() -> Option<TableIterator<'a, T>>,
    ) -> pg_sys::Datum {
        if fcx_needs_setup(fcinfo) {
            let fcx = deref_fcx(fcinfo);
            let table_iterator = srf_memcx(fcx).switch_to(|_| {
                // first off, ask the user's function to do the needful and return Option<TableIterator<T>>
                let table_iterator = wrapped_fn();

                // Build a tuple descriptor for our result type
                let mut tupdesc = ptr::null_mut();
                let ty_class = pg_sys::get_call_result_type(fcinfo, ptr::null_mut(), &mut tupdesc);
                if ty_class != pg_sys::TypeFuncClass_TYPEFUNC_COMPOSITE {
                    pg_sys::error!("return type must be a row type");
                }
                pg_sys::BlessTupleDesc(tupdesc);
                (*fcx).tuple_desc = tupdesc;

                table_iterator
            });

            if let ControlFlow::Break(datum) = finish_srf_init(table_iterator, fcinfo) {
                return datum;
            }
        }

        let fcx = deref_fcx(fcinfo);
        // SAFETY: fcx.user_fctx was set earlier, immediately before or in a prior call
        let table_iterator = &mut *(*fcx).user_fctx.cast::<TableIterator<T>>();

        match table_iterator.next() {
            Some(tuple) => {
                let heap_tuple = tuple.into_heap_tuple((*fcx).tuple_desc);
                srf_return_next(fcinfo, fcx);
                pg_sys::HeapTupleHeaderGetDatum((*heap_tuple).t_data)
            }
            None => empty_srf(fcinfo),
        }
    }
}

fn fcx_needs_setup(fcinfo: pg_sys::FunctionCallInfo) -> bool {
    let need = unsafe { srf_is_first_call(fcinfo) };
    if need {
        unsafe { pg_sys::init_MultiFuncCall(fcinfo) };
    }
    need
}

fn empty_srf(fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
    unsafe {
        let fcx = deref_fcx(fcinfo);
        srf_return_done(fcinfo, fcx);
        pg_return_null(fcinfo)
    }
}

/// "per_MultiFuncCall" but no FFI cost
fn deref_fcx(fcinfo: pg_sys::FunctionCallInfo) -> *mut pg_sys::FuncCallContext {
    unsafe { (*(*fcinfo).flinfo).fn_extra.cast() }
}

fn srf_memcx(fcx: *mut pg_sys::FuncCallContext) -> PgMemoryContexts {
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
