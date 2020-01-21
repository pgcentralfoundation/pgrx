mod test {
    use pgx::*;

    #[pg_extern]
    fn func_with_variadic_args(_field: &str, values: variadic!(Array<&str>)) -> String {
        values.get(0).unwrap().unwrap().to_string()
    }
}

mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;

    use pgx::*;

    #[pg_test]
    fn test_func_with_variadic_args() {
        let result =
            Spi::get_one::<&str>("SELECT test.func_with_variadic_args('test', 'a', 'b', 'c');")
                .expect("didn't get SPI result");
        assert_eq!(result, "a");
    }
}
