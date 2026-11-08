//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! A translated static inline function gets a Rust function with its binding's name and
//! signature that runs the translation, after the catalog's thread check when the translation
//! touches a global. Functions whose translation falls back, or whose values need native
//! transport, keep their binding.

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
    AnalysisSession, EmissionStatus, MacroScanner, generate_with_bindings, inspect,
};
use std::path::PathBuf;

/// Preserve the production OID classification boundary in the shared fixture collector.
fn is_builtin_oid(name: &str) -> bool {
    name.ends_with("OID") && name != "HEAP_HASOID"
        || name.ends_with("RelationId")
        || name == "TemplateDbOid"
}

/// Inline functions the fixture selects.
const NAMES: &[&str] = &[
    "WrapAdd",
    "WrapNext",
    "WrapStore",
    "WrapKeyword",
    "WrapBump",
    "WrapBumpTwice",
    "WrapShadeRank",
    "WrapNative",
];

/// The fixture header.
fn header() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/inline_functions.h")
}

/// Rewrite foreign functions as Rust functions that call them through the fixture's FFI
/// guard, as pgrx's `#[pg_guard]` rewrites the production bindings after cataloging them.
fn guard(bindings: &str) -> String {
    let mut file = syn::parse_file(bindings).unwrap();
    let mut items = Vec::new();
    for item in std::mem::take(&mut file.items) {
        let syn::Item::ForeignMod(mut block) = item else {
            items.push(item);
            continue;
        };
        let (functions, statics) = std::mem::take(&mut block.items)
            .into_iter()
            .partition::<Vec<_>, _>(|item| matches!(item, syn::ForeignItem::Fn(_)));
        let syn::ItemForeignMod { unsafety, abi, .. } = &block;
        for item in &functions {
            let syn::ForeignItem::Fn(function) = item else { unreachable!() };
            let syn::Signature { ident, inputs, output, .. } = &function.sig;
            let arguments = inputs.iter().map(|input| match input {
                syn::FnArg::Typed(input) => &input.pat,
                syn::FnArg::Receiver(_) => unreachable!("foreign functions take no receiver"),
            });
            items.push(syn::parse_quote! {
                pub unsafe fn #ident(#inputs) #output {
                    #unsafety #abi { #function }
                    unsafe { crate::ffi::pg_guard_ffi_boundary(move || #ident(#(#arguments),*)) }
                }
            });
        }
        if !statics.is_empty() {
            block.items = statics;
            items.push(syn::Item::ForeignMod(block));
        }
    }
    file.items = items;
    quote::quote!(#file).to_string()
}

#[test]
fn translated_inline_functions_replace_their_bindings() {
    let scanner = MacroScanner::new().expect("libclang must be available");
    let mut arguments = oracle::native_arguments();
    arguments.push("-std=c17".into());
    let frontend = inspect(&scanner, &header(), &arguments, None).expect("inspect definitions");
    let directory =
        std::env::temp_dir().join(format!("pgrx-inline-functions-{}", std::process::id()));
    std::fs::create_dir_all(&directory).unwrap();
    // Production bindings call static inline functions through bindgen's C wrappers.
    let bindings = bindgen::Builder::default()
        .rust_target(bindgen::RustTarget::stable(85, 0).unwrap())
        .rust_edition(bindgen::RustEdition::Edition2024)
        .header(header().to_str().unwrap())
        .clang_args(&frontend.profile().arguments)
        .allowlist_type("Wrap.*")
        .allowlist_function("Wrap.*")
        .allowlist_var("Wrap.*")
        .wrap_static_fns(true)
        .wrap_static_fns_path(directory.join("wrappers"))
        .wrap_static_fns_suffix("__pgrx_cshim")
        .layout_tests(false)
        .generate_comments(false)
        .formatter(bindgen::Formatter::None)
        .generate()
        .expect("generate bindings")
        .to_string();
    std::fs::remove_dir_all(&directory).unwrap();
    let session = AnalysisSession::prepare_with_inline_functions(
        &scanner,
        &frontend,
        &[] as &[String],
        &[] as &[String],
        NAMES,
    )
    .expect("prepare roots");
    let mut catalog = binding_symbols::collect_bindings(
        &syn::parse_file(&bindings).unwrap(),
        session.integer_constants(),
        frontend.declarations(),
        &frontend.profile().target,
    );
    // The functions need both the thread check they keep and the guard the translations'
    // native calls keep.
    catalog.ffi_boundary = Some(vec!["ffi".into(), "pg_guard_ffi_boundary".into()]);
    assert!(generate_with_bindings(&session, NAMES, &catalog).unwrap().functions.is_empty());
    catalog.thread_check = Some(vec!["thread_check".into()]);
    let boundary = catalog.ffi_boundary.take();
    assert!(generate_with_bindings(&session, NAMES, &catalog).unwrap().functions.is_empty());
    catalog.ffi_boundary = boundary;
    let generated = generate_with_bindings(&session, NAMES, &catalog).unwrap();
    for name in ["WrapAdd", "WrapNext", "WrapStore", "WrapKeyword", "WrapBump", "WrapBumpTwice"] {
        assert!(generated.functions.contains(&format!("pub unsafe fn {name}(")), "{name}");
    }
    // Only the two translations that write the global check the thread.
    assert_eq!(generated.functions.matches("thread_check()").count(), 2);
    assert!(generated.functions.contains("r#type: u32"), "keyword parameter");
    for name in ["WrapShadeRank", "WrapNative"] {
        assert!(!generated.functions.contains(&format!("fn {name}(")), "{name}");
    }

    let support = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../pgrx-pg-sys/src/c_macros/support.rs")
        .canonicalize()
        .unwrap();
    let mut rust = format!(
        "#![deny(unsafe_op_in_unsafe_fn)]\n#![allow(non_snake_case,non_camel_case_types,non_upper_case_globals,unused_parens,dead_code)]\n#[path={support:?}] pub mod __pgrx_c_macros;\n{}\n{}\n",
        guard(&bindings),
        generated.support.rust
    );
    for emission in &generated.macros {
        let EmissionStatus::Emitted { rust: definition, .. } = &emission.status else {
            panic!("unexpected emission: {emission:?}");
        };
        rust.push_str(definition);
    }
    rust.push_str(&generated.functions);
    rust.push_str(include_str!("fixtures/inline_functions.rs"));
    let profile = frontend.profile();
    let arguments = profile.arguments.iter().map(String::as_str).collect::<Vec<_>>();
    let output = rust_oracle::run_rust_linked(
        &rust,
        &profile.compiler.executable,
        &header(),
        &format!("{}\nint WrapCounter = 5;\n", generated.support.c_source),
        &arguments,
    );
    assert_eq!(
        output,
        "add\t5\nnext\t1\nstore\t7\nkeyword\t42\nbump\t7\nbump twice\t9\nchecks\t2\n"
    );
    session.verify_inputs().expect("original profile stayed unchanged");
}
