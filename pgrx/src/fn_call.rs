//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use pgrx_pg_sys::PgTryBuilder;
use pgrx_pg_sys::errcodes::PgSqlErrorCode;
use pgrx_pg_sys::ffi::pg_guard_ffi_boundary;
use std::ffi::c_int;
use std::panic::AssertUnwindSafe;

use crate::memcx;
use crate::pg_catalog::pg_proc::{PgProc, ProArgMode, ProKind};
use crate::seal::Sealed;
use crate::{
    Array, FromDatum, IntoDatum, direct_function_call, list::List, pg_sys, pg_sys::AsPgCStr,
};

/// Augments types that can be used as [`fn_call`] arguments.  This is only implemented for the
/// [`Arg`] enum.
pub unsafe trait FnCallArg: Sealed {
    /// Represent `&self` as the [`pg_sys::Datum`] for argument `argnum` of `pg_proc`, or `None`
    /// for a NULL.  An [`Arg::Value`] converts a clone of its value, [`Arg::Null`] returns
    /// `None`, and [`Arg::Default`] evaluates the parameter's `DEFAULT`.  This does not check
    /// the value's type against the parameter.
    ///
    /// [`fn_call`] calls this only for an [`Arg::Value`], after it checks the value's type.  It
    /// passes NULLs and `DEFAULT`s without calling this.
    fn as_datum(&self, pg_proc: &PgProc, argnum: usize) -> Result<Option<pg_sys::Datum>>;

    /// The Postgres type OID of the `T` in `Arg<T>`, whichever variant `&self` is.  [`fn_call`]
    /// selects the function by these types.
    fn type_oid(&self) -> pg_sys::Oid;

    /// `&self` with the value of an [`Arg::Value`] replaced by its Postgres type OID
    fn value_type(&self) -> Arg<pg_sys::Oid>;
}

/// The kinds of [`fn_call`] arguments.  
pub enum Arg<T> {
    /// The argument value is a SQL NULL
    Null,

    /// The argument's `DEFAULT` value should be used
    Default,

    /// Use this actual value
    Value(T),
}

impl<T> Sealed for Arg<T> {}

unsafe impl<T: IntoDatum + Clone> FnCallArg for Arg<T> {
    fn as_datum(&self, pg_proc: &PgProc, argnum: usize) -> Result<Option<pg_sys::Datum>> {
        match self {
            Arg::Null => Ok(None),
            Arg::Value(v) => Ok(Clone::clone(v).into_datum()),
            Arg::Default => Ok(create_default_value(pg_proc, argnum)?.map(|(datum, _)| datum)),
        }
    }

    #[inline]
    fn type_oid(&self) -> pg_sys::Oid {
        T::type_oid()
    }

    #[inline]
    fn value_type(&self) -> Arg<pg_sys::Oid> {
        match self {
            Arg::Null => Arg::Null,
            Arg::Default => Arg::Default,
            Arg::Value(_) => Arg::Value(T::type_oid()),
        }
    }
}

/// [`FnCallError`]s represent the set of conditions that could case [`fn_call()`] to fail in a
/// user-recoverable manner.
#[derive(thiserror::Error, Debug, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum FnCallError {
    #[error("Invalid identifier: `{0}`")]
    InvalidIdentifier(String),

    #[error("The specified function does not exist")]
    UndefinedFunction,

    #[error(
        "The specified function exists, but has overloaded versions which are ambiguous given the argument types provided"
    )]
    AmbiguousFunction,

    #[error("Can only dynamically call plain functions")]
    UnsupportedFunctionType,

    #[error("Functions with OUT/IN_OUT/TABLE/VARIADIC arguments are not supported")]
    UnsupportedArgumentModes,

    #[error("Functions with argument or return types of `internal` are not supported")]
    InternalTypeNotSupported,

    #[error(
        "The requested return type OID `{0:?}` is not compatible with the actual return type OID `{1:?}`"
    )]
    IncompatibleReturnType(pg_sys::Oid, pg_sys::Oid),

    /// The zero-based argument number, the type OID of the value passed for it, and the type OID
    /// of its parameter.  A value for a polymorphic parameter can match that type and still conflict
    /// with the other values, as the "Argument types" section of [`fn_call`] describes.
    #[error(
        "Argument #{0} has type OID `{1:?}`, which is not compatible with the parameter type OID `{2:?}`"
    )]
    IncompatibleArgumentType(usize, pg_sys::Oid, pg_sys::Oid),

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

