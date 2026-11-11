//LICENSE Portions Copyright 2026-2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Path to the freshly-built `cargo-pgrx` binary, provided by cargo at compile time.
const CARGO_PGRX: &str = env!("CARGO_BIN_EXE_cargo-pgrx");

fn run(args: &[&str]) -> (bool, String, String) {
    let output = Command::new(CARGO_PGRX).args(args).output().expect("failed to spawn cargo-pgrx");
    let stdout = String::from_utf8_lossy_owned(output.stdout);
    let stderr = String::from_utf8_lossy_owned(output.stderr);
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
    assert!(stdout.contains("--upstream"), "expected --upstream in help");
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
fn every_installing_command_can_leave_fragments_out() {
    for command in ["install", "run", "package", "test", "regress", "bench"] {
        let (success, stdout, stderr) = run(&["pgrx", command, "--help"]);
        assert!(success, "expected {command} --help to succeed, stderr was: {stderr}");
        assert!(
            stdout.contains("--no-assemble-unreleased"),
            "expected --no-assemble-unreleased in {command} help"
        );
    }
}

#[test]
fn package_flags_conflict() {
    let (success, _, stderr) =
        run(&["pgrx", "package", "--assemble-unreleased", "--no-assemble-unreleased"]);
    assert!(!success);
    assert!(stderr.contains("cannot be used with"), "{stderr}");
}

#[test]
fn package_help_has_assemble_unreleased() {
    let (success, stdout, stderr) = run(&["pgrx", "package", "--help"]);
    assert!(success, "expected package --help to succeed, stderr was: {stderr}");
    assert!(stdout.contains("--assemble-unreleased"), "expected --assemble-unreleased in help");
}

/// A crate shaped like `cargo pgrx new` output that has opted into fragments.
struct Fixture {
    _temp: tempfile::TempDir,
    crate_dir: PathBuf,
}

impl Fixture {
    fn new(version: &str) -> Self {
        Self::with_metadata(
            version,
            "[package.metadata.pgrx]\nunreleased-sql-dir = \"sql/unreleased\"\n",
        )
    }

    fn not_opted_in(version: &str) -> Self {
        Self::with_metadata(version, "")
    }

    fn with_metadata(version: &str, metadata: &str) -> Self {
        let temp = tempfile::tempdir().unwrap();
        let crate_dir = temp.path().join("workspace/extensions/my_ext");
        fs::create_dir_all(crate_dir.join("sql/unreleased")).unwrap();
        fs::create_dir_all(crate_dir.join("src")).unwrap();
        fs::write(crate_dir.join("src/lib.rs"), "// empty").unwrap();
        fs::write(
            crate_dir.join("Cargo.toml"),
            format!(
                "[package]\nname = \"my_ext\"\nversion = \"{version}\"\nedition = \"2021\"\n{metadata}\n[workspace]\n"
            ),
        )
        .unwrap();
        fs::write(
            crate_dir.join("my_ext.control"),
            "comment = 'test'\ndefault_version = '@CARGO_VERSION@'\n",
        )
        .unwrap();
        Self { _temp: temp, crate_dir }
    }

    fn path(&self, relative: &str) -> PathBuf {
        self.crate_dir.join(relative)
    }

    fn sql(&self, name: &str, sql: &str) -> &Self {
        let dir = self.path("sql");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join(name), sql).unwrap();
        self
    }

    fn fragment(&self, name: &str, sql: &str) -> &Self {
        let dir = self.path("sql/unreleased");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join(name), sql).unwrap();
        self
    }

    fn set_control(&self, default_version: &str) {
        fs::write(self.path("my_ext.control"), format!("default_version = '{default_version}'\n"))
            .unwrap();
    }

    fn migrate(&self, args: &[&str]) -> (bool, String, String) {
        let manifest = self.path("Cargo.toml");
        let mut all = vec!["pgrx", "migrate"];
        all.extend_from_slice(args);
        all.extend_from_slice(&["--manifest-path", manifest.to_str().unwrap()]);
        run(&all)
    }

    fn released_scripts(&self) -> Vec<String> {
        let mut names: Vec<_> = fs::read_dir(self.path("sql"))
            .unwrap()
            .filter_map(|entry| entry.unwrap().file_name().into_string().ok())
            .filter(|name| name.ends_with(".sql"))
            .collect();
        names.sort();
        names
    }

    fn fragments(&self) -> usize {
        fs::read_dir(self.path("sql/unreleased")).unwrap().count()
    }

    fn control(&self) -> String {
        fs::read_to_string(self.path("my_ext.control")).unwrap()
    }
}

