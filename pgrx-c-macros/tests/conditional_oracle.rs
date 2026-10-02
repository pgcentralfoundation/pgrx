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
    AnalysisSession, EmissionStatus, EvaluationRequirement, HelperRequirement, InvocationContract,
    MacroScanner, ParameterOrigin, emit_batch_with_bindings, emit_support_artifact_with_bindings,
    inspect,
};
use std::path::PathBuf;
use std::sync::Mutex;

static SCANNER_LOCK: Mutex<()> = Mutex::new(());

fn is_builtin_oid(name: &str) -> bool {
    name.ends_with("OID") && name != "HEAP_HASOID"
        || name.ends_with("RelationId")
        || name == "TemplateDbOid"
}

const NATIVE: &str = r#"
unsigned int conditional_trace;
unsigned int conditional_calls;
unsigned int conditional_float_left;
unsigned int conditional_float_right;
unsigned int conditional_float_places;
unsigned int conditional_record(unsigned int digit, unsigned int value) {
    conditional_trace = conditional_trace * 10 + digit;
    conditional_calls++;
    return value;
}
void conditional_store(ConditionalRecord *pointer, unsigned int digit, unsigned int value) {
    conditional_trace = conditional_trace * 10 + digit;
    conditional_calls++;
    pointer->total = value;
}
void conditional_hook(ConditionalRecord *pointer, unsigned int value) {
    conditional_store(pointer, 5, value);
}
double conditional_float_record(unsigned int operand, double value) {
    if (operand == 0) conditional_float_left++;
    else conditional_float_right++;
    return value;
}
double *conditional_float_place(double *pointer) {
    conditional_float_places++;
    return pointer;
}
"#;

struct Generated {
    rust: String,
    native: String,
    header: PathBuf,
    compiler: PathBuf,
    arguments: Vec<String>,
}

impl Generated {
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
        assert_eq!(emitted, original, "conditional control flow and effects must match C");
        assert_eq!(emitted.lines().count(), rows);
    }
}

fn generate(names: &[&str]) -> Generated {
    generate_with_arguments(names, &["-ffp-contract=off"])
}

