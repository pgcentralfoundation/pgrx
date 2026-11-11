//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! A macro whose C result is provably a truth value returns a Rust `bool`. Any other
//! expression macro returns the `CExpression` wrapper.
//!
//! The fixture has one macro or inline function for each shape the rule accepts and for
//! each it refuses. Only the public result changes: a truth-valued macro used as an
//! operand of another generated macro still contributes its C type.

#![cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos")
))]

/// Reconcile fixture bindings using production's actual storage collector.
#[path = "../../pgrx-bindgen/src/build/binding_symbols.rs"]
mod binding_symbols;
/// Locate the native SDK arguments.
#[path = "support/oracle.rs"]
#[allow(dead_code)]
mod oracle;
/// Compile generated Rust consumers with bounded tool execution.
#[path = "support/rust_oracle.rs"]
#[allow(dead_code)]
mod rust_oracle;

use pgrx_c_macros::{
    AnalysisSession, DocumentedResult, EmissionStatus, MacroEmission, MacroScanner,
    generate_with_bindings, inspect, postgres_function_macro_names, postgres_inline_function_names,
};
use std::path::PathBuf;

/// Preserve the production OID classification boundary in the shared fixture collector.
fn is_builtin_oid(name: &str) -> bool {
    name.ends_with("OID") && name != "HEAP_HASOID"
        || name.ends_with("RelationId")
        || name == "TemplateDbOid"
}

/// The directory that owns the fixture header, as a server include directory owns PostgreSQL's.
fn fixtures() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures")
}

/// The fixture header.
fn header() -> PathBuf {
    fixtures().join("bool_results.h")
}

/// The roots whose C result is a truth value.
const ACCEPTED: &[&str] = &[
    "B_EQUAL",
    "B_UNEQUAL",
    "B_LESS",
    "B_AT_LEAST",
    "B_AND",
    "B_OR",
    "B_NOT",
    "B_GROUPED",
    "B_CAST",
    "B_FLAG",
    "B_NATIVE",
    "B_INLINE",
    "B_COMMA",
    "B_STATEMENT",
    "B_CONDITIONAL",
    "B_CONDITIONAL_ZERO",
    "B_CONDITIONAL_ONE",
    "B_LIKELY",
    "B_UNLIKELY",
    "B_WRAPPER",
    "truth_inline",
];

/// The roots whose C result is a number, although some of them are always zero or one.
const REFUSED: &[&str] = &[
    "N_CONSTANT",
    "N_BOTH_CONSTANTS",
    "N_CONDITIONAL",
    "N_SUM",
    "N_MASK",
    "N_INT_CAST",
    "N_OPERAND",
    "N_EXPECTED",
    "N_NATIVE",
    "N_INLINE",
    "N_COUNT",
    "N_SIZE",
    "number_inline",
];

/// What the fixture generates.
struct Generated {
    /// The compiler arguments of the inspection.
    arguments: Vec<String>,
    /// The compiler that inspected it.
    compiler: PathBuf,
    /// The bindings.
    bindings: String,
    /// Each root's emission.
    macros: Vec<MacroEmission>,
    /// The Rust support of the macro set.
    support: String,
    /// The C support of the macro set.
    c_support: String,
}

impl Generated {
    /// Inspect the fixture and generate its macros and inline functions.
    fn new(scanner: &MacroScanner) -> Self {
        let mut arguments = oracle::native_arguments();
        arguments.push("-std=gnu17".into());
        let frontend = inspect(scanner, &header(), &arguments, None).expect("inspect the fixture");
        let mut names = postgres_function_macro_names(&frontend, &fixtures()).unwrap();
        let inline = postgres_inline_function_names(&frontend, &fixtures()).unwrap();
        let session = AnalysisSession::prepare_with_inline_functions(
            scanner,
            &frontend,
            &names,
            &[] as &[&str],
            &inline,
        )
        .expect("prepare roots");
        names.extend(inline);
        names.sort();
        let bindings = bindgen::Builder::default()
            .rust_target(bindgen::RustTarget::stable(85, 0).unwrap())
            .rust_edition(bindgen::RustEdition::Edition2024)
            .header(header().to_str().unwrap())
            .clang_args(&frontend.profile().arguments)
            .allowlist_var("truth_flag")
            .allowlist_function("truth_native|count_native")
            .use_core()
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
        let generated = generate_with_bindings(&session, &names, &catalog).unwrap();
        session.verify_inputs().expect("the inspected profile stayed unchanged");
        Self {
            arguments: frontend.profile().arguments.clone(),
            compiler: frontend.profile().compiler.executable.clone(),
            bindings,
            macros: generated.macros,
            support: generated.support.rust,
            c_support: generated.support.c_source,
        }
    }

