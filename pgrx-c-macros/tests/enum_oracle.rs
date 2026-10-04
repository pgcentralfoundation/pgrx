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
    AnalysisSession, EmissionStatus, MacroScanner, TypeShapeKind, emit_batch_with_bindings,
    emit_support_artifact_with_bindings, inspect,
};
use std::path::PathBuf;

fn is_builtin_oid(name: &str) -> bool {
    name.ends_with("OID") && name != "HEAP_HASOID"
        || name.ends_with("RelationId")
        || name == "TemplateDbOid"
}

const NAMES: &[&str] = &[
    "ENUM_READ",
    "ENUM_SET",
    "ENUM_ADD",
    "ENUM_POST",
    "ENUM_PREFIX",
    "ENUM_CAST",
    "ENUM_NOT",
    "ENUM_SIZE",
    "ENUM_ADDRESS",
    "ENUM_INDIRECT",
    "ENUM_VOLATILE",
    "ENUM_VOLATILE_SET",
    "ENUM_SIGNED",
    "ENUM_SIGNED_SET",
    "ENUM_WIDE",
    "ENUM_WIDE_SET",
    "ENUM_PACKED",
    "ENUM_PACKED_SET",
    "ENUM_IDENTITY",
    "ENUM_COMPARE",
    "ENUM_DISTINCT",
    "ENUM_INLINE",
    "ENUM_KEYWORD",
    "ENUM_KEYWORD_SET",
    "ENUM_PRIMITIVE_NAME",
    "ENUM_CONSTANT",
    "ENUM_CONSTANT_USE",
    "ENUM_CONSTANT_ZERO",
    "ENUM_ORDER",
    "ENUM_DIFFERENCE",
    "ENUM_QUALIFIED_POINTER",
];

#[test]
fn enum_integer_semantics_and_raw_object_access_match_clang() {
    let scanner = MacroScanner::new().expect("libclang must be available");
    for short_enums in [false, true] {
        check_profile(&scanner, short_enums);
    }
}