fn generate_with_arguments(names: &[&str], extra_arguments: &[&str]) -> Generated {
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let header = directory.join("tests/fixtures/conditional_oracle.h");
    let scanner = MacroScanner::new().expect("libclang required");
    // The oracle deliberately certifies the floating profile rather than
    // assuming all compiler flags permit Rust's IEEE scalar behavior.
    let mut arguments = vec!["-std=c17".into(), "-O2".into(), "-fwrapv".into()];
    arguments.extend(extra_arguments.iter().map(|argument| (*argument).to_owned()));
    #[cfg(target_os = "macos")]
    {
        let sdk = rust_oracle::run_tool(
            std::process::Command::new("xcrun").arg("--show-sdk-path"),
            "conditional oracle SDK",
        );
        arguments.extend(["-isysroot".into(), sdk.trim().into()]);
    }
    let frontend =
        inspect(&scanner, &header, &arguments, None).expect("inspect conditional fixture");
    let session =
        AnalysisSession::prepare(&scanner, &frontend, names).expect("prepare conditional fixture");
    let bindings = bindgen::Builder::default()
        .rust_target(bindgen::RustTarget::stable(85, 0).unwrap())
        .rust_edition(bindgen::RustEdition::Edition2024)
        .header(header.to_str().unwrap())
        .clang_args(&frontend.profile().arguments)
        .allowlist_type("Conditional.*")
        .allowlist_function("conditional_.*")
        .allowlist_var("conditional_.*")
        .layout_tests(false)
        .generate_comments(false)
        .formatter(bindgen::Formatter::None)
        .generate()
        .expect("generate conditional fixture bindings")
        .to_string();
    let catalog = binding_symbols::collect_bindings(
        &syn::parse_file(&bindings).unwrap(),
        session.integer_constants(),
        frontend.declarations(),
        &frontend.profile().target,
    );
    let artifact = emit_support_artifact_with_bindings(&session, names, &catalog)
        .expect("generate conditional support");
    let support = directory.join("../pgrx-pg-sys/src/c_macros/support.rs").canonicalize().unwrap();
    let mut rust = format!(
        "#![deny(unsafe_op_in_unsafe_fn)]\n#![allow(non_snake_case,non_camel_case_types,dead_code,unused_parens,unreachable_code)]\n#[path={support:?}] pub mod __pgrx_c_macros;\n{bindings}\n{}",
        artifact.rust
    );
    for emission in emit_batch_with_bindings(&session, names, &catalog) {
        let EmissionStatus::Emitted { rust: definition, .. } = &emission.status else {
            panic!("conditional macro must emit: {emission:?}")
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
            if emission.analysis.name == "COND_CAPTURE" { vec!["context"] } else { vec![] },
            "C branch locals must remain local and only free caller context becomes an operand"
        );
        if emission.analysis.name != "COND_USE" {
            assert!(emission.analysis.required_helpers.contains(&HelperRequirement::CTruth));
            assert!(
                emission
                    .analysis
                    .evaluation
                    .requirements
                    .contains(&EvaluationRequirement::PreserveLazyBranches)
            );
        }
        if matches!(emission.analysis.name.as_str(), "COND_DANGLING" | "COND_RAW_RETURN") {
            assert_eq!(emission.analysis.invocation, InvocationContract::ExplicitStatementBoundary);
        }
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

#[test]
fn conditional_statements_preserve_c_truth_lazy_effects_and_scopes() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let generated = generate(&[
        "COND_CHOOSE",
        "COND_REPEAT",
        "COND_DANGLING",
        "COND_BLOCKS",
        "COND_EMPTY",
        "COND_MUTATE",
        "COND_NATIVE",
        "COND_HOOK",
        "COND_SCOPES",
        "COND_INIT_BOTH",
        "COND_INIT_TEST",
        "COND_CAPTURE",
        "COND_USE",
    ]);
    let original = r#"
#include <stdio.h>
#include <math.h>
static void print_record(const ConditionalRecord *pointer) {
    printf("%u %u %u %u\n", (unsigned int) pointer->byte, pointer->total, conditional_trace, conditional_calls);
}
int main(void) {
    ConditionalRecord object = {0, 0};
    ConditionalRecord *pointer = &object;
    int integers[] = {0, 1, -1, -2147483647 - 1};
    for (unsigned int i = 0; i < 4; i++) { COND_CHOOSE(pointer, integers[i], 257, 258); print_record(pointer); }
    ConditionalRecord *pointers[] = {0, pointer};
    for (unsigned int i = 0; i < 2; i++) { COND_CHOOSE(pointer, pointers[i], 257, 258); print_record(pointer); }
    double floats[] = {0.0, -0.0, 0.5, -0.5, INFINITY, NAN};
    for (unsigned int i = 0; i < 6; i++) { COND_CHOOSE(pointer, floats[i], 257, 258); print_record(pointer); }
    COND_CHOOSE(pointer, conditional_record(1, 0), conditional_record(2, 300), conditional_record(3, 400)); print_record(pointer);
    COND_CHOOSE(pointer, conditional_record(1, 1), conditional_record(2, 300), conditional_record(3, 400)); print_record(pointer);
    unsigned int *missing = 0;
    COND_CHOOSE(pointer, 0, *missing, 257); print_record(pointer);
    COND_REPEAT(pointer, conditional_record(4, 1), conditional_record(5, 513)); print_record(pointer);
    COND_REPEAT(pointer, conditional_record(4, 0), conditional_record(5, 514)); print_record(pointer);
    object.total = 99;
    { COND_DANGLING(pointer, 0, conditional_record(6, 0)); } print_record(pointer);
    { COND_DANGLING(pointer, 1, conditional_record(6, 0)); } print_record(pointer);
    { COND_DANGLING(pointer, 1, conditional_record(6, 1)); } print_record(pointer);
    COND_BLOCKS(pointer, 0); COND_BLOCKS(pointer, 1);
    COND_EMPTY(conditional_record(7, 0)); COND_EMPTY(conditional_record(7, 1)); print_record(pointer);
    object.total = 0;
    COND_MUTATE(pointer); print_record(pointer);
    COND_MUTATE(pointer); print_record(pointer);
    conditional_trace = conditional_calls = object.total = 0;
    COND_NATIVE(pointer); print_record(pointer);
    ConditionalHooks hooks = {0};
    COND_HOOK(&hooks, pointer, conditional_record(6, 50)); print_record(pointer);
    hooks.callback = conditional_hook;
    COND_HOOK(&hooks, pointer, conditional_record(6, 50)); print_record(pointer);
    conditional_trace = conditional_calls = 0; object.total = 1;
    COND_NATIVE(pointer); print_record(pointer);
    COND_SCOPES(pointer, 1, conditional_record(8, 258)); print_record(pointer);
    COND_SCOPES(pointer, 0, conditional_record(8, 258)); print_record(pointer);
    COND_INIT_BOTH(pointer, 1, conditional_record(9, 61), conditional_record(8, 62)); print_record(pointer);
    COND_INIT_BOTH(pointer, 0, conditional_record(9, 61), conditional_record(8, 62)); print_record(pointer);
    COND_INIT_TEST(pointer, conditional_record(6, 0)); print_record(pointer);
    COND_INIT_TEST(pointer, conditional_record(6, 513)); print_record(pointer);
    ConditionalRecord *context = pointer;
    COND_CAPTURE(1, conditional_record(9, 81)); print_record(pointer);
    COND_CAPTURE(0, conditional_record(9, 81)); print_record(pointer);
}
"#;
    let consumer = r#"
/// # Safety
/// The caller excludes concurrent access to the initialized native counters.
unsafe fn print_record(object: &ConditionalRecord) {
    // SAFETY: The single-threaded executable owns these initialized C globals.
    let (trace, calls) = unsafe { (conditional_trace, conditional_calls) };
    println!("{} {} {} {}", object.byte, object.total, trace, calls);
}
fn main() {
    __pgrx_c_classify!(@if_available COND_CHOOSE { let available = true; });
    __pgrx_c_classify!(@if_available NEVER_EMITTED { compile_error!("absent C macro must not expose items"); });
    assert!(available);
    let mut object = ConditionalRecord { byte: 0, total: 0 };
    let pointer = &raw mut object;
    // SAFETY: pointer names the owned, aligned initialized record. Mutations
    // have exclusive access, and each temporary shared print borrow ends before
    // the following write. Native callbacks use only this record and process-local
    // scalar counters; this executable has one thread and no backend calls.
    unsafe {
        for condition in [0_i32, 1, -1, i32::MIN] { COND_CHOOSE!(pointer, condition, 257, 258); print_record(&object); }
        for condition in [core::ptr::null_mut::<ConditionalRecord>(), pointer] { COND_CHOOSE!(pointer, condition, 257, 258); print_record(&object); }
        for condition in [0.0_f64, -0.0, 0.5, -0.5, f64::INFINITY, f64::NAN] { COND_CHOOSE!(pointer, condition, 257, 258); print_record(&object); }
        COND_CHOOSE!(pointer, conditional_record(1, 0), conditional_record(2, 300), conditional_record(3, 400)); print_record(&object);
        COND_CHOOSE!(pointer, conditional_record(1, 1), conditional_record(2, 300), conditional_record(3, 400)); print_record(&object);
        let missing = core::ptr::null::<u32>();
        COND_CHOOSE!(pointer, 0, *missing, 257); print_record(&object);
        COND_REPEAT!(pointer, conditional_record(4, 1), conditional_record(5, 513)); print_record(&object);
        COND_REPEAT!(pointer, conditional_record(4, 0), conditional_record(5, 514)); print_record(&object);
        object.total = 99;
        COND_DANGLING!(@__pgrx_c_statement; pointer, 0, conditional_record(6, 0)); print_record(&object);
        COND_DANGLING!(@__pgrx_c_statement; pointer, 1, conditional_record(6, 0)); print_record(&object);
        COND_DANGLING!(@__pgrx_c_statement; pointer, 1, conditional_record(6, 1)); print_record(&object);
        COND_BLOCKS!(pointer, 0); COND_BLOCKS!(pointer, 1);
        let _: () = COND_EMPTY!(conditional_record(7, 0));
        COND_EMPTY!(@__pgrx_c_discard; conditional_record(7, 1)); print_record(&object);
        object.total = 0;
        COND_MUTATE!(pointer); print_record(&object);
        COND_MUTATE!(pointer); print_record(&object);
        conditional_trace = 0; conditional_calls = 0; object.total = 0;
        COND_NATIVE!(pointer); print_record(&object);
        let mut hooks = ConditionalHooks { callback: None };
        let hooks_pointer = &raw mut hooks;
        COND_HOOK!(hooks_pointer, pointer, conditional_record(6, 50)); print_record(&object);
        hooks.callback = Some(conditional_hook);
        COND_HOOK!(hooks_pointer, pointer, conditional_record(6, 50)); print_record(&object);
        conditional_trace = 0; conditional_calls = 0; object.total = 1;
        COND_NATIVE!(pointer); print_record(&object);
        COND_SCOPES!(pointer, 1, conditional_record(8, 258)); print_record(&object);
        COND_SCOPES!(pointer, 0, conditional_record(8, 258)); print_record(&object);
        COND_INIT_BOTH!(pointer, 1, conditional_record(9, 61), conditional_record(8, 62)); print_record(&object);
        COND_INIT_BOTH!(pointer, 0, conditional_record(9, 61), conditional_record(8, 62)); print_record(&object);
        COND_INIT_TEST!(pointer, conditional_record(6, 0)); print_record(&object);
        COND_INIT_TEST!(pointer, conditional_record(6, 513)); print_record(&object);
        COND_CAPTURE!(1, conditional_record(9, 81), pointer); print_record(&object);
        COND_CAPTURE!(0, conditional_record(9, 81), pointer); print_record(&object);
    }
}
"#;
    generated.compare(original, consumer, 35);

    for invocation in [
        "COND_SCOPES!(pointer, 0, then_local)",
        "COND_SCOPES!(pointer, 1, r#else_local)",
        "forward!(pointer, (then_local))",
    ] {
        let diagnostic = rust_oracle::reject_rust(&format!(
            "{}\nmacro_rules! forward {{ ($pointer:expr,$input:expr) => {{ COND_SCOPES!($pointer,1,$input) }}; }}\nfn main() {{ let mut object=ConditionalRecord{{byte:0,total:0}}; let pointer=&raw mut object; let then_local=3_u32; let else_local=4_u32; unsafe {{ {invocation}; }} }}",
            generated.rust,
        ));
        assert!(
            diagnostic.contains("C macro argument mentions"),
            "branch-local capture must reject even on an unselected branch: {diagnostic}"
        );
    }
    let diagnostic = rust_oracle::reject_rust(&format!(
        "{}\nfn main() {{ let mut object=ConditionalRecord{{byte:0,total:0}}; let pointer=&raw mut object; unsafe {{ COND_DANGLING!(pointer,1,0); }} }}",
        generated.rust,
    ));
    assert!(
        diagnostic.contains("@__pgrx_c_statement"),
        "unbraced C statements require an explicit invocation boundary: {diagnostic}"
    );
    let diagnostic = rust_oracle::reject_rust(&format!(
        "{}\nfn main() {{ let _ = COND_USE!(COND_EMPTY!(1)); }}",
        generated.rust,
    ));
    assert!(
        diagnostic.contains("not an expression operand"),
        "conditional statements cannot supply C expression operands: {diagnostic}"
    );
}

#[test]
fn conditional_returns_exit_the_caller_and_convert_only_the_selected_value() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let generated = generate(&[
        "COND_RETURN_ALL",
        "COND_RAW_RETURN",
        "COND_RETURN_IF",
        "COND_INIT_SURVIVOR",
        "COND_RETURN_ALIAS",
    ]);
    let original = r#"
#include <stdio.h>
static unsigned char byte_result(int condition) {
    COND_RETURN_ALL(condition, conditional_record(1, 257), conditional_record(2, 258));
    conditional_record(9, 0); return 99;
}
static unsigned long marker_result(int condition) {
    COND_RETURN_ALL(condition, -1, 256);
}
static unsigned long raw_marker_result(int condition) {
    { COND_RAW_RETURN(condition); }
}
static unsigned short sometimes(ConditionalRecord *pointer, int condition) {
    COND_RETURN_IF(pointer, condition, -1);
    conditional_record(3, 0); return 77;
}
static unsigned char surviving(ConditionalRecord *pointer, int condition) {
    COND_INIT_SURVIVOR(pointer, condition, conditional_record(4, 511));
}
static unsigned char alias_result(int condition) {
    COND_RETURN_ALIAS(condition, conditional_record(5, 259), conditional_record(6, 260));
}
int main(void) {
    ConditionalRecord object = {0, 0};
    for (int condition = 0; condition < 2; condition++) {
        unsigned int result = byte_result(condition);
        printf("%u %u %u\n", result, conditional_trace, conditional_calls);
    }
    printf("%lu %lu %lu %lu\n", marker_result(0), marker_result(1), raw_marker_result(0), raw_marker_result(1));
    for (int condition = 0; condition < 2; condition++) {
        unsigned int result = sometimes(&object, condition);
        printf("%u %u %u %u\n", result, object.total, conditional_trace, conditional_calls);
    }
    for (int condition = 0; condition < 2; condition++) {
        unsigned int result = surviving(&object, condition);
        printf("%u %u %u %u\n", result, object.total, conditional_trace, conditional_calls);
    }
    for (int condition = 0; condition < 2; condition++) {
        unsigned int result = alias_result(condition);
        printf("%u %u %u\n", result, conditional_trace, conditional_calls);
    }
}
"#;
    let consumer = r#"
/// # Safety
/// The caller owns exclusive access to the native oracle counters.
unsafe fn byte_result(condition: i32) -> u8 {
    // SAFETY: native callbacks mutate only initialized process-local counters.
    unsafe {
        COND_RETURN_ALL!(condition, conditional_record(1, 257), conditional_record(2, 258));
        conditional_record(9, 0);
    }
    99
}
fn marker_result(condition: i32) -> core::ffi::c_ulong {
    COND_RETURN_ALL!(@__pgrx_c_return_as [__pgrx_c_macros::CUnsignedLong]; condition, -1, 256);
}
fn raw_marker_result(condition: i32) -> core::ffi::c_ulong {
    COND_RAW_RETURN!(@__pgrx_c_statement; @__pgrx_c_return_as [__pgrx_c_macros::CUnsignedLong]; condition);
}
/// # Safety
/// pointer names a live aligned initialized exclusively writable record.
/// The caller also excludes concurrent native counter accesses.
unsafe fn sometimes(pointer: *mut ConditionalRecord, condition: i32) -> u16 {
    // SAFETY: the caller establishes record access and native counter exclusivity.
    unsafe { COND_RETURN_IF!(pointer, condition, -1); conditional_record(3, 0); }
    77
}
/// # Safety
/// pointer names a live aligned initialized exclusively writable record.
/// The caller also excludes concurrent native counter accesses.
unsafe fn surviving(pointer: *mut ConditionalRecord, condition: i32) -> u8 {
    // SAFETY: the caller establishes record access and native counter exclusivity.
    unsafe { COND_INIT_SURVIVOR!(pointer, condition, conditional_record(4, 511)); }
    99
}
/// # Safety
/// The caller owns exclusive access to the native oracle counters.
unsafe fn alias_result(condition: i32) -> u8 {
    // SAFETY: native callbacks mutate only initialized process-local counters.
    unsafe { COND_RETURN_ALIAS!(condition, conditional_record(5, 259), conditional_record(6, 260)); }
    99
}
fn main() {
    let mut object = ConditionalRecord { byte: 0, total: 0 };
    let pointer = &raw mut object;
    // SAFETY: the owned record is aligned and fully initialized; each callback
    // has exclusive access while it runs. Counters are process-local initialized
    // C scalars, and this executable has one thread and no backend functions.
    unsafe {
        for condition in [0_i32, 1] {
            let result = byte_result(condition);
            let (trace, calls) = (conditional_trace, conditional_calls);
            println!("{} {} {}", result, trace, calls);
        }
        println!("{} {} {} {}", marker_result(0), marker_result(1), raw_marker_result(0), raw_marker_result(1));
        for condition in [0_i32, 1] {
            let result = sometimes(pointer, condition);
            let (trace, calls) = (conditional_trace, conditional_calls);
            println!("{} {} {} {}", result, object.total, trace, calls);
        }
        for condition in [0_i32, 1] {
            let result = surviving(pointer, condition);
            let (trace, calls) = (conditional_trace, conditional_calls);
            println!("{} {} {} {}", result, object.total, trace, calls);
        }
        for condition in [0_i32, 1] {
            let result = alias_result(condition);
            let (trace, calls) = (conditional_trace, conditional_calls);
            println!("{} {} {}", result, trace, calls);
        }
    }
}
"#;
    generated.compare(original, consumer, 9);

    let diagnostic = rust_oracle::reject_rust(&format!(
        "{}\nfn invalid(condition:i32) -> *mut i32 {{ COND_RETURN_ALL!(condition,1,2); }}\nfn main() {{}}",
        generated.rust,
    ));
    assert!(
        diagnostic.contains("ImplicitTo"),
        "conditional return uses C assignment conversion, which rejects nonzero integers to pointers: {diagnostic}"
    );
}

