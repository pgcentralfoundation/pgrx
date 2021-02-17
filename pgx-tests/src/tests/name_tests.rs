use pgx::*;

#[pg_extern(name="renamed_func")]
fn func_to_rename() {}

#[cfg(any(test, feature = "pg_test"))]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;

    use pgx::*;

    #[pg_test]
    fn renamed_func() {
        Spi::run("SELECT renamed_func();");
    }
}