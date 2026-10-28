//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Check that documentation shells have the result types of the real macros.
//!
//! One program contains the runtime, the real generated support and macros, and
//! every macro's documentation shell renamed `__doc_X`. Each invocation asserts that
//! the real macro and its shell have one type. The program then runs shells with
//! side-effecting operands and checks that they panic without evaluating them.
//!
//! These generated consumers use the runtime's Linux/macOS host family.

#![cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos"),
))]

/// Reuse the binding build's collector so fixture tests reconcile exactly the Rust facts used
/// in production generation.
#[path = "../../pgrx-bindgen/src/build/binding_symbols.rs"]
mod binding_symbols;
/// Compile generated consumers through the bounded Rust oracle harness.
#[path = "support/rust_oracle.rs"]
mod rust_oracle;

use pgrx_c_macros::{
    AnalysisSession, DocumentedResult, EmissionStatus, MacroScanner, ParameterRole,
    documentation_shell_support, generate_with_bindings, inspect, pg_sys_integer_bridges,
    support_rust_cfg,
};
use std::collections::BTreeMap;
use std::fmt::Write;
use std::path::PathBuf;

/// Classify PostgreSQL OID constants as the binding build does; the fixture has none.
fn is_builtin_oid(name: &str) -> bool {
    name.ends_with("OID") && name != "HEAP_HASOID"
        || name.ends_with("RelationId")
        || name == "TemplateDbOid"
}

/// Expected result kind of each documented macro.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    /// `DocumentedResult::Value`.
    Value,
    /// `DocumentedResult::Unit`.
    Unit,
    /// `DocumentedResult::Diverges`.
    Diverges,
}

/// Macros, their expected result kinds, and invocations compared with their shells.
///
/// Each invocation is the body of a closure; `X!` in it is replaced by `__doc_X!`
/// for the shell. Operands come from `compare`'s parameters.
const MACROS: &[(&str, Kind, &str)] = &[
    ("DOC_CAST", Kind::Value, "DOC_CAST!(value)"),
    ("DOC_TYPEDEF", Kind::Value, "DOC_TYPEDEF!(value)"),
    ("DOC_SIZE_TYPEDEF", Kind::Value, "DOC_SIZE_TYPEDEF!(value)"),
    ("DOC_NULL", Kind::Value, "DOC_NULL!()"),
    ("DOC_ZERO", Kind::Value, "DOC_ZERO!()"),
    ("DOC_VOID", Kind::Value, "DOC_VOID!(value)"),
    ("DOC_FIELD", Kind::Value, "unsafe { DOC_FIELD!(record) }"),
    ("DOC_NESTED", Kind::Value, "unsafe { DOC_NESTED!(record) }"),
    ("DOC_STATE", Kind::Value, "unsafe { DOC_STATE!(record) }"),
    ("DOC_IS_BUSY", Kind::Value, "unsafe { DOC_IS_BUSY!(record) }"),
    ("DOC_MEMBER", Kind::Value, "unsafe { DOC_MEMBER!(record, value) }"),
    ("DOC_DATA", Kind::Value, "unsafe { DOC_DATA!(varlena) }"),
    ("DOC_MAX", Kind::Value, "DOC_MAX!(value, 3)"),
    ("DOC_AND", Kind::Value, "DOC_AND!(value, flag)"),
    ("DOC_NOT", Kind::Value, "DOC_NOT!(value)"),
    ("DOC_ADD", Kind::Value, "DOC_ADD!(value, word)"),
    ("DOC_ALIGN", Kind::Value, "DOC_ALIGN!(8, value)"),
    ("DOC_ALIGN8", Kind::Value, "DOC_ALIGN8!(value)"),
    ("DOC_CALLBACK", Kind::Value, "unsafe { DOC_CALLBACK!(record, value) }"),
    ("DOC_NATIVE", Kind::Value, "unsafe { DOC_NATIVE!(value) }"),
    ("DOC_VOID_CALL", Kind::Value, "unsafe { DOC_VOID_CALL!(value) }"),
    ("DOC_GLOBAL", Kind::Value, "unsafe { DOC_GLOBAL!() }"),
    ("DOC_OFFSET", Kind::Value, "DOC_OFFSET!(inner.count)"),
    ("DOC_OFFSET_INNER", Kind::Value, "DOC_OFFSET_INNER!()"),
    ("DOC_SIZEOF", Kind::Value, "DOC_SIZEOF!(value)"),
    ("DOC_SIZEOF_TYPE", Kind::Value, "DOC_SIZEOF_TYPE!()"),
    ("DOC_SIZEOF_PARAMETER", Kind::Value, "DOC_SIZEOF_PARAMETER!(DocRecord)"),
    ("DOC_TYPED", Kind::Value, "DOC_TYPED!(DocRecord, pointer)"),
    ("DOC_SWAP16", Kind::Value, "DOC_SWAP16!(value)"),
    ("DOC_LIKELY", Kind::Value, "DOC_LIKELY!(value)"),
    ("DOC_FILE", Kind::Value, "DOC_FILE!()"),
    ("DOC_LINE", Kind::Value, "DOC_LINE!()"),
    ("DOC_STRING", Kind::Value, "DOC_STRING!()"),
    ("DOC_REPORT", Kind::Value, "unsafe { DOC_REPORT!(value + 1) }"),
    ("DOC_ASSIGN", Kind::Value, "unsafe { DOC_ASSIGN!(pointer, value) }"),
    ("DOC_INCREMENT", Kind::Value, "unsafe { DOC_INCREMENT!(pointer) }"),
    ("DOC_COMMA", Kind::Value, "DOC_COMMA!(value, flag)"),
    ("DOC_ADDRESS", Kind::Value, "unsafe { DOC_ADDRESS!(record) }"),
    ("DOC_INDEX", Kind::Value, "unsafe { DOC_INDEX!(pointer, 1) }"),
    ("DOC_EMPTY", Kind::Unit, "DOC_EMPTY!();"),
    ("DOC_SWAP", Kind::Unit, "unsafe { DOC_SWAP!(pointer, other) }"),
    ("DOC_MAYBE_RETURN", Kind::Unit, "-> CValue<CInt> { DOC_MAYBE_RETURN!(flag); CValue::new(1) }"),
    ("DOC_RETURN", Kind::Diverges, "-> CValue<CInt> { DOC_RETURN!(@__pgrx_c_statement; flag) }"),
];

