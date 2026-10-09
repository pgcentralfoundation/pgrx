//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Validate the supported closed token-pasting cases against original C.
//!
//! Emission must preserve literal spelling, nominal identity, and symbol paths
//! without evaluating erased operands. Inspection and native executions check
//! supported pastes; open or ambiguous token construction remains a skip.
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
mod rust_oracle;

use pgrx_c_macros::{
    AnalysisSession, EmissionStatus, ExpansionResult, IntegerValue, MacroScanner, SkipReasonCode,
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
    "PASTE_UL",
    "PASTE_ULL",
    "PASTE_L",
    "PASTE_LL",
    "PASTE_PROFILE",
    "PASTE_NESTED",
    "PASTE_ENUM",
    "PASTE_OBJECT",
    "PASTE_TYPE",
    "PASTE_POINTER",
    "PASTE_TAG_POINTER",
    "PASTE_MEMBER",
    "PASTE_RENAMED",
    "PASTE_SET_MEMBER",
    "PASTE_CALL",
    "PASTE_HELPER",
    "PASTE_PLACE_LEFT",
    "PASTE_PLACE_RIGHT",
    "PASTE_REPEAT",
    "PASTE_UNUSED",
    "PASTE_SIZE",
    "PASTE_LAZY",
    "PASTE_ZERO",
    "PASTE_NULL_SELECT",
    "PASTE_NULL_CALL",
    "PASTE_RUNTIME_NULL",
    "PASTE_VALUE_BAD",
    "PASTE_VALUE_ADD",
    "PASTE_VALUE_TOP",
];
/// Fixture candidates deliberately outside the supported contract; each must retain an
/// explained skip.
const REJECTED: &[&str] = &[
    "PASTE_FORMAL_PREFIX",
    "PASTE_FORMAL_SUFFIX",
    "PASTE_FORMAL_MIDDLE",
    "PASTE_FORMAL_DIGRAPH",
    "PASTE_ERASED",
    "PASTE_SYNTH_STRING",
];
/// Fixture binding or native-support source paired with the unchanged C oracle.
const NATIVE: &str = r#"
unsigned int paste_int_calls;
unsigned int paste_long_calls;
int paste_int(int value) { paste_int_calls++; return value; }
long paste_record(long value) { paste_long_calls++; return value+9; }
int *paste_take_pointer(int *pointer) { return pointer; }
"#;
/// Original C recorder source whose header invocations establish expected semantic
/// observations.
const ORIGINAL: &str = r#"
#include <stdio.h>
#define C_RANK(value) _Generic((value), unsigned long:4, unsigned long long:5, long:4, long long:5, default:0)
_Static_assert(_Generic(PASTE_NULL_SELECT(1,(int *)0),int *:1,default:0),"pasted closed zero is a null pointer constant");
int main(void) {
    printf("%016llx %d %zu\n",(unsigned long long)PASTE_UL(),C_RANK(PASTE_UL()),sizeof(PASTE_UL()));
    printf("%016llx %d %zu\n",(unsigned long long)PASTE_ULL(),C_RANK(PASTE_ULL()),sizeof(PASTE_ULL()));
    printf("%ld %d %zu\n",(long)PASTE_L(),C_RANK(PASTE_L()),sizeof(PASTE_L()));
    printf("%lld %d %zu\n",(long long)PASTE_LL(),C_RANK(PASTE_LL()),sizeof(PASTE_LL()));
    printf("%lld %d %zu\n",(long long)PASTE_PROFILE(),C_RANK(PASTE_PROFILE()),sizeof(PASTE_PROFILE()));
    printf("%016llx %d %zu\n",(unsigned long long)PASTE_NESTED(),C_RANK(PASTE_NESTED()),sizeof(PASTE_NESTED()));
    int values[]={-1,0,7,19,257};
    for(unsigned int i=0;i<5;i++) {
        int value=values[i];
        printf("%d %u %lu %d\n",PASTE_ENUM(value),PASTE_OBJECT(value),(unsigned long)PASTE_TYPE(value),C_RANK(PASTE_TYPE(value)));
    }
    PasteRecord object={31,-3}; PasteRecord *pointer=&object;
    printf("%d %d %u %d\n",PASTE_POINTER(pointer)==pointer,PASTE_TAG_POINTER(pointer)==pointer,PASTE_MEMBER(pointer),PASTE_RENAMED(pointer));
    unsigned int assigned=PASTE_SET_MEMBER(pointer,paste_int(-1));
    printf("%u %u %u\n",assigned,object.field,paste_int_calls);
    paste_int_calls=0;
    long called=PASTE_CALL(paste_int(7));
    printf("%ld %u %u\n",called,paste_int_calls,paste_long_calls);
    paste_int_calls=0; paste_long_calls=0;
    int helper=PASTE_HELPER(paste_int(3));
    printf("%d %u %u\n",helper,paste_int_calls,paste_long_calls);
    paste_int_calls=0;
    int repeated=PASTE_REPEAT(paste_int(3));
    printf("%d %u\n",repeated,paste_int_calls);
    paste_int_calls=0;
    int left=PASTE_PLACE_LEFT(paste_int(17)); int right=PASTE_PLACE_RIGHT(paste_int(19));
    printf("%d %d %u\n",left,right,paste_int_calls);
    paste_int_calls=0;
    unsigned long size=PASTE_SIZE(paste_int(17));
    unsigned long unused=PASTE_UNUSED(paste_int(31));
    printf("%lu %016llx %u %u\n",size,(unsigned long long)unused,paste_int_calls,paste_long_calls);
    long absent=PASTE_LAZY(0,paste_int(7));
    printf("%ld %u %u\n",absent,paste_int_calls,paste_long_calls);
    long present=PASTE_LAZY(1,paste_int(7));
    printf("%ld %u %u\n",present,paste_int_calls,paste_long_calls);
    int pointee=31; int *address=&pointee;
    printf("%d %d %d %u\n",PASTE_NULL_SELECT(1,address)==0,PASTE_NULL_SELECT(0,address)==address,PASTE_NULL_CALL()==0,PASTE_ZERO());
    printf("%u %u %u\n",(unsigned int)PASTE_VALUE_BAD(3),(unsigned int)PASTE_VALUE_ADD(3),(unsigned int)PASTE_VALUE_TOP(3));
}
"#;
/// Rust consumer source exercising actual generated macros and adapters.
const CONSUMER: &str = r#"
fn rank<K:__pgrx_c_macros::CInteger>(_:__pgrx_c_macros::CValue<K>) -> u8 { K::RANK }
fn main() {
    let value=PASTE_UL!(); println!("{:016x} {} {}",value.get(),rank(value.into_value()),core::mem::size_of_val(&value.get()));
    let value=PASTE_ULL!(); println!("{:016x} {} {}",value.get(),rank(value.into_value()),core::mem::size_of_val(&value.get()));
    let value=PASTE_L!(); println!("{} {} {}",value.get(),rank(value.into_value()),core::mem::size_of_val(&value.get()));
    let value=PASTE_LL!(); println!("{} {} {}",value.get(),rank(value.into_value()),core::mem::size_of_val(&value.get()));
    let value=PASTE_PROFILE!(); println!("{} {} {}",value.get(),rank(value.into_value()),core::mem::size_of_val(&value.get()));
    let value=PASTE_NESTED!(); println!("{:016x} {} {}",value.get(),rank(value.into_value()),core::mem::size_of_val(&value.get()));
    for value in [-1_i32,0,7,19,257] {
        let word=PASTE_TYPE!(value);
        println!("{} {} {} {}",PASTE_ENUM!(value).get(),PASTE_OBJECT!(value).get(),word.get(),rank(word.into_value()));
    }
    // SAFETY: the record is live, aligned, fully initialized owned storage with
    // exclusive raw-pointer access. The generated casts preserve its address;
    // field accesses remain in this allocation. Native helpers touch only the
    // initialized scalar counters in this single-threaded standalone library.
    // Their small signed operands cannot overflow C int or C long. Pointer
    // helpers only forward live local addresses or null without dereferencing.
    unsafe {
        let mut object=PasteRecord {field:31,type_:-3}; let pointer=&raw mut object;
        let alias:PastePointer=PASTE_POINTER!(pointer).get();
        let tag:*mut PasteRecord=PASTE_TAG_POINTER!(pointer).get();
        println!("{} {} {} {}",u8::from(alias==pointer),u8::from(tag==pointer),PASTE_MEMBER!(pointer).get(),PASTE_RENAMED!(pointer).get());
        let assigned=PASTE_SET_MEMBER!(pointer,paste_int(-1)).get();
        let calls=paste_int_calls;
        println!("{} {} {}",assigned,core::ptr::addr_of!((*pointer).field).read(),calls);
        paste_int_calls=0;
        let called=PASTE_CALL!(paste_int(7)).get();
        let int_calls=paste_int_calls; let long_calls=paste_long_calls;
        println!("{} {} {}",called,int_calls,long_calls);
        paste_int_calls=0; paste_long_calls=0;
        let helper=PASTE_HELPER!(paste_int(3)).get();
        let int_calls=paste_int_calls; let long_calls=paste_long_calls;
        println!("{} {} {}",helper,int_calls,long_calls);
        paste_int_calls=0;
        let repeated=PASTE_REPEAT!(paste_int(3)).get(); let calls=paste_int_calls;
        println!("{} {}",repeated,calls);
        assert_eq!(calls,2,"closed helper rescan preserves both original argument occurrences");
        paste_int_calls=0;
        let left=PASTE_PLACE_LEFT!(paste_int(17)).get(); let right=PASTE_PLACE_RIGHT!(paste_int(19)).get();
        let calls=paste_int_calls; println!("{} {} {}",left,right,calls);
        paste_int_calls=0;
        let size=PASTE_SIZE!(paste_int(17)).get(); let unused=PASTE_UNUSED!(paste_int(31)).get();
        let int_calls=paste_int_calls; let long_calls=paste_long_calls;
        println!("{} {:016x} {} {}",size,unused,int_calls,long_calls);
        assert_eq!((int_calls,long_calls),(0,0),"unused and sizeof operands stay unevaluated");
        let absent=PASTE_LAZY!(0,paste_int(7)).get();
        let int_calls=paste_int_calls; let long_calls=paste_long_calls;
        println!("{} {} {}",absent,int_calls,long_calls);
        let present=PASTE_LAZY!(1,paste_int(7)).get();
        let int_calls=paste_int_calls; let long_calls=paste_long_calls;
        println!("{} {} {}",present,int_calls,long_calls);
        let mut pointee=31_i32; let address=&raw mut pointee;
        let absent:*mut i32=PASTE_NULL_SELECT!(1,address).get();
        let present:*mut i32=PASTE_NULL_SELECT!(0,address).get();
        let called:*mut i32=PASTE_NULL_CALL!().get();
        println!("{} {} {} {}",u8::from(absent.is_null()),u8::from(present==address),u8::from(called.is_null()),PASTE_ZERO!().get());
        println!("{} {} {}",PASTE_VALUE_BAD!(3).get(),PASTE_VALUE_ADD!(3).get(),PASTE_VALUE_TOP!(3).get());
    }
}
"#;

