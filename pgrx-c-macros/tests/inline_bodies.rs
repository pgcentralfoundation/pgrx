//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Translate static inline function bodies and compare them with the original C.
//! Bodies may use macros that expand the same way at the end of the translation unit,
//! void functions are translated, and calls between translations expand in place.

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
/// Compile generated Rust consumers and paired rejections with bounded tool execution.
#[path = "support/rust_oracle.rs"]
#[allow(dead_code)]
mod rust_oracle;

use pgrx_c_macros::{
    AnalysisSession, BindingCatalog, DocumentedResult, EmissionStatus, MacroScanner,
    emit_batch_with_bindings, emit_support_artifact_with_bindings, inspect,
};
use std::path::PathBuf;
use std::sync::Mutex;

/// Serialize libclang-backed inspection within this test process because its safe runtime
/// permits one active owner.
static SCANNER_LOCK: Mutex<()> = Mutex::new(());

/// Preserve the production OID classification boundary in the shared fixture collector.
fn is_builtin_oid(name: &str) -> bool {
    name.ends_with("OID") && name != "HEAP_HASOID"
        || name.ends_with("RelationId")
        || name == "TemplateDbOid"
}

/// Inline functions whose definitions the generator translates.
/// BODY_DROPPED names a macro in an argument that the enclosing macro drops unexpanded.
const TRANSLATED: &[&str] = &[
    "BODY_MACRO",
    "BODY_DROPPED",
    "BODY_BRANCH",
    "BODY_STORE",
    "BODY_COLOR",
    "BODY_LEAF",
    "BODY_CALLER",
    "BODY_RECURSIVE",
    // Its argument temporaries outlive the body.
    "BODY_MARK",
    // A sizeof in the callee checks arguments from translated and macro callers.
    "BODY_SIZED",
    "BODY_SIZED_CALLER",
    // The directives' comment and spliced line belong to them, not to the body.
    "BODY_DIRECTIVE_COMMENT",
    "BODY_DIRECTIVE_SPLICE",
    // A line splice is part of the spelling of the token after it.
    "BODY_BRACE_SPLICE",
    "BODY_SPLICED_HASH",
    "BODY_SPLICED_DIGRAPH",
    // Defined by the first of two entries of its header, as that entry's conditionals select.
    "BODY_FIRST_BRANCH",
    "BODY_RESCAN_TARGET",
    // A floating argument converts to an integer parameter.
    "BODY_ORDER",
    // Only the conversion of its integer result is floating.
    "BODY_RETURN_FMA",
    // A scalar member of a record value.
    "BODY_ARRAY_COUNT",
];

/// Macros that call translated inline functions, as their root and inside their spine.
const MACROS: &[&str] = &["BODY_SIZED_MACRO", "BODY_LEAF_PLUS"];

/// Inline functions that stay native calls, each for a reason the fixture names.
const NATIVE: &[&str] = &[
    // `return;` is outside the statement grammar.
    "BODY_EARLY",
    // A record cannot leave a translation by value.
    "BODY_PAIR",
    "BODY_ARRAY",
    // An array member of a record value would decay into a pointer into a copy.
    "BODY_ARRAY_INDEX",
    "BODY_ARRAY_DECAY",
    "BODY_ARRAY_SELECT",
    // The type of a member of a call's result through a pointer is not established.
    "BODY_MADE_COUNT",
    // The macro is defined only after the function, which used the enum constant.
    "BODY_LATE_USE",
    // The macro is redefined after the function.
    "BODY_REDEFINED_USE",
    // `__LINE__` names the header line, not the Rust invocation.
    "BODY_LINE",
    // The statement ends inside a macro argument, so its extent omits the closing `)`.
    "BODY_MACRO_ARG",
    // A popped definition, not the last one, is active at the end.
    "BODY_PUSHED",
    // A macro its body reaches is the subject of a pop pragma, also one a splice divides.
    "BODY_POPPED",
    "BODY_SPLICED_POP",
    // Its expansion used a definition other than the last one.
    "BODY_HIDDEN",
    // A macro reached through another macro is redefined after the function.
    "BODY_TRANSITIVE_USE",
    // A macro in an argument dropped by the invocation is redefined after the function.
    // The proof does not consider how the invocation uses its arguments.
    "BODY_DROPPED_LATER",
    // An argument's enum constant later becomes a macro.
    "BODY_ARGUMENT_LATE",
    // A name that only rescanning expanded is a macro that is later undefined.
    "BODY_RESCAN",
    "BODY_RESCAN_ARGUMENT",
    // Their replay at the end of the headers would not keep the spacing a macro stringifies.
    "BODY_STRING_SIZE",
    "BODY_STRING_CHAR",
    // Their own floating values need a floating profile for their arithmetic.
    "BODY_FMA",
    "BODY_INT_FMA",
    // So do floating objects they reach through a pointer, and a call through a member,
    // whose result type analysis does not establish.
    "BODY_FIELD_FMA",
    "BODY_INDEX_FMA",
    "BODY_DEREF_FMA",
    "BODY_STORE_FMA",
    "BODY_CALL_FMA",
    // Their header is entered twice, with different macros and conditionals. The first
    // entry's macro is redefined before the second; the others are in the second entry.
    "BODY_FIRST_VALUE",
    "BODY_SECOND_VALUE",
    "BODY_SECOND_BRANCH",
    "BODY_GUARDED_FIRST",
    "BODY_GUARDED_SECOND",
    // Trigraphs are not modeled.
    "BODY_TRIGRAPH_HASH",
];

