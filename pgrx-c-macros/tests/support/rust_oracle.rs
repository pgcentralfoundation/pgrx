//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

// Individual integration-test crates use different parts of this shared harness.

use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

/// Maximum compiler or consumer runtime before the oracle kills a stalled process.
const TIMEOUT: Duration = Duration::from_secs(30);
/// Maximum captured output size, keeping failed or malformed compiler probes from exhausting
/// test resources.
const OUTPUT_LIMIT: u64 = 8 * 1024 * 1024;
/// Allocate process-local unique directory suffixes for concurrent isolated oracle runs.
static NEXT_DIRECTORY: AtomicU64 = AtomicU64::new(0);

/// Compile actual emitted Rust and semantic support in an isolated standalone program.
#[allow(dead_code)]
pub fn run_rust(source: &str) -> String {
    let directory = TemporaryDirectory::new();
    let program = directory.0.join("oracle.rs");
    let executable = directory.0.join("oracle");
    fs::write(&program, source).expect("write Rust oracle source");
    let mut compiler = Command::new("rustc");
    compiler
        .args(["--edition=2024", "-C", "overflow-checks=yes"])
        .arg(&program)
        .arg("-o")
        .arg(&executable);
    run_tool(&mut compiler, "compile");
    run_tool(&mut Command::new(executable), "execute")
}

/// Link emitted Rust with the original C functions under the inspected compilation profile.
#[allow(dead_code)]
pub fn run_rust_linked(
    source: &str,
    c_compiler: &Path,
    header: &Path,
    c_source: &str,
    arguments: &[&str],
) -> String {
    let directory = TemporaryDirectory::new();
    let native = directory.0.join("native.c");
    let object = directory.0.join("native.o");
    fs::write(&native, c_source).expect("write original C function definitions");
    let mut compiler = Command::new(c_compiler);
    compiler
        .args(["-x", "c"])
        .args(arguments)
        .arg("-include")
        .arg(header)
        .arg("-c")
        .arg(&native)
        .arg("-o")
        .arg(&object);
    run_bounded(&mut compiler, &directory.0, "compile_native");
    let rust = directory.0.join("linked.rs");
    let executable = directory.0.join("linked");
    fs::write(&rust, source).expect("write emitted Rust call oracle");
    let mut compiler = Command::new("rustc");
    compiler
        .args(["--edition=2024", "-O", "-C", "overflow-checks=yes", "-C"])
        .arg(format!("link-arg={}", object.display()))
        .arg(&rust)
        .arg("-o")
        .arg(&executable);
    // Installed PostgreSQL headers can emit unused routines whose backend
    // symbols have no standalone definition. Keep only reachable C sections.
    #[cfg(target_os = "macos")]
    compiler.args(["-C", "link-arg=-Wl,-dead_strip"]);
    #[cfg(target_os = "linux")]
    compiler.args(["-C", "link-arg=-Wl,--gc-sections"]);
    run_bounded(&mut compiler, &directory.0, "compile_linked");
    run_bounded(&mut Command::new(executable), &directory.0, "execute_linked")
}

/// Require downstream type checking to reject an invalid invocation before any linking.
#[allow(dead_code)]
pub fn reject_rust(source: &str) -> String {
    let directory = TemporaryDirectory::new();
    let rust = directory.0.join("rejected.rs");
    fs::write(&rust, source).expect("write a rejected Rust invocation");
    let mut compiler = Command::new("rustc");
    compiler
        .args(["--edition=2024", "--emit=metadata"])
        .arg(&rust)
        .arg("-o")
        .arg(directory.0.join("rejected.rmeta"));
    let (status, stdout, stderr) = run_process(&mut compiler, &directory.0, "reject");
    assert!(!status.success(), "invalid macro invocation compiled: {stdout}");
    stderr
}

