//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Compare immutable C strings, dependency stringification and invocation diagnostics with C.

/// Collect fresh bindgen facts using the production binding reconciler.
#[path = "../../pgrx-bindgen/src/build/binding_symbols.rs"]
mod binding_symbols;
/// Compile and execute original C macro invocations with bounded resources.
#[path = "support/oracle.rs"]
mod oracle;
/// Compile generated Rust and link the original native diagnostic functions.
#[path = "support/rust_oracle.rs"]
mod rust_oracle;

use pgrx_c_macros::{
    AnalysisSession, BindingCatalog, EmissionStatus, FrontendOutput, MacroScanner, PostgresConfig,
    SkipReasonCode, emit_batch_with_bindings, emit_support_artifact_with_bindings,
    emit_with_bindings, inspect,
};
use std::path::PathBuf;
use std::sync::Mutex;

/// Serialize the safe libclang runtime owner within this integration test process.
static SCANNER_LOCK: Mutex<()> = Mutex::new(());
/// Admitted macro roots whose complete generated semantics are compared with original C.
const SUPPORTED: &[&str] = &[
    "STRING_CLOSED",
    "STRING_ESCAPES",
    "STRING_EMBEDDED",
    "STRING_SIZE",
    "STRING_POINTER",
    "STRING_DEREF_SIZE",
    "STRING_CHAR",
    "STRING_SELECT",
    "STRING_REPEAT",
    "STRING_CHECK",
    "STRING_LOCATION",
    "STRING_LOCATION_NESTED",
    "STRING_LINE",
    "STRING_FILE_SIZE",
    "STRING_WRITE",
    "STRING_DIAGNOSTIC",
    "STRING_DIAGNOSTIC_FORWARD",
    "STRING_DIAGNOSTIC_NESTED",
    "STRING_DIAGNOSTIC_MULTI",
    "STRING_OPERAND_ASSERT",
    "STRING_TEXT_FORMAL",
];
/// Unsupported preprocessing/encoding contracts that must retain explicit refusals.
const REJECTED: &[(&str, SkipReasonCode)] = &[
    ("STRINGIFY_OPERAND", SkipReasonCode::Stringification),
    ("STRING_ASSERT", SkipReasonCode::Stringification),
    ("STRING_OPEN", SkipReasonCode::Stringification),
    ("STRING_CONTEXT", SkipReasonCode::Stringification),
    ("STRING_OPEN_SIZE", SkipReasonCode::Stringification),
    ("STRING_OPEN_ADDRESS", SkipReasonCode::Stringification),
    ("STRING_OPEN_CONCAT", SkipReasonCode::Stringification),
    ("STRING_OPEN_RESULT", SkipReasonCode::Stringification),
    ("STRING_FUNCTION", SkipReasonCode::DynamicBuiltin),
    ("STRING_WIDE", SkipReasonCode::UnsupportedLiteral),
    ("STRING_UTF8", SkipReasonCode::UnsupportedLiteral),
    ("STRING_HIGH", SkipReasonCode::UnsupportedLiteral),
    ("STRING_NONASCII", SkipReasonCode::UnsupportedLiteral),
    ("STRING_UNIVERSAL", SkipReasonCode::UnsupportedLiteral),
];

/// Supply the classification context required by the shared production collector.
fn is_builtin_oid(name: &str) -> bool {
    name.ends_with("OID") && name != "HEAP_HASOID"
        || name.ends_with("RelationId")
        || name == "TemplateDbOid"
}

/// Locate the original guarded C fixture rather than reconstructing its macro bodies.
fn header() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/string_oracle.h")
}

/// Inspect one ordinary C17 profile with the native platform SDK when necessary.
fn native_arguments() -> Vec<String> {
    let mut arguments = vec!["-std=c17".into(), "-ffp-contract=off".into()];
    #[cfg(target_os = "macos")]
    {
        let sdk = rust_oracle::run_tool(
            std::process::Command::new("xcrun").arg("--show-sdk-path"),
            "sdk",
        );
        arguments.extend(["-isysroot".into(), sdk.trim().into()]);
    }
    arguments
}

