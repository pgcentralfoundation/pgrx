//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

#[path = "../../pgrx-bindgen/src/build/binding_symbols.rs"]
mod binding_symbols;
#[path = "support/oracle.rs"]
mod oracle;
#[path = "support/rust_oracle.rs"]
#[allow(dead_code)]
mod rust_oracle;

use pgrx_c_macros::{
    AnalysisSession, EmissionStatus, MacroScanner, emit_batch_with_bindings,
    emit_support_artifact_with_bindings, inspect,
};
use quote::ToTokens;
use std::path::PathBuf;

fn is_builtin_oid(name: &str) -> bool {
    name.ends_with("OID") && name != "HEAP_HASOID"
        || name.ends_with("RelationId")
        || name == "TemplateDbOid"
}

const NAMES: &[&str] = &[
    "RECORD_HEADER",
    "RECORD_BYTE",
    "RECORD_SET_BYTE",
    "RECORD_HEADER_ADDRESS",
    "RECORD_CHILD_SIZE",
    "RECORD_PROMOTED",
    "RECORD_SET_PROMOTED",
    "RECORD_FLEX",
    "RECORD_FLEX_ADDRESS",
    "RECORD_SET_FLEX",
    "RECORD_OPAQUE",
    "RECORD_OPAQUE_INLINE",
    "RECORD_OPAQUE_COMPARE",
    "RECORD_OPAQUE_VOID",
    "RECORD_ANON_INLINE",
    "RECORD_ARRAY",
    "RECORD_VOLATILE_READ",
    "RECORD_VOLATILE_ADDRESS",
    "RECORD_VOLATILE_INDIRECT",
    "RECORD_PACKED_ARRAY",
    "RECORD_SET_PACKED_ARRAY",
    "RECORD_PACKED_FLEX",
    "RECORD_FLEX_ARRAY_ADDRESS",
    "RECORD_FLEX_ARRAY_EQUAL",
    "RECORD_FLEX_ARRAY_VOID",
    "RECORD_CAST_ONLY",
    "RECORD_VOLATILE_WHOLE_ADDRESS",
    "RECORD_VOLATILE_WHOLE_SIZE",
    "RECORD_VOLATILE_CAST_FIELD",
    "RECORD_VOLATILE_ASSIGNMENT_SIZE",
];

