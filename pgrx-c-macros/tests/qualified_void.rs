//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Resolve qualified void casts without relying on unrelated header declarations.
//!
//! The inspected header contains only macros, so neither system function prototypes
//! nor typedefs can accidentally supply a missing void-pointee type. Original C
//! comparisons and Rust type checks preserve const, volatile, null-constant identity,
//! and operand evaluation across native and actual LP64/LLP64 compiler profiles.

/// Compile the original C fixture under the same inspected compiler arguments.
#[path = "support/oracle.rs"]
mod oracle;
/// Compile emitted consumers and reject erased pointer qualification.
#[path = "support/rust_oracle.rs"]
mod rust_oracle;

use pgrx_c_macros::{
    AnalysisSession, BindingCatalog, EmissionStatus, FrontendOutput, MacroScanner,
    generate_with_bindings, inspect,
};
use std::path::PathBuf;
use std::sync::Mutex;

/// Serialize libclang ownership while testing native and cross-target compiler contexts.
static SCANNER_LOCK: Mutex<()> = Mutex::new(());

/// Qualified casts and their dependent validity expressions all require successful emission.
const NAMES: &[&str] = &[
    "QVOID_CONST",
    "QVOID_VOLATILE",
    "QVOID_BOTH",
    "QVOID_REORDER",
    "QVOID_VALID",
    "QVOID_VOLATILE_VALID",
    "QVOID_BOTH_VALID",
];

/// Original C pointer types must preserve pointee qualification rather than top-level qualifiers.
const C_TYPE_PROOF: &str = r#"
_Static_assert(_Generic(QVOID_CONST((char *)0), const void *: 1, default: 0), "const void pointer");
_Static_assert(_Generic(QVOID_VOLATILE((char *)0), volatile void *: 1, default: 0), "volatile void pointer");
_Static_assert(_Generic(QVOID_BOTH((char *)0), const volatile void *: 1, default: 0), "const volatile void pointer");
_Static_assert(_Generic(QVOID_REORDER((char *)0), const volatile void *: 1, default: 0), "reordered qualifiers");
"#;

/// Instantiate the precise Rust pointer markers and every dependent validity expression.
const RUST_TYPE_PROOF: &str = r#"
fn prove(pointer: *const u8) {
    use __pgrx_c_macros::expression::{CVoid, CVolatile, Pointer, ReadOnly};
    let _: Pointer<CVoid, ReadOnly> = QVOID_CONST!(pointer).into_value();
    let _: Pointer<CVolatile<CVoid>> = QVOID_VOLATILE!(pointer).into_value();
    let _: Pointer<CVolatile<CVoid>, ReadOnly> = QVOID_BOTH!(pointer).into_value();
    let _: Pointer<CVolatile<CVoid>, ReadOnly> = QVOID_REORDER!(pointer).into_value();
    let _ = QVOID_VALID!(pointer);
    let _ = QVOID_VOLATILE_VALID!(pointer);
    let _ = QVOID_BOTH_VALID!(pointer);
}
"#;

/// Assemble actual emitted definitions without a Rust binding catalog or handwritten macro port.
fn generated(scanner: &MacroScanner, frontend: &FrontendOutput) -> String {
    let session = AnalysisSession::prepare(scanner, frontend, NAMES).unwrap();
    let generation = generate_with_bindings(&session, NAMES, &BindingCatalog::default()).unwrap();
    assert!(generation.support.c_source.is_empty(), "pointer casts need no native shim");
    let support = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../pgrx-pg-sys/src/c_macros/support.rs")
        .canonicalize()
        .unwrap();
    let mut source = format!(
        "#![allow(dead_code)]\n#[path={support:?}] pub mod __pgrx_c_macros;\n{}\n{RUST_TYPE_PROOF}\n",
        generation.support.rust,
    );
    for emission in generation.macros {
        let EmissionStatus::Emitted { rust, .. } = emission.status else {
            panic!("qualified void fixture must emit: {emission:?}");
        };
        source.push_str(&rust);
    }
    source
}

