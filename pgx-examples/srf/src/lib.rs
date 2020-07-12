use pgx::*;

pg_module_magic!();

#[pg_extern]
fn generate_series(
    start: i64,
    finish: i64,
    step: default!(i64, 1),
) -> impl std::iter::Iterator<Item = i64> {
    (start..=finish).step_by(step as usize)
}

#[pg_extern]
fn random_values(
    num_rows: i32,
) -> impl std::iter::Iterator<Item = (name!(index, i32), name!(value, f64))> {
    (1..=num_rows).map(|i| (i, rand::random::<f64>()))
}

#[pg_extern]
fn vector_of_static_values() -> impl std::iter::Iterator<Item = &'static str> {
    let values = vec!["Brandy", "Sally", "Anchovy"];
    values.into_iter()
}

#[cfg(any(test, feature = "pg_test"))]
mod tests {
    use pgx::*;

    #[pg_test]
    fn test_it() {
        // do testing here.
        //
        // #[pg_test] functions run *inside* Postgres and have access to all Postgres internals
        //
        // Normal #[test] functions do not
        //
        // In either case, they all run in parallel
    }
}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
