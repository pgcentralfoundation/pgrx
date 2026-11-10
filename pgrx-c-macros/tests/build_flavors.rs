//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! One set of headers, inspected with and without `USE_ASSERT_CHECKING`, generates the same
//! macros: the same names, operands, results and needs for `unsafe`, from the same Rust
//! source. Only the constant that says whether assertions run differs. With assertions every
//! check runs and a failed one stops the process; without them no check is evaluated.

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
    AnalysisSession, EmissionStatus, MacroEmission, MacroScanner, SkipReasonCode,
    generate_with_bindings, inspect, postgres_function_macro_names, postgres_inline_function_names,
};
use std::collections::BTreeSet;
use std::path::PathBuf;

/// libclang admits one instance per process, so the tests that inspect take turns.
static SCANNER_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

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
    fixtures().join("build_flavors.h")
}

/// What one build flavor of the fixture generates.
struct Flavor {
    /// The compiler arguments that select the flavor.
    arguments: Vec<String>,
    /// The compiler that inspected it.
    compiler: PathBuf,
    /// The roots selected as a binding generator selects PostgreSQL's.
    names: Vec<String>,
    /// Names a binding generator leaves out.
    hidden: BTreeSet<String>,
    /// The bindings generated without them.
    bindings: String,
    /// Each root's emission.
    macros: Vec<MacroEmission>,
    /// The Rust support of the macro set.
    support: String,
    /// The C support of the macro set.
    c_support: String,
}

