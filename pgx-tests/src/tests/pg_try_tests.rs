// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.


#[cfg(any(test, feature = "pg_test"))]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;

    use pgx::*;

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