#[test]
fn migrate_cli_e2e_flow() {
    let fixture = Fixture::new("0.26.0");
    fixture
        .sql("my_ext--0.24.0--0.25.0.sql", "-- 0.25.0 baseline\n")
        .fragment("102.dep.sql", "-- depends-on: 101\nALTER TABLE my_table ADD COLUMN d text;\n")
        .fragment("101.init.sql", "CREATE TABLE my_table (id int);\n");

    let (success, stdout, stderr) = fixture.migrate(&["check"]);
    assert!(success, "check failed: {stderr}\n{stdout}");
    assert!(stdout.contains("All migration fragments passed lint checks"));

    let (success, stdout, stderr) = fixture.migrate(&["info", "--json"]);
    assert!(success, "info failed: {stderr}\n{stdout}");
    let plan: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(plan["extension_name"], "my_ext");
    assert_eq!(plan["prev_version"], "0.25.0");
    assert_eq!(plan["target_version"], "0.26.0");
    assert_eq!(plan["is_empty"], false);
    assert_eq!(plan["fragments"][0]["filename"], "101.init.sql");
    assert_eq!(plan["fragments"][1]["dependencies"][0], "101");

    let (success, stdout, stderr) = fixture.migrate(&["assemble", "0.26.0", "--dry-run", "--json"]);
    assert!(success, "assemble dry-run failed: {stderr}\n{stdout}");
    assert_eq!(fixture.fragments(), 2);
    assert_eq!(fixture.released_scripts(), ["my_ext--0.24.0--0.25.0.sql"]);

    let (success, stdout, stderr) = fixture.migrate(&["assemble", "0.26.0", "--update-control"]);
    assert!(success, "assemble failed: {stderr}\n{stdout}");
    assert_eq!(fixture.fragments(), 0);

    let script = fs::read_to_string(fixture.path("sql/my_ext--0.25.0--0.26.0.sql")).unwrap();
    assert!(script.contains("ALTER EXTENSION my_ext UPDATE TO '0.26.0'"));
    let create = script.find("CREATE TABLE my_table").unwrap();
    let alter = script.find("ALTER TABLE my_table").unwrap();
    assert!(create < alter, "fragments out of order:\n{script}");
}

#[test]
fn migrate_commands_explain_how_to_opt_in() {
    let fixture = Fixture::not_opted_in("0.26.0");
    fixture.fragment("101.init.sql", "CREATE TABLE t (id int);\n");

    for command in ["assemble", "info", "check"] {
        let (success, stdout, stderr) = fixture.migrate(&[command]);
        assert!(!success, "{command} should fail without the opt-in: {stdout}");
        assert!(stderr.contains("not enabled for this package"), "{command}: {stderr}");
        assert!(stderr.contains("unreleased-sql-dir"), "{command}: {stderr}");
    }
    assert_eq!(fixture.fragments(), 1);
}

#[test]
fn assembling_twice_keeps_the_first_script() {
    let fixture = Fixture::new("0.26.0");
    fixture
        .sql("my_ext--0.24.0--0.25.0.sql", "-- old\n")
        .fragment("101.init.sql", "CREATE TABLE important (id int);\n");

    let (success, _, stderr) = fixture.migrate(&["assemble", "0.26.0"]);
    assert!(success, "{stderr}");
    let script = fixture.path("sql/my_ext--0.25.0--0.26.0.sql");
    let assembled = fs::read_to_string(&script).unwrap();
    assert!(assembled.contains("CREATE TABLE important"));

    for args in [
        &["assemble", "0.26.0"][..],
        &["assemble", "0.26.0", "--allow-empty"],
        &["assemble", "0.26.0", "--allow-empty", "--dry-run"],
    ] {
        let (success, _, stderr) = fixture.migrate(args);
        assert!(!success, "{args:?} should refuse");
        assert!(stderr.contains("already exists") || stderr.contains("no unreleased"), "{stderr}");
    }
    assert_eq!(fs::read_to_string(&script).unwrap(), assembled);
}

