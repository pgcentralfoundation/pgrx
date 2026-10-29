//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

/// Types named after built-in types. PostgreSQL searches `pg_catalog` before the schemas in
/// `search_path`, so looking these names up there finds the built-in types.
#[pgrx::pg_schema]
mod shadowing {
    use core::ffi::CStr;
    use pgrx::datum::PgVarlena;
    use pgrx::prelude::*;
    use pgrx::{PgVarlenaInOutFuncs, StringInfo};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PostgresType, Serialize, Deserialize)]
    pub struct Point {
        pub x: i32,
    }

    #[derive(Copy, Clone, Default, PostgresType)]
    #[bikeshed_postgres_type_manually_impl_from_into_datum]
    #[pgvarlena_inoutfuncs]
    pub struct Circle {
        pub radius: f64,
    }

    impl PgVarlenaInOutFuncs for Circle {
        fn input(input: &CStr) -> PgVarlena<Self> {
            let radius = input.to_str().expect("utf-8").parse().expect("an f64");
            PgVarlena::from(Circle { radius })
        }

        fn output(&self, buffer: &mut StringInfo) {
            buffer.push_str(&self.radius.to_string());
        }
    }
}

#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgrx_unit_tests;

    use super::shadowing::{Circle, Point};
    use crate::tests::Complex;
    use pgrx::datum::{PgVarlena, TryFromDatumError};
    use pgrx::prelude::*;
    use pgrx::spi;

    fn regtype(name: &str) -> pg_sys::Oid {
        Spi::get_one(&format!("SELECT '{name}'::regtype::oid")).unwrap().unwrap()
    }

    #[pg_test]
    fn test_type_oid_is_the_extension_type() {
        let point = regtype("shadowing.point");
        assert_ne!(point, pg_sys::POINTOID);
        assert_eq!(Point::type_oid(), point);
        assert_eq!(PgVarlena::<Circle>::type_oid(), regtype("shadowing.circle"));
        // pgrx_pg_sys structs name built-in types
        assert_eq!(PgBox::<pg_sys::Point>::type_oid(), pg_sys::POINTOID);
    }

    #[pg_test]
    fn test_type_oid_ignores_search_path() -> spi::Result<()> {
        let complex = regtype("complex");
        Spi::run("CREATE SCHEMA decoy")?;
        Spi::run("CREATE TYPE decoy.complex AS (r float8, i float8)")?;
        Spi::run("CREATE TYPE decoy.point AS (x float8, y float8)")?;
        Spi::run("SET LOCAL search_path TO decoy, pg_catalog, public")?;
        assert_ne!(regtype("complex"), complex);
        assert_ne!(regtype("point"), pg_sys::POINTOID);
        assert_eq!(PgBox::<Complex>::type_oid(), complex);
        assert_eq!(PgBox::<pg_sys::Point>::type_oid(), pg_sys::POINTOID);
        Ok(())
    }

    #[pg_test(
        error = "more than one type named \"complex\" belongs to extension \"pgrx_unit_tests\""
    )]
    fn test_type_oid_rejects_two_extension_types_of_one_name() -> spi::Result<()> {
        Spi::run("CREATE SCHEMA decoy")?;
        Spi::run("CREATE TYPE decoy.complex AS (r float8, i float8)")?;
        Spi::run("ALTER EXTENSION pgrx_unit_tests ADD TYPE decoy.complex")?;
        PgBox::<Complex>::type_oid();
        Ok(())
    }

    /// Add tables named `complex`, each in its own schema, whose row types share the extension
    /// type's name. After the first few, the lookup lists 64 of the extension's members for each
    /// same-named type it checks, so with this many it finishes by listing the members.
    fn add_same_named_tables() -> spi::Result<()> {
        let members = Spi::get_one::<i64>(
            "SELECT count(*) FROM pg_depend WHERE refclassid = 'pg_extension'::regclass \
             AND refobjid = (SELECT oid FROM pg_extension WHERE extname = 'pgrx_unit_tests')",
        )?
        .expect("a count");
        let tables = members / 16 + 2;
        Spi::run(&format!(
            "DO $$ BEGIN FOR i IN 1..{tables} LOOP \
               EXECUTE format('CREATE SCHEMA same_name_%s', i); \
               EXECUTE format('CREATE TABLE same_name_%s.complex (a int)', i); \
             END LOOP; END $$"
        ))
    }

    #[pg_test]
    fn test_type_oid_ignores_many_same_named_tables() -> spi::Result<()> {
        let complex = regtype("complex");
        add_same_named_tables()?;
        assert_eq!(PgBox::<Complex>::type_oid(), complex);
        Ok(())
    }

    #[pg_test(
        error = "more than one type named \"complex\" belongs to extension \"pgrx_unit_tests\""
    )]
    fn test_type_oid_rejects_two_extension_types_among_many_of_one_name() -> spi::Result<()> {
        add_same_named_tables()?;
        Spi::run("CREATE SCHEMA decoy")?;
        Spi::run("CREATE TYPE decoy.complex AS (r float8, i float8)")?;
        Spi::run("ALTER EXTENSION pgrx_unit_tests ADD TYPE decoy.complex")?;
        PgBox::<Complex>::type_oid();
        Ok(())
    }

    /// PostgreSQL 17 and later make a table's row type a member of the table's extension, and
    /// earlier versions do not. The lookup skips the row type on every version.
    #[pg_test]
    fn test_type_oid_ignores_an_extension_table_of_the_same_name() -> spi::Result<()> {
        let complex = regtype("complex");
        Spi::run("CREATE SCHEMA decoy")?;
        Spi::run("CREATE TABLE decoy.complex (r float8, i float8)")?;
        Spi::run("ALTER EXTENSION pgrx_unit_tests ADD TABLE decoy.complex")?;
        assert_eq!(PgBox::<Complex>::type_oid(), complex);
        Ok(())
    }

    /// Named like the type `decoy_b._zzelem` and the array type of `decoy_a.zzelem`.
    struct _ZzElem;

    /// PostgreSQL 17 and later make the array type of an extension's type a member of the
    /// extension too, and earlier versions do not. The lookup skips the array type on every
    /// version.
    #[pg_test]
    fn test_type_oid_ignores_an_extension_array_type_of_the_same_name() -> spi::Result<()> {
        Spi::run("CREATE SCHEMA decoy_a")?;
        Spi::run("CREATE TYPE decoy_a.zzelem AS (a int)")?;
        Spi::run("ALTER EXTENSION pgrx_unit_tests ADD TYPE decoy_a.zzelem")?;
        Spi::run("CREATE SCHEMA decoy_b")?;
        Spi::run("CREATE TYPE decoy_b._zzelem AS (b int)")?;
        Spi::run("ALTER EXTENSION pgrx_unit_tests ADD TYPE decoy_b._zzelem")?;
        assert_eq!(pgrx::rust_regtypein::<_ZzElem>(), regtype("decoy_b._zzelem"));
        Ok(())
    }

    /// Named like the shell type `decoy.zzshell`.
    struct ZzShell;

    #[pg_test(error = "type \"zzshell\" is only a shell")]
    fn test_type_oid_rejects_a_shell_type() -> spi::Result<()> {
        Spi::run("CREATE SCHEMA decoy")?;
        Spi::run("CREATE TYPE decoy.zzshell")?;
        Spi::run("ALTER EXTENSION pgrx_unit_tests ADD TYPE decoy.zzshell")?;
        pgrx::rust_regtypein::<ZzShell>();
        Ok(())
    }

    #[pg_test]
    fn test_builtin_type_of_the_same_name_is_incompatible() {
        let result = Spi::get_one::<Point>("SELECT '(1,2)'::pg_catalog.point");
        assert!(matches!(
            result,
            Err(spi::Error::DatumError(TryFromDatumError::IncompatibleTypes { .. }))
        ));

        let result = Spi::get_one::<PgVarlena<Circle>>("SELECT '<(1,2),3>'::pg_catalog.circle");
        assert!(matches!(
            result,
            Err(spi::Error::DatumError(TryFromDatumError::IncompatibleTypes { .. }))
        ));
    }
}