/// Require the original-header C invocation to fail type checking, establishing the negative
/// half of a paired rejection.
#[allow(dead_code)]
pub fn reject_c_invocation(
    compiler: &Path,
    header: &Path,
    source: &str,
    arguments: &[&str],
) -> String {
    let directory = TemporaryDirectory::new();
    let c = directory.0.join("rejected.c");
    // Strict compilers must reject the invocation's constraints rather than
    // diagnose a missing final newline in the generated translation unit.
    fs::write(&c, format!("{source}\n")).expect("write a rejected original C invocation");
    let mut compiler = Command::new(compiler);
    compiler
        .args(["-x", "c"])
        .args(arguments)
        .args(["-fsyntax-only", "-Werror"])
        .arg("-include")
        .arg(header)
        .arg(&c);
    let (status, stdout, stderr) = run_process(&mut compiler, &directory.0, "reject_c");
    assert!(!status.success(), "invalid original C invocation compiled: {stdout}");
    stderr
}

/// Resolve a read-only tool input using the same process and output bounds as the oracle.
pub fn run_tool(command: &mut Command, phase: &str) -> String {
    let directory = TemporaryDirectory::new();
    run_bounded(command, &directory.0, phase)
}

/// Require a tool to reject an input under the oracle's process and output bounds.
#[allow(dead_code)]
pub fn reject_tool(command: &mut Command, phase: &str) -> String {
    let directory = TemporaryDirectory::new();
    let (status, stdout, stderr) = run_process(command, &directory.0, phase);
    assert!(
        !status.success(),
        "Rust oracle {phase} unexpectedly succeeded ({status}):\n{stderr}\n{stdout}"
    );
    stderr
}

/// Run a compiler or consumer with output and time limits, rejecting failures instead of
/// comparing partial observations.
fn run_bounded(command: &mut Command, directory: &Path, phase: &str) -> String {
    let (status, stdout, stderr) = run_process(command, directory, phase);
    assert!(status.success(), "Rust oracle {phase} failed ({status}):\n{stderr}\n{stdout}");
    stdout
}

/// Capture exit status and bounded output in owned files, killing a stalled oracle before it
/// can exhaust the test run.
fn run_process(
    command: &mut Command,
    directory: &Path,
    phase: &str,
) -> (std::process::ExitStatus, String, String) {
    let stdout_path = directory.join(format!("{phase}.stdout"));
    let stderr_path = directory.join(format!("{phase}.stderr"));
    let mut child = command
        .stdin(Stdio::null())
        .stdout(File::create(&stdout_path).expect("create Rust oracle stdout"))
        .stderr(File::create(&stderr_path).expect("create Rust oracle stderr"))
        .spawn()
        .unwrap_or_else(|error| panic!("could not {phase} the Rust oracle: {error}"));
    let deadline = Instant::now() + TIMEOUT;
    let status = loop {
        let oversized = [&stdout_path, &stderr_path]
            .iter()
            .any(|path| fs::metadata(path).is_ok_and(|metadata| metadata.len() > OUTPUT_LIMIT));
        if oversized || Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            panic!("Rust oracle {phase} exceeded its time or output limit");
        }
        match child.try_wait().expect("wait for Rust oracle") {
            Some(status) => break status,
            None => std::thread::sleep(Duration::from_millis(10)),
        }
    };
    for path in [&stdout_path, &stderr_path] {
        assert!(
            fs::metadata(path).expect("inspect Rust oracle output").len() <= OUTPUT_LIMIT,
            "Rust oracle {phase} exceeded its output limit"
        );
    }
    let stdout = fs::read_to_string(stdout_path).expect("read Rust oracle stdout");
    let stderr = fs::read_to_string(stderr_path).expect("read Rust oracle stderr");
    (status, stdout, stderr)
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
        let number = NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir()
            .join(format!("pgrx-rust-oracle-{}-{nonce}-{number}", std::process::id()));
        fs::create_dir(&path).expect("create an isolated Rust oracle directory");
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
