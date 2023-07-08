/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
use pgrx::prelude::*;
use pgrx::WhoAllocated;

pgrx::pg_module_magic!();

#[derive(thiserror::Error, Debug)]
enum TriggerError {
    #[error("Null Trigger Tuple found")]
    NullTriggerTuple,
    #[error("PgHeapTuple error: {0}")]
    PgHeapTuple(#[from] pgrx::heap_tuple::PgHeapTupleError),
    #[error("TryFromDatumError error: {0}")]
    TryFromDatum(#[from] pgrx::datum::TryFromDatumError),
    #[error("TryFromInt error: {0}")]
    TryFromInt(#[from] std::num::TryFromIntError),
}

#[pg_trigger]
fn trigger_example<'a>(
    trigger: &'a pgrx::PgTrigger<'a>,
) -> Result<Option<PgHeapTuple<'a, impl WhoAllocated>>, TriggerError> {
    let mut new = trigger.new().ok_or(TriggerError::NullTriggerTuple)?.into_owned();
    let col_name = "title";

    if new.get_by_name(col_name)? == Some("Fox") {
        new.set_by_name(col_name, "Bear")?;
    }

    Ok(Some(new))
}

extension_sql!(
    r#"
CREATE TABLE test (
    id serial8 NOT NULL PRIMARY KEY,
    title varchar(50),
    description text,
    payload jsonb
);

CREATE TRIGGER test_trigger BEFORE INSERT ON test FOR EACH ROW EXECUTE PROCEDURE trigger_example();
INSERT INTO test (title, description, payload) VALUES ('Fox', 'a description', '{"key": "value"}');
"#,
    name = "create_trigger",
    requires = [trigger_example]
);

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    fn test_insert() -> Result<(), spi::Error> {
        Spi::run(
            r#"INSERT INTO test (title, description, payload) VALUES ('a different title', 'a different description', '{"key": "value"}')"#,
        )
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
