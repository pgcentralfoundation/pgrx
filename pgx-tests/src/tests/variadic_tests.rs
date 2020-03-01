mod test {
    use pgx::*;

    #[pg_extern]
    fn func_with_variadic_array_args(_field: &str, values: VariadicArray<&str>) -> String {
        values.get(0).unwrap().unwrap().to_string()
    }
}

#[cfg(any(test, feature = "pg_test"))]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;

    use pgx::*;

    #[pg_test]
    fn test_func_with_variadic_array_args() {
        let result = Spi::get_one::<&str>(
            "SELECT test.func_with_variadic_array_args('test', 'a', 'b', 'c');",
        )
        .expect("didn't get SPI result");
        assert_eq!(result, "a");
    }
}
