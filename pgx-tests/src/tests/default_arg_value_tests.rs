use pgx::*;

#[pg_extern]
fn default_argument(a: default!(i32, 99)) -> i32 {
    a
}

#[cfg(any(test, feature = "pg_test"))]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;

    use pgx::*;

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
}
