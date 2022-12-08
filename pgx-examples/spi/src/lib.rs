/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
use pgx::prelude::*;
use pgx::{info, IntoDatum};

pgx::pg_module_magic!();

extension_sql!(
    r#"

CREATE TABLE spi_example (
    id serial8 not null primary key,
    title text
);

INSERT INTO spi_example (title) VALUES ('This is a test');
INSERT INTO spi_example (title) VALUES ('Hello There!');
INSERT INTO spi_example (title) VALUES ('I like pudding');


"#,
    name = "create_sqi_example_table",
);

#[pg_extern]
fn spi_return_query(
) -> TableIterator<'static, (name!(oid, Option<pg_sys::Oid>), name!(name, Option<String>))> {
    #[cfg(feature = "pg11")]
    let query = "SELECT oid, relname::text || '-pg11' FROM pg_class";
    #[cfg(feature = "pg12")]
    let query = "SELECT oid, relname::text || '-pg12' FROM pg_class";
    #[cfg(feature = "pg13")]
    let query = "SELECT oid, relname::text || '-pg13' FROM pg_class";
    #[cfg(feature = "pg14")]
    let query = "SELECT oid, relname::text || '-pg14' FROM pg_class";
    #[cfg(feature = "pg15")]
    let query = "SELECT oid, relname::text || '-pg15' FROM pg_class";

    let results = Spi::connect(|client| {
        Ok::<_, ()>(
            client.select(query, None, None).map(|row| (row["oid"].value(), row[2].value())),
        )
    })
    .unwrap();

    TableIterator::new(results)
}

#[pg_extern(immutable, parallel_safe)]
fn spi_query_random_id() -> Option<i64> {
    Spi::get_one("SELECT id FROM spi.spi_example ORDER BY random() LIMIT 1").unwrap()
}

#[pg_extern]
fn spi_query_title(title: &str) -> Option<i64> {
    Spi::get_one_with_args(
        "SELECT id FROM spi.spi_example WHERE title = $1;",
        vec![(PgBuiltInOids::TEXTOID.oid(), title.into_datum())],
    )
    .unwrap()
}

#[pg_extern]
fn spi_query_by_id(id: i64) -> Option<String> {
    let (returned_id, title) = Spi::connect(|client| {
        let tuptable = client
            .select(
                "SELECT id, title FROM spi.spi_example WHERE id = $1",
                None,
                Some(vec![(PgBuiltInOids::INT8OID.oid(), id.into_datum())]),
            )
            .first();

        tuptable.get_two::<i64, String>()
    })
    .unwrap();

    info!("id={}", returned_id);

    Some(title)
}

#[pg_extern]
fn spi_insert_title(title: &str) -> i64 {
    Spi::get_one_with_args(
        "INSERT INTO spi.spi_example(title) VALUES ($1) RETURNING id",
        vec![(PgBuiltInOids::TEXTOID.oid(), title.into_datum())],
    )
    .unwrap()
}

#[pg_extern]
fn spi_insert_title2(
    title: &str,
) -> TableIterator<(name!(id, Option<i64>), name!(title, Option<String>))> {
    let tuple = Spi::get_two_with_args(
        "INSERT INTO spi.spi_example(title) VALUES ($1) RETURNING id, title",
        vec![(PgBuiltInOids::TEXTOID.oid(), title.into_datum())],
    )
    .unwrap();

    TableIterator::once(tuple)
}

extension_sql!(
    r#"

CREATE TABLE foo ();

"#,
    name = "create_foo_table"
);

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use crate::spi_query_by_id;
    use pgx::prelude::*;

    #[pg_test]
    fn test_spi_query_by_id_direct() {
        assert_eq!(Some("This is a test".to_string()), spi_query_by_id(1))
    }

    #[pg_test]
    fn test_spi_query_by_id_via_spi() {
        let result = Spi::get_one::<&str>("SELECT spi.spi_query_by_id(1)").unwrap();

        assert_eq!("This is a test", result)
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