pub type Result<T> = std::result::Result<T, FnCallError>;

/// Dynamically call a named function in the current database.  The function must be one previously
/// defined by `CREATE FUNCTION`.  Its underlying `LANGUAGE` is irrelevant -- call `LANGUAGE sql`,
/// `LANGUAGE plpgsql`, `LANGUAGE plperl`, or (our favorite) `LANGUAGE plrust` functions.
///
/// The function name itself, `fname`, is an optionally schema-qualified function name identifier.
/// If `fname` is not schema qualified, then the standard Postgres rules for searching the active
/// `SEARCH_PATH` are followed.
///
/// When resolving a function by name, its argument count and types must also be considered.  These
/// are determined by the types applied to the [`Arg`] variants of each provided argument.  To
/// help avoid ambiguities during function resolution, even the [`Arg::Default`] and [`Arg::Null`]
/// variants are typed -- [`Arg`] itself is generic.
///
/// If the called function is declared `STRICT` and at least one of the `args` are the [`Arg::Null`]
/// variant, then the function is **not** actually called, and an `Ok(None)` is returned.
///
/// Arguments with `DEFAULT` clauses always appear as the last function arguments.  If you want the
/// default value as defined by the function, either use [`Arg::Default`] or elide the default
/// arguments entirely -- [`fn_call`] will automatically fill them in.
///
/// # Argument types
///
/// [`fn_call`] passes each value to the function as it is, without the casts that PostgreSQL
/// would add.  Before it converts any [`Arg::Value`], it checks the type of each value against the
/// parameter's type.  The type of an [`Arg::Value`] is `T`'s [`IntoDatum::type_oid`], and the type
/// of a `DEFAULT`, from [`Arg::Default`] or an omitted argument, is the one PostgreSQL stored it
/// with:
///
/// - The value matches a parameter of the same type, or of a type it is binary coercible to, such
///   as `varchar` to `text` or a domain to its base type.  It does not match a parameter that
///   needs a cast, such as an `i32` for a `bigint` parameter or an `int4` for a domain over `int4`.
/// - Values for polymorphic parameters must be consistent as PostgreSQL defines it.  The values
///   for the `anycompatible` family must have the same type, or element or range type, instead of
///   one that PostgreSQL would cast to a common type.
/// - PostgreSQL stores the default of a polymorphic parameter with its own type, such as `int4` for
///   `anycompatible DEFAULT 1`, so it must be consistent with the other values too.  A default
///   written as an untyped literal, such as `anyelement DEFAULT 'x'`, has type `unknown` and
///   matches no polymorphic parameter.
/// - A value of a polymorphic type, such as an [`AnyArray`](crate::AnyArray), matches a
///   polymorphic parameter only of the same type, and only when no other value is passed to a
///   polymorphic parameter.
/// - A parameter of type `"any"` accepts a value of any type.  The function cannot learn that type,
///   because [`fn_call`] passes no call expression.  PostgreSQL's own `"any"` functions then raise
///   an ERROR, return NULL, or return a meaningless result: `pg_column_size` raises an ERROR,
///   `pg_collation_for` returns NULL, and `pg_typeof` returns `InvalidOid`.
/// - A value whose type is `InvalidOid`, as for a Rust tuple, or an OID that names no type matches
///   no parameter.
///
/// A mismatch returns [`FnCallError::IncompatibleArgumentType`].  [`Arg::Null`] and a `DEFAULT`
/// that is NULL pass no value, so their types are not checked.  The `T` of [`Arg::Null`] and
/// [`Arg::Default`] only selects the function.
///
/// # Returns
///
/// [`fn_call`] returns an instantiated Rust type of the return value from the called function.  If
/// that function returns NULL, [`fn_call`] returns `Ok(None)`.
///
/// # Errors
///
/// Any of the [`FnCallError`] variants could be returned.
///
/// # Panics
///
/// [`fn_call`] itself should not panic, but it may raise assertion panics if unexpected conditions are
/// detected.  These would indicate a bug and should be reported.
///
/// Note that if the function being called raises a Postgres ERROR, it is not caught by [`fn_call`]
/// and will immediately abort the transaction.  To catch such errors yourself, use [`fn_call`] with
/// [`PgTryBuilder`].
///
/// # Limitations
///
/// [`fn_call`] calls only plain functions whose parameters are all `IN` parameters.  It returns
/// [`FnCallError::UnsupportedFunctionType`] for a procedure, aggregate, or window function, and
/// [`FnCallError::UnsupportedArgumentModes`] for a function with an `OUT`, `INOUT`, or `VARIADIC`
/// parameter, which includes every function that `RETURNS TABLE(...)`.
///
/// It does not support other set-returning functions, those that `RETURNS SETOF`, but it does not
/// check for them either.  It calls one without the `ReturnSetInfo` that a set-returning function
/// needs, and PostgreSQL's and pgrx's set-returning functions then raise the ERROR "set-valued
/// function called in context that cannot accept a set".  [`fn_call`] does not catch that ERROR,
/// as described above, so it does not return an [`FnCallError`].
///
/// # Examples
///
/// ## Calling a UDF
///
/// ```sql
/// CREATE FUNCTION sum_array(a int4[]) RETURNS int4 LANGUAGE sql AS $$ SELECT sum(value) FROM (SELECT unnest($1) value) x $$;
/// ```
///
/// ```rust,no_run
/// use pgrx::fn_call::{Arg, fn_call};
///
/// let array = vec![1,2,3];
/// let sum = fn_call::<i32>("sum_array", &[&Arg::Value(array)]);
/// assert_eq!(sum, Ok(Some(6)));
/// ```
///
/// ## Calling a built-in
///
/// ```rust,no_run
/// use pgrx::fn_call::{Arg, fn_call};
/// let is_eq = fn_call::<bool>("texteq", &[&Arg::Value("hello"), &Arg::Value("world")]);
/// assert!(is_eq == Ok(Some(false)));
/// ```
///
/// ## Using DEFAULT values
///
/// ```sql
/// CREATE FUNCTION mul_by(input bigint, factor bigint DEFAULT 2) RETURNS bigint AS $$ SELECT input * factor $$;
/// ```
///
/// ```rust,no_run
/// use pgrx::fn_call::{Arg, fn_call};
///
/// let product = fn_call::<i64>("mul_by", &[&Arg::Value(42_i64)]);  // uses the default of `2` for `factor`
/// assert_eq!(product, Ok(Some(84)));
///
/// let product = fn_call::<i64>("mul_by", &[&Arg::Value(42_i64), &Arg::<i64>::Default]);  // uses the default of `2` for `factor`
/// assert_eq!(product, Ok(Some(84)));
///
/// let product = fn_call::<i64>("mul_by", &[&Arg::Value(42_i64), &Arg::Value(3_i64)]);  // specifies an explicit value for `factor`
/// assert_eq!(product, Ok(Some(126)));
///
/// ```
pub fn fn_call<R: FromDatum + IntoDatum>(
    fname: &str,
    args: &[&dyn FnCallArg],
) -> Result<Option<R>> {
    fn_call_with_collation(fname, pg_sys::DEFAULT_COLLATION_OID, args)
}

