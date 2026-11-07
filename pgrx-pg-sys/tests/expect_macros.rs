//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Exercise generated prediction-hint macros through pgrx-pg-sys exports.
//!
//! Runtime checks ensure C result identity and operand effects survive the public
//! entry point. The standalone compiler witnesses separately establish branch
//! weights; these integration tests check usable exported behavior. Macros with
//! backend function or global dependencies belong in pgrx-unit-tests instead.

#![cfg(not(docsrs))]
#![deny(unsafe_op_in_unsafe_fn)]

use core::cell::Cell;
use pgrx_pg_sys as pg;

/// Use this build's original headers and compiler invocation as an independent semantic oracle.
#[path = "support/postgres_oracle.rs"]
mod postgres_oracle;

/// Record exact C type, width, truth and operand effects for each public prediction macro.
/// PostgreSQL chooses either a builtin returning long or a comparison returning int according
/// to its actual preprocessing environment; neither the OS nor the Rust storage width decides
/// the expected rank.
const ORIGINAL: &str = r#"
#include <limits.h>
#include <math.h>
#include <stdio.h>
#undef printf
/* Use libc only for observations; retain the original prediction macro definitions. */
static unsigned int calls;
static int record_int(int value) { calls++; return value; }
static bool record_bool(bool value) { calls++; return value; }
static double record_double(double value) { calls++; return value; }
static void *record_pointer(void *value) { calls++; return value; }
#define OBSERVE(name, input) do { \
    calls = 0; \
    long long result = name(input); \
    printf("%s %lld %u %zu %u\n", #name, result, \
        (unsigned int)_Generic(name(input), int: 3, long: 4), sizeof(name(input)), calls); \
} while (0)
int main(void) {
    int integers[] = {INT_MIN, -2147483647, -1, 0, 1, 17, INT_MAX};
    bool booleans[] = {false, true};
    double floats[] = {-3.75, -0.0, 0.0, 0.75, NAN, INFINITY};
    int object = 31;
    void *pointers[] = {NULL, &object};
    unsigned int index;
    for (index = 0; index < sizeof integers / sizeof integers[0]; index++) {
        OBSERVE(likely, record_int(integers[index]));
        OBSERVE(unlikely, record_int(integers[index]));
    }
    for (index = 0; index < sizeof booleans / sizeof booleans[0]; index++) {
        OBSERVE(likely, record_bool(booleans[index]));
        OBSERVE(unlikely, record_bool(booleans[index]));
    }
    for (index = 0; index < sizeof floats / sizeof floats[0]; index++) {
        OBSERVE(likely, record_double(floats[index]));
        OBSERVE(unlikely, record_double(floats[index]));
    }
    for (index = 0; index < sizeof pointers / sizeof pointers[0]; index++) {
        OBSERVE(likely, record_pointer(pointers[index]));
        OBSERVE(unlikely, record_pointer(pointers[index]));
    }
}
"#;

/// Compare one generated result with the next original C observation, including nominal rank
/// even on platforms where int and long share the same storage width.
fn check<K: pg::__pgrx_c_macros::CInteger>(
    observations: &mut std::str::Lines<'_>,
    name: &str,
    value: pg::__pgrx_c_macros::CValue<K>,
    evaluations: u32,
) where
    K::Repr: Into<i64>,
{
    let row = observations.next().expect("original C observation for each generated invocation");
    let fields = row.split_whitespace().collect::<Vec<_>>();
    assert_eq!(fields.len(), 5, "unexpected original C oracle row: {row}");
    assert_eq!(fields[0], name);
    assert_eq!(value.get().into(), fields[1].parse::<i64>().expect("original C result"), "{row}");
    assert_eq!(K::RANK, fields[2].parse::<u8>().expect("original C rank"), "{row}");
    // The original _Generic witness admits only signed int or signed long.
    assert!(K::SIGNED, "{row}");
    assert_eq!(
        core::mem::size_of_val(&value.get()),
        fields[3].parse::<usize>().expect("original C width"),
        "{row}"
    );
    assert_eq!(evaluations, fields[4].parse::<u32>().expect("original C evaluations"), "{row}");
}

pg::__pgrx_c_classify! { @if_available likely {
pg::__pgrx_c_classify! { @if_available unlikely {
/// Checks exact original-header prediction result types, truth and single evaluation for
/// integer, boolean, floating and pointer operands under this build's compiler profile.
#[test]
fn generated_expectation_wrappers_match_original_c_types_truth_and_evaluation() {
    let original = postgres_oracle::run(ORIGINAL);
    let mut observations = original.lines();
    /// Give both generated entry points the same typed operand, tracking their effects
    /// separately so duplicate or missing evaluations cannot cancel one another out.
    macro_rules! check_pair {
        ($input:expr) => {{
            let value = $input;
            let calls = Cell::new(0_u32);
            let likely = pg::likely!({ calls.set(calls.get() + 1); value });
            check(&mut observations, "likely", likely.into_value(), calls.get());
            calls.set(0);
            let unlikely = pg::unlikely!({ calls.set(calls.get() + 1); value });
            check(&mut observations, "unlikely", unlikely.into_value(), calls.get());
        }};
    }
    for value in [i32::MIN, -2147483647_i32, -1, 0, 1, 17, i32::MAX] {
        check_pair!(value);
    }
    for value in [false, true] {
        check_pair!(value);
    }
    for value in [-3.75_f64, -0.0, 0.0, 0.75, f64::NAN, f64::INFINITY] {
        check_pair!(value);
    }
    let mut object = 31_i32;
    for pointer in [core::ptr::null_mut(), &raw mut object] {
        check_pair!(pointer);
    }
    assert!(observations.next().is_none(), "compare every original C observation");
}
} }
} }
