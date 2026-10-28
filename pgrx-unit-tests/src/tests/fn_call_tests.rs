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
    use std::sync::atomic::Ordering;

    use pgrx::AnyArray;
    use pgrx::fn_call::*;
    use pgrx::prelude::*;

    #[pg_extern]
    fn my_int4eq(l: i32, r: i32) -> bool {
        l == r
    }

    #[pg_extern]
    fn arg_might_be_null(v: Option<i32>) -> Option<i32> {
        v
    }

    /// An int4 passed as the domain `tests.fcall_positive`
    #[derive(Clone)]
    struct PositiveInt(i32);

    impl IntoDatum for PositiveInt {
        fn into_datum(self) -> Option<pg_sys::Datum> {
            self.0.into_datum()
        }

        fn type_oid() -> pg_sys::Oid {
            pgrx::regtypein("tests.fcall_positive")
        }
    }

    /// A label of the enum `tests.fcall_enum`
    #[derive(Clone)]
    struct FcallEnum(&'static str);

    impl IntoDatum for FcallEnum {
        fn into_datum(self) -> Option<pg_sys::Datum> {
            Some(pgrx::lookup_enum_by_label("tests.fcall_enum", self.0))
        }

        fn type_oid() -> pg_sys::Oid {
            pgrx::regtypein("tests.fcall_enum")
        }
    }

    /// An OID that names no type
    const NO_TYPE: u32 = u32::MAX;

    /// A value of type `OID` that fn_call must reject before converting it
    #[derive(Clone)]
    struct Unconvertible<const OID: u32>;

    impl<const OID: u32> IntoDatum for Unconvertible<OID> {
        fn into_datum(self) -> Option<pg_sys::Datum> {
            panic!("fn_call converted an argument of type {OID}")
        }

        fn type_oid() -> pg_sys::Oid {
            pg_sys::Oid::from(OID)
        }
    }

    extension_sql!(
        r#"
            CREATE FUNCTION tests.sql_int4eq(int4, int4) RETURNS bool STRICT LANGUAGE sql AS $$ SELECT $1 = $2; $$;
            
            CREATE FUNCTION tests.with_only_default(int4 DEFAULT 0) RETURNS text STRICT LANGUAGE sql AS $$ SELECT 'int4'; $$;
            CREATE FUNCTION tests.with_only_default(int8 DEFAULT 0) RETURNS text STRICT LANGUAGE sql AS $$ SELECT 'int8'; $$;
            
            CREATE FUNCTION tests.with_default(int4, int4 DEFAULT 0) RETURNS int4 STRICT LANGUAGE sql AS $$ SELECT $1 + $2; $$;
            
            CREATE FUNCTION tests.with_two_defaults(int4 DEFAULT 0, int4 DEFAULT 0) RETURNS int4 STRICT LANGUAGE sql AS $$ SELECT $1 + $2; $$;
            
            CREATE FUNCTION tests.with_arg_and_two_defaults(int4, int4 DEFAULT 1, int4 DEFAULT 2) RETURNS int4 STRICT LANGUAGE sql AS $$ SELECT $1 + $2 + $3; $$;
            
            CREATE FUNCTION tests.with_null_default(text DEFAULT NULL) RETURNS text STRICT LANGUAGE sql AS $$ SELECT $1; $$;
            
            CREATE FUNCTION tests.n() RETURNS text IMMUTABLE LANGUAGE sql AS $$ SELECT NULL; $$;
            CREATE FUNCTION tests.with_functional_default(text DEFAULT tests.n()) RETURNS text STRICT LANGUAGE sql AS $$ SELECT $1; $$;
            
            CREATE FUNCTION tests.fcall_raise_error(e text) RETURNS void LANGUAGE plpgsql AS $$ BEGIN RAISE EXCEPTION '%', e; END; $$;

            CREATE DOMAIN tests.fcall_positive AS int4 CHECK (VALUE > 0);
            CREATE FUNCTION tests.fcall_positive_int(tests.fcall_positive) RETURNS int4 LANGUAGE sql AS $$ SELECT $1; $$;

            CREATE FUNCTION tests.fcall_array_and_element(anyarray, anyelement) RETURNS int4 LANGUAGE sql AS $$ SELECT 1; $$;

            -- PostgreSQL stores each default with its own type, and casts it when a query calls the function
            CREATE FUNCTION tests.fcall_positions_default(anycompatiblearray, anycompatible DEFAULT 1) RETURNS int4[] LANGUAGE internal AS 'array_positions';
            CREATE FUNCTION tests.fcall_position_untyped_default(anycompatiblearray, anycompatible DEFAULT 'x') RETURNS int4 LANGUAGE internal AS 'array_position';
            CREATE FUNCTION tests.fcall_position_null_default(anycompatiblearray, anycompatible DEFAULT NULL) RETURNS int4 LANGUAGE internal AS 'array_position';

            CREATE TYPE tests.fcall_enum AS ENUM ('a');
            -- pg_backend_pid ignores the arguments
            CREATE FUNCTION tests.fcall_compatible_and_enum(anycompatible, anyenum) RETURNS int4 LANGUAGE internal AS 'pg_backend_pid';
        "#,
        name = "test_funcs",
        requires = [tests]
    );

    #[pg_test]
    fn test_int4eq_eq() {
        let result = fn_call::<bool>("pg_catalog.int4eq", &[&Arg::Value(1), &Arg::Value(1)]);
        assert_eq!(Ok(Some(true)), result)
    }

    #[pg_test]
    fn test_int4eq_ne() {
        let result = fn_call::<bool>("pg_catalog.int4eq", &[&Arg::Value(1), &Arg::Value(2)]);
        assert_eq!(Ok(Some(false)), result)
    }

    #[pg_test]
    fn test_my_int4eq() {
        let result = fn_call::<bool>("tests.my_int4eq", &[&Arg::Value(1), &Arg::Value(1)]);
        assert_eq!(Ok(Some(true)), result)
    }

    #[pg_test]
    fn test_sql_int4eq() {
        let result = fn_call::<bool>("tests.sql_int4eq", &[&Arg::Value(1), &Arg::Value(1)]);
        assert_eq!(Ok(Some(true)), result)
    }

    #[pg_test]
    fn test_null_arg_some() {
        let result = fn_call::<i32>("tests.arg_might_be_null", &[&Arg::Value(1)]);
        assert_eq!(Ok(Some(1)), result)
    }

    #[pg_test]
    fn test_null_arg_none() {
        let result = fn_call::<i32>("tests.arg_might_be_null", &[&Arg::<i32>::Null]);
        assert_eq!(Ok(None), result)
    }

    #[pg_test]
    fn test_strict() {
        // calling a STRICT function such as pg_catalog.float4 with a NULL argument will crash Postgres
        let result = fn_call::<f32>("pg_catalog.float4", &[&Arg::<AnyNumeric>::Null]);
        assert_eq!(Ok(None), result);
    }

    #[pg_test]
    fn test_incompatible_return_type() {
        let result = fn_call::<String>("pg_catalog.int4eq", &[&Arg::Value(1), &Arg::Value(1)]);
        assert_eq!(
            Err(FnCallError::IncompatibleReturnType(String::type_oid(), pg_sys::BOOLOID)),
            result
        );
    }

    #[pg_test]
    fn test_too_many_args() {
        let args: [&dyn FnCallArg; 32768] = [&Arg::Value(1); 32768];
        let result = fn_call::<bool>("pg_catalog.int4eq", &args);
        assert_eq!(Err(FnCallError::TooManyArguments), result);
    }

    #[pg_test]
    fn test_with_only_default_ambiguous() {
        let result = fn_call::<&str>("tests.with_only_default", &[]);
        assert_eq!(Err(FnCallError::AmbiguousFunction), result);
    }

    #[pg_test]
    fn test_with_only_default_int4() {
        let result = fn_call::<&str>("tests.with_only_default", &[&Arg::<i32>::Default]);
        assert_eq!(Ok(Some("int4")), result);
    }

    #[pg_test]
    fn test_with_only_default_int8() {
        let result = fn_call::<&str>("tests.with_only_default", &[&Arg::<i64>::Default]);
        assert_eq!(Ok(Some("int8")), result);
    }

    #[pg_test]
    fn test_with_default() {
        let result = fn_call::<i32>("tests.with_default", &[&Arg::Value(42), &Arg::<i32>::Default]);
        assert_eq!(Ok(Some(42)), result);
    }

    #[pg_test]
    fn test_with_two_defaults() {
        let result = fn_call::<i32>("tests.with_two_defaults", &[]);
        assert_eq!(Ok(Some(0)), result);

        let result = fn_call::<i32>("tests.with_two_defaults", &[&Arg::Value(1)]);
        assert_eq!(Ok(Some(1)), result);

        let result = fn_call::<i32>("tests.with_two_defaults", &[&Arg::Value(1), &Arg::Value(1)]);
        assert_eq!(Ok(Some(2)), result);
    }

    #[pg_test]
    fn test_with_arg_and_two_defaults_no_args() {
        let result = fn_call::<i32>("tests.with_arg_and_two_defaults", &[]);
        assert_eq!(Err(FnCallError::NotDefaultArgument(0)), result);
    }

    #[pg_test]
    fn test_with_arg_and_two_defaults_no_args_1_arg() {
        let result = fn_call::<i32>("tests.with_arg_and_two_defaults", &[&Arg::Value(42)]);
        assert_eq!(Ok(Some(45)), result);
    }

    #[pg_test]
    fn test_with_arg_and_two_defaults_no_args_2_args() {
        let result =
            fn_call::<i32>("tests.with_arg_and_two_defaults", &[&Arg::Value(42), &Arg::Value(0)]);
        assert_eq!(Ok(Some(44)), result);
    }

    #[pg_test]
    fn test_with_null_default() {
        let result = fn_call::<&str>("tests.with_null_default", &[]);
        assert_eq!(Ok(None), result);

        let result = fn_call::<&str>("tests.with_null_default", &[&Arg::Value("value")]);
        assert_eq!(Ok(Some("value")), result);
    }

    #[pg_test]
    fn test_with_functional_default() {
        let result = fn_call::<&str>("tests.with_functional_default", &[]);
        assert_eq!(Ok(None), result);

        let result = fn_call::<&str>("tests.with_functional_default", &[&Arg::Value("value")]);
        assert_eq!(Ok(Some("value")), result);
    }

    #[pg_test]
    fn test_with_unspecified_default() {
        let result = fn_call::<i32>("tests.with_default", &[&Arg::Value(42)]);
        assert_eq!(Ok(Some(42)), result);
    }

    #[pg_test]
    fn test_func_with_collation() {
        // `pg_catalog.texteq` requires a collation, so we use the default
        let result =
            fn_call::<bool>("pg_catalog.texteq", &[&Arg::Value("test"), &Arg::Value("test")]);
        assert_eq!(Ok(Some(true)), result);
    }

    #[pg_test]
    fn test_text_parameter_given_int4() {
        let result = fn_call::<i32>("pg_catalog.textlen", &[&Arg::Value(7_i32)]);
        assert_eq!(
            Err(FnCallError::IncompatibleArgumentType(0, pg_sys::INT4OID, pg_sys::TEXTOID)),
            result
        );
    }

    #[pg_test]
    fn test_anyarray_parameter_given_int4() {
        let result = fn_call::<i32>("pg_catalog.array_ndims", &[&Arg::Value(7_i32)]);
        assert_eq!(
            Err(FnCallError::IncompatibleArgumentType(0, pg_sys::INT4OID, pg_sys::ANYARRAYOID)),
            result
        );
    }

    #[pg_test]
    fn test_binary_coercible_argument() {
        // `char` passes a varchar, which is binary coercible to text
        let result = fn_call::<i32>("pg_catalog.textlen", &[&Arg::Value('x')]);
        assert_eq!(Ok(Some(1)), result);
    }

    #[pg_test]
    fn test_domain_argument() {
        let result = fn_call::<i32>("pg_catalog.int4um", &[&Arg::Value(PositiveInt(5))]);
        assert_eq!(Ok(Some(-5)), result);

        // passing an int4 for the domain would skip its CHECK constraint
        let result = fn_call::<i32>("tests.fcall_positive_int", &[&Arg::Value(-5_i32)]);
        assert_eq!(
            Err(FnCallError::IncompatibleArgumentType(0, pg_sys::INT4OID, PositiveInt::type_oid())),
            result
        );
    }

    #[pg_test]
    fn test_argument_that_needs_a_cast() {
        let result = fn_call::<i64>("pg_catalog.int8abs", &[&Arg::Value(-7_i32)]);
        assert_eq!(
            Err(FnCallError::IncompatibleArgumentType(0, pg_sys::INT4OID, pg_sys::INT8OID)),
            result
        );
    }

    #[pg_test]
    fn test_polymorphic_arguments() {
        let result = fn_call::<i32>(
            "pg_catalog.array_length",
            &[&Arg::Value(vec![1, 2, 3]), &Arg::Value(1)],
        );
        assert_eq!(Ok(Some(3)), result);

        let result = fn_call::<Vec<i32>>(
            "pg_catalog.array_positions",
            &[&Arg::Value(vec![1, 2, 1]), &Arg::Value(1)],
        );
        assert_eq!(Ok(Some(vec![1, 3])), result);
    }

    #[pg_test]
    fn test_inconsistent_polymorphic_arguments() {
        let result = fn_call::<i32>(
            "tests.fcall_array_and_element",
            &[&Arg::Value(vec![1_i32]), &Arg::Value(1_i64)],
        );
        assert_eq!(
            Err(FnCallError::IncompatibleArgumentType(1, pg_sys::INT8OID, pg_sys::ANYELEMENTOID)),
            result
        );
    }

    #[pg_test]
    fn test_anycompatible_arguments_of_different_types() {
        // PostgreSQL would cast the int4 to numeric, their common type, but fn_call casts nothing
        let result = fn_call::<Vec<i32>>(
            "pg_catalog.array_positions",
            &[&Arg::Value(vec![AnyNumeric::from(1)]), &Arg::Value(7_i32)],
        );
        assert_eq!(
            Err(FnCallError::IncompatibleArgumentType(
                1,
                pg_sys::INT4OID,
                pg_sys::ANYCOMPATIBLEOID
            )),
            result
        );
    }

    #[pg_test]
    fn test_anycompatible_before_anyenum() {
        let result = fn_call::<i32>(
            "tests.fcall_compatible_and_enum",
            &[&Arg::Value(1_i32), &Arg::Value(FcallEnum("a"))],
        );
        assert!(matches!(result, Ok(Some(_))), "{result:?}");
    }

    #[pg_test]
    fn test_polymorphic_default_of_another_type() {
        // array_positions would compare the default, the int4 1, as text
        let incompatible = Err(FnCallError::IncompatibleArgumentType(
            1,
            pg_sys::INT4OID,
            pg_sys::ANYCOMPATIBLEOID,
        ));
        let texts = vec!["a".to_string()];
        let result =
            fn_call::<Vec<i32>>("tests.fcall_positions_default", &[&Arg::Value(texts.clone())]);
        assert_eq!(incompatible, result);
        let result = fn_call::<Vec<i32>>(
            "tests.fcall_positions_default",
            &[&Arg::Value(texts), &Arg::<String>::Default],
        );
        assert_eq!(incompatible, result);

        let result =
            fn_call::<Vec<i32>>("tests.fcall_positions_default", &[&Arg::Value(vec![1, 2, 1])]);
        assert_eq!(Ok(Some(vec![1, 3])), result);
    }

    #[pg_test]
    fn test_polymorphic_untyped_default() {
        // PostgreSQL would cast the default 'x' to the element type, but fn_call would pass the
        // literal's cstring
        let result =
            fn_call::<i32>("tests.fcall_position_untyped_default", &[&Arg::Value(vec![1])]);
        assert_eq!(
            Err(FnCallError::IncompatibleArgumentType(
                1,
                pg_sys::UNKNOWNOID,
                pg_sys::ANYCOMPATIBLEOID
            )),
            result
        );

        // a NULL default passes no value, so array_position looks for a NULL element
        let result = fn_call::<i32>(
            "tests.fcall_position_null_default",
            &[&Arg::Value(vec![Some(1), None])],
        );
        assert_eq!(Ok(Some(2)), result);
    }

    #[pg_test]
    fn test_polymorphic_type_argument() -> std::result::Result<(), pgrx::spi::Error> {
        let array = Spi::get_one::<AnyArray>("SELECT ARRAY[1, 2]")?.expect("array is NULL");
        let result = fn_call::<i32>("pg_catalog.array_ndims", &[&Arg::Value(array)]);
        assert_eq!(Ok(Some(1)), result);

        // nothing says whether the array's elements have the element's type
        let result = fn_call::<i32>(
            "tests.fcall_array_and_element",
            &[&Arg::Value(array), &Arg::Value(1_i32)],
        );
        assert_eq!(
            Err(FnCallError::IncompatibleArgumentType(0, pg_sys::ANYARRAYOID, pg_sys::ANYARRAYOID)),
            result
        );
        Ok(())
    }

    #[pg_test]
    fn test_null_and_default_arguments_are_not_type_checked() {
        let result = fn_call::<i32>("tests.arg_might_be_null", &[&Arg::<String>::Null]);
        assert_eq!(Ok(None), result);

        let result =
            fn_call::<i32>("tests.with_default", &[&Arg::Value(42), &Arg::<String>::Default]);
        assert_eq!(Ok(Some(42)), result);
    }

    #[pg_test]
    fn test_more_arguments_than_parameters() {
        let result =
            fn_call::<i32>("tests.with_default", &[&Arg::Value(1), &Arg::Value(2), &Arg::Value(3)]);
        assert_eq!(Err(FnCallError::TooManyArguments), result);
    }

    #[pg_test]
    fn test_any_parameter() {
        // without a call expression, pg_collation_for cannot learn its argument's type, and
        // returns NULL
        let result = fn_call::<String>("pg_catalog.pg_collation_for", &[&Arg::Value(7_i32)]);
        assert_eq!(Ok(None), result);

        // a Rust tuple has no PostgreSQL type
        let result = fn_call::<String>(
            "pg_catalog.pg_collation_for",
            &[&Arg::Value((Some(1_i32), Some(2_i32)))],
        );
        assert_eq!(
            Err(FnCallError::IncompatibleArgumentType(0, pg_sys::Oid::INVALID, pg_sys::ANYOID)),
            result
        );
    }

    #[pg_test]
    fn test_argument_type_that_is_not_a_type() {
        let no_type = Unconvertible::<NO_TYPE>::type_oid();
        let result = fn_call::<i32>("pg_catalog.textlen", &[&Arg::Value(Unconvertible::<NO_TYPE>)]);
        assert_eq!(Err(FnCallError::IncompatibleArgumentType(0, no_type, pg_sys::TEXTOID)), result);

        let result =
            fn_call::<i32>("pg_catalog.array_ndims", &[&Arg::Value(Unconvertible::<NO_TYPE>)]);
        assert_eq!(
            Err(FnCallError::IncompatibleArgumentType(0, no_type, pg_sys::ANYARRAYOID)),
            result
        );

        // check_generic_type_consistency would skip it as an untyped literal
        let unknown = Unconvertible::<{ pg_sys::UNKNOWNOID.to_u32() }>;
        let result = fn_call::<i32>("pg_catalog.array_ndims", &[&Arg::Value(unknown)]);
        assert_eq!(
            Err(FnCallError::IncompatibleArgumentType(0, pg_sys::UNKNOWNOID, pg_sys::ANYARRAYOID)),
            result
        );
    }

    #[pg_test]
    fn unknown_function() {
        let result = fn_call::<()>("undefined_function", &[]);
        assert_eq!(Err(FnCallError::UndefinedFunction), result)
    }

    #[pg_test]
    fn blank_function() {
        let result = fn_call::<()>("", &[]);
        assert_eq!(Err(FnCallError::InvalidIdentifier(String::from(""))), result)
    }

    #[pg_test]
    fn invalid_identifier() {
        let stupid_name = "q234qasf )(A*q2342";
        let result = fn_call::<()>(stupid_name, &[]);
        assert_eq!(Err(FnCallError::InvalidIdentifier(String::from(stupid_name))), result)
    }

    #[pg_test]
    #[should_panic(expected = "it worked")]
    fn fn_raises_error() {
        use std::sync::atomic::AtomicBool;

        static DID_DROP: AtomicBool = AtomicBool::new(false);

        PgTryBuilder::new(|| {
            struct Tracker;
            impl Drop for Tracker {
                fn drop(&mut self) {
                    DID_DROP.store(true, Ordering::SeqCst);
                }
            }

            let _tracker = Tracker;
            fn_call::<()>("tests.fcall_raise_error", &[&Arg::Value("error message")])
                .expect("fcall failed");

            // NB:  we need the Drop impl for Tracker to run here
        })
        .finally(|| {
            assert!(DID_DROP.load(Ordering::SeqCst));
            panic!("it worked")
        })
        .execute();
    }
}