impl Flavor {
    /// Inspect the fixture as one flavor and generate its macros.
    fn generate(scanner: &MacroScanner, assert_enabled: bool) -> Self {
        let mut arguments = oracle::native_arguments();
        arguments.push("-std=c17".into());
        if assert_enabled {
            arguments.push("-DUSE_ASSERT_CHECKING=1".into());
        }
        let frontend = inspect(scanner, &header(), &arguments, None).expect("inspect the fixture");
        assert_eq!(frontend.assert_enabled(), assert_enabled);
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
        let hidden = frontend.flavor_specific_names().map(str::to_owned).collect::<BTreeSet<_>>();
        // Bind every variable the flavor declares, except those a binding generator leaves
        // out. Functions get generated adapters in both flavors.
        let bindings = hidden
            .iter()
            .fold(bindgen::Builder::default(), |builder, name| {
                builder.blocklist_item(format!("^{name}$"))
            })
            .rust_target(bindgen::RustTarget::stable(85, 0).unwrap())
            .rust_edition(bindgen::RustEdition::Edition2024)
            .header(header().to_str().unwrap())
            .clang_args(&frontend.profile().arguments)
            .allowlist_var(".*Variable")
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
            names,
            hidden,
            bindings,
            macros: generated.macros,
            support: generated.support.rust,
            c_support: generated.support.c_source,
        }
    }

    /// One root's emission.
    fn emission(&self, name: &str) -> &MacroEmission {
        self.macros
            .iter()
            .find(|emission| emission.analysis.name == name)
            .unwrap_or_else(|| panic!("{name} is not a root"))
    }

    /// An emitted macro's Rust and whether it needs the caller's `unsafe`.
    fn emitted(&self, name: &str) -> (&str, bool) {
        match &self.emission(name).status {
            EmissionStatus::Emitted { rust, caller_unsafe, .. } => (rust, *caller_unsafe),
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

    /// Compile and run a program against C definitions of the fixture's functions, which
    /// print when an assert-enabled build calls them.
    fn run(&self, main: &str) -> (std::process::ExitStatus, String, String) {
        let arguments = self.arguments.iter().map(String::as_str).collect::<Vec<_>>();
        let c = format!(
            "{}\n#include <stdio.h>\n#include <stdlib.h>\nvoid ExceptionalCondition(const char *c, const char *f, int l) {{ abort(); }}\nint *PointerVariable = 0;\n#ifdef USE_ASSERT_CHECKING\nvoid AssertTransactionIdInAllowableRange(unsigned int xid) {{ printf(\"called range\\t%u\\n\", xid); fflush(stdout); }}\nvoid SlruPagePrecedesUnitTests(const int *ctl, int per_page) {{ printf(\"called slru\\t%d\\t%d\\n\", *ctl, per_page); fflush(stdout); }}\nvoid AssertCouldGetRelation(void) {{ printf(\"called relation\\n\"); fflush(stdout); }}\nvoid VerifyElement(const int *elements, int index) {{ printf(\"called element\\t%d\\n\", elements[index]); fflush(stdout); }}\nvoid VerifyValue(int value) {{ printf(\"called value\\t%d\\n\", value); fflush(stdout); }}\nint AssertOnlyFunction(int value) {{ return 100; }}\nint AssertOnlyVariable = 0;\n#else\nint ReleaseOnlyFunction(int value) {{ return value; }}\n#endif\n{C_BODIES}",
            self.c_support
        );
        rust_oracle::run_rust_linked_status(
            &self.program(main),
            &self.compiler,
            &header(),
            &c,
            &arguments,
        )
    }
}

/// What this flavor's C compiler makes of each inline function whose definition differs
/// between the flavors, in the order and form in which `MAIN` reports its translations.
const C_BODIES: &str = r#"
void c_bodies(void)
{
	int elements[] = {10, 20, 30};
	printf("element\t%d\n", *CheckedElement(elements, 1));
	printf("flavored\t%d\n", FlavoredValue(1));
	printf("macro\t%d\n", ConditionalMacro(1));
	printf("braces\t%d\t%d\n", ConditionalBraces(1), ConditionalBraces(-1));
	printf("operand\t%d\n", ConditionalOperand(1));
	printf("branch\t%d\t%d\n", ConditionalBranch(1), ConditionalBranch(-1));
	printf("local\t%d\n", ConditionalLocal(1));
	fflush(stdout);
}
"#;

/// Uses every generated macro, counting how often each operand is evaluated, and ends with
/// an assertion that fails.
const MAIN: &str = r#"
use std::sync::atomic::{AtomicUsize, Ordering};

/// Operand evaluations since the last report.
static EVALUATIONS: AtomicUsize = AtomicUsize::new(0);

/// Count one evaluation of an operand.
fn counted(value: i32) -> i32 {
    EVALUATIONS.fetch_add(1, Ordering::Relaxed);
    value
}

/// Report and reset the evaluation count.
fn evaluations() -> usize {
    EVALUATIONS.swap(0, Ordering::Relaxed)
}

unsafe extern "C" {
    /// Report what C makes of the inline functions whose definitions differ.
    fn c_bodies();
}

/// `Assert` is a C statement, so it yields `()` and can end a function.
fn assert_statement(value: i32) {
    Assert!(counted(value))
}

fn main() {
    assert_statement(1);
    let () = Assert!(counted(1));
    println!("assert\t{}", evaluations());
    AssertPositive!(counted(2));
    println!("positive\t{}", evaluations());
    let half: i32 = CHECKED_HALF!(counted(4)).get();
    println!("half\t{half}\t{}", evaluations());
    let trapped: bool = TrapMacro!(counted(0)).get();
    println!("trap\t{trapped}\t{}", evaluations());
    let () = AssertTransactionIdInAllowableRange!(counted(7) as u32).get();
    println!("range\t{}", evaluations());
    let () = AssertCouldGetRelation!().get();
    let object = 3_i32;
    let mut counter = 0_i32;
    // SAFETY: both pointers address live locals of the pointee type, and the fixture's
    // functions only read through them.
    unsafe {
        ValidateObject!(&raw const object);
        SlruPagePrecedesUnitTests!(&raw const object, counted(2));
        println!("slru\t{}", evaluations());
        COUNT_OR_ABORT!(&raw mut counter);
        println!("body\t{}", ConditionalBody!(1_i32).get());
        println!("checked\t{}\t{}", USES_CHECKED!(counted(5)).get(), evaluations());
    }
    println!("counter\t{counter}");
    let elements = [10_i32, 20, 30];
    // SAFETY: `elements` holds the elements read, the fixture's functions only read
    // through the pointer, and `c_bodies` reads its own array.
    unsafe {
        println!("element\t{}", *CheckedElement!(elements.as_ptr(), counted(1)).get());
        println!("flavored\t{}", FlavoredValue!(1_i32).get());
        println!("macro\t{}", ConditionalMacro!(1_i32).get());
        println!("braces\t{}\t{}", ConditionalBraces!(1_i32).get(), ConditionalBraces!(-1_i32).get());
        println!("operand\t{}", ConditionalOperand!(1_i32).get());
        println!("branch\t{}\t{}", ConditionalBranch!(1_i32).get(), ConditionalBranch!(-1_i32).get());
        println!("local\t{}", ConditionalLocal!(1_i32).get());
        println!("bodies\t{}", evaluations());
        c_bodies();
    }
    println!("sizes\t{}\t{}", ASSERTED_SIZE!().get(), PLAIN_SIZE!().get());
    if std::hint::black_box(false) {
        let () = flavor_unreachable!().get();
    }
    println!("before");
    Assert!(counted(0) != 0);
    println!("after\t{}", evaluations());
}
"#;

#[test]
fn both_build_flavors_generate_the_same_macros() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let asserted = Flavor::generate(&scanner, true);
    let released = Flavor::generate(&scanner, false);

    // Neither flavor selects or binds a name that only one of them declares, and neither
    // binds a function that only an assert-enabled build defines.
    assert_eq!(asserted.names, released.names);
    assert_eq!(
        asserted.names,
        [
            "ASSERTED_SIZE",
            "Assert",
            "AssertCouldGetRelation",
            "AssertMacro",
            "AssertOrClear",
            "AssertPositive",
            "AssertTransactionIdInAllowableRange",
            "CHECKED",
            "CHECKED_HALF",
            "CHECKED_TAG",
            "COUNT_OR_ABORT",
            "CheckedElement",
            "ConditionalBody",
            "ConditionalBraces",
            "ConditionalBranch",
            "ConditionalLocal",
            "ConditionalMacro",
            "ConditionalOperand",
            "FlavoredValue",
            "NEEDS_ASSERT_ONLY",
            "PLAIN_SIZE",
            "SlruPagePrecedesUnitTests",
            "TrapMacro",
            "USES_CHECKED",
            "USES_NEEDS_ASSERT_ONLY",
            "VERIFIED_ELEMENT",
            "ValidateObject",
            "flavor_unreachable",
        ]
    );
    for name in [
        "ASSERT_ONLY_MACRO",
        "AssertOnlyFunction",
        "AssertOnlyVariable",
        "AssertOrClear",
        "CheckedImpl",
        "AssertTransactionIdInAllowableRange",
        "SlruPagePrecedesUnitTests",
        "VerifyElement",
        "VerifyValue",
    ] {
        assert!(asserted.hidden.contains(name), "{name}: {:?}", asserted.hidden);
    }
    assert!(!asserted.hidden.contains("AssertCouldGetRelation"));
    for name in ["RELEASE_ONLY_MACRO", "ReleaseOnlyFunction"] {
        assert!(released.hidden.contains(name), "{name}: {:?}", released.hidden);
    }
    for flavor in [&asserted, &released] {
        assert!(!flavor.bindings.contains("AssertOnlyVariable"), "{}", flavor.bindings);
    }

    // Every root has one outcome, one need for `unsafe` and one Rust definition.
    for name in &asserted.names {
        match (&asserted.emission(name).status, &released.emission(name).status) {
            (
                EmissionStatus::Emitted { rust, caller_unsafe, .. },
                EmissionStatus::Emitted { rust: other, caller_unsafe: other_unsafe, .. },
            ) => {
                assert_eq!(caller_unsafe, other_unsafe, "{name}");
                assert_eq!(rust, other, "{name}");
            }
            (EmissionStatus::Skipped { reason }, EmissionStatus::Skipped { reason: other }) => {
                assert_eq!((reason.code, &reason.message), (other.code, &other.message), "{name}")
            }
            (asserted, released) => {
                panic!("{name}: {asserted:?} with assertions, {released:?} without")
            }
        }
    }
    assert_ne!(asserted.support, released.support);
    assert!(asserted.support.contains("macro_rules! __pgrx_c_assert_checking"));

    // A macro whose assert-enabled definition needs an external function that a release
    // build lacks is generated by neither flavor, and neither is a macro that expands it.
    for name in ["NEEDS_ASSERT_ONLY", "USES_NEEDS_ASSERT_ONLY"] {
        let EmissionStatus::Skipped { reason } = &asserted.emission(name).status else {
            panic!("{name} was emitted");
        };
        assert_eq!(reason.code, SkipReasonCode::BuildFlavor, "{name}: {reason:?}");
        assert!(reason.message.contains("AssertOnlyFunction"), "{name}: {reason:?}");
    }
    // A static inline function that only the assert-enabled headers define is bound by
    // neither flavor. A release build compiles its definition, so both call it.
    for name in ["CHECKED", "USES_CHECKED"] {
        assert!(asserted.emitted(name).0.contains("Inline_CheckedImpl"), "{name}");
    }
    assert!(released.c_support.contains("CheckedImpl(int tag, int value)"));
    assert!(!asserted.c_support.contains("CheckedImpl(int tag, int value)"));
    // Checks on the caller's values need no `unsafe`. A check that reads through a pointer
    // does, as does a call to a function that takes one.
    for name in [
        "Assert",
        "AssertMacro",
        "AssertPositive",
        "CHECKED_HALF",
        "TrapMacro",
        "flavor_unreachable",
        "AssertTransactionIdInAllowableRange",
        "AssertCouldGetRelation",
        "ASSERTED_SIZE",
        "PLAIN_SIZE",
    ] {
        assert!(!asserted.emitted(name).1, "{name} needs no unsafe");
    }
    // The operand of a `sizeof` sits in generated `unsafe` where the assert-enabled
    // definition holds an unsafe operation, in a release build too, whose own definition of
    // `TrapMacro` holds none. An operand with no such operation gets no `unsafe` block.
    let unevaluated = "SAFETY: C does not evaluate this operand";
    for flavor in [&asserted, &released] {
        assert!(flavor.emitted("ASSERTED_SIZE").0.contains(unevaluated));
        assert!(!flavor.emitted("PLAIN_SIZE").0.contains(unevaluated));
    }
    for name in [
        "ValidateObject",
        "SlruPagePrecedesUnitTests",
        "COUNT_OR_ABORT",
        "ConditionalBody",
        "CHECKED",
    ] {
        assert!(asserted.emitted(name).1, "{name} needs unsafe");
    }
    // A static inline function whose definition is shared is translated with its checks.
    let (validate, _) = asserted.emitted("ValidateObject");
    assert!(validate.contains("assertion_failure") && !validate.contains("Inline_ValidateObject"));
    // So is one whose definition differs by whole statements that name what both flavors
    // can: the constant selects the statements of the installation's flavor. A call to a
    // function that only an assert-enabled build declares is one such statement.
    let translated = "Translates the original function definition";
    let checking = "$crate::__pgrx_c_assert_checking!()";
    for flavor in [&asserted, &released] {
        let (element, caller_unsafe) = flavor.emitted("CheckedElement");
        assert!(element.contains(translated) && caller_unsafe, "{element}");
        assert!(element.contains(&format!("if {checking} {{")), "{element}");
        assert!(element.contains("Inline_VerifyElement"), "{element}");
        assert!(!element.contains("Inline_CheckedElement"), "{element}");
        let (flavored, caller_unsafe) = flavor.emitted("FlavoredValue");
        assert!(flavored.contains(translated) && !caller_unsafe, "{flavored}");
        assert!(flavored.contains(&format!("if {checking} {{")), "{flavored}");
        assert!(flavored.contains("} else {"), "{flavored}");
        // A macro whose assert-enabled definition makes such a call is not shared: its
        // release definition is other text, which could do work the call leaves out.
        let EmissionStatus::Skipped { reason } = &flavor.emission("VERIFIED_ELEMENT").status else {
            panic!("VERIFIED_ELEMENT was emitted");
        };
        assert_eq!(reason.code, SkipReasonCode::BuildFlavor, "{reason:?}");
        assert!(reason.message.contains("VerifyElement"), "{reason:?}");
        // Nor does a call stand for a release macro that does work.
        let EmissionStatus::Skipped { reason } = &flavor.emission("AssertOrClear").status else {
            panic!("AssertOrClear was emitted");
        };
        assert_eq!(reason.code, SkipReasonCode::BuildFlavor, "{reason:?}");
        assert!(reason.message.contains("does work"), "{reason:?}");
    }
    assert!(!released.c_support.contains("VerifyElement"), "{}", released.c_support);
    // Any other difference leaves a call to the function, in both flavors: a statement
    // that needs a value or a macro only one flavor has, tokens that are no statements of
    // their own, a statement that is the branch of a conditional, and a local that later
    // statements use.
    for name in [
        "ConditionalBody",
        "ConditionalMacro",
        "ConditionalBraces",
        "ConditionalOperand",
        "ConditionalBranch",
        "ConditionalLocal",
    ] {
        for flavor in [&asserted, &released] {
            let (rust, _) = flavor.emitted(name);
            assert!(rust.contains(&format!("Inline_{name}")), "{name}: {rust}");
            assert!(!rust.contains(translated) && !rust.contains(checking), "{name}: {rust}");
        }
    }

    // With assertions every check runs, the functions are called, and the failed assertion
    // stops the process where it is.
    let (status, stdout, stderr) = asserted.run(MAIN);
    assert!(!status.success(), "{stdout}\n{stderr}");
    // Each inline function whose definition differs does what this flavor's C does, first
    // as generated and then as compiled. The checks that only this flavor has are made.
    let bodies = "called element\t20\nelement\t20\nflavored\t101\nmacro\t3\nbraces\t2\t1\noperand\t11\ncalled value\t1\nbranch\t6\t4\nlocal\t2\n";
    assert_eq!(
        stdout,
        format!(
            "assert\t2\npositive\t1\nhalf\t2\t2\ntrap\ttrue\t1\ncalled range\t7\nrange\t1\ncalled relation\ncalled slru\t3\t2\nslru\t1\nbody\t102\nchecked\t6\t1\ncounter\t1\n{bodies}bodies\t1\n{bodies}sizes\t1\t8\nbefore\n"
        )
    );
    let trap = stderr
        .lines()
        .find_map(|line| line.strip_prefix("TRAP: failed Assert(\"counted(0) != 0\"), File: \""))
        .unwrap_or_else(|| panic!("no report of the failed assertion:\n{stderr}"));
    assert!(trap.contains("\", Line: "), "{trap}");

    // Without assertions the same program evaluates no check and calls no assertion
    // function, and the other stop still counts. The inline functions do what release C
    // does, and an operand of a check that is not made is still evaluated once.
    let (status, stdout, stderr) = released.run(MAIN);
    assert!(status.success(), "{stdout}\n{stderr}");
    let bodies = "element\t20\nflavored\t201\nmacro\t2\nbraces\t4\t-1\noperand\t1\nbranch\t6\t-1\nlocal\t3\n";
    assert_eq!(
        stdout,
        format!(
            "assert\t0\npositive\t0\nhalf\t2\t1\ntrap\ttrue\t0\nrange\t0\nslru\t0\nbody\t2\nchecked\t6\t1\ncounter\t1\n{bodies}bodies\t1\n{bodies}sizes\t1\t8\nbefore\nafter\t0\n"
        )
    );

    // The helper's own assertion runs in the function PostgreSQL compiled with assertions,
    // and does nothing in the copy a release build compiles.
    let tagged = "fn main() {\n    // SAFETY: the helper reads only its arguments.\n    println!(\"tagged\\t{}\", unsafe { CHECKED_TAG!(2_i32, 7_i32) }.get());\n}\n";
    let (status, stdout, stderr) = asserted.run(tagged);
    assert!(!status.success() && stdout.is_empty(), "{stdout}\n{stderr}");
    let (status, stdout, stderr) = released.run(tagged);
    assert!(status.success(), "{stdout}\n{stderr}");
    assert_eq!(stdout, "tagged\t7\n");

    // Both flavors reject the same safe use of a check that reads through a pointer.
    let unsafe_use = "fn main() { let object = 3_i32; ValidateObject!(&raw const object); }";
    let rejections =
        [&asserted, &released].map(|flavor| rust_oracle::reject_rust(&flavor.program(unsafe_use)));
    assert!(rejections[0].contains("E0133"), "{}", rejections[0]);
    assert_eq!(
        diagnostics(&rejections[0]),
        diagnostics(&rejections[1]),
        "both flavors report the same errors"
    );
}

