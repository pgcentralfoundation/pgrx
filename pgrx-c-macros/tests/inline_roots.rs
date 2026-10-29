//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Compare one generated invocation interface across original macro and inline definitions.
//! Native consumers use the admitted Linux/macOS runtime ABI; compiler refusal and
//! physical ownership are covered independently of a running PostgreSQL backend.

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
/// Compile generated Rust consumers and paired type rejections with bounded tool execution.
#[path = "support/rust_oracle.rs"]
#[allow(dead_code)]
mod rust_oracle;

use pgrx_c_macros::{
    AnalysisSession, AnalysisStatus, EmissionStatus, MacroScanner, SkipReasonCode,
    emit_batch_with_bindings, emit_support_artifact_with_bindings, inspect,
    postgres_inline_function_names,
};
use std::path::PathBuf;

/// Preserve the production OID classification boundary in the shared fixture collector.
fn is_builtin_oid(name: &str) -> bool {
    name.ends_with("OID") && name != "HEAP_HASOID"
        || name.ends_with("RelationId")
        || name == "TemplateDbOid"
}

/// The same public names and Rust expressions are used in both independently inspected profiles.
const NAMES: &[&str] = &[
    "ROOT_INT",
    "ROOT_PRED",
    "ROOT_WORD",
    "ROOT_POINTER",
    "ROOT_CONST_POINTER",
    "ROOT_ENUM",
    "ROOT_VOID",
    "ROOT_REPEAT",
    "ROOT_SIZE",
    "ROOT_LAZY",
    "ROOT_CAPTURE",
    "__pgrx_inline_parameter_0",
    "ROOT_COLLISION",
    "ROOT_ZERO",
];

/// Compare values, native result types, argument effects and guard counts against actual C.
#[test]
fn one_invocation_interface_preserves_original_macro_and_inline_semantics() {
    let scanner = MacroScanner::new().expect("libclang must be available");
    for native in [false, true] {
        compare(&scanner, native);
    }
}