pub fn fn_call_with_collation<R: FromDatum + IntoDatum>(
    fname: &str,
    collation: pg_sys::Oid,
    args: &[&dyn FnCallArg],
) -> Result<Option<R>> {
    // lookup the function by its name
    let func_oid = lookup_fn(fname, args)?;

    // lookup the function's pg_proc entry and do some validation
    let pg_proc = PgProc::new(func_oid).ok_or(FnCallError::UndefinedFunction)?;
    let retoid = pg_proc.prorettype();
    let argtypes = pg_proc.proargtypes();

    //
    // do some validation to catch the cases we don't/can't directly call
    //

    if !matches!(pg_proc.prokind(), ProKind::Function) {
        // It only makes sense to directly call regular functions.  Calling aggregate or window
        // functions is nonsensical
        return Err(FnCallError::UnsupportedFunctionType);
    } else if pg_proc.proargmodes().iter().any(|mode| *mode != ProArgMode::In) {
        // Right now we only know how to support arguments with the IN mode.  Perhaps in the
        // future we can support IN_OUT and TABLE return types
        return Err(FnCallError::UnsupportedArgumentModes);
    } else if retoid == pg_sys::INTERNALOID || argtypes.contains(&pg_sys::INTERNALOID) {
        // No idea what to do with the INTERNAL type.  Generally it's just a raw pointer but pgrx
        // has no way to express that with `IntoDatum`.  And passing around raw pointers seem
        // unsafe enough that if someone needs to do that, they probably have the ability to
        // re-implement this function themselves.
        return Err(FnCallError::InternalTypeNotSupported);
    } else if !R::is_compatible_with(retoid) {
        // the requested Oid of `T` is not compatible with the actual function return type
        return Err(FnCallError::IncompatibleReturnType(R::type_oid(), retoid));
    }

    if args.len() > argtypes.len() {
        return Err(FnCallError::TooManyArguments);
    }

    // fill in the DEFAULT arguments, including any at the end
    let passed = (0..argtypes.len())
        .map(|argnum| match args.get(argnum).map(|&arg| (arg, arg.value_type())) {
            Some((arg, Arg::Value(oid))) => Ok(PassedArg::Value(arg, oid)),
            Some((_, Arg::Null)) => Ok(PassedArg::Datum(None)),
            Some((_, Arg::Default)) | None => {
                create_default_value(&pg_proc, argnum).map(PassedArg::Datum)
            }
        })
        .collect::<Result<Vec<_>>>()?;

    // the function reads each argument as its parameter's type, so check that before converting
    // any Arg::Value
    check_argument_types(&argtypes, &passed)?;

    // we're likely going to be able to call the function, so convert our arguments into Datums
    let mut null = false;
    let arg_datums = passed
        .into_iter()
        .enumerate()
        .map(|(argnum, arg)| match arg {
            PassedArg::Value(value, _) => value.as_datum(&pg_proc, argnum),
            PassedArg::Datum(value) => Ok(value.map(|(datum, _)| datum)),
        })
        .map(|datum| {
            null |= matches!(datum, Ok(None));
            datum
        })
        .collect::<Result<Vec<_>>>()?;
    let nargs = arg_datums.len();

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

        // SAFETY:  we just allocated `flinfo`.  Whatever objects `fmgr_info` may allocate
        // are allocated in `CurrentMemoryContext`, which is fine as this `flinfo` doesn't live longer
        // than this stack frame anyway.
        pg_sys::fmgr_info(func_oid, &mut flinfo);

        // heap allocate a `FunctionCallInfoBaseData` properly sized so there's enough room for `nargs` arguments
        //
        // SAFETY: we allocate enough zeroed space for the base FunctionCallInfoBaseData *plus* the number of arguments
        // we have, and we've asserted that we have the correct number of arguments
        assert_eq!(nargs, pg_proc.pronargs());
        let nargs_c = i16::try_from(nargs).expect("function argument count fits PostgreSQL nargs");
        let fcinfo = pg_sys::palloc0(pg_sys::SizeForFunctionCallInfo!(nargs_c).get() as usize)
            .cast::<pg_sys::FunctionCallInfoBaseData>();

        // initialize it
        // SAFETY: we just palloc'd the `fcinfo` instance so it's de-referencable
        pg_sys::InitFunctionCallInfoData!(
            *fcinfo,
            &raw mut flinfo,
            nargs_c,
            collation,
            std::ptr::null_mut::<pg_sys::Node>(),
            std::ptr::null_mut::<pg_sys::Node>()
        );

        // setup the argument array
        // SAFETY:  `fcinfo.args` is the over-allocated space we palloc0'd above.  it's an array
        // of `nargs` `NullableDatum` instances.
        {
            let args_slice = (*fcinfo).args.as_mut_slice(nargs);
            for (i, datum) in arg_datums.into_iter().enumerate() {
                assert!(!isstrict || datum.is_some()); // no NULL datums if this function is STRICT

                let arg = &mut args_slice[i];
                (arg.value, arg.isnull) =
                    datum.map(|d| (d, false)).unwrap_or_else(|| (pg_sys::Datum::from(0), true));
            }
        }

        // call the function
        // SAFETY: `flinfo` was create for us by `fmgr_info` above and `fn_addr` would have been properly set
        let func =
            *flinfo.fn_addr.as_ref().expect("function initialization problem: fn_addr not set");

        // SAFETY: `func` is most likely a function pointer on the other side of the Postgres FFI
        // boundary, and we must guard that boundary to ensure any ERRORs will still properly unwind
        // the stack.
        let result_datum = pg_guard_ffi_boundary(|| func(fcinfo));

        // Postgres' "OidFunctionCall" doesn't support returning null, but we can
        let result = R::from_datum(result_datum, (*fcinfo).isnull);

        // cleanup things we heap allocated
        // SAFETY: we allocated `fcinfo` and we're done with it and nothing we're about to return
        // contains any pointers to it or anything it contains.
        pg_sys::pfree(fcinfo.cast());

        Ok(result)
    }
}