#[test]
fn an_empty_release_needs_allow_empty_even_with_a_target() {
    let fixture = Fixture::new("0.26.0");
    fixture.sql("my_ext--0.24.0--0.25.0.sql", "-- old\n");

    for args in [&["assemble", "0.26.0"][..], &["assemble", "0.26.0", "--dry-run"]] {
        let (success, _, stderr) = fixture.migrate(args);
        assert!(!success, "{args:?} should refuse");
        assert!(stderr.contains("--allow-empty"), "{stderr}");
    }

    let (success, _, stderr) = fixture.migrate(&["assemble", "0.26.0", "--allow-empty"]);
    assert!(success, "{stderr}");
    assert!(fixture.path("sql/my_ext--0.25.0--0.26.0.sql").exists());
}

#[test]
fn a_first_release_asks_for_the_previous_version() {
    let fixture = Fixture::new("0.2.0");
    fixture.fragment("101.init.sql", "CREATE TABLE t (id int);\n");

    let (success, stdout, stderr) = fixture.migrate(&["assemble"]);
    assert!(!success, "{stdout}");
    assert!(stderr.contains("--prev-version"), "{stderr}");
    assert_eq!(fixture.fragments(), 1);
    assert!(fixture.released_scripts().is_empty());

    let (success, _, stderr) = fixture.migrate(&["assemble", "--prev-version", "0.1.0"]);
    assert!(success, "{stderr}");
    assert_eq!(fixture.released_scripts(), ["my_ext--0.1.0--0.2.0.sql"]);
}

#[test]
fn a_target_older_than_every_known_version_needs_a_previous_version() {
    let fixture = Fixture::new("0.26.0");
    fixture.fragment("101.init.sql", "CREATE TABLE t (id int);\n");

    // There's nothing older than 0.25.0 to start from, and 0.26.0 would make a downgrade.
    let (success, stdout, stderr) = fixture.migrate(&["assemble", "0.25.0"]);
    assert!(!success, "{stdout}");
    assert!(stderr.contains("--prev-version"), "{stderr}");
    assert!(fixture.released_scripts().is_empty());

    let (success, stdout, stderr) =
        fixture.migrate(&["assemble", "0.25.0", "--prev-version", "0.26.0"]);
    assert!(!success, "{stdout}");
    assert!(stderr.contains("must be older than the target"), "{stderr}");
}

#[test]
fn a_fragment_that_already_shipped_is_refused() {
    let fixture = Fixture::new("0.26.0");
    fixture.fragment("101.fix.sql", "CREATE TABLE t (id int);\n");
    // A stable branch released 101 in 0.25.1, and its script was copied here, but the fragment
    // wasn't deleted.
    let (success, _, stderr) = fixture.migrate(&[
        "assemble",
        "0.25.1",
        "--prev-version",
        "0.25.0",
        "--preserve-fragments",
    ]);
    assert!(success, "{stderr}");

    for args in [&["assemble"][..], &["assemble", "--dry-run"], &["info"]] {
        let (success, stdout, stderr) = fixture.migrate(args);
        assert!(!success, "{args:?} should refuse: {stdout}");
        assert!(stderr.contains("101.fix.sql") && stderr.contains("already shipped"), "{stderr}");
    }
    assert_eq!(fixture.released_scripts(), ["my_ext--0.25.0--0.25.1.sql"]);
}

