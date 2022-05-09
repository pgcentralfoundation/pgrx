

// #[pg_extern(sql = r#"
//     CREATE OR REPLACE FUNCTION "echo"(
//         "a" text[]
//     ) RETURNS text[]
//     STRICT
//     LANGUAGE c /* Rust */
//     AS 'MODULE_PATHNAME', '@FUNCTION_NAME@';
// "#)]

use pgx::*;

#[pg_extern]
fn internal_screaming() {
    pgx::log!("{}", unsafe { std::str::from_utf8_unchecked(&['a' as u8; 100000]) });
}

#[cfg(any(test, feature = "pg_test"))]
#[pgx::pg_schema]
mod tests {
    use pgx::*;
    #[allow(unused_imports)]
    use crate as pgx_tests;

    #[pg_test]
    fn test_internal_screaming() {
        Spi::run("SELECT internal_screaming();");
    }
}
