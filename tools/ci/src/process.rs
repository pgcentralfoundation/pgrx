//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Running child processes.

use std::fs::File;
use std::path::PathBuf;
use std::process::{Command, ExitStatus};
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use eyre::WrapErr;

/// Serializes output that must not interleave between threads, such as one test binary's log.
pub(crate) static OUTPUT: Mutex<()> = Mutex::new(());

/// Print a line without interleaving it with another thread's block of output.
pub(crate) fn log(line: impl std::fmt::Display) {
    let _guard = OUTPUT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    println!("{line}");
}

/// A command as a shell would spell it, for logs.
pub(crate) fn describe(command: &Command) -> String {
    let mut words = vec![command.get_program().to_string_lossy().into_owned()];
    words.extend(command.get_args().map(|arg| arg.to_string_lossy().into_owned()));
    words.join(" ")
}

/// Run `command` with inherited output and report whether it succeeded. Failing to start it is
/// an error.
pub(crate) fn run(command: &mut Command) -> eyre::Result<bool> {
    log(format_args!("+ {}", describe(command)));
    let status = command.status().wrap_err_with(|| format!("starting `{}`", describe(command)))?;
    Ok(status.success())
}

/// How a captured run ended.
pub(crate) enum Exit {
    Status(ExitStatus),
    /// The command ran past its time limit and was killed.
    TimedOut,
}

