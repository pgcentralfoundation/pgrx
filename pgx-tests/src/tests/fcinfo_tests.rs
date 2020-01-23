use pgx::*;

#[pg_extern]
fn add_two_numbers(a: i32, b: i32) -> i32 {
    a + b
}

#[pg_extern]
fn takes_i16(i: i16) -> i16 {
    i
}

#[pg_extern]
fn takes_i32(i: i32) -> i32 {
    i
}

#[pg_extern]
fn takes_i64(i: i32) -> i32 {
    i
}

#[pg_extern]
fn takes_bool(i: bool) -> bool {
    i
}

#[pg_extern]
fn takes_f32(i: f32) -> f32 {
    i
}

#[pg_extern]
fn takes_f64(i: f64) -> f64 {
    i
}

#[pg_extern]
fn takes_option(i: Option<i32>) -> i32 {
    match i {
        Some(i) => i,
        None => -1,
    }
}

#[pg_extern]
fn takes_str(s: &str) -> &str {
    s
}

#[pg_extern]
fn takes_string(s: String) -> String {
    s
}

#[pg_extern]
fn returns_some() -> Option<i32> {
    Some(42)
}

#[pg_extern]
fn returns_none() -> Option<i32> {
    None
}

#[pg_extern]
fn same_name(same_name: &str) -> &str {
    same_name
}

mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;

    use crate::tests::fcinfo_tests::same_name;
    use pgx::*;

    #[test]
    fn make_idea_happy() {
        assert_eq!(5, 5);
    }

    #[pg_test]
    fn test_add_two_numbers() {
        assert_eq!(super::add_two_numbers(2, 3), 5);
    }

    #[pg_test]
    unsafe fn test_takes_i16() {
        let input = 42i16;
        let result =
            direct_function_call::<i16>(super::takes_i16_wrapper, vec![input.into_datum()]);
        let result = result.expect("result is NULL");
        assert_eq!(result, input);
    }

    #[pg_test]
    unsafe fn test_takes_i32() {
        let input = 42i32;
        let result =
            direct_function_call::<i32>(super::takes_i32_wrapper, vec![input.into_datum()]);
        let result = result.expect("result is NULL");
        assert_eq!(result, input);
    }

    #[pg_test]
    unsafe fn test_takes_i64() {
        let input = 42i64;
        let result =
            direct_function_call::<i64>(super::takes_i64_wrapper, vec![input.into_datum()]);
        let result = result.expect("result is NULL");
        assert_eq!(result, input);
    }

    #[pg_test]
    unsafe fn test_takes_bool() {
        let input = true;
        let result =
            direct_function_call::<bool>(super::takes_bool_wrapper, vec![input.into_datum()]);
        let result = result.expect("result is NULL");
        assert_eq!(result, input);
    }

    #[pg_test]
    unsafe fn test_takes_f32() {
        let input = 42.424_244;
        let result =
            direct_function_call::<f32>(super::takes_f32_wrapper, vec![input.into_datum()]);
        let result = result.expect("result is NULL");
        assert!(result.eq(&input));
    }

    #[pg_test]
    unsafe fn test_takes_f64() {
        let input = 42.424_242_424_242f64;
        let result =
            direct_function_call::<f64>(super::takes_f64_wrapper, vec![input.into_datum()]);
        let result = result.expect("result is NULL");
        assert!(result.eq(&input));
    }

    #[pg_test]
    unsafe fn test_takes_option_with_null_arg() {
        let result = direct_function_call::<i32>(super::takes_option_wrapper, vec![None]);
        assert_eq!(-1, result.expect("result is NULL"))
    }

    #[pg_test]
    unsafe fn test_takes_option_with_non_null_arg() {
        let input = 42i32;
        let result =
            direct_function_call::<i32>(super::takes_option_wrapper, vec![input.into_datum()]);
        let result = result.expect("result is NULL");
        assert_eq!(result, input);
    }

    #[pg_test]
    unsafe fn test_takes_str() {
        let input = "this is a test";
        let result =
            direct_function_call::<&str>(super::takes_str_wrapper, vec![input.into_datum()]);
        let result = result.expect("result is NULL");
        assert_eq!(result, input);
    }

    #[pg_test]
    unsafe fn test_takes_string() {
        let input = "this is a test".to_string();
        let result = direct_function_call::<String>(
            super::takes_str_wrapper,
            vec![input.clone().into_datum()],
        );
        let result = result.expect("result is NULL");
        assert_eq!(result, input);
    }

    #[pg_test]
    unsafe fn test_returns_some() {
        let result = direct_function_call::<i32>(super::returns_some_wrapper, vec![]);
        assert!(result.is_some());
    }

    #[pg_test]
    unsafe fn test_returns_none() {
        let result = direct_function_call::<i32>(super::returns_none_wrapper, vec![]);
        assert!(result.is_none())
    }

    /// ensures that we can have a `#[pg_extern]` function with an argument that
    /// shares its name
    #[pg_test]
    fn test_same_name() {
        assert_eq!("test", same_name("test"));
    }
}
