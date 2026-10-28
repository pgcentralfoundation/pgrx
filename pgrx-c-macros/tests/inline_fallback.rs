//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! A translated inline function that cannot be rendered calls the original function
//! instead. The generated support must provide that call.

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
/// Compile generated Rust consumers with bounded tool execution.
#[path = "support/rust_oracle.rs"]
#[allow(dead_code)]
mod rust_oracle;

use pgrx_c_macros::{
    AnalysisSession, BindingCatalog, EmissionStatus, FrontendOutput, MacroScanner,
    generate_with_bindings, inspect,
};
use std::path::PathBuf;

/// Preserve the production OID classification boundary in the shared fixture collector.
fn is_builtin_oid(name: &str) -> bool {
    name.ends_with("OID") && name != "HEAP_HASOID"
        || name.ends_with("RelationId")
        || name == "TemplateDbOid"
}

/// Inline functions and macros the fixture can select.
const NAMES: &[&str] = &["FALLBACK_RANK", "FALLBACK_REFUSED", "FALLBACK_ADDRESS"];

/// The fixture header.
fn header() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/inline_fallback.h")
}

/// Inspect the fixture and collect its actual bindgen storage.
fn inspect_fixture(scanner: &MacroScanner) -> (FrontendOutput, String) {
    let mut arguments = oracle::native_arguments();
    arguments.push("-std=c17".into());
    let frontend = inspect(scanner, &header(), &arguments, None).expect("inspect definitions");
    let bindings = bindgen::Builder::default()
        .rust_target(bindgen::RustTarget::stable(85, 0).unwrap())
        .rust_edition(bindgen::RustEdition::Edition2024)
        .header(header().to_str().unwrap())
        .clang_args(&frontend.profile().arguments)
        .allowlist_type("Fallback.*")
        .layout_tests(false)
        .generate_comments(false)
        .formatter(bindgen::Formatter::None)
        .generate()
        .expect("generate bindings")
        .to_string();
    (frontend, bindings)
}

/// Collect the fixture catalog with the generated guard.
fn catalog(
    session: &AnalysisSession<'_>,
    frontend: &FrontendOutput,
    bindings: &str,
) -> BindingCatalog {
    let mut catalog = binding_symbols::collect_bindings(
        &syn::parse_file(bindings).unwrap(),
        session.integer_constants(),
        frontend.declarations(),
        &frontend.profile().target,
    );
    catalog.ffi_boundary = Some(vec!["ffi".into(), "pg_guard_ffi_boundary".into()]);
    catalog
}

/// A fallback call root keeps the native support it calls, whichever other roots demanded
/// that support first: a caller refused while rendering, or a use of only the address.
#[test]
fn fallback_calls_keep_their_generated_support() {
    let scanner = MacroScanner::new().expect("libclang must be available");
    let (frontend, bindings) = inspect_fixture(&scanner);
    let session = AnalysisSession::prepare_with_inline_functions(
        &scanner,
        &frontend,
        &[] as &[String],
        &[] as &[String],
        NAMES,
    )
    .expect("prepare roots");
    let catalog = catalog(&session, &frontend, &bindings);
    let support = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../pgrx-pg-sys/src/c_macros/support.rs")
        .canonicalize()
        .unwrap();
    let profile = frontend.profile();
    let arguments = profile.arguments.iter().map(String::as_str).collect::<Vec<_>>();
    let original = oracle::run_c(
        &profile.compiler.executable,
        &header(),
        include_str!("fixtures/inline_fallback.c"),
        &arguments,
        true,
    );
    for other in ["FALLBACK_REFUSED", "FALLBACK_ADDRESS"] {
        let names = ["FALLBACK_RANK", other];
        let generated = generate_with_bindings(&session, &names, &catalog).unwrap();
        let mut rust = format!(
            "#![deny(unsafe_op_in_unsafe_fn)]\n#![allow(non_snake_case,non_camel_case_types,non_upper_case_globals,unused_parens,dead_code)]\n#[path={support:?}] pub mod __pgrx_c_macros;\n{bindings}\n{}\n",
            generated.support.rust
        );
        for emission in &generated.macros {
            let name = emission.analysis.name.as_str();
            match (name, &emission.status) {
                ("FALLBACK_REFUSED", EmissionStatus::Skipped { reason }) => {
                    assert!(reason.message.contains("FALLBACK_HELPER"), "{reason:?}");
                }
                (_, EmissionStatus::Emitted { rust: definition, .. }) => {
                    if name == "FALLBACK_RANK" {
                        assert!(definition.contains("Typed call adapter for C inline function"));
                    }
                    rust.push_str(definition);
                }
                _ => panic!("unexpected emission: {emission:?}"),
            }
        }
        rust.push_str(
            include_str!("fixtures/inline_fallback.rs")
                .replace("/* ADDRESS */", if other == "FALLBACK_ADDRESS" { "" } else { "//" })
                .as_str(),
        );
        let generated = rust_oracle::run_rust_linked(
            &rust,
            &profile.compiler.executable,
            &header(),
            &generated.support.c_source,
            &arguments,
        );
        let expected = if other == "FALLBACK_ADDRESS" {
            original.clone()
        } else {
            original
                .lines()
                .filter(|line| line.starts_with("rank"))
                .map(|line| format!("{line}\n"))
                .collect()
        };
        assert_eq!(generated, expected, "{other}");
    }
    session.verify_inputs().expect("original profile stayed unchanged");
}
