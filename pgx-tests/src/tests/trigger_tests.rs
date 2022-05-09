/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

#[cfg(any(test, feature = "pg_test"))]
#[pgx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;
    use pgx::*;

    #[derive(thiserror::Error, Debug)]
    enum TriggerError {
        #[error("Null OLD found")]
        NullOld,
        #[error("PgHeapTuple: {0}")]
        PgHeapTuple(#[from] pgx::heap_tuple::PgHeapTupleError),
        #[error("TryFromDatumError: {0}")]
        TryFromDatum(#[from] pgx::datum::TryFromDatumError),
        #[error("TryFromIntError: {0}")]
        TryFromInt(#[from] std::num::TryFromIntError),
    }

    #[pg_trigger]
    fn field_species_fox_to_bear<'a>(trigger: &'a pgx::PgTrigger<'a>) -> Result<
        Option<PgHeapTuple<'a, impl WhoAllocated<pgx::pg_sys::HeapTupleData>>>,
        TriggerError
    > {
        let old = unsafe {
            trigger.old()?
        }.ok_or(TriggerError::NullOld)?;

        let mut current = old.into_owned();
        let field = "species";

        if current.get_by_name(field)? == Some("Fox") {
            current.set_by_name(field, "Bear")?;
        }

        Ok(Some(current))
    }

    #[pg_test]
    fn before_insert_field_update() {
        Spi::run(r#"
            CREATE TABLE tests.before_insert_field_update (species TEXT)
        "#);

        Spi::run(r#"
            CREATE TRIGGER foxes_to_bears
                BEFORE INSERT ON tests.before_insert_field_update
                FOR EACH ROW
                EXECUTE PROCEDURE tests.field_species_fox_to_bear()
        "#);

        Spi::run(r#"
            INSERT INTO tests.before_insert_field_update (species)
                VALUES ('Fox')
        "#);

        let retval = Spi::get_one::<&str>(
            "SELECT species FROM tests.before_insert_field_update;",
        ).expect("SQL select failed");
        assert_eq!(retval, "Bear");
    }

    #[pg_trigger]
    fn add_field_boopers<'a>(trigger: &'a pgx::PgTrigger<'a>) -> Result<
        Option<PgHeapTuple<'a, impl WhoAllocated<pgx::pg_sys::HeapTupleData>>>,
        TriggerError
    > {
        let old = unsafe {
            trigger.old()?
        }.ok_or(TriggerError::NullOld)?;

        let mut current = old.into_owned();
        let field = "booper";

        if current.get_by_name(field)? == Option::<&str>::None {
            current.set_by_name(field, "Swooper")?;
        }

        Ok(Some(current))
    }

    #[pg_test]
    fn before_insert_add_field() {
        Spi::run(r#"
            CREATE TABLE tests.before_insert_add_field (name TEXT, booper TEXT)
        "#);

        Spi::run(r#"
            CREATE TRIGGER add_field
                BEFORE INSERT ON tests.before_insert_add_field
                FOR EACH ROW
                EXECUTE PROCEDURE tests.add_field_boopers()
        "#);

        Spi::run(r#"
            INSERT INTO tests.before_insert_add_field (name)
                VALUES ('Nami')
        "#);

        let retval = Spi::get_one::<&str>(
            "SELECT booper FROM tests.before_insert_add_field;",
        ).expect("SQL select failed");
        assert_eq!(retval, "Swooper");
    }
}
