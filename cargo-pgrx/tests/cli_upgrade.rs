//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

/// Path to the freshly-built `cargo-pgrx` binary, provided by cargo at compile time.
const CARGO_PGRX: &str = env!("CARGO_BIN_EXE_cargo-pgrx");

fn run(args: &[&str]) -> (bool, String, String) {
    let output = Command::new(CARGO_PGRX).args(args).output().expect("failed to spawn cargo-pgrx");
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    (output.status.success(), stdout, stderr)
}

/// Directory name of the crates.io index under `$CARGO_HOME/registry/index` for Cargo 1.85 and later.
const CURRENT_INDEX_DIR: &str = "index.crates.io-1949cf8c6b5b557f";
/// The same directory name for Cargo before 1.85.
const LEGACY_INDEX_DIR: &str = "index.crates.io-6f17d22bba15001f";

/// A crates.io release as `(version, yanked)`.
type Release = (&'static str, bool);

/// 0.19.4 is yanked and 0.20.0-beta.1 is a prerelease, so 0.19.3 is the latest normal release.
const RELEASES: &[Release] = &[
    ("0.18.1", false),
    ("0.19.2", false),
    ("0.19.3", false),
    ("0.19.4", true),
    ("0.20.0-beta.1", false),
];

const PGRX_CRATES: [&str; 5] =
    ["pgrx", "pgrx-macros", "pgrx-pg-sys", "pgrx-pg-config", "pgrx-tests"];

const PGRX_ONLY: &str = r#"[package]
name = "ext"
version = "0.1.0"

[dependencies]
pgrx = "=0.18.1"
"#;

fn entry_path(cargo_home: &Path, index_dir: &str, name: &str) -> PathBuf {
    let dir = cargo_home.join("registry/index").join(index_dir).join(".cache");
    dir.join(&name[..2]).join(&name[2..4]).join(name)
}

/// Write `name`'s entry in the file format Cargo uses for its sparse index cache.
fn write_index_entry(cargo_home: &Path, index_dir: &str, name: &str, releases: &[Release]) {
    let mut entry = vec![3u8]; // cache format version
    entry.extend_from_slice(&2u32.to_le_bytes()); // index format version
    entry.extend_from_slice(b"etag:test\0");
    for (vers, yanked) in releases {
        let cksum = "0".repeat(64);
        let json = format!(
            r#"{{"name":"{name}","vers":"{vers}","deps":[],"cksum":"{cksum}","features":{{}},"yanked":{yanked}}}"#
        );
        entry.extend_from_slice(format!("{vers}\0{json}\0").as_bytes());
    }
    let path = entry_path(cargo_home, index_dir, name);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, entry).unwrap();
}

fn write_all_index_entries(cargo_home: &Path, index_dir: &str, releases: &[Release]) {
    for name in PGRX_CRATES {
        write_index_entry(cargo_home, index_dir, name, releases);
    }
}

fn write_manifest(contents: &str) -> (TempDir, PathBuf) {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("Cargo.toml");
    fs::write(&path, contents).unwrap();
    (dir, path)
}

/// Run `cargo pgrx upgrade <args>` in `cwd` with `cargo_home` as the only Cargo cache.
fn upgrade(cargo_home: &Path, cwd: &Path, args: &[&str]) -> (bool, String, String) {
    let output = Command::new(CARGO_PGRX)
        .args(["pgrx", "upgrade"])
        .args(args)
        .env("CARGO_HOME", cargo_home)
        .current_dir(cwd)
        .output()
        .expect("failed to spawn cargo-pgrx");
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    (output.status.success(), stdout, stderr)
}

