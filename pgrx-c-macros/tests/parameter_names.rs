//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Verify that original C formal names survive public Rust macro generation.
//!
//! Matchers, bodies, keyword escaping, and argument normalization are inspected
//! and exercised against C. Readable names must remain hygienic and cannot collide
//! with transpiler-owned context or temporary identifiers.

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

use pgrx_c_macros::{
    AnalysisSession, EmissionStatus, MacroScanner, ParameterOrigin, ParameterRole,
    generate_with_bindings, inspect,
};
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
    "NAME_PRIVS",
    "NAME_PAIR",
    "NAME_REPEAT",
    "NAME_UNUSED",
    "NAME_CAST",
    "NAME_CAST_PROVEN",
    "NAME_FIELD",
    "NAME_KEYWORDS",
    "NAME_RUST_SPECIAL",
    "NAME_SCAFFOLD",
    "NAME_RETURN",
    "NAME_RETURN_CHAIN",
    "NAME_CALL",
    "NAME_CAPTURE",
    "NAME_INNER",
    "NAME_FORMAL_CAPTURE",
    "NAME_INNER_CRATE",
    "NAME_CRATE_CAPTURE",
    "NAME_RESERVED",
];

/// Checks that original C parameter names survive matchers bodies and argument normalization.
#[test]
fn original_c_parameter_names_survive_matchers_bodies_and_argument_normalization() {
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let header = directory.join("tests/fixtures/parameter_names.h");
    let scanner = MacroScanner::new().expect("libclang required");
    let mut arguments = vec!["-std=c17".into(), "-O2".into(), "-fwrapv".into()];
    #[cfg(target_os = "macos")]
    {
        let sdk = rust_oracle::run_tool(
            std::process::Command::new("xcrun").arg("--show-sdk-path"),
            "parameter names oracle SDK",
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
        .allowlist_type("ParameterRecord")
        .allowlist_function("parameter_.*")
        .allowlist_var("parameter_.*")
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
    let generation = generate_with_bindings(&session, NAMES, &catalog).unwrap();
    let support = directory.join("../pgrx-pg-sys/src/c_macros/support.rs").canonicalize().unwrap();
    let mut rust = format!(
        "#![deny(unsafe_op_in_unsafe_fn)]\n#![allow(non_snake_case,non_camel_case_types,dead_code,unused_parens,unreachable_code)]\n#[path={support:?}] pub mod __pgrx_c_macros;\n{bindings}\n{}",
        generation.support.rust,
    );
    for emission in generation.macros {
        if emission.analysis.name == "NAME_CAST" {
            let EmissionStatus::Skipped { reason } = emission.status else {
                panic!("a formal application alone cannot establish a cast type: {emission:?}")
            };
            assert!(reason.message.contains("cast type or callable value"));
            continue;
        }
        if emission.analysis.name == "NAME_RESERVED" {
            let EmissionStatus::Skipped { reason } = emission.status else {
                panic!("the reserved Rust $crate formal cannot be silently renamed: {emission:?}")
            };
            assert!(reason.message.contains("$crate") && reason.message.contains("name"));
            continue;
        }
        let EmissionStatus::Emitted { rust: definition, .. } = emission.status else {
            panic!("C parameter spelling must not prevent valid Rust metavariables: {emission:?}")
        };
        if emission.analysis.name == "NAME_FORMAL_CAPTURE" {
            assert_eq!(emission.analysis.parameters.len(), 2);
            assert_eq!(emission.analysis.parameters[0].name, "fcinfo");
            assert_eq!(emission.analysis.parameters[0].origin, ParameterOrigin::Formal);
            assert_eq!(emission.analysis.parameters[1].name, "fcinfo");
            assert_eq!(emission.analysis.parameters[1].origin, ParameterOrigin::FreeIdentifier);
        }
        let parsed = syn::parse_file(&definition).expect("generated macro declarations must parse");
        let item = parsed
            .items
            .iter()
            .find_map(|item| match item {
                syn::Item::Macro(item)
                    if item.ident.as_ref().is_some_and(|name| name == &emission.analysis.name) =>
                {
                    Some(item)
                }
                _ => None,
            })
            .expect("find the public generated macro");
        let doc = item
            .attrs
            .iter()
            .filter_map(|attribute| match &attribute.meta {
                syn::Meta::NameValue(meta) if meta.path.is_ident("doc") => match &meta.value {
                    syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(value), .. }) => {
                        let line = value.value();
                        Some(line.strip_prefix(' ').unwrap_or(&line).to_owned())
                    }
                    _ => None,
                },
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("\n");
        let original = frontend
            .inventory()
            .macros
            .iter()
            .find(|original| original.name == emission.analysis.name)
            .unwrap();
        assert!(
            definition.contains(&format!("/// ```text\n/// {original}\n/// ```\n")),
            "renaming Rust bookkeeping must leave the original C definition untouched: {definition}"
        );
        assert!(
            doc.contains(&format!("\n```text\n{original}\n```")),
            "the original C definition must also remain available to Rustdoc: {doc}"
        );

        // Inspect the semantic arms rather than pinning the normalizer's private
        // bookkeeping names. Rust compilation below also detects duplicate
        // matcher bindings when a C name collides with that bookkeeping.
        let tokens = item.mac.tokens.clone().into_iter().collect::<Vec<_>>();
        let has_value_arm = tokens
            .chunks_exact(5)
            .any(|arm| arm[0].to_string().replace(' ', "").contains("@__pgrx_emit_value"));
        let mut semantic_arms = 0;
        let mut public_arms = 0;
        let mut evaluation_arms = 0;
        for arm in tokens.chunks_exact(5) {
            let matcher = arm[0].to_string().replace(' ', "");
            if !matcher.contains("@__pgrx_emit_") {
                continue;
            }
            semantic_arms += 1;
            let body = arm[3].to_string().replace(' ', "");
            if matcher.contains("@__pgrx_emit_public") {
                public_arms += 1;
            }
            // Public expressions forward normalized tokens, including unused
            // operands, to the shared value body without evaluating them.
            let evaluates = matcher.contains("@__pgrx_emit_value")
                || (!has_value_arm && matcher.contains("@__pgrx_emit_public"));
            evaluation_arms += usize::from(evaluates);
            for parameter in &emission.analysis.parameters {
                // Explicit captures may need a fresh Rust name when nested C
                // expansion gives them the same spelling as an outer formal.
                // The independent oracle below distinguishes their values.
                if parameter.origin == ParameterOrigin::FreeIdentifier {
                    continue;
                }
                let fragment = match parameter.roles.as_slice() {
                    [ParameterRole::Type] => "ty",
                    [ParameterRole::Identifier] => "ident",
                    _ => "tt",
                };
                assert!(
                    matcher.contains(&format!("${}:{fragment}", parameter.name)),
                    "{} must retain parameter {} in its {fragment} matcher: {matcher}",
                    emission.analysis.name,
                    parameter.name,
                );
                if evaluates {
                    assert_eq!(
                        body.contains(&format!("${}", parameter.name)),
                        !parameter.uses.is_empty(),
                        "{} must retain parameter {} in its actual C body: {body}",
                        emission.analysis.name,
                        parameter.name,
                    );
                }
            }
            assert!(
                !matcher.contains("$__pgrx_c_arg"),
                "synthetic positional arguments must not obscure C formal names: {matcher}"
            );
        }
        assert!(semantic_arms > 0, "source assertions must inspect actual semantic arms");
        assert_eq!(public_arms, 1);
        assert_eq!(evaluation_arms, 1, "inspect the body that evaluates C operands");
        rust.push_str(&definition);
    }
    let native = format!(
        "unsigned int parameter_evaluations;\nunsigned int parameter_record(unsigned int value) {{ parameter_evaluations++; return value; }}\n{}",
        generation.support.c_source,
    );
    let original = r#"
#include <stdio.h>
static unsigned char return_byte(void) { NAME_RETURN(250, 2, 3, 4, 5); }
static unsigned long return_marker(void) { NAME_RETURN(1, 2, 3, 4, -1); }
static unsigned long return_chain(void) { NAME_RETURN_CHAIN(1, 2, 3, 4); }
int main(void) {
    unsigned int a = NAME_PRIVS(parameter_record(15));
    unsigned int b = NAME_PAIR(3U, 4U);
    unsigned int c = NAME_REPEAT(parameter_record(29));
    unsigned int d = NAME_UNUSED(7U, unknown_identifier);
    unsigned int e = NAME_CAST_PROVEN(unsigned char, parameter_record(257));
    ParameterRecord object = {37, 41};
    unsigned int f = NAME_FIELD(&object, state);
    unsigned int g = NAME_KEYWORDS(1U, 2U);
    unsigned int h = NAME_RUST_SPECIAL(1U, 2U, 3U, 4U);
    unsigned int i = NAME_SCAFFOLD(1U, 2U, 3U, 4U, 5U);
    unsigned int j = NAME_CALL(parameter_record(15));
    unsigned int context = 43;
    unsigned int k = NAME_CAPTURE(7U);
    unsigned int fcinfo = 47;
    unsigned int crate = 53;
    unsigned int l = NAME_FORMAL_CAPTURE(3U);
    unsigned int m = NAME_CRATE_CAPTURE(5U);
    printf("%u %u %u %u %u %u %u %u %u %u %u %u %u %u\n",a,b,c,d,e,f,g,h,i,j,k,l,m,parameter_evaluations);
    printf("%u %lu %lu\n", (unsigned int) return_byte(), return_marker(), return_chain());
}
"#;
    let consumer = r#"
fn return_byte() -> u8 { NAME_RETURN!(250, 2, 3, 4, 5); }
fn return_marker() -> core::ffi::c_ulong {
    NAME_RETURN!(@__pgrx_c_return_as [__pgrx_c_macros::CUnsignedLong]; 1, 2, 3, 4, -1);
}
fn return_chain() -> core::ffi::c_ulong {
    NAME_RETURN_CHAIN!(@__pgrx_c_return_as [__pgrx_c_macros::CUnsignedLong]; 1, 2, 3, 4);
}
fn main() {
    let mut object = ParameterRecord { state: 37, byte: 41 };
    let pointer = &raw mut object;
    // SAFETY: The pointer names a live aligned initialized local record and is
    // used only for a field read. Native callbacks mutate initialized process-local
    // counters in this single-threaded executable and never call a backend.
    unsafe {
        let a = NAME_PRIVS!(parameter_record(15)).get();
        let b = NAME_PAIR!(3_u32, 4_u32).get();
        let c = NAME_REPEAT!(parameter_record(29)).get();
        let d = NAME_UNUSED!(7_u32, unknown_identifier).get();
        let e = NAME_CAST_PROVEN!(u8, parameter_record(257)).get();
        let f = NAME_FIELD!(pointer, state).get();
        let g = NAME_KEYWORDS!(1_u32, 2_u32).get();
        let h = NAME_RUST_SPECIAL!(1_u32, 2_u32, 3_u32, 4_u32).get();
        let i = NAME_SCAFFOLD!(1_u32, 2_u32, 3_u32, 4_u32, 5_u32).get();
        let j = NAME_CALL!(parameter_record(15)).get();
        let k = NAME_CAPTURE!(7_u32, 43_u32).get();
        let l = NAME_FORMAL_CAPTURE!(3_u32, 47_u32).get();
        let m = NAME_CRATE_CAPTURE!(5_u32, 53_u32).get();
        let evaluations = parameter_evaluations;
        println!("{} {} {} {} {} {} {} {} {} {} {} {} {} {}", a,b,c,d,e,f,g,h,i,j,k,l,m,evaluations);
        assert_eq!(evaluations, 5, "repeated and unused arguments preserve C evaluation");
    }
    println!("{} {} {}", return_byte(), return_marker(), return_chain());
}
"#;
    let profile = frontend.profile();
    let arguments = profile.arguments.iter().map(String::as_str).collect::<Vec<_>>();
    let original = oracle::run_c(
        &profile.compiler.executable,
        &header,
        &format!("{native}\n{original}"),
        &arguments,
        true,
    );
    let emitted = rust_oracle::run_rust_linked(
        &format!("{rust}\n{consumer}"),
        &profile.compiler.executable,
        &header,
        &native,
        &arguments,
    );
    assert_eq!(emitted, original, "formal spelling changes must preserve C behavior");
    assert_eq!(emitted.lines().count(), 2);
}
