use crate::pg_sys;

union Float4Union {
    value: i32,
    retval: f32,
}

union Float8Union {
    value: i64,
    retval: f64,
}

#[inline]
pub fn pg_arg_is_null(fcinfo: &pg_sys::FunctionCallInfoData, num: usize) -> bool {
    fcinfo.argnull[num]
}

#[inline]
pub fn pg_getarg_datum(fcinfo: &pg_sys::FunctionCallInfoData, num: usize) -> Option<pg_sys::Datum> {
    if pg_arg_is_null(fcinfo, num) {
        None
    } else {
        Some(fcinfo.arg[num])
    }
}

#[inline]
pub fn pg_getarg_i32(fcinfo: &pg_sys::FunctionCallInfoData, num: usize) -> Option<i32> {
    match pg_getarg_datum(fcinfo, num) {
        Some(d) => Some(datum_get_i32(d)),
        None => None,
    }
}

#[inline]
pub fn pg_getarg_u32(fcinfo: &pg_sys::FunctionCallInfoData, num: usize) -> Option<u32> {
    match pg_getarg_datum(fcinfo, num) {
        Some(d) => Some(datum_get_u32(d)),
        None => None,
    }
}

#[inline]
pub fn pg_getarg_i64(fcinfo: &pg_sys::FunctionCallInfoData, num: usize) -> Option<i64> {
    match pg_getarg_datum(fcinfo, num) {
        Some(d) => Some(datum_get_i64(d)),
        None => None,
    }
}

#[inline]
pub fn pg_getarg_u16(fcinfo: &pg_sys::FunctionCallInfoData, num: usize) -> Option<u16> {
    match pg_getarg_datum(fcinfo, num) {
        Some(d) => Some(datum_get_u16(d)),
        None => None,
    }
}

#[inline]
pub fn pg_getarg_char(fcinfo: &pg_sys::FunctionCallInfoData, num: usize) -> Option<char> {
    match pg_getarg_datum(fcinfo, num) {
        Some(d) => Some(datum_get_char(d)),
        None => None,
    }
}

#[inline]
pub fn pg_getarg_bool(fcinfo: &pg_sys::FunctionCallInfoData, num: usize) -> Option<bool> {
    match pg_getarg_datum(fcinfo, num) {
        Some(d) => Some(datum_get_bool(d)),
        None => None,
    }
}

#[inline]
pub fn pg_getarg_oid(fcinfo: &pg_sys::FunctionCallInfoData, num: usize) -> Option<pg_sys::Oid> {
    match pg_getarg_datum(fcinfo, num) {
        Some(d) => Some(datum_get_oid(d)),
        None => None,
    }
}

#[inline]
pub fn pg_getarg_cstring(
    fcinfo: &pg_sys::FunctionCallInfoData,
    num: usize,
) -> Option<std::ffi::CString> {
    match pg_getarg_datum(fcinfo, num) {
        Some(d) => Some(datum_get_cstring(d)),
        None => None,
    }
}

#[inline]
pub fn pg_getarg_name(fcinfo: &pg_sys::FunctionCallInfoData, num: usize) -> Option<pg_sys::Name> {
    match pg_getarg_datum(fcinfo, num) {
        Some(d) => Some(datum_get_name(d)),
        None => None,
    }
}

#[inline]
pub fn pg_getarg_float4(fcinfo: &pg_sys::FunctionCallInfoData, num: usize) -> Option<f32> {
    match pg_getarg_datum(fcinfo, num) {
        Some(d) => Some(datum_get_float4(d)),
        None => None,
    }
}

#[inline]
pub fn pg_getarg_float8(fcinfo: &pg_sys::FunctionCallInfoData, num: usize) -> Option<f64> {
    match pg_getarg_datum(fcinfo, num) {
        Some(d) => Some(datum_get_float8(d)),
        None => None,
    }
}

#[inline]
pub fn pg_getarg_text_p(
    fcinfo: &pg_sys::FunctionCallInfoData,
    num: usize,
) -> Option<*const pg_sys::text> {
    match pg_getarg_datum(fcinfo, num) {
        Some(d) => Some(datum_get_text_p(d)),
        None => None,
    }
}

pub fn pg_getarg_text_p_as_cstring(
    fcinfo: &pg_sys::FunctionCallInfoData,
    num: usize,
) -> Option<std::ffi::CString> {
    match pg_getarg_text_p(fcinfo, num) {
        Some(t) => Some(unsafe {
            std::ffi::CString::from(std::ffi::CStr::from_ptr(pg_sys::text_to_cstring(t)))
        }),
        None => None,
    }
}

#[inline]
pub fn datum_get_i32(d: pg_sys::Datum) -> i32 {
    d as i32
}

#[inline]
pub fn datum_get_u32(d: pg_sys::Datum) -> u32 {
    d as u32
}

#[inline]
pub fn datum_get_i64(d: pg_sys::Datum) -> i64 {
    d as i64
}

#[inline]
pub fn datum_get_u16(d: pg_sys::Datum) -> u16 {
    d as u16
}

#[inline]
pub fn datum_get_char(d: pg_sys::Datum) -> char {
    d as u8 as char
}

#[inline]
pub fn datum_get_bool(d: pg_sys::Datum) -> bool {
    d == 0
}

#[inline]
pub fn datum_get_oid(d: pg_sys::Datum) -> pg_sys::Oid {
    d as pg_sys::Oid
}

#[inline]
pub fn datum_get_cstring(d: pg_sys::Datum) -> std::ffi::CString {
    unsafe { std::ffi::CString::from(std::ffi::CStr::from_ptr(d as *mut std::os::raw::c_char)) }
}

#[inline]
pub fn datum_get_name(d: pg_sys::Datum) -> pg_sys::Name {
    d as *mut std::ffi::c_void as pg_sys::Name
}

#[inline]
pub fn datum_get_float4(d: pg_sys::Datum) -> f32 {
    if pg_sys::USE_FLOAT4_BYVAL == 1 {
        unsafe {
            Float4Union {
                value: datum_get_i32(d),
            }
            .retval
        }
    } else {
        unsafe { *((d as *const std::ffi::c_void) as *const f32) }
    }
}

#[inline]
pub fn datum_get_float8(d: pg_sys::Datum) -> f64 {
    if pg_sys::USE_FLOAT8_BYVAL == 1 {
        unsafe {
            Float8Union {
                value: datum_get_i64(d),
            }
            .retval
        }
    } else {
        unsafe { *((d as *const std::ffi::c_void) as *const f64) }
    }
}

#[inline]
pub fn datum_get_text_p(d: pg_sys::Datum) -> *const pg_sys::text {
    d as *const pg_sys::text
}
