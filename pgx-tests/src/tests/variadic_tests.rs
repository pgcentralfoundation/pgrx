use pgx::*;

#[pg_extern]
fn func_with_variadic_args(a: variadic!(Array<i32>)) -> i32 {
    let datum = a.get(0).unwrap();
    datum.unwrap()
}

mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;

    use pgx::*;

    #[pg_test]
    fn test_func_with_variadic_args() {
        let result = Spi::get_one::<i32>("SELECT func_with_variadic_args(1, 2, 3, 4, 5);")
            .expect("didn't get SPI result");
        assert_eq!(result, 1);
    }
}
