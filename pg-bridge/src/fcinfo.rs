use crate::{pg_sys, rust_str_to_text_p, DatumCompatible, PgDatum};

#[inline]
pub fn pg_getarg<T>(fcinfo: &pg_sys::FunctionCallInfo, num: usize) -> PgDatum<T>
where
    T: DatumCompatible<T>,
{
    PgDatum::<T>::new(
        (unsafe { *(*fcinfo) }).arg[num],
        (unsafe { *(*fcinfo) }).argnull[num] as bool,
    )
}

#[inline]
pub fn pg_arg_is_null(fcinfo: &pg_sys::FunctionCallInfo, num: usize) -> bool {
    (unsafe { *(*fcinfo) }).argnull[num] as bool
}

#[inline]
pub fn pg_getarg_datum(fcinfo: &pg_sys::FunctionCallInfo, num: usize) -> Option<pg_sys::Datum> {
    if pg_arg_is_null(fcinfo, num) {
        None
    } else {
        Some((unsafe { *(*fcinfo) }).arg[num])
    }
}

#[inline]
pub fn pg_return_text_p(s: &str) -> pg_sys::Datum {
    rust_str_to_text_p(s) as pg_sys::Datum
}

#[inline]
pub fn pg_return_null(fcinfo: &pg_sys::FunctionCallInfo) -> pg_sys::Datum {
    (unsafe { *(*fcinfo) }).isnull = true;
    0 as pg_sys::Datum
}

#[inline]
pub fn pg_return_void() -> pg_sys::Datum {
    0 as pg_sys::Datum
}