/// An argument as [`fn_call`] passes it to the function
enum PassedArg<'a> {
    /// An [`Arg::Value`] of the given type, converted to a Datum after the type check
    Value(&'a dyn FnCallArg, pg_sys::Oid),
    /// A NULL, or the Datum and type of a `DEFAULT` value
    Datum(Option<(pg_sys::Datum, pg_sys::Oid)>),
}

impl PassedArg<'_> {
    /// The type of the value passed, or `None` for a NULL
    fn value_type(&self) -> Option<pg_sys::Oid> {
        match *self {
            PassedArg::Value(_, oid) | PassedArg::Datum(Some((_, oid))) => Some(oid),
            PassedArg::Datum(None) => None,
        }
    }
}

/// Checks the type of each value in `args`, one per parameter, against `params`, the function's
/// parameter types, as the "Argument types" section of [`fn_call`] describes.
///
/// `lookup_fn` can return a function found by name alone, whose parameters may be of any type.
fn check_argument_types(params: &[pg_sys::Oid], args: &[PassedArg]) -> Result<()> {
    debug_assert_eq!(params.len(), args.len());

    // (argument number, value type, parameter type) for each value of a polymorphic parameter
    let mut polymorphic = Vec::new();
    for (argnum, (arg, &param)) in args.iter().zip(params).enumerate() {
        let Some(value) = arg.value_type() else {
            continue;
        };
        if is_polymorphic(param) {
            polymorphic.push((argnum, value, param));
        } else if value != param
            // SAFETY: IsBinaryCoercible takes the OIDs by value and looks them up.  `value` names
            // a type, and `param` came from pg_proc
            && !(names_a_type(value) && unsafe { pg_sys::IsBinaryCoercible(value, param) })
        {
            return Err(FnCallError::IncompatibleArgumentType(argnum, value, param));
        }
    }
    let Some(&last) = polymorphic.last() else {
        return Ok(());
    };

    // check_generic_type_consistency accepts an UNKNOWNOID value for any polymorphic parameter,
    // and a polymorphic value, such as an `anyarray`, without comparing its element type to the
    // other values
    for &(argnum, value, param) in &polymorphic {
        if value == pg_sys::UNKNOWNOID
            || !names_a_type(value)
            || is_polymorphic(value) && (value != param || polymorphic.len() > 1)
        {
            return Err(FnCallError::IncompatibleArgumentType(argnum, value, param));
        }
    }

    // The value types, and UNKNOWNOID for the other arguments, which
    // check_generic_type_consistency skips as it skips an untyped NULL literal
    let mut values = vec![pg_sys::UNKNOWNOID; params.len()];
    for &(argnum, value, _) in &polymorphic {
        values[argnum] = value;
    }
    // `params` with each `anycompatible` type replaced by its `anyelement` counterpart, which
    // requires the same type instead of a common one, and every other type by InvalidOid, which
    // check_generic_type_consistency ignores
    let uncast_params = params.iter().map(|&param| without_common_type(param)).collect::<Vec<_>>();
    // whether the first `nargs` arguments are consistent
    let consistent = |nargs: usize| {
        let nargs = c_int::try_from(nargs).expect("pronargs is an int2");
        // SAFETY: each array holds the function's pronargs OIDs, at least `nargs`, and PostgreSQL
        // limits pronargs to FUNC_MAX_ARGS, the length of an array that
        // check_generic_type_consistency fills.  Every OID in `values` names a type, and `params`
        // and `uncast_params` hold types from pg_proc, polymorphic types, or InvalidOid, which
        // check_generic_type_consistency ignores
        unsafe {
            pg_sys::check_generic_type_consistency(values.as_ptr(), params.as_ptr(), nargs)
                && pg_sys::check_generic_type_consistency(
                    values.as_ptr(),
                    uncast_params.as_ptr(),
                    nargs,
                )
        }
    };
    if consistent(params.len()) {
        return Ok(());
    }

    // Name the first value that conflicts with those before it.  A parameter after the last value
    // can also make the values inconsistent, as an `anyenum` parameter passed NULL requires the
    // `anyelement` values to be enums, so name the last value then.
    let (argnum, value, param) =
        polymorphic.into_iter().find(|&(argnum, ..)| !consistent(argnum + 1)).unwrap_or(last);
    Err(FnCallError::IncompatibleArgumentType(argnum, value, param))
}

