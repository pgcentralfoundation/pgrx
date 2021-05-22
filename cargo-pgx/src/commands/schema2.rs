use crate::commands::{
    install::{build_extension, find_library_file},
    get::{find_control_file},
};
use libloading::Library;
use pgx_utils::pg_config::PgConfig;
use std::ffi::CString;

pub(crate) fn generate_schema(pg_config: &PgConfig, is_release: bool, additional_features: &[&str]) -> Result<(), std::io::Error> {
    let major_version = pg_config.major_version()?;
    build_extension(major_version, is_release, additional_features);
    let (control_file, extname) = find_control_file();
    let shlibpath = find_library_file(&extname, is_release);
    let c_schema = unsafe {
        let library = Library::new(&shlibpath).expect("Could not load library.");
        let alloc_meta: libloading::Symbol<unsafe extern fn() -> *mut std::os::raw::c_char> = library.get(&"alloc_meta".as_bytes()).expect("No alloc_meta fn.");
        let drop_meta: libloading::Symbol<unsafe extern fn(*mut std::os::raw::c_char)> = library.get(&"drop_meta".as_bytes()).expect("No alloc_meta fn.");
        let c_schema = alloc_meta();
        let schema = CString::from_raw(c_schema);
        schema
    };
    let alloc_meta = c_schema.into_string().expect("Failed to make string");

    panic!("{}", alloc_meta);
    Ok(())
}