//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Verify value, place, discard, and unevaluated nested-macro contexts.
//!
//! The same inner C macro can require different treatment under assignment,
//! address-taking, sizeof, or ordinary value use. C/Rust comparisons track both
//! results and effects, while rejected consumers enforce lvalue and unsafe rules.
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
    AnalysisSession, BindingCatalog, EmissionStatus, FrontendOutput, MacroScanner,
    emit_batch_with_bindings, emit_support_artifact_with_bindings, inspect,
};
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

/// Serialize libclang-backed inspection within this test process because its safe runtime
/// permits one active owner.
static SCANNER_LOCK: Mutex<()> = Mutex::new(());
/// Fixture macros whose emitted behavior is compared with the original header.
const SUPPORTED: &[&str] = &[
    "CTX_FIELD",
    "CTX_ARRAY",
    "CTX_BIT",
    "CTX_VOLATILE",
    "CTX_IDENTITY",
    "CTX_SET",
    "CTX_MOD",
    "CTX_POST",
    "CTX_PRE",
    "CTX_ADDRESS",
    "CTX_SIZE",
    "CTX_VOID",
    "CTX_COMMA",
    "CTX_LAZY",
    "CTX_REPEAT",
    "CTX_ATOMIC",
    "CTX_TYPE_PROVEN",
    "CTX_IGNORE",
];

// Required by the actual production binding collector, whose own tests also run here.
/// Classify PostgreSQL OID constants so fixture bindgen uses the same checked-wrapper boundary
/// as the real binding build.
fn is_builtin_oid(name: &str) -> bool {
    name.ends_with("OID") && name != "HEAP_HASOID"
        || name.ends_with("RelationId")
        || name == "TemplateDbOid"
}

/// Return the original fixture header whose preprocessing and C definitions supply this test's
/// semantics.
fn header() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/context_oracle.h")
}

/// Supply native compiler arguments for the oracle, including the platform SDK needed by the
/// original header.
fn native_arguments() -> Vec<String> {
    let mut arguments = vec![
        "-std=c17".into(),
        "-ffp-contract=off".into(),
        "-Werror=shadow".into(),
        "-Werror=uninitialized".into(),
    ];
    #[cfg(target_os = "macos")]
    {
        let sdk = rust_oracle::run_tool(Command::new("xcrun").arg("--show-sdk-path"), "sdk");
        arguments.extend(["-isysroot".into(), sdk.trim().into()]);
    }
    arguments
}

/// Generate and collect fresh fixture bindings, preserving the actual Rust ABI and paths that
/// the emitter must reconcile with C.
fn original_bindings(
    frontend: &FrontendOutput,
    session: &AnalysisSession<'_>,
) -> (String, BindingCatalog) {
    let source = bindgen::Builder::default()
        .rust_target(bindgen::RustTarget::stable(85, 0).expect("Rust 2024 requires Rust 1.85"))
        .rust_edition(bindgen::RustEdition::Edition2024)
        .header(header().to_str().unwrap())
        .clang_args(&frontend.profile().arguments)
        .allowlist_function("context_.*")
        .allowlist_type("ContextRecord")
        .layout_tests(false)
        .generate_comments(false)
        .formatter(bindgen::Formatter::None)
        .generate()
        .expect("bind the original native context declarations")
        .to_string();
    let syntax = syn::parse_file(&source).expect("parse actual bindgen context declarations");
    let catalog = binding_symbols::collect_bindings(
        &syntax,
        session.integer_constants(),
        frontend.declarations(),
        &frontend.profile().target,
    );
    assert_eq!(catalog.functions.len(), 3, "actual fixture prototypes must all bind");
    (source, catalog)
}

/// Assemble emitted public macros with their real native support for the independently compiled
/// Rust consumer.
fn emitted_source(
    session: &AnalysisSession<'_>,
    bindings: &str,
    catalog: &BindingCatalog,
) -> (String, String) {
    let support = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../pgrx-pg-sys/src/c_macros/support.rs")
        .canonicalize()
        .unwrap();
    let mut rust = format!(
        "#![allow(non_snake_case, non_camel_case_types, dead_code, unused_parens, unused_unsafe)]\n\
         #[path = {support:?}]\npub mod __pgrx_c_macros;\n{bindings}\n"
    );
    let artifact = emit_support_artifact_with_bindings(session, SUPPORTED, catalog)
        .expect("derive compiler-verified context and native bitfield adapters");
    assert!(!artifact.c_source.is_empty(), "bitfield primitives must use original C accessors");
    rust.push_str(&artifact.rust);
    for emission in emit_batch_with_bindings(session, SUPPORTED, catalog).unwrap() {
        let EmissionStatus::Emitted { rust: definition, .. } = emission.status else {
            panic!("context macro must emit: {emission:?}")
        };
        rust.push_str(&definition);
    }
    (rust, format!("{}\n{}", artifact.c_source, include_str!("fixtures/context_oracle.c")))
}

