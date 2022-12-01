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
    use pgx::prelude::*;
    use pgx::{AllocatedByPostgres, AllocatedByRust, WhoAllocated};

    /// Test some various formats of trigger signature we expect to support
    ///
    /// These tests don't run, but they get built to SQL and compile checked.
    #[pgx::pg_schema]
    mod trigger_signature_compile_tests {
        use pgx::heap_tuple::PgHeapTupleError;
        use pgx::prelude::*;
        use pgx::{AllocatedByPostgres, AllocatedByRust};

        use super::*;

        #[pg_trigger]
        fn signature_standard(
            trigger: &pgx::PgTrigger,
        ) -> Result<PgHeapTuple<'_, impl WhoAllocated>, PgHeapTupleError> {
            Ok(trigger.current().unwrap().into_owned())
        }

        #[pg_trigger]
        fn signature_explicit_lifetimes<'a>(
            trigger: &'a pgx::PgTrigger,
        ) -> Result<PgHeapTuple<'a, impl WhoAllocated>, PgHeapTupleError> {
            Ok(trigger.current().unwrap().into_owned())
        }

        #[pg_trigger]
        fn signature_alloc_by_postgres(
            trigger: &pgx::PgTrigger,
        ) -> Result<PgHeapTuple<'_, AllocatedByPostgres>, PgHeapTupleError> {
            Ok(trigger.current().unwrap())
        }

        #[pg_trigger]
        fn signature_alloc_by_rust(
            trigger: &pgx::PgTrigger,
        ) -> Result<PgHeapTuple<'_, AllocatedByRust>, PgHeapTupleError> {
            Ok(trigger.current().unwrap().into_owned())
        }

        // Check type aliases
        type AliasedBorrowedPgTrigger<'a> = &'a pgx::PgTrigger;

        #[pg_trigger]
        fn signature_aliased_argument<'a>(
            trigger: AliasedBorrowedPgTrigger<'a>,
        ) -> Result<PgHeapTuple<'a, impl WhoAllocated>, core::str::Utf8Error> {
            Ok(trigger.current().unwrap().into_owned())
        }

        type AliasedTriggerResult<'a> = Result<PgHeapTuple<'a, AllocatedByRust>, TriggerError>;

        #[pg_trigger]
        fn signature_aliased_return(_trigger: &pgx::PgTrigger) -> AliasedTriggerResult<'_> {
            unimplemented!("Only testing signature compiles")
        }

        #[pg_trigger]
        fn signature_aliased_both(_trigger: AliasedBorrowedPgTrigger) -> AliasedTriggerResult<'_> {
            unimplemented!("Only testing signature compiles")
        }
    }

    #[derive(thiserror::Error, Debug)]
    enum TriggerError {
        #[error("Null OLD found")]
        NullCurrent,
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
    fn field_species_fox_to_bear(
        trigger: &pgx::PgTrigger,
    ) -> Result<PgHeapTuple<'_, impl WhoAllocated>, TriggerError> {
        let current = trigger.current().ok_or(TriggerError::NullCurrent)?;
        let mut current = current.into_owned();

        let field = "species";

        if current.get_by_name(field)? == Some("Fox") {
            current.set_by_name(field, "Bear")?;
        }

        Ok(current)
    }

    #[pg_test]
    fn before_insert_field_update() {
        Spi::run(
            r#"
            CREATE TABLE tests.before_insert_field_update (species TEXT)
        "#,
        );

        Spi::run(
            r#"
            CREATE TRIGGER foxes_to_bears
                BEFORE INSERT ON tests.before_insert_field_update
                FOR EACH ROW
                EXECUTE PROCEDURE tests.field_species_fox_to_bear()
        "#,
        );

        Spi::run(
            r#"
            INSERT INTO tests.before_insert_field_update (species)
                VALUES ('Fox')
        "#,
        );

        let retval = Spi::get_one::<&str>("SELECT species FROM tests.before_insert_field_update;")
            .expect("SQL select failed");
        assert_eq!(retval, "Bear");
    }

    #[pg_trigger]
    fn add_field_boopers(
        trigger: &pgx::PgTrigger,
    ) -> Result<PgHeapTuple<'_, impl WhoAllocated>, TriggerError> {
        let current = trigger.current().ok_or(TriggerError::NullCurrent)?;
        let mut current = current.into_owned();

        let field = "booper";

        if current.get_by_name(field)? == Option::<&str>::None {
            current.set_by_name(field, "Swooper")?;
        }

        Ok(current)
    }

    #[pg_test]
    fn before_insert_add_field() {
        Spi::run(
            r#"
            CREATE TABLE tests.before_insert_add_field (name TEXT, booper TEXT)
        "#,
        );

        Spi::run(
            r#"
            CREATE TRIGGER add_field
                BEFORE INSERT ON tests.before_insert_add_field
                FOR EACH ROW
                EXECUTE PROCEDURE tests.add_field_boopers()
        "#,
        );

        Spi::run(
            r#"
            INSERT INTO tests.before_insert_add_field (name)
                VALUES ('Nami')
        "#,
        );

        let retval = Spi::get_one::<&str>("SELECT booper FROM tests.before_insert_add_field;")
            .expect("SQL select failed");
        assert_eq!(retval, "Swooper");
    }

    #[pg_trigger]
    fn intercept_bears(
        trigger: &pgx::PgTrigger,
    ) -> Result<PgHeapTuple<'_, impl WhoAllocated>, TriggerError> {
        let new = trigger.new().ok_or(TriggerError::NullCurrent)?;

        for index in 1..(new.len() + 1) {
            if let Some(val) = new.get_by_index::<&str>(index.try_into()?)? {
                if val == "Bear" {
                    // We intercepted a bear! Avoid this update, return `current` instead.
                    let current = trigger.current().ok_or(TriggerError::NullCurrent)?;
                    return Ok(current);
                }
            }
        }

        Ok(new)
    }

    #[pg_test]
    fn before_update_skip() {
        Spi::run(
            r#"
            CREATE TABLE tests.before_update_skip (title TEXT)
        "#,
        );

        Spi::run(
            r#"
            CREATE TRIGGER add_field
                BEFORE UPDATE ON tests.before_update_skip
                FOR EACH ROW
                EXECUTE PROCEDURE tests.intercept_bears()
        "#,
        );

        Spi::run(
            r#"
            INSERT INTO tests.before_update_skip (title)
                VALUES ('Fox')
        "#,
        );
        Spi::run(
            r#"
            UPDATE tests.before_update_skip SET title = 'Bear'
                WHERE title = 'Fox'
        "#,
        );

        let retval = Spi::get_one::<&str>("SELECT title FROM tests.before_update_skip;")
            .expect("SQL select failed");
        assert_eq!(retval, "Fox");
    }

    #[pg_trigger]
    fn inserts_trigger_metadata(
        trigger: &pgx::PgTrigger,
    ) -> Result<PgHeapTuple<'_, impl WhoAllocated>, TriggerError> {
        let current = trigger.current().ok_or(TriggerError::NullCurrent)?;
        let mut current = current.into_owned();

        let trigger_name = trigger.name()?;
        current.set_by_name("trigger_name", trigger_name)?;

        let trigger_when = trigger.when()?.to_string();
        current.set_by_name("trigger_when", trigger_when)?;

        let trigger_level = trigger.level().to_string();
        current.set_by_name("trigger_level", trigger_level)?;

        let trigger_op = trigger.op()?.to_string();
        current.set_by_name("trigger_op", trigger_op)?;

        let trigger_relid = trigger.relid()?;
        current.set_by_name("trigger_relid", trigger_relid)?;

        let trigger_old_transition_table_name = trigger.old_transition_table_name()?;
        current
            .set_by_name("trigger_old_transition_table_name", trigger_old_transition_table_name)?;

        let trigger_new_transition_table_name = trigger.new_transition_table_name()?;
        current
            .set_by_name("trigger_new_transition_table_name", trigger_new_transition_table_name)?;

        let trigger_table_name = unsafe { trigger.table_name()? };
        current.set_by_name("trigger_table_name", trigger_table_name)?;

        let trigger_table_schema = unsafe { trigger.table_schema()? };
        current.set_by_name("trigger_table_schema", trigger_table_schema)?;

        let trigger_extra_args = trigger.extra_args()?;
        current.set_by_name("trigger_extra_args", trigger_extra_args)?;

        Ok(current)
    }

    #[pg_test]
    fn before_insert_metadata() {
        Spi::run(
            r#"
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
        "#,
        );

        Spi::run(
            r#"
            CREATE TRIGGER insert_trigger_metadata
                BEFORE INSERT ON tests.before_insert_trigger_metadata
                FOR EACH ROW
                EXECUTE PROCEDURE tests.inserts_trigger_metadata('Bears', 'Dogs')
        "#,
        );

        Spi::run(
            r#"
            INSERT INTO tests.before_insert_trigger_metadata (marker)
                VALUES ('Fox')
        "#,
        );

        let marker =
            Spi::get_one::<&str>("SELECT marker FROM tests.before_insert_trigger_metadata;")
                .unwrap();
        let trigger_name =
            Spi::get_one::<&str>("SELECT trigger_name FROM tests.before_insert_trigger_metadata;")
                .unwrap();
        let trigger_when =
            Spi::get_one::<&str>("SELECT trigger_when FROM tests.before_insert_trigger_metadata;")
                .unwrap();
        let trigger_level =
            Spi::get_one::<&str>("SELECT trigger_level FROM tests.before_insert_trigger_metadata;")
                .unwrap();
        let trigger_op =
            Spi::get_one::<&str>("SELECT trigger_op FROM tests.before_insert_trigger_metadata;")
                .unwrap();
        let trigger_relid = Spi::get_one::<Option<pg_sys::Oid>>(
            "SELECT trigger_relid FROM tests.before_insert_trigger_metadata;",
        )
        .unwrap();
        let trigger_old_transition_table_name = Spi::get_one::<Option<&str>>(
            "SELECT trigger_old_transition_table_name FROM tests.before_insert_trigger_metadata;",
        )
        .unwrap();
        let trigger_new_transition_table_name = Spi::get_one::<Option<&str>>(
            "SELECT trigger_new_transition_table_name FROM tests.before_insert_trigger_metadata;",
        )
        .unwrap();
        let trigger_table_name = Spi::get_one::<&str>(
            "SELECT trigger_table_name FROM tests.before_insert_trigger_metadata;",
        )
        .unwrap();
        let trigger_table_schema = Spi::get_one::<&str>(
            "SELECT trigger_table_schema FROM tests.before_insert_trigger_metadata;",
        )
        .unwrap();
        let trigger_extra_args = Spi::get_one::<Vec<String>>(
            "SELECT trigger_extra_args FROM tests.before_insert_trigger_metadata;",
        )
        .unwrap();

        assert_eq!(marker, "Fox");
        assert_eq!(trigger_name, "insert_trigger_metadata");
        assert_eq!(trigger_when, "BEFORE");
        assert_eq!(trigger_level, "ROW");
        assert_eq!(trigger_op, "INSERT");
        assert!(trigger_relid.is_some());
        assert_eq!(trigger_old_transition_table_name, None);
        assert_eq!(trigger_new_transition_table_name, None);
        assert_eq!(trigger_table_name, "before_insert_trigger_metadata");
        assert_eq!(trigger_table_schema, "tests");
        assert_eq!(trigger_extra_args, vec!["Bears".to_string(), "Dogs".to_string()]);
    }

    #[pg_trigger]
    fn inserts_trigger_metadata_safe(
        trigger: &pgx::PgTrigger,
    ) -> Result<PgHeapTuple<'_, impl WhoAllocated>, TriggerError> {
        let pgx::PgTriggerSafe {
            name,
            new: _new,
            current,
            event: _event,
            when,
            level,
            op,
            relid,
            old_transition_table_name,
            new_transition_table_name,
            relation: _relation,
            table_name,
            table_schema,
            extra_args,
        } = unsafe { trigger.to_safe()? };

        let mut current_owned = current.ok_or(TriggerError::NullCurrent)?.into_owned();

        current_owned.set_by_name("trigger_name", name)?;
        current_owned.set_by_name("trigger_when", when.to_string())?;
        current_owned.set_by_name("trigger_level", level.to_string())?;
        current_owned.set_by_name("trigger_op", op.to_string())?;
        current_owned.set_by_name("trigger_relid", relid)?;
        current_owned
            .set_by_name("trigger_old_transition_table_name", old_transition_table_name)?;
        current_owned
            .set_by_name("trigger_new_transition_table_name", new_transition_table_name)?;
        current_owned.set_by_name("trigger_table_name", table_name)?;
        current_owned.set_by_name("trigger_table_schema", table_schema)?;
        current_owned.set_by_name("trigger_extra_args", extra_args)?;

        Ok(current_owned)
    }

    #[pg_test]
    fn before_insert_metadata_safe() {
        Spi::run(
            r#"
            CREATE TABLE tests.before_insert_trigger_metadata_safe (
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
        "#,
        );

        Spi::run(
            r#"
            CREATE TRIGGER insert_trigger_metadata_safe
                BEFORE INSERT ON tests.before_insert_trigger_metadata_safe
                FOR EACH ROW
                EXECUTE PROCEDURE tests.inserts_trigger_metadata_safe('Bears', 'Dogs')
        "#,
        );

        Spi::run(
            r#"
            INSERT INTO tests.before_insert_trigger_metadata_safe (marker)
                VALUES ('Fox')
        "#,
        );

        let marker =
            Spi::get_one::<&str>("SELECT marker FROM tests.before_insert_trigger_metadata_safe;")
                .unwrap();
        let trigger_name = Spi::get_one::<&str>(
            "SELECT trigger_name FROM tests.before_insert_trigger_metadata_safe;",
        )
        .unwrap();
        let trigger_when = Spi::get_one::<&str>(
            "SELECT trigger_when FROM tests.before_insert_trigger_metadata_safe;",
        )
        .unwrap();
        let trigger_level = Spi::get_one::<&str>(
            "SELECT trigger_level FROM tests.before_insert_trigger_metadata_safe;",
        )
        .unwrap();
        let trigger_op = Spi::get_one::<&str>(
            "SELECT trigger_op FROM tests.before_insert_trigger_metadata_safe;",
        )
        .unwrap();
        let trigger_relid = Spi::get_one::<Option<pg_sys::Oid>>(
            "SELECT trigger_relid FROM tests.before_insert_trigger_metadata_safe;",
        )
        .unwrap();
        let trigger_old_transition_table_name = Spi::get_one::<Option<&str>>(
            "SELECT trigger_old_transition_table_name FROM tests.before_insert_trigger_metadata_safe;",
        ).unwrap();
        let trigger_new_transition_table_name = Spi::get_one::<Option<&str>>(
            "SELECT trigger_new_transition_table_name FROM tests.before_insert_trigger_metadata_safe;",
        ).unwrap();
        let trigger_table_name = Spi::get_one::<&str>(
            "SELECT trigger_table_name FROM tests.before_insert_trigger_metadata_safe;",
        )
        .unwrap();
        let trigger_table_schema = Spi::get_one::<&str>(
            "SELECT trigger_table_schema FROM tests.before_insert_trigger_metadata_safe;",
        )
        .unwrap();
        let trigger_extra_args = Spi::get_one::<Vec<String>>(
            "SELECT trigger_extra_args FROM tests.before_insert_trigger_metadata_safe;",
        )
        .unwrap();

        assert_eq!(marker, "Fox");
        assert_eq!(trigger_name, "insert_trigger_metadata_safe");
        assert_eq!(trigger_when, "BEFORE");
        assert_eq!(trigger_level, "ROW");
        assert_eq!(trigger_op, "INSERT");
        assert!(trigger_relid.is_some());
        assert_eq!(trigger_old_transition_table_name, None);
        assert_eq!(trigger_new_transition_table_name, None);
        assert_eq!(trigger_table_name, "before_insert_trigger_metadata_safe");
        assert_eq!(trigger_table_schema, "tests");
        assert_eq!(trigger_extra_args, vec!["Bears".to_string(), "Dogs".to_string()]);
    }

    #[pg_trigger(sql = r#"
        CREATE FUNCTION tests."has_sql_option_set_and_respects_it"()
        RETURNS TRIGGER
        LANGUAGE c
        AS 'MODULE_PATHNAME', '@FUNCTION_NAME@';
    "#)]
    fn has_sql_option_set(
        trigger: &pgx::PgTrigger,
    ) -> Result<PgHeapTuple<'_, impl WhoAllocated>, TriggerError> {
        let current = trigger.current().ok_or(TriggerError::NullCurrent)?;
        let current = current.into_owned();

        Ok(current)
    }

    #[pg_test]
    fn before_insert_has_sql_option_set() {
        Spi::run(
            r#"
            CREATE TABLE tests.has_sql_option_set (species TEXT)
        "#,
        );

        Spi::run(
            r#"
            CREATE TRIGGER has_sql_option_set
                BEFORE INSERT ON tests.has_sql_option_set
                FOR EACH ROW
                EXECUTE PROCEDURE tests.has_sql_option_set_and_respects_it()
        "#,
        );

        Spi::run(
            r#"
            INSERT INTO tests.has_sql_option_set (species)
                VALUES ('Fox')
        "#,
        );

        let retval = Spi::get_one::<&str>("SELECT species FROM tests.has_sql_option_set;")
            .expect("SQL select failed");
        assert_eq!(retval, "Fox");
    }

    #[pg_trigger]
    fn noop_postgres(
        trigger: &pgx::PgTrigger,
    ) -> Result<PgHeapTuple<'_, AllocatedByPostgres>, TriggerError> {
        Ok(trigger.current().unwrap())
    }

    #[pg_test]
    fn before_insert_noop_postgres() {
        Spi::run(
            r#"
            CREATE TABLE tests.has_noop_postgres (species TEXT)
        "#,
        );

        Spi::run(
            r#"
            CREATE TRIGGER noop_postgres
                BEFORE INSERT ON tests.has_noop_postgres
                FOR EACH ROW
                EXECUTE PROCEDURE tests.noop_postgres()
        "#,
        );

        Spi::run(
            r#"
            INSERT INTO tests.has_noop_postgres (species)
                VALUES ('Fox')
        "#,
        );

        let retval = Spi::get_one::<&str>("SELECT species FROM tests.has_noop_postgres;")
            .expect("SQL select failed");
        assert_eq!(retval, "Fox");
    }

    #[pg_trigger]
    fn noop_rust(
        trigger: &pgx::PgTrigger,
    ) -> Result<PgHeapTuple<'_, AllocatedByRust>, TriggerError> {
        Ok(trigger.current().unwrap().into_owned())
    }

    #[pg_test]
    fn before_insert_noop_rust() {
        Spi::run(
            r#"
            CREATE TABLE tests.has_noop_rust (species TEXT)
        "#,
        );

        Spi::run(
            r#"
            CREATE TRIGGER noop_postgres
                BEFORE INSERT ON tests.has_noop_rust
                FOR EACH ROW
                EXECUTE PROCEDURE tests.noop_rust()
        "#,
        );

        Spi::run(
            r#"
            INSERT INTO tests.has_noop_rust (species)
                VALUES ('Fox')
        "#,
        );

        let retval = Spi::get_one::<&str>("SELECT species FROM tests.has_noop_rust;")
            .expect("SQL select failed");
        assert_eq!(retval, "Fox");
    }
}
