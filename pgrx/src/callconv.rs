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
use crate::iter::{SetOfIterator, TableIterator};
use crate::{
    pg_return_null, pg_sys, srf_is_first_call, srf_return_done, srf_return_next, IntoDatum,
    IntoHeapTuple, PgMemoryContexts,
};
use core::ops::ControlFlow;
use core::ptr;

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
