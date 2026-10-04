//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Compare generated return statements and destination conversions with C.
//!
//! The suite exercises scalar, pointer, null, and set-returning forms in caller
//! functions, observing mutations and evaluation order. Negative cases reject
//! assignment conversions that C does not permit.

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
    AnalysisSession, EmissionStatus, MacroScanner, ParameterOrigin, TypeCategory,
    emit_batch_with_bindings, emit_support_artifact_with_bindings, inspect, pg_sys_integer_bridges,
};
use std::path::PathBuf;
use std::sync::Mutex;

/// Serialize libclang-backed inspection within this test process because its safe runtime
/// permits one active owner.
static SCANNER_LOCK: Mutex<()> = Mutex::new(());

/// Classify PostgreSQL OID constants so fixture bindgen uses the same checked-wrapper boundary
/// as the real binding build.
fn is_builtin_oid(name: &str) -> bool {
    name.ends_with("OID") && name != "HEAP_HASOID"
        || name.ends_with("RelationId")
        || name == "TemplateDbOid"
}

/// Fixture binding or native-support source paired with the unchanged C oracle.
const NATIVE: &str = r#"
unsigned int return_trace;
unsigned int return_evaluations;
Datum return_record(unsigned int digit, Datum value) {
    return_trace = return_trace * 10 + digit;
    return_evaluations++;
    return value;
}
FunctionCallInfo return_fcinfo(FunctionCallInfo value) {
    return_trace = return_trace * 10 + 2;
    return value;
}
FuncCallContext *return_context(FuncCallContext *value) {
    return_trace = return_trace * 10 + 1;
    return value;
}
void return_end(FunctionCallInfo fcinfo, FuncCallContext *context) {
    (void) fcinfo;
    return_trace = return_trace * 10 + 3;
    context->call_cntr = 99;
}
"#;

/// Retain generated Rust, native support, and the exact inspected C profile for paired
/// execution.
struct Generated {
    /// Complete translated consumer source paired with the C program.
    rust: String,
    /// Original C implementation and generated access helpers linked into the Rust consumer.
    native: String,
    /// Original fixture header compiled by both inspection and the independent oracle.
    header: PathBuf,
    /// Compiler selected by the verified frontend profile.
    compiler: PathBuf,
    /// Exact native profile arguments shared by C and Rust-linked compilation.
    arguments: Vec<String>,
}

/// Run paired native and generated consumers under one retained inspected profile.
impl Generated {
    /// Execute the original C program and generated Rust consumer and require their
    /// observations to agree.
    fn compare(&self, c: &str, rust: &str, rows: usize) {
        let arguments = self.arguments.iter().map(String::as_str).collect::<Vec<_>>();
        let original = oracle::run_c(
            &self.compiler,
            &self.header,
            &format!("{}\n{c}", self.native),
            &arguments,
            true,
        );
        let emitted = rust_oracle::run_rust_linked(
            &format!("{}\n{rust}", self.rust),
            &self.compiler,
            &self.header,
            &self.native,
            &arguments,
        );
        assert_eq!(emitted, original, "caller returns must match C conversions and effects");
        assert_eq!(emitted.lines().count(), rows);
    }
}

