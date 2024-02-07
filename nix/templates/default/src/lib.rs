//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use pgrx::prelude::*;

::pgrx::pg_module_magic!();

#[pg_extern]
fn hello() -> &'static str {
    "Hello"
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_hello() {
        assert_eq!("Hello", crate::hello());
    }
}

#[cfg(test)]
pub mod pg_test {
    use once_cell::sync::Lazy;
    use pgrx_utils::pg_config::Pgrx;
    use tempfile::tempdir;

    static WORK_DIR: Lazy<String> = Lazy::new(|| {
        let work_dir = tempdir().expect("Couldn't create tempdir");
        format!("plrust.work_dir='{}'", work_dir.path().display())
    });
    static PG_CONFIG: Lazy<String> = Lazy::new(|| {
        let pgrx_config = Pgrx::from_config().unwrap();
        let version = format!("pg{}", pgrx::pg_sys::get_pg_major_version_num());
        let pg_config = pgrx_config.get(&version).unwrap();
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
