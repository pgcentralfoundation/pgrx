use crate::{pg_sys, rust_str_to_text_p, DatumCompatible, PgDatum};

//
// PG 10
//

#[cfg(feature = "pg10")]
#[inline]
pub fn pg_getarg<T>(fcinfo: pg_sys::FunctionCallInfo, num: usize) -> PgDatum<T>
where
    T: DatumCompatible<T>,
{
    PgDatum::<T>::new(
        unsafe { fcinfo.as_ref() }.unwrap().arg[num],
        unsafe { fcinfo.as_ref() }.unwrap().argnull[num] as bool,
    )
}

#[cfg(feature = "pg10")]
#[inline]
pub fn pg_arg_is_null(fcinfo: pg_sys::FunctionCallInfo, num: usize) -> bool {
    unsafe { fcinfo.as_ref() }.unwrap().argnull[num] as bool
}

#[cfg(feature = "pg10")]
#[inline]
pub fn pg_getarg_datum(fcinfo: pg_sys::FunctionCallInfo, num: usize) -> Option<pg_sys::Datum> {
    if pg_arg_is_null(fcinfo, num) {
        None
    } else {
        Some(unsafe { fcinfo.as_ref() }.unwrap().arg[num])
    }
}

#[cfg(feature = "pg10")]
#[inline]
pub fn pg_return_null(fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
    unsafe { fcinfo.as_mut() }.unwrap().isnull = true;
    0 as pg_sys::Datum
}

//
// PG 11
//

#[cfg(feature = "pg11")]
#[inline]
pub fn pg_getarg<T>(fcinfo: pg_sys::FunctionCallInfo, num: usize) -> PgDatum<T>
where
    T: DatumCompatible<T>,
{
    PgDatum::<T>::new(
        unsafe { fcinfo.as_ref() }.unwrap().arg[num],
        unsafe { fcinfo.as_ref() }.unwrap().argnull[num] as bool,
    )
}

#[cfg(feature = "pg11")]
#[inline]
pub fn pg_arg_is_null(fcinfo: pg_sys::FunctionCallInfo, num: usize) -> bool {
    unsafe { fcinfo.as_ref() }.unwrap().argnull[num] as bool
}

#[cfg(feature = "pg11")]
#[inline]
pub fn pg_getarg_datum(fcinfo: pg_sys::FunctionCallInfo, num: usize) -> Option<pg_sys::Datum> {
    if pg_arg_is_null(fcinfo, num) {
        None
    } else {
        Some(unsafe { fcinfo.as_ref() }.unwrap().arg[num])
    }
}

#[cfg(feature = "pg11")]
#[inline]
pub fn pg_return_null(fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
    unsafe { fcinfo.as_mut() }.unwrap().isnull = true;
    0 as pg_sys::Datum
}

//
// PG 12
//

#[cfg(feature = "pg12")]
#[inline]
pub fn pg_getarg<T>(fcinfo: pg_sys::FunctionCallInfo, num: usize) -> PgDatum<T>
where
    T: DatumCompatible<T>,
{
    let datum = get_nullable_datum(fcinfo, num);
    PgDatum::<T>::new(datum.value, datum.isnull)
}

#[cfg(feature = "pg12")]
#[inline]
pub fn pg_arg_is_null(fcinfo: pg_sys::FunctionCallInfo, num: usize) -> bool {
    get_nullable_datum(fcinfo, num).isnull
}

#[cfg(feature = "pg12")]
#[inline]
pub fn pg_getarg_datum(fcinfo: pg_sys::FunctionCallInfo, num: usize) -> Option<pg_sys::Datum> {
    if pg_arg_is_null(fcinfo, num) {
        None
    } else {
        Some(get_nullable_datum(fcinfo, num).value)
    }
}

#[cfg(feature = "pg12")]
#[inline]
fn get_nullable_datum(
    fcinfo: pg_sys::FunctionCallInfo,
    num: usize,
) -> pg_sys::pg12_specific::NullableDatum {
    let fcinfo = unsafe { fcinfo.as_mut() }.unwrap();
    unsafe {
        let nargs = fcinfo.nargs;
        let len = std::mem::size_of::<pg_sys::pg12_specific::NullableDatum>() * nargs as usize;
        fcinfo.args.as_slice(len)[num]
    }
}

#[cfg(feature = "pg12")]
#[inline]
pub fn pg_return_null(fcinfo: pg_sys::FunctionCallInfo) -> pg_sys::Datum {
    let fcinfo = unsafe { fcinfo.as_mut() }.unwrap();
    fcinfo.isnull = true;
    0 as pg_sys::Datum
}

//
// common
//

#[inline]
pub fn pg_getarg_pointer<T>(fcinfo: pg_sys::FunctionCallInfo, num: usize) -> Option<*mut T> {
    match pg_getarg_datum(fcinfo, num) {
        Some(datum) => Some(datum as *mut T),
        None => None,
    }
}

#[inline]
pub fn pg_getarg_boxed<T>(fcinfo: pg_sys::FunctionCallInfo, num: usize) -> Option<crate::PgBox<T>>
where
    T: DatumCompatible<T>,
{
    match pg_getarg_pointer(fcinfo, num) {
        Some(ptr) => Some(crate::PgBox::from_pg(ptr)),
        None => None,
    }
}

/// this is intended for Postgres functions that take an actual `cstring` argument, not for getting
/// a varlena argument type as a CStr.
#[inline]
pub fn pg_getarg_cstr<'a>(
    fcinfo: pg_sys::FunctionCallInfo,
    num: usize,
) -> Option<&'a std::ffi::CStr> {
    match pg_getarg_pointer(fcinfo, num) {
        Some(ptr) => Some(unsafe { std::ffi::CStr::from_ptr(ptr) }),
        None => None,
    }
}

#[inline]
pub fn pg_return_text_p(s: &str) -> pg_sys::Datum {
    rust_str_to_text_p(s) as pg_sys::Datum
}

#[inline]
pub fn pg_return_void() -> pg_sys::Datum {
    0 as pg_sys::Datum
}
