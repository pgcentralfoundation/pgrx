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
    fn list_length() {
        let mut list = List::Nil;
        for i in 0..1000 {
            unsafe {
                assert_eq!(i as usize, list.len());
                list.unstable_push_in_context(i, pg_sys::CurrentMemoryContext);
            }
        }

        // Want to make sure the list length updates properly in the three major drain cases:
        // from start of list, from inside the middle of the list, and from middle to tail.
        let _ = list.drain(0..100);
        assert_eq!(900, list.len());
        let _ = list.drain(100..300);
        assert_eq!(700, list.len());
        let _ = list.drain(500..);
        assert_eq!(500, list.len());
    }
}