/// Checks that nested macro contexts match original C values types and evaluation.
#[test]
fn nested_macro_contexts_match_original_c_values_types_and_evaluation() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let frontend = inspect(&scanner, &header(), &native_arguments(), None).unwrap();
    let session = AnalysisSession::prepare(&scanner, &frontend, SUPPORTED).unwrap();
    let (bindings, catalog) = original_bindings(&frontend, &session);
    let (mut rust, native) = emitted_source(&session, &bindings, &catalog);
    rust.push_str(include_str!("fixtures/context_oracle.rs"));
    let profile = frontend.profile();
    let arguments = profile.arguments.iter().map(String::as_str).collect::<Vec<_>>();
    let generated = rust_oracle::run_rust_linked(
        &rust,
        &profile.compiler.executable,
        &header(),
        &native,
        &arguments,
    );
    let mut original_arguments = arguments.clone();
    original_arguments.push("-DPGRX_CONTEXT_ORACLE_MAIN");
    let original = oracle::run_c(
        &profile.compiler.executable,
        &header(),
        include_str!("fixtures/context_oracle.c"),
        &original_arguments,
        true,
    );
    assert_eq!(original.lines().count(), 44, "context corpus completeness");
    assert_eq!(generated, original, "nested context values and evaluations must agree with C");

    // Check the observable volatile effect independently of values: plain discard
    // must read, whereas sizeof must not even form an evaluated field access.
    let directory = TemporaryDirectory::new();
    let source = directory.0.join("contexts.rs");
    fs::write(&source, rust).unwrap();
    let llvm = rust_oracle::run_tool(
        Command::new("rustc")
            .args(["--edition=2024", "-O", "--emit=llvm-ir", "-o", "-"])
            .arg(&source),
        "context_llvm",
    );
    let original_llvm = rust_oracle::run_tool(
        Command::new(&profile.compiler.executable)
            .args(&arguments)
            .args(["-x", "c", "-S", "-emit-llvm", "-O2", "-o", "-", "-include"])
            .arg(header())
            .arg(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/context_oracle.c")),
        "original_context_llvm",
    );
    let original_volatile = function_body(&original_llvm, "context_native_observe_volatile")
        .matches("load volatile")
        .count();
    assert_eq!(original_volatile, 1, "the original C discarded value must read volatile storage");
    assert_eq!(
        function_body(&llvm, "context_observe_volatile").matches("load volatile").count(),
        original_volatile
    );
    let unevaluated = function_body(&llvm, "context_observe_size");
    assert!(!unevaluated.contains("load "), "sizeof must not access any storage: {unevaluated}");
    assert!(!unevaluated.contains("call "), "sizeof must not evaluate an operand: {unevaluated}");
    let original_size = function_body(&original_llvm, "context_native_observe_size");
    assert!(!original_size.contains("load ") && !original_size.contains("call "));
}

