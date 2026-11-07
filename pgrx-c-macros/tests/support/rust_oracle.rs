//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

// Compile generated Rust consumers independently of the test crate and compare their output
// with original C observations. Profile-aware callers supply the cfg operands produced by
// the validated production helper; this module also serves library unit tests without
// depending on an external crate name.
//
// Every compiler and consumer has bounded captured output and works in owned temporary
// directories; elapsed time is not limited. Cross-target checks stop at metadata and explicitly
// report missing Rust target libraries, while native comparisons execute the actual emitted
// macros and support.

use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

/// Maximum captured output size, keeping failed or malformed compiler probes from exhausting
/// test resources.
const OUTPUT_LIMIT: u64 = 8 * 1024 * 1024;
/// Allocate process-local unique directory suffixes for concurrent isolated oracle runs.
static NEXT_DIRECTORY: AtomicU64 = AtomicU64::new(0);

/// Compile actual emitted Rust and semantic support in an isolated standalone program.
#[allow(dead_code)]
pub fn run_rust(source: &str) -> String {
    run_rust_with_arguments(source, &[])
}

/// Execute emitted Rust with `support_rust_cfg`'s validated char, size_t, and ptrdiff_t identities
/// rather than assuming the Rust target defaults match PostgreSQL's compiler profile.
#[allow(dead_code)]
pub fn run_rust_with_cfg(source: &str, configurations: &[String]) -> String {
    run_rust_with_arguments(
        source,
        &configurations.iter().flat_map(|value| ["--cfg", value.as_str()]).collect::<Vec<_>>(),
    )
}

/// Compile and run a native consumer with only the explicitly selected runtime profile flags.
fn run_rust_with_arguments(source: &str, arguments: &[&str]) -> String {
    let directory = TemporaryDirectory::new();
    let program = directory.0.join("oracle.rs");
    let executable = directory.0.join(format!("oracle{}", std::env::consts::EXE_SUFFIX));
    fs::write(&program, source).expect("write Rust oracle source");
    let mut compiler = Command::new("rustc");
    compiler
        .args(["--edition=2024", "-C", "overflow-checks=yes"])
        .args(arguments)
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
    run_rust_linked_with_arguments(source, c_compiler, header, c_source, arguments, &[])
}

/// Link emitted Rust with its original C support using the production helper's validated
/// runtime identities together with the original compilation arguments.
#[allow(dead_code)]
pub fn run_rust_linked_with_cfg(
    source: &str,
    c_compiler: &Path,
    header: &Path,
    c_source: &str,
    arguments: &[&str],
    configurations: &[String],
) -> String {
    run_rust_linked_with_arguments(
        source,
        c_compiler,
        header,
        c_source,
        arguments,
        &configurations.iter().flat_map(|value| ["--cfg", value.as_str()]).collect::<Vec<_>>(),
    )
}

/// Apply native C arguments and Rust runtime configuration within one bounded oracle run.
/// The native startup also disables Windows CRT stdout translation, preserving exact mixed
/// C/Rust output without rewriting intentional CRLFs or changing fixture source locations.
fn run_rust_linked_with_arguments(
    source: &str,
    c_compiler: &Path,
    header: &Path,
    c_source: &str,
    arguments: &[&str],
    rust_arguments: &[&str],
) -> String {
    let directory = TemporaryDirectory::new();
    let native = directory.0.join("native.c");
    let object = directory.0.join("native.o");
    fs::write(&native, format!("{c_source}\n{}", include_str!("binary_stdout.h")))
        .expect("write original C function definitions");
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
    run_tool(&mut compiler, "compile_native");
    let rust = directory.0.join("linked.rs");
    let executable = directory.0.join(format!("linked{}", std::env::consts::EXE_SUFFIX));
    fs::write(&rust, source).expect("write emitted Rust call oracle");
    let mut compiler = Command::new("rustc");
    compiler
        .args(["--edition=2024", "-O", "-C", "overflow-checks=yes", "-C"])
        .arg(format!("link-arg={}", object.display()))
        .args(rust_arguments)
        .arg(&rust)
        .arg("-o")
        .arg(&executable);
    // Installed PostgreSQL headers can emit unused routines whose backend
    // symbols have no standalone definition. Keep only reachable C sections.
    #[cfg(target_os = "macos")]
    compiler.args(["-C", "link-arg=-Wl,-dead_strip"]);
    #[cfg(any(target_os = "linux", all(target_os = "windows", target_env = "gnu")))]
    compiler.args(["-C", "link-arg=-Wl,--gc-sections"]);
    #[cfg(all(target_os = "windows", target_env = "msvc"))]
    compiler.args(["-C", "link-arg=/OPT:REF"]);
    run_tool(&mut compiler, "compile_linked");
    run_tool(&mut Command::new(executable), "execute_linked")
}

