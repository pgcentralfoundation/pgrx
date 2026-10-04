//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Guard semantic behavior while simplifying emitted macro entry points.
//!
//! Public forwarding and shared bodies must preserve invocation contexts, explicit
//! captures, repetitions, and statement side effects. Original C observations and
//! invalid consumer cases ensure readability changes do not broaden the contract.

/// Reuse the binding build's collector so fixture tests reconcile exactly the Rust facts used
/// in production generation.
#[path = "../../pgrx-bindgen/src/build/binding_symbols.rs"]
mod binding_symbols;
/// Run original C headers through the bounded independent oracle harness.
#[path = "support/oracle.rs"]
mod oracle;
/// Compile generated consumers and paired negative cases through the bounded Rust oracle
/// harness.
#[path = "support/rust_oracle.rs"]
mod rust_oracle;

/// Use the production scanner, analysis, and emission contracts so these checks exercise the
/// actual C macro pipeline.
use pgrx_c_macros::{
    AnalysisSession, EmissionStatus, MacroScanner, ParameterOrigin, ParameterRole,
    generate_with_bindings, inspect,
};
/// Keep fixture and generated-output locations explicit so consumer builds remain independent
/// of the working directory.
use std::path::PathBuf;

/// Classify PostgreSQL OID constants so fixture bindgen uses the same checked-wrapper boundary
/// as the real binding build.
fn is_builtin_oid(name: &str) -> bool {
    name.ends_with("OID") && name != "HEAP_HASOID"
        || name.ends_with("RelationId")
        || name == "TemplateDbOid"
}

