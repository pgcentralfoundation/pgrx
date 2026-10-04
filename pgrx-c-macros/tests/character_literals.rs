//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "support/rust_oracle.rs"]
mod rust_oracle;

use pgrx_c_macros::{
    AnalysisSession, ConstCapability, EmissionStatus, FrontendOutput, MacroScanner, PostgresConfig,
    SkipReasonCode, emit, inspect,
};
use std::fmt::Write;
use std::path::PathBuf;
use std::sync::Mutex;

static SCANNER_LOCK: Mutex<()> = Mutex::new(());

const CONSTANTS: &[&str] = &[
    "CHARACTER_DIGIT",
    "CHARACTER_UPPER",
    "CHARACTER_LOWER",
    "CHARACTER_SPACE",
    "CHARACTER_TILDE",
    "CHARACTER_QUOTE",
    "CHARACTER_DOUBLE_QUOTE",
    "CHARACTER_QUESTION",
    "CHARACTER_BACKSLASH",
    "CHARACTER_ALERT",
    "CHARACTER_BACKSPACE",
    "CHARACTER_FORMFEED",
    "CHARACTER_NEWLINE",
    "CHARACTER_RETURN",
    "CHARACTER_TAB",
    "CHARACTER_VERTICAL_TAB",
    "CHARACTER_OCTAL_ZERO",
    "CHARACTER_OCTAL_ONE",
    "CHARACTER_OCTAL_TWO_DIGITS",
    "CHARACTER_OCTAL_MAX",
    "CHARACTER_HEX_ZERO",
    "CHARACTER_HEX_ONE_DIGIT",
    "CHARACTER_HEX_MAX",
    "CHARACTER_HEX_LEADING_ZEROES",
];
const OPERATORS: &[&str] = &["CHARACTER_OFFSET", "CHARACTER_REPEAT", "CHARACTER_COMPARE"];
const REJECTED: &[(&str, &str)] = &[
    ("CHARACTER_WIDE", "prefixed"),
    ("CHARACTER_UTF16", "prefixed"),
    ("CHARACTER_UTF32", "prefixed"),
    ("CHARACTER_MULTIPLE", "multi-character"),
    ("CHARACTER_NONASCII", "non-ASCII"),
    ("CHARACTER_UNIVERSAL", "encoding"),
    ("CHARACTER_UNKNOWN_ESCAPE", "unknown"),
    ("CHARACTER_OCTAL_HIGH", "range"),
    ("CHARACTER_HEX_HIGH", "range"),
    ("CHARACTER_NOT_BASIC", "basic execution"),
];

const C_RECORDING: &str = r#"
#include <limits.h>
#include <stdio.h>
#undef printf
#define KIND(value) _Generic((value), int: "CInt", unsigned int: "CUnsignedInt", \
    long: "CLong", unsigned long: "CUnsignedLong", \
    long long: "CLongLong", unsigned long long: "CUnsignedLongLong")
