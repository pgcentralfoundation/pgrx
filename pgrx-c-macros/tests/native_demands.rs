//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

#[path = "../../pgrx-bindgen/src/build/binding_symbols.rs"]
mod binding_symbols;
#[path = "support/oracle.rs"]
mod oracle;
#[path = "support/rust_oracle.rs"]
mod rust_oracle;

use pgrx_c_macros::{
    AnalysisSession, BindingCatalog, EmissionStatus, FrontendOutput, MacroGeneration, MacroScanner,
    RustBindingType, SkipReasonCode, generate_with_bindings, inspect,
};
use proc_macro2::{Delimiter, TokenTree};
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

fn is_builtin_oid(name: &str) -> bool {
    name.ends_with("OID") && name != "HEAP_HASOID"
        || name.ends_with("RelationId")
        || name == "TemplateDbOid"
}

fn adapter_fields(source: &str, capability: &str) -> BTreeSet<(String, String)> {
    fn collect(
        items: &[syn::Item],
        capability: &str,
        names: &BTreeMap<String, String>,
        output: &mut BTreeSet<(String, String)>,
    ) {
        for item in items {
            if let syn::Item::Mod(module) = item {
                if let Some((_, items)) = &module.content {
                    collect(items, capability, names, output);
                }
                continue;
            }
            let syn::Item::Impl(item) = item else { continue };
            let Some((_, implemented, _)) = &item.trait_ else { continue };
            if !implemented.segments.last().is_some_and(|segment| {
                segment.ident == capability
                    || capability == "Field" && segment.ident == "OrdinaryField"
            }) {
                continue;
            }
            let syn::Type::Path(ty) = item.self_ty.as_ref() else { continue };
            let Some(segment) = ty.path.segments.last() else { continue };
            let syn::PathArguments::AngleBracketed(arguments) = &segment.arguments else {
                continue;
            };
            let Some(syn::GenericArgument::Type(syn::Type::Path(storage))) = arguments.args.first()
            else {
                continue;
            };
            let syn::PathArguments::AngleBracketed(arguments) =
                &implemented.segments.last().unwrap().arguments
            else {
                continue;
            };
            let Some(syn::GenericArgument::Type(syn::Type::Path(marker))) = arguments.args.first()
            else {
                continue;
            };
            output.insert((
                storage.path.segments.last().unwrap().ident.to_string(),
                names[&marker.path.segments.last().unwrap().ident.to_string()].clone(),
            ));
        }
    }
    let file = syn::parse_file(source).expect("generated adapters must parse");
    let mut names = BTreeMap::new();
    for item in &file.items {
        let syn::Item::Macro(item) = item else { continue };
        if item.ident.as_ref().is_none_or(|name| name != "__pgrx_c_field_marker") {
            continue;
        }
        let tokens = item.mac.tokens.clone().into_iter().collect::<Vec<_>>();
        for window in tokens.windows(4) {
            let [
                TokenTree::Group(matcher),
                TokenTree::Punct(equal),
                TokenTree::Punct(arrow),
                TokenTree::Group(body),
            ] = window
            else {
                continue;
            };
            if matcher.delimiter() != Delimiter::Parenthesis
                || equal.as_char() != '='
                || arrow.as_char() != '>'
            {
                continue;
            }
            let matcher = matcher.stream().into_iter().collect::<Vec<_>>();
            let [TokenTree::Ident(name)] = matcher.as_slice() else { continue };
            let name = name.to_string().trim_start_matches("r#").to_owned();
            // Resolve the emitted selector's $crate path rather than duplicating
            // the generator's private field identity allocation.
            let path = syn::parse2::<syn::Path>(body.stream().into_iter().skip(1).collect())
                .expect("field selector must emit a type path");
            let marker = path.segments.last().unwrap().ident.to_string();
            if let Some(previous) = names.insert(marker, name.clone()) {
                assert_eq!(previous, name, "different C fields must have different identities");
            }
        }
    }
    let mut owners = BTreeSet::new();
    collect(&file.items, capability, &names, &mut owners);
    owners
}

fn adapter_owners(source: &str, capability: &str) -> BTreeSet<String> {
    adapter_fields(source, capability).into_iter().map(|(owner, _)| owner).collect()
}

fn capability_types(source: &str, capability: &str) -> BTreeSet<String> {
    fn collect(items: &[syn::Item], capability: &str, output: &mut BTreeSet<String>) {
        for item in items {
            match item {
                syn::Item::Mod(module) => {
                    if let Some((_, items)) = &module.content {
                        collect(items, capability, output);
                    }
                }
                syn::Item::Impl(item)
                    if item.trait_.as_ref().is_some_and(|(_, implemented, _)| {
                        implemented.segments.last().is_some_and(|part| {
                            part.ident == capability
                                || capability == "NativeType" && part.ident == "NativeRecord"
                        })
                    }) =>
                {
                    let syn::Type::Path(ty) = item.self_ty.as_ref() else { continue };
                    output.insert(ty.path.segments.last().unwrap().ident.to_string());
                }
                _ => {}
            }
        }
    }
    let mut output = BTreeSet::new();
    collect(&syn::parse_file(source).unwrap().items, capability, &mut output);
    output
}

fn callback_storage(ty: &RustBindingType, bindings: &BindingCatalog) -> String {
    fn normalize(ty: &mut RustBindingType, bindings: &BindingCatalog, depth: usize) {
        assert!(depth < 64, "fixture callback aliases must be finite");
        match ty {
            RustBindingType::Named { path } => {
                if let Some(alias) = bindings.types.get(&path.join("::")) {
                    *ty = alias.target.clone();
                    normalize(ty, bindings, depth + 1);
                }
            }
            RustBindingType::Pointer { pointee: value, .. } | RustBindingType::Option { value } => {
                normalize(value, bindings, depth + 1)
            }
            RustBindingType::Function { parameters, result, .. } => {
                for parameter in parameters {
                    normalize(parameter, bindings, depth + 1);
                }
                normalize(result, bindings, depth + 1);
            }
            _ => {}
        }
    }
    let mut ty = ty.clone();
    normalize(&mut ty, bindings, 0);
    serde_json::to_string(&ty).unwrap()
}