/// Generate exact native function bindings and reconcile them with the inspected declarations.
fn original_bindings(
    frontend: &FrontendOutput,
    session: &AnalysisSession<'_>,
) -> (String, BindingCatalog) {
    let source = bindgen::Builder::default()
        .rust_target(bindgen::RustTarget::stable(85, 0).expect("Rust 2024 target"))
        .rust_edition(bindgen::RustEdition::Edition2024)
        .header(header().to_str().unwrap())
        .clang_args(&frontend.profile().arguments)
        .allowlist_function("string_.*")
        .allowlist_var("string_global_values")
        .layout_tests(false)
        .generate_comments(false)
        .formatter(bindgen::Formatter::None)
        .generate()
        .expect("generate actual string recorder bindings")
        .to_string();
    let syntax = syn::parse_file(&source).expect("parse fresh bindgen output");
    let catalog = binding_symbols::collect_bindings(
        &syntax,
        session.integer_constants(),
        frontend.declarations(),
        &frontend.profile().target,
    );
    (source, catalog)
}

/// Assemble generated macros and only the native adapters demanded by admitted roots.
fn emitted_source(
    session: &AnalysisSession<'_>,
    source: &str,
    catalog: &BindingCatalog,
    names: &[&str],
) -> (String, String) {
    let support = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../pgrx-pg-sys/src/c_macros/support.rs")
        .canonicalize()
        .unwrap();
    let mut rust = format!(
        "#![allow(non_snake_case, non_camel_case_types, dead_code, unused_parens)]\n\
         #[path = {support:?}]\npub mod __pgrx_c_macros;\n{source}\n"
    );
    let artifact = emit_support_artifact_with_bindings(session, names, catalog).unwrap();
    rust.push_str(&artifact.rust);
    for emission in emit_batch_with_bindings(session, names, catalog).unwrap() {
        let EmissionStatus::Emitted { rust: definition, .. } = emission.status else {
            panic!("string macro must emit: {emission:?}");
        };
        rust.push_str(&definition);
    }
    (rust, artifact.c_source)
}

/// Check literal bytes, array size/decay/address, C lazy evaluation and source diagnostics.
#[test]
fn generated_strings_and_assertion_diagnostics_match_original_c() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let frontend = inspect(&scanner, &header(), &native_arguments(), None).unwrap();
    let names =
        SUPPORTED.iter().copied().chain(REJECTED.iter().map(|&(name, _)| name)).collect::<Vec<_>>();
    let session = AnalysisSession::prepare(&scanner, &frontend, &names).unwrap();
    let (bindings, catalog) = original_bindings(&frontend, &session);
    for &(name, expected) in REJECTED {
        let emission = emit_with_bindings(&session, name, &catalog);
        let EmissionStatus::Skipped { reason } = emission.status else {
            panic!("unmodeled string contract must skip: {emission:?}");
        };
        assert_eq!(reason.code, expected, "{name}: {reason:?}");
        assert!(!reason.message.is_empty(), "{name} must explain its refusal");
    }
    let (rust, adapters) = emitted_source(&session, &bindings, &catalog, SUPPORTED);
    let profile = frontend.profile();
    let arguments = profile.arguments.iter().map(String::as_str).collect::<Vec<_>>();
    let generated = rust_oracle::run_rust_linked(
        &format!("{rust}\n{}", include_str!("fixtures/string_oracle.rs")),
        &profile.compiler.executable,
        &header(),
        &format!("{}\n{adapters}", include_str!("fixtures/string_oracle.c")),
        &arguments,
    );
    let mut original_arguments = arguments.clone();
    original_arguments.push("-DPGRX_STRING_ORACLE_MAIN");
    let original = oracle::run_c(
        &profile.compiler.executable,
        &header(),
        include_str!("fixtures/string_oracle.c"),
        &original_arguments,
        true,
    );
    assert_eq!(original.lines().count(), 27, "string oracle corpus completeness");
    assert_eq!(generated, original, "generated strings and assertions must preserve C behavior");

    let mutation = rust_oracle::reject_rust(&format!(
        "{rust}\nfn main() {{ unsafe {{ let _ = STRING_WRITE!(); }} }}\n"
    ));
    assert!(mutation.contains("ReadOnly"), "C literal mutation must reject: {mutation}");
    let missing_unsafe = rust_oracle::reject_rust(&format!(
        "{rust}\nfn main() {{ let _ = STRING_CHECK!(0_i32); }}\n"
    ));
    assert!(missing_unsafe.contains("unsafe"), "native assertion calls require unsafe");
    let non_ascii = rust_oracle::reject_rust(&format!(
        "{rust}\nfn main() {{ unsafe {{ STRING_DIAGNOSTIC!(café); }} }}\n"
    ));
    assert!(
        non_ascii.contains("C diagnostic operand spelling requires ASCII tokens"),
        "non-ASCII diagnostic token spelling must reject: {non_ascii}"
    );
}

