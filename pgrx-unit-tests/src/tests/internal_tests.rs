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
    use pgrx::Internal;
    use pgrx::prelude::*;

    #[pg_test]
    fn internal_insert() {
        let mut val = Internal::default();
        assert!(!val.initialized());

        let inner = unsafe { val.insert::<i32>(5) };

        assert_eq!(*inner, 5);
        assert!(val.initialized());

        let inner = unsafe { val.insert::<i32>(6) };

        assert_eq!(*inner, 6);
        assert!(val.initialized());
    }

    #[pg_test]
    fn internal_get_or_insert_default() {
        let mut val = Internal::default();
        assert!(!val.initialized());

        let inner = unsafe { val.get_or_insert_default::<i32>() };

        assert_eq!(*inner, 0);
        assert!(val.initialized());
    }

    #[pg_test]
    fn internal_get_or_insert() {
        let mut val = Internal::default();
        assert!(!val.initialized());

        let inner = unsafe { val.get_or_insert::<i32>(5) };

        assert_eq!(*inner, 5);
        assert!(val.initialized());
    }

    #[pg_test]
    fn internal_get_or_insert_with() {
        let mut val = Internal::default();
        assert!(!val.initialized());

        let inner = unsafe { val.get_or_insert_with(|| 5) };

        assert_eq!(*inner, 5);
        assert!(val.initialized());
    }

    // PostgreSQL passes a null pointer that is not SQL NULL for some `internal` arguments, such
    // as the second argument of an aggregate deserialization function.
    #[pg_test]
    fn internal_not_null_zero_datum_is_uninitialized() {
        let null = || unsafe { Internal::from_datum(pg_sys::Datum::null(), false) }.unwrap();

        let val = null();
        assert!(!val.initialized());
        assert!(unsafe { val.get::<i32>() }.is_none());
        assert_eq!(val.unwrap(), Some(pg_sys::Datum::null()));

        let mut val = null();
        let inner = unsafe { val.get_or_insert_default::<i32>() };
        assert_eq!(*inner, 0);
        *inner = 5;
        assert!(val.initialized());
        assert_eq!(unsafe { val.get::<i32>() }, Some(&5));
    }
}
