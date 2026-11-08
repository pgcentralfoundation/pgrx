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
// typed. PG16+ turned these names into macros over `*_impl` functions, which the
// wrappers below call with the same signatures.
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
        walker: TreeWalker,
        context: *mut ::core::ffi::c_void,
        flags: ::core::ffi::c_int,
    ) -> bool;

    pub fn query_or_expression_tree_walker(
        node: *mut super::Node,
        walker: TreeWalker,
        context: *mut ::core::ffi::c_void,
        flags: ::core::ffi::c_int,
    ) -> bool;

    pub fn range_table_entry_walker(
        rte: *mut super::RangeTblEntry,
        walker: TreeWalker,
        context: *mut ::core::ffi::c_void,
        flags: ::core::ffi::c_int,
    ) -> bool;

    pub fn range_table_walker(
        rtable: *mut super::List,
        walker: TreeWalker,
        context: *mut ::core::ffi::c_void,
        flags: ::core::ffi::c_int,
    ) -> bool;

    pub fn expression_tree_walker(
        node: *mut super::Node,
        walker: TreeWalker,
        context: *mut ::core::ffi::c_void,
    ) -> bool;

    pub fn raw_expression_tree_walker(
        node: *mut super::Node,
        walker: TreeWalker,
        context: *mut ::core::ffi::c_void,
    ) -> bool;

    pub fn expression_tree_mutator(
        node: *mut super::Node,
        mutator: TreeMutator,
        context: *mut ::core::ffi::c_void,
    ) -> *mut super::Node;

    pub fn query_tree_mutator(
        query: *mut super::Query,
        mutator: TreeMutator,
        context: *mut ::core::ffi::c_void,
        flags: ::core::ffi::c_int,
    ) -> *mut super::Query;

    pub fn range_table_mutator(
        rtable: *mut super::List,
        mutator: TreeMutator,
        context: *mut ::core::ffi::c_void,
        flags: ::core::ffi::c_int,
    ) -> *mut super::List;

    pub fn query_or_expression_tree_mutator(
        node: *mut super::Node,
        mutator: TreeMutator,
        context: *mut ::core::ffi::c_void,
        flags: ::core::ffi::c_int,
    ) -> *mut super::Node;
}

/// A tree walker callback, as each version's walker functions take it.
type TreeWalker = ::core::option::Option<
    unsafe extern "C-unwind" fn(*mut super::Node, *mut ::core::ffi::c_void) -> bool,
>;

/// A tree mutator callback, as each version's mutator functions take it.
type TreeMutator = ::core::option::Option<
    unsafe extern "C-unwind" fn(*mut super::Node, *mut ::core::ffi::c_void) -> *mut super::Node,
>;

/// Defines a PG16+ function with the name and signature of a PG15 walker or mutator that calls
/// the `*_impl` function its PG16+ macro expands to.
macro_rules! tree_walker_wrappers {
    ($($name:ident => $impl_name:ident($($arg:ident: $ty:ty),*) -> $ret:ty;)*) => {$(
        #[doc = concat!("Call `", stringify!($impl_name), "`, which the PG16+ `", stringify!($name), "` macro expands to.")]
        ///
        /// # Safety
        ///
        #[doc = concat!("The same as PostgreSQL's `", stringify!($name), "`: the tree must be live, and the callback must accept the context.")]
        #[cfg(not(feature = "pg15"))]
        #[inline]
        pub unsafe fn $name($($arg: $ty),*) -> $ret {
            // SAFETY: the caller upholds the `*_impl` function's contract, and its callback
            // type is the one this function takes.
            unsafe { super::$impl_name($($arg),*) }
        }
    )*};
}

tree_walker_wrappers! {
    expression_tree_walker => expression_tree_walker_impl(node: *mut super::Node, walker: TreeWalker, context: *mut ::core::ffi::c_void) -> bool;
    raw_expression_tree_walker => raw_expression_tree_walker_impl(node: *mut super::Node, walker: TreeWalker, context: *mut ::core::ffi::c_void) -> bool;
    query_tree_walker => query_tree_walker_impl(query: *mut super::Query, walker: TreeWalker, context: *mut ::core::ffi::c_void, flags: ::core::ffi::c_int) -> bool;
    range_table_walker => range_table_walker_impl(rtable: *mut super::List, walker: TreeWalker, context: *mut ::core::ffi::c_void, flags: ::core::ffi::c_int) -> bool;
    range_table_entry_walker => range_table_entry_walker_impl(rte: *mut super::RangeTblEntry, walker: TreeWalker, context: *mut ::core::ffi::c_void, flags: ::core::ffi::c_int) -> bool;
    query_or_expression_tree_walker => query_or_expression_tree_walker_impl(node: *mut super::Node, walker: TreeWalker, context: *mut ::core::ffi::c_void, flags: ::core::ffi::c_int) -> bool;
    planstate_tree_walker => planstate_tree_walker_impl(planstate: *mut super::PlanState, walker: ::core::option::Option<unsafe extern "C-unwind" fn(*mut super::PlanState, *mut ::core::ffi::c_void) -> bool>, context: *mut ::core::ffi::c_void) -> bool;
    expression_tree_mutator => expression_tree_mutator_impl(node: *mut super::Node, mutator: TreeMutator, context: *mut ::core::ffi::c_void) -> *mut super::Node;
    query_tree_mutator => query_tree_mutator_impl(query: *mut super::Query, mutator: TreeMutator, context: *mut ::core::ffi::c_void, flags: ::core::ffi::c_int) -> *mut super::Query;
    range_table_mutator => range_table_mutator_impl(rtable: *mut super::List, mutator: TreeMutator, context: *mut ::core::ffi::c_void, flags: ::core::ffi::c_int) -> *mut super::List;
    query_or_expression_tree_mutator => query_or_expression_tree_mutator_impl(node: *mut super::Node, mutator: TreeMutator, context: *mut ::core::ffi::c_void, flags: ::core::ffi::c_int) -> *mut super::Node;
}
