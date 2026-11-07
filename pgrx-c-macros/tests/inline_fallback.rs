//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! A translated inline function that cannot be rendered calls the original function
//! instead. The generated support must provide that call, and the report must say why
//! the translation was not used.

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
    AnalysisSession, AnalysisStatus, BindingCatalog, EmissionStatus, FrontendOutput,
    IntegerBinding, IntegerBindingRepresentation, IntegerValue, MacroScanner, SkipReasonCode,
    emit_batch_with_bindings, generate_with_bindings, inspect,
};
use std::fmt::Write;
use std::path::PathBuf;

/// Preserve the production OID classification boundary in the shared fixture collector.
fn is_builtin_oid(name: &str) -> bool {
    name.ends_with("OID") && name != "HEAP_HASOID"
        || name.ends_with("RelationId")
        || name == "TemplateDbOid"
}

/// Inline functions and macros the fixture can select.
const NAMES: &[&str] =
    &["FALLBACK_RANK", "FALLBACK_REFUSED", "FALLBACK_ADDRESS", "FALLBACK_COLOR_USE"];

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
        .allowlist_function("FallbackDouble")
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

/// Check both fallbacks with the one Clang instance a process may hold.
#[test]
fn fallback_calls_keep_their_support_and_report_why() {
    let scanner = MacroScanner::new().expect("libclang must be available");
    fallback_calls_keep_their_generated_support(&scanner);
    binding_disagreements_are_reported_for_fallback_calls(&scanner);
    addresses_use_the_signatures_the_support_defines(&scanner);
}

/// A fallback call root keeps the native support it calls, whichever other roots demanded
/// that support first: a caller refused while rendering, or a use of only the address.
fn fallback_calls_keep_their_generated_support(scanner: &MacroScanner) {
    let (frontend, bindings) = inspect_fixture(scanner);
    let session = AnalysisSession::prepare_with_inline_functions(
        scanner,
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
                    // Demand planning covers only candidate analyses.
                    assert!(matches!(emission.analysis.status, AnalysisStatus::Candidate));
                    assert!(reason.message.contains("FALLBACK_HELPER"), "{reason:?}");
                }
                (_, EmissionStatus::Emitted { rust: definition, .. }) => {
                    if name == "FALLBACK_RANK" {
                        assert!(definition.contains("Typed call adapter for C inline function"));
                        let reason = emission.fallback.as_ref().expect("translation refusal");
                        assert!(reason.message.contains("FALLBACK_HELPER"), "{reason:?}");
                        // Token ranges index the translated body, which the report omits.
                        assert_eq!(reason.tokens, None);
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

/// A bindgen and Clang disagreement keeps the translation from being emitted. The
/// emission calls the original function, and its report names the disagreement.
fn binding_disagreements_are_reported_for_fallback_calls(scanner: &MacroScanner) {
    let (frontend, bindings) = inspect_fixture(scanner);
    let names = ["FALLBACK_COLOR_USE"];
    let session = AnalysisSession::prepare_with_inline_functions(
        scanner,
        &frontend,
        &[] as &[String],
        &[] as &[String],
        &names,
    )
    .expect("prepare roots");
    let mut catalog = catalog(&session, &frontend, &bindings);
    let blue = &frontend.declarations().integer_constants["FALLBACK_BLUE"];
    for value in [blue.value, IntegerValue::Signed(3)] {
        catalog.integer_constants.insert(
            "FALLBACK_BLUE".into(),
            IntegerBinding {
                path: vec!["FALLBACK_BLUE".into()],
                value,
                representation: IntegerBindingRepresentation::Primitive,
            },
        );
        let [emission] = &emit_batch_with_bindings(&session, &names, &catalog).unwrap()[..] else {
            panic!("one emission");
        };
        let EmissionStatus::Emitted { rust, .. } = &emission.status else {
            panic!("{emission:?}");
        };
        let report = serde_json::to_value(emission).unwrap();
        if value == blue.value {
            assert!(rust.contains("Translates the original function definition"));
            assert!(emission.fallback.is_none());
            assert!(report.get("fallback").is_none());
        } else {
            assert!(rust.contains("Typed call adapter for C inline function"));
            let reason = emission.fallback.as_ref().expect("translation refusal");
            assert_eq!(reason.code, SkipReasonCode::BindingValueMismatch);
            assert!(reason.message.contains("FALLBACK_BLUE"), "{reason:?}");
            assert_eq!(report["fallback"]["code"], "binding_value_mismatch");
        }
    }
}

/// A native call adds a thunk that witnesses the C function type with the C-unwind ABI,
/// while bindgen declares an external function of that type with the C ABI. The
/// conflicting witnesses leave the type without a callback signature, so the external
/// function's address is refused rather than named by a signature the support does not
/// define. The call comes from a translation that falls back or from a macro.
fn addresses_use_the_signatures_the_support_defines(scanner: &MacroScanner) {
    let (frontend, bindings) = inspect_fixture(scanner);
    let support = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../pgrx-pg-sys/src/c_macros/support.rs")
        .canonicalize()
        .unwrap();
    let profile = frontend.profile();
    let arguments = profile.arguments.iter().map(String::as_str).collect::<Vec<_>>();
    for (caller, title) in [
        ("FALLBACK_NEXT", "Typed call adapter for C inline function FALLBACK_NEXT"),
        ("FALLBACK_PLAIN_CALL", "C macro FALLBACK_PLAIN_CALL"),
    ] {
        let names = ["FALLBACK_DOUBLE_ADDRESS", caller];
        let session = AnalysisSession::prepare_with_inline_functions(
            scanner,
            &frontend,
            &[] as &[String],
            &[] as &[String],
            &names,
        )
        .expect("prepare roots");
        let catalog = catalog(&session, &frontend, &bindings);
        let generated = generate_with_bindings(&session, &names, &catalog).unwrap();
        let mut rust = format!(
            "#![deny(unsafe_op_in_unsafe_fn)]\n#![allow(non_snake_case,non_camel_case_types,non_upper_case_globals,unused_parens,dead_code)]\n#[path={support:?}] pub mod __pgrx_c_macros;\n{bindings}\n{}\n",
            generated.support.rust
        );
        for emission in &generated.macros {
            match (emission.analysis.name.as_str(), &emission.status) {
                (name, EmissionStatus::Emitted { rust: definition, .. }) if name == caller => {
                    assert!(definition.contains(title), "{caller}");
                    rust.push_str(definition);
                }
                ("FALLBACK_DOUBLE_ADDRESS", EmissionStatus::Skipped { reason }) => {
                    assert!(reason.message.contains("FallbackDouble"), "{reason:?}");
                }
                _ => panic!("unexpected emission: {emission:?}"),
            }
        }
        write!(
            rust,
            "mod ffi {{ pub unsafe fn pg_guard_ffi_boundary<T, F: FnOnce() -> T>(call: F) -> T {{ call() }} }}\n\
             fn main() {{\n\
             // SAFETY: The function reads only its by-value argument.\n\
             println!(\"call\\t{{}}\", unsafe {{ {caller}!(2_i32).get() }});\n\
             }}\n"
        )
        .unwrap();
        let actual = rust_oracle::run_rust_linked(
            &rust,
            &profile.compiler.executable,
            &header(),
            &generated.support.c_source,
            &arguments,
        );
        let expected = oracle::run_c(
            &profile.compiler.executable,
            &header(),
            &format!(
                "#include <stdio.h>\nint main(void) {{ printf(\"call\\t%d\\n\", {caller}(2)); }}\n"
            ),
            &arguments,
            true,
        );
        assert_eq!(actual, expected, "{caller}");
    }
}
