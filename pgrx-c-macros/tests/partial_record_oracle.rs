//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Check native aggregate calls over partially initialized C storage.
//!
//! The oracle transports raw record bytes without constructing a Rust value for
//! uninitialized fields. Selected field observations and native calls ensure
//! adapters preserve C behavior without reading or validating untouched storage.

/// Reuse the binding build's collector so fixture tests reconcile exactly the Rust facts used
/// in production generation.
#[path = "../../pgrx-bindgen/src/build/binding_symbols.rs"]
mod binding_symbols;
/// Run original C headers through the bounded independent oracle harness.
#[path = "support/oracle.rs"]
mod oracle;
/// Compile generated consumers and paired negative cases through the bounded Rust oracle
/// harness.
#[path = "support/rust_oracle.rs"]
#[allow(dead_code)] // Fixtures use different subsets of shared bounded oracle helpers.
mod rust_oracle;

use pgrx_c_macros::{
    AnalysisSession, EmissionStatus, MacroScanner, emit_batch_with_bindings,
    emit_support_artifact_with_bindings, inspect,
};
use std::path::PathBuf;

/// Classify PostgreSQL OID constants so fixture bindgen uses the same checked-wrapper boundary
/// as the real binding build.
fn is_builtin_oid(name: &str) -> bool {
    name.ends_with("OID") && name != "HEAP_HASOID"
        || name.ends_with("RelationId")
        || name == "TemplateDbOid"
}
/// Selected fixture macro names; explicit selection also exercises demand-driven adapter
/// generation.
const NAMES: &[&str] = &[
    "PARTIAL_RECORD",
    "PARTIAL_IDENTITY",
    "PARTIAL_FIRST",
    "PARTIAL_INNER",
    "PARTIAL_TAKE",
    "PARTIAL_ASSIGN",
    "PARTIAL_MUTATE",
    "PARTIAL_CHOOSE",
    "PARTIAL_SIZE",
    "PARTIAL_TEMP_INNER",
    "PARTIAL_DISCARD_RECORD",
    "PARTIAL_DISCARD",
    "PARTIAL_DISCARD_VALUE",
];

/// Checks that aggregate native calls copy uninitialized fields without materializing them.
#[test]
fn aggregate_native_calls_copy_uninitialized_fields_without_materializing_them() {
    let scanner = MacroScanner::new().expect("libclang must be available");
    let header =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/partial_record_oracle.h");
    let mut arguments = vec!["-std=c17".into(), "-ffp-contract=off".into()];
    #[cfg(target_os = "macos")]
    {
        let sdk = rust_oracle::run_tool(
            std::process::Command::new("xcrun").arg("--show-sdk-path"),
            "partial record SDK",
        );
        arguments.extend(["-isysroot".into(), sdk.trim().into()]);
    }
    let frontend =
        inspect(&scanner, &header, &arguments, None).expect("inspect native aggregate definitions");
    let session = AnalysisSession::prepare(&scanner, &frontend, NAMES)
        .expect("analyze aggregate macro expressions");
    let source = bindgen::Builder::default()
        .rust_target(bindgen::RustTarget::stable(85, 0).unwrap())
        .rust_edition(bindgen::RustEdition::Edition2024)
        .header(header.to_str().unwrap())
        .clang_args(&frontend.profile().arguments)
        .allowlist_type("Partial.*")
        .allowlist_function("partial_.*")
        .rustified_enum("PartialState")
        .no_copy("PartialDiscardRecord")
        .layout_tests(false)
        .generate_comments(false)
        .formatter(bindgen::Formatter::None)
        .generate()
        .expect("generate actual aggregate bindings")
        .to_string();
    let mut bindings = binding_symbols::collect_bindings(
        &syn::parse_file(&source).unwrap(),
        session.integer_constants(),
        frontend.declarations(),
        &frontend.profile().target,
    );
    assert!(!bindings.records["PartialDiscardRecord"].copy);
    bindings.ffi_boundary = Some(vec!["ffi".into(), "boundary".into()]);
    let artifact = emit_support_artifact_with_bindings(&session, NAMES, &bindings)
        .expect("derive raw aggregate call adapters");
    assert!(artifact.rust.contains("MaybeUninit<crate::PartialRecord>"));
    assert!(artifact.rust.contains("MaybeUninit<crate::PartialDiscardRecord>"));
    assert!(!artifact.c_source.contains("PARTIAL_"), "native helpers must not forward C macros");
    let support = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../pgrx-pg-sys/src/c_macros/support.rs")
        .canonicalize()
        .unwrap();
    let mut rust = format!(
        "#![deny(unsafe_op_in_unsafe_fn)]\n#![allow(non_snake_case,non_camel_case_types,dead_code,unused_parens)]\n#[path={support:?}]pub mod __pgrx_c_macros;\n{source}\n{}\n",
        artifact.rust
    );
    for emission in emit_batch_with_bindings(&session, NAMES, &bindings).unwrap() {
        let EmissionStatus::Emitted { rust: definition, .. } = emission.status else {
            panic!("aggregate macro must emit: {emission:?}")
        };
        rust.push_str(&definition);
    }
    let native = format!(
        "{}\n{}",
        include_str!("fixtures/partial_record_oracle_native.c"),
        artifact.c_source
    );
    let profile = frontend.profile();
    let arguments = profile.arguments.iter().map(String::as_str).collect::<Vec<_>>();
    let translated = rust_oracle::run_rust_linked(
        &format!("{rust}\n{}", include_str!("fixtures/partial_record_oracle.rs")),
        &profile.compiler.executable,
        &header,
        &native,
        &arguments,
    );
    let original = oracle::run_c(
        &profile.compiler.executable,
        &header,
        &format!("{native}\n{}", include_str!("fixtures/partial_record_oracle.c")),
        &arguments,
        true,
    );
    assert_eq!(original.lines().count(), 30, "native aggregate corpus completeness");
    assert!(original.ends_with("discard 2 2\n"), "void casts must preserve operand side effects");
    assert_eq!(translated, original, "raw aggregate values must match selected original C fields");
    let guard = "mod ffi{pub unsafe fn boundary<R,F:FnOnce()->R>(call:F)->R{call()}}";
    for body in [
        "let value=unsafe{PARTIAL_RECORD!(1)}.into_value();let _=value.assume_initialized();",
        "let value=unsafe{PARTIAL_RECORD!(1)}.get();let _=value.untouched;",
    ] {
        let diagnostic = rust_oracle::reject_rust(&format!("{rust}\n{guard}\nfn main(){{{body}}}"));
        assert!(
            diagnostic.contains("E0133") || diagnostic.contains("E0609"),
            "whole record extraction requires an initialization proof: {diagnostic}"
        );
    }
}