/// Selected fixture macro names; explicit selection also exercises demand-driven adapter
/// generation.
const NAMES: &[&str] = &[
    "CLEAN_EXPR_ZERO",
    "CLEAN_EXPR_BOUNDARY",
    "CLEAN_CAPTURE_ZERO",
    "CLEAN_STMT_ZERO",
    "CLEAN_STMT_BOUNDARY",
    "CLEAN_LOCAL_ZERO",
    "CLEAN_DISCARD",
    "CLEAN_LOCAL",
    "CLEAN_TYPE_PROVEN",
    "CLEAN_UNUSED",
    "CLEAN_REPEAT",
    "CLEAN_RETURN_ZERO",
    "CLEAN_RETURN_ALIAS",
    "CLEAN_RETURN_IF",
];
/// Fixture binding or native-support source paired with the unchanged C oracle.
const NATIVE: &str = r#"
unsigned int cleanup_calls;
void cleanup_tick(void) { cleanup_calls++; }
unsigned int cleanup_value(void) { cleanup_calls++; return cleanup_calls; }
"#;
/// Original C recorder source whose header invocations establish expected semantic
/// observations.
const ORIGINAL: &str = r#"
#include <stdio.h>
static unsigned char zero_result(void) { CLEAN_RETURN_ZERO(); return 99; }
static unsigned char alias_result(void) { CLEAN_RETURN_ALIAS(); return 99; }
static unsigned char maybe_result(int condition) { CLEAN_RETURN_IF(condition); return 99; }
int main(void) {
    int free_value = 41;
    printf("%d %d %d\n", CLEAN_EXPR_ZERO(), (CLEAN_EXPR_BOUNDARY()), CLEAN_CAPTURE_ZERO());
    CLEAN_STMT_ZERO(); CLEAN_STMT_ZERO(); CLEAN_STMT_ZERO();
    CLEAN_DISCARD(CLEAN_STMT_ZERO()); CLEAN_DISCARD(CLEAN_STMT_ZERO());
    CLEAN_DISCARD(CLEAN_STMT_ZERO()); CLEAN_DISCARD(CLEAN_STMT_ZERO());
    CLEAN_LOCAL_ZERO(); CLEAN_LOCAL(19);
    printf("%u\n", cleanup_calls);
    CLEAN_TYPE_PROVEN(unsigned int); CLEAN_TYPE_PROVEN(unsigned int);
    CLEAN_UNUSED(cleanup_value()); CLEAN_UNUSED(cleanup_value());
    CLEAN_REPEAT(cleanup_value()); CLEAN_REPEAT(cleanup_value());
    printf("%u\n", cleanup_calls);
    { CLEAN_STMT_BOUNDARY(); } { CLEAN_STMT_BOUNDARY(); }
    for(int outer=0; outer<2; outer++) {
        if(outer) { CLEAN_STMT_BOUNDARY(); } else cleanup_tick();
        printf("%u\n", cleanup_calls);
    }
    printf("%u %u %u\n", (unsigned int)zero_result(), (unsigned int)alias_result(), cleanup_calls);
    for(int condition=0; condition<2; condition++) {
        unsigned int result = maybe_result(condition);
        printf("%u %u\n", result, cleanup_calls);
    }
}
"#;
/// Rust consumer source exercising actual generated macros and adapters.
const CONSUMER: &str = r#"
fn zero_result() -> u8 { CLEAN_RETURN_ZERO!(); 99 }
fn alias_result() -> u8 {
    CLEAN_RETURN_ALIAS!(@__pgrx_c_return_as [__pgrx_c_macros::CUnsignedChar];);
    99
}
/// # Safety
/// The caller must exclude concurrent accesses to the C-owned counter.
unsafe fn maybe_result(condition:i32) -> u8 {
    // SAFETY: the native stub only increments the initialized counter, fewer
    // than twenty times in this single-threaded executable; it cannot unwind
    // or raise PostgreSQL errors, and has no pointer or resource arguments.
    unsafe { CLEAN_RETURN_IF!(condition); }
    99
}
fn main() {
    println!("{} {} {}", CLEAN_EXPR_ZERO!().get(), CLEAN_EXPR_BOUNDARY!(@__pgrx_c_expression;).get(), CLEAN_CAPTURE_ZERO!(41).get());
    CLEAN_EXPR_ZERO!(@__pgrx_c_discard;);
    CLEAN_EXPR_ZERO!(@__pgrx_emit_discard;);
    // SAFETY: the C-owned counter is initialized and accessed only by this
    // executable's main thread, with no Rust references to it. Its native stubs
    // only increment a bounded counter and cannot perform nonlocal jumps.
    // The generated C local slots stay live, initialized and aligned within
    // their lexical blocks; no pointers or borrows escape those blocks.
    unsafe {
        let _: () = CLEAN_STMT_ZERO!();
        CLEAN_STMT_ZERO!(@__pgrx_c_discard;);
        CLEAN_STMT_ZERO!(@__pgrx_emit_discard;);
        CLEAN_DISCARD!(CLEAN_STMT_ZERO!());
        CLEAN_DISCARD!(@__pgrx_emit_public; (@macro [CLEAN_STMT_ZERO] []),);
        CLEAN_DISCARD!(@__pgrx_emit_discard; (@macro [CLEAN_STMT_ZERO] []),);
        CLEAN_DISCARD!(@__pgrx_c_discard; CLEAN_STMT_ZERO!());
        CLEAN_LOCAL_ZERO!(); CLEAN_LOCAL!(19);
        let calls = cleanup_calls; println!("{}", calls);
        CLEAN_TYPE_PROVEN!(u32);
        CLEAN_TYPE_PROVEN!(@__pgrx_emit_discard; u32,);
        CLEAN_UNUSED!(cleanup_value());
        CLEAN_UNUSED!(@__pgrx_emit_discard; (@unused),);
        CLEAN_REPEAT!(cleanup_value());
        CLEAN_REPEAT!(@__pgrx_emit_discard; (@native [cleanup_value()]),);
        let calls = cleanup_calls; println!("{}", calls);
        CLEAN_STMT_BOUNDARY!(@__pgrx_c_discard; @__pgrx_c_statement;);
        CLEAN_STMT_BOUNDARY!(@__pgrx_emit_discard;);
        for outer in [false,true] {
            if outer { CLEAN_STMT_BOUNDARY!(@__pgrx_c_statement;); } else { cleanup_tick(); }
            let calls = cleanup_calls; println!("{}", calls);
        }
        let zero = zero_result(); let alias = alias_result(); let calls = cleanup_calls;
        println!("{} {} {}", zero, alias, calls);
        for condition in [0_i32,1] {
            let result = maybe_result(condition); let calls = cleanup_calls;
            println!("{} {}", result, calls);
        }
    }
}
"#;

/// Require an illegal grouped invocation to fail in both the original C and emitted Rust
/// consumers.
fn reject_group(rust: &str, invocations: &[String], message: &str) {
    let diagnostic = rust_oracle::reject_rust(&format!(
        "{rust}\nfn main() {{ {} }}",
        invocations.iter().map(|invocation| format!("let _ = {invocation};")).collect::<String>()
    ));
    assert_eq!(
        diagnostic
            .lines()
            .filter(|line| line.starts_with("error: ") && line.contains(message))
            .count(),
        invocations.len(),
        "every invalid route must retain its own diagnostic: {diagnostic}"
    );
}

