//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Compare native inline adapters and PostgreSQL guard boundaries with C.
//!
//! The consumer observes argument conversion, evaluation order, and guard counts.
//! The adapters must evaluate Rust operands before entering the boundary and
//! avoid exposing a different C prototype simply because storage is compatible.

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
#[allow(dead_code)] // This fixture uses only the linked compilation helpers.
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
    "INLINE_BYTE",
    "INLINE_SIGNED",
    "INLINE_BOOL",
    "INLINE_DOUBLE",
    "INLINE_WRITE",
    "INLINE_VOID",
    "INLINE_POINTER",
    "INLINE_CONST_POINTER",
    "INLINE_VOID_POINTER",
    "INLINE_NESTED",
    "INLINE_REPEAT",
    "INLINE_LAZY",
    "INLINE_EXISTING",
    "INLINE_WORD",
    "INLINE_RECORD",
    "INLINE_RECORD_MEMBER",
];

/// Construct the C invocation used for both inspection and the native oracle, so
/// compiler-profile differences cannot explain a mismatch.
fn arguments() -> Vec<String> {
    let mut arguments = vec![
        "-std=c17".into(),
        "-ffp-contract=off".into(),
        "-Werror=implicit-function-declaration".into(),
        "-Werror=incompatible-pointer-types".into(),
    ];
    #[cfg(target_os = "macos")]
    {
        let sdk = rust_oracle::run_tool(
            std::process::Command::new("xcrun").arg("--show-sdk-path"),
            "locate inline oracle SDK",
        );
        arguments.extend(["-isysroot".into(), sdk.trim().into()]);
    }
    arguments
}

/// Original C fixture functions linked to the emitted inline-call adapters.
const C_FUNCTIONS: &str = "void inline_oracle_reset(void) { inline_calls = 0; }\nunsigned int inline_oracle_count(void) { return inline_calls; }\nint inline_existing(int value) { ++inline_calls; return value + 1; }\n";

