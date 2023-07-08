use pgrx::prelude::*;

#[pg_extern(name = "renamed_func")]
fn func_to_rename() {}

#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgrx_tests;

    use pgrx::prelude::*;

    #[pg_test]
    fn renamed_func() {
        Spi::run("SELECT renamed_func();").expect("SPI failed");
    }
}
