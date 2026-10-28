//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Translate static inline function bodies and compare them with the original C.
//! Bodies may use macros that expand the same way at the end of the translation unit,
//! void functions are translated, and calls between translations expand in place.

#![cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos")
))]

/// Reconcile fixture bindings using production's actual storage collector.
#[path = "../../pgrx-bindgen/src/build/binding_symbols.rs"]
mod binding_symbols;
/// Independently compile and execute the original C definitions.
#[path = "support/oracle.rs"]
mod oracle;
/// Compile generated Rust consumers and paired rejections with bounded tool execution.
#[path = "support/rust_oracle.rs"]
#[allow(dead_code)]
mod rust_oracle;

use pgrx_c_macros::{
    AnalysisSession, EmissionStatus, MacroScanner, emit_batch_with_bindings,
    emit_support_artifact_with_bindings, inspect,
};
use std::path::PathBuf;

/// Preserve the production OID classification boundary in the shared fixture collector.
fn is_builtin_oid(name: &str) -> bool {
    name.ends_with("OID") && name != "HEAP_HASOID"
        || name.ends_with("RelationId")
        || name == "TemplateDbOid"
}

/// Inline functions whose definitions the generator translates.
/// BODY_DROPPED names a macro in an argument that the enclosing macro drops unexpanded.
const TRANSLATED: &[&str] = &[
    "BODY_MACRO",
    "BODY_DROPPED",
    "BODY_BRANCH",
    "BODY_STORE",
    "BODY_COLOR",
    "BODY_LEAF",
    "BODY_CALLER",
    "BODY_RECURSIVE",
];

/// Inline functions that stay native calls, each for a reason the fixture names.
const NATIVE: &[&str] = &[
    // `return;` is outside the statement grammar.
    "BODY_EARLY",
    // A record cannot leave a translation by value.
    "BODY_PAIR",
    // The macro is defined only after the function, which used the enum constant.
    "BODY_LATE_USE",
    // The macro is redefined after the function.
    "BODY_REDEFINED_USE",
    // `__LINE__` names the header line, not the Rust invocation.
    "BODY_LINE",
    // The statement ends inside a macro argument, so its extent omits the closing `)`.
    "BODY_MACRO_ARG",
];

/// Compare translated and native inline roots with the original compiler's results.
#[test]
fn translated_bodies_match_their_original_definitions() {
    let scanner = MacroScanner::new().expect("libclang must be available");
    let header = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/inline_bodies.h");
    let mut arguments = oracle::native_arguments();
    arguments.push("-std=c17".into());
    let frontend = inspect(&scanner, &header, &arguments, None).expect("inspect definitions");
    let names = TRANSLATED.iter().chain(NATIVE).copied().collect::<Vec<_>>();
    let session = AnalysisSession::prepare_with_inline_functions(
        &scanner,
        &frontend,
        &[] as &[String],
        &[] as &[String],
        &names,
    )
    .expect("prepare inline roots");
    let bindings = bindgen::Builder::default()
        .rust_target(bindgen::RustTarget::stable(85, 0).unwrap())
        .rust_edition(bindgen::RustEdition::Edition2024)
        .header(header.to_str().unwrap())
        .clang_args(&frontend.profile().arguments)
        .allowlist_type("Body.*")
        .layout_tests(false)
        .generate_comments(false)
        .formatter(bindgen::Formatter::None)
        .generate()
        .expect("generate bindings")
        .to_string();
    let mut catalog = binding_symbols::collect_bindings(
        &syn::parse_file(&bindings).unwrap(),
        session.integer_constants(),
        frontend.declarations(),
        &frontend.profile().target,
    );
    catalog.ffi_boundary = Some(vec!["ffi".into(), "pg_guard_ffi_boundary".into()]);
    let artifact = emit_support_artifact_with_bindings(&session, &names, &catalog)
        .expect("generate native call transport");
    let support = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../pgrx-pg-sys/src/c_macros/support.rs")
        .canonicalize()
        .unwrap();
    let mut rust = format!(
        "#![deny(unsafe_op_in_unsafe_fn)]\n#![allow(non_snake_case,non_camel_case_types,unused_parens,dead_code)]\n#[path={support:?}] pub mod __pgrx_c_macros;\n{bindings}\n{}\n",
        artifact.rust
    );
    for emission in emit_batch_with_bindings(&session, &names, &catalog).unwrap() {
        let name = emission.analysis.name.clone();
        let EmissionStatus::Emitted { rust: definition, .. } = emission.status else {
            panic!("root {name} must emit: {emission:?}");
        };
        assert_eq!(
            definition.contains("Translates the original function definition"),
            TRANSLATED.contains(&name.as_str()),
            "{name}"
        );
        let compact = definition.split_whitespace().collect::<String>();
        match name.as_str() {
            "BODY_CALLER" => {
                assert!(compact.contains("$crate::BODY_LEAF!(@__pgrx_emit_value;(@compiled"));
            }
            // Expanding a recursive translation would never terminate.
            "BODY_RECURSIVE" => {
                assert!(!compact.contains("BODY_RECURSIVE!(@__pgrx_emit_value;(@compiled"));
            }
            _ => {}
        }
        rust.push_str(&definition);
    }
    let prelude = include_str!("fixtures/inline_bodies.rs").split_once("fn main()").unwrap().0;
    // A translated pointer write still needs unsafe; a translated macro body that only
    // computes a value does not.
    let error = rust_oracle::reject_rust(&format!(
        "{rust}\n{prelude}\nconst EXPECTED_GUARDS: usize = 0;\nfn main() {{ let mut value = 0_i32; BODY_STORE!(&raw mut value, 7_i32).get(); }}\n"
    ));
    assert!(error.contains("unsafe"), "{error}");
    // BODY_EARLY is called twice; each other native root, and the native self-call inside
    // the translated BODY_RECURSIVE, once.
    rust.push_str("const EXPECTED_GUARDS: usize = 8;\n");
    rust.push_str("fn pure() -> i32 { BODY_MACRO!(1_i32).get() + BODY_CALLER!(1_i32).get() }\n");
    rust.push_str(include_str!("fixtures/inline_bodies.rs"));
    let profile = frontend.profile();
    let arguments = profile.arguments.iter().map(String::as_str).collect::<Vec<_>>();
    let generated = rust_oracle::run_rust_linked(
        &rust,
        &profile.compiler.executable,
        &header,
        &artifact.c_source,
        &arguments,
    );
    let original = oracle::run_c(
        &profile.compiler.executable,
        &header,
        include_str!("fixtures/inline_bodies.c"),
        &arguments,
        true,
    );
    assert_eq!(generated, original);
    session.verify_inputs().expect("original profile stayed unchanged");
}
