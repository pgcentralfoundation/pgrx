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

        let _ = list.drain(100..200);
        assert_eq!(900, list.len());
        let _ = list.drain(500..);
        assert_eq!(500, list.len());
    }
}
