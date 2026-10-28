//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Check that documentation shells have the result types of the real macros.
//!
//! One program contains the runtime, the real generated support and macros, and
//! every macro's documentation shell renamed `__doc_X`. Each invocation asserts that
//! the real macro and its shell have one type. The program then runs shells with
//! side-effecting operands and checks that they panic without evaluating them.
//! Separate programs pin the documented cases where a shell rejects an invocation
//! that its real macro accepts, and where only the real macro needs `unsafe`.
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
    AnalysisSession, DocumentedResult, EmissionStatus, IntegerKind, MacroScanner, ParameterRole,
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
    // The classifier's literal descriptor and the shell's native one type literals alike.
    ("DOC_ADD", Kind::Value, "DOC_ADD!(value, -1)"),
    ("DOC_ADD", Kind::Value, "DOC_ADD!(value, 3000000000u32)"),
    ("DOC_ALIGN", Kind::Value, "DOC_ALIGN!(8, value)"),
    ("DOC_ALIGN8", Kind::Value, "DOC_ALIGN8!(value)"),
    ("DOC_CALLBACK", Kind::Value, "unsafe { DOC_CALLBACK!(record, value) }"),
    ("DOC_NATIVE", Kind::Value, "unsafe { DOC_NATIVE!(value) }"),
    ("DOC_VOID_CALL", Kind::Value, "unsafe { DOC_VOID_CALL!(value) }"),
    ("DOC_GLOBAL", Kind::Value, "unsafe { DOC_GLOBAL!() }"),
    ("DOC_OFFSET", Kind::Value, "DOC_OFFSET!(inner.count)"),
    ("DOC_OFFSET_INNER", Kind::Value, "DOC_OFFSET_INNER!()"),
    // A member designator is an identifier path, so a Rust keyword names a member.
    ("DOC_OFFSET_KEYWORD", Kind::Value, "DOC_OFFSET_KEYWORD!(type)"),
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
    // An argument that is only stringified is spelled, not evaluated.
    ("DOC_REPORT", Kind::Value, "unsafe { DOC_REPORT!(record->value) }"),
    ("DOC_ASSIGN", Kind::Value, "unsafe { DOC_ASSIGN!(pointer, value) }"),
    ("DOC_INCREMENT", Kind::Value, "unsafe { DOC_INCREMENT!(pointer) }"),
    ("DOC_COMMA", Kind::Value, "DOC_COMMA!(value, flag)"),
    ("DOC_ADDRESS", Kind::Value, "unsafe { DOC_ADDRESS!(record) }"),
    ("DOC_INDEX", Kind::Value, "unsafe { DOC_INDEX!(pointer, 1) }"),
    // A named pointer's dereference read as a place keeps the pointer's qualification
    // when spelled `*pointer` or `(*pointer)`; any other place is read as `ReadOnly`.
    ("DOC_NAME", Kind::Value, "unsafe { DOC_NAME!(*name) }"),
    ("DOC_NAME", Kind::Value, "unsafe { DOC_NAME!((*name)) }"),
    ("DOC_NAME", Kind::Value, "unsafe { DOC_NAME!((*holder).name) }"),
    // Every spelling of a `*const` pointer's dereference reads it as `ReadOnly`.
    ("DOC_NAME", Kind::Value, "unsafe { DOC_NAME!(*cname) }"),
    ("DOC_NAME", Kind::Value, "unsafe { DOC_NAME!((*cname)) }"),
    ("DOC_NAME", Kind::Value, "unsafe { DOC_NAME!(*(cname)) }"),
    ("DOC_NAME", Kind::Value, "unsafe { DOC_NAME!(((*cname))) }"),
    ("DOC_NAME_OF", Kind::Value, "unsafe { DOC_NAME_OF!(holder) }"),
    ("DOC_SPELL_NAME", Kind::Value, "unsafe { DOC_SPELL_NAME!((*holder).name) }"),
    ("DOC_SPELL_NAME", Kind::Value, "unsafe { DOC_SPELL_NAME!(*name) }"),
    ("DOC_SPELL_NAME", Kind::Value, "unsafe { DOC_SPELL_NAME!((*name)) }"),
    ("DOC_PICK", Kind::Value, "unsafe { DOC_PICK!(flag, (*holder).name, (*holder).name) }"),
    ("DOC_PICK", Kind::Value, "unsafe { DOC_PICK!(flag, *cname, *cname) }"),
    ("DOC_SET", Kind::Value, "unsafe { DOC_SET!(*pointer, value) }"),
    ("DOC_POSTINC", Kind::Value, "unsafe { DOC_POSTINC!(*pointer) }"),
    ("DOC_ACCUM", Kind::Value, "unsafe { DOC_ACCUM!(*counter, *counter) }"),
    ("DOC_CLEAR", Kind::Value, "unsafe { DOC_CLEAR!(*counter) }"),
    ("DOC_ADDRESS_OF", Kind::Value, "unsafe { DOC_ADDRESS_OF!(*counter) }"),
    ("DOC_BITS", Kind::Value, "unsafe { DOC_BITS!(bits) }"),
    ("DOC_OID", Kind::Value, "DOC_OID!(word)"),
    ("DOC_NOTHING", Kind::Value, "DOC_NOTHING!()"),
    // An unused argument may be empty or any tokens, as the C macro ignores it.
    ("DOC_IGNORE", Kind::Value, "DOC_IGNORE!()"),
    ("DOC_IGNORE", Kind::Value, "DOC_IGNORE!(record->value)"),
    ("DOC_IGNORE_FIRST", Kind::Value, "DOC_IGNORE_FIRST!(record, value)"),
    ("DOC_EMPTY", Kind::Unit, "DOC_EMPTY!();"),
    ("DOC_SWAP", Kind::Unit, "unsafe { DOC_SWAP!(pointer, other) }"),
    ("DOC_MAYBE_RETURN", Kind::Unit, "-> CValue<CInt> { DOC_MAYBE_RETURN!(flag); CValue::new(1) }"),
    ("DOC_RETURN", Kind::Diverges, "-> CValue<CInt> { DOC_RETURN!(@__pgrx_c_statement; flag) }"),
    // Translated static inline functions yield their converted result, or `()`, and a
    // translated call, as a macro's root or in its spine, has the callee's result type.
    ("DOC_INLINE_VALUE", Kind::Value, "DOC_INLINE_VALUE!(value)"),
    ("DOC_INLINE_WORD", Kind::Value, "DOC_INLINE_WORD!(word)"),
    ("DOC_INLINE_STORE", Kind::Value, "unsafe { DOC_INLINE_STORE!(pointer, value) }"),
    ("DOC_INLINE_REAL", Kind::Value, "DOC_INLINE_REAL!(value)"),
    ("DOC_INLINE_STATE", Kind::Value, "unsafe { DOC_INLINE_STATE!(record) }"),
    ("DOC_INLINE_NAME", Kind::Value, "unsafe { DOC_INLINE_NAME!(name) }"),
    ("DOC_INLINE_BUSY", Kind::Value, "unsafe { DOC_INLINE_BUSY!(record) }"),
    ("DOC_INLINE_NESTED", Kind::Value, "DOC_INLINE_NESTED!(value)"),
    ("DOC_INLINE_STORE_CALL", Kind::Value, "unsafe { DOC_INLINE_STORE_CALL!(pointer, value) }"),
    ("DOC_INLINE_WORD_CALL", Kind::Value, "DOC_INLINE_WORD_CALL!(word)"),
    ("DOC_INLINE_REAL_CALL", Kind::Value, "DOC_INLINE_REAL_CALL!(value)"),
    ("DOC_INLINE_STATE_CALL", Kind::Value, "unsafe { DOC_INLINE_STATE_CALL!(record) }"),
    ("DOC_INLINE_NAME_CALL", Kind::Value, "unsafe { DOC_INLINE_NAME_CALL!(name) }"),
    ("DOC_INLINE_BUSY_CALL", Kind::Value, "unsafe { DOC_INLINE_BUSY_CALL!(record) }"),
];

