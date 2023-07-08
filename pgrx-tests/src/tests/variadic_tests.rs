#[pgrx::pg_schema]
mod test {
    use pgrx::prelude::*;
    use pgrx::VariadicArray;

    #[pg_extern]
    fn func_with_variadic_array_args(_field: &str, values: VariadicArray<&str>) -> String {
        values.get(0).unwrap().unwrap().to_string()
    }
}

#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgrx_tests;

    use pgrx::prelude::*;

    #[pg_test]
    fn test_func_with_variadic_array_args() {
        let result = Spi::get_one::<String>(
            "SELECT test.func_with_variadic_array_args('test', 'a', 'b', 'c');",
        );
        assert_eq!(result, Ok(Some("a".into())));
    }
}
