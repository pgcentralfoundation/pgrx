use crate::pg_sys;
use std::borrow::Cow;

extern "C" {
    fn zdb_varsize_exhdr(t: *const pg_sys::varlena) -> usize;
    fn zdb_vardata_any(t: *const pg_sys::varlena) -> *const std::ffi::c_void;
}

#[inline]
pub unsafe fn varsize_exhdr(t: *const pg_sys::varlena) -> usize {
    zdb_varsize_exhdr(t)
}

#[inline]
pub unsafe fn vardata_any(t: *const pg_sys::varlena) -> *const std::ffi::c_void {
    zdb_vardata_any(t)
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
