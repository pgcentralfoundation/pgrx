#[cfg(any(test, feature = "pg_test"))]
#[pgx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;
    use pgx::prelude::*;
    use std::convert::Infallible;

    #[pg_extern]
    fn return_result() -> Result<i32, Infallible> {
        Ok(42)
    }

    #[pg_extern]
    fn return_some_optional_result() -> Result<Option<i32>, Infallible> {
        Ok(Some(42))
    }

    #[pg_extern]
    fn return_none_optional_result() -> Result<Option<i32>, Infallible> {
        Ok(None)
    }

    #[pg_extern]
    fn return_some_error() -> Result<i32, Box<dyn std::error::Error>> {
        Err("error!".into())
    }

    #[pg_extern]
    fn return_eyre_result() -> eyre::Result<i32> {
        Ok(42)
    }

    #[pg_extern]
    fn return_eyre_result_error() -> eyre::Result<i32> {
        Err(eyre::eyre!("error!"))
    }

    #[pg_test(error = "No such file or directory (os error 2)")]
    fn test_return_io_error() -> Result<(), std::io::Error> {
        std::fs::read("/tmp/i-sure-hope-this-doest-exist.pgx-tests::test_result_result").map(|_| ())
    }

    #[pg_test]
    fn test_return_result() {
        Spi::get_one::<i32>("SELECT tests.return_result()").expect("SPI returned NULL");
    }

    #[pg_test]
    fn test_return_some_optional_result() {
        assert_eq!(Some(42), Spi::get_one::<i32>("SELECT tests.return_some_optional_result()"));
    }

    #[pg_test]
    fn test_return_none_optional_result() {
        assert_eq!(None, Spi::get_one::<i32>("SELECT tests.return_none_optional_result()"));
    }

    #[pg_test(error = "error!")]
    fn test_return_some_error() {
        Spi::get_one::<i32>("SELECT tests.return_some_error()").expect("SPI returned NULL");
    }

    #[pg_test]
    fn test_return_eyre_result() {
        Spi::get_one::<i32>("SELECT tests.return_eyre_result()").expect("SPI returned NULL");
    }

    #[pg_test(error = "error!")]
    fn test_return_eyre_result_error() {
        Spi::get_one::<i32>("SELECT tests.return_eyre_result_error()").expect("SPI returned NULL");
    }

    #[pg_test(error = "got proper sql errorcode")]
    fn test_proper_sql_errcode() -> Option<i32> {
        PgTryBuilder::new(|| Spi::get_one::<i32>("SELECT tests.return_eyre_result_error()"))
            .catch_when(PgSqlErrorCode::ERRCODE_DATA_EXCEPTION, |_| {
                panic!("got proper sql errorcode")
            })
            .execute()
    }
}