fn is_polymorphic(oid: pg_sys::Oid) -> bool {
    pg_sys::IsPolymorphicType!(oid).is_true()
}

/// Whether `oid` names a type.  IsBinaryCoercible and check_generic_type_consistency raise an
/// ERROR for an OID that does not.
fn names_a_type(oid: pg_sys::Oid) -> bool {
    // SAFETY: get_typtype takes the OID by value and returns '\0' for an OID that names no type
    unsafe { pg_sys::get_typtype(oid) != 0 }
}

/// The `anyelement`-family counterpart of an `anycompatible`-family type, or InvalidOid for any
/// other type
fn without_common_type(param: pg_sys::Oid) -> pg_sys::Oid {
    match param {
        pg_sys::ANYCOMPATIBLEOID => pg_sys::ANYELEMENTOID,
        pg_sys::ANYCOMPATIBLEARRAYOID => pg_sys::ANYARRAYOID,
        pg_sys::ANYCOMPATIBLENONARRAYOID => pg_sys::ANYNONARRAYOID,
        pg_sys::ANYCOMPATIBLERANGEOID => pg_sys::ANYRANGEOID,
        pg_sys::ANYCOMPATIBLEMULTIRANGEOID => pg_sys::ANYMULTIRANGEOID,
        _ => pg_sys::Oid::INVALID,
    }
}

