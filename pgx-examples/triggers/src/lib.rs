/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
use pgx::*;
use std::error::Error;

pg_module_magic!();

#[derive(thiserror::Error, Debug)]
enum TriggerError {
    #[error("Null OLD found")]
    NullOld,
    #[error("PgHeapTuple error: {0}")]
    PgHeapTuple(#[from] pgx::heap_tuple::PgHeapTupleError),
    #[error("TryFromDatumError error: {0}")]
    TryFromDatum(#[from] pgx::datum::TryFromDatumError),
    #[error("TryFromInt error: {0}")]
    TryFromInt(#[from] std::num::TryFromIntError),
}

#[pg_trigger]
fn trigger_example<'a>(trigger: &'a pgx::PgTrigger<'a>) -> Result<Option<PgHeapTuple<'a, impl WhoAllocated<pgx::pg_sys::HeapTupleData>>>, TriggerError> {
    let old = unsafe {
        trigger.old()?
    }.ok_or(TriggerError::NullOld)?;

    let mut current = old.into_owned();
    let index = 2.try_into()?;

    if current.get_by_index(index)? == Some("Fox") {
        current.set_by_index(index, "Bear")?;
    }

    Ok(Some(current))
}

/*
extension_sql!(r#"
CREATE TABLE test (
    id serial8 NOT NULL PRIMARY KEY,
    title varchar(50),
    description text,
    payload jsonb
);

CREATE TRIGGER test_trigger BEFORE INSERT ON test FOR EACH ROW EXECUTE PROCEDURE trigger_example();
INSERT INTO test (title, description, payload) VALUES ('the title', 'a description', '{"key": "value"}');
"#,
    name = "create_trigger",
);
*/

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgx::*;

    #[pg_test]
    fn test_insert() {
        Spi::run(
            r#"INSERT INTO test (title, description, payload) VALUES ('a different title', 'a different description', '{"key": "value"}')"#,
        );
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
