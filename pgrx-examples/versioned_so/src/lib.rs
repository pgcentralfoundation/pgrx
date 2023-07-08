use pgrx::prelude::*;

pgrx::pg_module_magic!();

#[pg_extern]
fn hello_versioned_so() -> &'static str {
    "Hello, versioned_so"
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

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
