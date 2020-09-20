use pgx::*;

pg_module_magic!();

extension_sql!(
    r#"   

CREATE TABLE spi_example (
    id serial8 not null primary key,
    title text
);

INSERT INTO spi_example (title) VALUES ('This is a test');
INSERT INTO spi_example (title) VALUES ('Hello There!');
INSERT INTO spi_example (title) VALUES ('I like pudding');


"#
);

#[pg_extern]
fn spi_return_query(
) -> impl std::iter::Iterator<Item = (name!(oid, Option<pg_sys::Oid>), name!(name, Option<String>))>
{
    #[cfg(feature = "pg10")]
    let query = "SELECT oid, relname::text || '-pg10' FROM pg_class";
    #[cfg(feature = "pg11")]
    let query = "SELECT oid, relname::text || '-pg11' FROM pg_class";
    #[cfg(feature = "pg12")]
    let query = "SELECT oid, relname::text || '-pg12' FROM pg_class";

    let mut results = Vec::new();
    Spi::connect(|client| {
        client
            .select(query, None, None)
            .map(|row| (row.get_datum(1), row.get_datum(2)))
            .for_each(|tuple| results.push(tuple));
        Ok(Some(()))
    });

    results.into_iter()
}

#[pg_extern(immutable, parallel_safe)]
fn spi_query_random_id() -> Option<i64> {
    Spi::get_one("SELECT id FROM spi.spi_example ORDER BY random() LIMIT 1")
}

#[pg_extern]
fn spi_query_title(title: &str) -> Option<i64> {
    Spi::get_one_with_args(
        "SELECT id FROM spi.spi_example WHERE title = $1;",
        vec![(PgBuiltInOids::TEXTOID.oid(), title.into_datum())],
    )
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

        Ok(Some(tuptable.get_two::<i64, String>()))
    })
    .unwrap();

    if returned_id.is_some() {
        info!("id={}", returned_id.unwrap());
    }

    title
}

#[pg_extern]
fn spi_insert_title(title: &str) -> i64 {
    Spi::get_one_with_args(
        "INSERT INTO spi.spi_example(title) VALUES ($1) RETURNING id",
        vec![(PgBuiltInOids::TEXTOID.oid(), title.into_datum())],
    )
    .expect("INSERT into spi_example returned NULL")
}

#[pg_extern]
fn spi_insert_title2(
    title: &str,
) -> impl std::iter::Iterator<Item = (name!(id, Option<i64>), name!(title, Option<String>))> {
    let tuple = Spi::get_two_with_args(
        "INSERT INTO spi.spi_example(title) VALUES ($1) RETURNING id, title",
        vec![(PgBuiltInOids::TEXTOID.oid(), title.into_datum())],
    );

    vec![tuple].into_iter()
}

extension_sql!(
    r#"

CREATE TABLE foo ();


"#
);

#[cfg(any(test, feature = "pg_test"))]
mod tests {
    use crate::spi_query_by_id;
    use pgx::*;

    #[pg_test]
    fn test_spi_query_by_id_direct() {
        assert_eq!(Some("This is a test".to_string()), spi_query_by_id(1))
    }

    #[pg_test]
    fn test_spi_query_by_id_via_spi() {
        let result =
            Spi::get_one::<&str>("SELECT spi.spi_query_by_id(1)").expect("SPI result was NULL");

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
