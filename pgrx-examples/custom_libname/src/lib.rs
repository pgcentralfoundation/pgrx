use pgrx::prelude::*;

pgrx::pg_module_magic!();

#[pg_extern]
fn hello_custom_libname() -> &'static str {
    "Hello, custom_libname"
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_hello_custom_libname() {
        assert_eq!("Hello, custom_libname", crate::hello_custom_libname());
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