/// Checks that emission cleanup preserves invocation contexts captures and statement effects.
#[test]
fn emission_cleanup_preserves_invocation_contexts_captures_and_statement_effects() {
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let header = directory.join("tests/fixtures/emission_cleanup.h");
    let scanner = MacroScanner::new().expect("libclang required");
    let mut arguments = vec!["-std=c17".into(), "-O2".into(), "-ffp-contract=off".into()];
    #[cfg(target_os = "macos")]
    {
        let sdk = rust_oracle::run_tool(
            std::process::Command::new("xcrun").arg("--show-sdk-path"),
            "cleanup oracle SDK",
        );
        arguments.extend(["-isysroot".into(), sdk.trim().into()]);
    }
    let frontend = inspect(&scanner, &header, &arguments, None).unwrap();
    let session = AnalysisSession::prepare(&scanner, &frontend, NAMES).unwrap();
    let bindings = bindgen::Builder::default()
        .rust_target(bindgen::RustTarget::stable(85, 0).unwrap())
        .rust_edition(bindgen::RustEdition::Edition2024)
        .header(header.to_str().unwrap())
        .clang_args(&frontend.profile().arguments)
        .allowlist_function("cleanup_.*")
        .allowlist_var("cleanup_.*")
        .use_core()
        .wrap_unsafe_ops(true)
        .layout_tests(false)
        .generate_comments(false)
        .formatter(bindgen::Formatter::None)
        .generate()
        .unwrap()
        .to_string();
    let catalog = binding_symbols::collect_bindings(
        &syn::parse_file(&bindings).unwrap(),
        session.integer_constants(),
        frontend.declarations(),
        &frontend.profile().target,
    );
    let generated = generate_with_bindings(&session, NAMES, &catalog).unwrap();
    assert_eq!(generated.macros.len(), NAMES.len());
    let runtime = directory.join("../pgrx-pg-sys/src/c_macros/support.rs").canonicalize().unwrap();
    let mut rust = format!(
        "#![deny(unsafe_op_in_unsafe_fn)]\n#![allow(non_snake_case,non_camel_case_types,dead_code,unused_parens,unreachable_code)]\n\
         #[path={runtime:?}] pub mod __pgrx_c_macros;\n{bindings}\n{}",
        generated.support.rust
    );
    for emission in &generated.macros {
        let EmissionStatus::Emitted { rust: definition, .. } = &emission.status else {
            panic!("cleanup fixture must transpile: {emission:?}");
        };
        let syntax = syn::parse_file(definition).unwrap();
        let public = syntax
            .items
            .iter()
            .find_map(|item| match item {
                syn::Item::Macro(item)
                    if item
                        .ident
                        .as_ref()
                        .is_some_and(|ident| ident == &emission.analysis.name) =>
                {
                    Some(item)
                }
                _ => None,
            })
            .unwrap();
        let normalizer = format!("__pgrx_c_args_{}", emission.analysis.name);
        let has_normalizer = syntax.items.iter().any(|item| {
            matches!(item, syn::Item::Macro(item) if item.ident.as_ref().is_some_and(|ident| ident == &normalizer))
        });
        assert_eq!(has_normalizer, !emission.analysis.parameters.is_empty());
        if emission.analysis.name == "CLEAN_CAPTURE_ZERO" {
            assert_eq!(emission.analysis.parameters.len(), 1);
            assert_eq!(emission.analysis.parameters[0].origin, ParameterOrigin::FreeIdentifier);
            assert_eq!(emission.analysis.parameters[0].name, "free_value");
        }
        if emission.analysis.name == "CLEAN_TYPE_PROVEN" {
            assert_eq!(emission.analysis.parameters.len(), 1);
            assert_eq!(emission.analysis.parameters[0].roles, [ParameterRole::Type]);
        }
        let body = public.mac.tokens.to_string();
        if emission.analysis.expression.as_ref().unwrap().syntax.statement_body.is_none() {
            let rules = public.mac.tokens.clone().into_iter().collect::<Vec<_>>();
            let public_body = rules
                .windows(4)
                .find_map(|rule| match rule {
                    [
                        proc_macro2::TokenTree::Group(matcher),
                        _,
                        _,
                        proc_macro2::TokenTree::Group(body),
                    ] if matcher.stream().to_string().starts_with("@ __pgrx_emit_public ;") => {
                        Some(body.stream())
                    }
                    _ => None,
                })
                .unwrap();
            let name = &emission.analysis.name;
            let operands = emission
                .analysis
                .parameters
                .iter()
                .map(|parameter| format!("${}", parameter.name))
                .collect::<Vec<_>>()
                .join(", ");
            let expected: proc_macro2::TokenStream = format!(
                "$crate::__pgrx_c_macros::expression_result::finish($crate::{name}!(@__pgrx_emit_value; {operands}))"
            )
            .parse()
            .unwrap();
            assert_eq!(public_body.to_string(), expected.to_string());
        }
        assert_eq!(body.contains("local_scope_allowed"), emission.analysis.name == "CLEAN_LOCAL");
        if matches!(
            emission.analysis.name.as_str(),
            "CLEAN_STMT_ZERO"
                | "CLEAN_STMT_BOUNDARY"
                | "CLEAN_LOCAL_ZERO"
                | "CLEAN_LOCAL"
                | "CLEAN_TYPE_PROVEN"
                | "CLEAN_UNUSED"
        ) {
            assert_eq!(
                body.matches("cleanup_tick").count(),
                1,
                "public/discard must share one rendered statement body"
            );
        }
        if body.contains("a C statement body is not an expression operand") {
            assert_eq!(
                body.matches("a C statement body is not an expression operand").count(),
                1,
                "unsupported statement contexts must share their diagnostic"
            );
        }
        rust.push_str(definition);
    }
    let profile = frontend.profile();
    let arguments = profile.arguments.iter().map(String::as_str).collect::<Vec<_>>();
    let native = format!("{NATIVE}\n{}", generated.support.c_source);
    let expected = oracle::run_c(
        &profile.compiler.executable,
        &header,
        &format!("{native}\n{ORIGINAL}"),
        &arguments,
        true,
    );
    let actual = rust_oracle::run_rust_linked(
        &format!("{rust}\n{CONSUMER}"),
        &profile.compiler.executable,
        &header,
        &native,
        &arguments,
    );
    assert_eq!(
        actual, expected,
        "cleanup must preserve C effects, return conversions and boundaries"
    );
    assert_eq!(actual.lines().count(), 8);

    reject_group(
        &rust,
        &[
            "CLEAN_EXPR_ZERO!(1)".into(),
            "CLEAN_STMT_ZERO!(,)".into(),
            "CLEAN_RETURN_ZERO!(@__pgrx_c_return_as [__pgrx_c_macros::CUnsignedChar]; 1)".into(),
            "CLEAN_CAPTURE_ZERO!()".into(),
        ],
        "invocation contract",
    );
    let modes = ["value", "place", "read_place", "size"];
    let invalid = modes
        .into_iter()
        .flat_map(|mode| {
            [
                format!("CLEAN_STMT_ZERO!(@__pgrx_c_{mode};)"),
                format!("CLEAN_STMT_ZERO!(@__pgrx_emit_{mode};)"),
            ]
        })
        .collect::<Vec<_>>();
    reject_group(&rust, &invalid, "not an expression operand");
    reject_group(
        &rust,
        &[
            "CLEAN_RETURN_ZERO!(@__pgrx_c_discard;)".into(),
            "CLEAN_RETURN_ZERO!(@__pgrx_emit_discard;)".into(),
        ],
        "not an expression operand",
    );
    for (invocation, message) in [
        ("CLEAN_EXPR_BOUNDARY!()", "explicit parenthesized invocation"),
        ("CLEAN_STMT_BOUNDARY!()", "explicit braced invocation"),
        ("CLEAN_EXPR_BOUNDARY!(@__pgrx_c_value;)", "explicit parenthesized invocation"),
        ("CLEAN_STMT_BOUNDARY!(@__pgrx_c_discard;)", "explicit braced invocation"),
    ] {
        reject_group(&rust, &[invocation.into()], message);
    }
    let diagnostic = rust_oracle::reject_rust(&format!(
        "{rust}\nfn main() {{ let temporary = 19_i32; unsafe {{ CLEAN_LOCAL!(temporary); }} }}"
    ));
    assert!(
        diagnostic.contains("C macro argument mentions local temporary"),
        "local capture guard must remain: {diagnostic}"
    );
}
