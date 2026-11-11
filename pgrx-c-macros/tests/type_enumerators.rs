//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! A macro that pastes a fixed prefix onto a type operand names an enumerator, as
//! PostgreSQL's `IsA`, `makeNode` and `castNode` paste `T_`. The fixture uses another
//! prefix and enum, so nothing here can depend on PostgreSQL's names.
//!
//! The generated macro takes the operand as a Rust type and reads the enumerator from a
//! sealed trait that only the bound structure types with an enumerator implement. Where
//! C gives one structure several names, the operand's spelling selects the name. Both
//! build flavors generate the same Rust, and a check behind such a macro runs only with
//! assertions.

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
    AnalysisSession, EmissionStatus, IntegerValue, MacroEmission, MacroScanner, SkipReasonCode,
    generate_with_bindings, inspect, postgres_function_macro_names,
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
    fixtures().join("type_enumerators.h")
}

/// What one build flavor of the fixture generates.
struct Flavor {
    /// The compiler arguments that select the flavor.
    arguments: Vec<String>,
    /// The compiler that inspected it.
    compiler: PathBuf,
    /// The fixture's function-like macros.
    names: Vec<String>,
    /// The bindings, which leave out `Unbound`.
    bindings: String,
    /// Each root's emission.
    macros: Vec<MacroEmission>,
    /// Each root's emission from bindings that hold a wrong value for `KIND_Square`.
    mismatched: Vec<MacroEmission>,
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
        let names = postgres_function_macro_names(&frontend, &fixtures()).unwrap();
        let session = AnalysisSession::prepare(scanner, &frontend, &names).expect("prepare roots");
        // `ShapeKind` is a Rust enum, as pgrx binds `NodeTag`, and `Paint` is constants in a
        // module, as it binds every other enum.
        let bindings = frontend
            .flavor_specific_names()
            .fold(bindgen::Builder::default(), |builder, name| {
                builder.blocklist_item(format!("^{name}$"))
            })
            .rust_target(bindgen::RustTarget::stable(85, 0).unwrap())
            .rust_edition(bindgen::RustEdition::Edition2024)
            .header(header().to_str().unwrap())
            .clang_args(&frontend.profile().arguments)
            .allowlist_type("Shape.*|Circle|Square|Segment|Plain|Twin.*|Masked.*|Paint")
            .default_enum_style(bindgen::EnumVariation::ModuleConsts)
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
        let mut wrong = catalog.clone();
        wrong.integer_constants.get_mut("KIND_Square").unwrap().value = IntegerValue::Unsigned(40);
        let mismatched = generate_with_bindings(&session, &names, &wrong).unwrap().macros;
        session.verify_inputs().expect("the inspected profile stayed unchanged");
        Self {
            arguments: frontend.profile().arguments.clone(),
            compiler: frontend.profile().compiler.executable.clone(),
            names,
            bindings,
            macros: generated.macros,
            mismatched,
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

    /// Compile and run a program against C definitions of the fixture's functions. A failed
    /// assertion writes PostgreSQL's `TRAP:` line and aborts.
    fn run(&self, main: &str) -> (std::process::ExitStatus, String, String) {
        let arguments = self.arguments.iter().map(String::as_str).collect::<Vec<_>>();
        let c = format!(
            "{}\n#include <stdio.h>\n#include <stdlib.h>\nvoid ExceptionalCondition(const char *condition, const char *file, int line) {{ fprintf(stderr, \"TRAP: failed Assert(\\\"%s\\\")\\n\", condition); fflush(stderr); abort(); }}\nShape *newShape(unsigned long size, ShapeKind kind) {{ Shape *shape = calloc(1, size); if (shape == 0) abort(); shape->kind = kind; return shape; }}\n",
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

/// Uses every generated macro with types that have an enumerator.
const MAIN: &str = r#"
/// A second Rust name of a bound type selects the same enumerator.
type Round = Circle;

/// A path through another module still ends in the C name.
mod names {
    pub use super::{Twin, TwinAlias};
}

/// A macro of the defining crate names a type through `$crate`.
macro_rules! alias_kind {
    () => {
        KindOf!($crate::TwinAlias)
    };
}

fn main() {
    println!(
        "named\t{}\t{}\t{}\t{}",
        KindOf!(Twin).get(),
        KindOf!(TwinAlias).get(),
        KindOf!(MaskedAlias).get(),
        BareKind!(TwinAlias,).get()
    );
    println!(
        "paths\t{}\t{}\t{}\t{}",
        KindOf!(self::Twin).get(),
        KindOf!(crate::names::TwinAlias).get(),
        KindThrough!(names::Twin).get(),
        alias_kind!().get()
    );
    println!(
        "twin size\t{}\t{}",
        KindAndSize!(Twin).get(),
        KindAndSize!(crate::TwinAlias).get()
    );
    println!("kind\t{}\t{}\t{}", KindOf!(Circle).get(), KindOf!(Square).get(), KindOf!(Shape).get());
    println!("alias\t{}\t{}", KindOf!(Round).get(), KindOf!(self::Circle).get());
    println!("incomplete\t{}\t{}", KindOf!(Segment).get(), BareKind!(Square).get());
    println!("through\t{}", KindThrough!(Square).get());
    println!("paint\t{}\t{}", PaintOf!(Circle).get(), PaintOf!(Square).get());
    println!("sized\t{}", KindAndSize!(Circle).get());
    // SAFETY: `newShape` returns zeroed storage of the requested size with its kind set, and
    // every pointer below addresses that storage or the live list that holds it.
    unsafe {
        let circle: *mut Circle = makeShape!(Circle).get();
        println!("made\t{}\t{}", shapeKind!(circle).get(), (*circle).radius);
        println!("is\t{}\t{}", IsShape!(circle, Circle).get(), IsShape!(circle, Square).get());
        let same: *mut Circle = castShape!(Circle, circle).get();
        println!("cast\t{}", same == circle);
        let null: *mut Square = castShape!(Square, core::ptr::null_mut::<Shape>()).get();
        println!("null\t{}", null.is_null());
        let list = ShapeList { first: circle.cast() };
        let first: *mut Circle = firstShape!(Circle, &raw const list).get();
        println!("first\t{}", first == circle);
        // `Twin` and `TwinAlias` name one structure, and each spelling selects its own
        // enumerator in every macro, as it does in C.
        let twin: *mut Twin = makeShape!(Twin).get();
        let alias: *mut Twin = makeShape!(self::TwinAlias).get();
        println!("twins\t{}\t{}", shapeKind!(twin).get(), shapeKind!(alias).get());
        println!(
            "is twin\t{}\t{}\t{}\t{}",
            IsShape!(twin, Twin).get(),
            IsShape!(twin, TwinAlias).get(),
            IsShape!(alias, crate::Twin,).get(),
            IsShape!(alias, names::TwinAlias).get()
        );
        let same: *mut TwinAlias = castShape!(TwinAlias, alias).get();
        let list = ShapeList { first: alias.cast() };
        let first: *mut Twin = firstShape!(TwinAlias, &raw const list).get();
        println!("twin cast\t{}\t{}", same == alias, first == alias);
        println!("before");
        let wrong: *mut Square = castShape!(Square, circle).get();
        println!("after\t{}", wrong.cast::<Circle>() == circle);
    }
}
"#;

/// The macros the fixture defines that every flavor emits: whether each needs the caller's
/// `unsafe`, and whether its documentation witness names the lookup. A witness that closes
/// an enclosing cast or comparison over the pasted name does not. Neither does a witness of
/// `KIND_`, whose enumerator can depend on the operand's spelling, which a shell cannot read.
const EMITTED: &[(&str, bool, bool)] = &[
    ("KindOf", false, false),
    ("BareKind", false, false),
    ("KindThrough", false, false),
    ("KindAndSize", false, false),
    ("PaintOf", false, true),
    ("IsShape", true, false),
    ("makeShape", true, false),
    ("castShape", true, false),
    ("firstShape", true, false),
];

/// The pastes that stay refused, each with its skip code and part of its reason.
const REFUSED: &[(&str, SkipReasonCode, &str)] = &[
    ("CppConcat", SkipReasonCode::TokenPaste, "depends on invocation operands"),
    ("SuffixOf", SkipReasonCode::TokenPaste, "depends on invocation operands"),
    ("ErasedKind", SkipReasonCode::TokenPaste, "does not survive in the expansion"),
    ("UnknownKind", SkipReasonCode::TokenPaste, "no structure type has a name that `NOPE_`"),
    ("KindAndValue", SkipReasonCode::UnsupportedType, "incompatible type and operand roles"),
    ("KindMember", SkipReasonCode::TokenPaste, "only as an expression that names an enumerator"),
];

#[test]
fn a_prefix_pasted_onto_a_type_operand_names_that_types_enumerator() {
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
    // Their supports define the same lookups and implement them for the same types.
    let lookups = |flavor: &Flavor| {
        flavor
            .support
            .lines()
            .filter(|line| line.contains("TypeEnumerator_") || line.contains("Seal_"))
            .map(str::to_owned)
            .collect::<Vec<_>>()
    };
    assert_eq!(lookups(&asserted), lookups(&released));

    // A lookup alone is a constant. A macro that reads through the caller's pointer or
    // calls a function needs `unsafe`, with or without assertions.
    for (name, caller_unsafe, witnessed) in EMITTED {
        let (rust, needs_unsafe) = asserted.emitted(name);
        assert_eq!(needs_unsafe, *caller_unsafe, "{name}");
        let lookup =
            if *name == "PaintOf" { "TypeEnumerator_PAINT_" } else { "TypeEnumerator_KIND_" };
        // The formatter may break the line anywhere between tokens.
        assert!(
            rust.split_whitespace()
                .collect::<String>()
                .contains(&format!("as$crate::__pgrx_c_generated::{lookup}>::VALUE")),
            "{name}: {rust}"
        );
        // The operand is a type in every form, and the documentation says which types.
        assert!(rust.contains("$_type_:ty"), "{name}: {rust}");
        let enumeration = if *name == "PaintOf" { "enum Paint" } else { "enum ShapeKind" };
        assert!(
            rust.contains(&format!("to name an enumerator of `{enumeration}`")),
            "{name}: {rust}"
        );
        // A documentation shell takes a type too, and a witness that renders the lookup
        // keeps the bound.
        let EmissionStatus::Emitted { documentation, .. } = &asserted.emission(name).status else {
            unreachable!("emitted above");
        };
        let pgrx_c_macros::DocumentedResult::Value { witness } = &documentation.result else {
            panic!("{name}: {:?}", documentation.result);
        };
        assert_eq!(witness.contains(lookup), *witnessed, "{name}: {witness}");
        assert!(documentation.forms.iter().all(|form| form.contains("$_type_:ty")), "{name}");
    }
    for (name, code, reason) in REFUSED {
        let EmissionStatus::Skipped { reason: skipped } = &asserted.emission(name).status else {
            panic!("{name} was emitted");
        };
        assert_eq!(skipped.code, *code, "{name}: {skipped:?}");
        assert!(skipped.message.contains(reason), "{name}: {skipped:?}");
    }

    // Each bound structure type with a bound enumerator implements the sealed lookup. A
    // type without an enumerator, one the bindings leave out, and one that two C names give
    // different enumerators implement none, even when a macro hides one of those names.
    let implementations = |lookup: &str| {
        asserted
            .support
            .lines()
            .filter_map(|line| line.strip_prefix(&format!("impl {lookup} for crate::")))
            .map(|rest| rest.split_whitespace().next().unwrap())
            .collect::<Vec<_>>()
    };
    assert_eq!(implementations("TypeEnumerator_KIND_"), ["Circle", "Segment", "Shape", "Square"]);
    assert_eq!(implementations("TypeEnumerator_PAINT_"), ["Circle", "Square"]);
    // Each C name of a shared structure whose enumerator no macro hides has a carrier,
    // which implements the lookup for that structure only. `PAINT_` has none.
    let carriers = asserted
        .support
        .lines()
        .filter_map(|line| {
            line.strip_prefix("impl TypeEnumerator_KIND_ for type_names_KIND_::")
                .and_then(|rest| rest.split_once(" {"))
                .map(|(carrier, _)| carrier)
        })
        .collect::<Vec<_>>();
    assert_eq!(
        carriers,
        ["MaskedAlias<crate::Masked>", "Twin<crate::Twin>", "TwinAlias<crate::Twin>"]
    );
    assert!(!asserted.support.contains("type_names_PAINT_"), "{}", asserted.support);
    assert!(!asserted.support.contains("__pgrx_c_type_name_PAINT_"), "{}", asserted.support);
    assert!(
        asserted.support.contains("a captured `ty` fragment: MaskedAlias; Twin, TwinAlias\""),
        "{}",
        asserted.support
    );
    // Only a macro that pastes `KIND_` reads its type operand's spelling, and its
    // documentation says so.
    for (name, ..) in EMITTED {
        let (rust, _) = asserted.emitted(name);
        let named = *name != "PaintOf";
        assert_eq!(rust.contains("__pgrx_c_type_name_KIND_!"), named, "{name}: {rust}");
        assert_eq!(
            rust.contains("whose last segment is the C name meant"),
            named,
            "{name}: {rust}"
        );
        assert!(!rust.contains("__pgrx_c_type_name_PAINT_"), "{name}: {rust}");
    }
    // A recovered call never forwards a type operand, so the list macro expands the cast
    // macro it calls and its own normalizer is the only one that reads the operand.
    let (first, _) = asserted.emitted("firstShape");
    assert!(!first.contains("$crate::castShape!"), "{first}");
    assert!(!asserted.bindings.contains("struct Unbound"), "{}", asserted.bindings);

    // A binding whose value disagrees with Clang's refuses the whole lookup, since any type
    // could select the wrong enumerator. Macros that expand a refused macro follow, and a
    // lookup of other enumerators is unaffected.
    for flavor in [&asserted, &released] {
        for emission in &flavor.mismatched {
            let name = emission.analysis.name.as_str();
            let expected = match name {
                "firstShape" => Some(SkipReasonCode::DependencySkipped),
                "PaintOf" => None,
                _ if EMITTED.iter().any(|(emitted, ..)| *emitted == name) => {
                    Some(SkipReasonCode::BindingValueMismatch)
                }
                _ => continue,
            };
            match (&emission.status, expected) {
                (EmissionStatus::Emitted { .. }, None) => {}
                (EmissionStatus::Skipped { reason }, Some(code)) => {
                    assert_eq!(reason.code, code, "{name}: {reason:?}");
                    assert!(
                        reason.message.contains(
                            "bindgen and clang do not agree on KIND_Square's value; bindgen=40, clang=4"
                        ),
                        "{name}: {reason:?}"
                    );
                }
                (status, expected) => panic!("{name}: {status:?}, expected {expected:?}"),
            }
        }
    }

    // With assertions the helper behind `castShape` checks the kind and a wrong one stops
    // the process. Without them the same program casts.
    let (status, stdout, stderr) = asserted.run(MAIN);
    assert!(!status.success(), "{stdout}\n{stderr}");
    let before = "named\t6\t7\t9\t7\npaths\t6\t7\t6\t7\ntwin size\t10\t11\nkind\t3\t4\t2\nalias\t3\t3\nincomplete\t5\t4\nthrough\t4\npaint\t10\t20\nsized\t11\nmade\t3\t0\nis\t1\t0\ncast\ttrue\nnull\ttrue\nfirst\ttrue\ntwins\t6\t7\nis twin\t1\t0\t0\t1\ntwin cast\ttrue\ttrue\nbefore\n";
    assert_eq!(stdout, before);
    assert!(
        stderr.contains("TRAP: failed Assert(\"shape == 0 || shapeKind(shape) == kind\")"),
        "{stderr}"
    );
    let (status, stdout, stderr) = released.run(MAIN);
    assert!(status.success(), "{stdout}\n{stderr}");
    assert_eq!(stdout, format!("{before}after\ttrue\n"));
    assert!(!stderr.contains("TRAP:"), "{stderr}");

    // A type whose name pastes into no enumerator does not compile, as in C, and the error
    // says why. Both flavors report the same errors.
    const UNIMPLEMENTED: &str =
        "no enumerator of `enum ShapeKind` is `KIND_` followed by the C name of";
    const NAMED: &str = "the operand must end in one of those names, written as the last segment of a plain path and not through an alias or a captured `ty` fragment: MaskedAlias; Twin, TwinAlias";
    for (invocation, error, explanation) in [
        ("let _ = KindOf!(Plain);", "E0277", UNIMPLEMENTED),
        // Rust sees one type where C has two names with different enumerators. An alias
        // spells neither name, so it selects neither, and the error names them.
        ("type Pair = Twin; let _ = KindOf!(Pair);", "E0277", NAMED),
        ("type Pair = TwinAlias; let _ = unsafe { makeShape!(Pair) };", "E0277", NAMED),
        // A type fragment that another macro captured hides its spelling.
        (
            "macro_rules! through { ($t:ty) => { KindOf!($t) } } let _ = through!(TwinAlias);",
            "E0277",
            NAMED,
        ),
        // A macro gives `Masked` another enumerator, so that name selects none, and its
        // alias's cannot stand in.
        ("let _ = KindOf!(Masked);", "E0277", UNIMPLEMENTED),
        ("type Hidden = MaskedAlias; let _ = KindOf!(Hidden);", "E0277", UNIMPLEMENTED),
        // Another type whose path ends in a shared name is not the shared structure.
        (
            "mod other { pub struct TwinAlias; } let _ = KindOf!(other::TwinAlias);",
            "E0277",
            UNIMPLEMENTED,
        ),
        (
            "mod other { pub type Twin = super::Circle; } let _ = KindOf!(other::Twin);",
            "E0277",
            UNIMPLEMENTED,
        ),
        // Only a plain path names a C type: a generic type is read as written.
        ("struct Generic<T>(T); let _ = KindOf!(Generic<Twin>);", "E0277", UNIMPLEMENTED),
        ("let _ = KindOf!(Option<TwinAlias>);", "E0277", UNIMPLEMENTED),
        ("let _ = KindOf!(TwinAlias<u8>);", "E0107", "takes 0 generic arguments"),
        ("let _ = KindOf!(*mut TwinAlias);", "E0277", UNIMPLEMENTED),
        // A name must end the operand.
        (
            "let _ = KindOf!(TwinAlias Twin);",
            "error",
            "arguments do not satisfy this C macro's invocation contract",
        ),
        ("let _ = KindOf!(u32);", "E0277", UNIMPLEMENTED),
        ("let _ = unsafe { makeShape!(Plain) };", "E0277", UNIMPLEMENTED),
        // An incomplete type has an enumerator and no size.
        ("let _ = unsafe { makeShape!(Segment) };", "E0277", "Segment"),
        // The operand is a type, so the name of an enumerator without one is no operand.
        ("let _ = KindOf!(Count);", "error", "cannot find type `Count`"),
        // A read through the caller's pointer needs `unsafe`.
        (
            "let shape = Shape { kind: ShapeKind::KIND_Shape }; let _ = IsShape!(&raw const shape, Shape);",
            "E0133",
            "unsafe",
        ),
        // Only the generated support can name the trait that seals a lookup.
        (
            "struct Mine; impl __pgrx_c_generated::TypeEnumerator_KIND_ for Mine { const VALUE: __pgrx_c_macros::CValue<__pgrx_c_macros::CInt> = KindOf!(Circle).into_value(); }",
            "E0277",
            "Seal_KIND_",
        ),
    ] {
        let main = format!("fn main() {{ {invocation} }}");
        let rejections =
            [&asserted, &released].map(|flavor| rust_oracle::reject_rust(&flavor.program(&main)));
        assert!(rejections[0].contains(error), "{invocation}: {}", rejections[0]);
        assert!(rejections[0].contains(explanation), "{invocation}: {}", rejections[0]);
        assert_eq!(diagnostics(&rejections[0]), diagnostics(&rejections[1]), "{invocation}");
    }
}

/// A compiler's error lines, without the source lines and locations that surround them.
fn diagnostics(stderr: &str) -> Vec<&str> {
    stderr.lines().filter(|line| line.starts_with("error")).collect()
}
