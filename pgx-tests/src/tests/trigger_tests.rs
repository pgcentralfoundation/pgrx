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
        #[error("Null NEW found")]
        NullNew,
        #[error("PgHeapTuple: {0}")]
        PgHeapTuple(#[from] pgx::heap_tuple::PgHeapTupleError),
        #[error("TryFromDatumError: {0}")]
        TryFromDatum(#[from] pgx::datum::TryFromDatumError),
        #[error("TryFromIntError: {0}")]
        TryFromInt(#[from] std::num::TryFromIntError),
        #[error("PgTrigger error: {0}")]
        PgTrigger(#[from] pgx::trigger_support::PgTriggerError),
    }

    #[pg_trigger]
    fn field_species_fox_to_bear(trigger: &pgx::PgTrigger) -> Result<
        Option<PgHeapTuple<'_, impl WhoAllocated<pgx::pg_sys::HeapTupleData>>>,
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
    fn add_field_boopers(trigger: &pgx::PgTrigger) -> Result<
        Option<PgHeapTuple<'_, impl WhoAllocated<pgx::pg_sys::HeapTupleData>>>,
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

    #[pg_trigger]
    fn intercept_bears(trigger: &pgx::PgTrigger) -> Result<
        Option<PgHeapTuple<'_, impl WhoAllocated<pgx::pg_sys::HeapTupleData>>>,
        TriggerError
    > {
        let new = unsafe {
            trigger.new()?
        }.ok_or(TriggerError::NullNew)?;

        for index in 1..(new.len() + 1) {
            if let Some(val) = new.get_by_index::<&str>(index.try_into()?)? {
                if val == "Bear" {
                    // We intercepted a bear! Avoid this update, return `OLD` instead.
                    let old = unsafe {
                        trigger.old()?
                    }.ok_or(TriggerError::NullOld)?;
                    return Ok(Some(old));
                }
            }
        }

        Ok(Some(new))
    }

    #[pg_test]
    fn before_update_skip() {
        Spi::run(r#"
            CREATE TABLE tests.before_update_skip (title TEXT)
        "#);

        Spi::run(r#"
            CREATE TRIGGER add_field
                BEFORE UPDATE ON tests.before_update_skip
                FOR EACH ROW
                EXECUTE PROCEDURE tests.intercept_bears()
        "#);

        Spi::run(r#"
            INSERT INTO tests.before_update_skip (title)
                VALUES ('Fox')
        "#);
        Spi::run(r#"
            UPDATE tests.before_update_skip SET title = 'Bear'
                WHERE title = 'Fox'
        "#);

        let retval = Spi::get_one::<&str>(
            "SELECT title FROM tests.before_update_skip;",
        ).expect("SQL select failed");
        assert_eq!(retval, "Fox");
    }



    #[pg_trigger]
    fn inserts_trigger_metadata(trigger: &pgx::PgTrigger) -> Result<
        Option<PgHeapTuple<'_, impl WhoAllocated<pgx::pg_sys::HeapTupleData>>>,
        TriggerError
    > {
        let new = unsafe {
            trigger.new()?
        };
        let current = if let Some(new) = unsafe { trigger.new()? } {
            new
        } else {
            unsafe { trigger.old()? }.ok_or(TriggerError::NullOld)?
        };
        let mut current = current.into_owned();

        let trigger_name = unsafe { trigger.name()? };
        current.set_by_name("trigger_name", trigger_name)?;

        let trigger_when = trigger.when()?.to_string();
        current.set_by_name("trigger_when", trigger_when)?;

        let trigger_level = trigger.level().to_string();
        current.set_by_name("trigger_level", trigger_level)?;

        let trigger_op = trigger.op()?.to_string();
        current.set_by_name("trigger_op", trigger_op)?;

        let trigger_relid = unsafe { trigger.relid() };
        current.set_by_name("trigger_relid", trigger_relid)?;

        let trigger_old_transition_table_name = unsafe { trigger.old_transition_table_name()? };
        current.set_by_name("trigger_old_transition_table_name", trigger_old_transition_table_name)?;

        let trigger_new_transition_table_name = unsafe { trigger.new_transition_table_name()? };
        current.set_by_name("trigger_new_transition_table_name", trigger_new_transition_table_name)?;

        let trigger_table_name = unsafe { trigger.table_name()? };
        current.set_by_name("trigger_table_name", trigger_table_name)?;

        let trigger_table_schema = unsafe { trigger.table_schema()? };
        current.set_by_name("trigger_table_schema", trigger_table_schema)?;

        let trigger_extra_args = unsafe { trigger.extra_args()? };
        current.set_by_name("trigger_extra_args", trigger_extra_args)?;
        
        Ok(Some(current))
    }

    #[pg_test]
    fn before_insert_metadata() {
        Spi::run(r#"
            CREATE TABLE tests.before_insert_trigger_metadata (
                marker TEXT,
                trigger_name TEXT,
                trigger_when TEXT,
                trigger_level TEXT,
                trigger_op TEXT,
                trigger_relid OID,
                trigger_old_transition_table_name TEXT,
                trigger_new_transition_table_name TEXT,
                trigger_table_name TEXT,
                trigger_table_schema TEXT,
                trigger_extra_args TEXT[]
            )
        "#);

        Spi::run(r#"
            CREATE TRIGGER insert_trigger_metadata
                BEFORE INSERT ON tests.before_insert_trigger_metadata
                FOR EACH ROW
                EXECUTE PROCEDURE tests.inserts_trigger_metadata('Bears', 'Dogs')
        "#);

        Spi::run(r#"
            INSERT INTO tests.before_insert_trigger_metadata (marker)
                VALUES ('Fox')
        "#);

        let marker = Spi::get_one::<&str>("SELECT marker FROM tests.before_insert_trigger_metadata;");
        let trigger_name = Spi::get_one::<&str>("SELECT trigger_name FROM tests.before_insert_trigger_metadata;");
        let trigger_when = Spi::get_one::<&str>("SELECT trigger_when FROM tests.before_insert_trigger_metadata;");
        let trigger_level = Spi::get_one::<&str>("SELECT trigger_level FROM tests.before_insert_trigger_metadata;");
        let trigger_op = Spi::get_one::<&str>("SELECT trigger_op FROM tests.before_insert_trigger_metadata;");
        let trigger_relid = Spi::get_one::<pg_sys::Oid>("SELECT trigger_relid FROM tests.before_insert_trigger_metadata;");
        let trigger_old_transition_table_name = Spi::get_one::<&str>("SELECT trigger_old_transition_table_name FROM tests.before_insert_trigger_metadata;");
        let trigger_new_transition_table_name = Spi::get_one::<&str>("SELECT trigger_new_transition_table_name FROM tests.before_insert_trigger_metadata;");
        let trigger_table_name = Spi::get_one::<&str>("SELECT trigger_table_name FROM tests.before_insert_trigger_metadata;");
        let trigger_table_schema = Spi::get_one::<&str>("SELECT trigger_table_schema FROM tests.before_insert_trigger_metadata;");
        let trigger_extra_args = Spi::get_one::<Vec<String>>("SELECT trigger_extra_args FROM tests.before_insert_trigger_metadata;");
        
        assert_eq!(marker, Some("Fox"));
        assert_eq!(trigger_name, Some("insert_trigger_metadata"));
        assert_eq!(trigger_when, Some("BEFORE"));
        assert_eq!(trigger_level, Some("ROW"));
        assert_eq!(trigger_op, Some("INSERT"));
        assert!(trigger_relid.is_some());
        assert_eq!(trigger_old_transition_table_name, None);
        assert_eq!(trigger_new_transition_table_name, None);
        assert_eq!(trigger_table_name, Some("before_insert_trigger_metadata"));
        assert_eq!(trigger_table_schema, Some("tests"));
        assert_eq!(trigger_extra_args, Some(vec!["Bears".to_string(), "Dogs".to_string()]));
    }
}
