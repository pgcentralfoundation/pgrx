mod complex;
mod fixed_size;
mod generic_enum;
mod hexint;
mod hstore_clone;
mod ordered;
mod rust_enum;

use pgrx::prelude::*;

pgrx::pg_module_magic!();

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