/// Inspect fixture input, collect bindings, and prepare generated consumers for C/Rust
/// comparison.
fn generate(names: &[&str]) -> Generated {
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let header = directory.join("tests/fixtures/return_oracle.h");
    let scanner = MacroScanner::new().expect("libclang required");
    let mut arguments = vec!["-std=c17".into(), "-O2".into(), "-ffp-contract=off".into()];
    #[cfg(target_os = "macos")]
    {
        let sdk = rust_oracle::run_tool(
            std::process::Command::new("xcrun").arg("--show-sdk-path"),
            "return oracle SDK",
        );
        arguments.extend(["-isysroot".into(), sdk.trim().into()]);
    }
    let frontend = inspect(&scanner, &header, &arguments, None).expect("inspect return fixture");
    let session = AnalysisSession::prepare(&scanner, &frontend, names)
        .expect("prepare caller-return fixture");
    let bindings = bindgen::Builder::default()
        .rust_target(bindgen::RustTarget::stable(85, 0).unwrap())
        .rust_edition(bindgen::RustEdition::Edition2024)
        .header(header.to_str().unwrap())
        .clang_args(&frontend.profile().arguments)
        .blocklist_type("Datum|Oid|TransactionId")
        .allowlist_type("Return.*|FunctionCall.*|FuncCall.*|ExprDoneCond")
        .allowlist_function("return_.*")
        .allowlist_var("return_.*")
        .layout_tests(false)
        .generate_comments(false)
        .formatter(bindgen::Formatter::None)
        .generate()
        .expect("generate actual return fixture bindings")
        .to_string();
    let mut catalog = binding_symbols::collect_bindings(
        &syn::parse_file(&bindings).unwrap(),
        session.integer_constants(),
        frontend.declarations(),
        &frontend.profile().target,
    );
    let TypeCategory::Integer(datum_kind) = frontend.declarations().types["Datum"].category else {
        panic!("the fixture Datum must have its compiler-established integer identity")
    };
    catalog.integer_storage.insert("Datum".into(), datum_kind);
    let artifact = emit_support_artifact_with_bindings(&session, names, &catalog)
        .expect("generate caller-return support");
    let support = directory.join("../pgrx-pg-sys/src/c_macros/support.rs").canonicalize().unwrap();
    let datum = directory.join("../pgrx-pg-sys/src/submodules/datum.rs").canonicalize().unwrap();
    let mut rust = format!(
        "\
#![deny(unsafe_op_in_unsafe_fn)]\n\
#![allow(non_snake_case, non_camel_case_types, dead_code, unused_parens, unreachable_code)]\n\
#[path = {support:?}] pub mod __pgrx_c_macros;\n\
#[path = {datum:?}] mod datum; pub use datum::Datum;\n\
pub struct NullableDatum {{ pub value: Datum, pub isnull: bool }}\n\
pub unsafe fn palloc(_: usize) -> *mut core::ffi::c_void {{ panic!(\"this oracle uses no pass-by-reference Datum conversions\") }}\n\
#[derive(Clone, Copy)] pub struct Oid(u32); impl Oid {{ pub fn to_u32(self) -> u32 {{ self.0 }} pub fn from_u32(value: u32) -> Self {{ Self(value) }} }}\n\
#[derive(Clone, Copy)] pub struct TransactionId(u32); impl TransactionId {{ pub fn into_inner(self) -> u32 {{ self.0 }} pub fn from_inner(value: u32) -> Self {{ Self(value) }} }}\n\
{bindings}\n"
    );
    rust.push_str(
        r#"
/// # Safety
/// The caller must exclude concurrent accesses to the native oracle counters.
unsafe fn native_state() -> (u32, u32) {
    // SAFETY: The caller owns exclusive access to these initialized C globals.
    unsafe { (return_trace, return_evaluations) }
}
"#,
    );
    rust.push_str(&pg_sys_integer_bridges(&frontend).unwrap());
    rust.push_str(&artifact.rust);
    for emission in emit_batch_with_bindings(&session, names, &catalog).unwrap() {
        let EmissionStatus::Emitted { rust: definition, .. } = &emission.status else {
            panic!("caller-return macro must emit: {emission:?}")
        };
        let captures = emission
            .analysis
            .parameters
            .iter()
            .filter(|parameter| parameter.origin == ParameterOrigin::FreeIdentifier)
            .map(|parameter| parameter.name.as_str())
            .collect::<Vec<_>>();
        assert_eq!(
            captures,
            match emission.analysis.name.as_str() {
                "RET_NULL" | "RET_SRF_NEXT" | "RET_SRF_NULL" | "RET_SRF_DONE" => vec!["fcinfo"],
                _ => Vec::<&str>::new(),
            },
            "locals must stay inside the macro; only caller captures become arguments"
        );
        rust.push_str(definition);
    }
    Generated {
        rust,
        native: format!("{NATIVE}\n{}", artifact.c_source),
        header,
        compiler: frontend.profile().compiler.executable.clone(),
        arguments: frontend.profile().arguments.clone(),
    }
}

/// Checks that caller returns preserve destination conversion and argument evaluation.
#[test]
fn caller_returns_preserve_destination_conversion_and_argument_evaluation() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let generated = generate(&[
        "RET_VALUE",
        "RET_ALIAS",
        "RET_BYTE",
        "RET_DATUM",
        "RET_ZERO",
        "RET_UNUSED",
        "RET_SEQUENCE",
    ]);
    let original = r#"
