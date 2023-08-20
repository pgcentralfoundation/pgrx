//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use crate::{
    direct_function_call, list::PgList, pg_sys, pg_sys::AsPgCStr, Array, FromDatum, IntoDatum,
};

pub trait FCallArg {
    fn as_datum(&self) -> Option<pg_sys::Datum>;
    fn type_oid(&self) -> pg_sys::Oid;
}

impl<T: IntoDatum + Clone> FCallArg for Option<T> {
    fn as_datum(&self) -> Option<pg_sys::Datum> {
        // TODO:  would prefer not to need `Clone`, but that requires changes to `IntoDatum`
        self.as_ref().map(|v| Clone::clone(v).into_datum()).flatten()
    }

    fn type_oid(&self) -> pg_sys::Oid {
        T::type_oid()
    }
}

// TODO:  This should return `Result<Option<T>, FCallError>`
//        We'll want to trap the common errors like ERRCODE_UNDEFINED_FUNCTION and ERRCODE_AMBIGUOUS_FUNCTION
//        along with making some of our own error types
pub fn fcall<T: FromDatum>(fname: &str, args: &[&dyn FCallArg]) -> Option<T> {
    let mut arg_types = Vec::with_capacity(args.len());
    let mut arg_datums = Vec::with_capacity(args.len());
    for (oid, datum) in args.iter().map(|a| (a.type_oid(), a.as_datum())) {
        arg_types.push(oid);
        arg_datums.push(datum);
    }
    unsafe {
        // let Postgres parse the function name -- it could be schema-qualified and Postgres knows
        // the parsing rules better than we do
        let ident_parts = direct_function_call::<Array<&str>>(
            pg_sys::parse_ident,
            &[fname.into_datum(), true.into_datum()],
        );

        // convert those into a PgList of each part as a `pg_sys::String`
        let func_oid = {
            // TODO:  `PgList` requires the "cshim" feature, which is **not** turned on for PL/Rust
            //         I think we'll need to fully port Postgres' "List" type to Rust...
            let mut parts_list = PgList::new();
            ident_parts
                .unwrap()
                .iter_deny_null()
                .map(|part| pg_sys::makeString(part.as_pg_cstr()))
                .for_each(|part| parts_list.push(part));

            // ask Postgres to find the function.  This will look for the possibly-qualified named
            // function following the normal SEARCH_PATH rules, ensuring its argument type Oids
            // exactly match the ones from the user's input arguments.  It does not evaluate the
            // return type, so we'll have to do that later
            let func_oid = pg_sys::LookupFuncName(
                parts_list.as_ptr(),
                args.len().try_into().unwrap(),
                arg_types.as_ptr(),
                false,
            );

            // free the individual `pg_sys::String` parts we allocated above
            parts_list.iter_ptr().for_each(|s| {
                #[cfg(any(
                    feature = "pg11",
                    feature = "pg12",
                    feature = "pg13",
                    feature = "pg14"
                ))]
                pg_sys::pfree((*s).val.str_.cast());

                #[cfg(any(feature = "pg15", feature = "pg16"))]
                pg_sys::pfree((*s).sval.cast());
            });

            func_oid
        };

        //
        // TODO:  lookup the function's pg_proc entry and do some validation around, at least,
        //  - STRICT: If the function is STRICT, any `None` argument Datum should return None right now
        //  - IN/OUT/TABLE arg types: I think we should only support IN in v1 of this
        //  - Make sure the return type `T` is compatible with the resolved function's return type
        //  - ??
        //

        //
        // The following code is Postgres-version specific.  Right now, it's compatible with v12+
        // v11 will need a different implementation.
        //
        // NB:  Which I don't want to do since it EOLs in 3 months
        //

        // initialize a stack-allocated `FmgrInfo` instance
        let mut flinfo = pg_sys::FmgrInfo::default();
        pg_sys::fmgr_info(func_oid, &mut flinfo);

        // heap allocate a `FunctionCallInfoBaseData` properly sized so there's enough room
        // for `args.len()` arguments
        let fcinfo = pg_sys::palloc0(
            std::mem::size_of::<pg_sys::FunctionCallInfoBaseData>()
                + std::mem::size_of::<pg_sys::NullableDatum>() * args.len(),
        ) as *mut pg_sys::FunctionCallInfoBaseData;

        // initialize it
        let fcinfo_ref = fcinfo.as_mut().unwrap();
        fcinfo_ref.flinfo = &mut flinfo;
        fcinfo_ref.fncollation = pg_sys::InvalidOid; // TODO:  We need to get a collation from somewhere?
        fcinfo_ref.context = std::ptr::null_mut();
        fcinfo_ref.resultinfo = std::ptr::null_mut();
        fcinfo_ref.isnull = false;
        fcinfo_ref.nargs = args.len().try_into().unwrap();

        // setup the argument array
        let args_slice = fcinfo_ref.args.as_mut_slice(args.len());
        for (i, datum) in arg_datums.into_iter().enumerate() {
            let arg = &mut args_slice[i];
            (arg.value, arg.isnull) =
                datum.map(|d| (d, false)).unwrap_or_else(|| (pg_sys::Datum::from(0), true));
        }

        // call the function
        // #define FunctionCallInvoke(fcinfo)	((* (fcinfo)->flinfo->fn_addr) (fcinfo))
        let func = (*fcinfo_ref.flinfo).fn_addr.as_ref().unwrap();
        let result = func(fcinfo);

        // Postgres' "OidFunctionCall" doesn't support returning null, but we can
        let result = T::from_datum(result, fcinfo_ref.isnull);

        // cleanup things we heap allocated
        pg_sys::pfree(fcinfo.cast());

        result
    }
}
