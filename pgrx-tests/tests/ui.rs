#![cfg(not(target_env = "musl"))]

use trybuild::TestCases;

/// These are tests which are intended to always fail.
#[test]
fn compile_fail() {
    let t = TestCases::new();
    t.compile_fail("tests/compile-fail/*.rs");
}

/// These are tests which currently fail, but should be fixed later.
#[test]
fn todo() {
    let t = TestCases::new();
    t.compile_fail("tests/todo/*.rs");
}
