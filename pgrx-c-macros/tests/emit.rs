//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

#[path = "support/oracle.rs"]
mod oracle;

use pgrx_c_macros::{
    AnalysisSession, AnalysisStatus, BindingCatalog, ConstCapability, EmissionStatus,
    InvocationContract, MacroEmission, MacroScanner, ParameterOrigin, SkipReasonCode, emit,
    emit_support_with_bindings, inspect,
};
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

static SCANNER_LOCK: Mutex<()> = Mutex::new(());
static NEXT_DIRECTORY: AtomicU64 = AtomicU64::new(0);
const PROCESS_TIMEOUT: Duration = Duration::from_secs(30);
const OUTPUT_LIMIT: u64 = 1024 * 1024;

const EMITTED: &[&str] = &[
    "EMIT_ID",
    "EMIT_ADD",
    "EMIT_NEGATE",
    "EMIT_PLUS",
    "EMIT_NOT",
    "EMIT_REPEAT",
    "EMIT_UNUSED",
    "EMIT_WORD",
    "EMIT_BOOL",
    "EMIT_COMPARE",
    "EMIT_CONSTANT",
    "EMIT_LONG",
    "EMIT_UNSIGNED_LONG_LONG",
    "EMIT_KEYWORDS",
    "EMIT_DIVIDE",
    "EMIT_REMAINDER",
    "EMIT_SHIFT",
    "EMIT_SHIFT_RIGHT",
    "EMIT_BITS",
    "EMIT_LOGICAL_AND",
    "EMIT_LOGICAL_OR",
    "EMIT_CHOOSE",
    "match",
];

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures").join(name)
}

fn source(emission: &MacroEmission) -> &str {
    let EmissionStatus::Emitted { rust, const_capability } = &emission.status else {
        panic!("{} must emit: {emission:?}", emission.analysis.name);
    };
    assert_eq!(*const_capability, ConstCapability::RuntimeOnly);
    assert!(matches!(emission.analysis.status, AnalysisStatus::Candidate));
    assert!(emission.analysis.provenance.is_some());
    rust
}

fn public_body(source: &str) -> &str {
    source
        .split_once("(@__pgrx_emit_public;")
        .unwrap()
        .1
        .split_once("=> {")
        .unwrap()
        .1
        .split_once("\n(@__pgrx_emit_value;")
        .unwrap()
        .0
}

#[test]
fn emitted_macros_keep_provenance_occurrences_names_and_structured_skips() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let frontend = inspect(&scanner, &fixture("emit_scalar.h"), &["-fwrapv".into()], None).unwrap();
    let mut names = EMITTED.to_vec();
    names.extend(["EMIT_UNGROUPED", "EMIT_UNKNOWN", "EMIT_POINTER", "EMIT_ABSENT"]);
    let session = AnalysisSession::prepare(&scanner, &frontend, &names).unwrap();
    for name in EMITTED {
        let emission = emit(&session, name);
        let rust = source(&emission);
        assert!(rust.contains("$crate::__pgrx_c_macros::"), "{name}: {rust}");
        assert!(rust.contains("emit_scalar.h:"), "source line must survive lowering");
        assert!(rust.contains("```text\\n#define "), "original C definition is plain text");
        assert!(rust.contains("compile_error!"), "standalone emissions need their ABI guard");
        assert!(!rust.contains("/Users/"), "emitted documentation must not contain private paths");
    }
    let repeated = emit(&session, "EMIT_REPEAT");
    assert_eq!(public_body(source(&repeated)).matches("$value").count(), 2);
    let unused = emit(&session, "EMIT_UNUSED");
    assert_eq!(public_body(source(&unused)).matches("$value").count(), 0);
    assert!(source(&emit(&session, "match")).contains("macro_rules! r#match"));
    let ungrouped = emit(&session, "EMIT_UNGROUPED");
    assert_eq!(ungrouped.analysis.invocation, InvocationContract::ExplicitExpressionBoundary);
    assert!(source(&ungrouped).contains("use @__pgrx_c_expression; before its arguments"));
    assert!(source(&ungrouped).contains("(@__pgrx_c_expression; $($raw:tt)*)"));
    let absent = emit(&session, "EMIT_ABSENT");
    let EmissionStatus::Skipped { reason } = &absent.status else {
        panic!("inactive EMIT_ABSENT must remain skipped: {absent:?}");
    };
    assert_eq!(reason.code, SkipReasonCode::NotActive, "{}", reason.message);
    assert!(!reason.message.is_empty());
    if let Some(span) = &absent.analysis.provenance {
        assert!(reason.spans.contains(span));
    }
    let unknown = emit(&session, "EMIT_UNKNOWN");
    assert!(source(&unknown).contains("explicit caller-scope operands"));
    assert_eq!(unknown.analysis.parameters.last().unwrap().origin, ParameterOrigin::FreeIdentifier);
    let pointer = emit(&session, "EMIT_POINTER");
    assert!(matches!(pointer.analysis.status, AnalysisStatus::Candidate));
    assert!(public_body(source(&pointer)).contains("::dereference("));
    assert!(source(&pointer).contains("::pointee("));
}