/// Extract the generated value-context arm so assertions inspect the translated expression
/// rather than public forwarding syntax.
fn value_body(source: &str, name: &str) -> String {
    let tokens = syn::parse_file(source)
        .unwrap()
        .items
        .into_iter()
        .find_map(|item| match item {
            syn::Item::Macro(item)
                if item.ident.as_ref().is_some_and(|identifier| identifier == name) =>
            {
                Some(item.mac.tokens.into_iter().collect::<Vec<_>>())
            }
            _ => None,
        })
        .unwrap();
    tokens
        .chunks_exact(5)
        .find(|arm| arm[0].to_string().replace(' ', "").contains("@__pgrx_emit_value"))
        .expect("expression macros have a semantic value arm")[3]
        .to_string()
        .replace(' ', "")
}

/// Checks that closed pastes preserve literal spelling C identity symbols and evaluation.
#[test]
fn closed_pastes_preserve_literal_spelling_c_identity_symbols_and_evaluation() {
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let header = directory.join("tests/fixtures/paste_oracle.h");
    let runtime = directory.join("../pgrx-pg-sys/src/c_macros/support.rs").canonicalize().unwrap();
    let scanner = MacroScanner::new().expect("libclang required");
    for optimization in ["-O0", "-O2"] {
        let mut arguments =
            vec!["-std=c17".into(), optimization.into(), "-ffp-contract=off".into()];
        #[cfg(target_os = "macos")]
        {
            let sdk = rust_oracle::run_tool(
                std::process::Command::new("xcrun").arg("--show-sdk-path"),
                "paste oracle SDK",
            );
            arguments.extend(["-isysroot".into(), sdk.trim().into()]);
        }
        let frontend = inspect(&scanner, &header, &arguments, None).unwrap();
        let names = NAMES.iter().chain(REJECTED).copied().collect::<Vec<_>>();
        let session = AnalysisSession::prepare(&scanner, &frontend, &names).unwrap();
        let bindings = bindgen::Builder::default()
            .rust_target(bindgen::RustTarget::stable(85, 0).unwrap())
            .rust_edition(bindgen::RustEdition::Edition2024)
            .header(header.to_str().unwrap())
            .clang_args(&frontend.profile().arguments)
            .allowlist_type("Paste.*")
            .allowlist_function("paste_.*")
            .allowlist_var("paste_.*|PASTE_CONSTANT|PASTE_VALUE")
            .rustified_enum("PasteTag")
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
        let generated = generate_with_bindings(&session, &names, &catalog).unwrap();
        let mut rust = format!(
            "#![deny(unsafe_op_in_unsafe_fn)]\n#![allow(non_snake_case,non_camel_case_types,dead_code,unused_parens)]\n#[path={runtime:?}] pub mod __pgrx_c_macros;\n{bindings}\n{}",
            generated.support.rust
        );
        assert_eq!(generated.macros.len(), 35);
        for emission in &generated.macros {
            if REJECTED.contains(&emission.analysis.name.as_str()) {
                let EmissionStatus::Skipped { reason } = &emission.status else {
                    panic!("symbolic paste/stringification must reject: {emission:?}");
                };
                assert!(!reason.message.is_empty());
                assert!(!reason.spans.is_empty(), "rejections retain original source provenance");
                continue;
            }
            let EmissionStatus::Emitted { rust: definition, .. } = &emission.status else {
                panic!("closed paste must emit under {optimization}: {emission:?}");
            };
            let body = value_body(definition, &emission.analysis.name);
            match emission.analysis.name.as_str() {
                "PASTE_UL" | "PASTE_ULL" => assert!(
                    body.contains("0xFFFFFFFFFFFFFFFF"),
                    "pasted literals keep hexadecimal spelling: {body}"
                ),
                "PASTE_NESTED" => assert!(body.contains("0x0123456789ABCDEF")),
                "PASTE_TYPE" => assert!(body.contains("::cast_as::<$crate::PasteWord,")),
                "PASTE_POINTER" => assert!(body.contains("::cast_as::<$crate::PastePointer,")),
                "PASTE_OBJECT" => assert!(
                    body.contains("$crate::PASTE_CONSTANT"),
                    "a verified closed-pasted object keeps its binding reference: {body}"
                ),
                "PASTE_VALUE_ADD" => {
                    // Bindgen omits this nested-paste object constant. Clang's
                    // value may be used only with the required audit explanation.
                    assert!(!catalog.integer_constants.contains_key("PASTE_VALUE"));
                    assert!(definition.contains("/* PGRX: PASTE_VALUE"));
                    assert!(definition.contains(
                        "no integer constant binding is available in the defining Rust crate"
                    ));
                    assert!(body.contains("CInt>::new(23i32)"));
                    assert!(!body.contains("$crate::PASTE_VALUE"));
                }
                _ => {}
            }
            rust.push_str(definition);
        }
        let ExpansionResult::Expanded { expansion } = &session.expansions().results["PASTE_HELPER"]
        else {
            panic!("closed helper must expand");
        };
        let dependency = expansion
            .dependencies
            .iter()
            .find(|dependency| dependency.name == "PASTE_ADD")
            .expect("rescan discovers pasted helper absent from raw root tokens");
        assert_eq!(
            dependency.provenance,
            frontend.environment().active["PASTE_ADD"].definition.provenance
        );
        assert!(
            !frontend.environment().active["PASTE_HELPER"]
                .definition
                .tokens
                .iter()
                .any(|token| token.spelling == "PASTE_ADD")
        );
        assert!(session.inputs().files.contains(&header));
        assert_eq!(
            frontend.environment().active["PASTE_CAT"]
                .definition
                .tokens
                .iter()
                .filter(|token| token.spelling == "##")
                .count(),
            1,
            "compiler proofs preserve the original active definition"
        );
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
            "closed paste semantics follow the exact {optimization} profile"
        );
        assert_eq!(actual.lines().count(), 22);
        let mut wrong = catalog.clone();
        assert!(!wrong.integer_constants.contains_key("PASTE_VALUE"));
        assert_eq!(session.integer_constants()["PASTE_VALUE"].value, IntegerValue::Signed(23));
        wrong.integer_constants.get_mut("BadEnum").unwrap().value = IntegerValue::Unsigned(0);
        let mismatch = generate_with_bindings(
            &session,
            &["PASTE_VALUE_BAD", "PASTE_VALUE_ADD", "PASTE_VALUE_TOP"],
            &wrong,
        )
        .unwrap();
        for (name, code) in [
            ("PASTE_VALUE_BAD", SkipReasonCode::BindingValueMismatch),
            ("PASTE_VALUE_ADD", SkipReasonCode::DependencySkipped),
            ("PASTE_VALUE_TOP", SkipReasonCode::DependencySkipped),
        ] {
            let emission =
                mismatch.macros.iter().find(|emission| emission.analysis.name == name).unwrap();
            let EmissionStatus::Skipped { reason } = &emission.status else {
                panic!("the hidden pasted enum mismatch must skip {name}: {emission:?}");
            };
            assert_eq!(reason.code, code);
            assert!(reason.message.contains("bindgen=0, clang=23"));
            assert!(reason.message.contains(&format!("skipping macro `{name}`")));
        }
        for (invocation, reason) in [
            ("PASTE_RUNTIME_NULL!(paste_int(0))", "ImplicitTo"),
            ("PASTE_MEMBER!(core::ptr::null_mut::<i32>())", "Field<"),
            ("PASTE_CALL!(core::ptr::null_mut::<i32>())", "ImplicitTo"),
            ("PASTE_SET_MEMBER!(core::ptr::null::<PasteRecord>(),7)", "WritePlace"),
        ] {
            let diagnostic = rust_oracle::reject_rust(&format!(
                "{rust}\nfn main() {{ unsafe {{ let _={invocation}; }} }}"
            ));
            assert!(
                diagnostic.contains(reason),
                "invalid caller type must reject through {reason}: {diagnostic}"
            );
        }
        let diagnostic = rust_oracle::reject_c_invocation(
            &profile.compiler.executable,
            &header,
            "void invalid(void) { (void)PASTE_RUNTIME_NULL(paste_int(0)); }",
            &arguments,
        );
        assert!(diagnostic.contains("integer to pointer"));
    }
}