/// The runtime support path in generated code.
const C: &str = "$crate::__pgrx_c_macros";
/// The documentation type witness, before its marker.
const WITNESS: &str = "$crate::__pgrx_c_macros::expression::type_witness::";

/// Compare translated and native inline roots with the original compiler's results.
#[test]
fn translated_bodies_match_their_original_definitions() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let header = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/inline_bodies.h");
    let mut arguments = oracle::native_arguments();
    arguments.push("-std=c17".into());
    let frontend = inspect(&scanner, &header, &arguments, None).expect("inspect definitions");
    let names = TRANSLATED.iter().chain(NATIVE).chain(MACROS).copied().collect::<Vec<_>>();
    let session = AnalysisSession::prepare_with_inline_functions(
        &scanner,
        &frontend,
        &[] as &[String],
        &[] as &[String],
        &names,
    )
    .expect("prepare inline roots");
    let bindings = bindgen::Builder::default()
        .rust_target(bindgen::RustTarget::stable(85, 0).unwrap())
        .rust_edition(bindgen::RustEdition::Edition2024)
        .header(header.to_str().unwrap())
        .clang_args(&frontend.profile().arguments)
        .allowlist_type("Body.*")
        .layout_tests(false)
        .generate_comments(false)
        .formatter(bindgen::Formatter::None)
        .generate()
        .expect("generate bindings")
        .to_string();
    let mut catalog = binding_symbols::collect_bindings(
        &syn::parse_file(&bindings).unwrap(),
        session.integer_constants(),
        frontend.declarations(),
        &frontend.profile().target,
    );
    catalog.ffi_boundary = Some(vec!["ffi".into(), "pg_guard_ffi_boundary".into()]);
    let artifact = emit_support_artifact_with_bindings(&session, &names, &catalog)
        .expect("generate native call transport");
    let support = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../pgrx-pg-sys/src/c_macros/support.rs")
        .canonicalize()
        .unwrap();
    let mut rust = format!(
        "#![deny(unsafe_op_in_unsafe_fn)]\n#![allow(non_snake_case,non_camel_case_types,unused_parens,dead_code)]\n#[path={support:?}] pub mod __pgrx_c_macros;\n{bindings}\n{}\n",
        artifact.rust
    );
    for emission in emit_batch_with_bindings(&session, &names, &catalog).unwrap() {
        let name = emission.analysis.name.clone();
        let EmissionStatus::Emitted { rust: definition, documentation, .. } = emission.status
        else {
            panic!("root {name} must emit: {emission:?}");
        };
        assert_eq!(
            definition.contains("Translates the original function definition"),
            TRANSLATED.contains(&name.as_str()),
            "{name}"
        );
        let compact = definition.split_whitespace().collect::<String>();
        match name.as_str() {
            "BODY_CALLER" | "BODY_LEAF_PLUS" => {
                assert!(compact.contains("$crate::BODY_LEAF!(@__pgrx_emit_value;(@compiled"));
            }
            "BODY_SIZED_CALLER" | "BODY_SIZED_MACRO" => {
                assert!(compact.contains("$crate::BODY_SIZED!(@__pgrx_emit_value;(@compiled"));
            }
            // Expanding a recursive translation would never terminate.
            "BODY_RECURSIVE" => {
                assert!(!compact.contains("BODY_RECURSIVE!(@__pgrx_emit_value;(@compiled"));
            }
            _ => {}
        }
        // A translated body evaluates its arguments outside generated `unsafe`, so only the
        // arm used by a caller's own size witness checks them. Its size witnesses, and the
        // operands it passes to translated callees, carry no checks that a chain of
        // translated calls would multiply.
        if TRANSLATED.contains(&name.as_str()) {
            assert_eq!(
                compact.matches("@check_safety").count(),
                emission.analysis.parameters.len(),
                "{name}"
            );
        }
        // A translated body's documentation witness borrows every argument and names the
        // marker of its declared result, or is `()` for a void function. A translated call
        // collapses to its result like a native call, so no witness names another macro's
        // internal arm. The documentation oracle compares these types with the real macros'.
        let DocumentedResult::Value { witness } = &documentation.result else {
            panic!("{name} has no value witness: {:?}", documentation.result);
        };
        assert!(!witness.contains("@__pgrx_emit_value"), "{name}: {witness}");
        if TRANSLATED.contains(&name.as_str()) {
            let borrows = emission
                .analysis
                .parameters
                .iter()
                .map(|parameter| format!("let _ = &${}; ", parameter.name))
                .collect::<String>();
            let result = &frontend.declarations().function_signatures[&name].signature.result;
            let value = match result.spelling.as_str() {
                "void" => "()".into(),
                "int" => format!("{WITNESS}<{C}::CInt>()"),
                "double" => format!("{WITNESS}<{C}::expression::CDouble>()"),
                "BodyColor" => format!(
                    "{WITNESS}<{C}::expression::CEnumObject<$crate::__pgrx_c_generated::EnumIdentity_enum__BodyColor, {C}::CUnsignedInt, u32>>()"
                ),
                other => panic!("{name}: no expected marker for {other}"),
            };
            assert_eq!(*witness, format!("{{ {borrows}{value} }}"), "{name}");
        }
        if MACROS.contains(&name.as_str()) {
            assert!(
                witness.contains("type_witness::<$crate::__pgrx_c_macros::CInt>()"),
                "{name}: {witness}"
            );
        }
        rust.push_str(&definition);
    }
    let prelude = include_str!("fixtures/inline_bodies.rs").split_once("fn main()").unwrap().0;
    // A translated pointer write still needs unsafe; a translated macro body that only
    // computes a value does not.
    let error = rust_oracle::reject_rust(&format!(
        "{rust}\n{prelude}\nconst EXPECTED_GUARDS: usize = 0;\nfn main() {{ let mut value = 0_i32; BODY_STORE!(&raw mut value, 7_i32).get(); }}\n"
    ));
    assert!(error.contains("unsafe"), "{error}");
    // BODY_EARLY and BODY_ARRAY_SELECT are called twice and each other native root once,
    // BODY_ARRAY only by the translated BODY_ARRAY_COUNT. The translated BODY_RECURSIVE
    // calls itself natively once.
    rust.push_str("const EXPECTED_GUARDS: usize = 38;\n");
    rust.push_str("fn pure() -> i32 { BODY_MACRO!(1_i32).get() + BODY_CALLER!(1_i32).get() }\n");
    rust.push_str(include_str!("fixtures/inline_bodies.rs"));
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
        include_str!("fixtures/inline_bodies.c"),
        &arguments,
        true,
    );
    assert_eq!(generated, original);
    session.verify_inputs().expect("original profile stayed unchanged");
}