#[test]
fn documented_definition_keeps_unexpanded_calls_parameters_and_fenced_comments() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().unwrap();
    let frontend = inspect(&scanner, &fixture("emit_scalar.h"), &[], None).unwrap();
    let session = AnalysisSession::prepare(&scanner, &frontend, &["EMIT_DOCUMENTED"]).unwrap();
    let emission = emit(&session, "EMIT_DOCUMENTED");
    let rust = source(&emission);
    assert!(
        rust.contains("\\n\\n``````text\\n#define EMIT_DOCUMENTED( value ) EMIT_ADD ( value ,")
    );
    assert!(rust.contains("/* ``` \\\"quoted\\\" \\\\\\\\ backslash\\n`````\\n"));
    assert!(rust.contains("*/ 1 )\\n``````\\n\""));
    assert!(!rust.contains("#define EMIT_DOCUMENTED( __pgrx_c_"));

    // Check the generated Rust doc attribute and ensure the C fence creates no
    // Rust doctest, even when its original comment contains shorter fences.
    let directory = TemporaryDirectory::new();
    let runtime = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../pgrx-pg-sys/src/c_macros/support.rs")
        .canonicalize()
        .unwrap();
    let library = directory.0.join("documented.rs");
    fs::write(&library, format!("#[path = {runtime:?}]\npub mod __pgrx_c_macros;\n{rust}"))
        .unwrap();
    let mut rustdoc = Command::new("rustdoc");
    rustdoc.args(["--edition=2024", "--test"]).arg(&library);
    let (status, stdout, stderr) = run_bounded(&mut rustdoc, &directory.0, "documented");
    assert!(status.success(), "rustdoc failed ({status}):\n{stderr}\n{stdout}");
    assert!(stdout.contains("running 0 tests"), "C documentation must not create Rust doctests");
}

