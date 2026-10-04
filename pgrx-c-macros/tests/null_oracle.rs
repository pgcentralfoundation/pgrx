//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Compare C null-constant rules with generated Rust expression boundaries.
//!
//! The fixtures distinguish literal zero, folded zero, ordinary integer values,
//! object pointers, and callbacks. Native observations and rejected consumers
//! ensure context-specific null conversion does not become a general integer cast.

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
#[allow(dead_code)]
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
    "NULL_CONST",
    "NULL_CAST_CONST",
    "NULL_ENUM_CONST",
    "NULL_INT_CONST",
    "NULL_ALIAS",
    "NULL_PICK",
    "NULL_PICK_REVERSE",
    "NULL_BOTH",
    "NULL_EQUAL",
    "NULL_EQUAL_REVERSE",
    "NULL_NOT_EQUAL",
    "NULL_COMPARE",
    "NULL_PASS_FUN",
    "NULL_PASS_OBJECT",
    "NULL_PASS_INLINE",
    "NULL_PRESENT_INLINE",
    "NULL_TYPED_INLINE",
    "NULL_GET",
    "NULL_SIZE",
    "NULL_SELF_SIZE",
    "NULL_DISCARD",
    "NULL_CONST_QUALIFIED",
    "NULL_VOLATILE_QUALIFIED",
    "NULL_POINTER_CAST",
    "NULL_RUNTIME_CAST",
    "NULL_COMPOUND_CONST",
    "NULL_UNSIGNED_CAST",
    "NULL_LAZY_ZERO",
    "NULL_LOGICAL_ZERO",
    "NULL_COMPOUND_INT",
    "NULL_SIGNED_INT",
    "NULL_RUNTIME_SUM",
    "NULL_COMMA_ZERO",
    "NULL_SUB_INT",
    "NULL_DELEGATED_ZERO",
    "NULL_CANCEL_VOID",
    "NULL_LITERAL_PICK",
    "NULL_NEGATIVE_VALUE",
];

