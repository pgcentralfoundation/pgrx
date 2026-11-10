//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! A GNU statement expression, `({ statements; value; })`, is an expression whose
//! statements run in order in their own scope. PostgreSQL 15 and 16 define `newNode` as
//! one, and as a comma expression through a global for compilers without them; `makeNode`
//! pastes and stringifies its type operand around either. The fixture models both with its
//! own names, so nothing here can depend on PostgreSQL's.
//!
//! Generated Rust is compared with the original C, in both build flavors, which must
//! generate the same macros. A check behind such a macro runs only with assertions.

#![cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos")
))]

/// Reconcile fixture bindings using production's actual storage collector.
#[path = "../../pgrx-bindgen/src/build/binding_symbols.rs"]
mod binding_symbols;
/// Run the original C and locate the native SDK arguments.
#[path = "support/oracle.rs"]
#[allow(dead_code)]
mod oracle;
/// Compile generated Rust consumers with bounded tool execution.
#[path = "support/rust_oracle.rs"]
#[allow(dead_code)]
mod rust_oracle;

use pgrx_c_macros::{
    AnalysisSession, DocumentedResult, EmissionStatus, MacroEmission, MacroScanner, SkipReasonCode,
    documentation_shell_support, generate_with_bindings, inspect, postgres_function_macro_names,
    postgres_inline_function_names,
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
    fixtures().join("statement_expressions.h")
}

/// C definitions of what the fixture declares. A failed check writes PostgreSQL's `TRAP:`
/// line and aborts.
const NATIVE: &str = r#"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
unsigned int shape_trace;
Shape *shapeHolder;
void ExceptionalCondition(const char *condition, const char *file, int line) {
    fprintf(stderr, "TRAP: failed Assert(\"%s\")\n", condition);
    fflush(stderr);
    abort();
}
unsigned int shape_record(unsigned int digit, unsigned int value) {
    shape_trace = shape_trace * 10 + digit;
    return value;
}
static void *shape_zeroed(unsigned long size, unsigned int digit) {
    void *shape = calloc(1, size < 16 ? 16 : size);
    if (shape == 0)
        abort();
    shape_record(digit, 0);
    return shape;
}
void shape_note(const char *text, unsigned long size) {
    shape_trace = shape_trace * 100 + (unsigned int) strlen(text) * 10 + (unsigned int) size / 4;
}
void *shape_alloc(unsigned long size) { return shape_zeroed(size, 8); }
void *shape_alloc_words(unsigned long size) { return shape_zeroed(size, 9); }
"#;

/// What one build flavor of the fixture generates.
struct Flavor {
    /// The compiler arguments that select the flavor.
    arguments: Vec<String>,
    /// The compiler that inspected it.
    compiler: PathBuf,
    /// The fixture's function-like macros and inline functions.
    names: Vec<String>,
    /// The bindings.
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
        arguments.push("-std=gnu17".into());
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
        let bindings = frontend
            .flavor_specific_names()
            .fold(bindgen::Builder::default(), |builder, name| {
                builder.blocklist_item(format!("^{name}$"))
            })
            .rust_target(bindgen::RustTarget::stable(85, 0).unwrap())
            .rust_edition(bindgen::RustEdition::Edition2024)
            .header(header().to_str().unwrap())
            .clang_args(&frontend.profile().arguments)
            .allowlist_type("Shape.*|Circle|Square|Triangle|Small")
            .allowlist_function("shape_.*")
            .allowlist_var("shape.*")
            .rustified_non_exhaustive_enum("ShapeKind")
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
            names,
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

    /// The runtime, bindings, support and generated macros, before a program's own items.
    fn prelude(&self) -> String {
        let support = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../pgrx-pg-sys/src/c_macros/support.rs")
            .canonicalize()
            .unwrap();
        let mut rust = format!(
            "#![deny(unsafe_op_in_unsafe_fn)]\n#![allow(non_snake_case,non_camel_case_types,non_upper_case_globals,unused_parens,unused_braces,dead_code)]\n#[path={support:?}] pub mod __pgrx_c_macros;\npub mod ffi {{ pub unsafe fn pg_guard_ffi_boundary<T, F: FnOnce() -> T>(call: F) -> T {{ call() }} }}\n{}\n{}\n",
            self.bindings, self.support
        );
        for emission in &self.macros {
            if let EmissionStatus::Emitted { rust: definition, .. } = &emission.status {
                rust.push_str(definition);
            }
        }
        rust
    }

