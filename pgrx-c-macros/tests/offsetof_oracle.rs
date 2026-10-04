//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

// offsetof initially requires the compiler-proven LP64 size_t identity.
#![cfg(all(target_pointer_width = "64", not(target_os = "windows")))]

#[path = "../../pgrx-bindgen/src/build/binding_symbols.rs"]
mod binding_symbols;
#[path = "support/oracle.rs"]
mod oracle;
#[path = "support/rust_oracle.rs"]
mod rust_oracle;

use pgrx_c_macros::{
    AnalysisSession, EmissionStatus, MacroScanner, generate_with_bindings, inspect,
};
use std::collections::BTreeSet;
use std::path::PathBuf;

fn is_builtin_oid(name: &str) -> bool {
    name.ends_with("OID") && name != "HEAP_HASOID"
        || name.ends_with("RelationId")
        || name == "TemplateDbOid"
}

const CASES: &[(&str, &str)] = &[
    ("OFFSET_ZERO", ""),
    ("OFFSET_TAG", ""),
    ("OFFSET_TYPEDEF", ""),
    ("OFFSET_NESTED", ""),
    ("OFFSET_RENAMED", ""),
    ("OFFSET_PACKED", ""),
    ("OFFSET_PACKED_NESTED", ""),
    ("OFFSET_VOLATILE", ""),
    ("OFFSET_FLEXIBLE", ""),
    ("OFFSET_UNSUPPORTED", ""),
    ("OFFSET_CALLBACK", ""),
    ("OFFSET_ENUM", ""),
    ("OFFSET_PROMOTED", ""),
    ("OFFSET_ANON_NESTED", ""),
    ("OFFSET_UNION", ""),
    ("OFFSET_BITFIELD_SIBLING", ""),
    ("OFFSET_SIMPLE", ""),
    ("OFFSET_CONST_ROOT", ""),
    ("OFFSET_VOLATILE_ROOT", ""),
    ("OFFSET_CONST_NESTED", ""),
    ("OFFSET_VOLATILE_NESTED", ""),
    ("OFFSET_FIELD", "nested.value"),
    ("OFFSET_FIELD", "type"),
    ("OFFSET_FIELD", "values"),
    ("OFFSET_GENERIC", "OffsetRecord, nested.value"),
    ("OFFSET_GENERIC", "OffsetAlias, type"),
    ("OFFSET_GENERIC", "OffsetPacked, nested.value"),
    ("OFFSET_GENERIC", "OffsetFlexible, payload"),
    ("OFFSET_GENERIC", "OffsetAnonymous, shared"),
    ("OFFSET_GENERIC", "OffsetUnion, nested.value"),
    ("OFFSET_GENERIC", "OffsetConstSimple, value"),
    ("OFFSET_GENERIC", "OffsetVolatileSimple, value"),
    ("OFFSET_GENERIC", "OffsetQualified, frozen.value"),
    ("OFFSET_GENERIC", "OffsetQualified, changed.value"),
    ("OFFSET_FORWARD", "nested.value"),
];
const AUXILIARY: &[&str] = &[
    "OFFSET_UNUSED",
    "OFFSET_SIZE",
    "OFFSET_LAZY",
    "OFFSET_NULL_SELECT",
    "OFFSET_NULL_CALL",
    "OFFSET_RUNTIME_NULL",
];
const REJECTED: &[&str] = &[
    "OFFSET_BITFIELD",
    "OFFSET_INCOMPLETE",
    "OFFSET_NONRECORD",
    "OFFSET_MISSING",
    "OFFSET_INDEX_CONSTANT",
    "OFFSET_INDEX_DYNAMIC",
    "OFFSET_CALL_PATH",
    "OFFSET_ARROW_PATH",
    "OFFSET_PATH_BUDGET",
];
const NATIVE: &str = r#"
unsigned int offset_evaluations;
int offset_record(int value) { offset_evaluations++; return value; }
int *offset_take_pointer(int *value) { return value; }
"#;

