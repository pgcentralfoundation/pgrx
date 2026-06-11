//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use std::process::Command;

/// Path to the freshly-built `cargo-pgrx` binary, provided by cargo at compile time.
const CARGO_PGRX: &str = env!("CARGO_BIN_EXE_cargo-pgrx");

fn run(args: &[&str]) -> (bool, String, String) {
    let output = Command::new(CARGO_PGRX).args(args).output().expect("failed to spawn cargo-pgrx");
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    (output.status.success(), stdout, stderr)
}

#[test]
fn upgrade_help_succeeds() {
    let (success, _stdout, stderr) = run(&["pgrx", "upgrade", "--help"]);
    assert!(success, "expected --help to succeed, stderr was: {stderr}");
}

#[test]
fn upgrade_to_rejects_garbage() {
    let (success, _stdout, stderr) = run(&["pgrx", "upgrade", "--to", "foo"]);
    assert!(!success, "expected non-zero exit for --to foo");
    assert!(
        stderr.contains("invalid value 'foo'") && stderr.contains("--to"),
        "expected clap error mentioning the invalid value and --to flag, stderr was: {stderr}"
    );
}

#[test]
fn upgrade_to_rejects_empty() {
    let (success, _stdout, stderr) = run(&["pgrx", "upgrade", "--to", ""]);
    assert!(!success, "expected non-zero exit for --to ''");
    assert!(
        stderr.contains("invalid value ''") && stderr.contains("--to"),
        "expected clap error mentioning the empty value and --to flag, stderr was: {stderr}"
    );
}