/// Prove explicit object roots retain bare C invocation and independent constant restoration.
#[test]
fn selected_object_expressions_preserve_original_signatures_and_native_places() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let frontend = inspect(&scanner, &header(), &native_arguments(), None).unwrap();
    let objects = ["STRING_OBJECT_NUMBER", "STRING_OBJECT_ELEMENT", "STRING_OBJECT_LITERAL"];
    let functions = ["STRING_DEREF_SIZE", "STRING_LOCATION_NESTED"];
    let session =
        AnalysisSession::prepare_with_objects(&scanner, &frontend, &functions, &objects).unwrap();
    for name in objects {
        let pgrx_c_macros::ExpansionResult::Expanded { expansion } =
            &session.expansions().results[name]
        else {
            panic!("selected object expression {name} must expand");
        };
        assert_eq!(expansion.definition.kind, pgrx_c_macros::MacroKind::ObjectLike);
        assert!(expansion.parameters.is_empty() && expansion.symbolic_parameters.is_empty());
        assert_eq!(
            expansion.definition.provenance,
            frontend.environment().active[name].definition.provenance
        );
        assert!(
            expansion.constant_fallbacks.is_empty(),
            "object symbol restoration must preserve the original token stream: {expansion:?}"
        );
    }
    assert!(session.integer_constants().contains_key("STRING_OBJECT_OFFSET"));
    let default = AnalysisSession::prepare(&scanner, &frontend, &objects).unwrap();
    assert!(
        matches!(default.analyze(objects[0]).status, pgrx_c_macros::AnalysisStatus::Skipped { ref reason } if reason.code == SkipReasonCode::NotFunctionLike)
    );
    let selected = AnalysisSession::prepare_objects(
        &scanner,
        &frontend,
        &["STRING_OBJECT_COUNTER", "STRING_LOCATION_NESTED"],
    )
    .unwrap();
    for (name, expected) in [
        ("STRING_OBJECT_COUNTER", SkipReasonCode::DynamicBuiltin),
        ("STRING_LOCATION_NESTED", SkipReasonCode::NotObjectLike),
    ] {
        assert!(
            matches!(selected.analyze(name).status, pgrx_c_macros::AnalysisStatus::Skipped { ref reason } if reason.code == expected)
        );
    }
    let (bindings, catalog) = original_bindings(&frontend, &session);
    let names = functions.iter().chain(&objects).copied().collect::<Vec<_>>();
    let (rust, adapters) = emitted_source(&session, &bindings, &catalog, &names);
    let c = format!(
        r#"{}
int main(void) {{
    printf("object\t%d\t%d\t%lu\t%s\n", STRING_OBJECT_NUMBER, *STRING_OBJECT_ELEMENT,
        (unsigned long)STRING_DEREF_SIZE(STRING_OBJECT_ELEMENT), STRING_OBJECT_LITERAL);
    int expected_line = __LINE__ + 1;
    STRING_LOCATION_NESTED();
    string_diagnostics(__FILE__, expected_line);
    return 0;
}}
"#,
        include_str!("fixtures/string_oracle.c")
    );
    let rust = format!(
        r#"{rust}
fn main() {{
    // SAFETY: The selected pointer designates the second initialized native
    // global element, and literal/filename arrays have static terminated storage.
    unsafe {{
        let pointer = STRING_OBJECT_ELEMENT!().get();
        let text = std::ffi::CStr::from_ptr(STRING_OBJECT_LITERAL!().get()).to_str().unwrap();
        println!("object\t{{}}\t{{}}\t{{}}\t{{}}", STRING_OBJECT_NUMBER!().get(), *pointer,
            STRING_DEREF_SIZE!(@__pgrx_c_expression; STRING_OBJECT_ELEMENT!()).get(), text);
        let expected_line = line!() + 1;
        STRING_LOCATION_NESTED!();
        string_diagnostics(concat!(file!(), "\0").as_ptr().cast(), expected_line as i32);
    }}
}}
"#
    );
    let profile = frontend.profile();
    let arguments = profile.arguments.iter().map(String::as_str).collect::<Vec<_>>();
    let original = oracle::run_c(&profile.compiler.executable, &header(), &c, &arguments, true);
    let generated = rust_oracle::run_rust_linked(
        &rust,
        &profile.compiler.executable,
        &header(),
        &format!("{}\n{adapters}", include_str!("fixtures/string_oracle.c")),
        &arguments,
    );
    assert_eq!(
        generated, original,
        "object roots must preserve original C values, places and diagnostic restoration"
    );
}