/// Read the actual ABI storage witness associated with each emitted Call marker.
fn callback_calls(
    source: &str,
    bindings: &BindingCatalog,
    frontend: &FrontendOutput,
) -> BTreeSet<String> {
    fn collect(items: &[syn::Item], calls: &BTreeSet<String>, output: &mut Vec<syn::Item>) {
        for item in items {
            if let syn::Item::Mod(module) = item {
                if let Some((_, items)) = &module.content {
                    collect(items, calls, output);
                }
                continue;
            }
            let syn::Item::Impl(item) = item else { continue };
            if item.trait_.as_ref().is_none_or(|(_, implemented, _)| {
                implemented
                    .segments
                    .last()
                    .is_none_or(|part| part.ident != "NativeFunctionSignature")
            }) {
                continue;
            }
            let syn::Type::Path(marker) = item.self_ty.as_ref() else { continue };
            if !calls.contains(&marker.path.segments.last().unwrap().ident.to_string()) {
                continue;
            }
            for item in &item.items {
                let syn::ImplItem::Type(item) = item else { continue };
                if item.ident != "Physical" {
                    continue;
                }
                let syn::Type::Path(physical) = &item.ty else { panic!("physical ABI witness") };
                let syn::PathArguments::AngleBracketed(arguments) =
                    &physical.path.segments.last().unwrap().arguments
                else {
                    panic!("physical ABI storage argument")
                };
                let Some(syn::GenericArgument::Type(storage)) = arguments.args.first() else {
                    panic!("physical ABI storage witness")
                };
                output.push(syn::parse_quote!(pub type __FixtureCallStorage = #storage;));
            }
        }
    }
    let mut output = Vec::new();
    collect(
        &syn::parse_file(source).unwrap().items,
        &capability_types(source, "Call"),
        &mut output,
    );
    callback_storage_types(output, bindings, frontend)
}

fn callback_storage_types(
    items: Vec<syn::Item>,
    bindings: &BindingCatalog,
    frontend: &FrontendOutput,
) -> BTreeSet<String> {
    items
        .into_iter()
        .map(|item| {
            let parsed = binding_symbols::collect_bindings(
                &syn::File { shebang: None, attrs: Vec::new(), items: vec![item] },
                &BTreeMap::new(),
                frontend.declarations(),
                &frontend.profile().target,
            );
            callback_storage(&parsed.types["__FixtureCallStorage"].target, bindings)
        })
        .collect()
}

fn callback_native_inputs(
    source: &str,
    bindings: &BindingCatalog,
    frontend: &FrontendOutput,
) -> BTreeSet<String> {
    fn collect(items: &[syn::Item], output: &mut Vec<syn::Item>) {
        for item in items {
            if let syn::Item::Mod(module) = item
                && let Some((_, items)) = &module.content
            {
                collect(items, output);
            }
            let syn::Item::Impl(item) = item else { continue };
            if item.trait_.as_ref().is_none_or(|(_, implemented, _)| {
                implemented.segments.last().is_none_or(|part| part.ident != "NativeType")
            }) {
                continue;
            }
            let syn::Type::Path(storage) = item.self_ty.as_ref() else { continue };
            if storage.path.segments.last().is_none_or(|part| part.ident != "Option") {
                continue;
            }
            output.push(syn::parse_quote!(pub type __FixtureCallStorage = #storage;));
        }
    }
    let mut output = Vec::new();
    collect(&syn::parse_file(source).unwrap().items, &mut output);
    callback_storage_types(output, bindings, frontend)
}

fn program(bindings: &str, generated: &MacroGeneration, body: &str) -> String {
    let support = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../pgrx-pg-sys/src/c_macros/support.rs")
        .canonicalize()
        .unwrap();
    let mut source = format!(
        "#![deny(unsafe_op_in_unsafe_fn)]\n#![allow(non_snake_case,non_camel_case_types,dead_code)]\n#[path={support:?}] pub mod __pgrx_c_macros;\n{bindings}\n{}\n",
        generated.support.rust
    );
    for emission in &generated.macros {
        let EmissionStatus::Emitted { rust, .. } = &emission.status else {
            panic!("requested fixture macro must emit: {emission:?}");
        };
        source.push_str(rust);
    }
    source.push_str(body);
    source
}

#[test]
fn native_capabilities_follow_nominal_owners_and_keep_unknown_caller_types() {
    let project = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let header = project.join("tests/fixtures/native_demands.h");
    let scanner = MacroScanner::new().expect("libclang required");
    let mut arguments = vec!["-std=c17".into(), "-ffp-contract=off".into()];
    #[cfg(target_os = "macos")]
    {
        let sdk = rust_oracle::run_tool(
            std::process::Command::new("xcrun").arg("--show-sdk-path"),
            "native demand oracle SDK",
        );
        arguments.extend(["-isysroot".into(), sdk.trim().into()]);
    }
    let frontend = inspect(&scanner, &header, &arguments, None).unwrap();
    let names = frontend
        .environment()
        .active
        .keys()
        .filter(|name| name.starts_with("DEMAND_"))
        .map(String::as_str)
        .collect::<Vec<_>>();
    let session = AnalysisSession::prepare(&scanner, &frontend, &names).unwrap();
    let bindings = bindgen::Builder::default()
        .rust_target(bindgen::RustTarget::stable(85, 0).unwrap())
        .rust_edition(bindgen::RustEdition::Edition2024)
        .header(header.to_str().unwrap())
        .clang_args(&frontend.profile().arguments)
        .allowlist_type("Demand.*")
        .allowlist_function("demand_.*")
        .allowlist_var("demand_.*")
        .rustified_enum("DemandIndex")
        .use_core()
        .wrap_unsafe_ops(true)
        .layout_tests(false)
        .generate_comments(false)
        .formatter(bindgen::Formatter::None)
        .generate()
        .expect("generate fresh native demand fixture bindings")
        .to_string();
    let catalog = binding_symbols::collect_bindings(
        &syn::parse_file(&bindings).unwrap(),
        session.integer_constants(),
        frontend.declarations(),
        &frontend.profile().target,
    );
    let compare = |generated: &MacroGeneration, rust_body: &str, c_body: &str, rows: usize| {
        assert!(
            generated.support.c_source.is_empty(),
            "pure fixture comparisons need no original-C access primitives"
        );
        let original = oracle::run_c(
            &frontend.profile().compiler.executable,
            &header,
            c_body,
            &frontend.profile().arguments.iter().map(String::as_str).collect::<Vec<_>>(),
            true,
        );
        let actual = rust_oracle::run_rust(&program(&bindings, generated, rust_body));
        assert_eq!(original.lines().count(), rows, "complete original-C demand comparison");
        assert_eq!(actual, original, "scoped adapters must preserve original C behavior");
    };
    let native_calls = r#"
int demand_integer_function(int value) { return value; }
DemandCallback demand_known_callback = demand_integer_function;
int demand_bool_function(_Bool value) { return value; }
int demand_pointer_function(void *value) { return value != (void *)0; }
DemandLeaf demand_record_function(DemandLeaf value) { return value; }
int demand_leaf_pointer_function(const DemandLeaf *value) { return value ? (int)value->leaf : 0; }
int demand_mutable_leaf_function(DemandLeaf *value) { return value ? (int)++value->leaf : 0; }
int demand_nested_pointer_function(DemandLeaf **value) { return value && *value ? (int)(*value)->leaf : 0; }
int demand_unsigned_pointer_function(unsigned int *value) { return value ? (int)*value : 0; }
int demand_callback_function(DemandCallback value) { return value ? value(47) : 0; }
unsigned int demand_upstream_unsigned(unsigned int value) { return value + 1U; }
DemandUpstreamLeafA *demand_upstream_fetch_a(int value) {
    static DemandUpstreamLeafA result;
    result.payload = value;
    result.finish = demand_integer_function;
    return &result;
}
const DemandUpstreamLeafB *demand_upstream_fetch_b(unsigned int value) {
    static DemandUpstreamLeafB result;
    result.payload = value;
    result.finish = demand_upstream_unsigned;
    return &result;
}
"#;
    let compare_calls = |generated: &MacroGeneration, rust_body: &str, c_body: &str| {
        let arguments = frontend.profile().arguments.iter().map(String::as_str).collect::<Vec<_>>();
        let original = oracle::run_c(
            &frontend.profile().compiler.executable,
            &header,
            &format!("{native_calls}\n{c_body}"),
            &arguments,
            true,
        );
        let actual = rust_oracle::run_rust_linked(
            &program(&bindings, generated, rust_body),
            &frontend.profile().compiler.executable,
            &header,
            &format!("{native_calls}\n{}", generated.support.c_source),
            &arguments,
        );
        assert_eq!(original.lines().count(), 1);
        assert_eq!(actual, original, "prototype constraints must preserve original C conversions");
    };

    // A caller formal in (formal)(operand) can be a type or a callee. Keep
    // those originals as explicit skip coverage; the siblings below establish
    // expression calls using ((formal))(operand), without guessing a type.
    let ambiguous = generate_with_bindings(
        &session,
        &[
            "DEMAND_OPEN_CALLBACK",
            "DEMAND_OPEN_INTEGER",
            "DEMAND_OPEN_NULL",
            "DEMAND_OPEN_LEAF",
            "DEMAND_OPEN_CONST_LEAF",
            "DEMAND_OPEN_VOID",
            "DEMAND_OPEN_RECORD",
            "DEMAND_OPEN_STORED_INTEGER",
            "DEMAND_OPEN_INTEGER_RESULT",
            "DEMAND_OPEN_TWO_SITES",
            "DEMAND_OPEN_UNCONSTRAINED_SITE",
        ],
        &catalog,
    )
    .unwrap();
    assert_eq!(ambiguous.macros.len(), 11, "every ambiguous original is checked");
    for emission in &ambiguous.macros {
        let EmissionStatus::Skipped { reason } = &emission.status else {
            panic!("ambiguous type/call must remain a structured skip: {emission:?}");
        };
        assert_eq!(reason.code, SkipReasonCode::TypeParameter, "{}", emission.analysis.name);
    }

    let callback_aliases = catalog
        .types
        .iter()
        .map(|(name, alias)| (name.clone(), callback_storage(&alias.target, &catalog)))
        .collect::<BTreeMap<_, _>>();
    let callback_calls = |source: &str| callback_calls(source, &catalog, &frontend);
    let callback_family = |name: &str, admitted: &[&str], excluded: &[&str]| {
        let generated = generate_with_bindings(&session, &[name], &catalog).unwrap();
        let calls = callback_calls(&generated.support.rust);
        for alias in admitted {
            assert!(
                calls.contains(&callback_aliases[*alias]),
                "{name} must retain {alias}: {calls:?}"
            );
        }
        for alias in excluded {
            assert!(
                !calls.contains(&callback_aliases[*alias]),
                "{name} must prune {alias}: {calls:?}"
            );
        }
        generated
    };
    let integer_calls = callback_family(
        "DEMAND_OPEN_INTEGER_CALL",
        &[
            "DemandCallback",
            "DemandBoolCallback",
            "DemandFloatCallback",
            "DemandDoubleIntCallback",
            "DemandUnsignedCallback",
            "DemandLeafFactory",
            "DemandVoidResultCallback",
        ],
        &[
            "DemandRecordCallback",
            "DemandLeafCallback",
            "DemandConstLeafCallback",
            "DemandOtherCallback",
            "DemandVoidCallback",
            "DemandConstVoidCallback",
            "DemandCallbackArgument",
        ],
    );
    let leaf_calls = callback_family(
        "DEMAND_OPEN_LEAF_CALL",
        &[
            "DemandBoolCallback",
            "DemandLeafCallback",
            "DemandConstLeafCallback",
            "DemandVoidCallback",
            "DemandConstVoidCallback",
        ],
        &[
            "DemandCallback",
            "DemandFloatCallback",
            "DemandRecordCallback",
            "DemandOtherCallback",
            "DemandCallbackArgument",
        ],
    );
    let const_calls = callback_family(
        "DEMAND_OPEN_CONST_LEAF_CALL",
        &["DemandBoolCallback", "DemandConstLeafCallback", "DemandConstVoidCallback"],
        &[
            "DemandCallback",
            "DemandLeafCallback",
            "DemandOtherCallback",
            "DemandVoidCallback",
            "DemandCallbackArgument",
        ],
    );
    callback_family(
        "DEMAND_OPEN_VOID_CALL",
        &[
            "DemandBoolCallback",
            "DemandLeafCallback",
            "DemandConstLeafCallback",
            "DemandOtherCallback",
            "DemandVoidCallback",
            "DemandConstVoidCallback",
        ],
        &[
            "DemandCallback",
            "DemandFloatCallback",
            "DemandRecordCallback",
            "DemandCallbackArgument",
        ],
    );
    callback_family(
        "DEMAND_OPEN_RECORD_CALL",
        &["DemandRecordCallback"],
        &[
            "DemandCallback",
            "DemandBoolCallback",
            "DemandFloatCallback",
            "DemandLeafCallback",
            "DemandVoidCallback",
            "DemandCallbackArgument",
        ],
    );
    callback_family(
        "DEMAND_OPEN_NULL_CALL",
        &[
            "DemandCallback",
            "DemandFloatCallback",
            "DemandLeafCallback",
            "DemandConstLeafCallback",
            "DemandOtherCallback",
            "DemandVoidCallback",
            "DemandConstVoidCallback",
            "DemandCallbackArgument",
        ],
        &["DemandRecordCallback"],
    );
    let stored_calls = callback_family(
        "DEMAND_OPEN_STORED_INTEGER_CALL",
        &["DemandCallback", "DemandBoolCallback", "DemandFloatCallback"],
        &[
            "DemandRecordCallback",
            "DemandLeafCallback",
            "DemandConstLeafCallback",
            "DemandOtherCallback",
            "DemandVoidCallback",
            "DemandConstVoidCallback",
            "DemandCallbackArgument",
        ],
    );
    callback_family(
        "DEMAND_OPEN_INTEGER_RESULT_CALL",
        &[
            "DemandCallback",
            "DemandBoolCallback",
            "DemandDoubleIntCallback",
            "DemandLeafCallback",
            "DemandVoidCallback",
            "DemandCallbackArgument",
        ],
        &[
            "DemandFloatCallback",
            "DemandRecordCallback",
            "DemandLeafFactory",
            "DemandVoidResultCallback",
        ],
    );
    let unknown_calls = callback_family(
        "DEMAND_OPEN_CALLBACK_CALL",
        &[
            "DemandCallback",
            "DemandBoolCallback",
            "DemandFloatCallback",
            "DemandRecordCallback",
            "DemandLeafCallback",
            "DemandConstLeafCallback",
            "DemandOtherCallback",
            "DemandVoidCallback",
            "DemandConstVoidCallback",
            "DemandCallbackArgument",
            "DemandLeafFactory",
            "DemandVoidResultCallback",
        ],
        &[],
    );
    let two_sites =
        generate_with_bindings(&session, &["DEMAND_OPEN_TWO_SITES_CALL"], &catalog).unwrap();
    let union = callback_calls(&integer_calls.support.rust)
        .union(&callback_calls(&leaf_calls.support.rust))
        .cloned()
        .collect::<BTreeSet<_>>();
    assert_eq!(callback_calls(&two_sites.support.rust), union, "two sites retain their union");
    let two_macros = generate_with_bindings(
        &session,
        &["DEMAND_OPEN_INTEGER_CALL", "DEMAND_OPEN_LEAF_CALL"],
        &catalog,
    )
    .unwrap();
    assert_eq!(
        callback_calls(&two_macros.support.rust),
        union,
        "requested roots retain their union"
    );
    let unconstrained =
        generate_with_bindings(&session, &["DEMAND_OPEN_UNCONSTRAINED_SITE_CALL"], &catalog)
            .unwrap();
    assert_eq!(
        callback_calls(&unconstrained.support.rust),
        callback_calls(&unknown_calls.support.rust),
        "an unknown site reopens every eligible signature"
    );

    let identity =
        generate_with_bindings(&session, &["DEMAND_CALLBACK_IDENTITY"], &catalog).unwrap();
    let native_inputs = callback_native_inputs(&identity.support.rust, &catalog, &frontend);
    for alias in [
        "DemandCallback",
        "DemandBoolCallback",
        "DemandFloatCallback",
        "DemandDoubleIntCallback",
        "DemandUnsignedCallback",
        "DemandRecordCallback",
        "DemandLeafCallback",
        "DemandConstLeafCallback",
        "DemandOtherCallback",
        "DemandVoidCallback",
        "DemandConstVoidCallback",
        "DemandCallbackArgument",
        "DemandLeafFactory",
        "DemandVoidResultCallback",
    ] {
        assert!(native_inputs.contains(&callback_aliases[alias]), "raw identity input {alias}");
    }
    assert!(callback_calls(&identity.support.rust).is_empty(), "identity never calls a callback");
    for name in ["DEMAND_CALLBACK_SIZE", "DEMAND_CALLBACK_DISCARD"] {
        let generated = generate_with_bindings(&session, &[name], &catalog).unwrap();
        assert_eq!(
            callback_native_inputs(&generated.support.rust, &catalog, &frontend),
            native_inputs,
            "{name} still typechecks each eligible raw callback operand",
        );
        assert!(callback_calls(&generated.support.rust).is_empty());
    }
    for names in [
        ["DEMAND_CALLBACK_IDENTITY", "DEMAND_OPEN_INTEGER_CALL"],
        ["DEMAND_OPEN_INTEGER_CALL", "DEMAND_CALLBACK_IDENTITY"],
    ] {
        let generated = generate_with_bindings(&session, &names, &catalog).unwrap();
        assert_eq!(
            callback_native_inputs(&generated.support.rust, &catalog, &frontend),
            native_inputs,
            "open native identities retain their family regardless of root order",
        );
        assert_eq!(
            callback_calls(&generated.support.rust),
            callback_calls(&integer_calls.support.rust)
        );
    }
    let values = generate_with_bindings(
        &session,
        &["DEMAND_CALLBACK_IDENTITY", "DEMAND_CALLBACK_SIZE", "DEMAND_CALLBACK_DISCARD"],
        &catalog,
    )
    .unwrap();
    compare_calls(
        &values,
        r#"
unsafe extern "C" fn target(value: i32) -> i32 { value }
fn main() {
    let callback: DemandCallback = Some(target);
    let mut evaluated = 0;
    let identity = DEMAND_CALLBACK_IDENTITY!(callback).into_value().get().is_some();
    let size = DEMAND_CALLBACK_SIZE!(@__pgrx_c_expression; { evaluated += 1; callback }).get();
    DEMAND_CALLBACK_DISCARD!({ evaluated += 1; callback });
    println!("{} {size} {evaluated}", i32::from(identity));
}
"#,
        r#"
#include <stdio.h>
static int target(int value) { return value; }
int main(void) {
    DemandCallback callback = target;
    int evaluated = 0;
    int identity = DEMAND_CALLBACK_IDENTITY(callback) != NULL;
    size_t size = DEMAND_CALLBACK_SIZE((evaluated++, callback));
    DEMAND_CALLBACK_DISCARD((evaluated++, callback));
    printf("%d %zu %d\n", identity, size, evaluated);
}
"#,
    );

    let open_oracle = generate_with_bindings(
        &session,
        &[
            "DEMAND_OPEN_INTEGER_CALL",
            "DEMAND_OPEN_NULL_CALL",
            "DEMAND_OPEN_LEAF_CALL",
            "DEMAND_OPEN_CONST_LEAF_CALL",
            "DEMAND_OPEN_VOID_CALL",
            "DEMAND_OPEN_RECORD_CALL",
            "DEMAND_OPEN_STORED_INTEGER_CALL",
            "DEMAND_OPEN_INTEGER_RESULT_CALL",
            "DEMAND_OPEN_TWO_SITES_CALL",
        ],
        &catalog,
    )
    .unwrap();
    compare(
        &open_oracle,
        r#"
unsafe extern "C" fn integer(value: i32) -> i32 { value }
unsafe extern "C" fn floating(value: f64) -> f64 { value + 0.5 }
unsafe extern "C" fn boolean(value: bool) -> i32 { i32::from(value) }
unsafe extern "C" fn leaf(value: *mut DemandLeaf) -> i32 {
    // SAFETY: The oracle passes null or a live initialized DemandLeaf.
    if value.is_null() { 0 } else { unsafe { (*value).leaf as i32 } }
}
unsafe extern "C" fn frozen(value: *const DemandLeaf) -> i32 {
    // SAFETY: The oracle passes null or a live initialized DemandLeaf.
    if value.is_null() { 0 } else { unsafe { (*value).sibling as i32 } }
}
unsafe extern "C" fn pointer(value: *mut core::ffi::c_void) -> i32 { i32::from(!value.is_null()) }
unsafe extern "C" fn record(value: DemandLeaf) -> DemandLeaf { value }
unsafe extern "C" fn callback(value: DemandCallback) -> i32 {
    // SAFETY: A nonnull pointer supplied by this oracle has the int(int) ABI
    // and points to integer, which has no state or additional preconditions.
    value.map_or(0, |value| unsafe { value(53) })
}
fn main() {
    let integer: DemandCallback = Some(integer);
    let floating: DemandFloatCallback = Some(floating);
    let boolean: DemandBoolCallback = Some(boolean);
    let leaf: DemandLeafCallback = Some(leaf);
    let frozen: DemandConstLeafCallback = Some(frozen);
    let pointer: DemandVoidCallback = Some(pointer);
    let record: DemandRecordCallback = Some(record);
    let callback: DemandCallbackArgument = Some(callback);
    let mut value = DemandLeaf { leaf: 59, sibling: 61 };
    let address = &raw mut value;
    let mut selector = core::mem::MaybeUninit::<DemandSelector>::uninit();
    // SAFETY: Every callback has the inspected C ABI and no backend state.
    // Both DemandLeaf fields are initialized and its allocation remains live.
    // Only the initialized scalar selector.second is loaded, without creating
    // a reference or reading the rest of the partially initialized selector.
    // The record callback copies a fully initialized record before assume_init.
    unsafe {
        core::ptr::addr_of_mut!((*selector.as_mut_ptr()).second).write(67);
        println!("{},{:.1},{},{},{},{},{},{},{},{},{},{},{},{}",
            DEMAND_OPEN_INTEGER_CALL!(integer).get(), DEMAND_OPEN_INTEGER_CALL!(floating).get(),
            DEMAND_OPEN_INTEGER_CALL!(boolean).get(), DEMAND_OPEN_LEAF_CALL!(leaf, address).get(),
            DEMAND_OPEN_LEAF_CALL!(frozen, address).get(), DEMAND_OPEN_LEAF_CALL!(pointer, address).get(),
            DEMAND_OPEN_CONST_LEAF_CALL!(frozen, address).get(), DEMAND_OPEN_VOID_CALL!(leaf, address).get(),
            DEMAND_OPEN_NULL_CALL!(leaf).get(), DEMAND_OPEN_NULL_CALL!(callback).get(),
            DEMAND_OPEN_RECORD_CALL!(record, address).get().assume_init().sibling,
            DEMAND_OPEN_STORED_INTEGER_CALL!(integer, selector.as_mut_ptr()).get(),
            DEMAND_OPEN_INTEGER_RESULT_CALL!(integer, 70_i32).get(),
            DEMAND_OPEN_TWO_SITES_CALL!(integer, leaf, address).get());
    }
}
"#,
        r#"
#include <stdio.h>
static int integer(int value) { return value; }
static double floating(double value) { return value + 0.5; }
static int boolean(_Bool value) { return value; }
static int leaf(DemandLeaf *value) { return value ? (int)value->leaf : 0; }
static int frozen(const DemandLeaf *value) { return value ? (int)value->sibling : 0; }
static int pointer(void *value) { return value != (void *)0; }
static DemandLeaf record(DemandLeaf value) { return value; }
static int callback(DemandCallback value) { return value ? value(53) : 0; }
int main(void) {
    DemandLeaf value = {59, 61};
    DemandSelector selector;
    selector.second = 67;
    printf("%d,%.1f,%d,%d,%d,%d,%d,%d,%d,%d,%u,%d,%d,%d\n",
        DEMAND_OPEN_INTEGER_CALL(integer), DEMAND_OPEN_INTEGER_CALL(floating), DEMAND_OPEN_INTEGER_CALL(boolean),
        DEMAND_OPEN_LEAF_CALL(leaf, &value), DEMAND_OPEN_LEAF_CALL(frozen, &value), DEMAND_OPEN_LEAF_CALL(pointer, &value),
        DEMAND_OPEN_CONST_LEAF_CALL(frozen, &value), DEMAND_OPEN_VOID_CALL(leaf, &value),
        DEMAND_OPEN_NULL_CALL(leaf), DEMAND_OPEN_NULL_CALL(callback), DEMAND_OPEN_RECORD_CALL(record, &value).sibling,
        DEMAND_OPEN_STORED_INTEGER_CALL(integer, &selector), DEMAND_OPEN_INTEGER_RESULT_CALL(integer, 70),
        DEMAND_OPEN_TWO_SITES_CALL(integer, leaf, &value));
}
"#,
        1,
    );
    for (generated, rust_body, c_body) in [
        (
            &leaf_calls,
            "unsafe extern \"C\" fn target(_: *mut DemandUnrelated) -> i32 { 0 } fn main() { let target: DemandOtherCallback = Some(target); unsafe { let _ = DEMAND_OPEN_LEAF_CALL!(target, core::ptr::null_mut::<DemandLeaf>()); } }",
            "static int target(DemandUnrelated *value) { return value != 0; } int main(void) { return DEMAND_OPEN_LEAF_CALL(target, (DemandLeaf *)0); }",
        ),
        (
            &integer_calls,
            "unsafe extern \"C\" fn target(_: *mut DemandLeaf) -> i32 { 0 } fn main() { let target: DemandLeafCallback = Some(target); unsafe { let _ = DEMAND_OPEN_INTEGER_CALL!(target); } }",
            "static int target(DemandLeaf *value) { return value != 0; } int main(void) { return DEMAND_OPEN_INTEGER_CALL(target); }",
        ),
        (
            &const_calls,
            "unsafe extern \"C\" fn target(_: *mut DemandLeaf) -> i32 { 0 } fn main() { let target: DemandLeafCallback = Some(target); unsafe { let _ = DEMAND_OPEN_CONST_LEAF_CALL!(target, core::ptr::null_mut::<DemandLeaf>()); } }",
            "static int target(DemandLeaf *value) { return value != 0; } int main(void) { return DEMAND_OPEN_CONST_LEAF_CALL(target, (DemandLeaf *)0); }",
        ),
        (
            &stored_calls,
            "unsafe extern \"C\" fn target(_: *mut core::ffi::c_void) -> i32 { 0 } fn main() { let target: DemandVoidCallback = Some(target); let value = core::ptr::null_mut::<DemandSelector>(); unsafe { let _ = DEMAND_OPEN_STORED_INTEGER_CALL!(target, value); } }",
            "static int target(void *value) { return value != 0; } int main(void) { DemandSelector value; value.second = 0; return DEMAND_OPEN_STORED_INTEGER_CALL(target, &value); }",
        ),
    ] {
        let rejection = rust_oracle::reject_rust(&program(&bindings, generated, rust_body));
        assert!(rejection.contains("E0277"), "pruned invalid prototype must fail: {rejection}");
        let rejection = rust_oracle::reject_c_invocation(
            &frontend.profile().compiler.executable,
            &header,
            c_body,
            &frontend.profile().arguments.iter().map(String::as_str).collect::<Vec<_>>(),
        );
        assert!(!rejection.is_empty(), "original C rejects the invalid conversion");
    }

    let known = generate_with_bindings(
        &session,
        &[
            "DEMAND_OWNER_READ",
            "DEMAND_OWNER_READ_PROMOTED",
            "DEMAND_OWNER_SET_PROMOTED",
            "DEMAND_OWNER_READ_NESTED",
            "DEMAND_OWNER_SET_NESTED",
        ],
        &catalog,
    )
    .unwrap();
    let known_owners = adapter_owners(&known.support.rust, "Field");
    assert!(known_owners.contains("DemandOwner"));
    assert!(known_owners.contains("DemandChoice"));
    assert!(known_owners.contains("DemandLeaf"));
    assert!(
        !known_owners.contains("DemandUnrelated"),
        "known fields must not broaden other owners"
    );
    assert!(adapter_owners(&known.support.rust, "OffsetField").is_empty());
    compare(
        &known,
        r#"
fn main() {
    let mut owner = core::mem::MaybeUninit::<DemandOwner>::uninit();
    let pointer = owner.as_mut_ptr();
    // SAFETY: The pointer names live, aligned DemandOwner storage. Each accessed
    // scalar field is initialized before its read; no whole record or Rust
    // reference is created. The union's active members are promoted and child.
    unsafe {
        core::ptr::addr_of_mut!((*pointer).selected).write(11);
        DEMAND_OWNER_SET_PROMOTED!(pointer, 29_u32);
        DEMAND_OWNER_SET_NESTED!(pointer, 37_u32);
        println!("{},{},{}", DEMAND_OWNER_READ!(pointer).get(), DEMAND_OWNER_READ_PROMOTED!(pointer).get(), DEMAND_OWNER_READ_NESTED!(pointer).get());
    }
}
"#,
        r#"
#include <stdio.h>
int main(void) {
    DemandOwner owner;
    owner.selected = 11;
    DEMAND_OWNER_SET_PROMOTED(&owner, 29U);
    DEMAND_OWNER_SET_NESTED(&owner, 37U);
    printf("%u,%u,%u\n", DEMAND_OWNER_READ(&owner), DEMAND_OWNER_READ_PROMOTED(&owner), DEMAND_OWNER_READ_NESTED(&owner));
}
"#,
        1,
    );

    let unknown = generate_with_bindings(&session, &["DEMAND_UNKNOWN_READ"], &catalog).unwrap();
    let unknown_owners = adapter_owners(&unknown.support.rust, "Field");
    assert!(unknown_owners.contains("DemandOwner") && unknown_owners.contains("DemandUnrelated"));
    compare(
        &unknown,
        r#"
fn main() {
    let mut owner = core::mem::MaybeUninit::<DemandOwner>::uninit();
    let mut unrelated = core::mem::MaybeUninit::<DemandUnrelated>::uninit();
    let owner = owner.as_mut_ptr();
    let unrelated = unrelated.as_mut_ptr();
    // SAFETY: Both pointers name separate live, aligned allocations; selected
    // is initialized as u32 before each read, with no whole-record load.
    unsafe {
        core::ptr::addr_of_mut!((*owner).selected).write(41);
        core::ptr::addr_of_mut!((*unrelated).selected).write(43);
        println!("{},{}", DEMAND_UNKNOWN_READ!(owner).get(), DEMAND_UNKNOWN_READ!(unrelated).get());
    }
}
"#,
        r#"
#include <stdio.h>
int main(void) {
    DemandOwner owner;
    DemandUnrelated unrelated;
    owner.selected = 41;
    unrelated.selected = 43;
    printf("%u,%u\n", DEMAND_UNKNOWN_READ(&owner), DEMAND_UNKNOWN_READ(&unrelated));
}
"#,
        1,
    );

    let constrained =
        generate_with_bindings(&session, &["DEMAND_INTEGER_FIELD"], &catalog).unwrap();
    assert_eq!(
        adapter_owners(&constrained.support.rust, "Field"),
        ["DemandOwner", "DemandUnrelated", "DemandRejected", "DemandSigned"]
            .into_iter()
            .map(str::to_owned)
            .collect(),
        "integer-shift context must exclude floating, record, array and pointer field candidates"
    );
    compare(
        &constrained,
        r#"
fn main() {
    let mut owner = core::mem::MaybeUninit::<DemandOwner>::uninit();
    let mut signed = core::mem::MaybeUninit::<DemandSigned>::uninit();
    let owner = owner.as_mut_ptr();
    let signed = signed.as_mut_ptr();
    // SAFETY: Both pointers name separate live, aligned allocations. Their
    // selected scalar fields are initialized with the exact binding types;
    // generated field operations load only those initialized fields.
    unsafe {
        core::ptr::addr_of_mut!((*owner).selected).write(41);
        core::ptr::addr_of_mut!((*signed).selected).write(-4);
        println!("{},{}", DEMAND_INTEGER_FIELD!(owner).get(), DEMAND_INTEGER_FIELD!(signed).get());
    }
}
"#,
        r#"
#include <stdio.h>
int main(void) {
    DemandOwner owner;
    DemandSigned signed_value;
    owner.selected = 41;
    signed_value.selected = -4;
    printf("%u,%u\n", DEMAND_INTEGER_FIELD(&owner), DEMAND_INTEGER_FIELD(&signed_value));
}
"#,
        1,
    );

    let dynamic_offset =
        generate_with_bindings(&session, &["DEMAND_DYNAMIC_OFFSET"], &catalog).unwrap();
    let offset_fields = adapter_fields(&dynamic_offset.support.rust, "Field");
    for owner in ["DemandFloat", "DemandRecord", "DemandArray", "DemandPointer"] {
        assert!(
            !offset_fields.contains(&(owner.to_owned(), "selected".into())),
            "pointer-offset context must exclude {owner}'s nonintegral selected field"
        );
    }
    compare(
        &dynamic_offset,
        r#"
fn main() {
    let mut owner = core::mem::MaybeUninit::<DemandOwner>::uninit();
    let mut signed = core::mem::MaybeUninit::<DemandSigned>::uninit();
    let owner = owner.as_mut_ptr();
    let signed = signed.as_mut_ptr();
    // SAFETY: Both allocations are live and aligned; each selected scalar is
    // initialized before a load. Offset 4 stays within DemandOwner and is the
    // permitted one-past address of DemandSigned. Addresses are only compared.
    unsafe {
        core::ptr::addr_of_mut!((*owner).selected).write(4);
        core::ptr::addr_of_mut!((*signed).selected).write(4);
        let unsigned_offset = DEMAND_DYNAMIC_OFFSET!(owner, selected).get() == owner.cast::<u8>().add(4).cast();
        let signed_offset = DEMAND_DYNAMIC_OFFSET!(signed, selected).get() == signed.cast::<u8>().add(4).cast();
        core::ptr::addr_of_mut!((*signed).selected).write(0);
        println!("{},{},{}", u8::from(unsigned_offset), u8::from(signed_offset), u8::from(DEMAND_DYNAMIC_OFFSET!(signed, selected).get().is_null()));
    }
}
"#,
        r#"
#include <stdio.h>
int main(void) {
    DemandOwner owner;
    DemandSigned signed_value;
    owner.selected = 4;
    signed_value.selected = 4;
    int unsigned_offset = DEMAND_DYNAMIC_OFFSET(&owner, selected) == (void *)((char *)&owner + 4);
    int signed_offset = DEMAND_DYNAMIC_OFFSET(&signed_value, selected) == (void *)((char *)&signed_value + 4);
    signed_value.selected = 0;
    printf("%d,%d,%d\n", unsigned_offset, signed_offset, DEMAND_DYNAMIC_OFFSET(&signed_value, selected) == 0);
}
"#,
        1,
    );

    let next = generate_with_bindings(&session, &["DEMAND_NEXT"], &catalog).unwrap();
    assert_eq!(
        adapter_owners(&next.support.rust, "Field"),
        ["DemandStateA", "DemandStateB", "DemandListA", "DemandListB"]
            .into_iter()
            .map(str::to_owned)
            .collect(),
        "list traversal needs compatible state, length and element families"
    );
    let next_fields = adapter_fields(&next.support.rust, "Field");
    for owner in ["DemandStateA", "DemandStateB"] {
        for name in ["not_list", "missing_elements", "record_list", "wrong_index"] {
            assert!(
                !next_fields.contains(&(owner.to_owned(), name.to_owned())),
                "structural list context must reject {owner}::{name}"
            );
        }
    }
    assert!(next_fields.contains(&("DemandStateB".into(), "array_list".into())));
    compare(
        &next,
        r#"
fn main() {
    let mut list_a = DemandListA { length: 3, elements: [5, 7, 11, 13] };
    let mut list_b = DemandListB { length: 3, elements: [17, 19, 23] };
    let mut state_a = core::mem::MaybeUninit::<DemandStateA>::uninit();
    let mut state_b = core::mem::MaybeUninit::<DemandStateB>::uninit();
    let a = state_a.as_mut_ptr();
    let b = state_b.as_mut_ptr();
    let mut cell_a = core::ptr::null_mut::<i32>();
    let mut cell_b = core::ptr::null_mut::<u32>();
    // SAFETY: The arrays and state allocations remain live, aligned and
    // exclusively accessed. Every selected pointer/index field is initialized.
    // The nonnull branches use indices 1 and 2 below both length and array size;
    // their returned element pointers are valid for the following scalar reads.
    unsafe {
        core::ptr::addr_of_mut!((*a).first).write(&raw mut list_a);
        core::ptr::addr_of_mut!((*a).second).write(core::ptr::null_mut());
        core::ptr::addr_of_mut!((*a).index).write(1);
        core::ptr::addr_of_mut!((*b).list).write(&raw mut list_b);
        core::ptr::addr_of_mut!((*b).empty).write(core::ptr::null_mut());
        core::ptr::addr_of_mut!((*b).array_list).write([DemandListB { length: 3, elements: [29, 31, 37] }]);
        core::ptr::addr_of_mut!((*b).position).write(2);
        let first = *DEMAND_NEXT!(cell_a, (*a), first, index).get();
        let no_first = DEMAND_NEXT!(cell_a, (*a), second, index).get().is_null();
        let second = *DEMAND_NEXT!(cell_b, (*b), list, position).get();
        let no_second = DEMAND_NEXT!(cell_b, (*b), empty, position).get().is_null();
        let embedded = *DEMAND_NEXT!(cell_b, (*b), array_list, position).get();
        println!("{},{},{},{},{}", first, u8::from(no_first), second, u8::from(no_second), embedded);
    }
}
"#,
        r#"
#include <stdio.h>
int main(void) {
    DemandListA list_a = {3, {5, 7, 11, 13}};
    DemandListB list_b = {3, {17, 19, 23}};
    DemandStateA a;
    DemandStateB b;
    int *cell_a = 0;
    unsigned int *cell_b = 0;
    a.first = &list_a;
    a.second = 0;
    a.index = 1;
    b.list = &list_b;
    b.empty = 0;
    b.array_list[0] = (DemandListB) {3, {29, 31, 37}};
    b.position = 2;
    int first = *DEMAND_NEXT(cell_a, a, first, index);
    int no_first = DEMAND_NEXT(cell_a, a, second, index) == 0;
    unsigned int second = *DEMAND_NEXT(cell_b, b, list, position);
    int no_second = DEMAND_NEXT(cell_b, b, empty, position) == 0;
    unsigned int embedded = *DEMAND_NEXT(cell_b, b, array_list, position);
    printf("%d,%d,%u,%d,%u\n", first, no_first, second, no_second, embedded);
}
"#,
        1,
    );

    if frontend.profile().target.offsetof_supported {
        let offsets = generate_with_bindings(
            &session,
            &["DEMAND_OWNER_OFFSET", "DEMAND_NAMED_OFFSET"],
            &catalog,
        )
        .unwrap();
        let offset_owners = adapter_owners(&offsets.support.rust, "OffsetField");
        assert!(offset_owners.contains("DemandOwner"));
        assert!(offset_owners.contains("DemandChoice"));
        assert!(offset_owners.contains("DemandLeaf"));
        assert!(
            !offset_owners.contains("DemandUnrelated"),
            "offset closure must not follow pointers"
        );
        assert!(adapter_owners(&offsets.support.rust, "Field").is_empty());
        compare(
            &offsets,
            r#"
fn main() {
    println!("{},{},{},{},{},{},{}", DEMAND_OWNER_OFFSET!(selected).get(), DEMAND_OWNER_OFFSET!(choice.child.leaf).get(), DEMAND_OWNER_OFFSET!(promoted).get(), DEMAND_OWNER_OFFSET!(hidden.leaf).get(), DEMAND_OWNER_OFFSET!(frozen.leaf).get(), DEMAND_OWNER_OFFSET!(observed.leaf).get(), DEMAND_NAMED_OFFSET!().get());
}
"#,
            r#"
#include <stdio.h>
int main(void) {
    printf("%zu,%zu,%zu,%zu,%zu,%zu,%zu\n", DEMAND_OWNER_OFFSET(selected), DEMAND_OWNER_OFFSET(choice.child.leaf), DEMAND_OWNER_OFFSET(promoted), DEMAND_OWNER_OFFSET(hidden.leaf), DEMAND_OWNER_OFFSET(frozen.leaf), DEMAND_OWNER_OFFSET(observed.leaf), DEMAND_NAMED_OFFSET());
}
"#,
            1,
        );
        let generic =
            generate_with_bindings(&session, &["DEMAND_GENERIC_OFFSET"], &catalog).unwrap();
        assert!(adapter_owners(&generic.support.rust, "OffsetField").contains("DemandUnrelated"));
        compare(
            &generic,
            "fn main() { println!(\"{}\", DEMAND_GENERIC_OFFSET!(DemandUnrelated, unrelated_only).get()); }",
            "#include <stdio.h>\nint main(void) { printf(\"%zu\\n\", DEMAND_GENERIC_OFFSET(DemandUnrelated, unrelated_only)); }",
            1,
        );
    }

    let fixed_selector =
        generate_with_bindings(&session, &["DEMAND_FIXED_INTEGER_FIELD"], &catalog).unwrap();
    assert_eq!(
        adapter_fields(&fixed_selector.support.rust, "Field"),
        ["first", "second", "enumeration"]
            .into_iter()
            .map(|field| ("DemandSelector".into(), field.to_owned()))
            .collect(),
        "a known owner with a generic field selector retains only integer-compatible fields"
    );
    compare(
        &fixed_selector,
        r#"
fn main() {
    let mut selector = core::mem::MaybeUninit::<DemandSelector>::uninit();
    let pointer = selector.as_mut_ptr();
    // SAFETY: The pointer names aligned, live storage, and all accessed scalar
    // fields have valid initialized values. No whole-record load occurs.
    unsafe {
        core::ptr::addr_of_mut!((*pointer).first).write(20);
        core::ptr::addr_of_mut!((*pointer).second).write(28);
        core::ptr::addr_of_mut!((*pointer).enumeration).write(DemandIndex::DemandTwo);
        println!("{},{},{}", DEMAND_FIXED_INTEGER_FIELD!(pointer, first).get(), DEMAND_FIXED_INTEGER_FIELD!(pointer, second).get(), DEMAND_FIXED_INTEGER_FIELD!(pointer, enumeration).get());
    }
}
"#,
        r#"
#include <stdio.h>
int main(void) {
    DemandSelector selector;
    selector.first = 20;
    selector.second = 28;
    selector.enumeration = DemandTwo;
    printf("%u,%u,%u\n", DEMAND_FIXED_INTEGER_FIELD(&selector, first), DEMAND_FIXED_INTEGER_FIELD(&selector, second), DEMAND_FIXED_INTEGER_FIELD(&selector, enumeration));
}
"#,
        1,
    );

    let upstream_fields = |member: &str, source: &str| -> BTreeSet<(String, String)> {
        let roots = match source {
            "upstream" => vec!["DemandUpstreamA", "DemandUpstreamB", "DemandUpstreamArray"],
            "slot" => vec!["DemandUpstreamSlotA", "DemandUpstreamSlotB"],
            "fetch" => vec!["DemandUpstreamFetchA", "DemandUpstreamFetchB"],
            _ => panic!("fixture source family"),
        };
        roots
            .into_iter()
            .map(|owner| (owner.to_owned(), source.to_owned()))
            .chain(
                ["DemandUpstreamLeafA", "DemandUpstreamLeafB"]
                    .into_iter()
                    .map(|owner| (owner.to_owned(), member.to_owned())),
            )
            .collect()
    };
    for name in [
        "DEMAND_UPSTREAM_READ",
        "DEMAND_UPSTREAM_MEMBER",
        "DEMAND_UPSTREAM_INDEX",
        "DEMAND_UPSTREAM_REVERSE",
    ] {
        let generated = generate_with_bindings(&session, &[name], &catalog).unwrap();
        assert_eq!(
            adapter_fields(&generated.support.rust, "Field"),
            upstream_fields("payload", "upstream"),
            "{name}: upstream pointer/array declarations exclude unreachable same-named owners"
        );
    }
    for (name, source) in [("DEMAND_UPSTREAM_DEREF", "slot"), ("DEMAND_UPSTREAM_RESULT", "fetch")] {
        let generated = generate_with_bindings(&session, &[name], &catalog).unwrap();
        assert_eq!(
            adapter_fields(&generated.support.rust, "Field"),
            upstream_fields("payload", source),
            "{name}: dereference/callback results retain their exact upstream owner family"
        );
    }
    let upstream_calls =
        generate_with_bindings(&session, &["DEMAND_UPSTREAM_CALL"], &catalog).unwrap();
    assert_eq!(
        adapter_fields(&upstream_calls.support.rust, "Field"),
        upstream_fields("finish", "upstream"),
        "callback field owners must come from the upstream family, not every finish field"
    );
    assert_eq!(
        capability_types(&upstream_calls.support.rust, "Call").len(),
        2,
        "the unreachable int(double) callback must not receive an indirect-call implementation"
    );
    let upstream = generate_with_bindings(
        &session,
        &[
            "DEMAND_UPSTREAM_READ",
            "DEMAND_UPSTREAM_MEMBER",
            "DEMAND_UPSTREAM_SET",
            "DEMAND_UPSTREAM_INDEX",
            "DEMAND_UPSTREAM_REVERSE",
            "DEMAND_UPSTREAM_DEREF",
            "DEMAND_UPSTREAM_CALL",
            "DEMAND_UPSTREAM_RESULT",
        ],
        &catalog,
    )
    .unwrap();
    assert!(!adapter_owners(&upstream.support.rust, "Field").contains("DemandUpstreamUnreachable"));
    compare_calls(
        &upstream,
        r#"
fn main() {
    let mut leaves = [
        DemandUpstreamLeafA { payload: -31, finish: Some(demand_integer_function) },
        DemandUpstreamLeafA { payload: 37, finish: Some(demand_integer_function) },
    ];
    let frozen = [
        DemandUpstreamLeafB { payload: 41, finish: Some(demand_upstream_unsigned) },
        DemandUpstreamLeafB { payload: 43, finish: Some(demand_upstream_unsigned) },
    ];
    let root_a = DemandUpstreamA { upstream: leaves.as_mut_ptr() };
    let root_b = DemandUpstreamB { upstream: frozen.as_ptr() };
    let root_array = DemandUpstreamArray { upstream: [
        DemandUpstreamLeafA { payload: 47, finish: Some(demand_integer_function) },
        DemandUpstreamLeafA { payload: 53, finish: Some(demand_integer_function) },
    ] };
    let mut slot_a = leaves.as_mut_ptr();
    let mut slot_b = frozen.as_ptr();
    let root_slot_a = DemandUpstreamSlotA { slot: &mut slot_a };
    let root_slot_b = DemandUpstreamSlotB { slot: &mut slot_b };
    let fetch_a = DemandUpstreamFetchA { fetch: Some(demand_upstream_fetch_a) };
    let fetch_b = DemandUpstreamFetchB { fetch: Some(demand_upstream_fetch_b) };
    // SAFETY: Every root and pointee is initialized and lives through these
    // accesses. Both array indices are in bounds; const pointees are read only.
    // The linked callbacks have the inspected C ABIs, perform no backend work,
    // and return pointers to initialized C-owned static records that remain live.
    unsafe {
        let a = DEMAND_UPSTREAM_READ!(&root_a as *const DemandUpstreamA).get();
        let b = DEMAND_UPSTREAM_MEMBER!(&root_b as *const DemandUpstreamB).get();
        let c = DEMAND_UPSTREAM_INDEX!(&root_a as *const DemandUpstreamA, DemandIndex::DemandOne).get();
        let d = DEMAND_UPSTREAM_REVERSE!(&root_b as *const DemandUpstreamB, 1_i32).get();
        let e = DEMAND_UPSTREAM_INDEX!(&root_array as *const DemandUpstreamArray, 1_u32).get();
        let f = DEMAND_UPSTREAM_REVERSE!(&root_array as *const DemandUpstreamArray, 0_i32).get();
        let g = DEMAND_UPSTREAM_DEREF!(&root_slot_a as *const DemandUpstreamSlotA).get();
        let h = DEMAND_UPSTREAM_DEREF!(&root_slot_b as *const DemandUpstreamSlotB).get();
        let i = DEMAND_UPSTREAM_CALL!(&root_a as *const DemandUpstreamA, -59_i32).get();
        let j = DEMAND_UPSTREAM_CALL!(&root_b as *const DemandUpstreamB, 61_u32).get();
        let k = DEMAND_UPSTREAM_RESULT!(&fetch_a as *const DemandUpstreamFetchA, -67_i32).get();
        let l = DEMAND_UPSTREAM_RESULT!(&fetch_b as *const DemandUpstreamFetchB, 71_u32).get();
        let m = DEMAND_UPSTREAM_SET!(&root_a as *const DemandUpstreamA, 73_i32).get();
        println!("{a},{b},{c},{d},{e},{f},{g},{h},{i},{j},{k},{l},{m},{}", leaves[0].payload);
    }
}
"#,
        r#"
#include <stdio.h>
int main(void) {
    DemandUpstreamLeafA leaves[2] = {{-31, demand_integer_function}, {37, demand_integer_function}};
    const DemandUpstreamLeafB frozen[2] = {{41, demand_upstream_unsigned}, {43, demand_upstream_unsigned}};
    DemandUpstreamA root_a = {leaves};
    DemandUpstreamB root_b = {frozen};
    DemandUpstreamArray root_array = {{{47, demand_integer_function}, {53, demand_integer_function}}};
    DemandUpstreamLeafA *slot_a = leaves;
    const DemandUpstreamLeafB *slot_b = frozen;
    DemandUpstreamSlotA root_slot_a = {&slot_a};
    DemandUpstreamSlotB root_slot_b = {&slot_b};
    DemandUpstreamFetchA fetch_a = {demand_upstream_fetch_a};
    DemandUpstreamFetchB fetch_b = {demand_upstream_fetch_b};
    int a = DEMAND_UPSTREAM_READ(&root_a);
    unsigned int b = DEMAND_UPSTREAM_MEMBER(&root_b);
    int c = DEMAND_UPSTREAM_INDEX(&root_a, DemandOne);
    unsigned int d = DEMAND_UPSTREAM_REVERSE(&root_b, 1);
    int e = DEMAND_UPSTREAM_INDEX(&root_array, 1U);
    int f = DEMAND_UPSTREAM_REVERSE(&root_array, 0);
    int g = DEMAND_UPSTREAM_DEREF(&root_slot_a);
    unsigned int h = DEMAND_UPSTREAM_DEREF(&root_slot_b);
    int i = DEMAND_UPSTREAM_CALL(&root_a, -59);
    unsigned int j = DEMAND_UPSTREAM_CALL(&root_b, 61U);
    int k = DEMAND_UPSTREAM_RESULT(&fetch_a, -67);
    unsigned int l = DEMAND_UPSTREAM_RESULT(&fetch_b, 71U);
    int m = DEMAND_UPSTREAM_SET(&root_a, 73);
    printf("%d,%u,%d,%u,%d,%d,%d,%u,%d,%u,%d,%u,%d,%d\n", a,b,c,d,e,f,g,h,i,j,k,l,m,leaves[0].payload);
}
"#,
    );
    let read_only = rust_oracle::reject_rust(&program(
        &bindings,
        &upstream,
        "fn main() { let leaf = DemandUpstreamLeafB { payload: 0, finish: None }; let root = DemandUpstreamB { upstream: &leaf }; unsafe { let _ = DEMAND_UPSTREAM_SET!(&root as *const DemandUpstreamB, 1_u32); } }",
    ));
    assert!(read_only.contains("E0277") && read_only.contains("ReadOnly"), "{read_only}");
    let read_only = rust_oracle::reject_c_invocation(
        &frontend.profile().compiler.executable,
        &header,
        "int main(void) { const DemandUpstreamLeafB leaf = {0}; DemandUpstreamB root = {&leaf}; return DEMAND_UPSTREAM_SET(&root, 1U); }",
        &frontend.profile().arguments.iter().map(String::as_str).collect::<Vec<_>>(),
    );
    assert!(
        read_only.contains("read-only") || read_only.contains("const-qualified"),
        "{read_only}"
    );

    let record_index =
        generate_with_bindings(&session, &["DEMAND_INDEX_RECORD"], &catalog).unwrap();
    assert_eq!(
        adapter_owners(&record_index.support.rust, "Field"),
        ["DemandOwner", "DemandUnrelated", "DemandRejected", "DemandSigned"]
            .into_iter()
            .map(str::to_owned)
            .collect(),
        "both subscript orientations must retain compatible record pointees and prune decoys"
    );
    compare(
        &record_index,
        r#"
fn main() {
    let mut unsigned = [DemandUnrelated { selected: 18, promoted: 0, unrelated_only: 0 }, DemandUnrelated { selected: 22, promoted: 0, unrelated_only: 0 }];
    let mut signed = [DemandSigned { selected: 26 }, DemandSigned { selected: 30 }];
    let unsigned = unsigned.as_mut_ptr();
    let signed = signed.as_mut_ptr();
    // SAFETY: All pointer operands name initialized record arrays with two
    // elements; integer and enum offsets are 0 or 1, so every read is in bounds.
    unsafe {
        println!("{},{},{},{}", DEMAND_INDEX_RECORD!(0_i32, unsigned).get(), DEMAND_INDEX_RECORD!(unsigned, 1_u32).get(), DEMAND_INDEX_RECORD!(DemandIndex::DemandOne, signed).get(), DEMAND_INDEX_RECORD!(signed, DemandIndex::DemandZero).get());
    }
}
"#,
        r#"
#include <stdio.h>
int main(void) {
    DemandUnrelated unsigned_records[2] = {{18, 0, 0}, {22, 0, 0}};
    DemandSigned signed_records[2] = {{26}, {30}};
    printf("%u,%u,%d,%d\n", DEMAND_INDEX_RECORD(0, unsigned_records), DEMAND_INDEX_RECORD(unsigned_records, 1U), DEMAND_INDEX_RECORD(DemandOne, signed_records), DEMAND_INDEX_RECORD(signed_records, DemandZero));
}
"#,
        1,
    );

    let integer_index =
        generate_with_bindings(&session, &["DEMAND_INDEX_INTEGER"], &catalog).unwrap();
    assert!(adapter_owners(&integer_index.support.rust, "Field").is_empty());
    compare(
        &integer_index,
        r#"
fn main() {
    let mut values = [12_u32, 20];
    let mut enumerations = [DemandIndex::DemandZero, DemandIndex::DemandTwo];
    let values = values.as_mut_ptr();
    let enumerations = enumerations.as_mut_ptr();
    // SAFETY: Both arrays are initialized and all offsets are 0 or 1. Enum
    // reads preserve actual enum storage and integer promotion before shifting.
    unsafe {
        println!("{},{},{},{}", DEMAND_INDEX_INTEGER!(DemandIndex::DemandOne, values).get(), DEMAND_INDEX_INTEGER!(values, 0_i32).get(), DEMAND_INDEX_INTEGER!(1_i32, enumerations).get(), DEMAND_INDEX_INTEGER!(enumerations, DemandIndex::DemandZero).get());
    }
}
"#,
        r#"
#include <stdio.h>
int main(void) {
    unsigned int values[2] = {12, 20};
    DemandIndex enumerations[2] = {DemandZero, DemandTwo};
    printf("%u,%u,%u,%u\n", DEMAND_INDEX_INTEGER(DemandOne, values), DEMAND_INDEX_INTEGER(values, 0), DEMAND_INDEX_INTEGER(1, enumerations), DEMAND_INDEX_INTEGER(enumerations, DemandZero));
}
"#,
        1,
    );

    let known_reverse =
        generate_with_bindings(&session, &["DEMAND_KNOWN_REVERSE_INDEX"], &catalog).unwrap();
    assert_eq!(
        adapter_owners(&known_reverse.support.rust, "Field"),
        ["DemandLeaf".into()].into(),
        "the right-hand compiler-declared pointee fixes field ownership"
    );
    compare(
        &known_reverse,
        r#"
fn main() {
    let leaves = [DemandLeaf { leaf: 31, sibling: 0 }, DemandLeaf { leaf: 37, sibling: 0 }];
    // SAFETY: The cast retains const qualification; both reads are within the
    // initialized two-element array, whose lifetime covers both invocations.
    unsafe {
        println!("{},{}", DEMAND_KNOWN_REVERSE_INDEX!(0_i32, leaves.as_ptr()).get(), DEMAND_KNOWN_REVERSE_INDEX!(DemandIndex::DemandOne, leaves.as_ptr()).get());
    }
}
"#,
        r#"
#include <stdio.h>
int main(void) {
    const DemandLeaf leaves[2] = {{31, 0}, {37, 0}};
    printf("%u,%u\n", DEMAND_KNOWN_REVERSE_INDEX(0, leaves), DEMAND_KNOWN_REVERSE_INDEX(DemandOne, leaves));
}
"#,
        1,
    );

    let invalid_index = program(
        &bindings,
        &record_index,
        "fn main() { let mut value = DemandSigned { selected: 1 }; unsafe { let _ = DEMAND_INDEX_RECORD!(1_f64, &mut value as *mut DemandSigned); } }",
    );
    let rejection = rust_oracle::reject_rust(&invalid_index);
    assert!(rejection.contains("Subscript"), "floating indices must be rejected: {rejection}");
    rust_oracle::reject_c_invocation(
        &frontend.profile().compiler.executable,
        &header,
        "int main(void) { DemandSigned value = {1}; return DEMAND_INDEX_RECORD(1.0, &value); }",
        &frontend.profile().arguments.iter().map(String::as_str).collect::<Vec<_>>(),
    );

    let direct_integer =
        generate_with_bindings(&session, &["DEMAND_DIRECT_INTEGER"], &catalog).unwrap();
    assert_eq!(
        capability_types(&direct_integer.support.rust, "NativeType"),
        ["DemandIndex".into()].into(),
        "an arithmetic prototype must not retain unrelated record or function-pointer bridges"
    );
    assert!(adapter_owners(&direct_integer.support.rust, "Field").is_empty());
    compare_calls(
        &direct_integer,
        r#"
fn main() {
    // SAFETY: The linked C function has the inspected int(int) ABI, performs
    // no backend work, and accepts each finite converted int value.
    unsafe {
        println!("{},{},{},{}", DEMAND_DIRECT_INTEGER!(-11_i32).get(), DEMAND_DIRECT_INTEGER!(25_u32).get(), DEMAND_DIRECT_INTEGER!(-13.75_f64).get(), DEMAND_DIRECT_INTEGER!(DemandIndex::DemandTwo).get());
    }
}
"#,
        r#"
#include <stdio.h>
int main(void) {
    printf("%d,%d,%d,%d\n", DEMAND_DIRECT_INTEGER(-11), DEMAND_DIRECT_INTEGER(25U), DEMAND_DIRECT_INTEGER(-13.75), DEMAND_DIRECT_INTEGER(DemandTwo));
}
"#,
    );

    let known_callback =
        generate_with_bindings(&session, &["DEMAND_KNOWN_CALLBACK"], &catalog).unwrap();
    assert_eq!(
        capability_types(&known_callback.support.rust, "NativeType"),
        ["DemandIndex".into()].into(),
        "a known callback keeps arithmetic operands, without unrelated record bridges"
    );
    assert_eq!(
        capability_types(&known_callback.support.rust, "Call").len(),
        1,
        "only the known callback signature needs a callable adapter"
    );
    compare_calls(
        &known_callback,
        r#"
fn main() {
    // SAFETY: The linked native global contains the matching non-null int(int)
    // callback throughout this single-threaded program; conversions stay finite.
    unsafe {
        println!("{},{},{},{}", DEMAND_KNOWN_CALLBACK!(-17_i32).get(), DEMAND_KNOWN_CALLBACK!(29_u32).get(), DEMAND_KNOWN_CALLBACK!(31.75_f64).get(), DEMAND_KNOWN_CALLBACK!(DemandIndex::DemandOne).get());
    }
}
"#,
        r#"
#include <stdio.h>
int main(void) {
    printf("%d,%d,%d,%d\n", DEMAND_KNOWN_CALLBACK(-17), DEMAND_KNOWN_CALLBACK(29U), DEMAND_KNOWN_CALLBACK(31.75), DEMAND_KNOWN_CALLBACK(DemandOne));
}
"#,
    );

    let conversions = generate_with_bindings(
        &session,
        &[
            "DEMAND_DIRECT_BOOL",
            "DEMAND_FUNCTION_VALUE",
            "DEMAND_DIRECT_POINTER",
            "DEMAND_CAST_POINTER",
            "DEMAND_DIRECT_RECORD",
        ],
        &catalog,
    )
    .unwrap();
    compare_calls(
        &conversions,
        r#"
fn main() {
    let mut leaf = DemandLeaf { leaf: 41, sibling: 43 };
    let pointer = &mut leaf as *mut DemandLeaf;
    // SAFETY: The linked functions have the inspected fixed ABIs, require no
    // backend state, and only copy scalar/record values or compare pointers.
    // The initialized record and its allocation remain live for every call.
    // The native echo copies both initialized u32 fields, so its MaybeUninit
    // result contains a valid DemandLeaf before assume_init materializes it.
    unsafe {
        println!("{},{},{},{},{},{},{},{},{}", DEMAND_DIRECT_BOOL!(0_i32).get(), DEMAND_DIRECT_BOOL!(-2.5_f64).get(), DEMAND_DIRECT_BOOL!(pointer).get(), DEMAND_DIRECT_BOOL!(DEMAND_FUNCTION_VALUE!()).get(), DEMAND_DIRECT_POINTER!(0).get(), DEMAND_DIRECT_POINTER!(pointer).get(), DEMAND_CAST_POINTER!(pointer).get(), DEMAND_DIRECT_RECORD!(leaf).get().assume_init().leaf, DEMAND_DIRECT_RECORD!(leaf).get().assume_init().sibling);
    }
}
"#,
        r#"
#include <stdio.h>
int main(void) {
    DemandLeaf leaf = {41, 43};
    printf("%d,%d,%d,%d,%d,%d,%d,%u,%u\n", DEMAND_DIRECT_BOOL(0), DEMAND_DIRECT_BOOL(-2.5), DEMAND_DIRECT_BOOL(&leaf), DEMAND_DIRECT_BOOL(DEMAND_FUNCTION_VALUE()), DEMAND_DIRECT_POINTER(0), DEMAND_DIRECT_POINTER(&leaf), DEMAND_CAST_POINTER(&leaf), DEMAND_DIRECT_RECORD(leaf).leaf, DEMAND_DIRECT_RECORD(leaf).sibling);
}
"#,
    );

    let prototype_families = [
        (
            "DEMAND_LEAF_POINTER",
            vec![
                "DemandLeafPointer",
                "DemandConstLeafPointer",
                "DemandVoidPointer",
                "DemandLeafArray",
            ],
        ),
        ("DEMAND_MUTABLE_LEAF", vec!["DemandLeafPointer", "DemandVoidPointer", "DemandLeafArray"]),
        ("DEMAND_NESTED_POINTER", vec!["DemandNestedPointer", "DemandVoidPointer"]),
        (
            "DEMAND_UNSIGNED_POINTER",
            vec!["DemandPointer", "DemandArray", "DemandIndexPointer", "DemandVoidPointer"],
        ),
        ("DEMAND_CALLBACK_POINTER", vec!["DemandCallbackPointer"]),
        ("DEMAND_RECORD_FIELD", vec!["DemandRecord", "DemandRecordLeaf"]),
        ("DEMAND_ASSIGN_LEAF", vec!["DemandRecord", "DemandRecordLeaf"]),
    ];
    for (name, owners) in prototype_families {
        let generated = generate_with_bindings(&session, &[name], &catalog).unwrap();
        assert_eq!(
            adapter_owners(&generated.support.rust, "Field"),
            owners.into_iter().map(str::to_owned).collect(),
            "{name} must retain only fields compatible with its declared destination"
        );
    }
    let precise = generate_with_bindings(
        &session,
        &[
            "DEMAND_LEAF_POINTER",
            "DEMAND_MUTABLE_LEAF",
            "DEMAND_NESTED_POINTER",
            "DEMAND_UNSIGNED_POINTER",
            "DEMAND_CALLBACK_POINTER",
            "DEMAND_RECORD_FIELD",
            "DEMAND_ASSIGN_LEAF",
        ],
        &catalog,
    )
    .unwrap();
    compare_calls(
        &precise,
        r#"
fn main() {
    let mut leaf = DemandLeaf { leaf: 53, sibling: 59 };
    let pointer = &mut leaf as *mut DemandLeaf;
    let leaf_pointer = DemandLeafPointer { selected: pointer };
    let const_pointer = DemandConstLeafPointer { selected: pointer.cast_const() };
    let void_pointer = DemandVoidPointer { selected: pointer.cast() };
    let leaf_array = DemandLeafArray { selected: [DemandLeaf { leaf: 53, sibling: 59 }; 2] };
    let mut nested = pointer;
    let nested_pointer = DemandNestedPointer { selected: &mut nested };
    let mut enumeration = DemandIndex::DemandTwo;
    let enum_pointer = DemandIndexPointer { selected: &mut enumeration };
    let callback = DemandCallbackPointer { selected: Some(demand_integer_function) };
    let record = DemandRecordLeaf { selected: DemandLeaf { leaf: 53, sibling: 59 } };
    let mut destination = core::mem::MaybeUninit::<DemandLeaf>::uninit();
    // SAFETY: Each pointer reaches aligned, initialized live storage. The
    // fixtures require no backend state. The enum's compatible integer access
    // does not write or fabricate an invalid Rust variant. The assignment
    // initializes both destination fields before its whole-record observation.
    unsafe {
        let a = DEMAND_LEAF_POINTER!(&leaf_pointer as *const DemandLeafPointer).get();
        let b = DEMAND_LEAF_POINTER!(&const_pointer as *const DemandConstLeafPointer).get();
        let c = DEMAND_LEAF_POINTER!(&void_pointer as *const DemandVoidPointer).get();
        let d = DEMAND_LEAF_POINTER!(&leaf_array as *const DemandLeafArray).get();
        let e = DEMAND_MUTABLE_LEAF!(&leaf_pointer as *const DemandLeafPointer).get();
        let f = DEMAND_NESTED_POINTER!(&nested_pointer as *const DemandNestedPointer).get();
        let g = DEMAND_UNSIGNED_POINTER!(&enum_pointer as *const DemandIndexPointer).get();
        let h = DEMAND_CALLBACK_POINTER!(&callback as *const DemandCallbackPointer).get();
        let i = DEMAND_RECORD_FIELD!(&record as *const DemandRecordLeaf).get().assume_init().leaf;
        let j = DEMAND_ASSIGN_LEAF!(&record as *const DemandRecordLeaf, destination.as_mut_ptr()).get().assume_init().sibling;
        println!("{a},{b},{c},{d},{e},{f},{g},{h},{i},{j}");
    }
}
"#,
        r#"
#include <stdio.h>
int main(void) {
    DemandLeaf leaf = {53, 59};
    DemandLeafPointer leaf_pointer = {&leaf};
    DemandConstLeafPointer const_pointer = {&leaf};
    DemandVoidPointer void_pointer = {&leaf};
    DemandLeafArray leaf_array = {{leaf, leaf}};
    DemandLeaf *nested = &leaf;
    DemandNestedPointer nested_pointer = {&nested};
    DemandIndex enumeration = DemandTwo;
    DemandIndexPointer enum_pointer = {&enumeration};
    DemandCallbackPointer callback = {demand_integer_function};
    DemandRecordLeaf record = {leaf};
    DemandLeaf destination;
    int a = DEMAND_LEAF_POINTER(&leaf_pointer);
    int b = DEMAND_LEAF_POINTER(&const_pointer);
    int c = DEMAND_LEAF_POINTER(&void_pointer);
    int d = DEMAND_LEAF_POINTER(&leaf_array);
    int e = DEMAND_MUTABLE_LEAF(&leaf_pointer);
    int f = DEMAND_NESTED_POINTER(&nested_pointer);
    int g = DEMAND_UNSIGNED_POINTER(&enum_pointer);
    int h = DEMAND_CALLBACK_POINTER(&callback);
    unsigned int i = DEMAND_RECORD_FIELD(&record).leaf;
    unsigned int j = DEMAND_ASSIGN_LEAF(&record, &destination).sibling;
    printf("%d,%d,%d,%d,%d,%d,%d,%d,%u,%u\n", a, b, c, d, e, f, g, h, i, j);
}
"#,
    );
    for (rust_body, c_body) in [
        (
            "let value = DemandOtherPointer { selected: core::ptr::null_mut() }; let _ = DEMAND_LEAF_POINTER!(&value as *const DemandOtherPointer);",
            "DemandOtherPointer value = {0}; return DEMAND_LEAF_POINTER(&value);",
        ),
        (
            "let value = DemandConstLeafPointer { selected: core::ptr::null() }; let _ = DEMAND_MUTABLE_LEAF!(&value as *const DemandConstLeafPointer);",
            "DemandConstLeafPointer value = {0}; return DEMAND_MUTABLE_LEAF(&value);",
        ),
        (
            "let value = DemandVolatileLeafPointer { selected: core::ptr::null_mut() }; let _ = DEMAND_LEAF_POINTER!(&value as *const DemandVolatileLeafPointer);",
            "DemandVolatileLeafPointer value = {0}; return DEMAND_LEAF_POINTER(&value);",
        ),
        (
            "let value = DemandNestedConstPointer { selected: core::ptr::null_mut() }; let _ = DEMAND_NESTED_POINTER!(&value as *const DemandNestedConstPointer);",
            "DemandNestedConstPointer value = {0}; return DEMAND_NESTED_POINTER(&value);",
        ),
        (
            "let value = DemandSigned { selected: 0 }; let _ = DEMAND_LEAF_POINTER!(&value as *const DemandSigned);",
            "DemandSigned value = {0}; return DEMAND_LEAF_POINTER(&value);",
        ),
    ] {
        rust_oracle::reject_rust(&program(
            &bindings,
            &precise,
            &format!("fn main() {{ unsafe {{ {rust_body} }} }}"),
        ));
        rust_oracle::reject_c_invocation(
            &frontend.profile().compiler.executable,
            &header,
            &format!("int main(void) {{ {c_body} }}"),
            &frontend.profile().arguments.iter().map(String::as_str).collect::<Vec<_>>(),
        );
    }

    let invalid_integer = generate_with_bindings(
        &session,
        &["DEMAND_DIRECT_INTEGER", "DEMAND_FUNCTION_VALUE"],
        &catalog,
    )
    .unwrap();
    let invalid_call = program(
        &bindings,
        &invalid_integer,
        "fn main() { unsafe { let _ = DEMAND_DIRECT_INTEGER!(DEMAND_FUNCTION_VALUE!()); } }",
    );
    let rejection = rust_oracle::reject_rust(&invalid_call);
    assert!(
        rejection.contains("ImplicitTo"),
        "a function pointer cannot implicitly convert to int: {rejection}"
    );
    rust_oracle::reject_c_invocation(
        &frontend.profile().compiler.executable,
        &header,
        "int main(void) { return DEMAND_DIRECT_INTEGER(DEMAND_FUNCTION_VALUE()); }",
        &frontend.profile().arguments.iter().map(String::as_str).collect::<Vec<_>>(),
    );

    let rejected =
        generate_with_bindings(&session, &["DEMAND_LITERAL", "DEMAND_REJECTED"], &catalog).unwrap();
    assert!(matches!(rejected.macros[0].status, EmissionStatus::Emitted { .. }));
    assert!(matches!(rejected.macros[1].status, EmissionStatus::Skipped { .. }));
    assert!(
        rejected.support.rust.is_empty(),
        "a rejected root must not retain native capabilities"
    );
    assert!(rejected.support.c_source.is_empty());
}
