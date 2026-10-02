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
    AnalysisSession, EmissionStatus, EvaluationRequirement, FrontendOutput, MacroScanner,
    generate_with_bindings, inspect,
};
use std::path::{Path, PathBuf};

fn is_builtin_oid(name: &str) -> bool {
    name.ends_with("OID") && name != "HEAP_HASOID"
        || name.ends_with("RelationId")
        || name == "TemplateDbOid"
}

const NAMES: &[&str] = &[
    "EXPECT_RAW",
    "EXPECT_GROUPED",
    "EXPECT_NESTED",
    "EXPECT_LIKELY",
    "EXPECT_UNLIKELY",
    "EXPECT_RECORD",
    "EXPECT_VOLATILE",
    "EXPECT_SIZE",
    "EXPECT_LAZY",
    "EXPECT_AND",
    "EXPECT_REPEAT",
    "EXPECT_UNUSED",
    "EXPECT_PROFILE",
    "EXPECT_ZERO",
    "EXPECT_NULL_SELECT",
    "EXPECT_NULL_CALL",
    "EXPECT_RUNTIME_NULL",
    "EXPECT_IMPURE_NULL",
];
const REJECTED: &[&str] = &[
    "EXPECT_WRONG0",
    "EXPECT_WRONG1",
    "EXPECT_WRONG3",
    "EXPECT_SYMBOL",
    "EXPECT_ADDRESS",
    "EXPECT_DEREF",
    "EXPECT_CALLBACK",
];
const NATIVE: &str = r#"
unsigned int expect_value_calls;
unsigned int expect_hint_calls;
unsigned int expect_trace;
volatile long expect_signal;
volatile long expect_hint_signal;
long expect_record_value(long value) { expect_value_calls++; expect_trace=expect_trace*10+1; return value; }
long expect_record_hint(long hint) { expect_hint_calls++; expect_trace=expect_trace*10+2; return hint; }
long expect_use_callback(ExpectCallback callback) { return callback(17,1); }
int *expect_take_pointer(int *pointer) { return pointer; }
"#;
const ORIGINAL: &str = r#"
#include <stdio.h>
#define C_RANK(value) _Generic((value), int: 3, long: 4, long long: 5, default: 0)
_Static_assert(_Generic(EXPECT_NULL_SELECT(1,(int *)0), int *: 1, default: 0), "closed expect zero is a null pointer constant");
static void observations(long result) {
    printf("%ld %u %u %u\n",result,expect_value_calls,expect_hint_calls,expect_trace);
    expect_value_calls=0; expect_hint_calls=0; expect_trace=0;
}
int main(void) {
    long values[]={-2147483647L,-2L,-1L,0L,1L,7L,2147483647L};
    for(unsigned int i=0;i<7;i++) {
        long value=values[i];
        printf("%ld %ld %ld %ld %ld\n",EXPECT_RAW(value,-9),EXPECT_GROUPED(value,1),EXPECT_NESTED(value),EXPECT_LIKELY(value),EXPECT_UNLIKELY(value));
    }
    double floats[]={-123.75,-0.0,0.0,0.75,1.75,123.75};
    for(unsigned int i=0;i<6;i++) printf("%ld %ld\n",EXPECT_RAW(floats[i],-3.75),EXPECT_RAW((float)floats[i],2.5f));
    unsigned long wide[]={0UL,1UL,0x8000000000000000UL,0xFFFFFFFFFFFFFFFFUL};
    for(unsigned int i=0;i<4;i++) printf("%ld %ld\n",EXPECT_RAW(wide[i],0),EXPECT_RAW((unsigned long long)wide[i],0));
    printf("%d %zu %d %zu\n",C_RANK(EXPECT_RAW(0,1)),sizeof(EXPECT_RAW(0,1)),C_RANK(EXPECT_LIKELY(7)),sizeof(EXPECT_LIKELY(7)));
    observations(EXPECT_RECORD(7,-3));
    (void)EXPECT_RAW(expect_record_value(11),expect_record_hint(29)); observations(0);
    observations(EXPECT_REPEAT(expect_record_value(3),expect_record_hint(1)));
    long first=41, __pgrx_c_expect_result=73;
    printf("%ld %ld\n",EXPECT_RAW(first,__pgrx_c_expect_result),EXPECT_RAW(__pgrx_c_expect_result,first));
    expect_signal=-17; expect_hint_signal=2;
    printf("%ld %ld %ld\n",EXPECT_VOLATILE(),expect_signal,expect_hint_signal);
    unsigned long size=EXPECT_SIZE(expect_record_value(17),expect_record_hint(1));
    printf("%lu %u %u\n",size,expect_value_calls,expect_hint_calls);
    observations(EXPECT_UNUSED(expect_record_value(31)));
    observations(EXPECT_LAZY(0,expect_record_value(13),expect_record_hint(1)));
    observations(EXPECT_AND(0,expect_record_value(13),expect_record_hint(1)));
    observations(EXPECT_LAZY(1,expect_record_value(13),expect_record_hint(1)));
    observations(EXPECT_AND(1,expect_record_value(13),expect_record_hint(1)));
    printf("%ld %ld\n",EXPECT_PROFILE(7),EXPECT_ZERO());
    int object=31; int *pointer=&object;
    printf("%d %d %d\n",EXPECT_NULL_SELECT(1,pointer)==0,EXPECT_NULL_SELECT(0,pointer)==pointer,EXPECT_NULL_CALL()==0);
}
"#;
const CONSUMER: &str = r#"
fn rank<K: __pgrx_c_macros::CInteger>(_: __pgrx_c_macros::CValue<K>) -> u8 { K::RANK }
fn long(value:i64) -> __pgrx_c_macros::CValue<__pgrx_c_macros::CLong> { __pgrx_c_macros::CValue::new(value) }
fn observations(result:i64) {
    // SAFETY: these initialized scalar globals belong to this single-threaded
    // standalone C library. No references or concurrent accesses exist.
    unsafe {
        let value_calls=expect_value_calls; let hint_calls=expect_hint_calls; let trace=expect_trace;
        println!("{} {} {} {}",result,value_calls,hint_calls,trace);
        expect_value_calls=0; expect_hint_calls=0; expect_trace=0;
    }
}
fn main() {
    for value in [-2147483647_i64,-2,-1,0,1,7,2147483647] {
        let value=long(value);
        println!("{} {} {} {} {}",EXPECT_RAW!(value,-9).get(),EXPECT_GROUPED!(value,1).get(),EXPECT_NESTED!(value).get(),EXPECT_LIKELY!(value).get(),EXPECT_UNLIKELY!(value).get());
    }
    for value in [-123.75_f64,-0.0,0.0,0.75,1.75,123.75] {
        println!("{} {}",EXPECT_RAW!(value,-3.75_f64).get(),EXPECT_RAW!(value as f32,2.5_f32).get());
    }
    for value in [0_u64,1,0x8000000000000000,u64::MAX] {
        let ulong=__pgrx_c_macros::CValue::<__pgrx_c_macros::CUnsignedLong>::new(value);
        let ull=__pgrx_c_macros::CValue::<__pgrx_c_macros::CUnsignedLongLong>::new(value);
        println!("{} {}",EXPECT_RAW!(ulong,0).get(),EXPECT_RAW!(ull,0).get());
    }
    let raw=EXPECT_RAW!(0,1); let likely=EXPECT_LIKELY!(7);
    println!("{} {} {} {}",rank(raw.into_value()),core::mem::size_of_val(&raw.get()),rank(likely.into_value()),core::mem::size_of_val(&likely.get()));
    // SAFETY: native callbacks access only initialized scalar state owned by
    // this single-threaded executable. Their complete C function executions
    // are sequenced, so tracing their order does not introduce unsequenced C
    // writes. Finite float inputs stay inside C long's representable range.
    // Pointer helpers only forward a live local address or a null pointer;
    // neither helper nor macro dereferences those pointers.
    unsafe {
        observations(EXPECT_RECORD!(long(7),long(-3)).get());
        EXPECT_RAW!(@__pgrx_c_discard; long(expect_record_value(11)),long(expect_record_hint(29))); observations(0);
        observations(EXPECT_REPEAT!(long(expect_record_value(3)),long(expect_record_hint(1))).get());
        let first=long(41); let __pgrx_c_expect_result=long(73);
        println!("{} {}",EXPECT_RAW!(first,__pgrx_c_expect_result).get(),EXPECT_RAW!(__pgrx_c_expect_result,first).get());
        core::ptr::write_volatile(core::ptr::addr_of_mut!(expect_signal),-17);
        core::ptr::write_volatile(core::ptr::addr_of_mut!(expect_hint_signal),2);
        let value=EXPECT_VOLATILE!().get();
        let signal=core::ptr::read_volatile(core::ptr::addr_of!(expect_signal));
        let hint=core::ptr::read_volatile(core::ptr::addr_of!(expect_hint_signal));
        println!("{} {} {}",value,signal,hint);
        let size=EXPECT_SIZE!(@__pgrx_c_expression; long(expect_record_value(17)),long(expect_record_hint(1))).get();
        let value_calls=expect_value_calls; let hint_calls=expect_hint_calls;
        println!("{} {} {}",size,value_calls,hint_calls);
        assert_eq!((value_calls,hint_calls),(0,0),"sizeof must suppress both operands");
        observations(EXPECT_UNUSED!(expect_record_value(31)).get());
        observations(EXPECT_LAZY!(0,long(expect_record_value(13)),long(expect_record_hint(1))).get());
        observations(EXPECT_AND!(0,long(expect_record_value(13)),long(expect_record_hint(1))).get().into());
        observations(EXPECT_LAZY!(1,long(expect_record_value(13)),long(expect_record_hint(1))).get());
        observations(EXPECT_AND!(1,long(expect_record_value(13)),long(expect_record_hint(1))).get().into());
        println!("{} {}",EXPECT_PROFILE!(7).get(),EXPECT_ZERO!().get());
        let mut object=31_i32; let pointer=&raw mut object;
        let absent:*mut i32=EXPECT_NULL_SELECT!(1,pointer).get();
        let present:*mut i32=EXPECT_NULL_SELECT!(0,pointer).get();
        let called:*mut i32=EXPECT_NULL_CALL!().get();
        println!("{} {} {}",u8::from(absent.is_null()),u8::from(present==pointer),u8::from(called.is_null()));
    }
}
"#;