#include <stdio.h>
unsigned char narrow(void) { RET_VALUE(257); }
_Bool truth(void) { RET_VALUE(256); }
unsigned long negative(void) { RET_VALUE(-1); }
int alias(void) { RET_ALIAS(19); }
unsigned int byte(void) { RET_BYTE(257); }
Datum datum(void) { RET_DATUM(return_record(4, 17)); }
Datum unused(void) { RET_UNUSED(return_record(5, 77)); }
Datum twice(void) { RET_SEQUENCE(return_record(6, 23)); }
int *null_pointer(void) { RET_ZERO(); }
int *pointer(int *value) { RET_VALUE(value); }
const int *qualified(int *value) { RET_VALUE(value); }
Datum passthrough(Datum value) { RET_VALUE(value); }
int early(void) { RET_VALUE(7); return_evaluations++; return 99; }
int main(void) {
    printf("%u %u %lu %d %u\n", narrow(), truth(), negative(), alias(), byte());
    Datum result = datum(); printf("%lu %u %u\n", (unsigned long) result, return_trace, return_evaluations);
    result = unused(); printf("%lu %u %u\n", (unsigned long) result, return_trace, return_evaluations);
    result = twice(); printf("%lu %u %u\n", (unsigned long) result, return_trace, return_evaluations);
    int value = 91; printf("%u %u %u\n", null_pointer() == 0, pointer(&value) == &value, qualified(&value) == &value);
    printf("%lu %u %d\n", (unsigned long) passthrough(18), (int *) passthrough((Datum) &value) == &value, *(int *) passthrough((Datum) &value));
    int exited = early(); printf("%d %u\n", exited, return_evaluations);
}
"#;
    let rust = r#"
fn narrow() -> u8 { RET_VALUE!(257); }
fn truth() -> bool { RET_VALUE!(256); }
fn negative() -> u64 {
    RET_VALUE!(@__pgrx_c_return_as [__pgrx_c_macros::CUnsignedLong]; -1);
}
fn alias() -> i32 { RET_ALIAS!(19); }
fn byte() -> u32 { RET_BYTE!(257); }
/// # Safety
/// The caller must exclude concurrent accesses to the native counters.
unsafe fn datum() -> Datum {
    // SAFETY: The native function only updates initialized process-local counters.
    unsafe { RET_DATUM!(return_record(4, Datum::from(17_usize))); }
}
/// # Safety
/// The caller must exclude concurrent accesses to the native counters.
unsafe fn unused() -> Datum {
    // SAFETY: The native function argument is not evaluated by the C macro.
    unsafe { RET_UNUSED!(return_record(5, Datum::from(77_usize))); }
}
/// # Safety
/// The caller must exclude concurrent accesses to the native counters.
unsafe fn twice() -> Datum {
    // SAFETY: Both native calls update initialized counters in statement order.
    unsafe { RET_SEQUENCE!(return_record(6, Datum::from(23_usize))); }
}
fn null_pointer() -> *mut i32 { RET_ZERO!(); }
fn pointer(value: *mut i32) -> *mut i32 { RET_VALUE!(value); }
fn qualified(value: *mut i32) -> *const i32 { RET_VALUE!(value); }
fn passthrough(value: Datum) -> Datum { RET_VALUE!(value); }
fn early() -> i32 {
    RET_VALUE!(7);
    // SAFETY: The initialized process-local counter is exclusively used here.
    unsafe { return_evaluations += 1; }
    99
}
fn main() {
    println!("{} {} {} {} {}", narrow(), u8::from(truth()), negative(), alias(), byte());
    // SAFETY: This executable has one thread and owns the C counters. No references
    // to mutable statics escape, and the native calls do not raise backend errors.
    unsafe {
        let result = datum(); println!("{} {} {}", result.value(), native_state().0, native_state().1);
        let result = unused(); println!("{} {} {}", result.value(), native_state().0, native_state().1);
        let result = twice(); println!("{} {} {}", result.value(), native_state().0, native_state().1);
        let mut value = 91;
        println!("{} {} {}", u8::from(null_pointer().is_null()), u8::from(pointer(&raw mut value) == &raw mut value), u8::from(qualified(&raw mut value) == &raw const value));
        println!("{} {} {}", passthrough(Datum::from(18_usize)).value(), u8::from(passthrough(Datum::from(&raw mut value)).cast_mut_ptr::<i32>() == &raw mut value), *passthrough(Datum::from(&raw mut value)).cast_mut_ptr::<i32>());
        let exited = early(); println!("{} {}", exited, native_state().1);
    }
}
"#;
    generated.compare(original, rust, 7);
}