/// Checks that invalid C lvalue contexts and unsafe obligations are rejected.
#[test]
fn invalid_c_lvalue_contexts_and_unsafe_obligations_are_rejected() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let frontend = inspect(&scanner, &header(), &native_arguments(), None).unwrap();
    let session = AnalysisSession::prepare(&scanner, &frontend, SUPPORTED).unwrap();
    let (bindings, catalog) = original_bindings(&frontend, &session);
    let (rust, _) = emitted_source(&session, &bindings, &catalog);
    let profile = frontend.profile();
    let arguments = profile.arguments.iter().map(String::as_str).collect::<Vec<_>>();
    for (label, c, invocation) in [
        ("bitfield sizeof", "CTX_SIZE(CTX_BIT(p))", "CTX_SIZE!(CTX_BIT!(pointer))"),
        (
            "identity bitfield sizeof",
            "CTX_SIZE(CTX_IDENTITY(CTX_BIT(p)))",
            "CTX_SIZE!(CTX_IDENTITY!(CTX_BIT!(pointer)))",
        ),
        ("bitfield address", "CTX_ADDRESS(CTX_BIT(p))", "CTX_ADDRESS!(CTX_BIT!(pointer))"),
        (
            "assignment address",
            "CTX_ADDRESS(CTX_SET(CTX_FIELD(p), 3))",
            "CTX_ADDRESS!(CTX_SET!(CTX_FIELD!(pointer), 3_i32))",
        ),
        (
            "comma address",
            "CTX_ADDRESS(CTX_COMMA(0, CTX_FIELD(p)))",
            "CTX_ADDRESS!(CTX_COMMA!(0_i32, CTX_FIELD!(pointer)))",
        ),
        (
            "conditional address",
            "CTX_ADDRESS(CTX_LAZY(1, CTX_FIELD(p), CTX_FIELD(p)))",
            "CTX_ADDRESS!(CTX_LAZY!(1_i32, CTX_FIELD!(pointer), CTX_FIELD!(pointer)))",
        ),
        ("const mutation", "CTX_SET(CTX_FIELD(p), 3)", "CTX_SET!(CTX_FIELD!(pointer), 3_i32)"),
    ] {
        let is_const = label == "const mutation";
        let c_qualifier = if is_const { "const " } else { "" };
        rust_oracle::reject_c_invocation(
            &profile.compiler.executable,
            &header(),
            &format!("void rejected({c_qualifier}ContextRecord *p) {{ (void)({c}); }}"),
            &arguments,
        );
        let pointer = if is_const { "*const ContextRecord" } else { "*mut ContextRecord" };
        let error = rust_oracle::reject_rust(&format!(
            "{rust}\nfn main() {{ let pointer: {pointer} = core::ptr::null_mut(); unsafe {{ let _ = {invocation}; }} }}\n"
        ));
        assert!(error.contains("error"), "{label}: the generated context must reject");
    }
    for invocation in ["CTX_FIELD!(pointer)", "CTX_SET!(CTX_FIELD!(pointer), 3_i32)"] {
        let error = rust_oracle::reject_rust(&format!(
            "{rust}\nfn main() {{ let pointer: *mut ContextRecord = core::ptr::null_mut(); let _ = {invocation}; }}\n"
        ));
        assert!(
            error.contains("unsafe"),
            "field access must expose its caller obligation: {error}"
        );
    }
    let atomic = rust_oracle::reject_rust(&format!(
        "{rust}\nfn main() {{ let _ = CTX_ATOMIC!(1_i32 + 2_i32); }}\n"
    ));
    assert!(
        atomic.contains("invocation contract"),
        "ungrouped C holes require atomic Rust operands"
    );
}

/// Extract a named LLVM function's body for control-flow and branch-weight inspection.
fn function_body<'a>(llvm: &'a str, name: &str) -> &'a str {
    let definition = llvm
        .lines()
        .find(|line| line.starts_with("define ") && line.contains(&format!("@{name}(")))
        .unwrap_or_else(|| panic!("LLVM did not preserve the exported witness {name}"));
    let start = llvm.find(definition).unwrap();
    let tail = &llvm[start..];
    &tail[..tail.find("\n}").expect("LLVM function must terminate") + 2]
}

/// Own isolated compiler inputs and outputs so oracle runs cannot reuse stale artifacts or
/// leave a growing target tree.
struct TemporaryDirectory(
    /// Owned fixture path used for isolated inputs and cleanup.
    PathBuf,
);
/// Allocate isolated compiler artifacts with process-local uniqueness and deterministic cleanup
/// ownership.
impl TemporaryDirectory {
    /// Create owned, uniquely named fixture storage so this test's headers and compiler outputs
    /// cannot collide with another invocation.
    fn new() -> Self {
        let nonce = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let path = std::env::temp_dir()
            .join(format!("pgrx-context-oracle-{}-{nonce}", std::process::id()));
        fs::create_dir(&path).expect("create context oracle scratch directory");
        Self(path)
    }
}
/// Release only temporary artifacts owned by this fixture, including on failed compiler or
/// assertion paths.
impl Drop for TemporaryDirectory {
    /// Remove only this fixture's owned temporary storage after the test or oracle completes.
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
