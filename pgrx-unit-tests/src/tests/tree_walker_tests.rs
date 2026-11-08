//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgrx_unit_tests;
    use core::ffi::c_void;
    use core::ptr;
    use pgrx::prelude::*;

    #[pg_guard]
    extern "C-unwind" fn count_nodes(_node: *mut pg_sys::Node, context: *mut c_void) -> bool {
        // SAFETY: the test passes a pointer to a live usize as the context.
        unsafe { *context.cast::<usize>() += 1 };
        false
    }

    #[pg_guard]
    extern "C-unwind" fn copy_nodes(
        node: *mut pg_sys::Node,
        context: *mut c_void,
    ) -> *mut pg_sys::Node {
        // SAFETY: `node` is a live expression node that the mutator handed to this callback.
        unsafe { pg_sys::expression_tree_mutator(node, Some(copy_nodes), context) }
    }

    // The walkers and mutators have the same signature on every supported version, so this test
    // needs no `cfg`.
    #[pg_test]
    fn walk_and_mutate_a_list_of_constants() {
        // SAFETY: the list and its constants are live in the current memory context, and each
        // callback reads only the context this test passes it.
        unsafe {
            let list = pg_sys::lappend(ptr::null_mut(), pg_sys::makeBoolConst(true, false).cast());
            let list = pg_sys::lappend(list, pg_sys::makeBoolConst(false, false).cast());

            let mut visited = 0_usize;
            let stopped = pg_sys::expression_tree_walker(
                list.cast(),
                Some(count_nodes),
                (&raw mut visited).cast(),
            );
            assert!(!stopped);
            assert_eq!(visited, 2);

            let copy =
                pg_sys::expression_tree_mutator(list.cast(), Some(copy_nodes), ptr::null_mut())
                    .cast::<pg_sys::List>();
            assert!(!copy.is_null());
            assert_ne!(copy, list);
            assert_eq!((*copy).length, 2);
        }
    }
}
