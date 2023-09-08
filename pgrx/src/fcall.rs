//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use once_cell::unsync::Lazy;
use pgrx_pg_sys::errcodes::PgSqlErrorCode;
use pgrx_pg_sys::PgTryBuilder;
use std::panic::AssertUnwindSafe;

use crate::pg_catalog::pg_proc::{PgProc, ProArgMode, ProKind};
use crate::{
    direct_function_call, is_a, list::PgList, pg_sys, pg_sys::AsPgCStr, Array, FromDatum, IntoDatum,
};

mod seal {
    pub trait Sealed {}

    impl<T: crate::IntoDatum + Clone> Sealed for T {}
}

pub unsafe trait FCallArg {
    fn as_datum(&self, pg_proc: &PgProc, argnum: usize) -> Result<Option<pg_sys::Datum>>;
    fn type_oid(&self) -> pg_sys::Oid;
}

pub enum Arg<T> {
    Null,
    Default,
    Value(T),
}

unsafe impl<T: IntoDatum + Clone + seal::Sealed> FCallArg for Arg<T> {
    fn as_datum(&self, pg_proc: &PgProc, argnum: usize) -> Result<Option<pg_sys::Datum>> {
        match self {
            Arg::Null => Ok(None),
            Arg::Value(v) => Ok(Clone::clone(v).into_datum()),
            Arg::Default => create_default_value(&pg_proc, argnum),
        }
    }

    fn type_oid(&self) -> pg_sys::Oid {
        T::type_oid()
    }
}

/// [`FCallError`]s represet the set of conditions that could case [`fcall()`] to fail in a
/// user-recoverable manner.
#[derive(thiserror::Error, Debug, Clone, Eq, PartialEq)]
pub enum FCallError {
    #[error("Invalid identifier: `{0}`")]
    InvalidIdentifier(String),

    #[error("The specified function does not exist")]
    UndefinedFunction,

    #[error("The specified function exists, but has overloaded versions which are ambiguous given the argument types provided")]
    AmbiguousFunction,

    #[error("Can only dymamically call plain functions")]
    UnsupportedFunctionType,

    #[error("Functions with OUT/IN_OUT/TABLE arguments are not supported")]
    UnsupportedArgumentModes,

    #[error("Functions with argument or return types of `internal` are not supported")]
    InternalTypeNotSupported,

    #[error("The requested return type `{0}` is not compatible with the actual return type `{1}`")]
    IncompatibleReturnType(pg_sys::Oid, pg_sys::Oid),

    #[error("Function call has more arguments than are supported")]
    TooManyArguments,

    #[error("Did not provide enough non-default arguments")]
    NotEnoughArguments,

    #[error("Function has no default arguments")]
    NoDefaultArguments,

    #[error("Argument #{0} does not have a DEFAULT value")]
    NotDefaultArgument(usize),

    #[error("Argument's default value is not a constant expression")]
    DefaultNotConstantExpression,
}

pub type Result<T> = std::result::Result<T, FCallError>;

pub fn fcall<T: FromDatum + IntoDatum>(fname: &str, args: &[&dyn FCallArg]) -> Result<Option<T>> {
    fcall_with_collation(fname, pg_sys::DEFAULT_COLLATION_OID, args)
}

