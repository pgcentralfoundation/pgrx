use pgx::*;

pg_module_magic!();

// This example works better in the test environment,
// more relealistically though, when using versioned .so 
// you'll end up with an older definition like:
//
// CREATE OR REPLACE FUNCTION hello_versioned_so() RETURNS text
// IMMUTABLE PARALLEL SAFE STRICT
// LANGUAGE C
// AS '$libdir/versioned_so-0.0.1', 'hello_versioned_so_wrapper';
extension_sql!(
    "\n\
    CREATE FUNCTION hello_versioned_so() RETURNS text \
    AS 'old definition' \
    LANGUAGE SQL \
    IMMUTABLE; \
    ",
    name = "boostrap",
    bootstrap,
);

#[pg_extern]
fn hello_versioned_so() -> &'static str {
    "Hello, versioned_so"
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::*;

    #[pg_test]
    fn test_hello_versioned_so() {
        assert_eq!("Hello, versioned_so", crate::hello_versioned_so());
    }

}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