/// A trigraph that spells a backslash, or a bare carriage return, could divide the name of
/// a pop pragma in a way the search for those pragmas does not model. In a unit with
/// either, as when a pop's operand cannot be read, no body that names a macro is retained.
#[test]
fn unmodeled_line_ends_leave_no_macro_bodies() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let header =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/inline_bodies_phases.h");
    let mut arguments = oracle::native_arguments();
    arguments.push("-std=c17".into());
    let frontend = inspect(&scanner, &header, &arguments, None).expect("inspect definitions");
    let retained = |name: &str| {
        let function = &frontend.declarations().function_signatures[name];
        function.definition.as_ref().expect("proved definition").body.is_some()
    };
    assert!(retained("BODY_PHASE_PLAIN"));
    for name in ["BODY_PHASE_MACRO", "BODY_TRIGRAPH_SPLICE", "BODY_CR_STORE"] {
        assert!(!retained(name), "{name}");
    }
}

/// A `-D` definition can pop a macro through `_Pragma` wherever a header expands it. A body
/// that reaches the popped name is not retained, whether the definition is joined to the
/// option or follows it, while a body that names another macro is.
#[test]
fn command_line_pop_pragmas_leave_no_body() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let directory = tempfile::tempdir().unwrap();
    let header = directory.path().join("defined_pop.h");
    std::fs::write(
        &header,
        "#define BODY_POP_VALUE 1000\n\
#pragma push_macro(\"BODY_POP_VALUE\")\n\
#undef BODY_POP_VALUE\n\
typedef enum BodyPopEnum { BODY_POP_VALUE = 5 } BodyPopEnum;\n\
#define BODY_POP_SCALE BODY_POP_VALUE\n\
static inline int BODY_DEFINED_POP(int value) { return value + BODY_POP_SCALE; }\n\
#define BODY_POP_OFFSET 1\n\
static inline int BODY_DEFINED_PLAIN(int value) { return value + BODY_POP_OFFSET; }\n\
BODY_POP\n",
    )
    .unwrap();
    let definition = r#"BODY_POP=_Pragma("pop_macro(\"BODY_POP_VALUE\")")"#;
    for operand in [vec![format!("-D{definition}")], vec!["-D".into(), definition.into()]] {
        let mut arguments = oracle::native_arguments();
        arguments.extend(operand);
        let frontend = inspect(&scanner, &header, &arguments, None).expect("inspect definitions");
        let retained = |name: &str| {
            let function = &frontend.declarations().function_signatures[name];
            function.definition.as_ref().expect("proved definition").body.is_some()
        };
        assert!(retained("BODY_DEFINED_PLAIN"));
        assert!(!retained("BODY_DEFINED_POP"));
    }
}

