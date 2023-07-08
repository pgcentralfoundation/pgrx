use pgrx::prelude::*;

#[cfg(any(test, feature = "pg_test"))]
#[pg_extern]
fn func_test_cfg() {}

#[cfg(feature = "nonexistent")]
#[pg_extern]
fn func_non_existent_cfg(t: NonexistentType) {}

#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgrx_tests;

    use pgrx::prelude::*;

    #[pg_test]
    fn test_cfg_exists() -> Result<(), spi::Error> {
        Spi::run("SELECT func_test_cfg();")
    }
}