/// Exercise one exact native profile without changing its real preprocessor environment.
fn compare(scanner: &MacroScanner, native: bool) {
    let header = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/inline_roots.h");
    let mut arguments = oracle::native_arguments();
    arguments.extend(["-std=c17".into(), "-Werror=incompatible-pointer-types".into()]);
    if native {
        arguments.push("-DROOT_NATIVE".into());
    }
    let frontend =
        inspect(scanner, &header, &arguments, None).expect("inspect original definitions");
    let active = serde_json::to_value(&frontend.environment().active).unwrap();
    let selected = postgres_inline_function_names(&frontend, header.parent().unwrap()).unwrap();
    assert!(!selected.iter().any(|name| name == "ROOT_COLLISION"));
    if native {
        assert!(selected.iter().any(|name| name == "ROOT_INT"));
        assert!(!selected.iter().any(|name| name == "ROOT_UNDEFINED"));
    } else {
        assert!(selected.is_empty());
    }
    let rejects = ["ROOT_VARIADIC", "ROOT_UNPROTOTYPED", "ROOT_UNDEFINED", "ROOT_ABI"];
    let inlines = NAMES
        .iter()
        .copied()
        .chain(native.then_some(rejects).into_iter().flatten())
        .chain(native.then_some(["ROOT_HIDDEN", "ROOT_PARTIAL"]).into_iter().flatten())
        .collect::<Vec<_>>();
    let session = AnalysisSession::prepare_with_inline_functions(
        scanner,
        &frontend,
        &[] as &[String],
        &[] as &[String],
        &inlines,
    )
    .expect("prepare compiler-owned roots");
    assert_eq!(serde_json::to_value(&frontend.environment().active).unwrap(), active);
    if native {
        for (name, expected) in rejects.into_iter().zip([
            SkipReasonCode::Variadic,
            SkipReasonCode::Call,
            SkipReasonCode::Call,
            SkipReasonCode::Call,
        ]) {
            let AnalysisStatus::Skipped { reason } = session.analyze(name).status else {
                panic!("unsupported inline root {name} was admitted");
            };
            assert_eq!(reason.code, expected, "{name}: {reason:?}");
        }
        let analysis = session.analyze("ROOT_CAPTURE");
        assert_eq!(analysis.parameters[0].name, "ROOT_CAPTURE");
        assert_eq!(analysis.parameters[0].uses.len(), 1);
        // ROOT_HIDDEN's ROOT_SHADOW was a macro that is now an enum constant, so its
        // unexpanded tokens would change meaning: only the native call remains.
        // A body that can fall off its end without a value is not translated either.
        for name in ["ROOT_HIDDEN", "ROOT_PARTIAL"] {
            let analysis = session.analyze(name);
            assert!(matches!(analysis.status, AnalysisStatus::Candidate), "{analysis:?}");
            assert!(analysis.expression.unwrap().syntax.statement_body.is_none(), "{name}");
        }
        let analysis = session.analyze("ROOT_INT");
        assert!(analysis.expression.unwrap().syntax.statement_body.is_some());
    }
    let bindings = bindgen::Builder::default()
        .rust_target(bindgen::RustTarget::stable(85, 0).unwrap())
        .rust_edition(bindgen::RustEdition::Edition2024)
        .header(header.to_str().unwrap())
        .clang_args(&frontend.profile().arguments)
        .allowlist_type("Root.*")
        .rustified_enum("RootEnum")
        .layout_tests(false)
        .generate_comments(false)
        .formatter(bindgen::Formatter::None)
        .generate()
        .expect("generate actual enum and typedef storage")
        .to_string();
    let mut catalog = binding_symbols::collect_bindings(
        &syn::parse_file(&bindings).unwrap(),
        session.integer_constants(),
        frontend.declarations(),
        &frontend.profile().target,
    );
    catalog.ffi_boundary = Some(vec!["ffi".into(), "pg_guard_ffi_boundary".into()]);
    let artifact = emit_support_artifact_with_bindings(&session, NAMES, &catalog)
        .expect("generate original native call transport");
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
            panic!("root {} must emit: {emission:?}", emission.analysis.name);
        };
        if native && emission.analysis.name != "ROOT_COLLISION" {
            // Definitions the analyzer accepts are translated; a parameter named like its
            // own function stays a native call.
            let translated = emission.analysis.name != "ROOT_CAPTURE";
            assert_eq!(
                definition.contains("Translates the original function definition"),
                translated,
                "{}",
                emission.analysis.name
            );
            assert_eq!(
                definition.contains("Typed call adapter for C inline function"),
                !translated,
                "{}",
                emission.analysis.name
            );
            assert!(definition.contains("static inline"));
            assert!(!definition.contains(&format!("#define {}", emission.analysis.name)));
        }
        rust.push_str(&definition);
    }
    if native {
        let prelude = include_str!("fixtures/inline_roots.rs").split_once("fn main()").unwrap().0;
        let negative = format!(
            "{rust}\n{prelude}\nfn main() {{ let value=7_i32; unsafe {{ ROOT_POINTER!(&raw const value).get(); }} }}\n"
        );
        let error = rust_oracle::reject_rust(&negative);
        assert!(error.contains("AddConst"), "{error}");
        let arguments = frontend.profile().arguments.iter().map(String::as_str).collect::<Vec<_>>();
        let error = rust_oracle::reject_c_invocation(
            &frontend.profile().compiler.executable,
            &header,
            "int main(void) { const int value=7; (void) ROOT_POINTER(&value); }",
            &arguments,
        );
        assert!(error.contains("discards qualifiers"), "{error}");
        // A translated pointer write, like a native call, still needs unsafe; a translated
        // pure function, like the equivalent C macro, does not.
        let error = rust_oracle::reject_rust(&format!(
            "{rust}\n{prelude}\nfn main() {{ let mut value = 0_i32; ROOT_VOID!(&raw mut value, 7_i32).get(); }}\n"
        ));
        assert!(error.contains("unsafe"), "{error}");
    }
    // Only ROOT_CAPTURE remains a native call through the guard.
    rust.push_str(&format!("const EXPECTED_GUARDS: usize = {};\n", if native { 1 } else { 0 }));
    rust.push_str(include_str!("fixtures/inline_roots.rs"));
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
        include_str!("fixtures/inline_roots.c"),
        &arguments,
        true,
    );
    assert_eq!(generated, original);
    session.verify_inputs().expect("original profile stayed unchanged");
}
