#![doc(hidden)]
//! Helper implementations for returning sets and tables from `#[pg_extern]`-style functions
use crate::iter::{SetOfIterator, TableIterator};
use crate::{
    pg_return_null, pg_sys, srf_first_call_init, srf_is_first_call, srf_per_call_setup,
    srf_return_done, srf_return_next, IntoDatum, IntoHeapTuple, PgMemoryContexts,
};

impl<'a, T: IntoDatum> SetOfIterator<'a, T> {
    #[doc(hidden)]
    pub unsafe fn srf_next<F: FnOnce() -> Option<SetOfIterator<'a, T>>>(
        fcinfo: pg_sys::FunctionCallInfo,
        first_call_func: F,
    ) -> pg_sys::Datum {
        if srf_is_first_call(fcinfo) {
            let funcctx = srf_first_call_init(fcinfo);

            let (setof_iterator, memcxt) = PgMemoryContexts::For((*funcctx).multi_call_memory_ctx)
                .switch_to(|_| {
                    // first off, ask the user's function to do the needful and return Option<SetOfIterator<T>>
                    let setof_iterator = first_call_func();

                    //
                    // and if we're here, it worked, so carry on with the initial SRF setup dance
                    //

                    // allocate and return a Context for holding our SrfIterator which is used on every call
                    (setof_iterator, (*funcctx).multi_call_memory_ctx)
                });

            let setof_iterator = match setof_iterator {
                // user's function returned None, so there's nothing for us to later iterate
                None => {
                    srf_return_done(fcinfo, funcctx);
                    return pg_return_null(fcinfo);
                }

                // user's function returned Some(TableIterator), so we need to leak it into the
                // memory context Postgres has decided is to be used for multi-call SRF functions
                Some(iter) => PgMemoryContexts::For(memcxt).leak_and_drop_on_delete(iter),
            };

            // it's the first call so we need to finish setting up `funcctx`
            (*funcctx).user_fctx = setof_iterator.cast();
        }

        let funcctx = srf_per_call_setup(fcinfo);

        // SAFETY: we created `funcctx.user_fctx` on the first call into this function so
        // we know it's valid
        let setof_iterator =
            (*funcctx).user_fctx.cast::<SetOfIterator<T>>().as_mut().unwrap_unchecked();

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
    pub unsafe fn srf_next<F: FnOnce() -> Option<TableIterator<'a, T>>>(
        fcinfo: pg_sys::FunctionCallInfo,
        first_call_func: F,
    ) -> pg_sys::Datum {
        if srf_is_first_call(fcinfo) {
            let mut funcctx = srf_first_call_init(fcinfo);

            let (table_iterator, tupdesc, memcxt) =
                PgMemoryContexts::For((*funcctx).multi_call_memory_ctx).switch_to(|_| {
                    // first off, ask the user's function to do the needful and return Option<TableIterator<T>>
                    let table_iterator = first_call_func();

                    //
                    // and if we're here, it worked, so carry on with the initial SRF setup dance
                    //

                    // Build a tuple descriptor for our result type
                    let mut tupdesc = std::ptr::null_mut();
                    if pg_sys::get_call_result_type(fcinfo, std::ptr::null_mut(), &mut tupdesc)
                        != pg_sys::TypeFuncClass_TYPEFUNC_COMPOSITE
                    {
                        pg_sys::error!("return type must be a row type");
                    }
                    pg_sys::BlessTupleDesc(tupdesc);

                    // allocate and return a Context for holding our SrfIterator which is used on every call
                    (table_iterator, tupdesc, (*funcctx).multi_call_memory_ctx)
                });

            let table_iterator = match table_iterator {
                // user's function returned None, so there's nothing for us to later iterate
                None => {
                    srf_return_done(fcinfo, funcctx);
                    return pg_return_null(fcinfo);
                }

                // user's function returned Some(TableIterator), so we need to leak it into the
                // memory context Postgres has decided is to be used for multi-call SRF functions
                Some(iter) => PgMemoryContexts::For(memcxt).leak_and_drop_on_delete(iter),
            };

            // it's the first call so we need to finish setting up `funcctx`
            (*funcctx).tuple_desc = tupdesc;
            (*funcctx).user_fctx = table_iterator.cast();
        }

        let funcctx = srf_per_call_setup(fcinfo);

        // SAFETY: we created `funcctx.user_fctx` on the first call into this function so
        // we know it's valid
        let table_iterator =
            (*funcctx).user_fctx.cast::<TableIterator<T>>().as_mut().unwrap_unchecked();

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
