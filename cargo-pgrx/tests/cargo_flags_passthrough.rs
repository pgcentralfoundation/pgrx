//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Tests for the `--cargo` passthrough (issue #2135).
//!
//! `--cargo` flags must reach **every** cargo invocation cargo-pgrx makes —
//! in particular `cargo metadata`, the first one — while `PGRX_BUILD_FLAGS`
//! stays build-only. We assert this by spawning the real binary with a bogus
//! flag and checking *where* it fails: at `cargo metadata` (forwarded) or not.

use std::path::PathBuf;
use std::process::Command;

const CARGO_PGRX: &str = env!("CARGO_BIN_EXE_cargo-pgrx");
/// A flag cargo doesn't recognize, so whichever cargo invocation receives it
/// fails loudly and unmistakably.
const BOGUS: &str = "--pgrx-cargo-flags-probe";

/// Create a throwaway crate and run `cargo pgrx install` against it with the
/// given extra CLI args and env vars. Returns the combined stdout+stderr.
fn install_with(test_tag: &str, extra_args: &[&str], envs: &[(&str, &str)]) -> String {
    let dir: PathBuf =
        std::env::temp_dir().join(format!("pgrx-cargoflags-{test_tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let status = Command::new(env!("CARGO"))
        .args(["new", "--lib", "--name", "cargoflagsprobe"])
        .arg(&dir)
        .status()
        .expect("failed to scaffold temp crate");
    assert!(status.success(), "could not create temp crate");

    let mut cmd = Command::new(CARGO_PGRX);
    cmd.args(["pgrx", "install", "--manifest-path"]).arg(dir.join("Cargo.toml"));
    cmd.args(extra_args);
    for (k, v) in envs {
        cmd.env(k, v);
    }
    let output = cmd.output().expect("failed to spawn cargo-pgrx");
    let mut combined = String::from_utf8_lossy(&output.stderr).into_owned();
    combined.push_str(&String::from_utf8_lossy(&output.stdout));

    let _ = std::fs::remove_dir_all(&dir);
    combined
}

/// `--cargo` must reach `cargo metadata`: a bogus flag there makes the very
/// first step — `cargo metadata` — fail.
#[test]
fn cargo_flag_reaches_cargo_metadata() {
    let out = install_with("cargo", &[&format!("--cargo={BOGUS}")], &[]);
    assert!(
        out.contains("couldn't get cargo metadata"),
        "expected the bogus --cargo flag to make `cargo metadata` fail, got:\n{out}"
    );
    assert!(
        out.contains(BOGUS),
        "expected cargo's error to mention the forwarded flag `{BOGUS}`, got:\n{out}"
    );
}

/// The quoted-string form (`--cargo "--flag-a --flag-b"`) is split on
/// whitespace: two valid cargo flags packed into one `--cargo` value get past
/// `cargo metadata`. Without splitting, the mashed-together token would be an
/// unknown argument and `cargo metadata` would reject it.
#[test]
fn cargo_flag_whitespace_string_is_split() {
    let out = install_with("split", &["--cargo", "--offline --color=never"], &[]);
    assert!(
        !out.contains("couldn't get cargo metadata"),
        "two valid flags in one --cargo value should be split and pass `cargo metadata`, got:\n{out}"
    );
}

/// `PGRX_BUILD_FLAGS` must stay build-only: a bogus flag there must NOT break
/// `cargo metadata` (the run gets past it and fails later, for other reasons).
/// This is the property that keeps `--cargo` distinct from build-only flags.
#[test]
fn pgrx_build_flags_does_not_reach_cargo_metadata() {
    let out = install_with("build", &[], &[("PGRX_BUILD_FLAGS", BOGUS)]);
    assert!(
        !out.contains("couldn't get cargo metadata"),
        "PGRX_BUILD_FLAGS must not be forwarded to `cargo metadata`, but it failed there:\n{out}"
    );
}

/// Sanity: with no `--cargo` flag, `cargo metadata` is not what fails for a
/// bogus reason — it succeeds and the run fails later (no control file, etc.).
#[test]
fn no_flags_does_not_break_cargo_metadata() {
    let out = install_with("none", &[], &[]);
    assert!(
        !out.contains("couldn't get cargo metadata"),
        "a clean run should get past `cargo metadata`, got:\n{out}"
    );
}
