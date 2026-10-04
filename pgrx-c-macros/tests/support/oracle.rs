//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

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

/// Supply the installed macOS SDK explicitly when a fixture needs native linking.
/// A standalone LLVM Clang may not discover it; inspection and execution must use
/// the same SDK. Other hosts need no additional arguments.
#[allow(dead_code)]
pub fn native_arguments() -> Vec<String> {
    #[cfg(target_os = "macos")]
    {
        let directory = TemporaryDirectory::new();
        let sdk =
            run_bounded(Command::new("xcrun").arg("--show-sdk-path"), &directory.0, "SDK lookup");
        let sdk = sdk.trim();
        assert!(!sdk.is_empty() && Path::new(sdk).is_dir(), "expected an installed macOS SDK");
        vec!["-isysroot".into(), sdk.into()]
    }
    #[cfg(not(target_os = "macos"))]
    Vec::new()
}

/// Each runner includes the original header; it never reconstructs its macro definitions.
/// Without execution, compiler-owned static assertions need no native linker or runtime.
pub fn run_c(
    compiler: &Path,
    header: &Path,
    source: &str,
    compiler_arguments: &[&str],
    execute: bool,
) -> String {
    let directory = TemporaryDirectory::new();
    let program = directory.0.join("oracle.c");
    let executable = directory.0.join("oracle");
    fs::write(&program, source).expect("write C oracle source");
    let mut compiler = Command::new(compiler);
    compiler.args(["-x", "c"]).args(compiler_arguments).arg("-include").arg(header).arg(&program);
    if execute {
        // Original headers can define unused backend routines with unresolved server
        // symbols. Remove their unreachable linker sections without changing C input.
        #[cfg(target_os = "macos")]
        compiler.arg("-Wl,-dead_strip");
        #[cfg(target_os = "linux")]
        compiler.arg("-Wl,--gc-sections");
        compiler.arg("-o").arg(&executable);
    } else {
        compiler.arg("-fsyntax-only");
    }
    let output = run_bounded(&mut compiler, &directory.0, "compile");
    if execute {
        run_bounded(&mut Command::new(executable), &directory.0, "execute")
    } else {
        output
    }
}

/// Run a compiler or consumer with output and time limits, rejecting failures instead of
/// comparing partial observations.
fn run_bounded(command: &mut Command, directory: &Path, phase: &str) -> String {
    let stdout_path = directory.join(format!("{phase}.stdout"));
    let stderr_path = directory.join(format!("{phase}.stderr"));
    let mut child = command
        .stdin(Stdio::null())
        .stdout(File::create(&stdout_path).expect("create oracle stdout"))
        .stderr(File::create(&stderr_path).expect("create oracle stderr"))
        .spawn()
        .unwrap_or_else(|error| panic!("could not {phase} the C oracle: {error}"));
    let deadline = Instant::now() + TIMEOUT;
    let status = loop {
        let oversized = [&stdout_path, &stderr_path]
            .iter()
            .any(|path| fs::metadata(path).is_ok_and(|metadata| metadata.len() > OUTPUT_LIMIT));
        if oversized || Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            panic!("C oracle {phase} exceeded its time or output limit");
        }
        match child.try_wait().expect("wait for C oracle") {
            Some(status) => break status,
            None => std::thread::sleep(Duration::from_millis(10)),
        }
    };
    for path in [&stdout_path, &stderr_path] {
        assert!(
            fs::metadata(path).expect("inspect oracle output").len() <= OUTPUT_LIMIT,
            "C oracle {phase} exceeded its output limit"
        );
    }
    let stdout = fs::read_to_string(stdout_path).expect("read oracle stdout");
    let stderr = fs::read_to_string(stderr_path).expect("read oracle stderr");
    assert!(status.success(), "C oracle {phase} failed ({status}):\n{stderr}\n{stdout}");
    stdout
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
