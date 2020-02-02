use crate::pg_sys;

#[inline]
pub fn name_data_to_str<'a>(name_data: &pg_sys::NameData) -> &'a str {
    unsafe { std::ffi::CStr::from_ptr(name_data.data.as_ptr()) }
        .to_str()
        .unwrap()
}
