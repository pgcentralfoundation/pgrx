//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Verify preserved cross-macro calls and explained expansion fallbacks.
//!
//! Generated bodies are inspected for call shape and then executed beside the
//! unchanged C definitions. Type, value, and occurrence comparisons ensure
//! readable delegation does not change textual substitution semantics.
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
    AnalysisSession, BindingCatalog, EmissionStatus, MacroScanner, emit,
    emit_support_with_bindings, emit_with_bindings, inspect,
};
use std::fmt::Write;
use std::path::PathBuf;
use std::sync::Mutex;

/// Serialize libclang-backed inspection within this test process because its safe runtime
/// permits one active owner.
static SCANNER_LOCK: Mutex<()> = Mutex::new(());
/// Selected fixture macro names; explicit selection also exercises demand-driven adapter
/// generation.
const NAMES: &[&str] = &[
    "DELEGATE_TYPEALIGN",
    "DELEGATE_BUFFERALIGN",
    "DELEGATE_GROUPED_ALIGN",
    "DELEGATE_ALIGN_ALIAS",
    "DELEGATE_ADD",
    "DELEGATE_SWAP",
    "DELEGATE_REPEAT",
    "DELEGATE_TWICE",
    "DELEGATE_NESTED",
    "DELEGATE_CHOOSE",
    "DELEGATE_SELECT",
    "DELEGATE_ONE_ARGUMENT",
    "DELEGATE_VALUE",
    "DELEGATE_ZERO_ARGUMENT",
    "match",
    "DELEGATE_KEYWORD",
    "DELEGATE_UNUSED",
    "DELEGATE_UNUSED_WRAPPER",
    "DELEGATE_UNGROUPED",
    "DELEGATE_GROUPING_FIXED",
];

/// Return the original fixture header whose preprocessing and C definitions supply this test's
/// semantics.
fn header() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/macro_delegation.h")
}

/// Construct the C invocation used for both inspection and the native oracle, so
/// compiler-profile differences cannot explain a mismatch.
fn arguments() -> Vec<String> {
    let arguments = vec!["-std=c11".into(), "-fwrapv".into()];
    #[cfg(target_os = "macos")]
    {
        let mut arguments = arguments;
        let sdk = rust_oracle::run_tool(
            std::process::Command::new("xcrun").arg("--show-sdk-path"),
            "Apple SDK lookup",
        );
        let sdk = sdk.trim();
        assert!(!sdk.is_empty() && std::path::Path::new(sdk).is_dir());
        arguments.extend(["-isysroot".into(), sdk.into()]);
        arguments
    }
    #[cfg(not(target_os = "macos"))]
    {
        arguments
    }
}

/// Require the selected macro to have an emitted body and surface its structured skip if
/// generation rejects it.
fn emitted(source: pgrx_c_macros::MacroEmission) -> String {
    match source.status {
        EmissionStatus::Emitted { rust, .. } => rust,
        other => panic!("{} must emit: {other:?}", source.analysis.name),
    }
}

/// Locate a named generated arm by its token structure for delegation assertions.
fn arm_boundary(source: &str, marker: &str) -> usize {
    source
        .match_indices('\n')
        .find_map(|(offset, _)| {
            source[offset + 1..]
                .lines()
                .next()
                .is_some_and(|line| line.trim_start().starts_with(marker))
                .then_some(offset)
        })
        .expect("generated normalized macro arm")
}

/// Extract the generated value-context arm so assertions inspect the translated expression
/// rather than public forwarding syntax.
fn value_body(source: &str) -> &str {
    let value_arm = &source[arm_boundary(source, "(@__pgrx_emit_value;") + 1..];
    let body = value_arm.split_once("=> {").expect("generated macro rule").1;
    &body[..arm_boundary(body, "(@__pgrx_c_value;")]
}

/// Build the fixture binding catalog used to validate symbolic references and native adapters
/// against compiler facts.
fn bindings() -> BindingCatalog {
    BindingCatalog {
        macros: NAMES
            .iter()
            .filter(|&&name| name != "DELEGATE_UNGROUPED")
            .map(|name| (*name).to_owned())
            .collect(),
        ..BindingCatalog::default()
    }
}

/// Checks that direct calls preserve shape and explain fallbacks.
#[test]
fn direct_calls_preserve_shape_and_explain_fallbacks() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let frontend = inspect(&scanner, &header(), &arguments(), None).unwrap();
    let session = AnalysisSession::prepare(&scanner, &frontend, NAMES).unwrap();
    let bindings = bindings();
    for (caller, callee) in [
        ("DELEGATE_BUFFERALIGN", "DELEGATE_TYPEALIGN"),
        ("DELEGATE_GROUPED_ALIGN", "DELEGATE_TYPEALIGN"),
        ("DELEGATE_ALIGN_ALIAS", "DELEGATE_BUFFERALIGN"),
        ("DELEGATE_SWAP", "DELEGATE_ADD"),
        ("DELEGATE_TWICE", "DELEGATE_REPEAT"),
        ("DELEGATE_NESTED", "DELEGATE_REPEAT"),
        ("DELEGATE_SELECT", "DELEGATE_CHOOSE"),
        ("DELEGATE_ONE_ARGUMENT", "DELEGATE_CHOOSE"),
        ("DELEGATE_ZERO_ARGUMENT", "DELEGATE_VALUE"),
        ("DELEGATE_KEYWORD", "r#match"),
    ] {
        let source = emitted(emit_with_bindings(&session, caller, &bindings));
        let body = value_body(&source);
        assert!(body.contains(&format!("$crate::{callee}!(")), "{caller}: {body}");
    }
    let source = emitted(emit_with_bindings(&session, "DELEGATE_BUFFERALIGN", &bindings));
    let body = value_body(&source);
    assert!(!body.contains("::bitand("), "wrapper must retain its callee: {body}");

    for (caller, callee, explanation) in [
        ("DELEGATE_UNUSED_WRAPPER", "DELEGATE_UNUSED", "unused"),
        ("DELEGATE_GROUPING_FIXED", "DELEGATE_UNGROUPED", "set of emitted Rust macros"),
    ] {
        let source = emitted(emit_with_bindings(&session, caller, &bindings));
        let body = value_body(&source);
        assert!(!body.contains(&format!("$crate::{callee}!")), "{body}");
        assert!(body.contains("/* PGRX:"), "fallback must explain expansion: {body}");
        assert!(body.contains(explanation), "{body}");
    }
    let source = emitted(emit(&session, "DELEGATE_BUFFERALIGN"));
    let body = value_body(&source);
    assert!(!body.contains("$crate::DELEGATE_TYPEALIGN!"));
    assert!(body.contains("/* PGRX:"));
}

