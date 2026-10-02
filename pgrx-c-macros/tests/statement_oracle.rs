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
    AnalysisSession, EmissionStatus, MacroScanner, ParameterOrigin, emit_batch_with_bindings,
    emit_support_artifact_with_bindings, inspect,
};
use std::path::PathBuf;
use std::sync::Mutex;

static SCANNER_LOCK: Mutex<()> = Mutex::new(());

fn is_builtin_oid(name: &str) -> bool {
    name.ends_with("OID") && name != "HEAP_HASOID"
        || name.ends_with("RelationId")
        || name == "TemplateDbOid"
}

const NAMES: &[&str] = &[
    "STMT_ASSIGN",
    "STMT_LOCAL",
    "STMT_NESTED",
    "STMT_ALIAS",
    "STMT_CAPTURE",
    "STMT_EMPTY",
    "STMT_EMPTY_BLOCK",
    "STMT_NOTHING",
    "STMT_VOLATILE",
    "STMT_VOID",
    "STMT_DISCARD",
    "STMT_USE",
    "STMT_BRANCH",
];

const NATIVE: &str = r#"
unsigned int statement_trace;
unsigned int statement_calls;
volatile unsigned int statement_signal;
unsigned int statement_record(unsigned int digit, unsigned int value) {
    statement_trace = statement_trace * 10 + digit;
    statement_calls++;
    return value;
}
void statement_store(StatementRecord *pointer, unsigned int value) {
    statement_trace = statement_trace * 10 + 8;
    statement_calls++;
    pointer->total = value;
}
"#;

#[test]
fn statement_blocks_preserve_c_assignment_conversion_order_and_local_storage() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let header = directory.join("tests/fixtures/statement_oracle.h");
    let scanner = MacroScanner::new().expect("libclang required");
    let mut arguments = vec!["-std=c17".into(), "-O2".into(), "-fwrapv".into()];
    #[cfg(target_os = "macos")]
    {
        let sdk = rust_oracle::run_tool(
            std::process::Command::new("xcrun").arg("--show-sdk-path"),
            "statement oracle SDK",
        );
        arguments.extend(["-isysroot".into(), sdk.trim().into()]);
    }
    let frontend = inspect(&scanner, &header, &arguments, None).expect("inspect statements");
    let session = AnalysisSession::prepare(&scanner, &frontend, NAMES).expect("prepare statements");
    let bindings = bindgen::Builder::default()
        .rust_target(bindgen::RustTarget::stable(85, 0).unwrap())
        .rust_edition(bindgen::RustEdition::Edition2024)
        .header(header.to_str().unwrap())
        .clang_args(&frontend.profile().arguments)
        .allowlist_type("StatementRecord")
        .allowlist_function("statement_.*")
        .allowlist_var("statement_.*")
        .layout_tests(false)
        .generate_comments(false)
        .formatter(bindgen::Formatter::None)
        .generate()
        .expect("generate statement oracle bindings")
        .to_string();
    let catalog = binding_symbols::collect_bindings(
        &syn::parse_file(&bindings).unwrap(),
        session.integer_constants(),
        frontend.declarations(),
        &frontend.profile().target,
    );
    let artifact = emit_support_artifact_with_bindings(&session, NAMES, &catalog)
        .expect("generate statement oracle support");
    let support = directory.join("../pgrx-pg-sys/src/c_macros/support.rs").canonicalize().unwrap();
    let mut rust = format!(
        "#![deny(unsafe_op_in_unsafe_fn)]\n#![allow(non_snake_case,non_camel_case_types,dead_code,unused_parens)]\n#[path={support:?}] pub mod __pgrx_c_macros;\n{bindings}\n{}",
        artifact.rust
    );
    for emission in emit_batch_with_bindings(&session, NAMES, &catalog) {
        let EmissionStatus::Emitted { rust: definition, .. } = &emission.status else {
            panic!("statement macro must emit: {emission:?}")
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
            if emission.analysis.name == "STMT_CAPTURE" { vec!["scope"] } else { vec![] },
            "declaration names must remain local; only caller context becomes an operand"
        );
        rust.push_str(definition);
    }
    assert!(rust.contains("CVolatile"), "generated storage must retain the C qualifier");
    let original = r#"
