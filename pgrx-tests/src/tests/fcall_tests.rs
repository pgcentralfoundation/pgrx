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

    use pgrx::fcall::*;
    use pgrx::prelude::*;

    #[pg_extern]
    fn my_int4eq(l: i32, r: i32) -> bool {
        l == r
    }

    #[pg_extern]
    fn arg_might_be_null(v: Option<i32>) -> Option<i32> {
        v
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
        let result = fcall::<bool>("pg_catalog.int4eq", &[&Some(1), &Some(1)]);
        assert_eq!(Ok(Some(true)), result)
    }

    #[pg_test]
    fn test_int4eq_ne() {
        let result = fcall::<bool>("pg_catalog.int4eq", &[&Some(1), &Some(2)]);
        assert_eq!(Ok(Some(false)), result)
    }

    #[pg_test]
    fn test_my_int4eq() {
        let result = fcall::<bool>("tests.my_int4eq", &[&Some(1), &Some(1)]);
        assert_eq!(Ok(Some(true)), result)
    }

    #[pg_test]
    fn test_sql_int4eq() {
        let result = fcall::<bool>("tests.sql_int4eq", &[&Some(1), &Some(1)]);
        assert_eq!(Ok(Some(true)), result)
    }

    #[pg_test]
    fn test_null_arg_some() {
        let result = fcall::<i32>("tests.arg_might_be_null", &[&Some(1)]);
        assert_eq!(Ok(Some(1)), result)
    }

    #[pg_test]
    fn test_null_arg_none() {
        let result = fcall::<i32>("tests.arg_might_be_null", &[&Option::<i32>::None]);
        assert_eq!(Ok(None), result)
    }

    #[pg_test]
    fn test_strict() {
        // calling a STRICT function such as pg_catalog.float4 with a NULL argument will crash Postgres
        let result = fcall::<f32>("pg_catalog.float4", &[&Option::<AnyNumeric>::None]);
        assert_eq!(Ok(None), result);
    }

    #[pg_test]
    fn test_incompatible_return_type() {
        let result = fcall::<String>("pg_catalog.int4eq", &[&Some(1), &Some(1)]);
        assert_eq!(
            Err(FCallError::IncompatibleReturnType(String::type_oid(), pg_sys::BOOLOID)),
            result
        );
    }

    #[pg_test]
    fn test_too_many_args() {
        let args: [&dyn FCallArg; 32768] = [&Some(1); 32768];
        let result = fcall::<bool>("pg_catalog.int4eq", &args);
        assert_eq!(Err(FCallError::TooManyArguments), result);
    }

    // NB:  I don't see a way for `fcall()` to be ambiguous about which function it wants to call?
    //      Spent about 30m trying to cook up an example and couldn't.
    // #[pg_test]
    // fn ambiguous_function() {
    //     let result = fcall::<bool>("tests.ambiguous", &[&Some(42)]);
    //     assert_eq!(Err(FCallError::AmbiguousFunction), result)
    // }

    #[pg_test]
    fn unknown_function() {
        let result = fcall::<()>("undefined_function", &[]);
        assert_eq!(Err(FCallError::UndefinedFunction), result)
    }

    #[pg_test]
    fn blank_function() {
        let result = fcall::<()>("", &[]);
        assert_eq!(Err(FCallError::InvalidIdentifier(String::from(""))), result)
    }

    #[pg_test]
    fn invalid_identifier() {
        let stupid_name = "q234qasf )(A*q2342";
        let result = fcall::<()>(stupid_name, &[]);
        assert_eq!(Err(FCallError::InvalidIdentifier(String::from(stupid_name))), result)
    }
}
