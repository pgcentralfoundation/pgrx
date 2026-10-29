//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Compare immutable C strings, dependency stringification and invocation diagnostics with C.

/// Collect fresh bindgen facts using the production binding reconciler.
#[path = "../../pgrx-bindgen/src/build/binding_symbols.rs"]
mod binding_symbols;
/// Select installed PostgreSQL header oracles from configured metadata.
#[path = "support/postgres.rs"]
mod installed;
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
    // Exported macro bodies resolve standard macro names where they are invoked.
    // This uncalled caller compiles with the consumer only if the emitted const
    // checks for operand spelling and the invocation line ignore its `assert!`.
    let consulted = "#[allow(unused_macros)] macro_rules! assert { ($($tokens:tt)*) => { ::core::compile_error!(\"caller assert! was consulted\") } }";
    let accepted = format!(
        "mod accepted {{ {consulted}\npub fn invoke() -> i32 {{\n// SAFETY: The native recorder only borrows static zero-terminated strings.\nunsafe {{ STRING_DIAGNOSTIC!(identifier); STRING_LINE!().get() }} }} }}"
    );
    let generated = rust_oracle::run_rust_linked(
        &format!("{rust}\n{}\n{accepted}", include_str!("fixtures/string_oracle.rs")),
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
    let ignored = "#[allow(unused_macros)] macro_rules! assert { ($($tokens:tt)*) => { () } }";
    let shadowed = rust_oracle::reject_rust(&format!(
        "{rust}\nmod caller {{ {ignored}\npub fn refused() {{ unsafe {{ STRING_DIAGNOSTIC!(café); }} }} }}\nfn main() {{}}\n"
    ));
    assert!(
        shadowed.contains("C diagnostic operand spelling requires ASCII tokens"),
        "a caller's own assert! must not accept non-ASCII diagnostic spelling: {shadowed}"
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

/// Verify each configured installation's real varlena macros match the original C, including
/// the native failing branch under assertion-enabled headers.
///
/// `-DUSE_ASSERT_CHECKING` enables assertions in installations configured without them, because
/// configure leaves the setting commented out in `pg_config.h`. Meson writes an active
/// `#undef USE_ASSERT_CHECKING` instead, which overrides the command line, so meson-built
/// installations such as PostgreSQL 17 for Windows compare the unasserted branch. The expected
/// branch comes from the installed header, so a profile that lost the definition fails here.
#[test]
fn postgres_varlena_assertions_match_original_headers() {
    let installations = installed::configured();
    if installations.is_empty() {
        return;
    }
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let names = ["VARTAG_SIZE", "VARSIZE_EXTERNAL", "VARSIZE_ANY", "VARSIZE_ANY_EXHDR"];
    for postgres in installations {
        let major = postgres.pg_config().major_version().unwrap();
        let arguments = vec!["-DUSE_ASSERT_CHECKING".into(), "-O2".into()];
        // ELF and COFF link-time garbage collection needs a section for each
        // unused backend routine; these flags do not change preprocessing.
        #[cfg(any(target_os = "linux", target_os = "windows"))]
        let arguments = {
            let mut arguments = arguments;
            arguments.extend(["-ffunction-sections".into(), "-fdata-sections".into()]);
            arguments
        };
        let frontend = installed::inspect(&scanner, &postgres, &arguments);
        let asserted = assertions_survive_installed_config(&postgres);
        assert_eq!(
            frontend.environment().active.contains_key("USE_ASSERT_CHECKING"),
            asserted,
            "PG{major} profile must enable assertions unless pg_config.h undefines them"
        );
        let defined =
            frontend.inventory().macros.iter().any(|definition| definition.name == "VARTAG_SIZE");
        // PG19 replaced these varlena macros with static inline functions.
        if major >= 19 {
            assert!(!defined, "PG{major} VARTAG_SIZE is expected to be a static inline function");
            continue;
        }
        assert!(defined, "PG{major} must define VARTAG_SIZE as a macro");
        if !installed::supports_rust_comparisons(&scanner, &frontend) {
            continue;
        }
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
    unsigned long invalid = VARTAG_SIZE(assertion_argument(0));
    printf("unasserted\t%lu\t%u\n", invalid, evaluations);
    return 0;
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
    // Valid tags produce sizes. With assertions, the final invalid tag exits in
    // native C, so no Rust destructor or unwinding state is required across
    // that failure.
    unsafe {{
        for tag in [1_i32, 2, 3, 18] {{
            assertion_site(core::ptr::null(), 0);
            let size = VARTAG_SIZE!(assertion_argument(tag));
            println!("size\t{{}}\t{{}}", size.get(), assertion_evaluations());
        }}
        assertion_site(concat!(file!(), "\0").as_ptr().cast(), (line!() + 1) as i32);
        let invalid = VARTAG_SIZE!(assertion_argument(0_i32));
        println!("unasserted\t{{}}\t{{}}", invalid.get(), assertion_evaluations());
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
            original.lines().last().is_some_and(|line| line.starts_with("failure\t")),
            asserted,
            "PG{major} invalid tag must fail exactly when assertions are active"
        );
        assert_eq!(
            generated, original,
            "PG{major} native assertion must preserve condition, source site and operand counts"
        );
    }
}

/// Read the installation's `pg_config.h`, independently of the profile under test, and report
/// whether a command-line `USE_ASSERT_CHECKING` definition survives it.
fn assertions_survive_installed_config(postgres: &PostgresConfig) -> bool {
    let path = postgres.pg_config().includedir_server().unwrap().join("pg_config.h");
    let header = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
    assertion_definition_survives(&header)
}

/// Report whether a command-line `USE_ASSERT_CHECKING` definition survives `pg_config.h`, whose
/// last active `#define` or `#undef` of the name decides.
fn assertion_definition_survives(pg_config_h: &str) -> bool {
    let mut asserted = true;
    for line in pg_config_h.lines() {
        let Some(directive) = line.trim_start().strip_prefix('#') else { continue };
        let mut words = directive.split_ascii_whitespace();
        match (words.next(), words.next()) {
            (Some("define"), Some("USE_ASSERT_CHECKING")) => asserted = true,
            (Some("undef"), Some("USE_ASSERT_CHECKING")) => asserted = false,
            _ => {}
        }
    }
    asserted
}

/// Configure comments out the setting unless assertions are enabled; meson writes an active
/// `#undef` when they are off, as PostgreSQL 17 for Windows does.
#[test]
fn assertion_branch_follows_the_installed_pg_config_spelling() {
    assert!(assertion_definition_survives("/* #undef USE_ASSERT_CHECKING */\n"));
    assert!(assertion_definition_survives("#define USE_ASSERT_CHECKING 1\r\n"));
    assert!(!assertion_definition_survives("#undef USE_ASSERT_CHECKING\r\n"));
    assert!(!assertion_definition_survives("  #  undef USE_ASSERT_CHECKING\n"));
    assert!(assertion_definition_survives("#undef USE_ASSERT_CHECKING_EXTRA\n"));
}