#[test]
fn compound_floating_conditions_require_a_profile_without_contraction() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let names = ["COND_FLOAT_ADD", "COND_FLOAT_MUL"];
    let generated = generate_with_arguments(&names, &["-ffp-contract=off"]);
    let original = r#"
#include <stdio.h>
#include <stdint.h>
#include <string.h>
static void print_value(double value) {
    uint64_t bits;
    memcpy(&bits, &value, sizeof bits);
    printf("%016llx %u %u %u %u\n", (unsigned long long) bits, conditional_float_left, conditional_float_right, conditional_float_places, conditional_calls);
}
static void reset_counts(void) {
    conditional_float_left = conditional_float_right = conditional_float_places = conditional_calls = 0;
}
int main(void) {
    for (unsigned int condition = 0; condition < 2; condition++) {
        double value = -1.0;
        reset_counts();
        COND_FLOAT_ADD(conditional_float_place(&value), conditional_record(1, condition), conditional_float_record(0, 0x1.0000002p0), conditional_float_record(1, 0x1.ffffffcp-1));
        print_value(value);
    }
    for (unsigned int condition = 0; condition < 2; condition++) {
        double value = 0x1.0000002p0;
        reset_counts();
        COND_FLOAT_MUL(conditional_float_place(&value), conditional_record(1, condition), conditional_float_record(0, 1.0), conditional_float_record(1, -0x1p-27));
        print_value(value);
    }
}
"#;
    let consumer = r#"