/// Compare null tests and single evaluation with C, then reject loss of const or volatile identity.
#[test]
fn qualified_void_native_casts_and_validity_match_original_c() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().unwrap();
    let header = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/qualified_void.h");
    for null in ["0", "0LL", "((void *)0)"] {
        let mut arguments = oracle::native_arguments();
        arguments.extend(["-std=c17".into(), format!("-DQVOID_NULL={null}")]);
        let frontend = inspect(&scanner, &header, &arguments, None).unwrap();
        let profile = frontend.profile();
        let rust = generated(&scanner, &frontend);
        let mut c = format!(
            "#include <stdio.h>\n{C_TYPE_PROOF}\nstatic char byte;\nstatic unsigned int count;\nstatic char *tick(void) {{ ++count; return &byte; }}\nint main(void) {{\n"
        );
        let mut consumer = String::from(
            "fn main() { let byte=0_u8; let pointer=&raw const byte; let count=std::cell::Cell::new(0_u32); let tick=|| { count.set(count.get()+1); pointer };\n",
        );
        for name in ["QVOID_VALID", "QVOID_VOLATILE_VALID", "QVOID_BOTH_VALID"] {
            c.push_str(&format!(
                "count=0; int result_{name}={name}(tick()); printf(\"%d %d %d %d %u\\n\", {name}((char *)0), {name}((const char *)0), {name}(&byte), result_{name}, count);\n"
            ));
            consumer.push_str(&format!(
                "count.set(0); let result={name}!(tick()); println!(\"{{}} {{}} {{}} {{}} {{}}\", {name}!(core::ptr::null_mut::<u8>()).get(), {name}!(core::ptr::null::<u8>()).get(), {name}!(pointer).get(), result.get(), count.get());\n"
            ));
        }
        c.push_str("return 0; }\n");
        consumer.push_str("}\n");
        let original = oracle::run_c(
            &profile.compiler.executable,
            &header,
            &c,
            &profile.arguments.iter().map(String::as_str).collect::<Vec<_>>(),
            true,
        );
        assert_eq!(original, "0 0 1 1 1\n".repeat(3));
        assert_eq!(
            rust_oracle::run_rust_with_cfg(
                &format!("{rust}\n{consumer}"),
                &pgrx_c_macros::support_rust_cfg(profile).unwrap(),
            ),
            original,
        );
        for (name, wrong_type) in [
            ("QVOID_CONST", "Pointer<CVoid>"),
            ("QVOID_VOLATILE", "Pointer<CVoid>"),
            ("QVOID_BOTH", "Pointer<CVolatile<CVoid>>"),
        ] {
            let diagnostics = rust_oracle::reject_rust(&format!(
                "{rust}\nfn main() {{ use __pgrx_c_macros::expression::{{CVoid,CVolatile,Pointer}}; let _: {wrong_type}={name}!(core::ptr::null_mut::<u8>()).into_value(); }}"
            ));
            assert!(diagnostics.contains("E0308"), "{name}: {diagnostics}");
        }
    }
}

/// Require genuine Clang LP64/LLP64 profiles to admit the same casts without system declarations.
#[test]
fn qualified_void_cross_target_casts_preserve_pointer_types() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().unwrap();
    let header = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/qualified_void.h");
    for target in ["x86_64-pc-windows-msvc", "x86_64-unknown-linux-gnu"] {
        let frontend =
            inspect(&scanner, &header, &[format!("--target={target}"), "-std=c17".into()], None)
                .unwrap();
        let profile = frontend.profile();
        let rust = generated(&scanner, &frontend);
        assert!(
            oracle::run_c(
                &profile.compiler.executable,
                &header,
                C_TYPE_PROOF,
                &profile.arguments.iter().map(String::as_str).collect::<Vec<_>>(),
                false,
            )
            .is_empty(),
        );
        rust_oracle::check_rust_with_cfg(
            &rust,
            &pgrx_c_macros::support_rust_cfg(profile).unwrap(),
            target,
        );
    }
}
