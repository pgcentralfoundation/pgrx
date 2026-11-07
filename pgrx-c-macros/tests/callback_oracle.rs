//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Check callback adapters without erasing nominal C function identities.
//!
//! The fixture combines aliases, raw callback storage, enums, and guard hooks.
//! C/Rust observations establish conversions and call counts, while negative
//! consumer builds reject storage that looks alike in Rust but differs in C.
//!
//! These generated consumers use the runtime's Linux/macOS host family. Emission
//! still validates the inspected C ABI and flags; unsupported-profile checks remain portable.

#![cfg(all(
    target_pointer_width = "64",
    target_endian = "little",
    any(target_arch = "x86_64", target_arch = "aarch64"),
    any(target_os = "linux", target_os = "macos"),
))]

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
#[allow(dead_code)] // Integration fixtures use different subsets of the shared oracle helpers.
mod rust_oracle;

use pgrx_c_macros::{
    AnalysisSession, EmissionStatus, IntegerKind, MacroScanner, RustBindingType, TypeCategory,
    emit_batch_with_bindings, emit_support_artifact_with_bindings, inspect,
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
    "CALLBACK_DIRECT",
    "CALLBACK_TWO",
    "CALLBACK_BYTE",
    "CALLBACK_LONG",
    "CALLBACK_WIDE",
    "CALLBACK_INT",
    "CALLBACK_READ",
    "CALLBACK_WRITE",
    "CALLBACK_CLEAR",
    "CALLBACK_NOARGS",
    "CALLBACK_FACTORY",
    "CALLBACK_GLOBAL",
    "CALLBACK_MEMBER",
    "CALLBACK_WRITE_MEMBER",
    "CALLBACK_LONG_MEMBER",
    "CALLBACK_WIDE_MEMBER",
    "CALLBACK_EQUAL",
    "CALLBACK_LAZY",
    "CALLBACK_GET",
    "CALLBACK_GET_CALL",
    "CALLBACK_GET_ADDRESS",
    "CALLBACK_DEREF",
    "CALLBACK_ADDRESS",
    "CALLBACK_ORIGINAL_ADDRESS",
    "CALLBACK_ORIGINAL_VALUE",
    "CALLBACK_RECORD",
    "CALLBACK_RECORD_VALUE",
    "CALLBACK_PARTIAL",
    "CALLBACK_PARTIAL_IDENTITY",
    "CALLBACK_PARTIAL_TAKE",
    "CALLBACK_PARTIAL_MEMBER",
    "CALLBACK_NEEDED",
    "CALLBACK_STATE",
    "CALLBACK_STATE_VALUE",
];

/// Construct the C invocation used for both inspection and the native oracle, so
/// compiler-profile differences cannot explain a mismatch.
fn arguments() -> Vec<String> {
    let mut arguments = vec![
        "-std=c17".into(),
        "-ffp-contract=off".into(),
        "-Werror=incompatible-pointer-types".into(),
    ];
    #[cfg(target_os = "macos")]
    {
        let sdk = rust_oracle::run_tool(
            std::process::Command::new("xcrun").arg("--show-sdk-path"),
            "callback SDK",
        );
        arguments.extend(["-isysroot".into(), sdk.trim().into()]);
    }
    arguments
}