/// meson writes an active `#undef USE_ASSERT_CHECKING` into the `pg_config.h` of a build
/// without assertions, which undoes a command-line definition. Such headers still show
/// their assert-enabled flavor, so a release build takes its definitions from it.
#[test]
fn headers_that_undefine_assertions_still_share_assert_enabled_definitions() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let directory = tempfile::tempdir().unwrap();
    // The definition shares its file with the line that is blanked to read it, so its text
    // must still be found at the offsets the compiler reports.
    let header = directory.path().join("main.h");
    std::fs::write(
        &header,
        "#undef USE_ASSERT_CHECKING\n#ifdef USE_ASSERT_CHECKING\n#define FLAVORED(value) ((value) + 1)\n#else\n#define FLAVORED(value) (value)\n#endif\n",
    )
    .unwrap();
    let scanner = MacroScanner::new().expect("libclang must be available");
    let frontend = inspect(&scanner, &header, &oracle::native_arguments(), None).unwrap();
    assert!(!frontend.assert_enabled());
    let session = AnalysisSession::prepare(&scanner, &frontend, &["FLAVORED"]).unwrap();
    let EmissionStatus::Emitted { rust, .. } = pgrx_c_macros::emit(&session, "FLAVORED").status
    else {
        panic!("FLAVORED must be emitted");
    };
    assert!(rust.contains("( ( value ) + 1 )"), "{rust}");
}