/// Checks that nullable and set returning macros preserve mutations native calls and order.
#[test]
fn nullable_and_set_returning_macros_preserve_mutations_native_calls_and_order() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let generated = generate(&[
        "RET_DATUM",
        "RET_NULL",
        "RET_LOCAL",
        "RET_LOCAL_WRAPPER",
        "RET_LOCAL_ALIAS",
        "RET_SELF",
        "RET_SRF_NEXT",
        "RET_SRF_NULL",
        "RET_SRF_DONE",
    ]);
    let original = r#"
#include <stdio.h>
unsigned int local_value(void) { RET_LOCAL(31); }
unsigned long local_marked(void) { RET_LOCAL(31); }
unsigned int local_alias(void) { RET_LOCAL_ALIAS(31); }
int self_address(void) { RET_SELF(); }
Datum nullable(FunctionCallInfo fcinfo) { RET_NULL(); }
Datum next(FuncCallContext *context, FunctionCallInfo input) {
    #define fcinfo return_fcinfo(input)
    RET_SRF_NEXT(return_context(context), return_record(4, 17));
    #undef fcinfo
}
Datum next_null(FuncCallContext *context, FunctionCallInfo input) {
    #define fcinfo return_fcinfo(input)
    RET_SRF_NULL(return_context(context));
    #undef fcinfo
}
Datum done(FuncCallContext *context, FunctionCallInfo input) {
    #define fcinfo return_fcinfo(input)
    RET_SRF_DONE(context);
    #undef fcinfo
}
int main(void) {
    ReturnSetInfo info = {ExprSingleResult};
    FunctionCallInfoData input = {0, &info};
    FuncCallContext context = {0};
    Datum value = nullable(&input);
    printf("%u %lu %u %u %lu %u %d\n", local_value(), (unsigned long) value, input.isnull, return_trace, local_marked(), local_alias(), self_address());
    input.isnull = 0; value = next(&context, &input);
    printf("%lu %lu %u %u %u\n", (unsigned long) value, context.call_cntr, info.isDone, input.isnull, return_trace);
    return_trace = 0; value = next_null(&context, &input);
    printf("%lu %lu %u %u %u\n", (unsigned long) value, context.call_cntr, info.isDone, input.isnull, return_trace);
    return_trace = 0; input.isnull = 0; value = done(&context, &input);
    printf("%lu %lu %u %u %u\n", (unsigned long) value, context.call_cntr, info.isDone, input.isnull, return_trace);
}
"#;
    let rust = r#"
