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
    AnalysisSession, BindingCatalog, EmissionStatus, MacroEmission, MacroScanner, RustBindingType,
    generate_with_bindings, inspect,
};
use std::path::PathBuf;

fn is_builtin_oid(name: &str) -> bool {
    name.ends_with("OID") && name != "HEAP_HASOID"
        || name.ends_with("RelationId")
        || name == "TemplateDbOid"
}

const CASTS: &[(&str, &str)] = &[
    ("ALIAS_MODE", "$crate::AclMode"),
    ("ALIAS_CHAIN", "$crate::AliasModeChain"),
    ("ALIAS_CONST_MODE", "$crate::AliasConstMode"),
    ("ALIAS_VOLATILE_MODE", "$crate::AliasVolatileMode"),
    ("ALIAS_LONG", "$crate::AliasLong"),
    ("ALIAS_LONG_LONG", "$crate::AliasLongLong"),
    ("ALIAS_FLOAT", "$crate::AliasDouble"),
    ("ALIAS_POINTER", "$crate::AliasPointer"),
    ("ALIAS_CONST_POINTER", "$crate::AliasConstPointer"),
    ("ALIAS_POINTER_CHAIN", "$crate::AliasConstPointerChain"),
    ("ALIAS_RECORD_POINTER", "$crate::AliasRecordPointer"),
    ("ALIAS_RECORD_TAG_POINTER", "*mut$crate::AliasRecord"),
    ("ALIAS_ARRAY_POINTER", "$crate::AliasArrayPointer"),
    ("ALIAS_MODE_POINTER", "*mut$crate::AclMode"),
    ("ALIAS_CONST_MODE_POINTER", "*const$crate::AclMode"),
    ("ALIAS_VOLATILE_MODE_POINTER", "*mut$crate::AclMode"),
    ("ALIAS_INTRINSIC_CONST_POINTER", "*const$crate::AliasConstWord"),
    ("ALIAS_ENUM", "$crate::AliasEnum"),
    ("ALIAS_CALLBACK", "$crate::AliasCallback"),
    ("ALIAS_CALLBACK_ZERO", "$crate::AliasCallback"),
    ("ALIAS_ZERO", "$crate::AliasVoidPointer"),
];
const NATIVE: &str = r#"
unsigned int alias_evaluations;
int alias_record(int value) { alias_evaluations++; return value; }
int alias_callback(int value) { return value + 7; }
int *alias_take_mut(int *value) { return value; }
unsigned long *alias_take_ulong(unsigned long *value) { return value; }
AliasCallback alias_take_callback(AliasCallback value) { return value; }
"#;

fn emitted(emission: &MacroEmission) -> &str {
    let EmissionStatus::Emitted { rust, .. } = &emission.status else {
        panic!("verified typedef cast must emit: {emission:?}")
    };
    rust
}

fn public_body(source: &str, name: &str) -> String {
    let parsed = syn::parse_file(source).unwrap();
    let item = parsed
        .items
        .iter()
        .find_map(|item| match item {
            syn::Item::Macro(item)
                if item.ident.as_ref().is_some_and(|identifier| identifier == name) =>
            {
                Some(item)
            }
            _ => None,
        })
        .unwrap();
    let tokens = item.mac.tokens.clone().into_iter().collect::<Vec<_>>();
    tokens
        .chunks_exact(5)
        .find(|arm| arm[0].to_string().replace(' ', "").contains("@__pgrx_emit_public"))
        .unwrap()[3]
        .to_string()
        .replace(' ', "")
}

fn base(directory: &std::path::Path, bindings: &str, support: &str) -> String {
    let runtime = directory.join("../pgrx-pg-sys/src/c_macros/support.rs").canonicalize().unwrap();
    format!(
        "#![deny(unsafe_op_in_unsafe_fn)]\n#![allow(non_snake_case,non_camel_case_types,dead_code,unused_parens)]\n#[path={runtime:?}] pub mod __pgrx_c_macros;\n{bindings}\n{support}"
    )
}