#[test]
fn update_control_keeps_the_cargo_version_placeholder() {
    let fixture = Fixture::new("0.26.0");
    fixture
        .sql("my_ext--0.24.0--0.25.0.sql", "-- old\n")
        .fragment("101.init.sql", "CREATE TABLE t (id int);\n");

    let (success, _, stderr) = fixture.migrate(&["assemble", "--update-control"]);
    assert!(success, "{stderr}");
    assert!(fixture.control().contains("default_version = '@CARGO_VERSION@'"));
}

#[test]
fn update_control_needs_cargo_at_the_target_when_it_follows_cargo() {
    let fixture = Fixture::new("0.25.0");
    fixture
        .sql("my_ext--0.24.0--0.25.0.sql", "-- old\n")
        .fragment("101.init.sql", "CREATE TABLE t (id int);\n");

    let (success, stdout, stderr) = fixture.migrate(&["assemble", "0.26.0", "--update-control"]);
    assert!(!success, "{stdout}");
    assert!(stderr.contains("@CARGO_VERSION@"), "{stderr}");
    assert_eq!(fixture.fragments(), 1);
    assert_eq!(fixture.released_scripts(), ["my_ext--0.24.0--0.25.0.sql"]);
}

#[test]
fn update_control_rewrites_a_literal_version() {
    let fixture = Fixture::new("9.9.9");
    fixture.set_control("0.25.0");
    fixture
        .sql("my_ext--0.24.0--0.25.0.sql", "-- old\n")
        .fragment("101.init.sql", "CREATE TABLE t (id int);\n");

    let (success, _, stderr) = fixture.migrate(&["assemble", "0.26.0", "--update-control"]);
    assert!(success, "{stderr}");
    assert!(fixture.control().contains("default_version = '0.26.0'"));
}

#[test]
fn assemble_json_is_the_plan_for_the_fragments_it_assembled() {
    let fixture = Fixture::new("0.26.0");
    fixture
        .sql("my_ext--0.24.0--0.25.0.sql", "-- old\n")
        .fragment("101.init.sql", "CREATE TABLE t (id int);\n");

    let (success, stdout, stderr) = fixture.migrate(&["assemble", "--json"]);
    assert!(success, "{stderr}");
    let plan: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(plan["is_empty"], false);
    assert_eq!(plan["fragments"][0]["filename"], "101.init.sql");
    assert_eq!(fixture.fragments(), 0);
}

#[test]
fn script_names_keep_the_previous_version_as_spelled_on_disk() {
    let fixture = Fixture::new("1.2.0");
    fixture
        .sql("my_ext--1.0--1.1.sql", "-- old\n")
        .fragment("101.init.sql", "CREATE TABLE t (id int);\n");

    let (success, _, stderr) = fixture.migrate(&["assemble"]);
    assert!(success, "{stderr}");
    assert!(fixture.path("sql/my_ext--1.1--1.2.0.sql").exists());
}

#[test]
fn pre_release_targets_keep_their_name() {
    let fixture = Fixture::new("0.26.0-rc.1");
    fixture
        .sql("my_ext--0.24.0--0.25.0.sql", "-- old\n")
        .fragment("101.init.sql", "CREATE TABLE t (id int);\n");

    let (success, _, stderr) = fixture.migrate(&["assemble", "--preserve-fragments"]);
    assert!(success, "{stderr}");
    let script = fixture.path("sql/my_ext--0.25.0--0.26.0-rc.1.sql");
    let sql = fs::read_to_string(script).unwrap();
    assert!(sql.contains("UPDATE TO '0.26.0-rc.1'"), "{sql}");
    assert_eq!(fixture.fragments(), 1);

    // The final release upgrades from the last stable version, not through the release candidate,
    // whose script has the same fragments.
    let (success, _, stderr) = fixture.migrate(&["assemble", "0.26.0"]);
    assert!(success, "{stderr}");
    assert!(fixture.path("sql/my_ext--0.25.0--0.26.0.sql").exists());
}