/// Original C definitions of the functions and global the fixture declares.
const NATIVE: &str = r#"
int doc_global = 4;
int doc_native(int value) { return value + doc_global; }
void doc_void(int value) { doc_global = value; }
void doc_report(const char *expression, const char *file, int line) {
    doc_global = expression[0] + file[0] + line;
}
"#;

/// Construct the C invocation used for both inspection and the native build.
fn arguments() -> Vec<String> {
    let mut arguments = vec!["-std=c17".into()];
    #[cfg(target_os = "macos")]
    {
        let sdk = rust_oracle::run_tool(
            std::process::Command::new("xcrun").arg("--show-sdk-path"),
            "documentation oracle SDK",
        );
        arguments.extend(["-isysroot".into(), sdk.trim().into()]);
    }
    arguments
}

/// Check that real macros and their documentation shells have one result type, that
/// each witness names only support items, and that shells never evaluate operands.
#[test]
fn documentation_shells_have_the_real_result_types() {
    let scanner = MacroScanner::new().expect("libclang must be available");
    let header =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/documentation_oracle.h");
    let frontend = inspect(&scanner, &header, &arguments(), None).expect("inspect the fixture");
    let names = MACROS.iter().map(|(name, ..)| *name).collect::<Vec<_>>();
    let session = AnalysisSession::prepare(&scanner, &frontend, &names).expect("analyze");
    let bindings = bindgen::Builder::default()
        .rust_target(bindgen::RustTarget::stable(85, 0).unwrap())
        .rust_edition(bindgen::RustEdition::Edition2024)
        .header(header.to_str().unwrap())
        .clang_args(&frontend.profile().arguments)
        .allowlist_type("Doc.*")
        .allowlist_function("doc_.*")
        .allowlist_var("doc_.*")
        .rustified_enum("DocState")
        .layout_tests(false)
        .generate_comments(false)
        .formatter(bindgen::Formatter::None)
        .generate()
        .expect("generate fixture bindings")
        .to_string();
    let mut catalog = binding_symbols::collect_bindings(
        &syn::parse_file(&bindings).unwrap(),
        session.integer_constants(),
        frontend.declarations(),
        &frontend.profile().target,
    );
    catalog.ffi_boundary = Some(vec!["ffi".into(), "boundary".into()]);
    let generated = generate_with_bindings(&session, &names, &catalog).unwrap();
    // pgrx-pg-sys includes these bridges with its support; they define `__pgrx_c_types`.
    let support =
        format!("{}{}", pg_sys_integer_bridges(&frontend).unwrap(), generated.support.rust);

    let mut macros = String::new();
    let mut shells = documentation_shell_support();
    let mut results = BTreeMap::new();
    for emission in &generated.macros {
        let name = emission.analysis.name.as_str();
        let EmissionStatus::Emitted { rust, documentation, .. } = &emission.status else {
            panic!("{name} must emit: {emission:?}");
        };
        macros.push_str(rust);
        let mut shell = (**documentation).clone();
        shell.identifier = format!("__doc_{name}");
        shells.push_str(&shell.render_shell(None, "").unwrap());
        check_witness(name, &documentation.result, &emission.analysis.parameters, &support);
        results.insert(name, documentation.result.clone());
    }
    for (name, kind, _) in MACROS {
        let actual = match &results[name] {
            DocumentedResult::Value { .. } => Kind::Value,
            DocumentedResult::Unit { .. } => Kind::Unit,
            DocumentedResult::Diverges => Kind::Diverges,
            DocumentedResult::Untyped { reason } => panic!("{name} is untyped: {reason}"),
        };
        assert_eq!(actual, *kind, "{name}");
    }
    // A delegating macro's witness renders the expanded tree, not the callee's arms.
    // Its operand is only cast, so the collapsed cast leaves it borrowed, and the
    // typedef cast names its `__pgrx_c_types` alias.
    let DocumentedResult::Value { witness } = &results["DOC_ALIGN8"] else { unreachable!() };
    assert!(!witness.contains("DOC_ALIGN!"), "{witness}");
    assert!(witness.starts_with("{ let _ = &$length; "), "{witness}");
    assert!(witness.contains("type_witness::<$crate::__pgrx_c_types::uintptr_t>()"), "{witness}");
    // Collapsed nodes name exactly the marker the real helper returns; other nodes,
    // such as member projection and select, render as in the real value arm.
    for (name, expected) in [
        ("DOC_CAST", "type_witness::<$crate::__pgrx_c_macros::CUnsignedShort>()"),
        ("DOC_TYPEDEF", "type_witness::<$crate::__pgrx_c_types::DocWord>()"),
        ("DOC_NULL", "null::null_void($crate::__pgrx_c_macros::expression::type_witness::<"),
        ("DOC_IS_BUSY", "{ let _ = &$record; "),
        ("DOC_AND", "type_witness::<$crate::__pgrx_c_macros::CInt>()"),
        ("DOC_SIZEOF", "type_witness::<$crate::__pgrx_c_macros::CSize>()"),
        ("DOC_OFFSET", "type_witness::<$crate::__pgrx_c_macros::CSize>()"),
        ("DOC_NATIVE", "type_witness::<$crate::__pgrx_c_macros::CInt>()"),
        ("DOC_VOID_CALL", "{ let _ = &$value; "),
        ("DOC_LIKELY", "type_witness::<$crate::__pgrx_c_macros::CLong>()"),
        ("DOC_FIELD", "project::<$crate::__pgrx_c_generated::Field_value, _, _>"),
        ("DOC_DATA", "type_witness::<$crate::__pgrx_c_macros::expression::CPointer<"),
        ("DOC_MAX", "select(if"),
        ("DOC_MAX", "(@native [$left])"),
        ("DOC_COMMA", "{ let _ = &$left; "),
    ] {
        let DocumentedResult::Value { witness } = &results[name] else { unreachable!() };
        assert!(witness.contains(expected), "{name}: {witness}");
    }
    assert_eq!(
        results["DOC_SWAP"],
        DocumentedResult::Unit { body: "{ let _ = &$left; let _ = &$right; }".into() }
    );
    assert_eq!(results["DOC_EMPTY"], DocumentedResult::Unit { body: "{ }".into() });

    let mut comparisons = String::new();
    for (name, _, invocation) in MACROS {
        let shell = invocation.replace(&format!("{name}!"), &format!("__doc_{name}!"));
        let (real, shell) = match invocation.strip_prefix("->") {
            Some(_) => (format!("||{invocation}"), format!("||{shell}")),
            None => (format!("|| {{ {invocation} }}"), format!("|| {{ {shell} }}")),
        };
        writeln!(comparisons, "    same({real}, {shell});").unwrap();
    }
    let program = format!(
        r#"#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, unused_parens, unused_unsafe, unreachable_code, clippy::all)]
#[path = {runtime:?}] pub mod __pgrx_c_macros;
mod ffi {{ pub unsafe fn boundary<R, F: FnOnce() -> R>(call: F) -> R {{ call() }} }}
#[derive(Clone, Copy)] pub struct Oid(u32);
impl Oid {{ pub fn to_u32(self) -> u32 {{ self.0 }} pub fn from_u32(value: u32) -> Self {{ Self(value) }} }}
#[derive(Clone, Copy)] pub struct TransactionId(u32);
impl TransactionId {{ pub fn into_inner(self) -> u32 {{ self.0 }} pub fn from_inner(value: u32) -> Self {{ Self(value) }} }}
{bindings}
{support}
{macros}
{shells}
use __pgrx_c_macros::{{CInt, CValue}};
/// Require both closures to return one type without calling either.
fn same<T>(_: impl FnOnce() -> T, _: impl FnOnce() -> T) {{}}
/// Compare every invocation; nothing here runs.
fn compare(record: *mut DocRecord, varlena: *mut DocVarlena, pointer: *mut i32, other: *mut i32, value: i32, word: u32, flag: i32) {{
{comparisons}}}
fn main() {{
    std::panic::set_hook(Box::new(|_| {{}}));
    let mut evaluated = 0;
    let value = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {{
        __doc_DOC_MAX!({{ evaluated += 1; 1 }}, {{ evaluated += 1; 2 }}).get()
    }}));
    assert!(value.is_err() && evaluated == 0);
    let (mut left, mut right) = (1, 2);
    let unit = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {{
        __doc_DOC_SWAP!({{ evaluated += 1; &raw mut left }}, &raw mut right)
    }}));
    assert!(unit.is_err() && evaluated == 0 && (left, right) == (1, 2));
    let _ = compare;
    println!("shells match");
}}
"#,
        runtime = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../pgrx-pg-sys/src/c_macros/support.rs")
            .canonicalize()
            .unwrap(),
    );
    let profile = frontend.profile();
    let arguments = profile.arguments.iter().map(String::as_str).collect::<Vec<_>>();
    let output = rust_oracle::run_rust_linked_with_cfg(
        &program,
        &profile.compiler.executable,
        &header,
        &format!("{NATIVE}\n{}", generated.support.c_source),
        &arguments,
        &support_rust_cfg(profile).unwrap(),
    );
    assert_eq!(output, "shells match\n");
}