static unsigned evaluations;
static int argument(void) { ++evaluations; return 7; }
#define RECORD(name, expression) do { \
    __typeof__(expression) value = (expression); \
    const unsigned __int128 bits = (unsigned __int128)value; \
    printf("%s\t%s\t%u\t%016llx%016llx\t%u\n", name, KIND(value), \
        (unsigned)(sizeof(value) * CHAR_BIT), (unsigned long long)(bits >> 64), \
        (unsigned long long)bits, evaluations); \
} while (0)
int main(void) {
"#;

const RUST_RECORDING: &str = r#"
use __pgrx_c_macros::{CInteger, CLong, CUnsignedLong, CValue};
use std::cell::Cell;
fn record<T: __pgrx_c_macros::IntoCValue>(name: &str, value: T, evaluations: u32) {
    let value = value.into_c_value();
    let kind = std::any::type_name::<T::Kind>().rsplit("::").next().unwrap();
    println!("{name}\t{kind}\t{}\t{:032x}\t{evaluations}", T::Kind::BITS, T::Kind::encode(value.get()));
}
fn main() {
    let evaluations = Cell::new(0_u32);
    let argument = || { evaluations.set(evaluations.get() + 1); 7_i32 };
"#;

fn generated_source(session: &AnalysisSession<'_>, names: &[&str]) -> String {
    let support = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../pgrx-pg-sys/src/c_macros/support.rs")
        .canonicalize()
        .unwrap();
    let mut source =
        format!("#[path = {:?}]\npub mod __pgrx_c_macros;\n", support.to_str().unwrap());
    let artifact = pgrx_c_macros::emit_support_artifact_with_bindings(
        session,
        names,
        &pgrx_c_macros::BindingCatalog::default(),
    )
    .unwrap();
    assert!(artifact.c_source.is_empty(), "character fixtures require no native adapters");
    source.push_str(&artifact.rust);
    for name in names {
        let emission = emit(session, name);
        let EmissionStatus::Emitted { rust, const_capability } = emission.status else {
            panic!("original C macro {name} must emit: {emission:?}");
        };
        assert_eq!(const_capability, ConstCapability::RuntimeOnly);
        source.push_str(&rust);
    }
    source.push_str(RUST_RECORDING);
    source
}

fn compare_original(frontend: &FrontendOutput, c: &str, rust: &str, records: usize) {
    let profile = frontend.profile();
    let original = oracle::run_c(
        &profile.compiler.executable,
        &profile.header,
        c,
        &profile.arguments.iter().map(String::as_str).collect::<Vec<_>>(),
        true,
    );
    let generated = rust_oracle::run_rust(rust);
    assert_eq!(original.lines().count(), records, "original C corpus completeness");
    assert_eq!(generated.lines().count(), records, "generated Rust corpus completeness");
    for (index, (c, rust)) in original.lines().zip(generated.lines()).enumerate() {
        assert_eq!(rust, c, "original C vs generated Rust character record {index}");
    }
}

#[cfg(target_os = "macos")]
fn native_include_arguments() -> Vec<String> {
    let sdk = rust_oracle::run_tool(
        std::process::Command::new("xcrun").arg("--show-sdk-path"),
        "Apple SDK lookup",
    );
    let sdk = sdk.trim();
    assert!(
        !sdk.is_empty() && std::path::Path::new(sdk).is_dir(),
        "Apple SDK lookup did not return an installed SDK directory"
    );
    vec!["-isysroot".to_owned(), sdk.to_owned()]
}

#[cfg(not(target_os = "macos"))]
fn native_include_arguments() -> Vec<String> {
    Vec::new()
}

#[test]
fn ordinary_character_literals_match_original_c_types_values_and_occurrences() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let header =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/character_literals.h");
    let mut arguments = vec!["-std=c11".into()];
    arguments.extend(native_include_arguments());
    let frontend = inspect(&scanner, &header, &arguments, None).unwrap();
    let names = CONSTANTS.iter().chain(OPERATORS).copied().collect::<Vec<_>>();
    let all_names =
        names.iter().copied().chain(REJECTED.iter().map(|(name, _)| *name)).collect::<Vec<_>>();
    let session = AnalysisSession::prepare(&scanner, &frontend, &all_names).unwrap();
    for (name, spelling) in [
        ("CHARACTER_DIGIT", "'0'"),
        ("CHARACTER_UPPER", "'A'"),
        ("CHARACTER_LOWER", "'z'"),
        ("CHARACTER_SPACE", "' '"),
        ("CHARACTER_TILDE", "'~'"),
        ("CHARACTER_QUOTE", r"'\''"),
        ("CHARACTER_DOUBLE_QUOTE", r#"'\"'"#),
        ("CHARACTER_QUESTION", "'?'"),
        ("CHARACTER_BACKSLASH", r"'\\'"),
        ("CHARACTER_ALERT", r"'\x07'"),
        ("CHARACTER_BACKSPACE", r"'\x08'"),
        ("CHARACTER_FORMFEED", r"'\x0c'"),
        ("CHARACTER_NEWLINE", r"'\n'"),
        ("CHARACTER_RETURN", r"'\r'"),
        ("CHARACTER_TAB", r"'\t'"),
        ("CHARACTER_VERTICAL_TAB", r"'\x0b'"),
        ("CHARACTER_OCTAL_ZERO", r"'\0'"),
        ("CHARACTER_OCTAL_ONE", r"'\x01'"),
        ("CHARACTER_OCTAL_TWO_DIGITS", r"'\x0a'"),
        ("CHARACTER_OCTAL_MAX", r"'\x7f'"),
        ("CHARACTER_HEX_ZERO", r"'\x00'"),
        ("CHARACTER_HEX_ONE_DIGIT", r"'\x07'"),
        ("CHARACTER_HEX_MAX", r"'\x7f'"),
        ("CHARACTER_HEX_LEADING_ZEROES", r"'\x7f'"),
    ] {
        let emission = emit(&session, name);
        let EmissionStatus::Emitted { rust, .. } = emission.status else {
            panic!("original C character macro {name} must emit: {emission:?}");
        };
        let body = rust.split_once("=> {").expect("generated macro rule").1;
        assert!(body.contains(&format!("::new({spelling} as i32)")), "{name}: {body}");
    }
    for &(name, message) in REJECTED {
        let emission = emit(&session, name);
        let EmissionStatus::Skipped { reason } = emission.status else {
            panic!("unsupported original literal {name} must skip");
        };
        assert_eq!(reason.code, SkipReasonCode::UnsupportedLiteral, "{name}: {reason:?}");
        assert!(reason.message.contains(message), "{name}: {}", reason.message);
        assert!(reason.spans.contains(emission.analysis.provenance.as_ref().unwrap()));
    }
    let mut c = String::from(C_RECORDING);
    let mut rust = generated_source(&session, &names);
    for value in 0..=127 {
        // Compiler assertions establish every accepted numeric escape's C value/type;
        // parser units independently cover the same complete bounded range.
        for spelling in [format!("'\\{value:03o}'"), format!("'\\x{value:02x}'")] {
            writeln!(c, "_Static_assert({spelling} == {value} && _Generic({spelling}, int: 1, default: 0), \"numeric character escape\");")
                .unwrap();
        }
    }
    for name in CONSTANTS {
        writeln!(c, "RECORD(\"{name}\", {name}(undefined_unused_binding));").unwrap();
        writeln!(rust, "record(\"{name}\", {name}!(undefined_unused_binding), evaluations.get());")
            .unwrap();
    }
    let samples = [-200_i32, -1, 0, 47, 48, 49, 65, 127, 255];
    for name in OPERATORS {
        for (index, value) in samples.into_iter().enumerate() {
            let label = format!("{name}_{index}");
            writeln!(c, "RECORD(\"{label}\", {name}({value}));").unwrap();
            writeln!(rust, "record(\"{label}\", {name}!({value}_i32), evaluations.get());")
                .unwrap();
        }
        for (label, c_value, rust_value) in [
            ("UNSIGNED", "0xFFFFFFFFU", "u32::MAX"),
            ("UCHAR", "(unsigned char)255", "255_u8"),
            ("LONG", "-1L", "CValue::<CLong>::new(-1)"),
            ("ULONG", "0UL", "CValue::<CUnsignedLong>::new(0)"),
        ] {
            writeln!(c, "RECORD(\"{name}_{label}\", {name}({c_value}));").unwrap();
            writeln!(rust, "record(\"{name}_{label}\", {name}!({rust_value}), evaluations.get());")
                .unwrap();
        }
        writeln!(c, "evaluations = 0; RECORD(\"{name}_EVAL\", {name}(argument()));").unwrap();
        writeln!(rust, "evaluations.set(0); let value = {name}!(argument()); record(\"{name}_EVAL\", value, evaluations.get());")
            .unwrap();
    }
    c.push_str("return 0; }\n");
    rust.push_str("}\n");
    compare_original(&frontend, &c, &rust, CONSTANTS.len() + OPERATORS.len() * (samples.len() + 5));
}

#[test]
#[ignore = "requires a configured native PostgreSQL 18 installation"]
fn postgres_sqlstate_macros_match_original_c_at_runtime() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let postgres = PostgresConfig::resolve("pg18").expect("native PG18 must be configured");
    let scanner = MacroScanner::new().expect("libclang must be available");
    let frontend = postgres.inspect(&scanner, None, &[], None).unwrap();
    let names = ["PGSIXBIT", "MAKE_SQLSTATE"];
    let session = AnalysisSession::prepare(&scanner, &frontend, &names).unwrap();
    let mut c = String::from(C_RECORDING);
    let mut rust = generated_source(&session, &names);
    let mut records = 0;
    for value in -256..=255 {
        writeln!(c, "RECORD(\"PGSIXBIT_{value}\", PGSIXBIT({value}));").unwrap();
        writeln!(rust, "record(\"PGSIXBIT_{value}\", PGSIXBIT!({value}_i32), 0);").unwrap();
        records += 1;
    }
    for (label, c_value, rust_value) in [
        ("UNSIGNED", "0xFFFFFFFFU", "u32::MAX"),
        ("UCHAR", "(unsigned char)255", "255_u8"),
        ("LONG", "-1L", "CValue::<CLong>::new(-1)"),
        ("ULONG", "0UL", "CValue::<CUnsignedLong>::new(0)"),
    ] {
        writeln!(c, "RECORD(\"PGSIXBIT_{label}\", PGSIXBIT({c_value}));").unwrap();
        writeln!(rust, "record(\"PGSIXBIT_{label}\", PGSIXBIT!({rust_value}), 0);").unwrap();
        records += 1;
    }
    for (index, code) in
        ["00000", "01000", "23505", "42P01", "XX000", "P0001", "ZZZZZ"].into_iter().enumerate()
    {
        let c_args = code.chars().map(|ch| format!("'{ch}'")).collect::<Vec<_>>().join(", ");
        let rust_args =
            code.bytes().map(|byte| format!("{byte}_i32")).collect::<Vec<_>>().join(", ");
        writeln!(c, "RECORD(\"MAKE_SQLSTATE_{index}\", MAKE_SQLSTATE({c_args}));").unwrap();
        writeln!(rust, "record(\"MAKE_SQLSTATE_{index}\", MAKE_SQLSTATE!({rust_args}), 0);")
            .unwrap();
        records += 1;
    }
    for value in 0..=127 {
        // Six-bit masking keeps every shifted signed operand representable in C int.
        writeln!(c, "RECORD(\"MAKE_SQLSTATE_ASCII_{value}\", MAKE_SQLSTATE({value}, {value}, {value}, {value}, {value}));").unwrap();
        writeln!(rust, "record(\"MAKE_SQLSTATE_ASCII_{value}\", MAKE_SQLSTATE!({value}_i32, {value}_i32, {value}_i32, {value}_i32, {value}_i32), 0);").unwrap();
        records += 1;
    }
    writeln!(c, "evaluations = 0; RECORD(\"PGSIXBIT_EVAL\", PGSIXBIT(argument()));").unwrap();
    writeln!(rust, "evaluations.set(0); let value = PGSIXBIT!(argument()); record(\"PGSIXBIT_EVAL\", value, evaluations.get());").unwrap();
    records += 1;
    let mut helpers = String::from("static unsigned argument_counts[5];\n");
    for index in 0..5 {
        writeln!(helpers, "static int argument_{index}(void) {{ ++argument_counts[{index}]; return 'A' + {index}; }}")
            .unwrap();
    }
    c = c.replace("int main(void) {", &format!("{helpers}int main(void) {{"));
    // Separate scalar objects make the unspecified C operand order well-defined.
    c.push_str("evaluations = 0; RECORD(\"MAKE_SQLSTATE_EVAL\", MAKE_SQLSTATE(argument_0(), argument_1(), argument_2(), argument_3(), argument_4()));\n");
    c.push_str("printf(\"MAKE_SQLSTATE_COUNTS\\t%u\\t%u\\t%u\\t%u\\t%u\\n\", argument_counts[0], argument_counts[1], argument_counts[2], argument_counts[3], argument_counts[4]);\n");
    rust.push_str("evaluations.set(0); let counts = [const { Cell::new(0_u32) }; 5];\n");
    for index in 0..5 {
        writeln!(rust, "let argument_{index} = || {{ counts[{index}].set(counts[{index}].get() + 1); 65_i32 + {index} }};")
            .unwrap();
    }
    rust.push_str("let value = MAKE_SQLSTATE!(argument_0(), argument_1(), argument_2(), argument_3(), argument_4()); record(\"MAKE_SQLSTATE_EVAL\", value, evaluations.get());\n");
    rust.push_str("println!(\"MAKE_SQLSTATE_COUNTS\\t{}\\t{}\\t{}\\t{}\\t{}\", counts[0].get(), counts[1].get(), counts[2].get(), counts[3].get(), counts[4].get());\n");
    records += 2;
    c.push_str("return 0; }\n");
    rust.push_str("}\n");
    compare_original(&frontend, &c, &rust, records);
}
