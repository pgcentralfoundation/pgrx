//LICENSE Portions Copyright 2026-2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
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
fn migrate_help_succeeds() {
    let (success, stdout, stderr) = run(&["pgrx", "migrate", "--help"]);
    assert!(success, "expected --help to succeed, stderr was: {stderr}");
    assert!(stdout.contains("assemble"), "expected assemble subcommand in help output");
    assert!(stdout.contains("info"), "expected info subcommand in help output");
    assert!(stdout.contains("check"), "expected check subcommand in help output");
}

#[test]
fn migrate_assemble_help_succeeds() {
    let (success, stdout, stderr) = run(&["pgrx", "migrate", "assemble", "--help"]);
    assert!(success, "expected assemble --help to succeed, stderr was: {stderr}");
    assert!(stdout.contains("--prev-version"), "expected --prev-version in help");
    assert!(stdout.contains("--preserve-fragments"), "expected --preserve-fragments in help");
    assert!(stdout.contains("--output-dir"), "expected --output-dir in help");
    assert!(stdout.contains("--allow-empty"), "expected --allow-empty in help");
    assert!(stdout.contains("--update-control"), "expected --update-control in help");
    assert!(stdout.contains("--dry-run"), "expected --dry-run in help");
    assert!(stdout.contains("--json"), "expected --json in help");
}

#[test]
fn migrate_info_help_succeeds() {
    let (success, stdout, stderr) = run(&["pgrx", "migrate", "info", "--help"]);
    assert!(success, "expected info --help to succeed, stderr was: {stderr}");
    assert!(stdout.contains("--prev-version"), "expected --prev-version in help");
    assert!(stdout.contains("--output-dir"), "expected --output-dir in help");
    assert!(stdout.contains("--json"), "expected --json in help");
}

#[test]
fn migrate_check_help_succeeds() {
    let (success, stdout, stderr) = run(&["pgrx", "migrate", "check", "--help"]);
    assert!(success, "expected check --help to succeed, stderr was: {stderr}");
    assert!(stdout.contains("--base"), "expected --base in help");
    assert!(stdout.contains("--deny-mixed-objects"), "expected --deny-mixed-objects in help");
    assert!(stdout.contains("--format"), "expected --format in help");
}

#[test]
fn migrate_lint_alias_help_succeeds() {
    let (success, stdout, stderr) = run(&["pgrx", "migrate", "lint", "--help"]);
    assert!(success, "expected lint --help to succeed, stderr was: {stderr}");
    assert!(stdout.contains("--base"), "expected --base in help");
    assert!(stdout.contains("--deny-mixed-objects"), "expected --deny-mixed-objects in help");
    assert!(stdout.contains("--format"), "expected --format in help");
}

#[test]
fn install_help_has_no_assemble_unreleased() {
    let (success, stdout, stderr) = run(&["pgrx", "install", "--help"]);
    assert!(success, "expected install --help to succeed, stderr was: {stderr}");
    assert!(
        stdout.contains("--no-assemble-unreleased"),
        "expected --no-assemble-unreleased in help"
    );
}

#[test]
fn package_help_has_assemble_unreleased() {
    let (success, stdout, stderr) = run(&["pgrx", "package", "--help"]);
    assert!(success, "expected package --help to succeed, stderr was: {stderr}");
    assert!(stdout.contains("--assemble-unreleased"), "expected --assemble-unreleased in help");
}

