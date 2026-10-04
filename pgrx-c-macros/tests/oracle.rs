//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Establish independent C observations before relying on translated Rust.
//!
//! Original headers are compiled directly and report C type, result, and operand
//! counts. These expectations cover synthetic integer macros and configured
//! PostgreSQL headers; handwritten pgrx ports are never the semantic oracle.

/// Run original C headers through the bounded independent oracle harness.
#[path = "support/oracle.rs"]
mod oracle;

/// Keep catalogs, output maps, and observation sets deterministic for exact selection and
/// publication comparisons.
use std::collections::BTreeMap;
/// Keep fixture and generated-output locations explicit so consumer builds remain independent
/// of the working directory.
use std::path::{Path, PathBuf};

/// Represent one independent C result together with its type and evaluation count.
#[derive(Debug, PartialEq, Eq)]
struct Observation {
    /// Original C expression type reported independently by the native oracle.
    c_type: String,
    /// Recorded numeric result for cross-language comparison.
    value: i128,
    /// Number of observable operand evaluations in the original C invocation.
    evaluations: u32,
}

/// Parse the C oracle's type, value, and evaluation columns into named observations.
fn observations(output: &str) -> BTreeMap<String, Observation> {
    let mut records = BTreeMap::new();
    for line in output.lines() {
        let fields = line.split('\t').collect::<Vec<_>>();
        assert_eq!(fields.len(), 4, "invalid C oracle record: {line}");
        let observation = Observation {
            c_type: fields[1].into(),
            value: fields[2].parse().expect("C result fits i128"),
            evaluations: fields[3].parse().expect("C counter fits u32"),
        };
        assert!(records.insert(fields[0].into(), observation).is_none(), "duplicate oracle case");
    }
    records
}

/// Checks that original C integer macros establish types values and evaluation counts.
#[test]
fn original_c_integer_macros_establish_types_values_and_evaluation_counts() {
    let header = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/oracle_integer.h");
    let output = oracle::run_c(
        Path::new("clang"),
        &header,
        r#"
#include <inttypes.h>
#include <limits.h>
#include <stdio.h>

#define TYPE_NAME(value) _Generic((value), \
    _Bool: "bool", char: "char", signed char: "signed_char", \
    unsigned char: "unsigned_char", short: "short", unsigned short: "unsigned_short", \
    int: "int", unsigned int: "unsigned_int", long: "long", unsigned long: "unsigned_long", \
    long long: "long_long", unsigned long long: "unsigned_long_long")
#define EXPECT_TYPE(value, type) _Static_assert(_Generic((value), type: 1, default: 0), #value)
#define RECORD(name, value) do { \
    const intmax_t observed = (value); \
    printf("%s\t%s\t%" PRIdMAX "\t%u\n", name, TYPE_NAME(value), observed, evaluations); \
} while (0)
#define RECORD_UNSIGNED(name, value) do { \
    const uintmax_t observed = (value); \
    printf("%s\t%s\t%" PRIuMAX "\t%u\n", name, TYPE_NAME(value), observed, evaluations); \
} while (0)

_Static_assert(CHAR_BIT == 8 && INT_MAX == 2147483647, "native oracle corpus requires 32-bit int");
EXPECT_TYPE(ORACLE_ADD((uint8_t) 1, (uint8_t) 2), int);
EXPECT_TYPE(ORACLE_ADD((_Bool) 1, (_Bool) 1), int);
EXPECT_TYPE(ORACLE_ADD((int16_t) 1, (uint16_t) 2), int);
EXPECT_TYPE(ORACLE_ADD(-1, 1U), unsigned int);
EXPECT_TYPE(ORACLE_ADD(1L, 2LL), long long);
EXPECT_TYPE(ORACLE_COMPARE(1U, -1), int);
EXPECT_TYPE(ORACLE_COMPLEMENT((uint8_t) 0), int);
EXPECT_TYPE(ORACLE_CAST_U8(-1), uint8_t);
EXPECT_TYPE(ORACLE_HEX, unsigned int);
EXPECT_TYPE(ORACLE_AND(1, 2), int);
EXPECT_TYPE(ORACLE_CHOOSE(1, -1, 1U), unsigned int);

static unsigned evaluations;
static int tick(int value) { ++evaluations; return value; }