/// Checks that generated callback signatures preserve native types calls and guards.
#[test]
fn generated_callback_signatures_preserve_native_types_calls_and_guards() {
    let scanner = MacroScanner::new().expect("libclang must be available");
    let header = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/callback_oracle.h");
    let frontend =
        inspect(&scanner, &header, &arguments(), None).expect("inspect native callbacks");
    let rejected = ["CALLBACK_VARIADIC", "CALLBACK_UNPROTOTYPED"];
    let names = NAMES.iter().copied().chain(rejected).collect::<Vec<_>>();
    let session = AnalysisSession::prepare(&scanner, &frontend, &names)
        .expect("analyze original callback macros");
    let bindings = bindgen::Builder::default()
        .rust_target(bindgen::RustTarget::stable(85, 0).unwrap())
        .rust_edition(bindgen::RustEdition::Edition2024)
        .header(header.to_str().unwrap())
        .clang_args(&frontend.profile().arguments)
        .allowlist_type("Callback.*")
        .allowlist_function("callback_.*")
        .allowlist_var("callback_.*")
        .blocklist_type("CallbackOid")
        .rustified_enum("CallbackState")
        .no_copy("CallbackPartialRecord")
        .layout_tests(false)
        .generate_comments(false)
        .formatter(bindgen::Formatter::None)
        .generate()
        .expect("generate actual callback bindings")
        .to_string();
    let mut catalog = binding_symbols::collect_bindings(
        &syn::parse_file(&bindings).unwrap(),
        session.integer_constants(),
        frontend.declarations(),
        &frontend.profile().target,
    );
    assert_eq!(
        frontend.declarations().types["CallbackOid"].category,
        TypeCategory::Integer(IntegerKind::UnsignedInt)
    );
    catalog.integer_storage.insert("CallbackOid".into(), IntegerKind::UnsignedInt);
    if frontend.profile().target.integers[&IntegerKind::Long].bits
        == frontend.profile().target.integers[&IntegerKind::LongLong].bits
    {
        assert_eq!(
            catalog.types["CallbackLong"].target, catalog.types["CallbackLongLong"].target,
            "equal Rust callback storage still needs distinct C rank identities"
        );
    }
    catalog.ffi_boundary = Some(vec!["ffi".into(), "boundary".into()]);
    let artifact = emit_support_artifact_with_bindings(&session, NAMES, &catalog)
        .expect("derive callback adapters");
    assert!(artifact.c_source.contains("__pgrx_function_address_"));
    assert!(!artifact.c_source.contains("CALLBACK_"), "adapters must never forward C macros");
    assert!(artifact.rust.contains("::NativeFunctionSignature for Signature_"));
    assert!(artifact.rust.contains("::Call<"));
    assert!(artifact.rust.contains("boundary(move || function("));
    for emission in emit_batch_with_bindings(&session, &rejected, &catalog).unwrap() {
        assert!(
            matches!(emission.status, EmissionStatus::Skipped { .. }),
            "unproven callback must be skipped: {emission:?}"
        );
    }
    let support = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../pgrx-pg-sys/src/c_macros/support.rs")
        .canonicalize()
        .unwrap();
    let mut rust = format!(
        "#![deny(unsafe_op_in_unsafe_fn)]\n#![allow(non_snake_case, non_camel_case_types, dead_code, unused_parens)]\n#[path={support:?}] pub mod __pgrx_c_macros;\n{}\n{bindings}\n{}\n",
        oid_storage(),
        artifact.rust
    );
    for emission in emit_batch_with_bindings(&session, NAMES, &catalog).unwrap() {
        let EmissionStatus::Emitted { rust: definition, .. } = emission.status else {
            panic!("callback macro {} must emit: {emission:?}", emission.analysis.name)
        };
        rust.push_str(&definition);
    }
    let native =
        format!("{}\n{}", include_str!("fixtures/callback_oracle_native.c"), artifact.c_source);
    let profile = frontend.profile();
    let arguments = profile.arguments.iter().map(String::as_str).collect::<Vec<_>>();
    let generated = rust_oracle::run_rust_linked(
        &format!("{rust}\n{}", include_str!("fixtures/callback_oracle.rs")),
        &profile.compiler.executable,
        &header,
        &native,
        &arguments,
    );
    let original = oracle::run_c(
        &profile.compiler.executable,
        &header,
        &format!("{native}\n{}", include_str!("fixtures/callback_oracle.c")),
        &arguments,
        true,
    );
    assert_eq!(generated, original, "generated callbacks must match the original C macros");
    assert_eq!(generated.lines().count(), 36, "all original C oracle rows must execute");
    let guard = "mod ffi { pub unsafe fn boundary<R,F:FnOnce()->R>(call:F)->R {call()} }";
    for (name, body, c_body) in [
        ("unsafe", "let p=unsafe{callback_table()};let _=CALLBACK_INT!(p,1);", None),
        (
            "arity",
            "unsafe{let p=callback_table();let f=CALLBACK_WRITE_MEMBER!(p);let _=CALLBACK_DIRECT!(f,1);}",
            Some(
                "void fixture(void){struct CallbackTable*p=callback_table();(void)CALLBACK_DIRECT(p->write,1);}",
            ),
        ),
        (
            "pointer_as_integer",
            "unsafe{let p=callback_table();let x=17_i32;let _=CALLBACK_BYTE!(p,&raw const x);}",
            Some(
                "void fixture(void){struct CallbackTable*p=callback_table();int x=17;(void)CALLBACK_BYTE(p,&x);}",
            ),
        ),
        (
            "const_drop",
            "unsafe{let p=callback_table();let x=17_i32;let _=CALLBACK_WRITE!(p,&raw const x,1);}",
            Some(
                "void fixture(void){struct CallbackTable*p=callback_table();const int x=17;(void)CALLBACK_WRITE(p,&x,1);}",
            ),
        ),
        (
            "pointer_as_enum",
            "unsafe{let p=callback_table();let x=17_i32;let _=CALLBACK_STATE!(p,&raw const x);}",
            Some(
                "void fixture(void){struct CallbackTable*p=callback_table();int x=17;(void)CALLBACK_STATE(p,&x);}",
            ),
        ),
        (
            "different_c_ranks",
            "unsafe{let p=callback_table();let _=CALLBACK_EQUAL!(CALLBACK_LONG_MEMBER!(p),CALLBACK_WIDE_MEMBER!(p));}",
            Some(
                "void fixture(void){struct CallbackTable*p=callback_table();(void)CALLBACK_EQUAL(CALLBACK_LONG_MEMBER(p),CALLBACK_WIDE_MEMBER(p));}",
            ),
        ),
    ] {
        let diagnostic = rust_oracle::reject_rust(&format!("{rust}\n{guard}\nfn main(){{{body}}}"));
        assert!(
            diagnostic.contains("E0133")
                || diagnostic.contains("E0277")
                || diagnostic.contains("E0308"),
            "{name}: {diagnostic}"
        );
        if let Some(c_body) = c_body {
            let diagnostic = rust_oracle::reject_c_invocation(
                &profile.compiler.executable,
                &header,
                c_body,
                &arguments,
            );
            assert!(!diagnostic.is_empty(), "original C must diagnose {name}");
        }
    }
    let mut safe_catalog = catalog.clone();
    for alias in safe_catalog.types.values_mut() {
        mark_function_storage_safe(&mut alias.target);
    }
    for variable in safe_catalog.variables.values_mut() {
        mark_function_storage_safe(&mut variable.ty);
    }
    for record in safe_catalog.records.values_mut() {
        for field in record.fields.values_mut() {
            mark_function_storage_safe(&mut field.ty);
        }
    }
    for function in safe_catalog.functions.values_mut() {
        for parameter in &mut function.parameters {
            mark_function_storage_safe(parameter);
        }
        mark_function_storage_safe(&mut function.result);
    }
    let names = ["CALLBACK_INT", "CALLBACK_GET_ADDRESS"];
    let artifact = emit_support_artifact_with_bindings(&session, &names, &safe_catalog)
        .expect("safe pointer witnesses are rejected locally");
    assert!(!artifact.rust.contains("::NativeFunctionSignature for Signature_"));
    assert!(!artifact.c_source.contains("__pgrx_function_address_"));
    for emission in emit_batch_with_bindings(&session, &names, &safe_catalog).unwrap() {
        assert!(
            matches!(emission.status, EmissionStatus::Skipped { .. }),
            "an unchecked safe native target must have no generated call/address capability: {emission:?}"
        );
    }
}

