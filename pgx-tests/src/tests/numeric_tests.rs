// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.


#[cfg(any(test, feature = "pg_test"))]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;

    use pgx::*;
    use serde::Deserialize;

    #[pg_extern]
    fn return_an_i32_numeric() -> Numeric {
        32.into()
    }

    #[pg_extern]
    fn return_a_f64_numeric() -> Numeric {
        64.64646464f64.into()
    }

    #[pg_extern]
    fn return_a_u64_numeric() -> Numeric {
        std::u64::MAX.into()
    }

    #[pg_test]
    fn test_return_an_i32_numeric() {
        let result = Spi::get_one::<bool>("SELECT 32::numeric = tests.return_an_i32_numeric();")
            .expect("failed to get SPI result");
        assert!(result);
    }

    #[pg_test]
    fn test_return_a_f64_numeric() {
        let result =
            Spi::get_one::<bool>("SELECT 64.64646464::numeric = tests.return_a_f64_numeric();")
                .expect("failed to get SPI result");
        assert!(result);
    }

    #[pg_test]
    fn test_return_a_u64_numeric() {
        let result = Spi::get_one::<bool>(
            "SELECT 18446744073709551615::numeric = tests.return_a_u64_numeric();",
        )
        .expect("failed to get SPI result");
        assert!(result);
    }

    #[pg_test]
    fn test_deserialize_numeric() {
        use serde_json::json;
        Numeric::deserialize(&json!(42)).unwrap();
        Numeric::deserialize(&json!(42.4242)).unwrap();
        Numeric::deserialize(&json!(18446744073709551615u64)).unwrap();
        Numeric::deserialize(&json!("64.64646464")).unwrap();

        let error = Numeric::deserialize(&json!("foo"))
            .err()
            .unwrap()
            .to_string();
        assert_eq!("invalid Numeric value: foo", &error);
    }
}