/// Run `cargo pgrx upgrade --dry-run --manifest-path <manifest> <args>`.
fn dry_run(cargo_home: &Path, manifest: &Path, args: &[&str]) -> (bool, String, String) {
    let mut all = vec!["--dry-run", "--manifest-path", manifest.to_str().unwrap()];
    all.extend_from_slice(args);
    upgrade(cargo_home, manifest.parent().unwrap(), &all)
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

const FULL_MANIFEST: &str = r#"# keep this comment
[package]
name = "ext"
version = "0.1.0"

[dependencies]
pgrx = "=0.18.1"
pgrx-macros = "~0.18.1"
pgrx-pg-sys = { version = "0.18.1", default-features = false }
renamed = { package = "pgrx-pg-config", version = "^0.18" }
serde_json = "1.0.100"

[dev-dependencies]
pgrx-tests = "=0.18.1"
"#;

/// Check that `FULL_MANIFEST` was rewritten using `RELEASES`, leaving unrelated lines alone.
fn assert_upgraded_to_0_19_3(manifest: &str) {
    for expected in [
        r#"pgrx = "=0.19.3""#,
        r#"pgrx-macros = "~0.19.3""#,
        r#"pgrx-pg-sys = { version = "0.19.3", default-features = false }"#,
        r#"renamed = { package = "pgrx-pg-config", version = "^0.19.3" }"#,
        r#"pgrx-tests = "=0.19.3""#,
        r#"serde_json = "1.0.100""#,
        "# keep this comment",
    ] {
        assert!(manifest.contains(expected), "missing `{expected}` in:\n{manifest}");
    }
    for stale in ["0.18", "0.19.2", "0.19.4", "0.20.0", "beta"] {
        assert!(!manifest.contains(stale), "unexpected `{stale}` in:\n{manifest}");
    }
}

#[test]
fn upgrade_uses_current_cargo_index_cache() {
    let home = tempfile::tempdir().unwrap();
    write_all_index_entries(home.path(), CURRENT_INDEX_DIR, RELEASES);
    let (_dir, manifest) = write_manifest(FULL_MANIFEST);

    let (success, stdout, stderr) = dry_run(home.path(), &manifest, &[]);
    assert!(success, "stderr was: {stderr}");
    assert_upgraded_to_0_19_3(&stdout);
}

#[test]
fn upgrade_ignores_the_pre_1_85_cache_directory() {
    let home = tempfile::tempdir().unwrap();
    write_all_index_entries(home.path(), LEGACY_INDEX_DIR, RELEASES);
    let (_dir, manifest) = write_manifest(PGRX_ONLY);

    let (success, _stdout, stderr) = dry_run(home.path(), &manifest, &[]);
    assert!(!success, "expected the pre-1.85 cache to be ignored");
    assert!(stderr.contains(CURRENT_INDEX_DIR), "stderr was: {stderr}");
    assert!(!stderr.contains(LEGACY_INDEX_DIR), "stderr was: {stderr}");
}

#[test]
fn upgrade_include_prereleases_picks_the_highest_version() {
    let home = tempfile::tempdir().unwrap();
    write_all_index_entries(home.path(), CURRENT_INDEX_DIR, RELEASES);
    let (_dir, manifest) = write_manifest(PGRX_ONLY);

    let (success, stdout, stderr) = dry_run(home.path(), &manifest, &["--include-prereleases"]);
    assert!(success, "stderr was: {stderr}");
    assert!(stdout.contains(r#"pgrx = "=0.20.0-beta.1""#), "unexpected manifest:\n{stdout}");
}

#[test]
fn upgrade_without_a_cache_entry_names_the_current_index_directory() {
    let home = tempfile::tempdir().unwrap();
    let (dir, manifest) = write_manifest(PGRX_ONLY);

    let (success, _stdout, stderr) =
        upgrade(home.path(), dir.path(), &["--manifest-path", manifest.to_str().unwrap()]);
    assert!(!success, "expected a missing cache entry to be an error");
    assert!(stderr.contains(CURRENT_INDEX_DIR), "stderr was: {stderr}");
    assert!(!stderr.contains(LEGACY_INDEX_DIR), "stderr was: {stderr}");
    assert_eq!(fs::read_to_string(&manifest).unwrap(), PGRX_ONLY, "manifest must be untouched");
}

#[test]
fn upgrade_reports_an_unreadable_cache_entry() {
    let home = tempfile::tempdir().unwrap();
    let entry = entry_path(home.path(), CURRENT_INDEX_DIR, "pgrx");
    fs::create_dir_all(entry.parent().unwrap()).unwrap();
    fs::write(entry, b"\x03not an index entry").unwrap();
    let (dir, manifest) = write_manifest(PGRX_ONLY);

    let (success, _stdout, stderr) =
        upgrade(home.path(), dir.path(), &["--manifest-path", manifest.to_str().unwrap()]);
    assert!(!success, "expected a corrupt cache entry to be an error");
    assert!(!stderr.contains("panicked"), "stderr was: {stderr}");
    assert_eq!(fs::read_to_string(&manifest).unwrap(), PGRX_ONLY, "manifest must be untouched");
}

#[test]
fn upgrade_to_does_not_read_the_cargo_cache() {
    let home = tempfile::tempdir().unwrap();
    let (_dir, manifest) = write_manifest(PGRX_ONLY);

    let (success, stdout, stderr) = dry_run(home.path(), &manifest, &["--to", "=0.19.3"]);
    assert!(success, "stderr was: {stderr}");
    assert!(stdout.contains(r#"pgrx = "=0.19.3""#), "unexpected manifest:\n{stdout}");

    let (success, stdout, stderr) = dry_run(home.path(), &manifest, &["--to", "0.19.3"]);
    assert!(success, "stderr was: {stderr}");
    assert!(stdout.contains(r#"pgrx = "^0.19.3""#), "unexpected manifest:\n{stdout}");
}

#[test]
fn upgrade_rewrites_the_manifest_in_place() {
    let home = tempfile::tempdir().unwrap();
    write_all_index_entries(home.path(), CURRENT_INDEX_DIR, RELEASES);
    let (dir, manifest) = write_manifest(FULL_MANIFEST);

    let (success, stdout, stderr) =
        upgrade(home.path(), dir.path(), &["--manifest-path", manifest.to_str().unwrap()]);
    assert!(success, "stderr was: {stderr}");
    assert!(stdout.is_empty(), "expected nothing on stdout without --dry-run, got: {stdout}");
    assert_upgraded_to_0_19_3(&fs::read_to_string(&manifest).unwrap());
}

#[test]
fn upgrade_without_a_manifest_in_the_working_directory_names_the_path() {
    let home = tempfile::tempdir().unwrap();
    let cwd = tempfile::tempdir().unwrap();

    let (success, _stdout, stderr) = upgrade(home.path(), cwd.path(), &[]);
    assert!(!success, "expected a missing ./Cargo.toml to be an error");
    assert!(
        stderr.contains("Error opening manifest")
            && stderr.contains("./Cargo.toml")
            && stderr.contains("--manifest-path"),
        "expected the message to name the path and the flag, stderr was: {stderr}"
    );
}