/// Type-check emitted Rust for an actual cross-target profile when that Rust target's standard
/// library is installed, without requiring its linker or running foreign machine code.
#[allow(dead_code)]
pub fn check_rust_with_cfg(source: &str, configurations: &[String], rust_target: &str) -> bool {
    let library = run_tool(
        Command::new("rustc").args(["--print", "target-libdir", "--target", rust_target]),
        "locate cross-target Rust standard library",
    );
    if !Path::new(library.trim()).is_dir() {
        eprintln!(
            "cross-target Rust type checking omitted for {rust_target}: its standard library is not installed; original C facts and macro emission remain required"
        );
        return false;
    }
    let directory = TemporaryDirectory::new();
    let program = directory.0.join("cross.rs");
    fs::write(&program, source).expect("write cross-target Rust oracle source");
    let mut compiler = Command::new("rustc");
    compiler
        .args(["--edition=2024", "--crate-type=lib", "--emit=metadata", "--target", rust_target])
        .args(configurations.iter().flat_map(|value| ["--cfg", value.as_str()]))
        .arg(&program)
        .arg("-o")
        .arg(directory.0.join("cross.rmeta"));
    run_tool(&mut compiler, "check_cross_target");
    true
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
    let (status, stdout, stderr) = run_process(&mut compiler, "reject");
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
    let (status, stdout, stderr) = run_process(&mut compiler, "reject_c");
    assert!(!status.success(), "invalid original C invocation compiled: {stdout}");
    stderr
}

/// Require a tool to reject an input under the oracle's output bound.
#[allow(dead_code)]
pub fn reject_tool(command: &mut Command, phase: &str) -> String {
    let (status, stdout, stderr) = run_process(command, phase);
    assert!(
        !status.success(),
        "Rust oracle {phase} unexpectedly succeeded ({status}):\n{stderr}\n{stdout}"
    );
    stderr
}

/// Run a compiler, consumer or read-only tool with bounded output, rejecting failures instead of
/// comparing partial observations.
pub fn run_tool(command: &mut Command, phase: &str) -> String {
    let (status, stdout, stderr) = run_process(command, phase);
    assert!(status.success(), "Rust oracle {phase} failed ({status}):\n{stderr}\n{stdout}");
    stdout
}

/// Capture exit status and bounded output.
///
/// No time limit applies, so a slow machine cannot fail a correct run. A stream that passes the
/// output limit is closed rather than its process killed: the next write fails, and the tool
/// exits through its own error handling, which waits for any linker it started. No process is
/// left writing into a directory that the caller removes.
fn run_process(command: &mut Command, phase: &str) -> (std::process::ExitStatus, String, String) {
    let mut child = command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|error| panic!("could not {phase} the Rust oracle: {error}"));
    let stdout = child.stdout.take().expect("piped Rust oracle stdout");
    let stderr = child.stderr.take().expect("piped Rust oracle stderr");
    let (stdout, stderr) = std::thread::scope(|scope| {
        let stderr = scope.spawn(move || capture(stderr));
        (capture(stdout), stderr.join().expect("read Rust oracle stderr"))
    });
    let status = child.wait().expect("wait for Rust oracle");
    let (Some(stdout), Some(stderr)) = (stdout, stderr) else {
        panic!("Rust oracle {phase} exceeded its output limit");
    };
    let stdout = String::from_utf8(stdout).expect("read Rust oracle stdout");
    let stderr = String::from_utf8(stderr).expect("read Rust oracle stderr");
    (status, stdout, stderr)
}

/// Read a stream to its end, or `None` once it passes [`OUTPUT_LIMIT`]. Returning drops the
/// pipe, so a writer past the limit fails its next write instead of filling memory or disk.
fn capture(stream: impl Read) -> Option<Vec<u8>> {
    let mut bytes = Vec::new();
    stream.take(OUTPUT_LIMIT + 1).read_to_end(&mut bytes).expect("read Rust oracle output");
    (bytes.len() as u64 <= OUTPUT_LIMIT).then_some(bytes)
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