/// A header need not be UTF-8 and may hold a NUL byte. A definition keeps its body when
/// only another part of its header, here a comment, has a Latin-1 byte. A definition with
/// one in its own text keeps its native call, documented with the text converted lossily,
/// but no body, since libclang spells its tokens lossily.
#[test]
fn latin1_header_keeps_utf8_definitions() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let directory = tempfile::tempdir().unwrap();
    let header = directory.path().join("latin1.h");
    std::fs::write(
        &header,
        b"/* Caf\xe9 \0 */\n#define BODY_LATIN_OFFSET 1\n\
static inline int BODY_LATIN_PLAIN(int value) { return value + BODY_LATIN_OFFSET; }\n\
static inline int BODY_LATIN_INSIDE(int value) { return value; /* na\xefve */ }\n",
    )
    .unwrap();
    let frontend =
        inspect(&scanner, &header, &oracle::native_arguments(), None).expect("inspect definitions");
    let definition = |name: &str| {
        let function = &frontend.declarations().function_signatures[name];
        function.definition.clone().expect("proved definition")
    };
    let plain = definition("BODY_LATIN_PLAIN");
    assert!(plain.source.is_some_and(|source| source.ends_with("BODY_LATIN_OFFSET; }")));
    assert!(plain.body.is_some());
    let inside = definition("BODY_LATIN_INSIDE");
    assert!(inside.source.is_some_and(|source| source.contains("na\u{fffd}ve")));
    assert!(inside.body.is_none());
    let names = ["BODY_LATIN_PLAIN", "BODY_LATIN_INSIDE"];
    let session = AnalysisSession::prepare_with_inline_functions(
        &scanner,
        &frontend,
        &[] as &[String],
        &[] as &[String],
        &names,
    )
    .expect("prepare inline roots");
    let catalog = BindingCatalog {
        ffi_boundary: Some(vec!["ffi".into(), "pg_guard_ffi_boundary".into()]),
        ..BindingCatalog::default()
    };
    for emission in emit_batch_with_bindings(&session, &names, &catalog).unwrap() {
        let name = emission.analysis.name.clone();
        let EmissionStatus::Emitted { rust: definition, .. } = emission.status else {
            panic!("root {name} must emit: {emission:?}");
        };
        assert_eq!(
            definition.contains("Translates the original function definition"),
            name == "BODY_LATIN_PLAIN",
            "{name}"
        );
    }
}