/// Checks that preserved macro calls match original C types values and evaluation.
#[test]
fn preserved_macro_calls_match_original_c_types_values_and_evaluation() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let frontend = inspect(&scanner, &header(), &arguments(), None).unwrap();
    let session = AnalysisSession::prepare(&scanner, &frontend, NAMES).unwrap();
    let bindings = bindings();
    let support = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../pgrx-pg-sys/src/c_macros/support.rs")
        .canonicalize()
        .unwrap();
    let mut rust = format!("#[path = {support:?}]\npub mod __pgrx_c_macros;\n");
    rust.push_str(&emit_support_with_bindings(&session, NAMES, &bindings).unwrap());
    for &name in NAMES.iter().filter(|&&name| name != "DELEGATE_UNGROUPED") {
        rust.push_str(&emitted(emit_with_bindings(&session, name, &bindings)));
    }
    rust.push_str(
        r#"
use __pgrx_c_macros::{CInteger, CValue, CUnsignedLong};
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
static int calls;
static int tick(void) { return ++calls; }
int main(void) {
"#,
    );
    for (name, c_arguments, rust_arguments) in [
        ("DELEGATE_BUFFERALIGN", "0U", "0_u32"),
        ("DELEGATE_BUFFERALIGN", "33", "33_i32"),
        ("DELEGATE_GROUPED_ALIGN", "-1", "-1_i32"),
        ("DELEGATE_ALIGN_ALIAS", "65", "65_i32"),
        ("DELEGATE_SWAP", "-1, 1U", "-1_i32, 1_u32"),
        ("DELEGATE_TWICE", "2147483647", "2147483647_i32"),
        ("DELEGATE_NESTED", "-7", "-7_i32"),
        ("DELEGATE_SELECT", "0, -1, 1U", "0_i32, -1_i32, 1_u32"),
        ("DELEGATE_SELECT", "1, -1, 1U", "1_i32, -1_i32, 1_u32"),
        ("DELEGATE_ONE_ARGUMENT", "-1", "-1_i32"),
        ("DELEGATE_ONE_ARGUMENT", "0", "0_i32"),
        ("DELEGATE_ZERO_ARGUMENT", "", ""),
        ("DELEGATE_KEYWORD", "(unsigned long)3", "CValue::<CUnsignedLong>::new(3)"),
        ("DELEGATE_GROUPING_FIXED", "3 + 4", "3_i32 + 4_i32"),
        ("DELEGATE_UNUSED_WRAPPER", "not_a_binding", "not_a_binding"),
    ] {
        writeln!(c, "RECORD(\"{name}\", {name}({c_arguments}));").unwrap();
        writeln!(rust, "record(\"{name}\", {name}!({rust_arguments}));").unwrap();
    }
    c.push_str(
        r#"
calls = 0;
RECORD("repeat_evaluation", DELEGATE_TWICE(tick()));
printf("calls\t%d\n", calls);
calls = 0;
RECORD("lazy_evaluation", DELEGATE_SELECT(0, tick(), 9U));
printf("calls\t%d\n", calls);
calls = 0;
RECORD("chosen_evaluation", DELEGATE_SELECT(1, tick(), 9U));
printf("calls\t%d\n", calls);
return 0;
}
"#,
    );
    rust.push_str(
        r#"
let calls = std::cell::Cell::new(0_i32);
let tick = || { calls.set(calls.get() + 1); calls.get() };
record("repeat_evaluation", DELEGATE_TWICE!(tick()));
println!("calls\t{}", calls.get());
calls.set(0);
record("lazy_evaluation", DELEGATE_SELECT!(0_i32, tick(), 9_u32));
println!("calls\t{}", calls.get());
calls.set(0);
record("chosen_evaluation", DELEGATE_SELECT!(1_i32, tick(), 9_u32));
println!("calls\t{}", calls.get());
}
"#,
    );
    let profile = frontend.profile();
    let original = oracle::run_c(
        &profile.compiler.executable,
        &profile.header,
        &c,
        &profile.arguments.iter().map(String::as_str).collect::<Vec<_>>(),
        true,
    );
    let generated = rust_oracle::run_rust(&rust);
    assert_eq!(original.lines().count(), 21, "complete original C comparison corpus");
    assert_eq!(generated, original, "C and Rust delegated types, values and evaluation");
}
