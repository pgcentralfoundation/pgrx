use pg_bridge::*;
use pg_bridge_macros::*;

pg_module_magic!();

#[pg_extern]
fn add_two_numbers(a: i32, b: i32) -> i32 {
    a + b
}

mod tests {
    #[allow(unused_imports)]
    use crate as pg_bridge_tests;

    use pg_bridge::*;
    use pg_bridge_macros::*;

    #[pg_test]
    fn test_add_two_numbers() {
        assert_eq!(super::add_two_numbers(2, 3), 5);
    }
}