fn lookup_fn(fname: &str, args: &[&dyn FnCallArg]) -> Result<pg_sys::Oid> {
    // ask Postgres to find the function.  This will look for the possibly-qualified named
    // function following the normal SEARCH_PATH rules, ensuring its argument type Oids
    // exactly match the ones from the user's input arguments.  It does not evaluate the
    // return type, so we'll have to do that later
    memcx::current_context(|mcx| {
        let mut parts_list = List::<*mut std::ffi::c_void>::default();
        let result = PgTryBuilder::new(AssertUnwindSafe(|| unsafe {
            let arg_types = args.iter().map(|a| a.type_oid()).collect::<Vec<_>>();
            let nargs: i16 =
                arg_types.len().try_into().map_err(|_| FnCallError::TooManyArguments)?;

            // parse the function name into its possibly-qualified name parts
            let ident_parts = parse_sql_ident(fname)?;
            ident_parts
                .iter_deny_null()
                .map(|part| {
                    // SAFETY:  `.as_pg_cstr()` palloc's a char* and `makeString` just takes ownership of it
                    pg_sys::makeString(part.as_pg_cstr())
                })
                .for_each(|part| {
                    parts_list.unstable_push_in_context(part.cast(), mcx);
                });

            // look up an exact match based on the exact number of arguments we have
            //
            // SAFETY:  we've allocated a PgList with the proper String node elements representing its name
            // and we've allocated Vec of argument type oids which can be represented as a pointer.
            let mut fnoid = pg_sys::LookupFuncName(
                parts_list.as_mut_ptr(),
                nargs as _,
                arg_types.as_ptr(),
                true,
            );

            if fnoid == pg_sys::Oid::INVALID {
                // if that didn't find a function, maybe we've got some defaults in there, or
                // polymorphic parameters.  A `nargs` of -1 finds a function by its name alone,
                // whatever its parameters, so `check_argument_types` checks them later
                fnoid = pg_sys::LookupFuncName(
                    parts_list.as_mut_ptr(),
                    -1,
                    arg_types.as_ptr(),
                    false, // we want the ERROR here -- could be UNDEFINED_FUNCTION or AMBIGUOUS_FUNCTION
                );
            }

            Ok(fnoid)
        }))
        .catch_when(PgSqlErrorCode::ERRCODE_INVALID_PARAMETER_VALUE, |_| {
            Err(FnCallError::InvalidIdentifier(fname.to_string()))
        })
        .catch_when(PgSqlErrorCode::ERRCODE_AMBIGUOUS_FUNCTION, |_| {
            Err(FnCallError::AmbiguousFunction)
        })
        .catch_when(PgSqlErrorCode::ERRCODE_UNDEFINED_FUNCTION, |_| {
            Err(FnCallError::UndefinedFunction)
        })
        .execute();

        unsafe {
            // SAFETY:  we palloc'd the `pg_sys::String` elements of `parts_list` above and so it's
            // safe for us to free them now that they're no longer being used
            parts_list.drain(..).for_each(|s| {
                let s = s.cast::<pg_sys::String>();
                pg_sys::pfree((*s).sval.cast());
            });
        }
        result
    })
}

