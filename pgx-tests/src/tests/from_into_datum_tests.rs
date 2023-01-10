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

    #[pg_test]
    fn test_incompatible_datum_returns_error() {
        let result = unsafe {
            String::try_from_datum(
                pg_sys::Datum::from(false),
                true,
                pg_sys::PgBuiltInOids::BOOLOID.value(),
            )
        };
        assert!(result.is_err());
        assert_eq!("Postgres type boolean `Oid(16)` is not compatible with the Rust type alloc::string::String `Oid(25)`", result.unwrap_err().to_string());
    }
}
