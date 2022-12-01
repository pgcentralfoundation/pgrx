/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

#[pgx::pg_schema]
mod test {
    use pgx::prelude::*;
    use pgx::VariadicArray;

    #[pg_extern]
    fn func_with_variadic_array_args(_field: &str, values: VariadicArray<&str>) -> String {
        values.get(0).unwrap().unwrap().to_string()
    }
}

#[cfg(any(test, feature = "pg_test"))]
#[pgx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;

    use pgx::prelude::*;

    #[pg_test]
    fn test_func_with_variadic_array_args() {
        let result = Spi::get_one::<String>(
            "SELECT test.func_with_variadic_array_args('test', 'a', 'b', 'c');",
        )
        .expect("didn't get SPI result");
        assert_eq!(result, String::from("a"));
    }
}