    /// One root's Rust, whether a call returns `bool`, and its documentation.
    fn emitted(&self, name: &str) -> (&str, bool, &pgrx_c_macros::MacroDocumentation) {
        let emission = self
            .macros
            .iter()
            .find(|emission| emission.analysis.name == name)
            .unwrap_or_else(|| panic!("{name} is not a root"));
        match &emission.status {
            EmissionStatus::Emitted { rust, returns_bool, documentation, .. } => {
                (rust, *returns_bool, documentation)
            }
            EmissionStatus::Skipped { reason } => panic!("{name}: {reason:?}"),
        }
    }

    /// A Rust program that defines the generated macros and then runs `main`.
    fn program(&self, main: &str) -> String {
        let support = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../pgrx-pg-sys/src/c_macros/support.rs")
            .canonicalize()
            .unwrap();
        let mut rust = format!(
            "#![deny(unsafe_op_in_unsafe_fn, unused_unsafe)]\n#![allow(non_snake_case,non_camel_case_types,non_upper_case_globals,unused_parens,unused_braces,dead_code)]\n#[path={support:?}] pub mod __pgrx_c_macros;\npub mod ffi {{ pub unsafe fn pg_guard_ffi_boundary<T, F: FnOnce() -> T>(call: F) -> T {{ call() }} }}\n{}\n{}\n",
            self.bindings, self.support
        );
        for emission in &self.macros {
            if let EmissionStatus::Emitted { rust: definition, .. } = &emission.status {
                rust.push_str(definition);
            }
        }
        rust.push_str(main);
        rust
    }

    /// Compile and run a program against C definitions of the fixture's functions.
    fn run(&self, main: &str) -> String {
        let arguments = self.arguments.iter().map(String::as_str).collect::<Vec<_>>();
        let c = format!(
            "{}\n_Bool truth_flag = 1;\n_Bool truth_native(int value) {{ return value > 2; }}\nint count_native(int value) {{ return value > 2; }}\n",
            self.c_support
        );
        rust_oracle::run_rust_linked(&self.program(main), &self.compiler, &header(), &c, &arguments)
    }
}

/// Uses every accepted root as a Rust condition and every refused one through its wrapper.
const MAIN: &str = r#"
/// Print a name beside a `bool`, which only a `bool` satisfies.
fn truth(name: &str, value: bool) {
    println!("{name}\t{value}");
}

fn main() {
    // A proved truth value is a condition, with no method call.
    if B_EQUAL!(3_i32, 3_i32) && !B_EQUAL!(3_i32, 4_i32) {
        println!("conditions");
    }
    truth("unequal", B_UNEQUAL!(3_i32, 4_i32));
    truth("less", B_LESS!(3_i32, 4_i32));
    truth("at least", B_AT_LEAST!(3_i32, 4_i32));
    truth("and", B_AND!(3_i32, 0_i32));
    truth("or", B_OR!(3_i32, 0_i32));
    truth("not", B_NOT!(0_i32));
    truth("grouped", B_GROUPED!(4_i32, 3_i32));
    // A value that is neither zero nor one is still true once C converts it to `_Bool`.
    truth("cast", B_CAST!(256_i32));
    truth("comma", B_COMMA!(1_i32, 0_i32));
    truth("statement", B_STATEMENT!(7_i32));
    truth("conditional", B_CONDITIONAL!(0_i32, -2_i32));
    truth("conditional zero", B_CONDITIONAL_ZERO!(0_i32, 5_i32));
    truth("conditional one", B_CONDITIONAL_ONE!(1_i32, 0_i32));
    truth("wrapper", B_WRAPPER!(5_i32, 5_i32));
    truth("likely", B_LIKELY!(5_i32));
    truth("unlikely", B_UNLIKELY!(0_i32));
    println!("expected\t{}", N_EXPECTED!(5_i32).get());
    // SAFETY: the C side defines the functions and the object these read, and only this
    // thread runs.
    unsafe {
        truth("flag", B_FLAG!());
        truth("native", B_NATIVE!(3_i32));
        truth("inline", B_INLINE!(3_i32));
        truth("inline function", truth_inline!(2_i32));
        println!("native number\t{}", N_NATIVE!(3_i32).get());
        println!("inline number\t{}\t{}", N_INLINE!(3_i32).get(), number_inline!(3_i32).get());
    }
    // A number keeps its C identity behind the wrapper, also when it is always zero or one.
    let one: i32 = N_CONSTANT!(9_i32).get();
    println!("constant\t{one}\t{}", N_BOTH_CONSTANTS!(9_i32).get());
    println!("conditional number\t{}", N_CONDITIONAL!(0_i32, 0_i32).get());
    println!("sum\t{}\t{}", N_SUM!(3_i32, 4_i32).get(), N_MASK!(3_i32).is_true());
    println!("int cast\t{}\t{}", N_INT_CAST!(3_i32, 3_i32).get(), N_OPERAND!(7_u8).get());
    // Inside another macro a truth-valued macro is the C `int` it was: two of them add,
    // and its size is an `int`'s. The same invocation passed as a Rust expression is a
    // `bool`, which is C's `_Bool`.
    println!("count\t{}", N_COUNT!(3_i32, 4_i32).get());
    println!("operand sum\t{}", N_SUM!(B_EQUAL!(3_i32, 3_i32), B_LESS!(3_i32, 4_i32)).get());
    println!(
        "sizes\t{}\t{}",
        N_SIZE!(B_EQUAL!(3_i32, 3_i32)).get(),
        N_SIZE!((@__pgrx_c_native [B_EQUAL!(3_i32, 3_i32)])).get()
    );
}
"#;

