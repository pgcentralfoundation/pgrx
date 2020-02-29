use crate as pg_sys;

#[inline]
pub fn name_data_to_str(name_data: &pg_sys::NameData) -> &str {
    unsafe { std::ffi::CStr::from_ptr(name_data.data.as_ptr()) }
        .to_str()
        .unwrap()
}
