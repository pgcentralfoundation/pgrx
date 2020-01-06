use pg_bridge::*;
use pg_bridge_macros::*;

pg_module_magic!();

#[pg_extern]
fn test_i32_arg(a: i32) {}

#[pg_extern]
fn foo_bar() {
    let result = direct_function_call(test_i32_arg, vec![PgDatum::from(2), PgDatum::from(3)]);

    let val: i32 = result.try_into().unwrap();

    info!("some_func result={}", val);
}

#[cfg(test)]
mod tests {
    use crate::run_test;
    use crate::tests::{foo_bar, test_i32_arg};

    #[test]
    fn test_it() {
        run_test(foo_bar);
    }

    #[test]
    fn test_it2() {
        run_test(test_i32_arg);
    }
}
