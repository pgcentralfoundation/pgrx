//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

#[path = "../../pgrx-bindgen/src/build/binding_symbols.rs"]
mod binding_symbols;
#[path = "support/oracle.rs"]
mod oracle;
#[path = "support/rust_oracle.rs"]
#[allow(dead_code)] // This integration needs the linked subset of the shared oracle helpers.
mod rust_oracle;

use pgrx_c_macros::{
    AnalysisSession, BindingCatalog, EmissionStatus, FrontendOutput, MacroScanner, RustBindingType,
    SkipReasonCode, TypeCategory, emit_batch_with_bindings, emit_support_artifact_with_bindings,
    emit_with_bindings, inspect,
};
use std::path::PathBuf;
use std::sync::Mutex;

static SCANNER_LOCK: Mutex<()> = Mutex::new(());

// Parent-module context required by the included production collector. The call
// fixture has no OID constants; this also preserves the collector's existing tests.
fn is_builtin_oid(name: &str) -> bool {
    name.ends_with("OID") && name != "HEAP_HASOID"
        || name.ends_with("RelationId")
        || name == "TemplateDbOid"
}
const SUPPORTED: &[&str] = &[
    "CALL_BYTE",
    "CALL_SHORT",
    "CALL_INT",
    "CALL_LONG",
    "CALL_ULONG",
    "CALL_LLONG",
    "CALL_ULL",
    "CALL_BOOL",
    "CALL_READ",
    "CALL_WRITE",
    "CALL_VOID",
    "CALL_LAZY",
    "CALL_LAZY_VOID",
    "CALL_COMMA",
    "CALL_REPEAT",
    "CALL_NESTED",
    "CALL_SEQUENCE",
    "CALL_VOID_DISCARD",
    "CALL_POINTER",
    "CALL_CONST_POINTER",
    "CALL_RECORD",
    "CALL_RECORD_SUM",
    "CALL_RECORD_NESTED",
    "CALL_RECORD_FIELD",
];
const REJECTED: &[(&str, SkipReasonCode)] = &[
    ("CALL_UNPROTOTYPED", SkipReasonCode::Call),
    ("CALL_VARIADIC", SkipReasonCode::Variadic),
    ("CALL_WRONG_ARITY", SkipReasonCode::Call),
];

fn header() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/call_oracle.h")
}

#[cfg(target_os = "macos")]
fn native_arguments() -> Vec<String> {
    let sdk =
        rust_oracle::run_tool(std::process::Command::new("xcrun").arg("--show-sdk-path"), "sdk");
    vec![
        "-std=c17".into(),
        "-ffp-contract=off".into(),
        "-Werror=shadow".into(),
        "-Werror=uninitialized".into(),
        "-isysroot".into(),
        sdk.trim().into(),
    ]
}

#[cfg(not(target_os = "macos"))]
fn native_arguments() -> Vec<String> {
    vec![
        "-std=c17".into(),
        "-ffp-contract=off".into(),
        "-Werror=shadow".into(),
        "-Werror=uninitialized".into(),
    ]
}

fn original_bindings(
    frontend: &FrontendOutput,
    session: &AnalysisSession<'_>,
) -> (String, BindingCatalog) {
    let source = bindgen::Builder::default()
        .rust_target(bindgen::RustTarget::stable(85, 0).expect("Rust 2024 requires Rust 1.85"))
        .rust_edition(bindgen::RustEdition::Edition2024)
        .header(header().to_str().unwrap())
        .clang_args(&frontend.profile().arguments)
        .allowlist_function("call_.*")
        .allowlist_type("CallRecord")
        .layout_tests(false)
        .generate_comments(false)
        .formatter(bindgen::Formatter::None)
        .generate()
        .expect("generate actual native fixture bindings")
        .to_string();
    let syntax = syn::parse_file(&source).expect("parse actual bindgen output");
    let catalog = binding_symbols::collect_bindings(
        &syntax,
        session.integer_constants(),
        frontend.declarations(),
        &frontend.profile().target,
    );
    assert_eq!(catalog.functions.len(), frontend.declarations().function_signatures.len());
    for (name, function) in &catalog.functions {
        assert_eq!(function.link_name, *name, "fixture binds the original C symbols");
    }
    (source, catalog)
}

fn emitted_source(
    session: &AnalysisSession<'_>,
    source: &str,
    catalog: &BindingCatalog,
) -> (String, String) {
    let support = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../pgrx-pg-sys/src/c_macros/support.rs")
        .canonicalize()
        .unwrap();
    let mut rust = format!(
        "#![allow(non_snake_case, non_camel_case_types, dead_code, unused_parens)]\n\
         #[path = {support:?}]\npub mod __pgrx_c_macros;\n{source}\n"
    );
    let names = SUPPORTED.iter().copied().chain(["CALL_MISSING"]).collect::<Vec<_>>();
    let artifact = emit_support_artifact_with_bindings(session, &names, catalog)
        .expect("derive verified binding adapters");
    rust.push_str(&artifact.rust);
    for emission in emit_batch_with_bindings(session, &names, catalog) {
        let EmissionStatus::Emitted { rust: definition, .. } = emission.status else {
            panic!("native macro {} must emit: {emission:?}", emission.analysis.name);
        };
        rust.push_str(&definition);
    }
    (rust, artifact.c_source)
}