#[test]
fn actual_support_and_emitted_macros_compile_in_a_renamed_downstream_crate() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let header = fixture("emit_scalar.h");
    let frontend = inspect(&scanner, &header, &["-fwrapv".into()], None).unwrap();
    let session = AnalysisSession::prepare(&scanner, &frontend, EMITTED).unwrap();
    let directory = TemporaryDirectory::new();
    let runtime = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../pgrx-pg-sys/src/c_macros/support.rs")
        .canonicalize()
        .unwrap();
    let mut generated =
        format!("#[path = {:?}]\npub mod __pgrx_c_macros;\n", runtime.to_str().unwrap());
    generated.push_str(
        &emit_support_with_bindings(&session, EMITTED, &BindingCatalog::default()).unwrap(),
    );
    for name in EMITTED {
        generated.push_str(source(&emit(&session, name)));
    }
    let library = directory.0.join("generated.rs");
    let rlib = directory.0.join("libgenerated_c_semantics.rlib");
    fs::write(&library, generated).unwrap();
    let mut compile_library = Command::new("rustc");
    compile_library
        .args(["--edition=2024", "--crate-type=rlib", "--crate-name=generated_c_semantics"])
        .arg(&library)
        .arg("-o")
        .arg(&rlib);
    run_success(&mut compile_library, &directory.0, "library");

    let consumer = directory.0.join("consumer.rs");
    let executable = directory.0.join("consumer");
    fs::write(&consumer, include_str!("fixtures/emit_consumer.rs")).unwrap();
    let mut compile_consumer = downstream_command(&consumer, &rlib, &executable);
    run_success(&mut compile_consumer, &directory.0, "consumer");
    run_success(&mut Command::new(&executable), &directory.0, "execute");

    // C validates facts about the original header, independently of the Rust tree.
    // No library headers or native link step are needed under the exact profile.
    oracle::run_c(
        &frontend.profile().compiler.executable,
        &header,
        r#"
#define TYPE_IS(value, type) _Static_assert(_Generic((value), type: 1, default: 0), #value)
TYPE_IS(EMIT_ID((_Bool) 1), _Bool);
TYPE_IS(EMIT_ID((unsigned char) 255), unsigned char);
TYPE_IS(EMIT_ADD((unsigned char) 255, (unsigned char) 1), int);
TYPE_IS(EMIT_ADD(-1, 1U), unsigned int);
TYPE_IS(EMIT_ADD(0UL, 0LL), unsigned long long);
TYPE_IS(EMIT_LONG(unknown_unused_binding), long);
TYPE_IS(EMIT_UNSIGNED_LONG_LONG(unknown_unused_binding), unsigned long long);
TYPE_IS(EMIT_WORD(-1), unsigned short);
TYPE_IS(EMIT_BOOL(256), _Bool);
TYPE_IS(EMIT_COMPARE(-1, 1U), int);
TYPE_IS(EMIT_CHOOSE(1, -1, 1U), unsigned int);
TYPE_IS(EMIT_CHOOSE(0, (unsigned char) 2, (unsigned char) 3), int);
_Static_assert(EMIT_ADD(-1, 1U) == 0U, "common unsigned conversion");
_Static_assert(EMIT_WORD(-1) == 65535, "explicit unsigned narrowing");
_Static_assert(EMIT_BOOL(256) == 1, "truth before narrowing");
_Static_assert(EMIT_COMPARE(-1, 1U) == 0, "comparison after common conversion");
_Static_assert(EMIT_DIVIDE(-7, 3) == -2, "C division truncates toward zero");
_Static_assert(EMIT_REMAINDER(-7, 3) == -1, "C remainder sign");
_Static_assert(EMIT_CHOOSE(1, -1, 1U) == 4294967295U, "unchosen arm determines common type");
_Static_assert(_Generic(EMIT_ADD(1.0, 2.0f), double: 1, default: 0), "float addition keeps C common identity");
_Static_assert(EMIT_ADD(1.0, 2.0f) == 3.0, "noncontracting float addition");
"#,
        &frontend.profile().arguments.iter().map(String::as_str).collect::<Vec<_>>(),
        false,
    );

    for (label, input) in [
        ("custom", "Custom"),
        ("ambiguous_signed", "1_i64"),
        ("ambiguous_unsigned", "1_u64"),
        ("pointer_sized_signed", "1_isize"),
        ("pointer_sized_unsigned", "1_usize"),
    ] {
        let rejected = directory.0.join(format!("reject_{label}.rs"));
        fs::write(
            &rejected,
            format!(
                "struct Custom;\nfn main() {{ let _ = renamed_generated::EMIT_ID!({input}); }}\n"
            ),
        )
        .unwrap();
        let mut compiler = downstream_command(&rejected, &rlib, &directory.0.join(label));
        let (status, _, stderr) = run_bounded(&mut compiler, &directory.0, label);
        assert!(!status.success(), "unsupported {input} must fail compilation");
        assert!(
            stderr.contains("IntoExpression") || stderr.contains("AllowedProfile"),
            "{input} must fail its sealed scalar constraint: {stderr}"
        );
    }
    let non_const = directory.0.join("reject_const.rs");
    fs::write(
        &non_const,
        "const VALUE: i32 = renamed_generated::EMIT_ADD!(1_i32, 2_i32).get();\nfn main() { let _ = VALUE; }\n",
    )
    .unwrap();
    let mut compiler = downstream_command(&non_const, &rlib, &directory.0.join("const"));
    let (status, _, stderr) = run_bounded(&mut compiler, &directory.0, "const");
    assert!(!status.success());
    assert!(
        stderr.contains("non-const"),
        "runtime capability must agree with compiler rejection: {stderr}"
    );
}

