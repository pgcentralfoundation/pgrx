//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Build test binaries with one `cargo test --no-run`, then run several at once.
//!
//! `cargo test` runs one test binary at a time. Each pg_test binary starts its own PostgreSQL
//! server on its own port and data directory, so several binaries can run together.
//!
//! The pg_test harness reads Cargo's arguments from the `cargo test` process that started it.
//! Binaries started here have no such parent, so callers pass the features to install the
//! extension with through `PGRX_FEATURES` and `PGRX_NO_DEFAULT_FEATURES` instead.

use std::io::{BufReader, Write as _};
use std::process::{Command, Stdio};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use cargo_metadata::camino::Utf8PathBuf;
use cargo_metadata::{Message, Package};
use eyre::{WrapErr, bail, eyre};

use crate::process::{self, Exit, OUTPUT, log};
use crate::workspace::{self, Workspace};

/// A built test binary and what `cargo test` would run it with.
pub(crate) struct TestBinary {
    pub(crate) package: String,
    pub(crate) executable: Utf8PathBuf,
    label: String,
    dir: Utf8PathBuf,
    env: Vec<(&'static str, String)>,
}

impl TestBinary {
    fn new(package: &Package, target: &cargo_metadata::Target, executable: Utf8PathBuf) -> Self {
        let kind = target.kind.first().map(ToString::to_string).unwrap_or_default();
        Self {
            package: package.name.to_string(),
            label: format!("{} ({kind} {})", package.name, target.name),
            dir: workspace::package_dir(package).to_owned(),
            env: cargo_env(package, &target.name),
            executable,
        }
    }