#[test]
fn a_proved_truth_value_returns_rust_bool() {
    let scanner = MacroScanner::new().expect("libclang must be available");
    let generated = Generated::new(&scanner);

    // The two lists cover every root, and every root is emitted.
    let mut roots =
        generated.macros.iter().map(|emission| emission.analysis.name.as_str()).collect::<Vec<_>>();
    roots.sort_unstable();
    let mut expected = [ACCEPTED, REFUSED].concat();
    expected.sort_unstable();
    assert_eq!(roots, expected);

    for (names, truth) in [(ACCEPTED, true), (REFUSED, false)] {
        for name in names {
            let (rust, returns_bool, documentation) = generated.emitted(name);
            assert_eq!(returns_bool, truth, "{name}: {rust}");
            let compact = rust.split_whitespace().collect::<String>();
            // Only the public arm differs: it finishes the value arm as a `bool` or as the
            // wrapper, and no other arm finishes anything.
            assert_eq!(
                compact.matches("expression_result::finish_truth").count(),
                usize::from(truth),
                "{name}: {rust}"
            );
            assert_eq!(
                compact.matches("expression_result::finish(").count()
                    + compact.matches("expression_result::finish_checked(").count(),
                usize::from(!truth),
                "{name}: {rust}"
            );
            // The documentation says what a call returns, and the shell has that type.
            assert_eq!(rust.contains("Returns a Rust `bool`"), truth, "{name}: {rust}");
            assert_eq!(
                matches!(documentation.result, DocumentedResult::Truth { .. }),
                truth,
                "{name}: {:?}",
                documentation.result
            );
            assert_eq!(
                matches!(documentation.result, DocumentedResult::Value { .. }),
                !truth,
                "{name}: {:?}",
                documentation.result
            );
        }
    }

    assert_eq!(
        generated.run(MAIN),
        "conditions\nunequal\ttrue\nless\ttrue\nat least\tfalse\nand\tfalse\nor\ttrue\nnot\ttrue\ngrouped\ttrue\ncast\ttrue\ncomma\tfalse\nstatement\ttrue\nconditional\ttrue\nconditional zero\tfalse\nconditional one\ttrue\nwrapper\ttrue\nlikely\ttrue\nunlikely\tfalse\nexpected\t5\nflag\ttrue\nnative\ttrue\ninline\ttrue\ninline function\tfalse\nnative number\t1\ninline number\t1\t1\nconstant\t1\t1\nconditional number\t2\nsum\t7\ttrue\nint cast\t1\t7\ncount\t1\noperand sum\t2\nsizes\t4\t1\n"
    );

    // A `bool` has no wrapper methods, and a wrapper is no condition.
    for (invocation, error, explanation) in [
        (
            "let _ = B_EQUAL!(3_i32, 3_i32).get();",
            "E0599",
            "no method named `get` found for type `bool`",
        ),
        (
            "let _ = B_NOT!(3_i32).is_true();",
            "E0599",
            "no method named `is_true` found for type `bool`",
        ),
        (
            "let _ = B_CAST!(3_i32).into_value();",
            "E0599",
            "no method named `into_value` found for type `bool`",
        ),
        ("if N_SUM!(3_i32, 4_i32) {}", "E0308", "expected `bool`, found `CExpression<"),
        ("if N_BOTH_CONSTANTS!(3_i32) {}", "E0308", "expected `bool`, found `CExpression<"),
        (
            "let _: bool = N_INT_CAST!(3_i32, 3_i32);",
            "E0308",
            "expected `bool`, found `CExpression<",
        ),
    ] {
        let rejection =
            rust_oracle::reject_rust(&generated.program(&format!("fn main() {{ {invocation} }}")));
        assert!(rejection.contains(error), "{invocation}: {rejection}");
        assert!(rejection.contains(explanation), "{invocation}: {rejection}");
    }
}
