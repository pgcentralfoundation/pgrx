//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Running child processes.

use std::io::{self, Read};
use std::process::{Command, ExitStatus};
use std::sync::Mutex;

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

/// Run `command` with its standard output and error captured together, in the order the
/// command wrote them.
pub(crate) fn run_captured(mut command: Command) -> eyre::Result<(ExitStatus, String)> {
    let description = describe(&command);
    let (mut reader, writer) = io::pipe().wrap_err("creating an output pipe")?;
    command.stdout(writer.try_clone().wrap_err("duplicating an output pipe")?).stderr(writer);
    let mut child = command.spawn().wrap_err_with(|| format!("starting `{description}`"))?;
    // The command holds the pipe's write ends; drop them so the read below ends when the
    // child exits instead of waiting forever.
    drop(command);
    let mut output = Vec::new();
    // Read before waiting: a child blocked on a full pipe would otherwise never exit.
    reader
        .read_to_end(&mut output)
        .wrap_err_with(|| format!("reading the output of `{description}`"))?;
    let status = child.wait().wrap_err_with(|| format!("waiting for `{description}`"))?;
    Ok((status, String::from_utf8_lossy(&output).into_owned()))
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;

    #[test]
    fn captured_output_keeps_the_streams_in_order() {
        let mut command = Command::new("sh");
        command.args(["-c", "echo one; echo two >&2; echo three; exit 3"]);
        let (status, output) = run_captured(command).unwrap();
        assert_eq!(status.code(), Some(3));
        assert_eq!(output, "one\ntwo\nthree\n");
    }
}
