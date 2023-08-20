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
    use crate as pgrx_tests;

    use pgrx::prelude::*;

    #[pg_extern]
    fn my_int4eq(l: i32, r: i32) -> bool {
        l == r
    }

    extension_sql!(
        r#"
            CREATE FUNCTION tests.sql_int4eq(int4, int4) RETURNS bool STRICT LANGUAGE sql AS $$ SELECT $1 = $2; $$;
        "#,
        name = "test_funcs",
        requires = [tests]
    );

    #[pg_test]
    fn test_int4eq_eq() {
        let result = pgrx::fcall::fcall::<bool>("int4eq", &[&Some(1), &Some(1)]);
        assert_eq!(Some(true), result)
    }

    #[pg_test]
    fn test_int4eq_ne() {
        let result = pgrx::fcall::fcall::<bool>("int4eq", &[&Some(1), &Some(2)]);
        assert_eq!(Some(false), result)
    }

    #[pg_test]
    fn test_my_int4eq() {
        let result = pgrx::fcall::fcall::<bool>("tests.my_int4eq", &[&Some(1), &Some(1)]);
        assert_eq!(Some(true), result)
    }

    #[pg_test]
    fn test_sql_int4eq() {
        let result = pgrx::fcall::fcall::<bool>("tests.sql_int4eq", &[&Some(1), &Some(1)]);
        assert_eq!(Some(true), result)
    }
}
