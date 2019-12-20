use crate::pg_sys;
use std::borrow::Cow;

extern "C" {
    fn zdb_varsize_exhdr(t: *const pg_sys::varlena) -> usize;
    fn zdb_vardata_any(t: *const pg_sys::varlena) -> *const std::ffi::c_void;
}

/// ```c
/// #define VARSIZE_4B(PTR) \
/// ((((varattrib_4b *) (PTR))->va_4byte.va_header >> 2) & 0x3FFFFFFF)
/// ```
#[inline]
pub fn varsize_4b(ptr: *const pg_sys::varlena) -> u32 {
    unsafe {
        let va4b = ptr as *const pg_sys::varattrib_4b__bindgen_ty_1;
        ((*va4b).va_header >> 2) & 0x3FFFFFFF
    }
}

/// ```c
/// #define VARSIZE_1B(PTR) \
/// ((((varattrib_1b *) (PTR))->va_header >> 1) & 0x7F)
/// ```
#[inline]
pub fn varsize_1b(ptr: *const pg_sys::varlena) -> u8 {
    unsafe {
        let va1b = ptr as *const pg_sys::varattrib_1b;
        ((*va1b).va_header >> 1) & 0x7F
    }
}

/// ```c
/// #define VARTAG_1B_E(PTR) \
/// (((varattrib_1b_e *) (PTR))->va_tag)
/// ```
#[inline]
pub fn vartag_1b_e(ptr: *const pg_sys::varlena) -> u8 {
    unsafe {
        let va1be = ptr as *const pg_sys::varattrib_1b_e;
        (*va1be).va_tag
    }
}

#[inline]
pub fn varsize(ptr: *const pg_sys::varlena) -> u32 {
    varsize_4b(ptr)
}

/// ```c
/// #define VARATT_IS_4B(PTR) \
/// ((((varattrib_1b *) (PTR))->va_header & 0x01) == 0x00)
/// ```
#[inline]
pub fn varatt_is_4b(ptr: *const pg_sys::varlena) -> bool {
    unsafe {
        let va1b = ptr as *const pg_sys::varattrib_1b;
        (*va1b).va_header & 0x01 == 0x00
    }
}

/// ```c
/// #define VARATT_IS_4B_U(PTR) \
/// ((((varattrib_1b *) (PTR))->va_header & 0x03) == 0x00)
/// ```
#[inline]
pub fn varatt_is_4b_u(ptr: *const pg_sys::varlena) -> bool {
    unsafe {
        let va1b = ptr as *const pg_sys::varattrib_1b;
        (*va1b).va_header & 0x03 == 0x00
    }
}

/// ```c
/// #define VARATT_IS_4B_C(PTR) \
/// ((((varattrib_1b *) (PTR))->va_header & 0x03) == 0x02)
/// ```
#[inline]
pub fn varatt_is_b8_c(ptr: *const pg_sys::varlena) -> bool {
    unsafe {
        let va1b = ptr as *const pg_sys::varattrib_1b;
        (*va1b).va_header & 0x03 == 0x02
    }
}

/// ```c
/// #define VARATT_IS_1B(PTR) \
/// ((((varattrib_1b *) (PTR))->va_header & 0x01) == 0x01)
/// ```
#[inline]
pub fn varatt_is_b8(ptr: *const pg_sys::varlena) -> bool {
    unsafe {
        let va1b = ptr as *const pg_sys::varattrib_1b;
        (*va1b).va_header & 0x01 == 0x01
    }
}

/// ```c
/// #define VARATT_IS_1B_E(PTR) \
/// ((((varattrib_1b *) (PTR))->va_header) == 0x01)
/// ```
#[inline]
pub fn varatt_is_b8_e(ptr: *const pg_sys::varlena) -> bool {
    unsafe {
        let va1b = ptr as *const pg_sys::varattrib_1b;
        (*va1b).va_header == 0x01
    }
}

/// ```c
/// #define VARATT_NOT_PAD_BYTE(PTR) \
/// (*((uint8 *) (PTR)) != 0)
/// ```
#[inline]
pub fn varatt_not_pad_byte(ptr: *const pg_sys::varlena) -> bool {
    !ptr.is_null()
}

#[inline]
pub fn varsize_exhdr(t: *const pg_sys::varlena) -> usize {
    unsafe { zdb_varsize_exhdr(t) }
}

#[inline]
pub fn vardata_any(t: *const pg_sys::varlena) -> *const std::ffi::c_void {
    unsafe { zdb_vardata_any(t) }
}

#[inline]
pub fn text_to_rust_str<'a>(t: *const pg_sys::text) -> Cow<'a, str> {
    unsafe {
        let len = varsize_exhdr(t);
        let data = vardata_any(t);

        Cow::Borrowed(std::str::from_utf8_unchecked(std::slice::from_raw_parts(
            data as *mut u8,
            len,
        )))
    }
}

#[inline]
pub fn rust_str_to_text_p(s: &str) -> *const pg_sys::text {
    let len = s.len();
    let ptr = s.as_ptr();
    unsafe { pg_sys::cstring_to_text_with_len(ptr as *const std::os::raw::c_char, len as i32) }
}
