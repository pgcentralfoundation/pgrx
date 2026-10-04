//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Check empty, void, and comma-expression macro semantics.
//!
//! Original C and emitted Rust report type, value, and evaluation effects. The
//! consumer deliberately uses contexts that distinguish discarding a value from
//! an unevaluated operand; unsupported cases remain explicit exclusions.

/// Run original C headers through the bounded independent oracle harness.
#[path = "support/oracle.rs"]
mod oracle;
/// Compile generated consumers and paired negative cases through the bounded Rust oracle
/// harness.
#[path = "support/rust_oracle.rs"]
mod rust_oracle;

/// Use the production scanner, analysis, and emission contracts so these checks exercise the
/// actual C macro pipeline.
use pgrx_c_macros::{
    AnalysisSession, BindingCatalog, EmissionStatus, MacroScanner, SkipReasonCode, emit,
    emit_support_with_bindings, inspect,
};
/// Keep fixture and generated-output locations explicit so consumer builds remain independent
/// of the working directory.
use std::path::PathBuf;

/// Fixture macros whose emitted behavior is compared with the original header.
const SUPPORTED: &[&str] = &[
    "EXPR_EMPTY",
    "EXPR_EMPTY_ZERO",
    "EXPR_DROP",
    "EXPR_DROP_CONSTANT",
    "EXPR_COMMA",
    "EXPR_THREE",
    "EXPR_REPEAT",
    "EXPR_DROP_THEN",
    "EXPR_LAZY",
    "EXPR_LAZY_VOID",
    "EXPR_NESTED",
    "EXPR_ATOMIC_POW2",
    "EXPR_STATEMENT",
];
/// Fixture forms deliberately excluded from successful generation and asserted separately.
const EXCLUDED: &[(&str, SkipReasonCode)] = &[
    ("EXPR_BAD_LOOP", SkipReasonCode::Statement),
    ("EXPR_BAD_PASTE", SkipReasonCode::TokenPaste),
    ("EXPR_BAD_STRINGIFY", SkipReasonCode::Stringification),
    ("EXPR_BAD_VARIADIC", SkipReasonCode::Variadic),
];

/// Resolve platform include arguments for native oracle compilation without changing the
/// fixture's C definitions.
#[cfg(target_os = "macos")]
fn native_include_arguments() -> Vec<String> {
    let sdk = rust_oracle::run_tool(
        std::process::Command::new("xcrun").arg("--show-sdk-path"),
        "Apple SDK lookup",
    );
    let sdk = sdk.trim();
    assert!(!sdk.is_empty() && std::path::Path::new(sdk).is_dir());
    vec!["-isysroot".into(), sdk.into()]
}

/// Resolve platform include arguments for native oracle compilation without changing the
/// fixture's C definitions.
#[cfg(not(target_os = "macos"))]
fn native_include_arguments() -> Vec<String> {
    Vec::new()
}

/// Checks that empty void and comma macros match original C types values and evaluation.
#[test]
fn empty_void_and_comma_macros_match_original_c_types_values_and_evaluation() {
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let header = directory.join("tests/fixtures/expression_oracle.h");
    let scanner = MacroScanner::new().expect("libclang must be available");
    let mut arguments = vec!["-std=c17".into()];
    arguments.extend(native_include_arguments());
    let frontend = inspect(&scanner, &header, &arguments, None).expect("inspect original C macros");
    let names =
        SUPPORTED.iter().copied().chain(EXCLUDED.iter().map(|(name, _)| *name)).collect::<Vec<_>>();
    let session =
        AnalysisSession::prepare(&scanner, &frontend, &names).expect("expand original C macros");
    let support = directory.join("../pgrx-pg-sys/src/c_macros/support.rs").canonicalize().unwrap();
    let mut rust = format!("#[path = {support:?}]\npub mod __pgrx_c_macros;\n");
    rust.push_str(
        &emit_support_with_bindings(&session, SUPPORTED, &BindingCatalog::default()).unwrap(),
    );
    for name in SUPPORTED {
        let emission = emit(&session, name);
        let EmissionStatus::Emitted { rust: definition, .. } = emission.status else {
            panic!("original macro {name} must emit: {emission:?}");
        };
        rust.push_str(&definition);
    }
    for &(name, expected) in EXCLUDED {
        let emission = emit(&session, name);
        let EmissionStatus::Skipped { reason } = emission.status else {
            panic!("excluded construct {name} must retain its explicit skip");
        };
        assert_eq!(reason.code, expected, "{name}: {reason:?}");
    }
    rust.push_str(include_str!("fixtures/expression_oracle.rs"));
    let profile = frontend.profile();
    let original = oracle::run_c(
        &profile.compiler.executable,
        &header,
        include_str!("fixtures/expression_oracle.c"),
        &profile.arguments.iter().map(String::as_str).collect::<Vec<_>>(),
        true,
    );
    let generated = rust_oracle::run_rust(&rust);
    assert_eq!(original.lines().count(), 21, "C corpus completeness");
    assert_eq!(
        generated, original,
        "generated macros must match original C types, bits, order and occurrence counts"
    );
}