#include <stdio.h>
int main(void) {
    StatementRecord object = {0, 0, 0};
    StatementRecord *pointer = &object;
    STMT_ASSIGN(pointer, statement_record(1, 257));
    printf("%u %u %u %u %u\n", (unsigned int) object.byte, (unsigned int) object.flag, object.total, statement_trace, statement_calls);
    STMT_ALIAS(pointer, statement_record(2, 256));
    printf("%u %u %u %u %u\n", (unsigned int) object.byte, (unsigned int) object.flag, object.total, statement_trace, statement_calls);
    STMT_LOCAL(pointer, statement_record(3, 99));
    printf("%u %u %u %u %u\n", (unsigned int) object.byte, (unsigned int) object.flag, object.total, statement_trace, statement_calls);
    STMT_NESTED(pointer, statement_record(4, 20));
    printf("%u %u %u %u %u\n", (unsigned int) object.byte, (unsigned int) object.flag, object.total, statement_trace, statement_calls);
    STMT_EMPTY(); STMT_EMPTY_BLOCK(); STMT_NOTHING(statement_record(5, 700));
    StatementRecord *scope = pointer;
    STMT_CAPTURE(statement_record(6, 9));
    unsigned int continued = 1;
    printf("%u %u %u %u %u %u\n", (unsigned int) object.byte, (unsigned int) object.flag, object.total, statement_trace, statement_calls, continued);
    STMT_VOID(pointer, statement_record(7, 513));
    printf("%u %u %u %u %u\n", (unsigned int) object.byte, (unsigned int) object.flag, object.total, statement_trace, statement_calls);
    statement_signal = 0xFFFFFFFFU;
    STMT_VOLATILE();
    printf("%u\n", statement_signal);
    STMT_DISCARD(pointer);
    printf("%u %u\n", (unsigned int) object.byte, statement_signal);
    STMT_BRANCH((StatementRecord *) 0);
    STMT_BRANCH(pointer);
    printf("%u\n", object.total);
}
"#;
    let consumer = r#"
