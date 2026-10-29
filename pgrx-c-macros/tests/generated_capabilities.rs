//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Render a generated capability only where batch generation created it.
//!
//! Callback signatures, original function address getters and field and offset
//! adapters exist only in generated support. A caller's catalog cannot assert one,
//! so single-macro emission refuses a macro that needs one, even when its catalog
//! comes from real bindgen output that lets batch generation emit the same macro.

#![cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos"),
))]

/// Reuse the binding build's collector, so the catalog holds exactly what a real
/// binding build gives the generator.
#[path = "../../pgrx-bindgen/src/build/binding_symbols.rs"]
mod binding_symbols;
/// Find the platform's C headers for the fixture's `offsetof`.
#[path = "support/oracle.rs"]
#[allow(dead_code)] // This test needs only the SDK lookup of the shared oracle helpers.
mod oracle;

use pgrx_c_macros::{
    AnalysisSession, EmissionStatus, MacroScanner, emit_with_bindings, generate_with_bindings,
    inspect,
};
use std::path::PathBuf;

/// Classify PostgreSQL OID constants as the real binding build does; the fixture has none.
fn is_builtin_oid(name: &str) -> bool {
    name.ends_with("OID") && name != "HEAP_HASOID"
        || name.ends_with("RelationId")
        || name == "TemplateDbOid"
}

/// Each macro, the generated support it references after batch generation, and the
/// reason single-macro emission, which has no generated support, gives instead.
const CASES: [(&str, &str, &str); 4] = [
    (
        "CAPABILITY_ADDRESS",
        "__pgrx_c_generated::Address_",
        "capability_original: no verified original C function-address adapter was emitted",
    ),
    (
        "CAPABILITY_GLOBAL_CALL",
        "__pgrx_c_generated::Signature_",
        "function pointer values require a generated ABI adapter",
    ),
    (
        "CAPABILITY_FIELD",
        "__pgrx_c_generated::Field_value",
        "value: no compiler-checked field adapter is available",
    ),
    (
        "CAPABILITY_OFFSET",
        "__pgrx_c_field_marker!(@path; value)",
        "struct capability_record::value: no compiler-verified offset capability",
    ),
];

/// Refuse a macro in single-macro emission when its capability exists only as generated support.
#[test]
fn single_macro_emission_renders_no_generated_capability() {
    let scanner = MacroScanner::new().expect("libclang must be available");
    let header =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/generated_capabilities.h");
    let mut arguments = oracle::native_arguments();
    arguments.push("-std=c17".into());
    let frontend = inspect(&scanner, &header, &arguments, None).unwrap();
    let names = CASES.map(|(name, ..)| name);
    let session = AnalysisSession::prepare(&scanner, &frontend, &names).unwrap();
    let bindings = bindgen::Builder::default()
        .rust_target(bindgen::RustTarget::stable(85, 0).unwrap())
        .rust_edition(bindgen::RustEdition::Edition2024)
        .header(header.to_str().unwrap())
        .clang_args(&frontend.profile().arguments)
        .allowlist_type("capability_.*")
        .allowlist_function("capability_.*")
        .allowlist_var("capability_.*")
        .layout_tests(false)
        .generate_comments(false)
        .formatter(bindgen::Formatter::None)
        .generate()
        .expect("generate actual fixture bindings")
        .to_string();
    let catalog = binding_symbols::collect_bindings(
        &syn::parse_file(&bindings).unwrap(),
        session.integer_constants(),
        frontend.declarations(),
        &frontend.profile().target,
    );
    let generated = generate_with_bindings(&session, &names, &catalog).unwrap();
    for ((name, support, _), emission) in CASES.iter().zip(&generated.macros) {
        let EmissionStatus::Emitted { rust, .. } = &emission.status else {
            panic!("batch generation must emit {name}: {emission:?}")
        };
        assert!(rust.contains(support), "{name} must use the generated {support}");
    }
    for (name, _, expected) in CASES {
        let emission = emit_with_bindings(&session, name, &catalog);
        let EmissionStatus::Skipped { reason } = &emission.status else {
            panic!("{name} rendered a capability that was never generated: {emission:?}")
        };
        assert!(reason.message.contains(expected), "{name}: {reason:?}");
    }
}
