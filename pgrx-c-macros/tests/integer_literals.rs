//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Verify readable integer spelling without changing C literal typing.
//!
//! The oracle compares emitted literals with the original C tokens across radices,
//! suffixes, and C23 binary syntax. Keeping the header's digits must preserve its
//! compiler-selected rank and signedness rather than relying on Rust inference.
//!
//! These generated consumers use the runtime's Linux/macOS host family. Emission
//! still validates the inspected C ABI and flags; unsupported-profile checks remain portable.

#![cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos"),
))]

/// Run original C headers through the bounded independent oracle harness.
#[path = "support/oracle.rs"]
mod oracle;
/// Compile generated consumers and paired negative cases through the bounded Rust oracle
/// harness.
#[path = "support/rust_oracle.rs"]
mod rust_oracle;

use pgrx_c_macros::{
    AnalysisSession, BindingCatalog, EmissionStatus, FrontendError, FrontendOutput, MacroScanner,
    emit, emit_support_artifact_with_bindings, inspect,
};
use std::fmt::Write;
use std::path::PathBuf;
use std::sync::Mutex;

/// Serialize libclang-backed inspection within this test process because its safe runtime
/// permits one active owner.
static SCANNER_LOCK: Mutex<()> = Mutex::new(());

/// Integer literal spellings and expected Rust storage used to check readable emission without
/// losing C typing.
const INTEGER_CASES: &[(&str, &str)] = &[
    ("INTEGER_HEX_MAX", "0xFFFFFFFFu"),
    ("INTEGER_DECIMAL_SAME_VALUE", "4294967295i"),
    ("INTEGER_HEX_MIXED_CASE", "0xaBcDeFi"),
    ("INTEGER_HEX_UPPER_PREFIX", "0x00000ABCu"),
    ("INTEGER_HEX_UNSIGNED_SUFFIX", "0xabcDEFu"),
    ("INTEGER_HEX_UNSIGNED_LONG_LONG", "0xFFFFFFFFFFFFFFFFu"),
    ("INTEGER_HEX_LONG_LONG_UNSIGNED", "0xffffffffffffffffu"),
    ("INTEGER_DECIMAL_UNSIGNED_LONG_LONG", "18446744073709551615u"),
    ("INTEGER_DECIMAL_SIGNED_LONG_LONG", "9223372036854775807i"),
    ("INTEGER_OCTAL", "0o777i"),
    ("INTEGER_OCTAL_LEADING_ZEROES", "0o000777u"),
    ("INTEGER_OCTAL_MAX", "0o1777777777777777777777u"),
    ("INTEGER_ZERO", "0i"),
    ("INTEGER_ZERO_SUFFIX", "0u"),
    ("INTEGER_OCTAL_ZEROES", "0o00u"),
    ("INTEGER_HEX_ZERO", "0x000u"),
    ("INTEGER_DECIMAL_NEGATIVE", "2147483648i"),
    ("INTEGER_HEX_NEGATIVE", "0x00aFi"),
];
/// C23 binary literal spellings checked under a profile that actually accepts that syntax.
const BINARY_CASES: &[(&str, &str)] = &[
    ("INTEGER_BINARY", "0b00110101i"),
    ("INTEGER_BINARY_UPPER_PREFIX", "0b00110101u"),
    ("INTEGER_BINARY_MAX", "0b1111111111111111111111111111111111111111111111111111111111111111u"),
];

/// Return the original fixture header whose preprocessing and C definitions supply this test's
/// semantics.
fn header() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/integer_literals.h")
}

/// Construct the C invocation used for both inspection and the native oracle, so
/// compiler-profile differences cannot explain a mismatch.
fn arguments(standard: &str) -> Vec<String> {
    let arguments = vec![format!("-std={standard}")];
    #[cfg(target_os = "macos")]
    {
        let mut arguments = arguments;
        let sdk = rust_oracle::run_tool(
            std::process::Command::new("xcrun").arg("--show-sdk-path"),
            "Apple SDK lookup",
        );
        let sdk = sdk.trim();
        assert!(
            !sdk.is_empty() && std::path::Path::new(sdk).is_dir(),
            "Apple SDK lookup did not return an installed SDK directory"
        );
        arguments.extend(["-isysroot".into(), sdk.into()]);
        arguments
    }
    #[cfg(not(target_os = "macos"))]
    {
        arguments
    }
}