/// Preprocessed compiler output is read as UTF-8 and never converted, which could change a
/// literal that analysis compares. A string literal that is not UTF-8, here in a function
/// that is not a root, fails the preparation of a root's macro expansion with an error
/// rather than changing the literal.
#[test]
fn latin1_literals_fail_preparation() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let directory = tempfile::tempdir().unwrap();
    let header = directory.path().join("latin1_literal.h");
    std::fs::write(
        &header,
        b"#define BODY_LATIN_ONE 1\n\
static inline int BODY_LATIN_ROOT(int value) { return value + BODY_LATIN_ONE; }\n\
static inline int BODY_LATIN_LITERAL(int value) { return value + (int) sizeof(\"caf\xe9\"); }\n",
    )
    .unwrap();
    let frontend =
        inspect(&scanner, &header, &oracle::native_arguments(), None).expect("inspect definitions");
    let Err(error) = AnalysisSession::prepare_with_inline_functions(
        &scanner,
        &frontend,
        &[] as &[String],
        &[] as &[String],
        &["BODY_LATIN_ROOT"],
    ) else {
        panic!("a literal that is not UTF-8 must fail preparation");
    };
    assert!(error.to_string().contains("non-UTF-8 output"), "{error}");
}

/// Clang names a header it finds through a relative include directory by a path relative to
/// the working directory. A header with the same relative path beside the main header is
/// another file, so each definition is read from its own file. Both headers have the same
/// layout, but only the first splices its directive over the line that adds 1.
#[test]
fn headers_with_one_relative_path_keep_their_own_text() {
    /// The main header's directory, set only in the subprocess that runs in another one.
    const CHILD: &str = "PGRX_C_MACROS_RELATIVE_PATH_TEST_CHILD";
    let Some(directory) = std::env::var_os(CHILD) else {
        let root = tempfile::tempdir().unwrap();
        let (directory, working) = (root.path().join("main"), root.path().join("working"));
        for (parent, name, splice) in
            [(&directory, "BODY_SPAN_A", '\\'), (&working, "BODY_SPAN_B", ' ')]
        {
            std::fs::create_dir_all(parent.join("relative")).unwrap();
            std::fs::write(
                parent.join("relative/span.h"),
                format!(
                    "static inline int {name}(int value)\n{{\n\treturn value\n#if 1 {splice}\n\t+ 1\n#endif\n\t;\n}}\n"
                ),
            )
            .unwrap();
        }
        std::fs::write(
            directory.join("span.h"),
            "#include \"relative/span.h\"\n#include <span.h>\n",
        )
        .unwrap();
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "headers_with_one_relative_path_keep_their_own_text", "--nocapture"])
            .env(CHILD, &directory)
            .current_dir(&working)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        return;
    };
    let scanner = MacroScanner::new().expect("libclang must be available");
    let mut arguments = oracle::native_arguments();
    arguments.push("-Irelative".into());
    let header = PathBuf::from(directory).join("span.h");
    let frontend = inspect(&scanner, &header, &arguments, None).expect("inspect definitions");
    for (name, body) in [
        ("BODY_SPAN_A", &["return", "value", ";"][..]),
        ("BODY_SPAN_B", &["return", "value", "+", "1", ";"][..]),
    ] {
        let function = &frontend.declarations().function_signatures[name];
        let definition = function.definition.clone().expect("proved definition");
        assert!(definition.source.is_some_and(|source| source.contains(name)), "{name}");
        let tokens = definition.body.expect("retained body");
        assert!(
            tokens.iter().map(|token| token.spelling.as_str()).eq(body.iter().copied()),
            "{name}"
        );
    }
}