#[test]
fn verified_alias_names_retain_c_type_identity_qualifiers_layout_and_null_tags() {
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let header = directory.join("tests/fixtures/alias_casts.h");
    let scanner = MacroScanner::new().expect("libclang required");
    let mut arguments = vec!["-std=c17".into(), "-O2".into(), "-ffp-contract=off".into()];
    #[cfg(target_os = "macos")]
    {
        let sdk = rust_oracle::run_tool(
            std::process::Command::new("xcrun").arg("--show-sdk-path"),
            "alias casts oracle SDK",
        );
        arguments.extend(["-isysroot".into(), sdk.trim().into()]);
    }
    let frontend = inspect(&scanner, &header, &arguments, None).unwrap();
    let mut names = CASTS.iter().map(|(name, _)| *name).collect::<Vec<_>>();
    names.extend([
        "ALIAS_RANK",
        "ALIAS_NULL_SELECT",
        "ALIAS_NULL_CALLBACK",
        "ALIAS_MUT_CALL",
        "ALIAS_LONG_MUT_CALL",
        "ALIAS_ENUM_CHAIN",
        "ALIAS_NATIVE_CALLBACK",
    ]);
    let session = AnalysisSession::prepare(&scanner, &frontend, &names).unwrap();
    let bindings = bindgen::Builder::default()
        .rust_target(bindgen::RustTarget::stable(85, 0).unwrap())
        .rust_edition(bindgen::RustEdition::Edition2024)
        .header(header.to_str().unwrap())
        .clang_args(&frontend.profile().arguments)
        .allowlist_type("AclMode|Alias.*")
        .allowlist_function("alias_.*")
        .allowlist_var("alias_.*")
        .rustified_enum("AliasEnum")
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
    assert!(catalog.types.contains_key("AliasModeChain"));
    assert!(catalog.enums.contains_key("AliasEnum"));
    assert!(catalog.records.contains_key("AliasRecord"));
    assert!(catalog.types.contains_key("AliasRecordPointer"));
    assert!(catalog.types.contains_key("AliasArrayPointer"));
    let generated = generate_with_bindings(&session, &names, &catalog).unwrap();
    let mut rust = base(&directory, &bindings, &generated.support.rust);
    for emission in &generated.macros {
        let source = emitted(emission);
        if emission.analysis.name == "ALIAS_ENUM_CHAIN" {
            assert!(!catalog.types.contains_key("AliasEnumChain"));
            assert!(source.contains("/* PGRX:") && source.contains("AliasEnumChain"));
            assert!(source.contains("the C type has no corresponding named Rust binding"));
            assert!(
                !public_body(source, "ALIAS_ENUM_CHAIN")
                    .contains("::cast_as::<$crate::AliasEnumChain,")
            );
        }
        if let Some((_, alias)) = CASTS.iter().find(|(name, _)| *name == emission.analysis.name) {
            let body = public_body(source, &emission.analysis.name);
            assert!(
                body.contains(&format!("::cast_as::<{alias},")),
                "the first cast_as type argument must name the actual binding: {body}"
            );
            assert!(!source.contains("/* PGRX:"), "verified aliases need no fallback: {source}");
        }
        rust.push_str(source);
    }
    let native = format!("{NATIVE}\n{}", generated.support.c_source);
    let original = r#"
#include <stdio.h>
#include <stdint.h>
#include <string.h>
#define C_RANK(value) _Generic((value), unsigned long: 4, unsigned long long: 5, default: 0)
_Static_assert(_Generic(ALIAS_NULL_SELECT(1, (int *)0), int *: 1, default: 0), "void typedef zero keeps its null constant identity");
int main(void) {
    int values[] = {-1, 0, 7, 257};
    for (unsigned int index = 0; index < 4; index++) {
        int value = values[index];
        AclMode mode = ALIAS_MODE(alias_record(value));
        AclMode chain = ALIAS_CHAIN(alias_record(value));
        AclMode qualified = ALIAS_CONST_MODE(alias_record(value));
        AclMode volatile_value = ALIAS_VOLATILE_MODE(alias_record(value));
        printf("%u %u %u %u\n", mode, chain, qualified, volatile_value);
    }
    printf("%d %d %d %llu %llu\n", C_RANK(ALIAS_LONG(-1)), C_RANK(ALIAS_LONG_LONG(-1)), C_RANK(ALIAS_RANK(-1)), (unsigned long long) ALIAS_LONG(-1), (unsigned long long) ALIAS_LONG_LONG(-1));
    double floating = ALIAS_FLOAT(257); uint64_t bits;
    memcpy(&bits, &floating, sizeof bits);
    printf("%016llx\n", (unsigned long long) bits);
    int object = 31; int *pointer = &object;
    AclMode mode = 7;
    unsigned long word = 11;
    printf("%d %d %d %d %d %d %d\n", ALIAS_POINTER(pointer)==pointer, ALIAS_CONST_POINTER(pointer)==pointer, ALIAS_POINTER_CHAIN(pointer)==pointer, ALIAS_MODE_POINTER(&mode)==&mode, ALIAS_CONST_MODE_POINTER(&mode)==&mode, ALIAS_VOLATILE_MODE_POINTER(&mode)==&mode, ALIAS_INTRINSIC_CONST_POINTER(&word)==&word);
    AliasRecord record = {17, -3};
    AliasRecordPointer record_pointer = ALIAS_RECORD_POINTER(&record);
    struct AliasRecord *tag_pointer = ALIAS_RECORD_TAG_POINTER(&record);
    printf("%d %d %u %d %zu %zu\n", record_pointer==&record, tag_pointer==&record, record_pointer->count, tag_pointer->flag, sizeof *record_pointer, _Alignof(AliasRecord));
    AliasArray array = {11, 13, 17};
    AliasArrayPointer array_pointer = ALIAS_ARRAY_POINTER(&array);
    printf("%d %u %u %zu %zu\n", array_pointer==&array, (*array_pointer)[0], (*array_pointer)[2], sizeof *array_pointer, _Alignof(AliasArray));
    printf("%d %d %d\n", ALIAS_NULL_SELECT(1,pointer)==0, ALIAS_NULL_SELECT(0,pointer)==pointer, ALIAS_NULL_CALLBACK()==0);
    printf("%u %u\n", (unsigned int) ALIAS_ENUM(7), (unsigned int) ALIAS_ENUM_CHAIN(9));
    AliasCallback callback = alias_callback;
    printf("%d %d %u\n", ALIAS_CALLBACK(callback)(6), ALIAS_CALLBACK_ZERO()==0, alias_evaluations);
}
"#;
    let consumer = r#"
fn rank<K: __pgrx_c_macros::CInteger>(_: __pgrx_c_macros::CValue<K>) -> u8 { K::RANK }
fn main() {
    // SAFETY: native callbacks operate only on initialized process-local scalar
    // counters in this single-threaded executable. Pointers name live aligned
    // initialized locals. Record and array field/element reads stay within these
    // owned allocations with no references or concurrent mutation. Callback calls
    // use the exact original C ABI and small values cannot overflow int.
    unsafe {
        for value in [-1_i32, 0, 7, 257] {
            let mode = ALIAS_MODE!(alias_record(value)).get();
            let chain = ALIAS_CHAIN!(alias_record(value)).get();
            let qualified = ALIAS_CONST_MODE!(alias_record(value)).get();
            let volatile_value = ALIAS_VOLATILE_MODE!(alias_record(value)).get();
            println!("{} {} {} {}", mode, chain, qualified, volatile_value);
        }
        let long = ALIAS_LONG!(-1);
        let long_long = ALIAS_LONG_LONG!(-1);
        let combined = ALIAS_RANK!(-1);
        println!("{} {} {} {} {}", rank(long.into_value()), rank(long_long.into_value()), rank(combined.into_value()), long.get(), long_long.get());
        let floating: f64 = ALIAS_FLOAT!(257).get();
        println!("{:016x}", floating.to_bits());
        let mut object=31_i32; let pointer=&raw mut object;
        let mut mode=7_u32; let mode_pointer=&raw mut mode;
        let a: *mut i32 = ALIAS_POINTER!(pointer).get();
        let b: *const i32 = ALIAS_CONST_POINTER!(pointer).get();
        let c: *const i32 = ALIAS_POINTER_CHAIN!(pointer).get();
        let d: *mut AclMode = ALIAS_MODE_POINTER!(mode_pointer).get();
        let e: *const AclMode = ALIAS_CONST_MODE_POINTER!(mode_pointer).get();
        let f: *mut AclMode = ALIAS_VOLATILE_MODE_POINTER!(mode_pointer).get();
        let mut word: core::ffi::c_ulong = 11; let word_pointer=&raw mut word;
        // u64 alone cannot distinguish C unsigned long from unsigned long long.
        let typed_word=__pgrx_c_macros::expression::Pointer::<__pgrx_c_macros::CUnsignedLong,__pgrx_c_macros::expression::ReadWrite>::new(word_pointer);
        let g: *const AliasConstWord = ALIAS_INTRINSIC_CONST_POINTER!(typed_word).get();
        println!("{} {} {} {} {} {} {}", u8::from(a==pointer),u8::from(b==pointer),u8::from(c==pointer),u8::from(d==mode_pointer),u8::from(e==mode_pointer),u8::from(f==mode_pointer),u8::from(g==word_pointer));
        let mut record=AliasRecord { count:17, flag:-3 }; let record_address=&raw mut record;
        let record_pointer: AliasRecordPointer = ALIAS_RECORD_POINTER!(record_address).get();
        let tag_pointer: *mut AliasRecord = ALIAS_RECORD_TAG_POINTER!(record_address).get();
        println!("{} {} {} {} {} {}", u8::from(record_pointer==record_address),u8::from(tag_pointer==record_address),(*record_pointer).count,(*tag_pointer).flag,core::mem::size_of::<AliasRecord>(),core::mem::align_of::<AliasRecord>());
        let mut array: AliasArray=[11,13,17]; let array_address=&raw mut array;
        let array_pointer: AliasArrayPointer = ALIAS_ARRAY_POINTER!(array_address).get();
        println!("{} {} {} {} {}", u8::from(array_pointer==array_address),(*array_pointer)[0],(*array_pointer)[2],core::mem::size_of::<AliasArray>(),core::mem::align_of::<AliasArray>());
        let absent: *mut i32 = ALIAS_NULL_SELECT!(1,pointer).get();
        let present: *mut i32 = ALIAS_NULL_SELECT!(0,pointer).get();
        println!("{} {} {}", u8::from(absent.is_null()),u8::from(present==pointer),u8::from(ALIAS_NULL_CALLBACK!().get().is_none()));
        // Unnamed C enum values remain tagged compatible integers. This does
        // not construct an AliasEnum Rust value with an invalid discriminant.
        println!("{} {}", ALIAS_ENUM!(7).get(), ALIAS_ENUM_CHAIN!(9).get());
        // The original function designator adapter supplies the exact C
        // signature; Rust Option<extern fn> alone has no nominal C identity.
        let callback = ALIAS_NATIVE_CALLBACK!();
        let converted: AliasCallback = ALIAS_CALLBACK!(callback).get();
        let result = converted.expect("the non-null original callback survives its cast")(6);
        let evaluations = alias_evaluations;
        println!("{} {} {}", result, u8::from(ALIAS_CALLBACK_ZERO!().get().is_none()), evaluations);
        assert_eq!(evaluations, 16, "every cast evaluates its operand once");
    }
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
    assert_eq!(emitted, original, "retaining Rust aliases must preserve compiler C identities");
    assert_eq!(emitted.lines().count(), 12);

    let diagnostic = rust_oracle::reject_rust(&format!(
        "{rust}\nfn main() {{ let mut value=7_i32; let pointer=&raw mut value; unsafe {{ let _=ALIAS_MUT_CALL!(ALIAS_CONST_POINTER!(pointer)); }} }}"
    ));
    assert!(diagnostic.contains("ImplicitTo"), "typedef pointers must retain const: {diagnostic}");
    let diagnostic = rust_oracle::reject_c_invocation(
        &profile.compiler.executable,
        &header,
        "void invalid(int *pointer) { (void) ALIAS_MUT_CALL(ALIAS_CONST_POINTER(pointer)); }",
        &arguments,
    );
    assert!(
        diagnostic.contains("discards qualifiers"),
        "original C qualifier failure: {diagnostic}"
    );
    let diagnostic = rust_oracle::reject_rust(&format!(
        "{rust}\nfn main() {{ let mut value: core::ffi::c_ulong=7; let pointer=__pgrx_c_macros::expression::Pointer::<__pgrx_c_macros::CUnsignedLong,__pgrx_c_macros::expression::ReadWrite>::new(&raw mut value); unsafe {{ let _=ALIAS_LONG_MUT_CALL!(ALIAS_INTRINSIC_CONST_POINTER!(pointer)); }} }}"
    ));
    assert!(
        diagnostic.contains("ImplicitTo"),
        "an outer volatile qualifier must not remove intrinsic typedef const: {diagnostic}"
    );
    let diagnostic = rust_oracle::reject_c_invocation(
        &profile.compiler.executable,
        &header,
        "void invalid(unsigned long *pointer) { (void) ALIAS_LONG_MUT_CALL(ALIAS_INTRINSIC_CONST_POINTER(pointer)); }",
        &arguments,
    );
    assert!(diagnostic.contains("discards qualifiers"), "original C: {diagnostic}");
    let diagnostic = rust_oracle::reject_rust(&format!(
        "{rust}\nfn main() {{ let _=__pgrx_c_macros::expression::cast_as::<AliasDouble, __pgrx_c_macros::CUnsignedInt, _>(__pgrx_c_macros::CValue::<__pgrx_c_macros::CInt>::new(7)); }}"
    ));
    assert!(
        diagnostic.contains("Storage") || diagnostic.contains("type mismatch"),
        "cast_as must enforce alias storage equality instead of relying on a type name: {diagnostic}"
    );

    check_fallback(&directory, &bindings, &session, &catalog, false);
    check_fallback(&directory, &bindings, &session, &catalog, true);
}

fn check_fallback(
    directory: &std::path::Path,
    bindings: &str,
    session: &AnalysisSession<'_>,
    catalog: &BindingCatalog,
    incompatible: bool,
) {
    let mut catalog = catalog.clone();
    if incompatible {
        catalog.types.get_mut("AclMode").unwrap().target =
            RustBindingType::Integer { signed: true, bits: 32 };
    } else {
        catalog.types.remove("AclMode");
    }
    let generated = generate_with_bindings(session, &["ALIAS_MODE"], &catalog).unwrap();
    let source = emitted(&generated.macros[0]);
    let body = public_body(source, "ALIAS_MODE");
    assert!(
        source.contains("/* PGRX:") && source.contains("AclMode"),
        "fallback must explain its source typedef: {source}"
    );
    assert!(
        !body.contains("::cast_as::<$crate::AclMode,"),
        "unverified aliases must never be referenced as cast storage: {body}"
    );
    assert!(
        body.contains("::cast::<"),
        "compiler-owned canonical identity remains available: {body}"
    );
    let rust = format!(
        "{}\n{source}\nfn main() {{ assert_eq!(ALIAS_MODE!(-1).get(), u32::MAX); }}",
        base(directory, bindings, &generated.support.rust),
    );
    assert!(rust_oracle::run_rust(&rust).is_empty());
}
