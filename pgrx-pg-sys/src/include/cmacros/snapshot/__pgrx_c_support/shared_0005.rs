// Shared generated C macro support.

#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Pg_abi_values, version) == 0);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Pg_abi_values, funcmaxargs) == 4);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Pg_abi_values, indexmaxkeys) == 8);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Pg_abi_values, namedatalen) == 12);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Pg_abi_values, float8byval) == 16);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Pg_abi_values, abi_extra) == 20);
#[cfg(any(feature = "pg18", feature = "pg19"))]
/// Compile-time storage witness for every initialized field of this metadata record.
#[allow(dead_code)]
fn __pgrx_metadata_fields_1(value: &crate::__pgrx_c_bindings::Pg_abi_values) {
    let _: &i32 = &value.version;
    let _: &i32 = &value.funcmaxargs;
    let _: &i32 = &value.indexmaxkeys;
    let _: &i32 = &value.namedatalen;
    let _: &i32 = &value.float8byval;
    let _: &[::core::ffi::c_char; 32] = &value.abi_extra;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Pg_abi_values>() == 52);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Pg_abi_values>() == 4);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Pg_magic_struct, abi_fields) == 4);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<::core::ffi::c_char>() == 1);
    assert!(::core::mem::align_of::<::core::ffi::c_char>() == 1);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<*const ::core::ffi::c_char>() == 8);
    assert!(::core::mem::align_of::<*const ::core::ffi::c_char>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Pg_magic_struct, name) == 56);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<::core::ffi::c_char>() == 1);
    assert!(::core::mem::align_of::<::core::ffi::c_char>() == 1);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<*const ::core::ffi::c_char>() == 8);
    assert!(::core::mem::align_of::<*const ::core::ffi::c_char>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Pg_magic_struct, version) == 64);
#[cfg(any(feature = "pg18", feature = "pg19"))]
/// Compile-time storage witness for every initialized field of this metadata record.
#[allow(dead_code)]
fn __pgrx_metadata_fields_0(value: &crate::__pgrx_c_bindings::Pg_magic_struct) {
    let _: &i32 = &value.len;
    let _: &crate::__pgrx_c_bindings::Pg_abi_values = &value.abi_fields;
    let _: &*const ::core::ffi::c_char = &value.name;
    let _: &*const ::core::ffi::c_char = &value.version;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Pg_magic_struct>() == 72);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Pg_magic_struct>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
/// Compile-time storage witness for every initialized field of this metadata record.
#[allow(dead_code)]
fn __pgrx_metadata_fields_2(value: &crate::__pgrx_c_bindings::Pg_finfo_record) {
    let _: &i32 = &value.api_version;
}