#[test]
fn compiler_anchored_record_places_match_c_without_reading_uninitialized_fields() {
    let scanner = MacroScanner::new().expect("libclang must be available");
    let header = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/record_oracle.h");
    let mut arguments = vec!["-std=c17".into(), "-ffp-contract=off".into()];
    #[cfg(target_os = "macos")]
    {
        let sdk = rust_oracle::run_tool(
            std::process::Command::new("xcrun").arg("--show-sdk-path"),
            "locate record oracle SDK",
        );
        arguments.extend(["-isysroot".into(), sdk.trim().into()]);
    }
    let frontend =
        inspect(&scanner, &header, &arguments, None).expect("inspect record declarations");
    let rejected = [
        "RECORD_OPAQUE_SIZE",
        "RECORD_OPAQUE_OFFSET",
        "RECORD_OPAQUE_READ",
        "RECORD_FLEX_SIZE",
        "RECORD_FLEX_ARRAY_STEP",
    ];
    let names = NAMES.iter().copied().chain(rejected).collect::<Vec<_>>();
    let names = names
        .into_iter()
        .chain([
            "RECORD_VOLATILE_WHOLE",
            "RECORD_VOLATILE_WHOLE_SET",
            "RECORD_VOLATILE_WHOLE_ROUNDTRIP",
            "RECORD_VOLATILE_WHOLE_SELECT",
            "RECORD_VOLATILE_WHOLE_SELECT_MIXED",
        ])
        .collect::<Vec<_>>();
    let session =
        AnalysisSession::prepare(&scanner, &frontend, &names).expect("parse record macros");
    let bindings = bindgen::Builder::default()
        .rust_target(bindgen::RustTarget::stable(85, 0).unwrap())
        .rust_edition(bindgen::RustEdition::Edition2024)
        .header(header.to_str().unwrap())
        .clang_args(&frontend.profile().arguments)
        .allowlist_type(".*Record")
        .allowlist_function("opaque_identity")
        .manually_drop_union(".*")
        .use_core()
        .wrap_unsafe_ops(true)
        .layout_tests(false)
        .generate_comments(false)
        .formatter(bindgen::Formatter::None)
        .generate()
        .expect("generate record storage")
        .to_string();
    let catalog = binding_symbols::collect_bindings(
        &syn::parse_file(&bindings).unwrap(),
        session.integer_constants(),
        frontend.declarations(),
        &frontend.profile().target,
    );
    assert!(matches!(
        catalog.records["AnonymousRecord"].fields["expanded"].ty,
        pgrx_c_macros::RustBindingType::ManuallyDrop { .. }
    ));
    assert!(matches!(
        catalog.records["FlexibleRecord"].fields["items"].ty,
        pgrx_c_macros::RustBindingType::IncompleteArrayField { .. }
    ));
    let artifact = emit_support_artifact_with_bindings(&session, NAMES, &catalog)
        .expect("derive verified record capabilities");
    assert!(artifact.rust.contains("COpaque<Self>"));
    assert!(artifact.rust.contains("CFlexibleArray"));
    let isolated = emit_support_artifact_with_bindings(&session, &["RECORD_CAST_ONLY"], &catalog)
        .expect("cast-only record bridge");
    assert!(
        isolated.rust.contains("NativeType for crate::CastOnlyRecord"),
        "a record solely behind a typedef pointer cast needs its native adapter: {}",
        isolated.rust
    );
    let support = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../pgrx-pg-sys/src/c_macros/support.rs")
        .canonicalize()
        .unwrap();
    let base = format!(
        "#![deny(unsafe_op_in_unsafe_fn)]\n#![allow(non_snake_case,non_camel_case_types,unused_parens,dead_code)]\n#[path={support:?}] pub mod __pgrx_c_macros;\n{bindings}\n{}\n",
        artifact.rust
    );
    let mut rust = base.clone();
    for emission in emit_batch_with_bindings(&session, NAMES, &catalog) {
        let EmissionStatus::Emitted { rust: definition, .. } = emission.status else {
            panic!("record macro must emit: {emission:?}");
        };
        rust.push_str(&definition);
    }
    rust.push_str(include_str!("fixtures/record_oracle.rs"));
    let profile = frontend.profile();
    let arguments = profile.arguments.iter().map(String::as_str).collect::<Vec<_>>();
    let native = "OpaqueRecord *opaque_identity(OpaqueRecord *pointer) { return pointer; }\n";
    let generated = rust_oracle::run_rust_linked(
        &rust,
        &profile.compiler.executable,
        &header,
        &format!("{}\n{native}", artifact.c_source),
        &arguments,
    );
    let original = oracle::run_c(
        &profile.compiler.executable,
        &header,
        &format!("{native}\n{}", include_str!("fixtures/record_oracle.c")),
        &arguments,
        true,
    );
    assert_eq!(generated, original);
    for (name, original_invocation) in [
        (
            "RECORD_VOLATILE_WHOLE",
            "void valid(void *pointer) { CastOnlyRecord copy=RECORD_VOLATILE_WHOLE(pointer); (void)copy; }",
        ),
        (
            "RECORD_VOLATILE_WHOLE_SET",
            "void valid(void *pointer, CastOnlyRecord value) { (void)RECORD_VOLATILE_WHOLE_SET(pointer,value); }",
        ),
        (
            "RECORD_VOLATILE_WHOLE_ROUNDTRIP",
            "void valid(void *pointer) { CastOnlyRecord copy=RECORD_VOLATILE_WHOLE_ROUNDTRIP(pointer); (void)copy; }",
        ),
        (
            "RECORD_VOLATILE_WHOLE_SELECT",
            "void valid(void *pointer) { CastOnlyRecord copy=RECORD_VOLATILE_WHOLE_SELECT(pointer); (void)copy; }",
        ),
        (
            "RECORD_VOLATILE_WHOLE_SELECT_MIXED",
            "void valid(void *pointer) { CastOnlyRecord copy=RECORD_VOLATILE_WHOLE_SELECT_MIXED(pointer); (void)copy; }",
        ),
    ] {
        let emission = emit_batch_with_bindings(&session, &[name], &catalog).remove(0);
        let EmissionStatus::Skipped { reason } = emission.status else {
            panic!("unsupported forced volatile aggregate access must skip: {emission:?}");
        };
        assert!(
            reason.message.contains("volatile whole-record")
                && reason.message.contains("compiler-verified"),
            "specific volatile aggregate reason: {reason:?}"
        );
        rust_oracle::run_rust_linked(
            "fn main() {}",
            &profile.compiler.executable,
            &header,
            original_invocation,
            &arguments,
        );
    }
    let mut compiler = std::process::Command::new(&profile.compiler.executable);
    compiler
        .args(&arguments)
        .args(["-x", "c", "-S", "-emit-llvm", "-O1", "-include"])
        .arg(&header)
        .arg(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/record_oracle.c"))
        .args(["-o", "-"]);
    let llvm =
        rust_oracle::run_tool(&mut compiler, "inspect original packed-array access alignment");
    for (name, operation) in [
        ("record_oracle_packed_load", "load i32"),
        ("record_oracle_packed_store", "store i32"),
        ("record_oracle_packed_flex_load", "load i32"),
    ] {
        let body = llvm
            .lines()
            .skip_while(|line| !line.starts_with("define ") || !line.contains(&format!("@{name}(")))
            .take_while(|line| *line != "}")
            .collect::<Vec<_>>();
        assert!(
            body.iter().any(|line| line.contains(operation) && line.contains("align 1")),
            "original Clang packed access must use alignment1: {body:?}"
        );
    }
    for name in rejected {
        let original_invocation = match name {
            "RECORD_OPAQUE_OFFSET" => {
                "void rejected(OpaqueRecord *p) { (void)RECORD_OPAQUE_OFFSET(p, 1); }"
            }
            "RECORD_OPAQUE_SIZE" => {
                "void rejected(OpaqueRecord *p) { (void)RECORD_OPAQUE_SIZE(p); }"
            }
            "RECORD_OPAQUE_READ" => {
                "void rejected(OpaqueRecord *p) { (void)RECORD_OPAQUE_READ(p); }"
            }
            "RECORD_FLEX_ARRAY_STEP" => {
                "void rejected(FlexibleRecord *p) { (void)RECORD_FLEX_ARRAY_STEP(p, 1); }"
            }
            _ => "void rejected(FlexibleRecord *p) { (void)RECORD_FLEX_SIZE(p); }",
        };
        rust_oracle::reject_c_invocation(
            &profile.compiler.executable,
            &header,
            original_invocation,
            &arguments,
        );
        let emission = emit_batch_with_bindings(&session, &[name], &catalog).remove(0);
        if let EmissionStatus::Emitted { rust: definition, .. } = emission.status {
            let invocation = match name {
                "RECORD_OPAQUE_OFFSET" => {
                    "RECORD_OPAQUE_OFFSET!(core::ptr::null_mut::<OpaqueRecord>(), 1)"
                }
                "RECORD_OPAQUE_SIZE" => {
                    "RECORD_OPAQUE_SIZE!(core::ptr::null_mut::<OpaqueRecord>())"
                }
                "RECORD_OPAQUE_READ" => {
                    "RECORD_OPAQUE_READ!(core::ptr::null_mut::<OpaqueRecord>())"
                }
                "RECORD_FLEX_ARRAY_STEP" => {
                    "RECORD_FLEX_ARRAY_STEP!(core::ptr::null_mut::<FlexibleRecord>(), 1)"
                }
                _ => "RECORD_FLEX_SIZE!(core::ptr::null_mut::<FlexibleRecord>())",
            };
            let error = rust_oracle::reject_rust(&format!(
                "{base}\n{definition}\nfn main() {{ unsafe {{ let _ = {invocation}; }} }}"
            ));
            assert!(
                error.contains("CompleteObject")
                    || error.contains("ReadObject")
                    || error.contains("ScalarObject"),
                "{name}: {error}"
            );
        }
    }
    let read = emit_batch_with_bindings(&session, &["RECORD_HEADER"], &catalog).remove(0);
    let EmissionStatus::Emitted { rust: definition, .. } = read.status else {
        panic!("field macro")
    };
    let error = rust_oracle::reject_rust(&format!(
        "{base}\n{definition}\nfn main() {{ let _ = RECORD_HEADER!(core::ptr::null_mut::<AnonymousRecord>()); }}"
    ));
    assert!(error.contains("unsafe"), "field load must require unsafe: {error}");

    let equality =
        emit_batch_with_bindings(&session, &["RECORD_FLEX_ARRAY_EQUAL"], &catalog).remove(0);
    let EmissionStatus::Emitted { rust: definition, .. } = equality.status else {
        panic!("incomplete-array equality")
    };
    let error = rust_oracle::reject_rust(&format!(
        "{base}\n{definition}\nfn main() {{ unsafe {{ let _ = RECORD_FLEX_ARRAY_EQUAL!(core::ptr::null_mut::<FlexibleRecord>(), core::ptr::null_mut::<[i32; 3]>()); }} }}"
    ));
    assert!(
        error.contains("CompatibleIdentity"),
        "different C array element types must be rejected: {error}"
    );
    rust_oracle::reject_c_invocation(
        &profile.compiler.executable,
        &header,
        "void rejected(FlexibleRecord *p, int (*other)[3]) { (void)RECORD_FLEX_ARRAY_EQUAL(p, other); }",
        &arguments,
    );

    // A field-name match alone cannot bless a different actual record layout.
    let mut changed = syn::parse_file(&bindings).unwrap();
    for item in &mut changed.items {
        if let syn::Item::Struct(record) = item
            && record.ident == "FlexibleRecord"
        {
            for field in &mut record.fields {
                if field.ident.as_ref().is_some_and(|name| name == "count") {
                    field.ty = syn::parse_quote!(u8);
                }
            }
        }
    }
    let changed_catalog = binding_symbols::collect_bindings(
        &changed,
        session.integer_constants(),
        frontend.declarations(),
        &profile.target,
    );
    let changed_artifact =
        emit_support_artifact_with_bindings(&session, &["RECORD_FLEX"], &changed_catalog).unwrap();
    let changed_emission =
        emit_batch_with_bindings(&session, &["RECORD_FLEX"], &changed_catalog).remove(0);
    let EmissionStatus::Emitted { rust: definition, .. } = changed_emission.status else {
        panic!("layout assertions must remain attached to field adapter")
    };
    let error = rust_oracle::reject_rust(&format!(
        "#![allow(non_camel_case_types,dead_code)]\n#[path={support:?}] pub mod __pgrx_c_macros;\n{}\n{}\n{definition}\nfn main() {{}}",
        changed.to_token_stream(),
        changed_artifact.rust
    ));
    assert!(
        error.contains("E0080") && error.contains("offset_of!"),
        "layout mismatch must fail its compiler-owned assertion: {error}"
    );

    // Two actual anonymous child candidates remain ambiguous even if their
    // individual layouts and fields coincide. Neither is chosen by its name.
    let mut changed = syn::parse_file(&bindings).unwrap();
    for item in &mut changed.items {
        if let syn::Item::Struct(record) = item
            && record.ident == "PromotedRecord"
        {
            let syn::Fields::Named(fields) = &mut record.fields else {
                panic!("named bindgen record")
            };
            let mut duplicate = fields
                .named
                .iter()
                .find(|field| field.ident.as_ref().is_some_and(|name| name == "__bindgen_anon_1"))
                .unwrap()
                .clone();
            duplicate.ident = Some(syn::parse_quote!(__indistinguishable_child));
            fields.named.push(duplicate);
        }
    }
    let changed_catalog = binding_symbols::collect_bindings(
        &changed,
        session.integer_constants(),
        frontend.declarations(),
        &profile.target,
    );
    let changed_emission =
        emit_batch_with_bindings(&session, &["RECORD_PROMOTED"], &changed_catalog).remove(0);
    assert!(
        matches!(changed_emission.status, EmissionStatus::Skipped { .. }),
        "ambiguous compiler-to-binding identity cannot be inferred: {changed_emission:?}"
    );
}