/// Run `command` with its standard output and error captured together, in the order it wrote
/// them, killing it after `limit`. A command that is killed also reports the processes it had
/// started, which show what it was waiting for.
///
/// The output goes to a file rather than a pipe. A test binary can leave a process behind,
/// such as a PostgreSQL server, that inherited its output handles; a pipe would not reach its
/// end until that process exits too, but the run ends when the command itself does.
pub(crate) fn run_captured(mut command: Command, limit: Duration) -> eyre::Result<(Exit, String)> {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    let description = describe(&command);
    let path = std::env::temp_dir().join(format!(
        "pgrx-ci-{}-{}.log",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    let log = LogFile::create(path)?;
    command.stdout(log.file.try_clone()?).stderr(log.file.try_clone()?);
    let mut child = command.spawn().wrap_err_with(|| format!("starting `{description}`"))?;
    drop(command);
    let started = Instant::now();
    let mut waiting_on = String::new();
    let exit = loop {
        if let Some(status) =
            child.try_wait().wrap_err_with(|| format!("waiting for `{description}`"))?
        {
            break Exit::Status(status);
        }
        if started.elapsed() >= limit {
            waiting_on = descendants(child.id());
            // The process may have exited since the check above, so a failed kill is fine.
            let _ = child.kill();
            let _ = child.wait();
            break Exit::TimedOut;
        }
        std::thread::sleep(Duration::from_millis(50));
    };
    let output = std::fs::read(&log.path)
        .wrap_err_with(|| format!("reading the output of `{description}`"))?;
    Ok((exit, String::from_utf8_lossy(&output).into_owned() + &waiting_on))
}

/// Describe the processes `root` started, directly or not, that are still running. A hung
/// command is usually waiting for one of them. Failing to list processes describes none.
fn descendants(root: u32) -> String {
    let mut list = if cfg!(windows) {
        let mut list = Command::new("powershell");
        list.args([
            "-NoProfile",
            "-Command",
            "Get-CimInstance Win32_Process | ForEach-Object { \"$($_.ProcessId) $($_.ParentProcessId) $($_.CommandLine)\" }",
        ]);
        list
    } else {
        let mut list = Command::new("ps");
        list.args(["-eo", "pid=,ppid=,args="]);
        list
    };
    match list.output() {
        Ok(output) => describe_descendants(root, &String::from_utf8_lossy(&output.stdout)),
        Err(_) => String::new(),
    }
}

/// The lines of `listing`, each a process id, its parent's id and its command line, for the
/// processes below `root`, parents before children.
fn describe_descendants(root: u32, listing: &str) -> String {
    let processes: Vec<(u32, u32, &str)> = listing
        .lines()
        .filter_map(|line| {
            let line = line.trim_start();
            let (pid, rest) = line.split_once(char::is_whitespace)?;
            let rest = rest.trim_start();
            let (parent, command) = rest.split_once(char::is_whitespace).unwrap_or((rest, ""));
            Some((pid.parse().ok()?, parent.parse().ok()?, command.trim()))
        })
        .collect();
    let mut described = String::new();
    let mut parents = vec![root];
    // Process ids are reused, so a listing can hold a cycle; visit each process once.
    let mut seen = vec![root];
    while let Some(parent) = parents.pop() {
        for (pid, _, command) in processes.iter().filter(|process| process.1 == parent) {
            if !seen.contains(pid) {
                seen.push(*pid);
                parents.push(*pid);
                described.push_str(&format!(
                    "still running: process {pid}, started by {parent}: {command}\n"
                ));
            }
        }
    }
    described
}

/// A log file removed when dropped.
struct LogFile {
    path: PathBuf,
    file: File,
}

impl LogFile {
    fn create(path: PathBuf) -> eyre::Result<Self> {
        let file =
            File::create_new(&path).wrap_err_with(|| format!("creating {}", path.display()))?;
        Ok(Self { path, file })
    }
}

impl Drop for LogFile {
    fn drop(&mut self) {
        // On Windows a lingering child may still hold the file open; it stays in the temp
        // directory then.
        let _ = std::fs::remove_file(&self.path);
    }
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;

    #[test]
    fn captured_output_keeps_the_streams_in_order() {
        let mut command = Command::new("sh");
        command.args(["-c", "echo one; echo two >&2; echo three; exit 3"]);
        let (exit, output) = run_captured(command, Duration::from_secs(60)).unwrap();
        assert!(matches!(exit, Exit::Status(status) if status.code() == Some(3)));
        assert_eq!(output, "one\ntwo\nthree\n");
    }

    #[test]
    fn a_lingering_grandchild_does_not_delay_the_result() {
        let mut command = Command::new("sh");
        command.args(["-c", "echo started; sleep 30 & exit 0"]);
        let started = Instant::now();
        let (exit, output) = run_captured(command, Duration::from_secs(60)).unwrap();
        assert!(matches!(exit, Exit::Status(status) if status.success()));
        assert_eq!(output, "started\n");
        assert!(started.elapsed() < Duration::from_secs(10));
    }

    #[test]
    fn a_hung_command_is_killed_at_its_limit() {
        let mut command = Command::new("sh");
        command.args(["-c", "echo partial; exec sleep 30"]);
        let (exit, output) = run_captured(command, Duration::from_millis(500)).unwrap();
        assert!(matches!(exit, Exit::TimedOut));
        assert_eq!(output, "partial\n");
    }

    #[test]
    fn a_hung_command_reports_the_processes_it_started() {
        let mut command = Command::new("sh");
        command.args(["-c", "echo partial; sleep 31; echo unreachable"]);
        let (exit, output) = run_captured(command, Duration::from_millis(500)).unwrap();
        assert!(matches!(exit, Exit::TimedOut));
        let (partial, waiting_on) = output.split_once('\n').unwrap();
        assert_eq!(partial, "partial");
        assert!(waiting_on.starts_with("still running: process "), "{waiting_on}");
        assert!(waiting_on.trim_end().ends_with(": sleep 31"), "{waiting_on}");
    }

    #[test]
    fn descendants_are_listed_parents_first_and_once() {
        let listing = "1 0 init\n 10 1 test binary\n11 10 cargo pgrx install\n12 11 rustc --crate-name x\n13 1 unrelated\n10 12 reused id\n";
        assert_eq!(
            describe_descendants(10, listing),
            "still running: process 11, started by 10: cargo pgrx install\nstill running: process 12, started by 11: rustc --crate-name x\n"
        );
        assert_eq!(describe_descendants(13, listing), "");
    }
}
