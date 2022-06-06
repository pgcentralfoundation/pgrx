/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
use pgx::*;

pg_module_magic!();

extension_sql!(r#"
CREATE TYPE Dog AS (
    name TEXT,
    scritches INT
);
"#, name = "create_dog", bootstrap);

#[pg_extern]
fn gets_name_field(
    value: pgx::composite_type!("Dog"),
) -> Option<&str> {
    value.get_by_name("name").ok().unwrap_or_default()
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::*;

    #[pg_test]
    fn test_gets_name_field() {
        let retval = Spi::get_one::<&str>("
            SELECT gets_name_field(ROW('Nami', 0)::Dog)
        ").expect("SQL select failed");
        assert_eq!(retval, "Nami");
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