#[test]
fn record_offsets_preserve_original_c_layout_type_identity_and_unevaluated_operands() {
    let project = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let header = project.join("tests/fixtures/offsetof_oracle.h");
    let scanner = MacroScanner::new().expect("libclang required");
    let mut arguments = vec!["-std=c17".into(), "-O2".into(), "-ffp-contract=off".into()];
    #[cfg(target_os = "macos")]
    {
        let sdk = rust_oracle::run_tool(
            std::process::Command::new("xcrun").arg("--show-sdk-path"),
            "offsetof oracle SDK",
        );
        arguments.extend(["-isysroot".into(), sdk.trim().into()]);
    }
    let frontend = inspect(&scanner, &header, &arguments, None).unwrap();
    let names = CASES
        .iter()
        .map(|(name, _)| *name)
        .chain(AUXILIARY.iter().copied())
        .chain(REJECTED.iter().copied())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let session = AnalysisSession::prepare(&scanner, &frontend, &names).unwrap();
    let bindings = bindgen::Builder::default()
        .rust_target(bindgen::RustTarget::stable(85, 0).unwrap())
        .rust_edition(bindgen::RustEdition::Edition2024)
        .header(header.to_str().unwrap())
        .clang_args(&frontend.profile().arguments)
        .allowlist_type("Offset.*|size_t")
        .allowlist_function("offset_.*")
        .allowlist_var("offset_.*")
        .rustified_enum("OffsetState")
        .use_core()
        .wrap_unsafe_ops(true)
        .layout_tests(false)
        .generate_comments(false)
        .formatter(bindgen::Formatter::None)
        .generate()
        .expect("generate actual offset fixture bindings")
        .to_string();
    let catalog = binding_symbols::collect_bindings(
        &syn::parse_file(&bindings).unwrap(),
        session.integer_constants(),
        frontend.declarations(),
        &frontend.profile().target,
    );
    assert!(catalog.records["OffsetPacked"].packed);
    assert!(
        catalog.records["OffsetRecord"]
            .fields
            .values()
            .any(|field| field.rust_name == "type_" || field.rust_name == "r#type"),
        "the original C keyword field must use its actual renamed bindgen field"
    );
    let generated = generate_with_bindings(&session, &names, &catalog).unwrap();
    let runtime = project.join("../pgrx-pg-sys/src/c_macros/support.rs").canonicalize().unwrap();
    let mut rust = format!(
        "#![deny(unsafe_op_in_unsafe_fn)]\n#![allow(non_snake_case,non_camel_case_types,dead_code,unused_parens)]\n#[path={runtime:?}] pub mod __pgrx_c_macros;\n{bindings}\n{}\n",
        generated.support.rust
    );
    let mut emitted = 0;
    let mut skipped = 0;
    for emission in &generated.macros {
        if REJECTED.contains(&emission.analysis.name.as_str()) {
            let EmissionStatus::Skipped { reason } = &emission.status else {
                panic!("unsupported offsetof designator must be skipped: {emission:?}");
            };
            assert!(!reason.message.is_empty());
            skipped += 1;
        } else {
            let EmissionStatus::Emitted { rust: definition, .. } = &emission.status else {
                panic!("valid offset must emit without requiring a field load: {emission:?}");
            };
            rust.push_str(definition);
            emitted += 1;
        }
    }
    assert_eq!(emitted, 30, "every fixture offset family must be represented");
    assert_eq!(skipped, REJECTED.len());
    let mut original = String::from(
        "#include <stdio.h>\n#define C_RANK(value) _Generic((value), unsigned long:4, unsigned long long:5, default:0)\nint main(void) {\n",
    );
    let mut consumer = String::from(
        "fn rank<K:__pgrx_c_macros::CInteger>(_:__pgrx_c_macros::CValue<K>)->u8 { K::RANK }\nfn main() {\n",
    );
    for &(name, arguments) in CASES {
        original.push_str(&format!(
            "printf(\"{name} %zu %d %zu\\n\",{name}({arguments}),C_RANK({name}({arguments})),sizeof({name}({arguments})));\n"
        ));
        // No object is constructed, borrowed, or read: even packed, volatile,
        // flexible and non-loadable field offsets must remain safe Rust calls.
        consumer.push_str(&format!(
            "let result={name}!({arguments}); let value=result.get(); println!(\"{name} {{}} {{}} {{}}\",value,rank(result.into_value()),core::mem::size_of_val(&value));\n"
        ));
    }
    original.push_str(
        r#"
    offset_evaluations=0;
    size_t unused=OFFSET_UNUSED(offset_record(17));
    size_t size=OFFSET_SIZE(offset_record(19));
    printf("UNEVALUATED %zu %zu %u\n",unused,size,offset_evaluations);
    size_t absent=OFFSET_LAZY(1,offset_record(73));
    printf("LAZY_TRUE %zu %u\n",absent,offset_evaluations);
    size_t present=OFFSET_LAZY(0,offset_record(73));
    printf("LAZY_FALSE %zu %u\n",present,offset_evaluations);
    int value=31; int *pointer=&value;
    printf("NULL %d %d %d\n",OFFSET_NULL_SELECT(1,pointer)==0,OFFSET_NULL_SELECT(0,pointer)==pointer,OFFSET_NULL_CALL()==0);
}
"#,
    );
    consumer.push_str(
        r#"
    // SAFETY: Native functions only increment this initialized standalone
    // scalar counter or return an input pointer without dereferencing it.
    // The executable is single-threaded and all payloads are valid C scalars.
    // sizeof, unused arguments and the unselected lazy branch execute no call.
    unsafe {
        offset_evaluations=0;
        let unused=OFFSET_UNUSED!(offset_record(17)).get();
        let size=OFFSET_SIZE!(offset_record(19)).get();
        let evaluations=offset_evaluations;
        println!("UNEVALUATED {} {} {}",unused,size,evaluations);
        assert_eq!(evaluations,0);
        let absent=OFFSET_LAZY!(1,offset_record(73)).get();
        let evaluations=offset_evaluations;
        println!("LAZY_TRUE {} {}",absent,evaluations);
        assert_eq!(evaluations,0);
        let present=OFFSET_LAZY!(0,offset_record(73)).get();
        let evaluations=offset_evaluations;
        println!("LAZY_FALSE {} {}",present,evaluations);
        assert_eq!(evaluations,1);
        let mut value=31_i32; let pointer=&raw mut value;
        let absent: *mut i32=OFFSET_NULL_SELECT!(1,pointer).get();
        let present: *mut i32=OFFSET_NULL_SELECT!(0,pointer).get();
        let called: *mut i32=OFFSET_NULL_CALL!().get();
        println!("NULL {} {} {}",u8::from(absent.is_null()),u8::from(present==pointer),u8::from(called.is_null()));
    }
}
"#,
    );
    let profile = frontend.profile();
    let arguments = profile.arguments.iter().map(String::as_str).collect::<Vec<_>>();
    let native = format!("{NATIVE}\n{}", generated.support.c_source);
    let expected = oracle::run_c(
        &profile.compiler.executable,
        &header,
        &format!("{native}\n{original}"),
        &arguments,
        true,
    );
    let actual = rust_oracle::run_rust_linked(
        &format!("{rust}\n{consumer}"),
        &profile.compiler.executable,
        &header,
        &native,
        &arguments,
    );
    assert_eq!(actual, expected, "offsets, sizeof and null constants must match original C");
    assert_eq!(actual.lines().count(), CASES.len() + 4);

    let runtime_zero = rust_oracle::reject_rust(&format!(
        "{rust}\nfn main() {{ let condition=1_i32; unsafe {{ let _=OFFSET_RUNTIME_NULL!(condition); }} }}"
    ));
    assert!(
        runtime_zero.contains("ImplicitTo"),
        "runtime-selected offset zero is not an ICE: {runtime_zero}"
    );
    let runtime_zero = rust_oracle::reject_c_invocation(
        &profile.compiler.executable,
        &header,
        "void invalid(int condition) { (void) OFFSET_RUNTIME_NULL(condition); }",
        &arguments,
    );
    assert!(
        runtime_zero.contains("integer to pointer"),
        "original C rejects runtime zero: {runtime_zero}"
    );

    for (invocation, expected_diagnostic) in [
        ("OFFSET_FIELD!(absent)", "field has no compiler-verified PostgreSQL binding capability"),
        ("OFFSET_FIELD!(zero.value)", "OffsetField"),
        ("OFFSET_GENERIC!(*mut OffsetRecord, zero)", "OffsetField"),
        ("OFFSET_GENERIC!(OffsetIncomplete, zero)", "NativeType"),
    ] {
        let diagnostic =
            rust_oracle::reject_rust(&format!("{rust}\nfn main() {{ let _={invocation}; }}"));
        assert!(
            diagnostic.contains(expected_diagnostic),
            "invalid field/type continuation must be rejected: {invocation}: {diagnostic}"
        );
    }
    for invocation in ["OFFSET_FIELD!(values[1])", "OFFSET_FIELD!(nested->value)"] {
        let diagnostic =
            rust_oracle::reject_rust(&format!("{rust}\nfn main() {{ let _={invocation}; }}"));
        assert!(
            diagnostic.contains("arguments do not satisfy this C macro's invocation contract"),
            "the field designator grammar must reject indices and arrows: {invocation}: {diagnostic}"
        );
    }
    for invocation in [
        "OFFSET_BITFIELD()",
        "OFFSET_INCOMPLETE()",
        "OFFSET_NONRECORD()",
        "OFFSET_MISSING()",
        "OFFSET_CALL_PATH()",
        "OFFSET_ARROW_PATH()",
        "OFFSET_GENERIC(OffsetRecord, zero.absent)",
    ] {
        let diagnostic = rust_oracle::reject_c_invocation(
            &profile.compiler.executable,
            &header,
            &format!("void invalid(void) {{ (void) {invocation}; }}"),
            &arguments,
        );
        assert!(!diagnostic.is_empty());
    }
}