/// Check preserved literal spelling and compare execution with C under the same selected
/// language profile.
fn compare_literals(scanner: &MacroScanner, frontend: &FrontendOutput, cases: &[(&str, &str)]) {
    let names = cases.iter().map(|&(name, _)| name).collect::<Vec<_>>();
    let session = AnalysisSession::prepare(scanner, frontend, &names).unwrap();
    let support = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../pgrx-pg-sys/src/c_macros/support.rs")
        .canonicalize()
        .unwrap();
    let mut rust = format!("#[path = {support:?}]\npub mod __pgrx_c_macros;\n");
    let artifact =
        emit_support_artifact_with_bindings(&session, &names, &BindingCatalog::default()).unwrap();
    assert!(artifact.c_source.is_empty(), "literal fixtures require no native adapters");
    rust.push_str(&artifact.rust);
    for &(name, spelling) in cases {
        let emission = emit(&session, name);
        let EmissionStatus::Emitted { rust: source, .. } = emission.status else {
            panic!("original C macro {name} must emit: {emission:?}");
        };
        let body = source.split_once("=> {").expect("generated macro rule").1;
        assert!(
            body.split("::new(")
                .skip(1)
                .any(|argument| argument.trim_start().starts_with(spelling)),
            "{name}: {body}"
        );
        rust.push_str(&source);
    }
    rust.push_str(
        r#"
use __pgrx_c_macros::{CInteger, CValue};
fn record<T: __pgrx_c_macros::IntoCValue>(name: &str, value: T) {
    let value = value.into_c_value();
    let kind = std::any::type_name::<T::Kind>().rsplit("::").next().unwrap();
    println!("{name}\t{kind}\t{}\t{:032x}", T::Kind::BITS, T::Kind::encode(value.get()));
}
fn main() {
"#,
    );
    let mut c = String::from(
        r#"
#include <limits.h>
#include <stdio.h>
#define KIND(value) _Generic((value), int: "CInt", unsigned int: "CUnsignedInt", \
    long: "CLong", unsigned long: "CUnsignedLong", \
    long long: "CLongLong", unsigned long long: "CUnsignedLongLong")
#define RECORD(name, expression) do { \
    __typeof__(expression) value = (expression); \
    const unsigned __int128 bits = (unsigned __int128)value; \
    printf("%s\t%s\t%u\t%016llx%016llx\n", name, KIND(value), \
        (unsigned)(sizeof(value) * CHAR_BIT), (unsigned long long)(bits >> 64), \
        (unsigned long long)bits); \
} while (0)
int main(void) {
"#,
    );
    for &(name, _) in cases {
        writeln!(c, "RECORD(\"{name}\", {name}(undefined_unused_binding));").unwrap();
        writeln!(rust, "record(\"{name}\", {name}!(undefined_unused_binding));").unwrap();
    }
    c.push_str("return 0; }\n");
    rust.push_str("}\n");
    let profile = frontend.profile();
    let original = oracle::run_c(
        &profile.compiler.executable,
        &profile.header,
        &c,
        &profile.arguments.iter().map(String::as_str).collect::<Vec<_>>(),
        true,
    );
    let generated = rust_oracle::run_rust(&rust);
    assert_eq!(original.lines().count(), cases.len(), "original C corpus completeness");
    assert_eq!(generated.lines().count(), cases.len(), "generated Rust corpus completeness");
    assert_eq!(generated, original, "original C and generated Rust literal types and values");
}

/// Checks that integer literals keep radix digits and C types against original c.
#[test]
fn integer_literals_keep_radix_digits_and_c_types_against_original_c() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let frontend = inspect(&scanner, &header(), &arguments("c11"), None).unwrap();
    compare_literals(&scanner, &frontend, INTEGER_CASES);
}

/// Checks that binary literals keep digits and C types under c23.
#[test]
fn binary_literals_keep_digits_and_c_types_under_c23() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let frontend = match inspect(&scanner, &header(), &arguments("c23"), None) {
        Ok(frontend) => frontend,
        Err(FrontendError::CompilerFailed { diagnostics, .. })
            if diagnostics
                .lines()
                .any(|line| line.ends_with("error: invalid value 'c23' in '-std=c23'")) =>
        {
            eprintln!("C23 binary-literal spelling probe unavailable: Clang rejects -std=c23");
            return;
        }
        Err(error) => panic!("inspect original literal definitions under C23: {error}"),
    };
    compare_literals(&scanner, &frontend, BINARY_CASES);
}