/// Verify real assertion-enabled PG15–18 varlena macros retain the native failing branch.
#[test]
#[ignore = "requires configured native PostgreSQL 15 through 18 installations"]
fn postgres_varlena_assertions_match_original_headers() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let names = ["VARTAG_SIZE", "VARSIZE_EXTERNAL", "VARSIZE_ANY", "VARSIZE_ANY_EXHDR"];
    for major in 15..=18 {
        let postgres = PostgresConfig::resolve(&format!("pg{major}")).unwrap();
        let frontend = postgres
            .inspect(&scanner, None, &["-DUSE_ASSERT_CHECKING".into(), "-O2".into()], None)
            .unwrap();
        let session = AnalysisSession::prepare(&scanner, &frontend, &names).unwrap();
        let header = &frontend.profile().header;
        let bindings = bindgen::Builder::default()
            .rust_target(bindgen::RustTarget::stable(85, 0).expect("Rust 2024 target"))
            .rust_edition(bindgen::RustEdition::Edition2024)
            .header(header.to_str().unwrap())
            .clang_args(&frontend.profile().arguments)
            .allowlist_function("ExceptionalCondition")
            .allowlist_type("varatt_.*|varattrib_.*|vartag_external")
            // Match production's public enum paths and non-Copy union storage.
            // Bindgen's default union wrapper is intentionally not an admitted
            // field capability and cannot stand in for these actual bindings.
            .default_enum_style(bindgen::EnumVariation::ModuleConsts)
            .size_t_is_usize(true)
            .use_core()
            .disable_nested_struct_naming()
            .default_non_copy_union_style(bindgen::NonCopyUnionStyle::ManuallyDrop)
            .layout_tests(false)
            .generate_comments(false)
            .formatter(bindgen::Formatter::None)
            .generate()
            .expect("generate fresh native varlena bindings")
            .to_string();
        let syntax = syn::parse_file(&bindings).unwrap();
        let catalog = binding_symbols::collect_bindings(
            &syntax,
            session.integer_constants(),
            frontend.declarations(),
            &frontend.profile().target,
        );
        assert!(
            catalog.records.get("varattrib_4b").is_some_and(|record| {
                record.fields.contains_key("va_4byte")
                    && record.fields.contains_key("va_compressed")
            }),
            "PG{major} fresh bindings must retain the original varlena union fields"
        );
        let (rust, adapters) = emitted_source(&session, &bindings, &catalog, &names);
        let condition = if major == 15 {
            "void ExceptionalCondition(const char *condition, const char *error_type, const char *file, int line) { printf(\"failure\\t%s\\t%s\\t%d\\t%d\\t%u\\n\", condition, error_type, strcmp(file, expected_file) == 0, line == expected_line, evaluations); exit(0); }"
        } else {
            "void ExceptionalCondition(const char *condition, const char *file, int line) { printf(\"failure\\t%s\\t%s\\t%d\\t%d\\t%u\\n\", condition, \"no error-type parameter\", strcmp(file, expected_file) == 0, line == expected_line, evaluations); exit(0); }"
        };
        let native = format!(
            r#"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#undef printf
static const char *expected_file;
static int expected_line;
static unsigned evaluations;
void assertion_site(const char *file, int line) {{ expected_file = file; expected_line = line; evaluations = 0; }}
int assertion_argument(int value) {{ ++evaluations; return value; }}
unsigned assertion_evaluations(void) {{ return evaluations; }}
{condition}
"#
        );
        let c = format!(
            r#"{native}
int main(void) {{
    const int tags[] = {{ 1, 2, 3, 18 }};
    for (unsigned index = 0; index < 4; ++index) {{
        evaluations = 0;
        unsigned long size = VARTAG_SIZE(assertion_argument(tags[index]));
        printf("size\t%lu\t%u\n", size, evaluations);
    }}
    assertion_site(__FILE__, __LINE__ + 1);
    (void)VARTAG_SIZE(assertion_argument(0));
    return 1;
}}
"#
        );
        let rust = format!(
            r#"{rust}
unsafe extern "C" {{
    /// Borrow the static expected filename until the immediately following assertion call.
    fn assertion_site(file: *const core::ffi::c_char, line: core::ffi::c_int);
    /// Increment a process-local test counter and return the argument without backend state.
    fn assertion_argument(value: core::ffi::c_int) -> core::ffi::c_int;
    /// Read only the native fixture counter updated by the preceding argument calls.
    fn assertion_evaluations() -> core::ffi::c_uint;
}}
fn main() {{
    // SAFETY: The fixture functions have these exact fixed C prototypes and
    // only borrow static filename storage or increment a process-local counter.
    // Valid tags produce sizes; the final invalid tag exits in native C, so no
    // Rust destructor or unwinding state is required across that failure.
    unsafe {{
        for tag in [1_i32, 2, 3, 18] {{
            assertion_site(core::ptr::null(), 0);
            let size = VARTAG_SIZE!(assertion_argument(tag));
            println!("size\t{{}}\t{{}}", size.get(), assertion_evaluations());
        }}
        assertion_site(concat!(file!(), "\0").as_ptr().cast(), (line!() + 1) as i32);
        let _ = VARTAG_SIZE!(assertion_argument(0_i32));
    }}
}}
"#
        );
        let profile = frontend.profile();
        let arguments = profile.arguments.iter().map(String::as_str).collect::<Vec<_>>();
        let original = oracle::run_c(&profile.compiler.executable, header, &c, &arguments, true);
        let generated = rust_oracle::run_rust_linked(
            &rust,
            &profile.compiler.executable,
            header,
            &format!("{native}\n{adapters}"),
            &arguments,
        );
        assert_eq!(original.lines().count(), 5, "PG{major} assertion corpus completeness");
        assert_eq!(
            generated, original,
            "PG{major} native assertion must preserve condition, source site and operand counts"
        );
    }
}
