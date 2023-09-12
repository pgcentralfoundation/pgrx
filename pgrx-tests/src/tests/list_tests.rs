//LICENSE Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    use crate as pgrx_tests;
    use pgrx::list::List;
    use pgrx::prelude::*;

    #[pg_test]
    fn list_length_10() {
        let mut list = List::Nil;
        // Make sure the list length grows correctly:
        for i in 0..10 {
            unsafe {
                list.unstable_push_in_context(i, pg_sys::CurrentMemoryContext);
                assert_eq!(i as usize + 1, list.len());
            }
        }
    }

    #[pg_test]
    fn list_length_1000() {
        let mut list = List::Nil;
        // Make sure the list length grows correctly:
        for i in 0..1000 {
            unsafe {
                list.unstable_push_in_context(i, pg_sys::CurrentMemoryContext);
                assert_eq!(i as usize + 1, list.len());
            }
        }
    }

    #[pg_test]
    fn list_length_drained() {
        let mut list = List::Nil;
        for i in 0..100 {
            unsafe {
                list.unstable_push_in_context(i, pg_sys::CurrentMemoryContext);
            }
        }

        // Want to make sure the list length updates properly in the three major drain cases:
        // from start of list, from inside the middle of the list, and from middle to tail.
        let _ = list.drain(0..10);
        assert_eq!(90, list.len());
        let _ = list.drain(10..30);
        assert_eq!(70, list.len());
        let _ = list.drain(50..);
        assert_eq!(50, list.len());
    }
}
