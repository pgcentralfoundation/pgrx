//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const TIMEOUT: Duration = Duration::from_secs(30);
const OUTPUT_LIMIT: u64 = 8 * 1024 * 1024;
static NEXT_DIRECTORY: AtomicU64 = AtomicU64::new(0);

/// Compile actual emitted Rust and semantic support in an isolated standalone program.
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

/// Resolve a read-only tool input using the same process and output bounds as the oracle.
pub fn run_tool(command: &mut Command, phase: &str) -> String {
    let directory = TemporaryDirectory::new();
    run_bounded(command, &directory.0, phase)
}

fn run_bounded(command: &mut Command, directory: &Path, phase: &str) -> String {
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
    assert!(status.success(), "Rust oracle {phase} failed ({status}):\n{stderr}\n{stdout}");
    stdout
}

struct TemporaryDirectory(PathBuf);

impl TemporaryDirectory {
    fn new() -> Self {
        let nonce = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let number = NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir()
            .join(format!("pgrx-rust-oracle-{}-{nonce}-{number}", std::process::id()));
        fs::create_dir(&path).expect("create an isolated Rust oracle directory");
        Self(path)
    }
}

impl Drop for TemporaryDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