#[test]
fn atomic_argument_rules_reject_unparenthesized_multi_token_substitutions() {
    let _lock = SCANNER_LOCK.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let scanner = MacroScanner::new().expect("libclang must be available");
    let frontend = inspect(&scanner, &fixture("expression_oracle.h"), &[], None).unwrap();
    let session = AnalysisSession::prepare(&scanner, &frontend, &["EXPR_ATOMIC_POW2"]).unwrap();
    let emission = emit(&session, "EXPR_ATOMIC_POW2");
    assert!(source(&emission).contains("$value:tt"));

    let directory = TemporaryDirectory::new();
    let runtime = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../pgrx-pg-sys/src/c_macros/support.rs")
        .canonicalize()
        .unwrap();
    let library = directory.0.join("atomic.rs");
    let rlib = directory.0.join("libatomic_c_semantics.rlib");
    fs::write(
        &library,
        format!(
            "#[path = {runtime:?}]\npub mod __pgrx_c_macros;\n{}\n{}",
            emit_support_with_bindings(&session, &["EXPR_ATOMIC_POW2"], &BindingCatalog::default())
                .unwrap(),
            source(&emission)
        ),
    )
    .unwrap();
    let mut compile_library = Command::new("rustc");
    compile_library
        .args(["--edition=2024", "--crate-type=rlib", "--crate-name=atomic_c_semantics"])
        .arg(&library)
        .arg("-o")
        .arg(&rlib);
    run_success(&mut compile_library, &directory.0, "atomic_library");

    let accepted = directory.0.join("accepted.rs");
    fs::write(
        &accepted,
        r#"
fn main() {
    let atomic_value = 16_i32;
    let _ = renamed_generated::EXPR_ATOMIC_POW2!(atomic_value);
    let _ = renamed_generated::EXPR_ATOMIC_POW2!(15_i32);
    let _ = renamed_generated::EXPR_ATOMIC_POW2!((1_i32 << 1_i32));
    let _ = renamed_generated::EXPR_ATOMIC_POW2!((-1_i32));
}
"#,
    )
    .unwrap();
    let executable = directory.0.join("accepted");
    run_success(
        &mut downstream_command(&accepted, &rlib, &executable),
        &directory.0,
        "atomic_accepted",
    );
    run_success(&mut Command::new(&executable), &directory.0, "atomic_execute");

    for (label, argument) in [
        ("operator", "1_i32 << 1_i32"),
        ("negative", "-1_i32"),
        ("call", "std::hint::black_box(4_i32)"),
    ] {
        let rejected = directory.0.join(format!("{label}.rs"));
        fs::write(
            &rejected,
            format!("fn main() {{ let _ = renamed_generated::EXPR_ATOMIC_POW2!({argument}); }}\n"),
        )
        .unwrap();
        let mut compiler = downstream_command(&rejected, &rlib, &directory.0.join(label));
        let (status, _, stderr) = run_bounded(&mut compiler, &directory.0, label);
        assert!(!status.success(), "ungrouped multi-token argument {argument} must fail");
        assert!(
            stderr.contains("invocation contract")
                || stderr.contains("requires a parenthesized negative literal"),
            "argument shape must fail matching before any semantic conversion: {stderr}"
        );
    }
}

fn downstream_command(source: &Path, library: &Path, executable: &Path) -> Command {
    let mut command = Command::new("rustc");
    command
        .arg("--edition=2024")
        .arg("--extern")
        .arg(format!("renamed_generated={}", library.display()))
        .arg(source)
        .arg("-o")
        .arg(executable);
    command
}

fn run_success(command: &mut Command, directory: &Path, phase: &str) {
    let (status, stdout, stderr) = run_bounded(command, directory, phase);
    assert!(status.success(), "{phase} failed ({status}):\n{stderr}\n{stdout}");
}

fn run_bounded(
    command: &mut Command,
    directory: &Path,
    phase: &str,
) -> (ExitStatus, String, String) {
    let stdout_path = directory.join(format!("{phase}.stdout"));
    let stderr_path = directory.join(format!("{phase}.stderr"));
    let mut child = command
        .stdin(Stdio::null())
        .stdout(File::create(&stdout_path).unwrap())
        .stderr(File::create(&stderr_path).unwrap())
        .spawn()
        .unwrap_or_else(|error| panic!("could not start {phase}: {error}"));
    let deadline = Instant::now() + PROCESS_TIMEOUT;
    let status = loop {
        let oversized = [&stdout_path, &stderr_path]
            .iter()
            .any(|path| fs::metadata(path).is_ok_and(|metadata| metadata.len() > OUTPUT_LIMIT));
        if oversized || Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            panic!("{phase} exceeded its time or output limit");
        }
        if let Some(status) = child.try_wait().expect("wait for test child") {
            break status;
        }
        std::thread::sleep(Duration::from_millis(10));
    };
    for path in [&stdout_path, &stderr_path] {
        assert!(
            fs::metadata(path).unwrap().len() <= OUTPUT_LIMIT,
            "{phase} output exceeded its bound"
        );
    }
    (status, fs::read_to_string(stdout_path).unwrap(), fs::read_to_string(stderr_path).unwrap())
}

struct TemporaryDirectory(PathBuf);

impl TemporaryDirectory {
    fn new() -> Self {
        let nonce = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let number = NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir()
            .join(format!("pgrx-c-emission-{}-{nonce}-{number}", std::process::id()));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
}

impl Drop for TemporaryDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
