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

impl<'a, T: IntoDatum> SetOfIterator<'a, T> {
    #[doc(hidden)]
    pub unsafe fn srf_next(
        fcinfo: pg_sys::FunctionCallInfo,
        wrapped_fn: impl FnOnce() -> Option<SetOfIterator<'a, T>>,
    ) -> pg_sys::Datum {
        if srf_is_first_call(fcinfo) {
            let funcctx = pg_sys::init_MultiFuncCall(fcinfo);

            // first off, ask the user's function to do the needful and return Option<SetOfIterator<T>>
            let setof_iterator =
                PgMemoryContexts::For((*funcctx).multi_call_memory_ctx).switch_to(|_| wrapped_fn());

            let setof_iterator = match setof_iterator {
                // user's function returned None, so there's nothing for us to later iterate
                None => {
                    srf_return_done(fcinfo, funcctx);
                    return pg_return_null(fcinfo);
                }

                // user's function returned Some(SetOfIterator), so we need to leak it into the
                // memory context Postgres has decided is to be used for multi-call SRF functions
                Some(iter) => PgMemoryContexts::For((*funcctx).multi_call_memory_ctx)
                    .leak_and_drop_on_delete(iter),
            };

            // it's the first call so we need to finish setting up `funcctx`
            (*funcctx).user_fctx = setof_iterator.cast();
        }

        let funcctx = pg_sys::per_MultiFuncCall(fcinfo);

        // SAFETY: we created `funcctx.user_fctx` on the first call into this function so
        // we know it's valid
        let setof_iterator = &mut *(*funcctx).user_fctx.cast::<SetOfIterator<T>>();

        match setof_iterator.next() {
            Some(datum) => {
                srf_return_next(fcinfo, funcctx);
                datum.into_datum().unwrap_or_else(|| pg_return_null(fcinfo))
            }
            None => {
                srf_return_done(fcinfo, funcctx);
                pg_return_null(fcinfo)
            }
        }
    }
}

impl<'a, T: IntoHeapTuple> TableIterator<'a, T> {
    #[doc(hidden)]
    pub unsafe fn srf_next(
        fcinfo: pg_sys::FunctionCallInfo,
        wrapped_fn: impl FnOnce() -> Option<TableIterator<'a, T>>,
    ) -> pg_sys::Datum {
        if srf_is_first_call(fcinfo) {
            let funcctx = pg_sys::init_MultiFuncCall(fcinfo);

            let table_iterator =
                PgMemoryContexts::For((*funcctx).multi_call_memory_ctx).switch_to(|_| {
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
                None => {
                    srf_return_done(fcinfo, funcctx);
                    return pg_return_null(fcinfo);
                }

                // user's function returned Some(TableIterator), so we need to leak it into the
                // memory context Postgres has decided is to be used for multi-call SRF functions
                Some(iter) => PgMemoryContexts::For((*funcctx).multi_call_memory_ctx)
                    .leak_and_drop_on_delete(iter),
            };

            // it's the first call so we need to finish setting up `funcctx`
            (*funcctx).user_fctx = table_iterator.cast();
        }

        let funcctx = pg_sys::per_MultiFuncCall(fcinfo);

        // SAFETY: we created `funcctx.user_fctx` on the first call into this function so
        // we know it's valid
        let table_iterator = &mut *(*funcctx).user_fctx.cast::<TableIterator<T>>();

        match table_iterator.next() {
            Some(tuple) => {
                let heap_tuple = tuple.into_heap_tuple((*funcctx).tuple_desc);
                srf_return_next(fcinfo, funcctx);
                pg_sys::HeapTupleHeaderGetDatum((*heap_tuple).t_data)
            }
            None => {
                srf_return_done(fcinfo, funcctx);
                pg_return_null(fcinfo)
            }
        }
    }
}
