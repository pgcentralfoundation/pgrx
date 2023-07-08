mod framework;
#[cfg(any(test, feature = "pg_test"))]
mod tests;

pub use framework::*;

#[cfg(any(test, feature = "pg_test"))]
pgrx::pg_sql_graph_magic!();

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // noop
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        vec!["shared_preload_libraries='pgrx_tests'"]
    }
}
