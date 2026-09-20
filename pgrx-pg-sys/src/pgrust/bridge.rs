//! SQL-callable builtins with the C `fn(FunctionCallInfo) -> Datum` shape.
//!
//! The pgrx runtime calls dozens of backend functions the way C does:
//! `direct_function_call(pg_sys::numeric_add, &[a, b])`. Each such name is
//! defined here as a function that converts the C-shaped frame pgrx built
//! into a native pgrust frame and invokes the builtin of the same name from
//! pgrust's fmgr table.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

use ::pgr_datum::Datum as NDatum;
use ::pgr_fmgr::{LocalFcinfo, PGFunction as NativePGFunction};

use super::error::{raise, unwrap_pg};
use super::fmgr::{FunctionCallInfoBaseData, result_mcx};
use super::types::FunctionCallInfo;
use crate::Datum;

fn builtin(name: &'static str) -> NativePGFunction {
    static CACHE: OnceLock<Mutex<HashMap<&'static str, NativePGFunction>>> = OnceLock::new();
    let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));
    if let Some(f) = cache.lock().unwrap().get(name) {
        return *f;
    }
    match ::pgr_fmgr_core::fmgr_lookup_by_name(name) {
        Some(row) => {
            cache.lock().unwrap().insert(name, row.func);
            row.func
        }
        None => raise(Box::new(::pgr_types_error::PgError::error(format!(
            "pgrx: builtin function \"{name}\" is not available in pgrust"
        )))),
    }
}

macro_rules! call_with_arity {
    ($func:expr, $c:expr, $n:expr, $($N:literal),*) => {
        match $n {
            $(
                $N => {
                    let mut frame = LocalFcinfo::<$N>::new($c.fncollation.to_u32());
                    // SAFETY: the frame lives for this call only.
                    unsafe { frame.set_result_mcx(result_mcx()) };
                    let args = unsafe { $c.args.as_slice($N) };
                    for (i, a) in args.iter().enumerate() {
                        if a.isnull {
                            frame.set_arg_null(i);
                        } else {
                            frame.set_arg(i, NDatum::from_usize(a.value.value()));
                        }
                    }
                    let r = unwrap_pg($func(None, &mut frame));
                    $c.isnull = frame.isnull;
                    Datum::from(r.as_usize())
                }
            )*
            _ => raise(Box::new(::pgr_types_error::PgError::error(
                "pgrx: builtin call with more than 9 arguments",
            ))),
        }
    };
}

/// Invoke the pgrust builtin `name` on a C-shaped frame.
///
/// # Safety
/// `fcinfo` must be a live frame with `nargs` valid arguments.
pub unsafe fn call_builtin(name: &'static str, fcinfo: FunctionCallInfo) -> Datum {
    let func = builtin(name);
    let c: &mut FunctionCallInfoBaseData = &mut *fcinfo;
    let n = c.nargs as usize;
    call_with_arity!(func, c, n, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9)
}

macro_rules! fmgr_builtins {
    ($($name:ident),* $(,)?) => {
        $(
            #[allow(non_snake_case)]
            pub unsafe fn $name(fcinfo: FunctionCallInfo) -> Datum {
                call_builtin(stringify!($name), fcinfo)
            }
        )*
    };
}

// Every C-convention builtin the pgrx runtime names. Extend freely: a name
// that pgrust's fmgr table lacks fails at call time with a clear error.
fmgr_builtins! {
    // json
    jsonb_in, jsonb_out, json_in, json_out,
    // regproc
    regtypein, regclassin, regprocedurein, regtypeout,
    // numeric
    numeric_in, numeric_out, numeric_abs, numeric_ceil, numeric_cmp, numeric_eq, numeric_exp,
    numeric_floor, numeric_gcd, numeric_ge, numeric_gt, numeric_le, numeric_log, numeric_lt,
    numeric_ne, numeric_sqrt, numeric_uminus, numeric_add, numeric_sub, numeric_mul, numeric_div,
    numeric_mod, numeric_power, numeric_round, numeric_trunc, numeric_sign, numeric_ln,
    numeric_inc, numeric_uplus, numeric_larger, numeric_smaller, numeric_div_trunc,
    hash_numeric, int2_numeric, int4_numeric, int8_numeric, float4_numeric, float8_numeric,
    numeric_int2, numeric_int4, numeric_int8, numeric_float4, numeric_float8,
    // date/time
    date_in, date_out, date_timestamp, date_timestamptz, date_pli, date_mii, date_pl_interval,
    date_mi_interval, date_eq, date_cmp, extract_date,
    time_in, time_out, time_interval, time_timetz, time_pl_interval, time_mi_interval,
    time_mi_time, time_part, time_eq, time_cmp, time_hash, extract_time,
    timetz_in, timetz_out, timetz_izone, timetz_time, timetz_zone, timetz_pl_interval,
    timetz_mi_interval, timetz_part, timetz_eq, timetz_cmp, timetz_hash, extract_timetz,
    timestamp_in, timestamp_out, timestamp_age, timestamp_date, timestamp_time,
    timestamp_timestamptz, timestamp_trunc, timestamp_pl_interval, timestamp_mi_interval,
    timestamp_mi, timestamp_part, timestamp_eq, timestamp_cmp, timestamp_hash, extract_timestamp,
    timestamptz_in, timestamptz_out, timestamptz_date, timestamptz_time, timestamptz_timestamp,
    timestamptz_timetz, timestamptz_trunc, timestamptz_trunc_zone, timestamptz_zone,
    timestamptz_pl_interval, timestamptz_mi_interval, timestamptz_part, extract_timestamptz,
    interval_in, interval_out, interval_justify_days, interval_justify_hours,
    interval_justify_interval, interval_time, interval_trunc, interval_pl, interval_mi,
    interval_mul, interval_div, interval_um, interval_part, interval_eq, interval_cmp,
    interval_hash, extract_interval,
    make_date, make_time, make_interval, make_timestamp, make_timestamptz,
    make_timestamptz_at_timezone, datetime_timestamp, datetimetz_timestamptz, float8_timestamptz,
    timeofday,
    // network
    inet_in, inet_out,
    // uuid
    uuid_in, uuid_out,
    // hashing / misc
    hashint8, hashint4, hashtext, numeric,
    // text
    textin, textout, quote_ident, quote_literal, quote_nullable,
}