    /// The C definitions a program links: the fixture's functions and the generated support.
    fn native(&self) -> String {
        format!("{NATIVE}\n{}", self.c_support)
    }

    /// Compile and run a Rust program that uses the generated macros.
    fn run(&self, main: &str) -> (std::process::ExitStatus, String, String) {
        let arguments = self.arguments.iter().map(String::as_str).collect::<Vec<_>>();
        rust_oracle::run_rust_linked_status(
            &format!("{}{main}", self.prelude()),
            &self.compiler,
            &header(),
            &self.native(),
            &arguments,
        )
    }

    /// Compile and run a C program that uses the fixture's own macros.
    fn run_c(&self, main: &str) -> String {
        let arguments = self.arguments.iter().map(String::as_str).collect::<Vec<_>>();
        oracle::run_c(&self.compiler, &header(), &format!("{NATIVE}\n{main}"), &arguments, true)
    }
}

/// The macros and the function that every flavor emits, each with whether it needs the
/// caller's `unsafe`.
const EMITTED: &[(&str, bool)] = &[
    ("blockNewShape", true),
    ("blockMakeShape", true),
    ("commaNewShape", true),
    ("commaMakeShape", true),
    ("NOTE_SIZE", true),
    // Locals are the macro's own, so reading and writing them needs nothing of the caller.
    ("BLOCK_MAX", false),
    ("BLOCK_ORDER", true),
    ("BLOCK_IF", true),
    ("BLOCK_NESTED", false),
    ("BLOCK_LAZY", true),
    ("BLOCK_SIZE", false),
    ("BLOCK_STORE", true),
    ("block_next", false),
];

/// The statement expressions that stay refused, each with its skip code and part of its
/// reason.
const REFUSED: &[(&str, SkipReasonCode, &str)] = &[
    ("BLOCK_VOID", SkipReasonCode::Statement, "must end with the expression that is its value"),
    (
        "BLOCK_EMPTY_LAST",
        SkipReasonCode::Statement,
        "must end with the expression that is its value",
    ),
    ("BLOCK_RETURN", SkipReasonCode::Statement, "a return inside a statement expression"),
    ("BLOCK_UNINITIALIZED", SkipReasonCode::Statement, "read before definite initialization"),
    ("BLOCK_SHADOW", SkipReasonCode::Statement, "cannot shadow macro formals or other locals"),
    ("BLOCK_ESCAPE", SkipReasonCode::Statement, "outside its C block scope"),
    ("BLOCK_PLACE", SkipReasonCode::UnsupportedType, "not a place"),
];

/// The original C. Each line it prints, the Rust program prints too.
const C_MAIN: &str = r#"
int main(void) {
    Circle *circle = blockMakeShape(Circle);
    printf("block\t%d\t%d\t%u\n", (int) circle->kind, circle->radius, shape_trace);
    Triangle *triangle = blockMakeShape(Triangle);
    printf("block\t%d\t%d\t%u\n", (int) triangle->kind, triangle->height, shape_trace);
    Square *square = commaMakeShape(Square);
    printf("comma\t%d\t%ld\t%u\t%d\n", (int) square->kind, square->area, shape_trace,
           (void *) shapeHolder == (void *) square);
    triangle = commaMakeShape(Triangle);
    printf("comma\t%d\t%u\t%d\n", (int) triangle->kind, shape_trace,
           (void *) shapeHolder == (void *) triangle);
    Shape *shape = blockNewShape(sizeof(Square), KIND_Shape);
    printf("new\t%d\t%u\n", (int) shape->kind, shape_trace);
    shape = commaNewShape(sizeof(Square), KIND_Shape);
    printf("new\t%d\t%u\n", (int) shape->kind, shape_trace);
    shape_trace = 0;
    printf("max\t%d\t%d\t%u\n", BLOCK_MAX(shape_record(5, 3), shape_record(6, 7)),
           BLOCK_MAX(-1, -2), shape_trace);
    shape_trace = 0;
    printf("order\t%u", BLOCK_ORDER(shape_record(4, 40)));
    printf("\t%u\n", shape_trace);
    square->side = 12;
    printf("if\t%d\t%d\n", BLOCK_IF(square), BLOCK_IF((Square *) 0));
    printf("nested\t%d\n", BLOCK_NESTED(5));
    shape_trace = 0;
    printf("lazy\t%u\t%u", BLOCK_LAZY(0, 9), BLOCK_LAZY(1, 9));
    printf("\t%u\n", shape_trace);
    shape_trace = 0;
    printf("size\t%u\t%u\n", (unsigned int) BLOCK_SIZE(shape_record(7, 1)), shape_trace);
    BLOCK_STORE(square, 21);
    printf("store\t%d\n", square->side);
    printf("inline\t%d\n", block_next(41));
    shape_trace = 0;
    NOTE_SIZE(Circle);
    NOTE_SIZE(Triangle);
    printf("note\t%u\n", shape_trace);
    return 0;
}
"#;

