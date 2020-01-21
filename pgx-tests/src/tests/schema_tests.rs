mod test_schema {
    use pgx::*;

    #[pg_extern]
    fn func_in_diff_schema() {}
}

mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;

    use pgx::*;

    #[pg_test]
    fn test_in_different_schema() {
        Spi::run("SELECT test_schema.func_in_diff_schema();");
    }
}