/// # Safety
/// The caller excludes concurrent access to the initialized oracle counters.
unsafe fn print_value(value: f64, selected: u32) {
    // SAFETY: The single-threaded executable owns these initialized C globals.
    let (left, right, places, calls) = unsafe {
        (conditional_float_left, conditional_float_right, conditional_float_places, conditional_calls)
    };
    println!("{:016x} {} {} {} {}", value.to_bits(), left, right, places, calls);
    assert_eq!((left, right, places), (selected, selected, selected), "each selected operand once");
    assert_eq!(calls, 1, "condition once");
}
/// # Safety
/// The caller excludes concurrent access to the initialized oracle counters.
unsafe fn reset_counts() {
    // SAFETY: This executable owns exclusive access to initialized C globals.
    unsafe { conditional_float_left = 0; conditional_float_right = 0; conditional_float_places = 0; conditional_calls = 0; }
}
fn main() {
    // These caller f64 operands have exact binary64 encodings. The first case
    // distinguishes separately rounded multiply/add from a fused operation:
    // (-1) + (1 + 2^-27) * (1 - 2^-27) rounds to zero only when unfused.
    let above_one = f64::from_bits(0x3ff0000002000000);
    let below_one = f64::from_bits(0x3feffffffc000000);
    let negative_epsilon = -f64::from_bits(0x3e40000000000000);
    // SAFETY: Every pointer identifies an owned, aligned initialized f64 slot
    // with exclusive write access. Native operand callbacks use only initialized
    // process-local counters in this single-threaded executable. No backend or
    // floating-environment mutation occurs, and all arithmetic inputs are finite.
    unsafe {
        for condition in [0_u32, 1] {
            let mut value = -1.0_f64;
            let pointer = &raw mut value;
            reset_counts();
            COND_FLOAT_ADD!(conditional_float_place(pointer), conditional_record(1, condition), conditional_float_record(0, above_one), conditional_float_record(1, below_one));
            print_value(value, condition);
        }
        for condition in [0_u32, 1] {
            let mut value = above_one;
            let pointer = &raw mut value;
            reset_counts();
            COND_FLOAT_MUL!(conditional_float_place(pointer), conditional_record(1, condition), conditional_float_record(0, 1.0), conditional_float_record(1, negative_epsilon));
            print_value(value, condition);
        }
    }
}
"#;
    generated.compare(original, consumer, 4);

    let generated = generate_with_arguments(&names, &["-ffp-contract=on"]);
    // The same generic C syntax still admits integer instantiations: floating
    // contraction has no effect on the compiler-established unsigned operations.
    generated.compare(
        r#"
#include <stdio.h>
int main(void) {
    unsigned int value = 4;
    COND_FLOAT_ADD(&value, conditional_record(1, 1), conditional_record(2, 2), conditional_record(3, 3));
    COND_FLOAT_MUL(&value, conditional_record(1, 1), conditional_record(2, 2), conditional_record(3, 3));
    printf("%u %u\n", value, conditional_calls);
}
"#,
        r#"
fn main() {
    let mut value = 4_u32;
    let pointer = &raw mut value;
    // SAFETY: pointer identifies owned aligned initialized exclusive u32 storage.
    // Native callbacks mutate only this executable's initialized local counters.
    unsafe {
        COND_FLOAT_ADD!(pointer, conditional_record(1, 1), conditional_record(2, 2), conditional_record(3, 3));
        COND_FLOAT_MUL!(pointer, conditional_record(1, 1), conditional_record(2, 2), conditional_record(3, 3));
        let calls = conditional_calls;
        println!("{} {}", value, calls);
        assert_eq!(calls, 6);
    }
}
"#,
        1,
    );
    for invocation in [
        "COND_FLOAT_ADD!(pointer, 1, left, right)",
        "COND_FLOAT_MUL!(pointer, 1, left, right)",
        "COND_FLOAT_ADD!(pointer, 1, 2_i32, 3_i32)",
        "COND_FLOAT_MUL!(pointer, 1, 2_i32, 3_i32)",
    ] {
        let diagnostic = rust_oracle::reject_rust(&format!(
            "{}\nfn main() {{ let mut value=4.0_f64; let pointer=&raw mut value; let left=1.5_f64; let right=2.5_f64; unsafe {{ {invocation}; }} }}",
            generated.rust,
        ));
        assert!(
            diagnostic.contains("CDouble") && diagnostic.contains("AllowedProfile<false>"),
            "caller floating operands and destination must reject the uncertified contraction profile: {diagnostic}"
        );
    }
}

#[test]
fn local_reads_need_initialization_on_every_fallthrough_branch() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let header = directory.join("tests/fixtures/conditional_oracle.h");
    let scanner = MacroScanner::new().expect("libclang required");
    let names = ["COND_INIT_ONE", "COND_INIT_COMPOUND", "COND_INIT_CONDITION"];
    let frontend = inspect(&scanner, &header, &["-std=c17".into()], None).unwrap();
    let session = AnalysisSession::prepare(&scanner, &frontend, &names).unwrap();
    for name in names {
        let emission = pgrx_c_macros::emit(&session, name);
        let EmissionStatus::Skipped { reason } = emission.status else {
            panic!("uninitialized conditional local read must not emit: {emission:?}")
        };
        assert!(
            reason.message.contains("initializ"),
            "uninitialized branch read must fail for its actual reason: {reason:?}"
        );
    }
}