pub fn fcall_with_collation<T: FromDatum + IntoDatum>(
    fname: &str,
    collation: pg_sys::Oid,
    args: &[&dyn FCallArg],
) -> Result<Option<T>> {
    // lookup the function by its name
    let func_oid = lookup_fn(fname, args)?;

    // lookup the function's pg_proc entry and do some validation
    let pg_proc = PgProc::new(func_oid).ok_or(FCallError::UndefinedFunction)?;
    let retoid = pg_proc.prorettype();

    //
    // do some validation to catch the cases we don't/can't directly call
    //

    if !matches!(pg_proc.prokind(), ProKind::Function) {
        // It only makes sense to directly call regular functions.  Calling aggregate or window
        // functions is nonsensical
        return Err(FCallError::UnsupportedFunctionType);
    } else if pg_proc.proargmodes().iter().any(|mode| *mode != ProArgMode::In) {
        // Right now we only know how to support arguments with the IN mode.  Perhaps in the
        // future we can support IN_OUT and TABLE return types
        return Err(FCallError::UnsupportedArgumentModes);
    } else if retoid == pg_sys::INTERNALOID
        || pg_proc.proargtypes().iter().any(|oid| *oid == pg_sys::INTERNALOID)
    {
        // No idea what to do with the INTERNAL type.  Generally it's just a raw pointer but pgrx
        // has no way to express that with `IntoDatum`.  And passing around raw pointers seem
        // unsafe enough that if someone needs to do that, they probably have the ability to
        // re-implement this function themselves.
        return Err(FCallError::InternalTypeNotSupported);
    } else if !T::is_compatible_with(retoid) {
        // the requested Oid of `T` is not compatible with the actual function return type
        return Err(FCallError::IncompatibleReturnType(T::type_oid(), retoid));
    }

    // we're likely going to be able to call the function, so convert our arguments into Datums,
    // filling in any DEFAULT arguments at the end
    let mut null = false;
    let arg_datums = args
        .iter()
        .enumerate()
        .map(|(i, a)| a.as_datum(&pg_proc, i))
        .chain((args.len()..pg_proc.pronargs()).map(|i| create_default_value(&pg_proc, i)))
        .map(|datum| {
            null |= matches!(datum, Ok(None));
            datum
        })
        .collect::<Result<Vec<_>>>()?;

    let nargs = arg_datums.len();
    assert_eq!(nargs, pg_proc.pronargs());

    // if the function is STRICT and at least one of our argument values is `None` (ie, NULL)...
    // we must return `None` now and not call the function.  Passing a NULL argument to a STRICT
    // function will likely crash Postgres
    let isstrict = pg_proc.proisstrict();
    if null && isstrict {
        return Ok(None);
    }

    //
    // The following code is Postgres-version specific.  Right now, it's compatible with v12+
    // v11 will need a different implementation.
    //
    // NB:  Which I don't want to do since it EOLs in November 2023
    //

    unsafe {
        // construct a stack-allocated `FmgrInfo` instance
        let mut flinfo = pg_sys::FmgrInfo::default();
        pg_sys::fmgr_info(func_oid, &mut flinfo);

        // heap allocate a `FunctionCallInfoBaseData` properly sized so there's enough room
        // for `args.len()` arguments
        let fcinfo = pg_sys::palloc0(
            std::mem::size_of::<pg_sys::FunctionCallInfoBaseData>()
                + std::mem::size_of::<pg_sys::NullableDatum>() * nargs,
        ) as *mut pg_sys::FunctionCallInfoBaseData;

        // initialize it
        let fcinfo_ref = fcinfo.as_mut().unwrap();
        fcinfo_ref.flinfo = &mut flinfo;
        fcinfo_ref.fncollation = collation;
        fcinfo_ref.context = std::ptr::null_mut();
        fcinfo_ref.resultinfo = std::ptr::null_mut();
        fcinfo_ref.isnull = false;
        fcinfo_ref.nargs = nargs as _;

        // setup the argument array
        let args_slice = fcinfo_ref.args.as_mut_slice(nargs);
        for (i, datum) in arg_datums.into_iter().enumerate() {
            assert!(!isstrict || (isstrict && datum.is_some())); // no NULL datums if this function is STRICT

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

        Ok(result)
    }
}

fn lookup_fn(fname: &str, args: &[&dyn FCallArg]) -> Result<pg_sys::Oid> {
    // ask Postgres to find the function.  This will look for the possibly-qualified named
    // function following the normal SEARCH_PATH rules, ensuring its argument type Oids
    // exactly match the ones from the user's input arguments.  It does not evaluate the
    // return type, so we'll have to do that later
    let mut parts_list = PgList::new();
    let result = PgTryBuilder::new(AssertUnwindSafe(|| unsafe {
        let arg_types = args.iter().map(|a| a.type_oid()).collect::<Vec<_>>();
        let nargs: i16 = arg_types.len().try_into().map_err(|_| FCallError::TooManyArguments)?;

        // parse the function name into its possibly-qualified name parts
        let ident_parts = parse_fn_name(&fname)?;
        ident_parts
            .iter_deny_null()
            .map(|part| pg_sys::makeString(part.as_pg_cstr()))
            .for_each(|part| parts_list.push(part));

        // look up an exact-match function based on the exact number of arguments we have
        let mut fnoid =
            pg_sys::LookupFuncName(parts_list.as_ptr(), nargs as _, arg_types.as_ptr(), true);

        if fnoid == pg_sys::InvalidOid {
            // if that didn't find a function, maybe we've got some defaults in there, so do a lookup
            // where Postgres will consider that
            fnoid = pg_sys::LookupFuncName(
                parts_list.as_ptr(),
                -1,
                arg_types.as_ptr(),
                false, // we want the ERROR here -- could be UNDEFINED_FUNCTION or AMBIGUOUS_FUNCTION
            );
        }

        Ok(fnoid)
    }))
    .catch_when(PgSqlErrorCode::ERRCODE_INVALID_PARAMETER_VALUE, |_| {
        Err(FCallError::InvalidIdentifier(fname.to_string()))
    })
    .catch_when(PgSqlErrorCode::ERRCODE_AMBIGUOUS_FUNCTION, |_| Err(FCallError::AmbiguousFunction))
    .catch_when(PgSqlErrorCode::ERRCODE_UNDEFINED_FUNCTION, |_| Err(FCallError::UndefinedFunction))
    .execute();

    unsafe {
        // free the individual `pg_sys::String` parts we allocated above
        parts_list.iter_ptr().for_each(|s| {
            #[cfg(any(feature = "pg11", feature = "pg12", feature = "pg13", feature = "pg14"))]
            pg_sys::pfree((*s).val.str_.cast());

            #[cfg(any(feature = "pg15", feature = "pg16"))]
            pg_sys::pfree((*s).sval.cast());
        });
    }

    result
}

fn parse_fn_name(fname: &str) -> Result<Array<&str>> {
    unsafe {
        direct_function_call::<Array<&str>>(
            pg_sys::parse_ident,
            &[fname.into_datum(), true.into_datum()],
        )
        .ok_or_else(|| FCallError::InvalidIdentifier(fname.to_string()))
    }
}

fn create_default_value(pg_proc: &PgProc, argnum: usize) -> Result<Option<pg_sys::Datum>> {
    let non_default_args_cnt = pg_proc.pronargs() - pg_proc.pronargdefaults();
    if argnum < non_default_args_cnt {
        return Err(FCallError::NotDefaultArgument(argnum));
    }
    let default_argnum = argnum - non_default_args_cnt;
    let default_value_tree = pg_proc.proargdefaults().ok_or(FCallError::NoDefaultArguments)?;

    let node = default_value_tree
        .get_ptr(default_argnum)
        .ok_or(FCallError::NotDefaultArgument(default_argnum))?;
    unsafe {
        let evaluated = pg_sys::eval_const_expressions(std::ptr::null_mut(), node);

        if is_a(evaluated, pg_sys::NodeTag_T_Const) {
            let con: *mut pg_sys::Const = evaluated.cast();
            let con_ref = &*con;

            if con_ref.constisnull {
                Ok(None)
            } else {
                Ok(Some(con_ref.constvalue))
            }
        } else {
            Err(FCallError::DefaultNotConstantExpression)
        }
    }
}