/// Checks that original C null constant contexts and boundaries are preserved.
#[test]
fn original_c_null_constant_contexts_and_boundaries_are_preserved() {
    let scanner = MacroScanner::new().expect("libclang must be available");
    let header = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/null_oracle.h");
    let mut arguments = vec![
        "-std=c17".into(),
        "-ffp-contract=off".into(),
        "-Werror=incompatible-pointer-types".into(),
    ];
    #[cfg(target_os = "macos")]
    {
        let sdk = rust_oracle::run_tool(
            std::process::Command::new("xcrun").arg("--show-sdk-path"),
            "null SDK",
        );
        arguments.extend(["-isysroot".into(), sdk.trim().into()]);
    }
    let frontend =
        inspect(&scanner, &header, &arguments, None).expect("inspect original null macros");
    let session =
        AnalysisSession::prepare(&scanner, &frontend, NAMES).expect("analyze original null macros");
    for name in [
        "NULL_COMPOUND_CONST",
        "NULL_UNSIGNED_CAST",
        "NULL_LAZY_ZERO",
        "NULL_LOGICAL_ZERO",
        "NULL_COMPOUND_INT",
        "NULL_SIGNED_INT",
    ] {
        assert!(
            !session.analyze(name).expression.unwrap().integer_zero_constants.is_empty(),
            "Clang must prove {name}'s pure integer ICE zero"
        );
    }
    for name in ["NULL_RUNTIME_SUM", "NULL_COMMA_ZERO"] {
        assert!(
            session.analyze(name).expression.unwrap().integer_zero_constants.is_empty(),
            "{name} cannot establish an integer ICE"
        );
    }
    let bindings = bindgen::Builder::default()
        .rust_target(bindgen::RustTarget::stable(85, 0).unwrap())
        .rust_edition(bindgen::RustEdition::Edition2024)
        .header(header.to_str().unwrap())
        .clang_args(&frontend.profile().arguments)
        .allowlist_type("NullCallback")
        .allowlist_function("null_.*")
        .layout_tests(false)
        .generate_comments(false)
        .formatter(bindgen::Formatter::None)
        .generate()
        .expect("generate actual null bindings")
        .to_string();
    let catalog = binding_symbols::collect_bindings(
        &syn::parse_file(&bindings).unwrap(),
        session.integer_constants(),
        frontend.declarations(),
        &frontend.profile().target,
    );
    let artifact = emit_support_artifact_with_bindings(&session, NAMES, &catalog)
        .expect("derive null adapters");
    let support = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../pgrx-pg-sys/src/c_macros/support.rs")
        .canonicalize()
        .unwrap();
    let mut rust = format!(
        "#![deny(unsafe_op_in_unsafe_fn)]\n#![allow(non_snake_case,non_camel_case_types,dead_code,unused_parens)]\n#[path={support:?}] pub mod __pgrx_c_macros;\n{bindings}\n{}\n",
        artifact.rust
    );
    for emission in emit_batch_with_bindings(&session, NAMES, &catalog).unwrap() {
        let EmissionStatus::Emitted { rust: definition, .. } = emission.status else {
            panic!("null macro {} must emit: {emission:?}", emission.analysis.name);
        };
        rust.push_str(&definition);
    }
    let profile = frontend.profile();
    let mut arguments = profile.arguments.iter().map(String::as_str).collect::<Vec<_>>();
    let native = format!("{}\n{}", include_str!("fixtures/null_oracle.c"), artifact.c_source);
    let generated = rust_oracle::run_rust_linked(
        &format!("{rust}\n{}", include_str!("fixtures/null_oracle.rs")),
        &profile.compiler.executable,
        &header,
        &native,
        &arguments,
    );
    arguments.push("-DPGRX_NULL_ORACLE_MAIN");
    let original = oracle::run_c(&profile.compiler.executable, &header, &native, &arguments, true);
    assert_eq!(generated, original, "null constants must match original C types and effects");
    assert_eq!(generated.lines().count(), 61);
    arguments.pop();
    arguments.push("-pedantic-errors");
    for (name, body, c_body) in [
        (
            "runtime_integer",
            "let zero=0_i32;let _=NULL_PASS_FUN!(zero);",
            "int zero=0; (void)NULL_PASS_FUN(zero);",
        ),
        (
            "native_void_pointer",
            "let pointer=core::ptr::null_mut::<core::ffi::c_void>();let _=NULL_PASS_FUN!(pointer);",
            "void *pointer=0;(void)NULL_PASS_FUN(pointer);",
        ),
        (
            "stored_macro_result",
            "let stored=NULL_CONST!();let _=NULL_PASS_FUN!(stored);",
            "void *stored=NULL_CONST();(void)NULL_PASS_FUN(stored);",
        ),
        (
            "qualified_void",
            "let _=NULL_PASS_FUN!(NULL_CONST_QUALIFIED!());",
            "(void)NULL_PASS_FUN(NULL_CONST_QUALIFIED());",
        ),
        (
            "volatile_void",
            "let _=NULL_PASS_FUN!(NULL_VOLATILE_QUALIFIED!());",
            "(void)NULL_PASS_FUN(NULL_VOLATILE_QUALIFIED());",
        ),
        (
            "nested_pointer_cast",
            "let _=NULL_PASS_FUN!(NULL_POINTER_CAST!());",
            "(void)NULL_PASS_FUN(NULL_POINTER_CAST());",
        ),
        (
            "runtime_void_cast",
            "let zero=0_i32;let _=NULL_PASS_FUN!(NULL_RUNTIME_CAST!(zero));",
            "int zero=0;(void)NULL_PASS_FUN(NULL_RUNTIME_CAST(zero));",
        ),
        (
            "conditional_pointer",
            "let _=NULL_PASS_FUN!(NULL_BOTH!(1_i32));",
            "(void)NULL_PASS_FUN(NULL_BOTH(1));",
        ),
        ("nonzero_literal", "let _=NULL_PASS_FUN!(1);", "(void)NULL_PASS_FUN(1);"),
        ("float_zero", "let _=NULL_PASS_FUN!(0.0_f64);", "(void)NULL_PASS_FUN(0.0);"),
        (
            "negative_runtime_zero",
            "let zero=0_i32;let _=NULL_PASS_FUN!(-zero);",
            "int zero=0;(void)NULL_PASS_FUN(-zero);",
        ),
        (
            "literal_stored",
            "let stored=NULL_NEGATIVE_VALUE!(0);let _=NULL_PASS_FUN!(stored);",
            "int stored=NULL_NEGATIVE_VALUE(0);(void)NULL_PASS_FUN(stored);",
        ),
        (
            "runtime_sum",
            "let zero=0_i32;let _=NULL_PASS_FUN!(NULL_RUNTIME_SUM!(zero));",
            "int zero=0;(void)NULL_PASS_FUN(NULL_RUNTIME_SUM(zero));",
        ),
        (
            "comma_zero",
            "let _=NULL_PASS_FUN!(NULL_COMMA_ZERO!());",
            "(void)NULL_PASS_FUN(NULL_COMMA_ZERO());",
        ),
        (
            "cancelled_void_identity",
            "let _=NULL_PASS_FUN!(NULL_CANCEL_VOID!());",
            "(void)NULL_PASS_FUN(NULL_CANCEL_VOID());",
        ),
    ] {
        let diagnostic =
            rust_oracle::reject_rust(&format!("{rust}\nfn main(){{unsafe{{{body}}}}}"));
        assert!(
            diagnostic.contains("E0277") || diagnostic.contains("E0308"),
            "{name}: {diagnostic}"
        );
        let diagnostic = rust_oracle::reject_c_invocation(
            &profile.compiler.executable,
            &header,
            &format!("void fixture(void){{{c_body}}}"),
            &arguments,
        );
        assert!(!diagnostic.is_empty(), "C must reject {name}");
    }
}