fn local_value() -> u32 { RET_LOCAL!(31); }
fn local_marked() -> u64 {
    RET_LOCAL!(@__pgrx_c_return_as [__pgrx_c_macros::CUnsignedLong]; 31);
}
fn local_alias() -> u32 {
    // SAFETY: The macro owns its initialized, aligned local slot. Its pointer
    // alias addresses that same slot and is used only before the caller returns.
    unsafe { RET_LOCAL_ALIAS!(31); }
}
fn self_address() -> i32 { RET_SELF!(); }
/// # Safety
/// fcinfo must identify exclusively writable, initialized, aligned live storage.
unsafe fn nullable(fcinfo: FunctionCallInfo) -> Datum {
    // SAFETY: The caller supplies exclusive, aligned, initialized live storage.
    unsafe { RET_NULL!(fcinfo); }
}
/// # Safety
/// All record pointers must identify exclusively writable, initialized, aligned
/// live storage, and the caller must exclude concurrent native counter access.
unsafe fn next(context: *mut FuncCallContext, input: FunctionCallInfo) -> Datum {
    // SAFETY: Both records and input.resultinfo refer to exclusively writable,
    // initialized live local storage. The native functions only record effects.
    unsafe { RET_SRF_NEXT!(return_context(context), return_record(4, Datum::from(17_usize)), return_fcinfo(input)); }
}
/// # Safety
/// The same live record and exclusive native counter contracts as next apply.
unsafe fn next_null(context: *mut FuncCallContext, input: FunctionCallInfo) -> Datum {
    // SAFETY: The same local record access and native function contracts apply.
    unsafe { RET_SRF_NULL!(return_context(context), return_fcinfo(input)); }
}
/// # Safety
/// The same live record and exclusive native counter contracts as next apply.
unsafe fn done(context: *mut FuncCallContext, input: FunctionCallInfo) -> Datum {
    // SAFETY: return_end mutates only the live context counter; its fcinfo is live.
    unsafe { RET_SRF_DONE!(context, return_fcinfo(input)); }
}
fn main() {
    let mut info = ReturnSetInfo { isDone: ExprDoneCond_ExprSingleResult };
    let mut input = FunctionCallInfoData { isnull: false, resultinfo: (&raw mut info).cast() };
    let mut context = FuncCallContext { call_cntr: 0 };
    // SAFETY: All pointers stay within live, aligned, initialized local records;
    // no aliased references or other threads access them or the native counters.
    unsafe {
        let value = nullable(&raw mut input);
        println!("{} {} {} {} {} {} {}", local_value(), value.value(), u8::from(input.isnull), native_state().0, local_marked(), local_alias(), self_address());
        input.isnull = false; let value = next(&raw mut context, &raw mut input);
        println!("{} {} {} {} {}", value.value(), context.call_cntr, info.isDone, u8::from(input.isnull), native_state().0);
        return_trace = 0; let value = next_null(&raw mut context, &raw mut input);
        println!("{} {} {} {} {}", value.value(), context.call_cntr, info.isDone, u8::from(input.isnull), native_state().0);
        return_trace = 0; input.isnull = false; let value = done(&raw mut context, &raw mut input);
        println!("{} {} {} {} {}", value.value(), context.call_cntr, info.isDone, u8::from(input.isnull), native_state().0);
    }
}
"#;
    generated.compare(original, rust, 4);
    let diagnostic = rust_oracle::reject_rust(&format!(
        "{}\nfn invalid() -> Datum {{ RET_NULL!(); }} fn main() {{}}",
        generated.rust
    ));
    assert!(diagnostic.contains("argument"), "missing fcinfo: {diagnostic}");
    for invocation in [
        "RET_LOCAL!(local)",
        "RET_LOCAL!(r#local)",
        "RET_LOCAL_WRAPPER!(({ local }))",
        "wrapper!(local)",
        "wrapper!((local))",
    ] {
        let diagnostic = rust_oracle::reject_rust(&format!(
            "{}\nmacro_rules! wrapper {{ ($input:expr) => {{ RET_LOCAL!($input) }}; }}\nfn invalid(local: u32) -> u32 {{ {invocation}; }} fn main() {{}}",
            generated.rust
        ));
        assert!(
            diagnostic.contains("C macro argument mentions"),
            "a macro argument must not silently bind a different local: {diagnostic}"
        );
    }
}

/// Checks that return assignment constraints reject illegal pointer and storage conversions.
#[test]
fn return_assignment_constraints_reject_illegal_pointer_and_storage_conversions() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let generated = generate(&["RET_VALUE"]);
    let mut arguments = generated.arguments.iter().map(String::as_str).collect::<Vec<_>>();
    arguments.push("-pedantic-errors");
    for (name, rust, c) in [
        (
            "nonzero_integer",
            "fn invalid() -> *mut i32 { RET_VALUE!(1); }",
            "int *invalid(void) { RET_VALUE(1); }",
        ),
        (
            "runtime_integer_zero",
            "fn invalid(value: i32) -> *mut i32 { RET_VALUE!(value); }",
            "int *invalid(int value) { RET_VALUE(value); }",
        ),
        (
            "discard_const",
            "fn invalid(value: *const i32) -> *mut i32 { RET_VALUE!(value); }",
            "int *invalid(const int *value) { RET_VALUE(value); }",
        ),
        (
            "wrong_storage",
            "fn invalid() -> FunctionCallInfoData { RET_VALUE!(1); }",
            "FunctionCallInfoData invalid(void) { RET_VALUE(1); }",
        ),
    ] {
        let diagnostic =
            rust_oracle::reject_rust(&format!("{}\n{rust}\nfn main() {{}}", generated.rust));
        assert!(
            diagnostic.contains("E0277") || diagnostic.contains("E0308"),
            "{name}: {diagnostic}"
        );
        let diagnostic =
            rust_oracle::reject_c_invocation(&generated.compiler, &generated.header, c, &arguments);
        assert!(!diagnostic.is_empty(), "{name} must violate the original C return contract");
    }
}
