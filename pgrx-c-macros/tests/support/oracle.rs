//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

// Compile original C headers directly, preserving caller-supplied profile arguments as the
// semantic oracle. Native runs execute bounded programs; cross-target proofs use compiler
// static assertions and require neither a foreign linker nor a PostgreSQL backend.
//
// Captured output is limited, elapsed time is not, and owned temporary directories release
// compiler artifacts on success and failure. macOS SDK lookup is explicit so header inspection
// and execution can share the same effective native environment.

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

/// Supply the installed macOS SDK explicitly when a fixture needs native linking.
/// A standalone LLVM Clang may not discover it; inspection and execution must use
/// the same SDK. Other hosts need no additional arguments.
#[allow(dead_code)]
pub fn native_arguments() -> Vec<String> {
    #[cfg(target_os = "macos")]
    {
        let sdk = run_bounded(Command::new("xcrun").arg("--show-sdk-path"), "SDK lookup");
        let sdk = sdk.trim();
        assert!(!sdk.is_empty() && Path::new(sdk).is_dir(), "expected an installed macOS SDK");
        vec!["-isysroot".into(), sdk.into()]
    }
    #[cfg(not(target_os = "macos"))]
    Vec::new()
}

/// Each runner includes the original header; it never reconstructs its macro definitions.
/// Without execution, compiler-owned static assertions need no native linker or runtime.
/// Execution returns exact observations with Windows CRT newline translation disabled before
/// main, so literal carriage returns and all other bytes can be compared with Rust stdout.
pub fn run_c(
    compiler: &Path,
    header: &Path,
    source: &str,
    compiler_arguments: &[&str],
    execute: bool,
) -> String {
    let directory = TemporaryDirectory::new();
    let program = directory.0.join("oracle.c");
    let executable = directory.0.join(format!("oracle{}", std::env::consts::EXE_SUFFIX));
    let source = if execute {
        format!("{source}\n{}", include_str!("binary_stdout.h"))
    } else {
        source.to_owned()
    };
    fs::write(&program, source).expect("write C oracle source");
    let mut compiler = Command::new(compiler);
    compiler.args(["-x", "c"]).args(compiler_arguments).arg("-include").arg(header).arg(&program);
    if execute {
        // Original headers can define unused backend routines with unresolved server
        // symbols, such as pg_checksum_page calling ExceptionalCondition under assertions.
        // Remove their unreachable linker sections without changing C input. ld64 strips
        // per symbol; GNU ld drops whole sections, so each definition needs its own.
        #[cfg(target_os = "macos")]
        compiler.arg("-Wl,-dead_strip");
        #[cfg(any(target_os = "linux", all(target_os = "windows", target_env = "gnu")))]
        compiler.args(["-ffunction-sections", "-fdata-sections", "-Wl,--gc-sections"]);
        #[cfg(all(target_os = "windows", target_env = "msvc"))]
        compiler.arg("-Wl,/OPT:REF");
        compiler.arg("-o").arg(&executable);
    } else {
        compiler.arg("-fsyntax-only");
    }
    let output = run_bounded(&mut compiler, "compile");
    if execute { run_bounded(&mut Command::new(executable), "execute") } else { output }
}

/// Run a compiler or consumer with bounded output, rejecting failures instead of comparing
/// partial observations.
///
/// No time limit applies, so a slow machine cannot fail a correct run. A stream that passes the
/// output limit is closed rather than its process killed: the next write fails, and the tool
/// exits through its own error handling, which waits for any linker it started. No process is
/// left writing into a directory that the caller removes.
fn run_bounded(command: &mut Command, phase: &str) -> String {
    let mut child = command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|error| panic!("could not {phase} the C oracle: {error}"));
    let stdout = child.stdout.take().expect("piped C oracle stdout");
    let stderr = child.stderr.take().expect("piped C oracle stderr");
    let (stdout, stderr) = std::thread::scope(|scope| {
        let stderr = scope.spawn(move || capture(stderr));
        (capture(stdout), stderr.join().expect("read C oracle stderr"))
    });
    let status = child.wait().expect("wait for C oracle");
    let (Some(stdout), Some(stderr)) = (stdout, stderr) else {
        panic!("C oracle {phase} exceeded its output limit");
    };
    let stdout = String::from_utf8(stdout).expect("read oracle stdout");
    let stderr = String::from_utf8(stderr).expect("read oracle stderr");
    assert!(status.success(), "C oracle {phase} failed ({status}):\n{stderr}\n{stdout}");
    stdout
}

/// Read a stream to its end, or `None` once it passes [`OUTPUT_LIMIT`]. Returning drops the
/// pipe, so a writer past the limit fails its next write instead of filling memory or disk.
fn capture(stream: impl Read) -> Option<Vec<u8>> {
    let mut bytes = Vec::new();
    stream.take(OUTPUT_LIMIT + 1).read_to_end(&mut bytes).expect("read C oracle output");
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
            .join(format!("pgrx-c-oracle-{}-{nonce}-{number}", std::process::id()));
        fs::create_dir(&path).expect("create an isolated oracle directory");
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
