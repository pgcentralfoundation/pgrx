// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.


use pgx::*;
use serde_json::*;

/// ```funcname
/// sum_array
/// ```
#[pg_extern]
fn sum_array_i32(values: Array<i32>) -> i32 {
    values.iter().map(|v| v.unwrap_or(0i32)).sum()
}

/// ```funcname
/// sum_array
/// ```
#[pg_extern]
fn sum_array_i64(values: Array<i64>) -> i64 {
    values.iter().map(|v| v.unwrap_or(0i64)).sum()
}

/// ```funcname
/// sum_array_siced
/// ```
#[pg_extern]
fn sum_array_i32_sliced(values: Array<i32>) -> i32 {
    values.as_slice().iter().sum()
}

/// ```funcname
/// sum_array_sliced
/// ```
#[pg_extern]
fn sum_array_i64_sliced(values: Array<i64>) -> i64 {
    values.as_slice().iter().sum()
}

#[pg_extern]
fn count_true(values: Array<bool>) -> i32 {
    values.iter().filter(|b| b.unwrap_or(false)).count() as i32
}

#[pg_extern]
fn count_true_sliced(values: Array<bool>) -> i32 {
    values.as_slice().iter().filter(|b| **b).count() as i32
}

#[pg_extern]
fn count_nulls(values: Array<i32>) -> i32 {
    values.iter().map(|v| v.is_none()).filter(|v| *v).count() as i32
}

#[pg_extern]
fn optional_array_arg(values: Option<Array<f32>>) -> f32 {
    values.unwrap().iter().map(|v| v.unwrap_or(0f32)).sum()
}

#[pg_extern]
fn iterate_array_with_deny_null(values: Array<i32>) {
    for _ in values.iter_deny_null() {
        // noop
    }
}

#[pg_extern]
fn optional_array_with_default(values: Option<default!(Array<i32>, NULL)>) -> i32 {
    values.unwrap().iter().map(|v| v.unwrap_or(0)).sum()
}

#[pg_extern]
fn serde_serialize_array(values: Array<&str>) -> Json {
    Json(json! { { "values": values } })
}

#[pg_extern]
fn serde_serialize_array_i32(values: Array<i32>) -> Json {
    Json(json! { { "values": values } })
}

#[pg_extern]
fn serde_serialize_array_i32_deny_null(values: Array<i32>) -> Json {
    Json(json! { { "values": values.iter_deny_null() } })
}

#[pg_extern]
fn return_text_array() -> Vec<&'static str> {
    vec!["a", "b", "c", "d"]
}

#[pg_extern]
fn return_zero_length_vec() -> Vec<i32> {
    Vec::new()
}

#[cfg(any(test, feature = "pg_test"))]
mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;

    use pgx::*;
    use serde_json::json;

    #[pg_test]
    fn test_sum_array_i32() {
        let sum = Spi::get_one::<i32>("SELECT sum_array(ARRAY[1,2,3]::integer[])");
        assert!(sum.is_some());
        assert_eq!(sum.unwrap(), 6);
    }

    #[pg_test]
    fn test_sum_array_i64() {
        let sum = Spi::get_one::<i64>("SELECT sum_array(ARRAY[1,2,3]::bigint[])");
        assert!(sum.is_some());
        assert_eq!(sum.unwrap(), 6);
    }

    #[pg_test]
    fn test_sum_array_i32_sliced() {
        let sum = Spi::get_one::<i32>("SELECT sum_array_sliced(ARRAY[1,2,3]::integer[])");
        assert!(sum.is_some());
        assert_eq!(sum.unwrap(), 6);
    }

    #[pg_test]
    fn test_sum_array_i64_sliced() {
        let sum = Spi::get_one::<i64>("SELECT sum_array_sliced(ARRAY[1,2,3]::bigint[])");
        assert!(sum.is_some());
        assert_eq!(sum.unwrap(), 6);
    }

    #[pg_test(error = "attempt to add with overflow")]
    fn test_sum_array_i32_overflow() {
        Spi::get_one::<i64>(
            "SELECT sum_array(a) FROM (SELECT array_agg(s) a FROM generate_series(1, 1000000) s) x;",
        );
    }

    #[pg_test]
    fn test_count_true() {
        let cnt = Spi::get_one::<i32>("SELECT count_true(ARRAY[true, true, false, true])");
        assert!(cnt.is_some());
        assert_eq!(cnt.unwrap(), 3);
    }

    #[pg_test]
    fn test_count_true_sliced() {
        let cnt = Spi::get_one::<i32>("SELECT count_true_sliced(ARRAY[true, true, false, true])");
        assert!(cnt.is_some());
        assert_eq!(cnt.unwrap(), 3);
    }

    #[pg_test]
    fn test_count_nulls() {
        let cnt = Spi::get_one::<i32>("SELECT count_nulls(ARRAY[NULL, 1, 2, NULL]::integer[])");
        assert!(cnt.is_some());
        assert_eq!(cnt.unwrap(), 2);
    }

    #[pg_test]
    fn test_optional_array() {
        let sum = Spi::get_one::<f32>("SELECT optional_array_arg(ARRAY[1,2,3]::real[])");
        assert!(sum.is_some());
        assert_eq!(sum.unwrap(), 6f32);
    }

    #[pg_test(error = "array contains NULL")]
    fn test_array_deny_nulls() {
        Spi::run("SELECT iterate_array_with_deny_null(ARRAY[1,2,3, NULL]::int[])");
    }

    #[pg_test]
    fn test_serde_serialize_array() {
        let json = Spi::get_one::<Json>(
            "SELECT serde_serialize_array(ARRAY['one', null, 'two', 'three'])",
        )
        .expect("returned json was null");
        assert_eq!(json.0, json! {{"values": ["one", null, "two", "three"]}});
    }

    #[pg_test]
    fn test_optional_array_with_default() {
        let sum = Spi::get_one::<i32>("SELECT optional_array_with_default(ARRAY[1,2,3])")
            .expect("failed to get SPI result");
        assert_eq!(sum, 6);
    }

    #[pg_test]
    fn test_serde_serialize_array_i32() {
        let json = Spi::get_one::<Json>("SELECT serde_serialize_array_i32(ARRAY[1,2,3,null, 4])")
            .expect("returned json was null");
        assert_eq!(json.0, json! {{"values": [1,2,3,null,4]}});
    }

    #[pg_test(error = "array contains NULL")]
    fn test_serde_serialize_array_i32_deny_null() {
        Spi::get_one::<Json>("SELECT serde_serialize_array_i32_deny_null(ARRAY[1,2,3,null, 4])")
            .expect("returned json was null");
    }

    #[pg_test]
    fn test_return_text_array() {
        let rc = Spi::get_one::<bool>("SELECT ARRAY['a', 'b', 'c', 'd'] = return_text_array();")
            .expect("failed to get SPI result");
        assert!(rc)
    }

    #[pg_test]
    fn test_return_zero_length_vec() {
        let rc = Spi::get_one::<bool>("SELECT ARRAY[]::integer[] = return_zero_length_vec();")
            .expect("failed to get SPI result");
        assert!(rc)
    }
}
