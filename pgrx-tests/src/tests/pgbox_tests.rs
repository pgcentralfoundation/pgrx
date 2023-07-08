#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgrx_tests;
    use pgrx::prelude::*;
    use pgrx::AllocatedByRust;

    #[pg_test]
    fn pgbox_alloc() {
        let mut ptr: PgBox<i32, AllocatedByRust> = unsafe { PgBox::<i32>::alloc() };
        // ptr is uninitialized data!!! This is dangerous to read from!!!
        *ptr = 5;

        assert_eq!(*ptr, 5);
    }

    #[pg_test]
    fn pgbox_alloc0() {
        let mut ptr: PgBox<i32, AllocatedByRust> = unsafe { PgBox::<i32>::alloc0() };

        assert_eq!(*ptr, 0);

        *ptr = 5;

        assert_eq!(*ptr, 5);
    }
}
