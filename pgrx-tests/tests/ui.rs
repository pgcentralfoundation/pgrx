#![cfg(not(target_env = "musl"))]

use trybuild::TestCases;

#[test]
fn ui() {
    let t = TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}