fn arguments(optimization: &str, shadow: bool) -> Vec<String> {
    let mut arguments = vec!["-std=c17".into(), optimization.into(), "-ffp-contract=off".into()];
    if shadow {
        arguments.push("-DEXPECT_SHADOW=1".into());
    }
    #[cfg(target_os = "macos")]
    {
        let sdk = rust_oracle::run_tool(
            std::process::Command::new("xcrun").arg("--show-sdk-path"),
            "expect oracle SDK",
        );
        arguments.extend(["-isysroot".into(), sdk.trim().into()]);
    }
    arguments
}

fn bindings(frontend: &FrontendOutput) -> String {
    bindgen::Builder::default()
        .rust_target(bindgen::RustTarget::stable(85, 0).unwrap())
        .rust_edition(bindgen::RustEdition::Edition2024)
        .header(frontend.profile().header.to_str().unwrap())
        .clang_args(&frontend.profile().arguments)
        .allowlist_type("Expect.*")
        .allowlist_function("expect_.*")
        .allowlist_var("expect_.*")
        .use_core()
        .wrap_unsafe_ops(true)
        .layout_tests(false)
        .generate_comments(false)
        .formatter(bindgen::Formatter::None)
        .generate()
        .unwrap()
        .to_string()
}

fn rust_base(directory: &Path, bindings: &str, support: &str) -> String {
    let runtime = directory.join("../pgrx-pg-sys/src/c_macros/support.rs").canonicalize().unwrap();
    format!(
        "#![deny(unsafe_op_in_unsafe_fn)]\n#![allow(non_snake_case,non_camel_case_types,dead_code,unused_parens)]\n#[path={runtime:?}] pub mod __pgrx_c_macros;\n{bindings}\n{support}"
    )
}

