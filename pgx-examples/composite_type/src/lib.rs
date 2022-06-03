/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
use pgx::{*, pg_sys::FunctionCallInfo};

pg_module_magic!();

#[pg_extern]
fn gets_name_field(
    value: pgx::composite_type!("Bear"),
    fcinfo: pgx::pg_sys::FunctionCallInfo,
) -> pgx::composite_type!("Buffalo") {
    value
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use crate::IntegerAvgState;
    use pgx::*;

    #[pg_test]
    fn test_gets_name_field() {
        Spi::run(
            r#"
            CREATE TYPE composite AS (
                name TEXT,
                scritches INT
            )
        "#,
        );

        let retval = Spi::get_one::<&str>("
            SELECT gets_name_field(ROW('Nami', 0)::composite)
        ").expect("SQL select failed");
        assert_eq!(retval, 0);
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