/// The same program with the generated macros.
const RUST_MAIN: &str = r#"
fn trace() -> u32 {
    // SAFETY: this program has one thread, and the fixture's functions only add digits.
    unsafe { shape_trace }
}

fn record(digit: u32, value: u32) -> u32 {
    // SAFETY: as in `trace`.
    unsafe { shape_record(digit, value) }
}

fn main() {
    let square: *mut Square;
    // SAFETY: each allocator returns zeroed storage of at least 16 bytes, which holds any
    // of the fixture's structures, and every pointer below addresses such storage. The
    // program has one thread, which is the only user of the fixture's globals.
    unsafe {
        let circle: *mut Circle = blockMakeShape!(Circle).get();
        println!("block\t{}\t{}\t{}", (*circle).kind as i32, (*circle).radius, trace());
        let mut triangle: *mut Triangle = blockMakeShape!(Triangle).get();
        println!("block\t{}\t{}\t{}", (*triangle).kind as i32, (*triangle).height, trace());
        square = commaMakeShape!(Square).get();
        println!(
            "comma\t{}\t{}\t{}\t{}",
            (*square).kind as i32,
            (*square).area,
            trace(),
            i32::from(shapeHolder.cast::<Square>() == square)
        );
        triangle = commaMakeShape!(Triangle).get();
        println!(
            "comma\t{}\t{}\t{}",
            (*triangle).kind as i32,
            trace(),
            i32::from(shapeHolder.cast::<Triangle>() == triangle)
        );
        let mut shape: *mut Shape =
            blockNewShape!(core::mem::size_of::<Square>(), ShapeKind::KIND_Shape).get();
        println!("new\t{}\t{}", (*shape).kind as i32, trace());
        shape = commaNewShape!(core::mem::size_of::<Square>(), ShapeKind::KIND_Shape).get();
        println!("new\t{}\t{}", (*shape).kind as i32, trace());
        shape_trace = 0;
        let larger = BLOCK_MAX!(shape_record(5, 3), shape_record(6, 7)).get();
        println!("max\t{}\t{}\t{}", larger, BLOCK_MAX!(-1, -2).get(), trace());
        shape_trace = 0;
        print!("order\t{}", BLOCK_ORDER!(shape_record(4, 40)).get());
        println!("\t{}", trace());
        (*square).side = 12;
        println!(
            "if\t{}\t{}",
            BLOCK_IF!(square).get(),
            BLOCK_IF!(core::ptr::null_mut::<Square>()).get()
        );
        println!("nested\t{}", BLOCK_NESTED!(5).get());
        shape_trace = 0;
        print!("lazy\t{}\t{}", BLOCK_LAZY!(0, 9).get(), BLOCK_LAZY!(1, 9).get());
        println!("\t{}", trace());
        shape_trace = 0;
    }
    // A size is never evaluated, so it needs no `unsafe` of its own.
    println!("size\t{}\t{}", BLOCK_SIZE!(record(7, 1)).get(), trace());
    // SAFETY: as above.
    unsafe {
        BLOCK_STORE!(square, 21);
        println!("store\t{}", (*square).side);
        println!("inline\t{}", block_next!(41).get());
        shape_trace = 0;
        NOTE_SIZE!(Circle);
        NOTE_SIZE!(Triangle);
        println!("note\t{}", trace());
    }
}
"#;