/// Parses an arbitrary string as if it is a SQL identifier.  If it's not, [`FnCallError::InvalidIdentifier`]
/// is returned
fn parse_sql_ident(ident: &str) -> Result<Array<'_, &str>> {
    unsafe {
        direct_function_call::<Array<&str>>(
            pg_sys::parse_ident,
            &[ident.into_datum(), true.into_datum()],
        )
        .ok_or_else(|| FnCallError::InvalidIdentifier(ident.to_string()))
    }
}

/// Materializes a the `DEFAULT` value at the specified argument position `argnum` for the specified
/// function `pg_proc`.
///
/// `argnum` is the overall argument position, not the specific argument from just the set of
/// arguments with defaults.  This is noted as Postgres internally understands these as different
/// things.  Given the argument number, [`create_default_value`] determines which argument from the
/// list of DEFAULTed arguments is being requested.
///
/// Returns the value's Datum and type, or `None` if it is NULL.
///
/// # Errors
///
/// - [`FnCallError::NotDefaultArgument`] if the specified `argnum` does not have a `DEFAULT` clause
/// - [`FnCallError::DefaultNotConstantExpression`] if the `DEFAULT` clause is one we cannot evaluate
fn create_default_value(
    pg_proc: &PgProc,
    argnum: usize,
) -> Result<Option<(pg_sys::Datum, pg_sys::Oid)>> {
    let non_default_args_cnt = pg_proc.pronargs() - pg_proc.pronargdefaults();
    if argnum < non_default_args_cnt {
        return Err(FnCallError::NotDefaultArgument(argnum));
    }

    let default_argnum = argnum - non_default_args_cnt;
    let node = memcx::current_context(|mcx| {
        let default_value_tree =
            pg_proc.proargdefaults(mcx).ok_or(FnCallError::NoDefaultArguments)?;
        default_value_tree
            .get(default_argnum)
            .ok_or(FnCallError::NotDefaultArgument(argnum))
            .copied()
    })?;

    unsafe {
        // SAFETY:  `arg_root` is okay to be the null pointer here, which indicates we don't care
        // about `eval_const_expressions` providing us extra metrics about what it did/found while
        // evaluating `node`.
        //
        // With that, `node` is a valid Node* taken from the PgProc entry
        let evaluated = pg_sys::eval_const_expressions(std::ptr::null_mut(), node.cast());

        // SAFETY:  evaluated is a valid Node* as that's all `eval_const_expressions` can return
        if !evaluated.is_null()
            && pg_sys::__pgrx_c_macros::expression_result::return_value::<pg_sys::NodeTag, _>(
                pg_sys::nodeTag!(evaluated).into_value(),
            ) == pg_sys::NodeTag::T_Const
        {
            let con: *mut pg_sys::Const = evaluated.cast();
            let con_ref = &*con;

            if con_ref.constisnull {
                Ok(None)
            } else {
                Ok(Some((con_ref.constvalue, con_ref.consttype)))
            }
        } else {
            // NB:  I am not sure this case could ever happen in the context of a function argument
            // `DEFAULT` clause, but if it does, we should let the caller know.  I don't know what
            // they'd do about it other than instead specifying an explicit value for this argnum
            Err(FnCallError::DefaultNotConstantExpression)
        }
    }
}