    /// A command that runs the binary the way `cargo test` would.
    fn command(&self) -> Command {
        let mut command = Command::new(&self.executable);
        command.current_dir(&self.dir).envs(self.env.iter().map(|(key, value)| (key, value)));
        if let Some(cargo) = std::env::var_os("CARGO") {
            command.env("CARGO", cargo);
        } else {
            command.env("CARGO", "cargo");
        }
        command
    }
}

/// The package variables `cargo test` sets for a test binary.
fn cargo_env(package: &Package, target: &str) -> Vec<(&'static str, String)> {
    let version = &package.version;
    let optional = |value: &Option<String>| value.clone().unwrap_or_default();
    vec![
        ("CARGO_MANIFEST_DIR", workspace::package_dir(package).to_string()),
        ("CARGO_MANIFEST_PATH", package.manifest_path.to_string()),
        ("CARGO_PKG_NAME", package.name.to_string()),
        ("CARGO_PKG_VERSION", version.to_string()),
        ("CARGO_PKG_VERSION_MAJOR", version.major.to_string()),
        ("CARGO_PKG_VERSION_MINOR", version.minor.to_string()),
        ("CARGO_PKG_VERSION_PATCH", version.patch.to_string()),
        ("CARGO_PKG_VERSION_PRE", version.pre.to_string()),
        ("CARGO_PKG_AUTHORS", package.authors.join(":")),
        ("CARGO_PKG_DESCRIPTION", optional(&package.description)),
        ("CARGO_PKG_HOMEPAGE", optional(&package.homepage)),
        ("CARGO_PKG_REPOSITORY", optional(&package.repository)),
        ("CARGO_PKG_LICENSE", optional(&package.license)),
        (
            "CARGO_PKG_LICENSE_FILE",
            package.license_file.as_ref().map(ToString::to_string).unwrap_or_default(),
        ),
        (
            "CARGO_PKG_RUST_VERSION",
            package.rust_version.as_ref().map(ToString::to_string).unwrap_or_default(),
        ),
        ("CARGO_PKG_README", package.readme.as_ref().map(ToString::to_string).unwrap_or_default()),
        ("CARGO_CRATE_NAME", target.replace('-', "_")),
    ]
}

/// Build the test targets that `cargo test ARGS` would run and return their executables.
pub(crate) fn build(workspace: &Workspace, args: &[String]) -> eyre::Result<Vec<TestBinary>> {
    let started = Instant::now();
    let mut command = Command::new("cargo");
    command.args(["test", "--no-run", "--message-format=json-render-diagnostics"]).args(args);
    log(format_args!("+ {}", process::describe(&command)));
    let mut child =
        command.stdout(Stdio::piped()).spawn().wrap_err("starting `cargo test --no-run`")?;
    let stdout = child.stdout.take().expect("stdout was piped");
    let mut binaries = Vec::new();
    for message in Message::parse_stream(BufReader::new(stdout)) {
        let Message::CompilerArtifact(artifact) = message.wrap_err("reading Cargo's messages")?
        else {
            continue;
        };
        let Some(executable) = artifact.executable else { continue };
        if !artifact.profile.test {
            continue;
        }
        let package = workspace
            .by_id(&artifact.package_id)
            .ok_or_else(|| eyre!("Cargo built {} outside the workspace", artifact.package_id))?;
        binaries.push(TestBinary::new(package, &artifact.target, executable));
    }
    if !child.wait().wrap_err("waiting for `cargo test --no-run`")?.success() {
        bail!("`cargo test --no-run` failed");
    }
    log(format_args!("built {} test binaries in {:.0?}", binaries.len(), started.elapsed()));
    Ok(binaries)
}

/// The longest one test binary may run. The slowest takes about a minute, so a binary still
/// running after this has hung, and stopping it reports its output instead of holding the job
/// until the workflow's timeout.
const BINARY_LIMIT: Duration = Duration::from_secs(10 * 60);

/// How one test binary's run ended.
pub(crate) struct Outcome {
    label: String,
    passed: bool,
    elapsed: Duration,
}

/// Run `binaries`, `jobs` at a time, in order. `configure` adds what the job needs to each
/// binary's command. Each binary's output prints as one collapsed group when it finishes.
pub(crate) fn run_all(
    binaries: Vec<TestBinary>,
    jobs: usize,
    configure: impl Fn(&TestBinary, &mut Command) + Sync,
) -> Vec<Outcome> {
    log(format_args!("running {} test binaries, {jobs} at a time", binaries.len()));
    let started = Instant::now();
    let queue = Mutex::new(binaries.into_iter());
    let outcomes = Mutex::new(Vec::new());
    std::thread::scope(|scope| {
        for _ in 0..jobs.max(1) {
            scope.spawn(|| {
                loop {
                    // A panicking worker poisons nothing it shares except these plain lists.
                    let next =
                        queue.lock().unwrap_or_else(std::sync::PoisonError::into_inner).next();
                    let Some(binary) = next else { break };
                    let outcome = run_one(&binary, &configure);
                    outcomes
                        .lock()
                        .unwrap_or_else(std::sync::PoisonError::into_inner)
                        .push(outcome);
                }
            });
        }
    });
    let mut outcomes = outcomes.into_inner().unwrap_or_else(std::sync::PoisonError::into_inner);
    outcomes.sort_by_key(|outcome| std::cmp::Reverse(outcome.elapsed));
    log(format_args!(
        "ran {} test binaries in {:.0?}; the slowest:",
        outcomes.len(),
        started.elapsed()
    ));
    for outcome in outcomes.iter().take(8) {
        log(format_args!("  {:>6.1?}  {}", outcome.elapsed, outcome.label));
    }
    outcomes
}

fn run_one(binary: &TestBinary, configure: &impl Fn(&TestBinary, &mut Command)) -> Outcome {
    let started = Instant::now();
    let mut command = binary.command();
    configure(binary, &mut command);
    let (verdict, passed, output) = match process::run_captured(command, BINARY_LIMIT) {
        Ok((Exit::Status(status), output)) if status.success() => ("ok", true, output),
        Ok((Exit::Status(_), output)) => ("FAILED", false, output),
        Ok((Exit::TimedOut, output)) => ("TIMED OUT", false, output),
        Err(error) => ("FAILED", false, format!("{error:?}")),
    };
    let elapsed = started.elapsed();
    let _guard = OUTPUT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    let mut stdout = std::io::stdout().lock();
    let output = output.trim_end();
    let _ = writeln!(
        stdout,
        "::group::{}: {verdict} in {elapsed:.1?}\n{output}\n::endgroup::",
        binary.label
    );
    if !passed {
        // Repeat a failure outside its collapsed group.
        let _ = writeln!(stdout, "::error title=Test binary failed::{}\n{output}", binary.label);
    }
    Outcome { label: binary.label.clone(), passed, elapsed }
}

/// Log the failures and report whether every binary passed.
pub(crate) fn passed(outcomes: &[Outcome]) -> bool {
    let failed: Vec<&str> =
        outcomes.iter().filter(|o| !o.passed).map(|o| o.label.as_str()).collect();
    for label in &failed {
        log(format_args!("FAILED: {label}"));
    }
    failed.is_empty()
}