/// Invocations that a real macro accepts and its shell rejects, where the shell's
/// `expr` fragments and `@native` operand descriptor replace the argument normalizer
/// and classifier.
///
/// These are the cases ARCHITECTURE.md §15 and the extension guide list. Each
/// statement type-checks with the real macros in the main program, and the same
/// statement with shells must fail with the given diagnostic. A case that starts
/// compiling with shells means the documented list is out of date.
const DIVERGENCES: &[(&str, &str, &str)] = &[
    // A literal `0` passed as a null pointer to an operand-dependent spine.
    ("literal null", "unsafe { DOC_SET!(*slot, 0) };", "E0277"),
    // A dereference of a raw `*mut` pointer read as a place, spelled other than
    // `*pointer` or `(*pointer)`, or in a macro that reads several operands as places.
    (
        "((*pointer))",
        "let _: *mut ::core::ffi::c_char = unsafe { DOC_NAME!(((*name))) }.get();",
        "types differ in mutability",
    ),
    (
        "*(pointer)",
        "let _: *mut ::core::ffi::c_char = unsafe { DOC_NAME!(*(name)) }.get();",
        "types differ in mutability",
    ),
    (
        "several places",
        "let _: *mut ::core::ffi::c_char = unsafe { DOC_PICK!(flag, *name, *name) }.get();",
        "types differ in mutability",
    ),
    // A nested generated macro's bitfield and null-constant facts.
    ("nested bitfield", "let _: i32 = unsafe { DOC_ADD!(DOC_BITS!(bits), value) }.get();", "E0308"),
    ("nested null", "unsafe { DOC_SET!(*slot, DOC_NOTHING!()) };", "E0277"),
    // A nested generated macro where the C macro needs its operand's place.
    ("nested place", "unsafe { DOC_SET!(DOC_FIELD!(record), value) };", "E0745"),
    ("nested increment", "unsafe { DOC_POSTINC!(DOC_FIELD!(record)) };", "E0745"),
    ("nested address", "unsafe { DOC_ADDRESS_OF!(DOC_FIELD!(record)) };", "E0745"),
    ("nested read place", "unsafe { DOC_NAME!(DOC_NAME_OF!(holder)) };", "E0745"),
    // An unused or only stringified argument followed by another must be an expression.
    (
        "unparsed argument",
        "DOC_IGNORE_FIRST!(record->value, value);",
        "`->` is not valid syntax for field accesses",
    ),
    // The internal native operand form.
    (
        "internal native form",
        "DOC_ADD!((@__pgrx_c_native [value]), 1);",
        "expected expression, found `@`",
    ),
];

