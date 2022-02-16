// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use pgx::*;

#[pg_extern(immutable)]
fn returns_tuple_with_attributes() -> (
    name!(arg, String),
    name!(arg2, String),
) {
   ("hi".to_string(), "bye".to_string())
}

#[cfg(any(test, feature = "pg_test"))]
#[pgx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;

    use pgx::*;

    #[pg_extern(immutable)]
    fn is_immutable() {}

    #[pg_test]
    fn test_immutable() {
        let result = Spi::get_one::<bool>(
            "SELECT provolatile = 'i' FROM pg_proc WHERE proname = 'is_immutable'",
        )
        .expect("failed to get SPI result");
        assert!(result)
    }

    
    // Ensures `@FUNCTION_NAME@` is handled.
    #[pg_extern(sql = r#"
        CREATE OR REPLACE FUNCTION tests."overridden_sql_with_fn_name"() RETURNS void
        STRICT
        LANGUAGE c /* Rust */
        AS 'MODULE_PATHNAME', '@FUNCTION_NAME@';
    "#)]
    fn overridden_sql_with_fn_name() -> bool {
        true
    }

    #[pg_test]
    fn test_overridden_sql_with_fn_name() {
        let result = Spi::get_one::<bool>(
            r#"SELECT tests."overridden_sql_with_fn_name"()"#,
        )
        .expect("failed to get SPI result");
        assert!(result)
    }
}