#[test]
fn migrate_cli_check_formats_and_failures() {
    let fixture = Fixture::new("0.26.0");
    fixture.fragment("101.init.sql", "CREATE TABLE my_table (id int);\n").fragment(
        "102.bad.sql",
        "ALTER TABLE my_table ADD COLUMN c text;\nALTER TABLE released_tbl ADD COLUMN x int;\n",
    );

    let (success, stdout, stderr) = fixture.migrate(&["check"]);
    assert!(!success, "expected check to fail for undeclared dependency");
    assert!(stdout.contains("touches unreleased object(s) from fragment `101`"), "{stderr}");

    let (success, stdout, _) = fixture.migrate(&["check", "--format", "github"]);
    assert!(!success);
    assert!(stdout.contains("::error file="));

    let (success, stdout, _) = fixture.migrate(&["check", "--format", "json"]);
    assert!(!success);
    let issues: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(issues[0]["rule"], "undeclared_dependency");

    let (success, stdout, _) =
        fixture.migrate(&["check", "--deny-mixed-objects", "--format", "json"]);
    assert!(!success);
    let issues: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(issues[1]["rule"], "mixed_objects");
}

fn git(dir: &Path, args: &[&str]) {
    let output = Command::new("git")
        .args([
            "-c",
            "user.name=test",
            "-c",
            "user.email=test@example.com",
            "-c",
            "commit.gpgsign=false",
        ])
        .args(args)
        .current_dir(dir)
        .output()
        .expect("failed to spawn git");
    assert!(output.status.success(), "git {args:?}: {}", String::from_utf8_lossy(&output.stderr));
}

#[test]
fn check_base_works_for_a_crate_below_the_repository_root() {
    let fixture = Fixture::new("0.26.0");
    let repo = fixture.crate_dir.parent().unwrap().parent().unwrap();

    fixture.fragment("101.init.sql", "CREATE TABLE t (id int);\n");
    git(repo, &["init", "-q", "-b", "main"]);
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "base"]);

    git(repo, &["checkout", "-q", "-b", "feature"]);
    fixture.fragment("102.old_problem.sql", "ALTER TABLE t ADD COLUMN x int;\n");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "one"]);
    let (success, stdout, _) = fixture.migrate(&["check", "--base", "main", "--format", "json"]);
    assert!(!success, "a changed fragment should be linted: {stdout}");
    let issues: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(issues[0]["filename"], "102.old_problem.sql");

    // Relative to that commit nothing has changed, so 102 is no longer this branch's problem.
    let (success, stdout, stderr) = fixture.migrate(&["check", "--base", "HEAD"]);
    assert!(success, "unchanged fragments shouldn't be reported: {stderr}\n{stdout}");

    let (success, stdout, _) = fixture.migrate(&["check", "--base", "no-such-ref"]);
    assert!(!success, "a bad ref must not read as a clean lint: {stdout}");
}