/// Statements that need `unsafe` with the real macros but not with shells, because
/// every unsafe operation in them is collapsed or dropped from the witness.
///
/// The main program compiles each with shells outside `unsafe`, and the same statement
/// with the real macros must fail, so the documented case stays accurate.
const UNSAFE_DROPPED: &[&str] = &[
    "let _ = DOC_NATIVE!(value);",
    "let _ = DOC_IS_BUSY!(record);",
    "DOC_SWAP!(pointer, other);",
    "let _ = DOC_INLINE_STORE!(pointer, value);",
];

/// Parameters of the functions holding compared invocations.
const OPERANDS: &str = "record: *mut DocRecord, varlena: *mut DocVarlena, pointer: *mut i32, other: *mut i32, name: *mut DocName, cname: *const DocName, holder: *mut DocHolder, counter: *mut DocCounter, bits: *mut DocBits, slot: *mut *mut DocRecord, value: i32, word: u32, flag: i32";

/// Rename every fixture macro invoked in `invocation` to its shell.
fn shell_invocation(invocation: &str) -> String {
    let mut shell = String::with_capacity(invocation.len());
    let mut rest = invocation;
    while let Some(start) = rest.find(|character: char| character.is_ascii_alphabetic()) {
        shell.push_str(&rest[..start]);
        rest = &rest[start..];
        let end = rest
            .find(|character: char| !(character.is_ascii_alphanumeric() || character == '_'))
            .unwrap_or(rest.len());
        let (identifier, after) = rest.split_at(end);
        let preceded = shell
            .ends_with(|character: char| character.is_ascii_alphanumeric() || character == '_');
        if !preceded
            && after.starts_with('!')
            && MACROS.iter().any(|(name, ..)| *name == identifier)
        {
            shell.push_str("__doc_");
        }
        shell.push_str(identifier);
        rest = after;
    }
    shell.push_str(rest);
    shell
}

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
    let mut names = MACROS.iter().map(|(name, ..)| *name).collect::<Vec<_>>();
    // A macro compared through several adjacent invocations is analyzed once.
    names.dedup();
    let session = AnalysisSession::prepare_with_inline_functions(
        &scanner,
        &frontend,
        &[] as &[&str],
        &[] as &[&str],
        &names,
    )
    .expect("analyze");
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
    // pgrx-pg-sys registers its `Oid` newtype as the storage of C `Oid`.
    catalog.integer_storage.insert("Oid".into(), IntegerKind::UnsignedInt);
    catalog.ffi_boundary = Some(vec!["ffi".into(), "boundary".into()]);
    let generated = generate_with_bindings(&session, &names, &catalog).unwrap();
    // pgrx-pg-sys includes these bridges with its support; they define `__pgrx_c_types`.
    let support =
        format!("{}{}", pg_sys_integer_bridges(&frontend).unwrap(), generated.support.rust);

    let mut macros = String::new();
    let mut shells = documentation_shell_support();
    let mut results = BTreeMap::new();
    let mut sources = BTreeMap::new();
    for emission in &generated.macros {
        let name = emission.analysis.name.as_str();
        let EmissionStatus::Emitted { rust, documentation, .. } = &emission.status else {
            panic!("{name} must emit: {emission:?}");
        };
        sources.insert(name, rust.split_whitespace().collect::<String>());
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
        ("DOC_NAME", "@read_place; (@native [$name])"),
        ("DOC_SET", "@place; (@native [$place])"),
        ("DOC_POSTINC", "post_modify("),
        ("DOC_ACCUM", "modify("),
        ("DOC_CLEAR", "null_constant("),
        ("DOC_ADDRESS_OF", "@place; (@native [$place])"),
        ("DOC_BITS", "Field_high"),
        ("DOC_OID", "CIntegerStorage<"),
    ] {
        let DocumentedResult::Value { witness } = &results[name] else { unreachable!() };
        assert!(witness.contains(expected), "{name}: {witness}");
    }
    assert_eq!(
        results["DOC_SWAP"],
        DocumentedResult::Unit { body: "{ let _ = &$left; let _ = &$right; }".into() }
    );
    assert_eq!(results["DOC_EMPTY"], DocumentedResult::Unit { body: "{ }".into() });
    // A translated body's witness is its result, or `()` for a void function, and it
    // borrows every argument. A translated call collapses like a native one, so no
    // witness names the callee's internal value arm, which the real macro expands. A
    // call root keeps the floating-profile wrapper of the real value arm. The markers
    // are pinned exactly, since a Rust alias such as `DocWord` has its target's type.
    const C: &str = "$crate::__pgrx_c_macros";
    const STATE: &str = "$crate::__pgrx_c_generated::EnumIdentity_enum__DocState";
    let name = format!("{C}::expression::CPointer<{C}::CChar, {C}::expression::ReadWrite>");
    // The body converts to the declared C enum's Rust enum. The call converts to the
    // integer storage that the native binding returns, as a native call does.
    let state =
        format!("{C}::expression::CEnumObject<{STATE}, {C}::CUnsignedInt, $crate::DocState>");
    let state_call = format!("{C}::expression::CEnumObject<{STATE}, {C}::CUnsignedInt, u32>");
    for (macro_name, borrows, marker) in [
        ("DOC_INLINE_VALUE", "$value", format!("{C}::CUnsignedShort")),
        ("DOC_INLINE_WORD", "$word", "$crate::__pgrx_c_types::DocWord".into()),
        ("DOC_INLINE_REAL", "$value", format!("{C}::expression::CDouble")),
        ("DOC_INLINE_STATE", "$record", state),
        ("DOC_INLINE_NAME", "$name", name.clone()),
        ("DOC_INLINE_BUSY", "$record", format!("{C}::CBool")),
        ("DOC_INLINE_WORD_CALL", "$word", "$crate::__pgrx_c_types::DocWord".into()),
        ("DOC_INLINE_REAL_CALL", "$value", format!("{C}::expression::CDouble")),
        ("DOC_INLINE_STATE_CALL", "$record", state_call),
        ("DOC_INLINE_NAME_CALL", "$name", name),
        ("DOC_INLINE_BUSY_CALL", "$record", format!("{C}::CBool")),
    ] {
        let value = format!("{C}::expression::type_witness::<{marker}>()");
        let value = if macro_name.ends_with("_CALL") {
            format!("{C}::expression::profile_value::<true, _>({value})")
        } else {
            value
        };
        assert_eq!(
            results[macro_name],
            DocumentedResult::Value { witness: format!("{{ let _ = &{borrows}; {value} }}") },
            "{macro_name}"
        );
    }
    for (macro_name, value) in [
        ("DOC_INLINE_STORE", "()".to_owned()),
        ("DOC_INLINE_STORE_CALL", format!("{C}::expression::profile_value::<true, _>(())")),
    ] {
        assert_eq!(
            results[macro_name],
            DocumentedResult::Value {
                witness: format!("{{ let _ = &$pointer; let _ = &$value; {value} }}")
            },
            "{macro_name}"
        );
    }
    let DocumentedResult::Value { witness } = &results["DOC_INLINE_NESTED"] else { unreachable!() };
    assert!(witness.starts_with("{ let _ = &$value; "), "{witness}");
    assert!(witness.contains(&format!("type_witness::<{C}::CUnsignedShort>()")), "{witness}");
    // The real macros expand the translation with compiled operands.
    for (name, ..) in MACROS {
        let callee = match name.strip_suffix("_CALL") {
            Some(callee) if callee.starts_with("DOC_INLINE_") => callee,
            _ if *name == "DOC_INLINE_NESTED" => "DOC_INLINE_VALUE",
            _ => continue,
        };
        let call = format!("$crate::{callee}!(@__pgrx_emit_value;(@compiled");
        assert!(sources[name].contains(&call), "{name} must expand {callee}'s translation");
    }

    let mut comparisons = String::new();
    for (_, _, invocation) in MACROS {
        let shell = shell_invocation(invocation);
        let (real, shell) = match invocation.strip_prefix("->") {
            Some(_) => (format!("||{invocation}"), format!("||{shell}")),
            None => (format!("|| {{ {invocation} }}"), format!("|| {{ {shell} }}")),
        };
        writeln!(comparisons, "    same({real}, {shell});").unwrap();
    }
    let mut divergences = String::new();
    for (_, invocation, _) in DIVERGENCES {
        writeln!(divergences, "    {invocation}").unwrap();
    }
    let mut safe_shells = String::new();
    for statement in UNSAFE_DROPPED {
        writeln!(safe_shells, "    {}", shell_invocation(statement)).unwrap();
    }
    let prelude = format!(
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
"#,
        runtime = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../pgrx-pg-sys/src/c_macros/support.rs")
            .canonicalize()
            .unwrap(),
    );
    let program = format!(
        r#"{prelude}
/// Require both closures to return one type without calling either.
fn same<T>(_: impl FnOnce() -> T, _: impl FnOnce() -> T) {{}}
/// Compare every invocation; nothing here runs.
fn compare({OPERANDS}) {{
{comparisons}}}
/// The real macros accept every documented divergence; nothing here runs.
fn divergences({OPERANDS}) {{
{divergences}}}
/// These shells need no `unsafe`; nothing here runs.
fn safe_shells({OPERANDS}) {{
{safe_shells}}}
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
    let inline = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {{
        __doc_DOC_INLINE_VALUE!({{ evaluated += 1; 1 }}).get()
    }}));
    assert!(inline.is_err() && evaluated == 0);
    let _ = (compare, divergences, safe_shells);
    println!("shells match");
}}
"#,
    );
    let profile = frontend.profile();
    let arguments = profile.arguments.iter().map(String::as_str).collect::<Vec<_>>();
    let configurations = support_rust_cfg(profile).unwrap();
    let output = rust_oracle::run_rust_linked_with_cfg(
        &program,
        &profile.compiler.executable,
        &header,
        &format!("{NATIVE}\n{}", generated.support.c_source),
        &arguments,
        &configurations,
    );
    assert_eq!(output, "shells match\n");
    for (case, invocation, diagnostic) in DIVERGENCES {
        let rejected = format!(
            "{prelude}\nfn divergence({OPERANDS}) {{\n    {}\n}}\nfn main() {{ let _ = divergence; }}\n",
            shell_invocation(invocation)
        );
        let stderr = rust_oracle::reject_rust_with_cfg(&rejected, &configurations);
        assert!(stderr.contains(diagnostic), "{case}: expected {diagnostic}:\n{stderr}");
    }
    for statement in UNSAFE_DROPPED {
        let rejected = format!(
            "{prelude}\nfn real({OPERANDS}) {{\n    {statement}\n}}\nfn main() {{ let _ = real; }}\n"
        );
        let stderr = rust_oracle::reject_rust_with_cfg(&rejected, &configurations);
        assert!(stderr.contains("E0133"), "{statement} needs unsafe:\n{stderr}");
    }
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
