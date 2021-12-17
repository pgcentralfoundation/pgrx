use pgx::*;

pg_module_magic!();

#[pg_extern]
fn hello() -> &'static str {
    "Hello"
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::*;

    #[pg_test]
    fn test_hello() {
        assert_eq!("Hello", crate::hello());
    }
}

#[cfg(test)]
pub mod pg_test {
    use once_cell::sync::Lazy;
    use pgx_utils::pg_config::Pgx;
    use tempfile::tempdir;

    static WORK_DIR: Lazy<String> = Lazy::new(|| {
        let work_dir = tempdir().expect("Couldn't create tempdir");
        format!("plrust.work_dir='{}'", work_dir.path().display())
    });
    static PG_CONFIG: Lazy<String> = Lazy::new(|| {
        let pgx_config = Pgx::from_config().unwrap();
        let version = format!("pg{}", pgx::pg_sys::get_pg_major_version_num());
        let pg_config = pgx_config.get(&version).unwrap();
        let path = pg_config.path().unwrap();
        format!("plrust.pg_config='{}'", path.as_path().display())
    });

    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    pub fn postgresql_conf_options() -> Vec<&'static str> {
        vec![&*WORK_DIR, &*PG_CONFIG]
    }
}
