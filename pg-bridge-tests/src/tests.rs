use pg_bridge::*;
use pg_bridge_macros::*;

pg_module_magic!();

#[pg_extern]
fn add_two_numbers(a: i32, b: i32) -> i32 {
    a + b
}

#[pg_extern]
fn foo_bar() {
    let result = direct_function_call(add_two_numbers, vec![PgDatum::from(2), PgDatum::from(3)]);

    let val: i32 = result.try_into().unwrap();

    info!("some_func result={}", val);
}

#[cfg(test)]
mod tests {
    use crate::run_test;
    use crate::tests::{add_two_numbers, foo_bar};

    #[test]
    fn test_it() {
        run_test(foo_bar);
    }

    #[test]
    fn test_it2() {
        run_test(add_two_numbers);
    }
}
