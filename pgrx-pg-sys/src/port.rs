//! PostgreSQL version utilities and guarded callback declarations for older servers.

use core::str::FromStr;

#[inline]
pub fn get_pg_major_version_string() -> &'static str {
    super::PG_MAJORVERSION.to_str().unwrap()
}

#[inline]
pub fn get_pg_major_version_num() -> u16 {
    u16::from_str(super::get_pg_major_version_string()).unwrap()
}

#[cfg(any(not(target_env = "msvc"), feature = "pg17", feature = "pg18", feature = "pg19"))]
#[inline]
pub fn get_pg_version_string() -> &'static str {
    super::PG_VERSION_STR.to_str().unwrap()
}

#[inline]
pub fn get_pg_major_minor_version_string() -> &'static str {
    super::PG_VERSION.to_str().unwrap()
}

// PG15 declares these callbacks without prototypes. Keep their Rust signatures
// typed; PG16+ uses generated macros and the current implementation functions.
#[cfg(feature = "pg15")]
#[::pgrx_macros::pg_guard]
unsafe extern "C-unwind" {
    pub fn planstate_tree_walker(
        planstate: *mut super::PlanState,
        walker: ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut super::PlanState, *mut ::core::ffi::c_void) -> bool,
        >,
        context: *mut ::core::ffi::c_void,
    ) -> bool;

    pub fn query_tree_walker(
        query: *mut super::Query,
        walker: ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut super::Node, *mut ::core::ffi::c_void) -> bool,
        >,
        context: *mut ::core::ffi::c_void,
        flags: ::core::ffi::c_int,
    ) -> bool;

    pub fn query_or_expression_tree_walker(
        node: *mut super::Node,
        walker: ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut super::Node, *mut ::core::ffi::c_void) -> bool,
        >,
        context: *mut ::core::ffi::c_void,
        flags: ::core::ffi::c_int,
    ) -> bool;

    pub fn range_table_entry_walker(
        rte: *mut super::RangeTblEntry,
        walker: ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut super::Node, *mut ::core::ffi::c_void) -> bool,
        >,
        context: *mut ::core::ffi::c_void,
        flags: ::core::ffi::c_int,
    ) -> bool;

    pub fn range_table_walker(
        rtable: *mut super::List,
        walker: ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut super::Node, *mut ::core::ffi::c_void) -> bool,
        >,
        context: *mut ::core::ffi::c_void,
        flags: ::core::ffi::c_int,
    ) -> bool;

    pub fn expression_tree_walker(
        node: *mut super::Node,
        walker: ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut super::Node, *mut ::core::ffi::c_void) -> bool,
        >,
        context: *mut ::core::ffi::c_void,
    ) -> bool;

    pub fn raw_expression_tree_walker(
        node: *mut super::Node,
        walker: ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut super::Node, *mut ::core::ffi::c_void) -> bool,
        >,
        context: *mut ::core::ffi::c_void,
    ) -> bool;
}