fn check_profile(scanner: &MacroScanner, short_enums: bool) {
    let header = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/enum_oracle.h");
    let mut arguments = vec!["-std=c17".into(), "-ffp-contract=off".into()];
    if short_enums {
        arguments.push("-fshort-enums".into());
    }
    #[cfg(target_os = "macos")]
    {
        let sdk = rust_oracle::run_tool(
            std::process::Command::new("xcrun").arg("--show-sdk-path"),
            "locate enum oracle SDK",
        );
        arguments.extend(["-isysroot".into(), sdk.trim().into()]);
    }
    let frontend =
        inspect(scanner, &header, &arguments, None).expect("inspect compiler enum identities");
    let session = AnalysisSession::prepare(scanner, &frontend, NAMES).expect("parse enum macros");
    let bindings = bindgen::Builder::default()
        .rust_target(bindgen::RustTarget::stable(85, 0).unwrap())
        .rust_edition(bindgen::RustEdition::Edition2024)
        .header(header.to_str().unwrap())
        .clang_args(&frontend.profile().arguments)
        .allowlist_type("Enum.*|PackedEnumRecord")
        .rustified_enum(".*")
        .use_core()
        .wrap_unsafe_ops(true)
        .layout_tests(false)
        .generate_comments(false)
        .formatter(bindgen::Formatter::None)
        .generate()
        .expect("generate actual enum storage")
        .to_string();
    let catalog = binding_symbols::collect_bindings(
        &syn::parse_file(&bindings).unwrap(),
        session.integer_constants(),
        frontend.declarations(),
        &frontend.profile().target,
    );
    assert!(catalog.enums.contains_key("EnumSmall"));
    let artifact = emit_support_artifact_with_bindings(&session, NAMES, &catalog)
        .expect("derive raw enum capabilities");
    assert!(artifact.rust.contains("EnumStorage"));
    let support = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../pgrx-pg-sys/src/c_macros/support.rs")
        .canonicalize()
        .unwrap();
    let mut base = format!(
        "#![deny(unsafe_op_in_unsafe_fn)]\n#![allow(non_snake_case,non_camel_case_types,unused_parens,dead_code)]\n#[path={support:?}] pub mod __pgrx_c_macros;\n{bindings}\n{}\n",
        artifact.rust
    );
    let mut c_types = String::new();
    for (name, alias) in [("EnumSmall", "EnumSmallStorage"), ("EnumSigned", "EnumSignedStorage")] {
        let ty = &frontend.declarations().types[name];
        let TypeShapeKind::Enum { underlying: Some(underlying) } =
            &frontend.declarations().type_shapes[&ty.canonical_spelling].kind
        else {
            panic!("compiler enum must establish compatible type");
        };
        let pgrx_c_macros::TypeCategory::Integer(kind) = underlying.category else {
            panic!("compatible enum integer");
        };
        let integer = &frontend.profile().target.integers[&kind];
        base.push_str(&format!(
            "type {alias} = {}{};\n",
            if integer.signed { 'i' } else { 'u' },
            integer.bits
        ));
        c_types.push_str(&format!("typedef {} {alias};\n_Static_assert(__builtin_types_compatible_p({name},{alias}),\"compiler enum compatibility\");\n", underlying.canonical_spelling));
    }
    for emission in emit_batch_with_bindings(&session, NAMES, &catalog).unwrap() {
        let EmissionStatus::Emitted { rust, .. } = emission.status else {
            panic!("enum macro must emit: {emission:?}");
        };
        if emission.analysis.name == "ENUM_CONSTANT" {
            assert!(
                rust.contains("$crate::EnumSmall::SmallOne"),
                "enum constant must retain its actual symbol: {rust}"
            );
            assert!(
                !rust.contains("PGRX:"),
                "verified enum constant needs no resolved-value fallback: {rust}"
            );
        }
        base.push_str(&rust);
    }
    let rust = format!("{base}\n{}", include_str!("fixtures/enum_oracle.rs"));
    let profile = frontend.profile();
    let arguments = profile.arguments.iter().map(String::as_str).collect::<Vec<_>>();
    let generated = rust_oracle::run_rust_linked(
        &rust,
        &profile.compiler.executable,
        &header,
        &artifact.c_source,
        &arguments,
    );
    let original = oracle::run_c(
        &profile.compiler.executable,
        &header,
        &format!("{c_types}\n{}", include_str!("fixtures/enum_oracle.c")),
        &arguments,
        true,
    );
    assert_eq!(original.lines().count(), 293, "enum and compatible-pointer corpus completeness");
    assert_eq!(generated, original, "enum semantics for short_enums={short_enums}");
    let rejection = rust_oracle::reject_rust(&format!(
        "{base}\nfn main() {{ let mut object=core::mem::MaybeUninit::<EnumRecord>::uninit(); let mut other=EnumOther::OtherOne; unsafe {{ let _=ENUM_DISTINCT!(object.as_mut_ptr(), &raw mut other); }} }}"
    ));
    assert!(
        rejection.contains("CompatibleIdentity"),
        "different enum pointers must be rejected: {rejection}"
    );
    let rejection = rust_oracle::reject_c_invocation(
        &profile.compiler.executable,
        &header,
        "void rejected(EnumRecord *object, EnumOther *other) { (void)ENUM_DISTINCT(object,other); }",
        &arguments,
    );
    assert!(
        rejection.contains("distinct pointer types"),
        "original C rejects different enum identities: {rejection}"
    );
    let mut rejected_arguments = arguments.clone();
    rejected_arguments.push("-Wpointer-arith");
    for (name, rust, c, rust_reason, c_reason) in [
        (
            "distinct enum ordering",
            "let _=ENUM_ORDER!(core::ptr::null_mut::<EnumSmall>(),core::ptr::null_mut::<EnumOther>());",
            "void rejected(EnumSmall *left,EnumOther *right){(void)ENUM_ORDER(left,right);}",
            "CompatibleIdentity",
            "distinct pointer types",
        ),
        (
            "distinct enum subtraction",
            "let _=ENUM_DIFFERENCE!(core::ptr::null_mut::<EnumSmall>(),core::ptr::null_mut::<EnumOther>());",
            "void rejected(EnumSmall *left,EnumOther *right){(void)ENUM_DIFFERENCE(left,right);}",
            "CompatibleIdentity",
            "not pointers to compatible types",
        ),
        (
            "void left ordering",
            "let _=ENUM_ORDER!(core::ptr::null_mut::<core::ffi::c_void>(),core::ptr::null_mut::<EnumSmall>());",
            "void rejected(void *left,EnumSmall *right){(void)ENUM_ORDER(left,right);}",
            "NonVoidIdentity",
            "distinct pointer types",
        ),
        (
            "void right ordering",
            "let _=ENUM_ORDER!(core::ptr::null_mut::<EnumSmall>(),core::ptr::null_mut::<core::ffi::c_void>());",
            "void rejected(EnumSmall *left,void *right){(void)ENUM_ORDER(left,right);}",
            "NonVoidIdentity",
            "distinct pointer types",
        ),
        (
            "void subtraction",
            "let _=ENUM_DIFFERENCE!(core::ptr::null_mut::<core::ffi::c_void>(),core::ptr::null_mut::<core::ffi::c_void>());",
            "void rejected(void *left,void *right){(void)ENUM_DIFFERENCE(left,right);}",
            "CompleteObject",
            "void",
        ),
        (
            "incomplete subtraction",
            "let pointer=Pointer::<COpaque<()>,ReadWrite>::new(core::ptr::null_mut());let _=ENUM_DIFFERENCE!(pointer,pointer);",
            "void rejected(EnumOpaque *left,EnumOpaque *right){(void)ENUM_DIFFERENCE(left,right);}",
            "CompleteObject",
            "incomplete type",
        ),
        (
            "rank ordering",
            "let left=Pointer::<CLong,ReadWrite>::new(core::ptr::null_mut());let right=Pointer::<CLongLong,ReadWrite>::new(core::ptr::null_mut());let _=ENUM_ORDER!(left,right);",
            "void rejected(long *left,long long *right){(void)ENUM_ORDER(left,right);}",
            "CompatibleIdentity",
            "distinct pointer types",
        ),
        (
            "rank subtraction",
            "let left=Pointer::<CLong,ReadWrite>::new(core::ptr::null_mut());let right=Pointer::<CLongLong,ReadWrite>::new(core::ptr::null_mut());let _=ENUM_DIFFERENCE!(left,right);",
            "void rejected(long *left,long long *right){(void)ENUM_DIFFERENCE(left,right);}",
            "CompatibleIdentity",
            "not pointers to compatible types",
        ),
        (
            "signedness ordering",
            "let _=ENUM_ORDER!(core::ptr::null_mut::<i32>(),core::ptr::null_mut::<u32>());",
            "void rejected(int *left,unsigned int *right){(void)ENUM_ORDER(left,right);}",
            "CompatibleIdentity",
            "distinct pointer types",
        ),
        (
            "signedness subtraction",
            "let _=ENUM_DIFFERENCE!(core::ptr::null_mut::<i32>(),core::ptr::null_mut::<u32>());",
            "void rejected(int *left,unsigned int *right){(void)ENUM_DIFFERENCE(left,right);}",
            "CompatibleIdentity",
            "not pointers to compatible types",
        ),
    ] {
        let diagnostic = rust_oracle::reject_rust(&format!(
            "{base}\nfn main(){{use __pgrx_c_macros::expression::{{COpaque,Pointer,ReadWrite}};use __pgrx_c_macros::{{CLong,CLongLong}};{rust}}}"
        ));
        assert!(diagnostic.contains(rust_reason), "{name} must reject in Rust: {diagnostic}");
        let diagnostic = rust_oracle::reject_c_invocation(
            &profile.compiler.executable,
            &header,
            c,
            &rejected_arguments,
        );
        assert!(diagnostic.contains(c_reason), "{name} must reject in original C: {diagnostic}");
    }
}