/// Check one macro's documented result against the witness rules.
///
/// A witness names no generated macro's internal arms, argument normalizers or other
/// generated macros, and each support item it names is defined by the support. Each
/// evaluated operand occurs exactly in the witness or in a borrow, and no other
/// operand occurs at all.
fn check_witness(
    name: &str,
    result: &DocumentedResult,
    parameters: &[pgrx_c_macros::ParameterAnalysis],
    support: &str,
) {
    let block = match result {
        DocumentedResult::Value { witness } => witness,
        DocumentedResult::Unit { body } => body,
        DocumentedResult::Diverges => return,
        DocumentedResult::Untyped { reason } => panic!("{name} is untyped: {reason}"),
    };
    assert!(!block.contains("@__pgrx_emit_"), "{name}: {block}");
    assert!(!block.contains("__pgrx_c_args_"), "{name}: {block}");
    for (start, _) in block.match_indices("$crate::") {
        let path = &block[start + "$crate::".len()..];
        let end = path
            .find(|character: char| {
                !(character.is_alphanumeric() || character == '_' || character == ':')
            })
            .unwrap_or(path.len());
        let (path, rest) = path.split_at(end);
        if rest.starts_with('!') {
            assert!(
                matches!(path, "__pgrx_c_operand" | "__pgrx_c_field_marker"),
                "{name} names generated macro {path}"
            );
        }
        if let Some(item) = path.strip_prefix("__pgrx_c_generated::") {
            let item = item.split("::").next().unwrap();
            assert!(
                support.contains(&format!(" {item} "))
                    || support.contains(&format!(" {item}<"))
                    || support.contains(&format!(" {item}("))
                    || support.contains(&format!(" {item};")),
                "{name} names {item}, which the support does not define"
            );
        }
    }
    for parameter in parameters {
        let operand = format!("${}", parameter.name);
        let borrowed = block.contains(&format!("let _ = &{operand};"));
        let used = block.replace(&format!("let _ = &{operand};"), "").contains(&operand);
        if parameter.roles.contains(&ParameterRole::Value) {
            assert!(borrowed != used, "{name}: {operand} must occur once: {block}");
        } else if !parameter.roles.contains(&ParameterRole::Spelling)
            && !parameter.roles.contains(&ParameterRole::Identifier)
            && !parameter.roles.contains(&ParameterRole::Type)
        {
            assert!(!borrowed && !used, "{name}: {operand} is not evaluated: {block}");
        }
    }
}
