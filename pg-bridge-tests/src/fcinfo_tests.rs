use pg_bridge::*;
use pg_bridge_macros::*;

pg_module_magic!();

#[pg_extern]
fn add_two_numbers(a: i32, b: i32) -> i32 {
    a + b
}

#[pg_extern]
fn takes_i8(i: i8) -> i8 {
    i
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
fn takes_char(i: char) -> char {
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

mod tests {
    #[allow(unused_imports)]
    use crate as pg_bridge_tests;

    use pg_bridge::*;
    use pg_bridge_macros::*;

    #[test]
    #[ignore]
    fn make_idea_happy() {
        assert_eq!(5, 5);
    }

    #[pg_test]
    fn test_add_two_numbers() {
        assert_eq!(super::add_two_numbers(2, 3), 5);
    }

    #[pg_test]
    fn test_takes_i8() {
        let input = 42i8;
        let result = direct_function_call(super::takes_i8_wrapper, vec![PgDatum::from(input)]);
        let result: i8 = result.try_into().unwrap();
        assert_eq!(result, input);
    }

    #[pg_test]
    fn test_takes_i16() {
        let input = 42i16;
        let result = direct_function_call(super::takes_i16_wrapper, vec![PgDatum::from(input)]);
        let result: i16 = result.try_into().unwrap();
        assert_eq!(result, input);
    }

    #[pg_test]
    fn test_takes_i32() {
        let input = 42i32;
        let result = direct_function_call(super::takes_i32_wrapper, vec![PgDatum::from(input)]);
        let result: i32 = result.try_into().unwrap();
        assert_eq!(result, input);
    }

    #[pg_test]
    fn test_takes_i64() {
        let input = 42i64;
        let result = direct_function_call(super::takes_i64_wrapper, vec![PgDatum::from(input)]);
        let result: i64 = result.try_into().unwrap();
        assert_eq!(result, input);
    }

    #[pg_test]
    fn test_takes_bool() {
        let input = true;
        let result = direct_function_call(super::takes_bool_wrapper, vec![PgDatum::from(input)]);
        let result: bool = result.try_into().unwrap();
        assert_eq!(result, input);
    }

    #[pg_test]
    fn test_takes_char() {
        let input = 'A';
        let result = direct_function_call(super::takes_char_wrapper, vec![PgDatum::from(input)]);
        let result: char = result.try_into().unwrap();
        assert_eq!(result, input);
    }

    #[pg_test]
    fn test_takes_f32() {
        let input = 42.424_244;
        let result = direct_function_call(super::takes_f32_wrapper, vec![PgDatum::from(input)]);
        let result: f32 = result.try_into().unwrap();
        assert!(result.eq(&input));
    }

    #[pg_test]
    fn test_takes_f64() {
        let input = 42.424_242_424_242f64;
        let result = direct_function_call(super::takes_f64_wrapper, vec![PgDatum::from(input)]);
        let result: f64 = result.try_into().unwrap();
        assert!(result.eq(&input));
    }

    #[pg_test]
    fn test_takes_option_with_null_arg() {
        let result = direct_function_call(super::takes_option_wrapper, vec![PgDatum::null()]);
        let result: i32 = result.try_into().unwrap();
        assert_eq!(result, -1);
    }

    #[pg_test]
    fn test_takes_option_with_non_null_arg() {
        let input = 42i32;
        let result = direct_function_call(super::takes_option_wrapper, vec![PgDatum::from(input)]);
        let result: i32 = result.try_into().unwrap();
        assert_eq!(result, input);
    }

    #[pg_test]
    fn test_takes_str() {
        let input = "this is a test";
        let result = direct_function_call(super::takes_str_wrapper, vec![PgDatum::from(input)]);
        let result: &str = result.try_into().unwrap();
        assert_eq!(result, input);
    }

    #[pg_test]
    fn test_takes_string() {
        let input = "this is a test".to_string();
        let result = direct_function_call(super::takes_str_wrapper, vec![PgDatum::from(&input)]);
        let result: String = result.try_into().unwrap();
        assert_eq!(result, input);
    }

    #[pg_test]
    fn test_returns_some() {
        let result = direct_function_call(super::returns_some_wrapper, vec![]);
        let result: i32 = result.try_into().unwrap();
        assert_eq!(result, 42);
    }

    #[pg_test]
    fn test_returns_none() {
        let result = direct_function_call(super::returns_none_wrapper, vec![]);
        assert!(result.is_null())
    }
}
