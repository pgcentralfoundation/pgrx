mod framework;
mod tests;

pub use framework::*;

#[cfg(test)]
pub fn pg_test_setup(_options: Vec<&str>) {
    // noop
}

#[cfg(test)]
pub fn pg_test_postgresql_conf_options() -> Vec<&'static str> {
    vec![]
}
