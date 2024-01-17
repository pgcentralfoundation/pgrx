#![cfg(not(target_env = "musl"))]

use trybuild::TestCases;

/// These are tests which are intended to always fail.

#[test]
fn ui() {
    let t = TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}

/// These are tests which currently fail, but should be fixed later.
#[test]
fn todo() {
    let t = TestCases::new();
    t.compile_fail("tests/todo/*.rs");
}