#[test]
fn migrate_cli_e2e_flow() {
    let temp = tempfile::tempdir().unwrap();
    let crate_dir = temp.path().join("my_ext");
    let unreleased_dir = crate_dir.join("sql/unreleased");
    let sql_dir = crate_dir.join("sql");
    std::fs::create_dir_all(&unreleased_dir).unwrap();

    let manifest_path = crate_dir.join("Cargo.toml");
    std::fs::write(
        &manifest_path,
        r#"[package]
name = "my_ext"
version = "0.26.0"
edition = "2021"
"#,
    )
    .unwrap();
    std::fs::create_dir_all(crate_dir.join("src")).unwrap();
    std::fs::write(crate_dir.join("src/lib.rs"), "// empty").unwrap();

    let control_path = crate_dir.join("my_ext.control");
    std::fs::write(&control_path, "default_version = '0.25.0'\ncomment = 'my_ext test'\n").unwrap();

    std::fs::write(sql_dir.join("my_ext--0.24.0--0.25.0.sql"), "-- 0.25.0 baseline\n").unwrap();

    std::fs::write(unreleased_dir.join("101.init.sql"), "CREATE TABLE my_table (id int);\n")
        .unwrap();

    std::fs::write(
        unreleased_dir.join("102.dep.sql"),
        "-- depends-on: 101\nALTER TABLE my_table ADD COLUMN description text;\n",
    )
    .unwrap();

    let manifest_arg = manifest_path.to_str().unwrap();

    // 1. Run check
    let (success, stdout, stderr) =
        run(&["pgrx", "migrate", "check", "--manifest-path", manifest_arg]);
    assert!(success, "check failed: {stderr}\n{stdout}");
    assert!(stdout.contains("All migration fragments passed lint checks"));

    // 2. Run info --json
    let (success, stdout, stderr) =
        run(&["pgrx", "migrate", "info", "--manifest-path", manifest_arg, "--json"]);
    assert!(success, "info failed: {stderr}\n{stdout}");
    assert!(stdout.contains(r#""extension_name": "my_ext""#));
    assert!(stdout.contains(r#""prev_version": "0.25.0""#));
    assert!(stdout.contains(r#""target_version": "0.26.0""#));
    assert!(stdout.contains(r#""is_empty": false"#));

    // 3. Run assemble --dry-run
    let (success, stdout, stderr) = run(&[
        "pgrx",
        "migrate",
        "assemble",
        "0.26.0",
        "--manifest-path",
        manifest_arg,
        "--dry-run",
        "--json",
    ]);
    assert!(success, "assemble dry-run failed: {stderr}\n{stdout}");
    assert!(unreleased_dir.join("101.init.sql").exists());
    assert!(unreleased_dir.join("102.dep.sql").exists());

    // 4. Run assemble with --update-control
    let (success, stdout, stderr) = run(&[
        "pgrx",
        "migrate",
        "assemble",
        "0.26.0",
        "--manifest-path",
        manifest_arg,
        "--update-control",
    ]);
    assert!(success, "assemble failed: {stderr}\n{stdout}");
    let assembled_file = sql_dir.join("my_ext--0.25.0--0.26.0.sql");
    assert!(assembled_file.exists());
    assert!(!unreleased_dir.join("101.init.sql").exists());

    let control_content = std::fs::read_to_string(&control_path).unwrap();
    assert!(control_content.contains("default_version = '0.26.0'"));
}

#[test]
fn migrate_cli_check_formats_and_failures() {
    let temp = tempfile::tempdir().unwrap();
    let crate_dir = temp.path().join("my_ext");
    let unreleased_dir = crate_dir.join("sql/unreleased");
    std::fs::create_dir_all(&unreleased_dir).unwrap();

    let manifest_path = crate_dir.join("Cargo.toml");
    std::fs::write(
        &manifest_path,
        r#"[package]
name = "my_ext"
version = "0.26.0"
edition = "2021"
"#,
    )
    .unwrap();
    std::fs::create_dir_all(crate_dir.join("src")).unwrap();
    std::fs::write(crate_dir.join("src/lib.rs"), "// empty").unwrap();

    std::fs::write(crate_dir.join("my_ext.control"), "default_version = '0.25.0'\n").unwrap();

    std::fs::write(unreleased_dir.join("101.init.sql"), "CREATE TABLE my_table (id int);\n")
        .unwrap();

    // 102 modifies my_table without declaring dependency on 101, and alters a released table
    std::fs::write(
        unreleased_dir.join("102.bad.sql"),
        "ALTER TABLE my_table ADD COLUMN c text;\nALTER TABLE released_tbl ADD COLUMN x int;\n",
    )
    .unwrap();

    let manifest_arg = manifest_path.to_str().unwrap();

    // 1. Text format
    let (success, stdout, stderr) =
        run(&["pgrx", "migrate", "check", "--manifest-path", manifest_arg]);
    assert!(!success, "expected check to fail for undeclared dependency");
    assert!(
        stdout.contains("touches unreleased object(s) from fragment `101`")
            || stderr.contains("touches unreleased object(s) from fragment `101`")
    );

    // 2. GitHub format
    let (success, stdout, stderr) =
        run(&["pgrx", "migrate", "check", "--manifest-path", manifest_arg, "--format", "github"]);
    assert!(!success);
    assert!(stdout.contains("::error file=") || stderr.contains("::error file="));

    // 3. JSON format
    let (success, stdout, _stderr) =
        run(&["pgrx", "migrate", "check", "--manifest-path", manifest_arg, "--format", "json"]);
    assert!(!success);
    assert!(stdout.contains(r#""rule": "undeclared_dependency""#));

    // 4. Deny mixed objects
    let (success, stdout, _stderr) = run(&[
        "pgrx",
        "migrate",
        "check",
        "--manifest-path",
        manifest_arg,
        "--deny-mixed-objects",
        "--format",
        "json",
    ]);
    assert!(!success);
    assert!(stdout.contains(r#""rule": "mixed_objects""#));
}
