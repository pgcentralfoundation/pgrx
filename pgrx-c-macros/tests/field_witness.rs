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
    AnalysisSession, EmissionStatus, MacroScanner, generate_with_bindings, inspect,
};
use std::path::PathBuf;

fn is_builtin_oid(name: &str) -> bool {
    name.ends_with("OID") && name != "HEAP_HASOID"
        || name.ends_with("RelationId")
        || name == "TemplateDbOid"
}

#[test]
fn unused_field_witnesses_check_exact_storage_and_eager_layout() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let header = root.join("tests/fixtures/field_witness.h");
    let support = root.join("../pgrx-pg-sys/src/c_macros/support.rs").canonicalize().unwrap();
    let mut arguments = vec!["-std=c11".into()];
    #[cfg(target_os = "macos")]
    {
        let sdk = rust_oracle::run_tool(
            std::process::Command::new("xcrun").arg("--show-sdk-path"),
            "locate the C witness oracle SDK",
        );
        arguments.extend(["-isysroot".into(), sdk.trim().into()]);
    }
    let scanner = MacroScanner::new().unwrap();
    let frontend = inspect(&scanner, &header, &arguments, None).unwrap();
    let names = [
        "WITNESS_SCALAR",
        "WITNESS_POINTER",
        "WITNESS_ARRAY",
        "WITNESS_CALLBACK",
        "WITNESS_FROZEN",
        "WITNESS_PROMOTED",
        "WITNESS_WRAPPED",
        "WITNESS_ANCESTOR",
    ];
    let session = AnalysisSession::prepare(&scanner, &frontend, &names).unwrap();
    let native = bindgen::Builder::default()
        .header(header.to_str().unwrap())
        .clang_args(&arguments)
        .allowlist_type("Witness.*")
        .derive_copy(false)
        .manually_drop_union("WitnessWrapped")
        .layout_tests(false)
        .generate_comments(false)
        .generate()
        .unwrap()
        .to_string();
    let original = syn::parse_file(&native).unwrap();
    let catalog = binding_symbols::collect_bindings(
        &original,
        session.integer_constants(),
        frontend.declarations(),
        &frontend.profile().target,
    );
    let generated = generate_with_bindings(&session, &names, &catalog).unwrap();
    assert!(generated.support.c_source.is_empty());
    let mut macros = String::new();
    for emission in generated.macros {
        let EmissionStatus::Emitted { rust, .. } = emission.status else {
            panic!("original field macro must emit: {emission:?}")
        };
        macros.push_str(&rust);
    }
    let adapters = generated.support.rust;
    assert!(adapters.contains("Projection<crate::WitnessPlain, u32>"));
    assert!(!adapters.contains("const WITNESS:"));
    assert!(adapters.contains("ManuallyDrop<crate::WitnessPayload>"));
    assert!(adapters.contains(
        "Projection<crate::WitnessWrapped, ::core::mem::ManuallyDrop<crate::WitnessPayload>>"
    ));
    assert!(
        adapters
            .contains("Projection<crate::WitnessPromoted, crate::WitnessPromoted__bindgen_ty_1>")
    );

    for (member, replacement) in [
        ("scalar", syn::parse_quote!(i32)),
        ("pointer", syn::parse_quote!(*const ::std::os::raw::c_uint)),
        ("array", syn::parse_quote!([i32; 2])),
        ("callback", syn::parse_quote!(Option<unsafe extern "C-unwind" fn(u32) -> u32>)),
    ] {
        let mut edited = original.clone();
        let record = edited
            .items
            .iter_mut()
            .find_map(|item| match item {
                syn::Item::Struct(record) if record.ident == "WitnessPlain" => Some(record),
                _ => None,
            })
            .unwrap();
        record
            .fields
            .iter_mut()
            .find(|field| field.ident.as_ref().is_some_and(|name| name == member))
            .unwrap()
            .ty = replacement;
        let diagnostics = rust_oracle::reject_rust(&format!(
            "#[path={support:?}] pub mod __pgrx_c_macros; {} {adapters} fn main() {{}}",
            quote::quote!(#edited),
        ));
        assert!(diagnostics.contains("E0308"), "{member}: {diagnostics}");
        assert!(!diagnostics.contains("E0080"), "equal layout must pass: {diagnostics}");
    }

    let mut reordered = original.clone();
    let record = reordered
        .items
        .iter_mut()
        .find_map(|item| match item {
            syn::Item::Struct(record) if record.ident == "WitnessPlain" => Some(record),
            _ => None,
        })
        .unwrap();
    let mut fields = record.fields.iter().cloned().collect::<Vec<_>>();
    fields.swap(1, 3);
    let syn::Fields::Named(named) = &mut record.fields else { unreachable!() };
    named.named = fields.into_iter().collect();
    let diagnostics = rust_oracle::reject_rust(&format!(
        "#[path={support:?}] pub mod __pgrx_c_macros; {} {adapters} fn main() {{}}",
        quote::quote!(#reordered),
    ));
    assert!(diagnostics.contains("E0080"), "unused offsets must stay eager: {diagnostics}");
    assert!(!diagnostics.contains("E0308"), "field storage is unchanged: {diagnostics}");

    for (ty, member, nested) in [
        ("WitnessPlain", "frozen", None),
        ("WitnessPromoted", "promoted", None),
        ("WitnessAncestor", "const_payload", Some("value")),
    ] {
        let mut place =
            format!("project::<__pgrx_c_field_marker!({member}), _, _>(pointee(input(pointer)))");
        if let Some(nested) = nested {
            place = format!("project::<__pgrx_c_field_marker!({nested}), _, _>({place})");
        }
        let diagnostics = rust_oracle::reject_rust(&format!(
            r#"#[path={support:?}] pub mod __pgrx_c_macros;
{native} {adapters} {macros}
use __pgrx_c_macros::expression::{{WritePlace, project, pointee, input}};
fn main() {{
    let pointer = core::ptr::null_mut::<{ty}>();
    unsafe {{ {place}.store(__pgrx_c_macros::CValue::new(7u32)); }}
}}"#,
        ));
        assert!(diagnostics.contains("ReadOnly") && diagnostics.contains("store"), "{diagnostics}");
    }

    let actual = rust_oracle::run_rust(&format!(
        r#"#[path={support:?}] pub mod __pgrx_c_macros;
{native} {adapters} {macros}
fn main() {{
    let mut pointed = 11u32;
    let mut record = WitnessPlain {{ scalar: 3, pointer: &raw mut pointed, array: [5, 7], callback: None, frozen: 13 }};
    unsafe {{ println!("{{}},{{}},{{}},{{}}", WITNESS_SCALAR!(&raw mut record).get(), *WITNESS_POINTER!(&raw mut record).get(), *WITNESS_ARRAY!(&raw mut record).get(), usize::from(WITNESS_CALLBACK!(&raw mut record).get().is_some())); }}
}}"#,
    ));
    let expected = oracle::run_c(
        &frontend.profile().compiler.executable,
        &header,
        r#"#include <stdio.h>
int main(void) {
    unsigned int pointed = 11;
    WitnessPlain record = {3, &pointed, {5, 7}, 0, 13};
    printf("%u,%u,%u,%d\n", WITNESS_SCALAR(&record), *WITNESS_POINTER(&record), *WITNESS_ARRAY(&record), WITNESS_CALLBACK(&record) != 0);
}"#,
        &arguments.iter().map(String::as_str).collect::<Vec<_>>(),
        true,
    );
    assert_eq!(actual, expected);
}