int main(void) {
    RECORD("promote_u8", ORACLE_ADD((uint8_t) 255, (uint8_t) 1));
    RECORD("promote_bool", ORACLE_ADD((_Bool) 1, (_Bool) 1));
    RECORD("promote_i16_u16", ORACLE_ADD((int16_t) -1, (uint16_t) 65535));
    RECORD_UNSIGNED("mixed_signed_unsigned", ORACLE_ADD(-1, 0U));
    RECORD("mixed_rank", ORACLE_ADD(1L, 2LL));
    RECORD("comparison_int", ORACLE_COMPARE(1U, -1));
    RECORD("complement_promotes", ORACLE_COMPLEMENT((uint8_t) 0));
    RECORD("cast_narrow", ORACLE_CAST_U8(-1));
    RECORD("cast_signed", ORACLE_CAST_I16(-123));
    RECORD_UNSIGNED("unsigned_wrap", ORACLE_UNSIGNED_WRAP(UINT_MAX));
    RECORD_UNSIGNED("hex_type", ORACLE_HEX);
    RECORD("decimal_type", ORACLE_DECIMAL);
    RECORD("textual_grouping", ORACLE_UNGROUPED(1 + 2));
    RECORD("negative_division", ORACLE_DIVIDE(-7, 3));
    RECORD("negative_remainder", ORACLE_REMAINDER(-7, 3));
    RECORD_UNSIGNED("defined_shift", ORACLE_SHIFT(1U, 31));
    RECORD_UNSIGNED("conditional_conversion", ORACLE_CHOOSE(1, -1, 1U));

    // Separate function calls may occur in either order, but their independent increments
    // are sequenced within each call. The value and count are identical in either order.
    evaluations = 0;
    RECORD("repeated_argument", ORACLE_REPEAT(tick(3)));
    evaluations = 0;
    RECORD("unused_argument", ORACLE_UNUSED(tick(3)));
    evaluations = 0;
    RECORD("and_lazy", ORACLE_AND(tick(0), tick(9)));
    evaluations = 0;
    RECORD("or_lazy", ORACLE_OR(tick(1), tick(9)));
    evaluations = 0;
    RECORD("conditional_lazy", ORACLE_CHOOSE(tick(0), tick(7), tick(11)));
    evaluations = 0;
    RECORD("comma_sequencing", ORACLE_SEQUENCE(tick(5), tick(8)));

    // Every execution is in the defined domain: narrow operands promote to int,
    // unsigned addition wraps, division uses a nonzero divisor, and shifts are in range.
    evaluations = 0;
    for (unsigned left = 0; left <= 255; ++left) {
        for (unsigned right = 0; right <= 255; right += 17) {
            const int result = ORACLE_ADD((uint8_t) left, (uint8_t) right);
            printf("u8_%u_%u\t%s\t%d\t0\n", left, right,
                   TYPE_NAME(ORACLE_ADD((uint8_t) left, (uint8_t) right)), result);
        }
    }
    return 0;
}

"#,
        &[],
        true,
    );
    let records = observations(&output);
    for (name, c_type, value, evaluations) in [
        ("promote_u8", "int", 256, 0),
        ("promote_bool", "int", 2, 0),
        ("promote_i16_u16", "int", 65534, 0),
        ("mixed_signed_unsigned", "unsigned_int", 4294967295, 0),
        ("mixed_rank", "long_long", 3, 0),
        ("comparison_int", "int", 1, 0),
        ("complement_promotes", "int", -1, 0),
        ("cast_narrow", "unsigned_char", 255, 0),
        ("cast_signed", "short", -123, 0),
        ("unsigned_wrap", "unsigned_int", 0, 0),
        ("hex_type", "unsigned_int", 4294967295, 0),
        ("textual_grouping", "int", 5, 0),
        ("negative_division", "int", -2, 0),
        ("negative_remainder", "int", -1, 0),
        ("defined_shift", "unsigned_int", 2147483648, 0),
        ("conditional_conversion", "unsigned_int", 4294967295, 0),
        ("repeated_argument", "int", 6, 2),
        ("unused_argument", "int", 17, 0),
        ("and_lazy", "int", 0, 1),
        ("or_lazy", "int", 1, 1),
        ("conditional_lazy", "int", 11, 2),
        ("comma_sequencing", "int", 8, 2),
    ] {
        assert_eq!(
            records[name],
            Observation { c_type: c_type.into(), value, evaluations },
            "{name}"
        );
    }
    let decimal = &records["decimal_type"];
    assert!(matches!(decimal.c_type.as_str(), "long" | "long_long"));
    assert_eq!(decimal.value, 2147483648);
    assert_eq!(decimal.evaluations, 0);
    for left in 0..=255 {
        for right in (0..=255).step_by(17) {
            assert_eq!(
                records[&format!("u8_{left}_{right}")],
                Observation { c_type: "int".into(), value: left + right, evaluations: 0 }
            );
        }
    }
    assert_eq!(records.len(), 23 + 256 * 16);
}

/// Checks that configured PostgreSQL 18 original headers establish scalar semantics.
#[test]
#[ignore = "requires a configured native PostgreSQL 18 installation"]
fn configured_postgres_18_original_headers_establish_scalar_semantics() {
    /// Use the production scanner, analysis, and emission contracts so these checks exercise
    /// the actual C macro pipeline.
    use pgrx_c_macros::{MacroScanner, PostgresConfig};

    let postgres = PostgresConfig::resolve("pg18").expect("a native PG18 must be configured");
    assert_eq!(postgres.pg_config().major_version().unwrap(), 18);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let frontend =
        postgres.inspect(&scanner, None, &[], None).expect("resolve the native PG18 profile");
    let profile = frontend.profile();
    let arguments = profile.arguments.iter().map(String::as_str).collect::<Vec<_>>();
    let stdout = oracle::run_c(
        &profile.compiler.executable,
        &profile.header,
        include_str!("fixtures/oracle_postgres.c"),
        &arguments,
        true,
    );
    assert!(stdout.is_empty(), "the native oracle communicates failures by its exit code");
}
