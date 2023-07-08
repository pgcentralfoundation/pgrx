#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    #[allow(unused_imports)]
    use crate as pgrx_tests;

    use pgrx::prelude::*;

    #[pg_test]
    #[ignore = "This test should be ignored."]
    fn test_for_ignore_attribute() {
        assert_eq!(true, true);
    }

    #[pg_test]
    #[should_panic(expected = "I should panic")]
    fn test_for_should_panic_attribute() {
        assert_eq!(1, 2, "I should panic");
    }
}
