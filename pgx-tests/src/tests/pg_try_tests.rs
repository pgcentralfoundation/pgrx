/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use pgx::*;

// if our Postgres ERROR and Rust panic!() handling is incorrect, this little bit of useless code
// will crash postgres.  If things are correct it'll simply raise an ERROR saying "panic in walker".
#[pg_extern]
fn crash() {
    unsafe {
        let mut node = PgList::<pg_sys::Node>::new();
        node.push(PgList::<pg_sys::Node>::new().into_pg() as *mut pg_sys::Node);

        pg_sys::raw_expression_tree_walker(
            node.into_pg() as *mut pg_sys::Node,
            Some(walker),
            std::ptr::null_mut(),
        );
    }
}

#[pg_guard]
extern "C" fn walker() -> bool {
    panic!("panic in walker");
}

#[cfg(any(test, feature = "pg_test"))]
#[pgx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;

    use pgx::*;

    #[pg_test(error = "panic in walker")]
    fn test_panic_in_extern_c_fn() {
        Spi::get_one::<()>("SELECT crash()");
    }

    #[pg_test]
    fn test_pg_try_unwrap_no_error() {
        let result = pg_try(|| 42).unwrap();
        assert_eq!(42, result)
    }

    #[pg_test(error = "unwrapped a panic")]
    fn test_pg_try_unwrap_with_error() {
        pg_try(|| panic!("unwrapped a panic")).unwrap();
    }

    #[pg_test]
    fn test_pg_try_unwrap_or_no_error() {
        let result = unsafe { pg_try(|| 42).unwrap_or(99) };
        assert_eq!(42, result);
    }

    #[pg_test]
    fn test_pg_try_unwrap_or_with_error() {
        let result = unsafe { pg_try(|| panic!("unwrapped a panic")).unwrap_or(99) };
        assert_eq!(99, result);
    }

    #[pg_test]
    fn test_pg_try_unwrap_or_else_no_error() {
        let result = unsafe { pg_try(|| 42).unwrap_or_else(|| 99) };
        assert_eq!(42, result);
    }

    #[pg_test]
    fn test_pg_try_unwrap_or_else_with_error() {
        let result = unsafe { pg_try(|| panic!("unwrapped a panic")).unwrap_or_else(|| 99) };
        assert_eq!(99, result);
    }

    #[pg_test(error = "panic in catch")]
    fn test_pg_try_unwrap_or_else_with_nested_error() {
        unsafe {
            pg_try(|| panic!("unwrapped a panic")).unwrap_or_else(|| panic!("panic in catch"))
        };
    }

    #[pg_test]
    fn test_pg_try_unwrap_or_rethrow_no_error() {
        let result = pg_try(|| 42).unwrap_or_rethrow(|| ());
        assert_eq!(42, result);
    }

    #[pg_test(error = "rethrow a panic")]
    fn test_pg_try_unwrap_or_rethrow_with_error() {
        pg_try(|| panic!("rethrow a panic")).unwrap_or_rethrow(|| ());
    }

    #[pg_test(error = "panic in rethrow")]
    fn test_pg_try_unwrap_or_rethrow_with_error_in_rethrow() {
        pg_try(|| panic!("rethrow a panic")).unwrap_or_rethrow(|| panic!("panic in rethrow"));
    }
}
