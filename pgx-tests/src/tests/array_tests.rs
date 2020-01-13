use pgx::*;

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
    values
        .iter()
        .map(|v| if v.is_none() { true } else { false })
        .filter(|v| *v)
        .count() as i32
}

mod tests {
    #[allow(unused_imports)]
    use crate as pgx_tests;

    use pgx::*;

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
}
