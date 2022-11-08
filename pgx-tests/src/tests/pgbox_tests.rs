/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
#[cfg(any(test, feature = "pg_test"))]
#[pgx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;
    use pgx::prelude::*;
    use pgx::{AllocatedByRust, PgMemoryContexts};

    #[pg_test]
    fn pgbox_alloc() {
        let mut ptr: PgBox<i32, AllocatedByRust> = PgBox::<i32>::alloc();
        // ptr is uninitialized data!!! This is dangerous to read from!!!
        *ptr = 5;

        assert_eq!(*ptr, 5);
    }

    #[pg_test]
    fn pgbox_alloc0() {
        let mut ptr: PgBox<i32, AllocatedByRust> = PgBox::<i32>::alloc0();

        assert_eq!(*ptr, 0);

        *ptr = 5;

        assert_eq!(*ptr, 5);
    }

    #[pg_test]
    fn pgbox_new() {
        let ptr: PgBox<i32, AllocatedByRust> = PgBox::new(5);
        assert_eq!(*ptr, 5);

        let mut ptr: PgBox<Vec<i32>, AllocatedByRust> = PgBox::new(vec![]);
        assert_eq!(*ptr, Vec::<i32>::default());

        ptr.push(1);
        assert_eq!(*ptr, vec![1]);

        ptr.push(2);
        assert_eq!(*ptr, vec![1, 2]);

        ptr.push(3);
        assert_eq!(*ptr, vec![1, 2, 3]);

        let drained = ptr.drain(..).collect::<Vec<_>>();
        assert_eq!(drained, vec![1, 2, 3])
    }

    #[pg_test]
    fn pgbox_new_in_context() {
        let ptr: PgBox<i32, AllocatedByRust> =
            PgBox::new_in_context(5, PgMemoryContexts::CurrentMemoryContext);
        assert_eq!(*ptr, 5);

        let mut ptr: PgBox<Vec<i32>, AllocatedByRust> =
            PgBox::new_in_context(vec![], PgMemoryContexts::CurrentMemoryContext);
        assert_eq!(*ptr, Vec::<i32>::default());

        ptr.push(1);
        assert_eq!(*ptr, vec![1]);

        ptr.push(2);
        assert_eq!(*ptr, vec![1, 2]);

        ptr.push(3);
        assert_eq!(*ptr, vec![1, 2, 3]);

        let drained = ptr.drain(..).collect::<Vec<_>>();
        assert_eq!(drained, vec![1, 2, 3])
    }
}