/// Both flavors must prove a body that they merge. Only the inspected flavor's proof can
/// use the driver's report of `#undef` directives, which admits a macro that a header
/// defines again each time it is included, as Clang's `<stddef.h>` defines `NULL` under
/// glibc. A definition that differs between the flavors and needs that report is therefore
/// called in both, while a definition both flavors share is still translated.
#[test]
fn a_body_that_needs_the_undefinition_report_is_not_merged() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let directory = tempfile::tempdir().unwrap();
    std::fs::write(directory.path().join("again.h"), "#undef AGAIN\n#define AGAIN 7\n").unwrap();
    let scanner = MacroScanner::new().expect("libclang must be available");
    // What each flavor decides about `differs`, and whether it retains a body for `shared`.
    let decide = |again: &str| {
        let header = directory.path().join("main.h");
        std::fs::write(
            &header,
            format!(
                "#include \"again.h\"\nextern void check(int value);\nstatic inline int differs(int value)\n{{\n#ifdef USE_ASSERT_CHECKING\n\tcheck(value);\n#endif\n\treturn value + AGAIN;\n}}\nstatic inline int shared(int value)\n{{\n\treturn value + AGAIN;\n}}\n{again}"
            ),
        )
        .unwrap();
        [false, true].map(|assert_enabled| {
            let mut arguments = oracle::native_arguments();
            if assert_enabled {
                arguments.push("-DUSE_ASSERT_CHECKING=1".into());
            }
            let frontend = inspect(&scanner, &header, &arguments, None).unwrap();
            assert_eq!(frontend.assert_enabled(), assert_enabled);
            let flavors = serde_json::to_value(frontend.flavors()).unwrap();
            let translated = frontend.declarations().function_signatures["shared"]
                .definition
                .as_ref()
                .is_some_and(|definition| definition.body.is_some());
            (flavors["merged"].clone(), flavors["native_only"].clone(), translated)
        })
    };
    let merged = (serde_json::json!(["differs"]), serde_json::json!([]), true);
    assert_eq!(decide(""), [merged.clone(), merged]);
    let called = (serde_json::json!([]), serde_json::json!(["differs"]), true);
    assert_eq!(decide("#include \"again.h\"\n"), [called.clone(), called]);
}

/// A compiler's error lines, without the source lines and locations that surround them.
fn diagnostics(stderr: &str) -> Vec<&str> {
    stderr.lines().filter(|line| line.starts_with("error")).collect()
}