/// Normalize fixture function-pointer safety storage for this oracle without changing its C
/// signature identities.
fn mark_function_storage_safe(storage: &mut RustBindingType) {
    match storage {
        RustBindingType::Function { unsafe_, parameters, result, .. } => {
            *unsafe_ = false;
            for parameter in parameters {
                mark_function_storage_safe(parameter);
            }
            mark_function_storage_safe(result);
        }
        RustBindingType::Pointer { pointee, .. } => mark_function_storage_safe(pointee),
        RustBindingType::Array { element, .. } => mark_function_storage_safe(element),
        RustBindingType::Option { value }
        | RustBindingType::MaybeUninit { value }
        | RustBindingType::ManuallyDrop { value } => mark_function_storage_safe(value),
        _ => {}
    }
}

/// Adjust OID wrapper storage in the fixture catalog to match the checked binding contract used
/// by generation.
fn oid_storage() -> &'static str {
    r#"
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct CallbackOid(u32);
impl __pgrx_c_macros::sealed::Sealed for CallbackOid {}
impl __pgrx_c_macros::expression::IntegerStorage<__pgrx_c_macros::CUnsignedInt> for CallbackOid {
    fn decode(self) -> __pgrx_c_macros::CValue<__pgrx_c_macros::CUnsignedInt> { __pgrx_c_macros::CValue::new(self.0) }
    fn encode(value: __pgrx_c_macros::CValue<__pgrx_c_macros::CUnsignedInt>) -> Self { Self(value.get()) }
}
impl __pgrx_c_macros::expression::NativeType for CallbackOid {
    type Marker = __pgrx_c_macros::expression::CIntegerStorage<__pgrx_c_macros::CUnsignedInt,Self>;
}
impl __pgrx_c_macros::expression::IntoExpression for CallbackOid {
    type Value = __pgrx_c_macros::CValue<__pgrx_c_macros::CUnsignedInt>;
    fn into_expression(self) -> Self::Value { __pgrx_c_macros::CValue::new(self.0) }
}
"#
}