fn main() {
    let mut object = StatementRecord { byte: 0, flag: false, total: 0 };
    let pointer = &raw mut object;
    // SAFETY: pointer identifies the live initialized local record with proper
    // alignment and exclusive write access. The macro-local pointer aliases
    // remain within their live initialized slots. Native calls only mutate
    // initialized process-local counters; this executable has one thread.
    unsafe {
        STMT_ASSIGN!(pointer, statement_record(1, 257));
        let (trace, calls) = (statement_trace, statement_calls);
        println!("{} {} {} {} {}", object.byte, u8::from(object.flag), object.total, trace, calls);
        STMT_ALIAS!(pointer, statement_record(2, 256));
        let (trace, calls) = (statement_trace, statement_calls);
        println!("{} {} {} {} {}", object.byte, u8::from(object.flag), object.total, trace, calls);
        STMT_LOCAL!(pointer, statement_record(3, 99));
        let (trace, calls) = (statement_trace, statement_calls);
        println!("{} {} {} {} {}", object.byte, u8::from(object.flag), object.total, trace, calls);
        STMT_NESTED!(pointer, statement_record(4, 20));
        let (trace, calls) = (statement_trace, statement_calls);
        println!("{} {} {} {} {}", object.byte, u8::from(object.flag), object.total, trace, calls);
        let _: () = STMT_EMPTY!();
        let _: () = STMT_EMPTY_BLOCK!();
        STMT_NOTHING!(statement_record(5, 700));
        STMT_EMPTY!(@__pgrx_c_discard;);
        STMT_CAPTURE!(statement_record(6, 9), pointer);
        let continued = 1_u32;
        let (trace, calls) = (statement_trace, statement_calls);
        println!("{} {} {} {} {} {}", object.byte, u8::from(object.flag), object.total, trace, calls, continued);
        STMT_VOID!(pointer, statement_record(7, 513));
        let (trace, calls) = (statement_trace, statement_calls);
        println!("{} {} {} {} {}", object.byte, u8::from(object.flag), object.total, trace, calls);
        core::ptr::write_volatile(core::ptr::addr_of_mut!(statement_signal), u32::MAX);
        STMT_VOLATILE!();
        let signal = core::ptr::read_volatile(core::ptr::addr_of!(statement_signal));
        println!("{}", signal);
        STMT_DISCARD!(pointer);
        let signal = core::ptr::read_volatile(core::ptr::addr_of!(statement_signal));
        println!("{} {}", object.byte, signal);
        STMT_BRANCH!(core::ptr::null_mut::<StatementRecord>());
        STMT_BRANCH!(pointer);
        println!("{}", object.total);
    }
}
"#;
    let profile = frontend.profile();
    let arguments = profile.arguments.iter().map(String::as_str).collect::<Vec<_>>();
    let native = format!("{NATIVE}\n{}", artifact.c_source);
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
    assert_eq!(emitted, original, "statement sequencing and C conversions must match");
    assert_eq!(emitted.lines().count(), 9);

    for invocation in [
        "STMT_LOCAL!(pointer, temporary)",
        "STMT_NESTED!(pointer, r#temporary)",
        "forward!(pointer, (temporary))",
    ] {
        let diagnostic = rust_oracle::reject_rust(&format!(
            "{rust}\nmacro_rules! forward {{ ($pointer:expr, $input:expr) => {{ STMT_LOCAL!($pointer, $input) }}; }}\nfn main() {{ let mut object=StatementRecord{{byte:0,flag:false,total:0}}; let pointer=&raw mut object; let temporary=7_u32; unsafe {{ {invocation}; }} }}"
        ));
        assert!(
            diagnostic.contains("C macro argument mentions"),
            "local capture must be rejected before Rust expression capture: {diagnostic}"
        );
    }
    let diagnostic =
        rust_oracle::reject_rust(&format!("{rust}\nfn main() {{ let _: i32 = STMT_EMPTY!(); }}"));
    assert!(diagnostic.contains("E0308"), "a statement block must yield unit: {diagnostic}");
    let diagnostic = rust_oracle::reject_rust(&format!(
        "{rust}\nfn main() {{ let mut object=StatementRecord{{byte:0,flag:false,total:0}}; let pointer=&raw mut object; unsafe {{ let _=STMT_USE!(STMT_ASSIGN!(pointer,1)); }} }}"
    ));
    assert!(
        diagnostic.contains("not an expression operand"),
        "C statement blocks cannot be consumed as C values: {diagnostic}"
    );
    for mode in ["size", "place", "read_place"] {
        let diagnostic = rust_oracle::reject_rust(&format!(
            "{rust}\nfn main() {{ let _ = STMT_EMPTY!(@__pgrx_c_{mode};); }}"
        ));
        assert!(
            diagnostic.contains("not an expression operand"),
            "statement blocks cannot supply C {mode} operands: {diagnostic}"
        );
    }
}

#[test]
fn unsupported_control_flow_and_uninitialized_local_reads_remain_skipped() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let header = directory.join("tests/fixtures/statement_oracle.h");
    let scanner = MacroScanner::new().expect("libclang required");
    let names = ["STMT_UNINITIALIZED", "STMT_UNINITIALIZED_COMPOUND", "STMT_UNSUPPORTED_LOOP"];
    let frontend = inspect(&scanner, &header, &["-std=c17".into()], None).unwrap();
    let session = AnalysisSession::prepare(&scanner, &frontend, &names).unwrap();
    for name in names {
        let emission = pgrx_c_macros::emit(&session, name);
        let EmissionStatus::Skipped { reason } = emission.status else {
            panic!("unsupported or uninitialized statement macro must not emit: {emission:?}")
        };
        if name.starts_with("STMT_UNINITIALIZED") {
            assert!(
                reason.message.contains("initializ"),
                "uninitialized local read must fail for its actual reason: {reason:?}"
            );
        }
    }
}
