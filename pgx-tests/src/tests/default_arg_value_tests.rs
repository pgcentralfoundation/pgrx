// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.


use pgx::*;

#[pg_extern]
fn default_argument(a: default!(i32, 99)) -> i32 {
    a
}

#[pg_extern]
fn option_default_argument(a: Option<default!(&str, "NULL")>) -> &str {
    match a {
        Some(a) => a,
        None => "got default of null",
    }
}

#[cfg(any(test, feature = "pg_test"))]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;

    use pgx::*;

    #[test]
    fn make_idea_happy() {}

    #[pg_test]
    fn test_default_argument() {
        let result =
            Spi::get_one::<i32>("SELECT default_argument();").expect("didn't get SPI result");
        assert_eq!(result, 99);
    }

    #[pg_test]
    fn test_default_argument_specified() {
        let result =
            Spi::get_one::<i32>("SELECT default_argument(2);").expect("didn't get SPI result");
        assert_eq!(result, 2);
    }

    #[pg_test]
    fn test_option_default_argument() {
        let result = Spi::get_one::<&str>("SELECT option_default_argument();")
            .expect("didn't get SPI result");
        assert_eq!(result, "got default of null");
    }

    #[pg_test]
    fn test_option_default_argument_specified() {
        let result = Spi::get_one::<&str>("SELECT option_default_argument('test');")
            .expect("didn't get SPI result");
        assert_eq!(result, "test");
    }
}
