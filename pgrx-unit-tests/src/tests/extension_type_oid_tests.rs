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