#[test]
fn expectation_builtins_preserve_long_identity_and_both_operand_evaluations() {
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let header = directory.join("tests/fixtures/expect_oracle.h");
    let scanner = MacroScanner::new().expect("libclang required");
    for optimization in ["-O0", "-O2"] {
        let frontend = inspect(&scanner, &header, &arguments(optimization, false), None).unwrap();
        let names = NAMES.iter().chain(REJECTED).copied().collect::<Vec<_>>();
        let session = AnalysisSession::prepare(&scanner, &frontend, &names).unwrap();
        let bindings = bindings(&frontend);
        let catalog = binding_symbols::collect_bindings(
            &syn::parse_file(&bindings).unwrap(),
            session.integer_constants(),
            frontend.declarations(),
            &frontend.profile().target,
        );
        let generated = generate_with_bindings(&session, &names, &catalog).unwrap();
        let mut rust = rust_base(&directory, &bindings, &generated.support.rust);
        assert_eq!(generated.macros.len(), 25);
        for emission in &generated.macros {
            if REJECTED.contains(&emission.analysis.name.as_str()) {
                let EmissionStatus::Skipped { reason } = &emission.status else {
                    panic!("invalid expectation builtin use must be rejected: {emission:?}");
                };
                assert!(!reason.message.is_empty());
                continue;
            }
            let EmissionStatus::Emitted { rust: definition, .. } = &emission.status else {
                panic!("proved expectation builtin must emit under {optimization}: {emission:?}");
            };
            if emission.analysis.name == "EXPECT_RAW" {
                assert!(
                    emission
                        .analysis
                        .evaluation
                        .requirements
                        .contains(&EvaluationRequirement::UnspecifiedOperandOrder)
                );
            }
            if emission.analysis.name == "EXPECT_VOLATILE" {
                assert!(definition.contains("CVolatile"));
            }
            rust.push_str(definition);
        }
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
        assert_eq!(actual, expected, "expect semantics follow the exact {optimization} profile");
        assert_eq!(actual.lines().count(), 31);
        for invocation in [
            "EXPECT_RUNTIME_NULL!(__pgrx_c_macros::CValue::<__pgrx_c_macros::CLong>::new(expect_record_value(0)))",
            "EXPECT_IMPURE_NULL!()",
            "EXPECT_RAW!(core::ptr::null_mut::<i32>(),1)",
            "EXPECT_RAW!(1,core::ptr::null_mut::<i32>())",
        ] {
            let diagnostic = rust_oracle::reject_rust(&format!(
                "{rust}\nfn main() {{ unsafe {{ let _={invocation}; }} }}"
            ));
            assert!(diagnostic.contains("ImplicitTo"), "invalid implicit conversion: {diagnostic}");
        }
        // The impure hint's unusual Clang constant-folding rules are deliberately
        // not used to establish Rust null identity. Compare C's definite runtime
        // and pointer-argument rejections, independent of that conservative case.
        for invocation in [
            "EXPECT_RUNTIME_NULL(expect_record_value(0))",
            "EXPECT_RAW((int *)0,1)",
            "EXPECT_RAW(1,(int *)0)",
            "EXPECT_WRONG0()",
            "EXPECT_WRONG1(7)",
            "EXPECT_WRONG3(7)",
            "EXPECT_SYMBOL()",
            "EXPECT_ADDRESS()",
            "EXPECT_DEREF()",
            "EXPECT_CALLBACK()",
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
    let frontend = inspect(&scanner, &header, &arguments("-O2", true), None).unwrap();
    let names = ["EXPECT_RAW", "EXPECT_NESTED"];
    let session = AnalysisSession::prepare(&scanner, &frontend, &names).unwrap();
    let bindings = bindings(&frontend);
    let catalog = binding_symbols::collect_bindings(
        &syn::parse_file(&bindings).unwrap(),
        session.integer_constants(),
        frontend.declarations(),
        &frontend.profile().target,
    );
    let generated = generate_with_bindings(&session, &names, &catalog).unwrap();
    let mut rust = rust_base(&directory, &bindings, &generated.support.rust);
    for emission in &generated.macros {
        let EmissionStatus::Emitted { rust: definition, .. } = &emission.status else {
            panic!("source macro shadowing must follow C preprocessing: {emission:?}");
        };
        rust.push_str(definition);
    }
    let original = "#include <stdio.h>\nint main(void) { int values[]={-7,0,1,17}; for(unsigned int i=0;i<4;i++) printf(\"%ld %ld\\n\",EXPECT_RAW(values[i],0),EXPECT_NESTED(values[i])); }";
    let consumer = "fn main() { for value in [-7_i32,0,1,17] { println!(\"{} {}\",EXPECT_RAW!(value,0).get(),EXPECT_NESTED!(value).get()); } }";
    let profile = frontend.profile();
    let arguments = profile.arguments.iter().map(String::as_str).collect::<Vec<_>>();
    let expected = oracle::run_c(&profile.compiler.executable, &header, original, &arguments, true);
    let actual = rust_oracle::run_rust(&format!("{rust}\n{consumer}"));
    assert_eq!(actual, expected, "source macros shadow compiler builtins before lowering");
}
