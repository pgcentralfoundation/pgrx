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
use std::ops::ControlFlow;

use crate::iter::{SetOfIterator, TableIterator};
use crate::{
    pg_return_null, pg_sys, srf_is_first_call, srf_return_done, srf_return_next, IntoDatum,
    IntoHeapTuple, PgMemoryContexts,
};

impl<'a, T: IntoDatum> SetOfIterator<'a, T> {
    #[doc(hidden)]
    pub unsafe fn srf_next(
        fcinfo: pg_sys::FunctionCallInfo,
        wrapped_fn: impl FnOnce() -> Option<SetOfIterator<'a, T>>,
    ) -> pg_sys::Datum {
        if let ControlFlow::Continue(funcctx) = init_value_per_call_srf(fcinfo) {
            // first off, ask the user's function to do the needful and return Option<SetOfIterator<T>>
            let setof_iterator = srf_memcx(funcctx).switch_to(|_| wrapped_fn());

            let setof_iterator = match setof_iterator {
                // user's function returned None, so there's nothing for us to later iterate
                None => return empty_srf(fcinfo),
                // user's function returned Some(SetOfIterator), so we need to leak it into the
                // memory context Postgres has decided is to be used for multi-call SRF functions
                Some(iter) => srf_memcx(funcctx).leak_and_drop_on_delete(iter),
            };

            // it's the first call so we need to finish setting up `funcctx`
            (*funcctx).user_fctx = setof_iterator.cast();
        }

        let fcx = deref_fcx(fcinfo);

        // SAFETY: we created `fcx.user_fctx` on the first call into this function so
        // we know it's valid
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
        if let ControlFlow::Continue(funcctx) = init_value_per_call_srf(fcinfo) {
            let table_iterator = srf_memcx(funcctx).switch_to(|_| {
                // first off, ask the user's function to do the needful and return Option<TableIterator<T>>
                let table_iterator = wrapped_fn();

                // and if we're here, it worked, so carry on with the initial SRF setup dance

                // Build a tuple descriptor for our result type
                let mut tupdesc = std::ptr::null_mut();
                if pg_sys::get_call_result_type(fcinfo, std::ptr::null_mut(), &mut tupdesc)
                    != pg_sys::TypeFuncClass_TYPEFUNC_COMPOSITE
                {
                    pg_sys::error!("return type must be a row type");
                }
                pg_sys::BlessTupleDesc(tupdesc);
                (*funcctx).tuple_desc = tupdesc;

                table_iterator
            });

            let table_iterator = match table_iterator {
                // user's function returned None, so there's nothing for us to later iterate
                None => return empty_srf(fcinfo),

                // user's function returned Some(TableIterator), so we need to leak it into the
                // memory context Postgres has decided is to be used for multi-call SRF functions
                Some(iter) => srf_memcx(funcctx).leak_and_drop_on_delete(iter),
            };

            // it's the first call so we need to finish setting up `funcctx`
            (*funcctx).user_fctx = table_iterator.cast();
        }

        let fcx = deref_fcx(fcinfo);

        // SAFETY: we created `fcx.user_fctx` on the first call into this function so
        // we know it's valid
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

fn init_value_per_call_srf(
    fcinfo: pg_sys::FunctionCallInfo,
) -> ControlFlow<(), *mut pg_sys::FuncCallContext> {
    if unsafe { srf_is_first_call(fcinfo) } {
        let fcx = unsafe { pg_sys::init_MultiFuncCall(fcinfo) };
        ControlFlow::Continue(fcx)
    } else {
        ControlFlow::Break(())
    }
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