/// A structure too small for its kind fails the check in front of the store.
const SMALL_MAIN: &str = r#"
fn main() {
    // SAFETY: as in the program above; the allocators never return less than 16 bytes.
    unsafe {
        println!("before");
        let small: *mut Small = FORM!(Small).get();
        println!("after\t{}", (*small.cast::<Shape>()).kind as i32);
    }
}
"#;

#[test]
fn statement_expressions_run_their_statements_in_order_and_yield_their_last_value() {
    let scanner = MacroScanner::new().expect("libclang must be available");
    let asserted = Flavor::generate(&scanner, true);
    let released = Flavor::generate(&scanner, false);

    // Both flavors generate one Rust definition and one need for `unsafe` for each root,
    // or skip it for one reason.
    assert_eq!(asserted.names, released.names);
    for name in &asserted.names {
        match (&asserted.emission(name).status, &released.emission(name).status) {
            (
                EmissionStatus::Emitted { rust, caller_unsafe, documentation, .. },
                EmissionStatus::Emitted {
                    rust: other,
                    caller_unsafe: other_unsafe,
                    documentation: other_documentation,
                    ..
                },
            ) => {
                assert_eq!(caller_unsafe, other_unsafe, "{name}");
                assert_eq!(rust, other, "{name}");
                assert_eq!(documentation, other_documentation, "{name}");
            }
            (EmissionStatus::Skipped { reason }, EmissionStatus::Skipped { reason: other }) => {
                assert_eq!((reason.code, &reason.message), (other.code, &other.message), "{name}")
            }
            (asserted, released) => {
                panic!("{name}: {asserted:?} with assertions, {released:?} without")
            }
        }
    }
    for (name, caller_unsafe) in EMITTED {
        match &asserted.emission(name).status {
            EmissionStatus::Emitted { caller_unsafe: needs_unsafe, .. } => {
                assert_eq!(needs_unsafe, caller_unsafe, "{name}")
            }
            EmissionStatus::Skipped { reason } => panic!("{name}: {reason:?}"),
        }
    }
    for (name, code, reason) in REFUSED {
        let EmissionStatus::Skipped { reason: skipped } = &asserted.emission(name).status else {
            panic!("{name} was emitted");
        };
        assert_eq!(skipped.code, *code, "{name}: {skipped:?}");
        assert!(skipped.message.contains(reason), "{name}: {skipped:?}");
    }
    // The function's body is translated, statement expression included, not called.
    let EmissionStatus::Emitted { rust, .. } = &asserted.emission("block_next").status else {
        unreachable!("emitted above");
    };
    assert!(rust.contains("__pgrx_c_local") && !rust.contains("Inline_block_next"), "{rust}");

    // Rust and C agree in both flavors, and the flavors agree with each other: every size
    // here passes its check.
    let expected = asserted.run_c(C_MAIN);
    assert_eq!(expected.lines().count(), 15, "{expected}");
    // Each note recorded the length of its type's name and a quarter of the size.
    assert!(expected.ends_with("note\t6485\n"), "{expected}");
    assert_eq!(released.run_c(C_MAIN), expected);
    for flavor in [&asserted, &released] {
        let (status, stdout, stderr) = flavor.run(RUST_MAIN);
        assert!(status.success(), "{stdout}\n{stderr}");
        assert_eq!(stdout, expected);
    }

    // With assertions the check in front of the allocation stops the process, and its
    // text spells the type operand. Without them the same program stores the kind.
    for form in ["blockMakeShape", "commaMakeShape"] {
        let main = SMALL_MAIN.replace("FORM", form);
        let (status, stdout, stderr) = asserted.run(&main);
        assert!(!status.success(), "{form}: {stdout}\n{stderr}");
        assert_eq!(stdout, "before\n", "{form}");
        assert!(
            stderr.contains("TRAP: failed Assert(\"(sizeof(Small)) >= sizeof(Shape)\")"),
            "{form}: {stderr}"
        );
        let (status, stdout, stderr) = released.run(&main);
        assert!(status.success(), "{form}: {stdout}\n{stderr}");
        assert_eq!(stdout, "before\nafter\t6\n", "{form}");
        assert!(!stderr.contains("TRAP:"), "{form}: {stderr}");
    }

    // An operand that names one of the macro's locals would be captured in C and not in
    // Rust, so it does not compile. Neither does a use of the value as a place, or of a
    // macro that allocates and stores without `unsafe`. Both flavors report the same errors.
    for (invocation, error) in [
        ("let _a = 5; let _ = BLOCK_MAX!(_a, 1);", "C macro argument mentions local _a, _b"),
        ("let _ = BLOCK_MAX!(@__pgrx_c_place; 1, 2);", "not a place"),
        ("let _ = BLOCK_NESTED!(1, 2);", "invocation contract"),
        ("let _ = blockMakeShape!(Circle);", "E0133"),
        ("let _ = commaMakeShape!(Circle);", "E0133"),
        // The type operand is a type, and one whose name pastes into no enumerator fails.
        ("let _ = unsafe { blockMakeShape!(ShapeKind) };", "E0277"),
    ] {
        let rejections = [&asserted, &released].map(|flavor| {
            rust_oracle::reject_rust(&format!("{}fn main() {{ {invocation} }}", flavor.prelude()))
        });
        assert!(rejections[0].contains(error), "{invocation}: {}", rejections[0]);
        assert_eq!(diagnostics(&rejections[0]), diagnostics(&rejections[1]), "{invocation}");
    }

    // A documentation shell has the real macro's result type, from a witness that declares
    // the locals its value names and runs none of the statements.
    let mut shells = documentation_shell_support();
    for (name, _) in EMITTED {
        let EmissionStatus::Emitted { documentation, .. } = &asserted.emission(name).status else {
            unreachable!("emitted above");
        };
        match (*name, &documentation.result) {
            ("BLOCK_STORE", DocumentedResult::Unit { .. }) => {}
            (_, DocumentedResult::Value { witness }) if *name != "BLOCK_STORE" => {
                assert!(!witness.contains("shape_record"), "{name}: {witness}");
            }
            (_, result) => panic!("{name}: {result:?}"),
        }
        let mut shell = (**documentation).clone();
        shell.identifier = format!("__doc_{name}");
        shells.push_str(&shell.render_shell(None, "").unwrap());
    }
    let same = r#"
fn same<T>(_real: T, _shell: T) {}
fn compare(square: *mut Square) {
    // SAFETY: never run; the shells panic.
    unsafe {
        same(blockMakeShape!(Circle), __doc_blockMakeShape!(Circle));
        same(commaMakeShape!(Circle), __doc_commaMakeShape!(Circle));
        same(blockNewShape!(8usize, ShapeKind::KIND_Shape), __doc_blockNewShape!(8usize, ShapeKind::KIND_Shape));
        same(BLOCK_MAX!(1, 2), __doc_BLOCK_MAX!(1, 2));
        same(BLOCK_ORDER!(1u32), __doc_BLOCK_ORDER!(1u32));
        same(BLOCK_IF!(square), __doc_BLOCK_IF!(square));
        same(BLOCK_NESTED!(1), __doc_BLOCK_NESTED!(1));
        same(BLOCK_LAZY!(1, 2u32), __doc_BLOCK_LAZY!(1, 2u32));
        same(BLOCK_SIZE!(1), __doc_BLOCK_SIZE!(1));
        same(block_next!(1), __doc_block_next!(1));
    }
}
fn main() {}
"#;
    let (status, stdout, stderr) = asserted.run(&format!("{shells}{same}"));
    assert!(status.success(), "{stdout}\n{stderr}");
}

/// A compiler's error lines, without the source lines and locations that surround them.
fn diagnostics(stderr: &str) -> Vec<&str> {
    stderr.lines().filter(|line| line.starts_with("error")).collect()
}
