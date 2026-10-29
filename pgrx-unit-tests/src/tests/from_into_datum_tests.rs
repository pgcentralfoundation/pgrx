//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgrx_unit_tests;
    use std::error::Error;
    use std::ffi::CStr;

    use pgrx::datum::TryFromDatumError;
    use pgrx::prelude::*;
    use pgrx::{AllocatedByRust, Array, PgMemoryContexts};

    #[pg_test]
    fn test_incompatible_datum_returns_error() {
        let result = unsafe {
            String::try_from_datum(
                pg_sys::Datum::from(false),
                true,
                pg_sys::BuiltinOid::BOOLOID.value(),
            )
        };
        assert!(result.is_err());
        assert_eq!(
            "Postgres type boolean (Oid(16)) is not compatible with the Rust type alloc::string::String (Oid(25))",
            result.unwrap_err().to_string()
        );
    }

    #[pg_test]
    fn test_unknown_type_oid_returns_error() {
        // No type has this OID: PostgreSQL assigns OIDs upward from 16384 and wraps only after
        // exhausting the 32-bit range.
        let unknown = pg_sys::Oid::from(u32::MAX);
        let incompatible =
            |rust_type: &'static str, rust_oid| TryFromDatumError::IncompatibleTypes {
                rust_type,
                rust_oid,
                datum_type: "???".to_owned(),
                datum_oid: unknown,
            };

        let result = unsafe { i32::try_from_datum(pg_sys::Datum::from(0), false, unknown) };
        assert_eq!(result, Err(incompatible("i32", pg_sys::INT4OID)));

        let result = unsafe {
            String::try_from_datum_in_memory_context(
                PgMemoryContexts::CurrentMemoryContext,
                pg_sys::Datum::from(0),
                false,
                unknown,
            )
        };
        assert_eq!(result, Err(incompatible(std::any::type_name::<String>(), pg_sys::TEXTOID)));

        // PgHeapTuple's compatibility check looks the type up itself.
        let result = unsafe {
            PgHeapTuple::<AllocatedByRust>::try_from_datum(pg_sys::Datum::from(0), false, unknown)
        };
        assert_eq!(
            result.map(|tuple| tuple.map(|_| ())),
            Err(incompatible(
                std::any::type_name::<PgHeapTuple<AllocatedByRust>>(),
                pg_sys::RECORDOID
            ))
        );
    }

    #[pg_test]
    fn test_invalid_type_oid_returns_error() {
        let incompatible =
            |rust_type: &'static str, rust_oid| TryFromDatumError::IncompatibleTypes {
                rust_type,
                rust_oid,
                datum_type: "-".to_owned(),
                datum_oid: pg_sys::Oid::INVALID,
            };

        let result =
            unsafe { i32::try_from_datum(pg_sys::Datum::from(0), false, pg_sys::Oid::INVALID) };
        assert_eq!(result, Err(incompatible("i32", pg_sys::INT4OID)));

        let result = unsafe {
            PgHeapTuple::<AllocatedByRust>::try_from_datum(
                pg_sys::Datum::from(0),
                false,
                pg_sys::Oid::INVALID,
            )
        };
        assert_eq!(
            result.map(|tuple| tuple.map(|_| ())),
            Err(incompatible(
                std::any::type_name::<PgHeapTuple<AllocatedByRust>>(),
                pg_sys::RECORDOID
            ))
        );

        // int4[] has no array type, so this `T`'s type_oid() is InvalidOid too. Accepting the
        // match would read each int4 element as a pointer to an array.
        let datum = vec![7_i32, 8].into_datum().expect("an array datum");
        let result =
            unsafe { Array::<Array<i32>>::try_from_datum(datum, false, pg_sys::Oid::INVALID) };
        assert_eq!(
            result.map(|array| array.map(|array| array.len())),
            Err(incompatible(std::any::type_name::<Array<Array<i32>>>(), pg_sys::Oid::INVALID))
        );
    }

    #[pg_extern]
    fn cstring_roundtrip(s: &CStr) -> &CStr {
        s
    }

    #[pg_test]
    fn test_cstring_roundtrip() -> Result<(), Box<dyn Error>> {
        let cstr = Spi::get_one::<&CStr>("SELECT tests.cstring_roundtrip('hello')")?
            .expect("SPI result was NULL");
        let expected = c"hello";
        assert_eq!(cstr, expected);
        Ok(())
    }

    #[pg_test]
    fn null_string_is_none() {
        let val: pg_sys::Datum = pg_sys::Datum::null();
        let is_null: bool = true;

        unsafe {
            if String::from_datum(val, is_null).is_some() {
                // <- this seg fault
                panic!("converted a null Datum into a valid string")
            }
        }
    }
}
