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

// This crate has no direct `postgres` dependency, so these names resolve only
// through the `pgrx_tests::postgres` re-export.
#[cfg(test)]
use pgrx_tests::postgres::{Client, Row};

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgrx_unit_tests;
    use pgrx::prelude::*;

    // Gives `client_from_reexported_postgres` a test function to run, which
    // starts the test server that `pgrx_tests::client()` connects to.
    #[pg_test]
    fn test_postgres_reexport_server() {}

    // A `#[pg_test]` body runs inside a backend, where `pgrx_tests::client()`
    // doesn't know the server's address. Read it from the server's settings
    // and connect through the re-exported crate instead.
    #[pg_test]
    fn test_postgres_reexport_from_pg_test() {
        use pgrx_tests::postgres::{Config, NoTls};

        let setting = |sql: &str| Spi::get_one::<String>(sql).unwrap().unwrap();
        let socket_dir = setting("SHOW unix_socket_directories");
        let host = match socket_dir.split(',').next().map(str::trim) {
            Some(dir) if !cfg!(windows) && !dir.is_empty() => dir.to_string(),
            _ => "127.0.0.1".to_string(),
        };
        let mut client = Config::new()
            .host(&host)
            .port(setting("SHOW port").parse().unwrap())
            .user(&setting("SELECT current_user::text"))
            .dbname(&setting("SELECT current_database()::text"))
            .connect(NoTls)
            .expect("connect to the test server");

        // The client is a separate backend, so it can't see this test's
        // uncommitted table.
        Spi::run("CREATE TABLE tests.postgres_reexport_uncommitted ()").unwrap();
        let row = client
            .query_one("SELECT to_regclass('tests.postgres_reexport_uncommitted') IS NULL", &[])
            .unwrap();
        assert!(row.get::<_, bool>(0));
    }
}

#[cfg(test)]
#[test]
fn client_from_reexported_postgres() -> eyre::Result<()> {
    pgrx_tests::run_test(
        "test_postgres_reexport_server",
        None,
        crate::pg_test::postgresql_conf_options(),
    )?;

    let (mut client, _session_id): (Client, String) = pgrx_tests::client()?;
    let row: Row = client.query_one("SELECT $1::int4 + 1", &[&41_i32])?;
    assert_eq!(row.get::<_, i32>(0), 42);
    Ok(())
}