#[test]
fn actual_bindgen_calls_match_original_c_abi_types_values_and_evaluation() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let frontend = inspect(&scanner, &header(), &native_arguments(), None)
        .expect("inspect native C prototypes");
    let names = SUPPORTED
        .iter()
        .copied()
        .chain(REJECTED.iter().map(|&(name, _)| name))
        .chain(["CALL_MISSING"])
        .collect::<Vec<_>>();
    let session = AnalysisSession::prepare(&scanner, &frontend, &names)
        .expect("expand original macro expressions");
    let (bindings, catalog) = original_bindings(&frontend, &session);
    assert!(
        frontend.declarations().function_signatures["call_unprototyped"]
            .signature
            .parameters
            .is_none()
    );
    for &(name, expected) in REJECTED {
        let emission = emit_with_bindings(&session, name, &catalog);
        let EmissionStatus::Skipped { reason } = emission.status else {
            panic!("unproven prototype must not emit: {emission:?}")
        };
        assert_eq!(reason.code, expected, "{name}: {reason:?}");
        assert!(reason.spans.contains(emission.analysis.provenance.as_ref().unwrap()));
    }
    let (mut rust, adapters) = emitted_source(&session, &bindings, &catalog);
    rust.push_str(include_str!("fixtures/call_oracle.rs"));
    let profile = frontend.profile();
    let arguments = profile.arguments.iter().map(String::as_str).collect::<Vec<_>>();
    let generated = rust_oracle::run_rust_linked(
        &rust,
        &profile.compiler.executable,
        &header(),
        &format!("{}\n{adapters}", include_str!("fixtures/call_oracle.c")),
        &arguments,
    );
    let mut original_arguments = arguments.clone();
    original_arguments.push("-DPGRX_CALL_ORACLE_MAIN");
    let original = oracle::run_c(
        &profile.compiler.executable,
        &header(),
        include_str!("fixtures/call_oracle.c"),
        &original_arguments,
        true,
    );
    assert_eq!(original.lines().count(), 44, "native corpus completeness");
    assert_eq!(generated, original, "actual emitted macros must match the original native C calls");

    let mut incorrect = catalog.clone();
    let parameter = &frontend.declarations().function_signatures["call_byte"]
        .signature
        .parameters
        .as_ref()
        .unwrap()[0];
    let TypeCategory::Integer(kind) = parameter.category else {
        panic!("C fixture parameter must be integral")
    };
    let integer = profile.target.integers[&kind];
    incorrect.functions.get_mut("call_byte").unwrap().parameters[0] =
        RustBindingType::Integer { signed: !integer.signed, bits: integer.bits };
    let emission = emit_with_bindings(&session, "CALL_BYTE", &incorrect);
    assert!(
        matches!(emission.status, EmissionStatus::Skipped { ref reason } if reason.code == SkipReasonCode::UnsupportedType && reason.message.contains("differs from bindgen storage")),
        "a C/bindgen prototype disagreement must reject: {emission:?}"
    );
}

#[test]
fn generated_calls_require_unsafe_and_reject_non_c_or_incompatible_prototype_inputs() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let frontend = inspect(&scanner, &header(), &native_arguments(), None).unwrap();
    let names = SUPPORTED.iter().copied().chain(["CALL_MISSING"]).collect::<Vec<_>>();
    let session = AnalysisSession::prepare(&scanner, &frontend, &names).unwrap();
    let (bindings, catalog) = original_bindings(&frontend, &session);
    let (rust, _) = emitted_source(&session, &bindings, &catalog);
    let missing_context = rust_oracle::reject_rust(&format!(
        "{rust}\nfn main() {{ unsafe {{ let _ = CALL_MISSING!(1_i32); }} }}\n"
    ));
    assert!(
        missing_context.contains("arguments do not satisfy"),
        "a free C callee requires explicit caller context: {missing_context}"
    );
    let missing_c = rust_oracle::reject_c_invocation(
        &frontend.profile().compiler.executable,
        &header(),
        "void invalid(void) { (void) CALL_MISSING(1); }",
        &frontend.profile().arguments.iter().map(String::as_str).collect::<Vec<_>>(),
    );
    assert!(
        missing_c.contains("call_missing"),
        "the original C also requires an invocation-scope declaration: {missing_c}"
    );
    let no_unsafe =
        rust_oracle::reject_rust(&format!("{rust}\nfn main() {{ let _ = CALL_BYTE!(1_i32); }}\n"));
    assert!(
        no_unsafe.contains("E0133"),
        "native calls must require caller-visible unsafe: {no_unsafe}"
    );
    let custom = rust_oracle::reject_rust(&format!(
        "{rust}\nstruct Custom;\nfn main() {{ unsafe {{ let _ = CALL_BYTE!(Custom); }} }}\n"
    ));
    assert!(
        custom.contains("IntoExpression"),
        "arbitrary Rust values must fail C input proof: {custom}"
    );
    let arguments = frontend.profile().arguments.iter().map(String::as_str).collect::<Vec<_>>();
    for (c, rust_invocation, c_diagnostic) in [
        (
            "void invalid(void) { int value = 1; (void) CALL_BYTE(&value); }",
            "let mut value = 1_i32; let _ = CALL_BYTE!((&mut value as *mut i32));",
            "pointer",
        ),
        (
            "void invalid(void) { const int value = 1; (void) CALL_WRITE(&value, 2); }",
            "let value = 1_i32; let _ = CALL_WRITE!((&value as *const i32), 2_i32);",
            "qualifier",
        ),
    ] {
        let rejected = rust_oracle::reject_c_invocation(
            &frontend.profile().compiler.executable,
            &header(),
            c,
            &arguments,
        );
        assert!(
            rejected.contains(c_diagnostic),
            "the original prototype needs the expected C constraint: {rejected}"
        );
        let rust_error = rust_oracle::reject_rust(&format!(
            "{rust}\nfn main() {{ unsafe {{ {rust_invocation} }} }}\n"
        ));
        assert!(
            rust_error.contains("Implicit") || rust_error.contains("CastTo"),
            "incompatible C prototype conversion must fail its capability: {rust_error}"
        );
    }
}