#[test]
fn package_exposes_effective_extversion_at_compile_time() {
    use pgrx_pg_config::{PgConfigSelector, Pgrx};
    use pgrx_sql_entity_graph::section::schema_section_sentinel_entry;

    let Some(pg_config) = Pgrx::from_config().ok().and_then(|pgrx| {
        pgrx.iter(PgConfigSelector::All)
            .filter_map(Result::ok)
            .find(|config| config.path().is_some_and(|path| path.is_file()))
    }) else {
        eprintln!("skipping extversion build test: no configured Postgres installation");
        return;
    };

    let fixture = Fixture::new("0.26.0");
    fixture
        .sql("my_ext--0.25.0--0.26.0.sql", "-- released\n")
        .fragment("101.init.sql", "SELECT 1;\n");

    fs::write(
        fixture.path("Cargo.toml"),
        r#"[package]
name = "my_ext"
version = "0.26.0"
edition = "2024"

[workspace]

[lib]
crate-type = ["cdylib"]

[features]
pg15 = []
pg16 = []
pg17 = []
pg18 = []
pg19 = []

[package.metadata.pgrx]
unreleased-sql-dir = "sql/unreleased"
"#,
    )
    .unwrap();
    fs::write(
        fixture.path("my_ext.control"),
        "comment = 'test'\ndefault_version = '@CARGO_VERSION@'\nmodule_pathname = '$libdir/my_ext'\nrelocatable = false\nsuperuser = false\n",
    )
    .unwrap();

    let sentinel = schema_section_sentinel_entry();
    fs::write(
        fixture.path("src/lib.rs"),
        format!(
            r#"#[used]
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "macos", unsafe(link_section = "__DATA,__pgrxsc"))]
#[cfg_attr(not(target_os = "macos"), unsafe(link_section = ".pgrxsc"))]
pub static PGRX_SCHEMA: [u8; {len}] = {sentinel:?};

const fn str_eq(a: &str, b: &str) -> bool {{
    let (a, b) = (a.as_bytes(), b.as_bytes());
    if a.len() != b.len() {{
        return false;
    }}
    let mut i = 0;
    while i < a.len() {{
        if a[i] != b[i] {{
            return false;
        }}
        i += 1;
    }}
    true
}}

const _: () = assert!(
    str_eq(env!("CARGO_PGRX_EXTVERSION"), env!("EXPECTED_EXTVERSION")),
    "CARGO_PGRX_EXTVERSION did not match EXPECTED_EXTVERSION",
);
"#,
            len = sentinel.len(),
        ),
    )
    .unwrap();

    let manifest = fixture.path("Cargo.toml");
    let out_dir = fixture.path("out");
    for (flag, expected) in
        [("--assemble-unreleased", "0.26.1"), ("--no-assemble-unreleased", "0.26.0")]
    {
        let output = Command::new(CARGO_PGRX)
            .args([
                "pgrx",
                "package",
                "--debug",
                "--no-distrib-packages",
                flag,
                "--pg-config",
                pg_config.path().unwrap().to_str().unwrap(),
                "--manifest-path",
                manifest.to_str().unwrap(),
                "--out-dir",
                out_dir.to_str().unwrap(),
            ])
            .env("CARGO_PGRX_EXTVERSION", "9.9.9")
            .env("EXPECTED_EXTVERSION", expected)
            .output()
            .expect("failed to spawn cargo-pgrx");
        assert!(
            output.status.success(),
            "package ({flag}) failed:\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        );
    }
}