/// Checks that generated inline adapters preserve native coercions and guard boundaries.
#[test]
fn generated_inline_adapters_preserve_native_coercions_and_guard_boundaries() {
    let scanner = MacroScanner::new().expect("libclang must be available");
    let header = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/inline_oracle.h");
    let frontend =
        inspect(&scanner, &header, &arguments(), None).expect("inspect inline declarations");
    let rejected = ["INLINE_VARIADIC", "INLINE_UNPROTOTYPED", "INLINE_UNDEFINED"];
    let names = NAMES.iter().copied().chain(rejected).collect::<Vec<_>>();
    let session =
        AnalysisSession::prepare(&scanner, &frontend, &names).expect("parse inline macros");
    let bindings = bindgen::Builder::default()
        .rust_target(bindgen::RustTarget::stable(85, 0).unwrap())
        .rust_edition(bindgen::RustEdition::Edition2024)
        .header(header.to_str().unwrap())
        .clang_args(&frontend.profile().arguments)
        .allowlist_function("inline_oracle_.*")
        .allowlist_function("inline_existing")
        .allowlist_type("InlineRecord")
        .blocklist_type("NativeWord")
        .layout_tests(false)
        .generate_comments(false)
        .formatter(bindgen::Formatter::None)
        .generate()
        .expect("generate original callable bindings")
        .to_string();
    let syntax = syn::parse_file(&bindings).unwrap();
    let mut catalog = binding_symbols::collect_bindings(
        &syntax,
        session.integer_constants(),
        frontend.declarations(),
        &frontend.profile().target,
    );
    catalog.ffi_boundary = Some(vec!["ffi".into(), "pg_guard_ffi_boundary".into()]);
    let pgrx_c_macros::TypeCategory::Integer(word_kind) =
        frontend.declarations().types["NativeWord"].category
    else {
        panic!("native pointer word must be an integer typedef")
    };
    catalog.integer_storage.insert("NativeWord".into(), word_kind);
    assert!(!catalog.functions.contains_key("inline_byte"));
    assert!(frontend.declarations().function_signatures["inline_byte"].definition_available);
    assert!(!frontend.declarations().function_signatures["inline_undefined"].definition_available);
    for emission in emit_batch_with_bindings(&session, &rejected, &catalog).unwrap() {
        let EmissionStatus::Skipped { reason } = emission.status else {
            panic!("unproven inline capability must be rejected: {emission:?}");
        };
        assert!(
            reason.message.contains("variadic")
                || reason.message.contains("prototype")
                || reason.message.contains("partially initialized")
                || reason.message.contains("definition"),
            "{reason:?}"
        );
    }
    let artifact = emit_support_artifact_with_bindings(&session, NAMES, &catalog)
        .expect("derive original C inline primitives");
    assert!(artifact.c_source.contains("(inline_byte)(arg0)"));
    assert!(!artifact.c_source.contains("(inline_existing)"));
    assert!(artifact.rust.contains("arg0: crate::NativeWord"));
    assert!(artifact.rust.contains("-> crate::NativeWord"));
    assert!(artifact.rust.contains("pg_guard_ffi_boundary(move || raw_inline_"));
    let support = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../pgrx-pg-sys/src/c_macros/support.rs")
        .canonicalize()
        .unwrap();
    let mut rust = format!(
        "#![deny(unsafe_op_in_unsafe_fn)]\n#![allow(non_snake_case,non_camel_case_types,unused_parens,dead_code)]\n#[path={support:?}] pub mod __pgrx_c_macros;\n{bindings}\n{}\n",
        artifact.rust
    );
    for emission in emit_batch_with_bindings(&session, NAMES, &catalog).unwrap() {
        let EmissionStatus::Emitted { rust: definition, .. } = emission.status else {
            panic!("inline macro {} must emit: {emission:?}", emission.analysis.name);
        };
        rust.push_str(&definition);
    }
    let word_marker = match word_kind {
        pgrx_c_macros::IntegerKind::UnsignedLong => "CUnsignedLong",
        pgrx_c_macros::IntegerKind::UnsignedLongLong => "CUnsignedLongLong",
        _ => panic!("unsupported native pointer word identity {word_kind:?}"),
    };
    rust.push_str(&format!(
        "#[repr(transparent)] #[derive(Copy,Clone)] pub struct NativeWord(*mut ());\n\
        impl __pgrx_c_macros::sealed::Sealed for NativeWord {{}}\n\
        impl __pgrx_c_macros::expression::IntegerStorage<__pgrx_c_macros::{word_marker}> for NativeWord {{\n\
        fn decode(self) -> __pgrx_c_macros::CValue<__pgrx_c_macros::{word_marker}> {{ assert!(!ffi::INSIDE.load(core::sync::atomic::Ordering::SeqCst)); __pgrx_c_macros::CValue::new(self.0.expose_provenance() as u64) }}\n\
        fn encode(value:__pgrx_c_macros::CValue<__pgrx_c_macros::{word_marker}>) -> Self {{ assert!(!ffi::INSIDE.load(core::sync::atomic::Ordering::SeqCst)); Self(core::ptr::with_exposed_provenance_mut(value.get() as usize)) }}\n}}\n\
        impl __pgrx_c_macros::expression::IntoExpression for NativeWord {{ type Value=__pgrx_c_macros::CValue<__pgrx_c_macros::{word_marker}>; fn into_expression(self)->Self::Value {{ <Self as __pgrx_c_macros::expression::IntegerStorage<__pgrx_c_macros::{word_marker}>>::decode(self) }} }}\n"
    ));
    rust.push_str(include_str!("fixtures/inline_oracle.rs"));
    let profile = frontend.profile();
    let arguments = profile.arguments.iter().map(String::as_str).collect::<Vec<_>>();
    let generated = rust_oracle::run_rust_linked(
        &rust,
        &profile.compiler.executable,
        &header,
        &format!("{}\n{C_FUNCTIONS}", artifact.c_source),
        &arguments,
    );
    let original = oracle::run_c(
        &profile.compiler.executable,
        &header,
        &format!("{C_FUNCTIONS}\n{}", include_str!("fixtures/inline_oracle.c")),
        &arguments,
        true,
    );
    assert_eq!(generated, original);
}