#[test]
fn check_enforces_shipped_deps_upstream_identity_and_released_immutability() {
    let fixture = Fixture::new("0.26.0");
    let repo = fixture.crate_dir.parent().unwrap().parent().unwrap();

    // Baseline commit (`branch_point`): `100.shipped.sql` already shipped in `0.24.0--0.25.0`.
    fixture.sql(
        "my_ext--0.24.0--0.25.0.sql",
        "\\echo Use \"ALTER EXTENSION my_ext UPDATE TO '0.25.0'\" to load this file. \\quit\n\n-- Fragment: 100.shipped.sql\nCREATE TABLE shipped_t (id int);\n",
    );
    git(repo, &["init", "-q", "-b", "main"]);
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "branch_point"]);
    git(repo, &["branch", "0.25.x"]);

    // Unresolved `depends-on` is rejected by `check`, `info`, and `assemble`.
    fixture.fragment("199.typo.sql", "-- depends-on: 999\nSELECT 1;\n");
    let (success, stdout, _) = fixture.migrate(&["check", "--format", "json"]);
    assert!(!success, "unresolved depends-on must fail check: {stdout}");
    let issues: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(issues[0]["rule"], "unresolved_dependency");
    for cmd in [&["info"][..], &["assemble", "--dry-run"]] {
        let (success, _, stderr) = fixture.migrate(cmd);
        assert!(!success, "{cmd:?} must reject unresolved depends-on");
        assert!(stderr.contains("depends on `999`"), "{stderr}");
    }
    fs::remove_file(fixture.path("sql/unreleased/199.typo.sql")).unwrap();

    // Dependencies satisfied by `sql/` (`100`) or `sql/unreleased/` (`101`) pass on `main`.
    fixture
        .fragment("101.from_shipped.sql", "-- depends-on: 100\nCREATE TABLE a (id int);\n")
        .fragment("102.needs_101.sql", "-- depends-on: 101\nALTER TABLE a ADD COLUMN x int;\n");
    let (success, stdout, stderr) = fixture.migrate(&["check", "--deny-mixed-objects"]);
    assert!(success, "valid shipped + unreleased deps should pass: {stderr}\n{stdout}");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "main_fragments"]);

    // On `0.25.x`, backporting `101` identically passes `--upstream main`, while backporting `102`
    // without `101` fails `unresolved_dependency`, and diverging/renaming fails `upstream_divergence`.
    git(repo, &["checkout", "-q", "0.25.x"]);
    fixture.fragment("101.from_shipped.sql", "-- depends-on: 100\nCREATE TABLE a (id int);\n");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "backport_101"]);
    let (success, stdout, stderr) = fixture.migrate(&[
        "check",
        "--base",
        "HEAD~1",
        "--upstream",
        "main",
        "--deny-mixed-objects",
    ]);
    assert!(success, "clean backport should pass: {stderr}\n{stdout}");

    let (success, stdout, _) = fixture.migrate(&["check", "--upstream", "no-such-ref"]);
    assert!(!success, "bad upstream ref must fail: {stdout}");

    fs::remove_file(fixture.path("sql/unreleased/101.from_shipped.sql")).unwrap();
    fixture
        .fragment("102.needs_101.sql", "-- depends-on: 101\nALTER TABLE a ADD COLUMN x int;\n")
        .fragment("101.from_shipped.sql", "-- depends-on: 100\nCREATE TABLE a (id bigint);\n")
        .fragment("300.not_on_main.sql", "SELECT 1;\n");
    fs::remove_file(fixture.path("sql/unreleased/101.from_shipped.sql")).unwrap();
    // First check missing prerequisite `101` + non-existent `300.not_on_main.sql`, plus edited `102`:
    fixture.fragment("102.needs_101.sql", "-- depends-on: 101\nSELECT 'diverged';\n");
    let (success, stdout, _) =
        fixture.migrate(&["check", "--upstream", "main", "--format", "json"]);
    assert!(!success, "{stdout}");
    let issues: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let rules: Vec<&str> =
        issues.as_array().unwrap().iter().map(|issue| issue["rule"].as_str().unwrap()).collect();
    assert_eq!(rules, ["upstream_divergence", "upstream_divergence", "unresolved_dependency"]);

    // Restore clean fragments on `0.25.x` and verify released-script immutability under `--base`:
    // adding a new script (`A`) is allowed, while modifying (`M`) or deleting (`D`) an existing one fails.
    fs::remove_file(fixture.path("sql/unreleased/102.needs_101.sql")).unwrap();
    fs::remove_file(fixture.path("sql/unreleased/300.not_on_main.sql")).unwrap();
    git(repo, &["checkout", "-q", "--", "."]);

    fixture.sql("my_ext--0.25.0--0.25.1.sql", "-- newly assembled patch release\n");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "add_patch_script"]);
    let (success, stdout, stderr) = fixture.migrate(&["check", "--base", "HEAD~1"]);
    assert!(success, "adding a new upgrade script must pass: {stderr}\n{stdout}");

    fixture.sql("my_ext--0.24.0--0.25.0.sql", "-- tampered\n");
    git(repo, &["add", "."]);
    git(repo, &["commit", "-q", "-m", "modify_released_script"]);
    let (success, stdout, _) = fixture.migrate(&["check", "--base", "HEAD~1", "--format", "json"]);
    assert!(!success, "modifying an existing upgrade script must fail: {stdout}");
    let issues: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(issues[0]["rule"], "released_script_modified");
    assert!(issues[0]["message"].as_str().unwrap().contains("modified"));

    fs::remove_file(fixture.path("sql/my_ext--0.24.0--0.25.0.sql")).unwrap();
    git(repo, &["add", "-A"]);
    git(repo, &["commit", "-q", "-m", "delete_released_script"]);
    let (success, stdout, _) = fixture.migrate(&["check", "--base", "HEAD~1", "--format", "json"]);
    assert!(!success, "deleting an existing upgrade script must fail: {stdout}");
    let issues: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(issues[0]["rule"], "released_script_modified");
    assert!(issues[0]["message"].as_str().unwrap().contains("deleted"));
}
