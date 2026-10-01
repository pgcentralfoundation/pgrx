#![cfg(all(feature = "cli", unix))]

use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures").join(name)
}

struct TestConfig {
    home: PathBuf,
}

impl TestConfig {
    fn new() -> Self {
        let nonce = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let home =
            std::env::temp_dir().join(format!("pgrx-c-macros-cli-{}-{nonce}", std::process::id()));
        std::fs::create_dir(&home).expect("isolated PGRX_HOME must be created");
        let config = Self { home };
        let pg_config = config.home.join("pg_config");
        std::fs::copy(fixture("pg_config.sh"), &pg_config).unwrap();
        std::fs::set_permissions(&pg_config, std::fs::Permissions::from_mode(0o700)).unwrap();
        let config_toml =
            format!("[configs]\npg18 = {}\n", serde_json::to_string(&pg_config).unwrap());
        std::fs::write(config.home.join("config.toml"), config_toml).unwrap();
        config
    }

    fn command(&self) -> Command {
        let mut command = Command::new(env!("CARGO_BIN_EXE_pgrx-c-macros"));
        command
            .env("PGRX_HOME", &self.home)
            .env("PGRX_C_MACROS_TEST_INCLUDE_DIR", fixture("postgres.h").parent().unwrap())
            .env_remove("PGRX_C_MACROS_TEST_CPPFLAGS")
            .env_remove("PGRX_PG_CONFIG_PATH")
            .env_remove("PGRX_PG_CONFIG_AS_ENV")
            .env_remove("PG_CONFIG");
        command
    }
}

impl Drop for TestConfig {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.home);
    }
}

fn list(header: &str, arguments: &[&str]) -> Output {
    let config = TestConfig::new();
    config
        .command()
        .arg("list")
        .arg("pg18")
        .arg(fixture(header))
        .args(arguments)
        .output()
        .expect("macro discovery CLI must run")
}

fn successful_stdout(output: Output) -> String {
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    String::from_utf8(output.stdout).expect("CLI output must be UTF-8")
}

#[test]
fn requires_a_configured_postgres_version_and_uses_its_default_header_and_cppflags() {
    let config = TestConfig::new();
    for version in ["pg18", "18"] {
        let output = config
            .command()
            .args([
                "list",
                version,
                "--format",
                "names",
                "--name",
                "POSTGRES_FIXTURE_SERVER",
                "--name",
                "INCLUDED_VALUE",
                "--name",
                "PG_CONFIG_FIXTURE_VALUE",
            ])
            .output()
            .unwrap();
        assert_eq!(
            successful_stdout(output),
            "INCLUDED_VALUE\nPG_CONFIG_FIXTURE_VALUE\nPOSTGRES_FIXTURE_SERVER\n"
        );
    }
    let main_file = config
        .command()
        .args(["list", "pg18", "--format", "names", "--main-file-only"])
        .output()
        .unwrap();
    assert_eq!(successful_stdout(main_file), "POSTGRES_FIXTURE_SERVER\n");

    let missing_version = config.command().arg("list").output().unwrap();
    assert_eq!(missing_version.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&missing_version.stderr).contains("<PG_VERSION>"));
    assert!(missing_version.stdout.is_empty());

    let help = successful_stdout(config.command().args(["list", "--help"]).output().unwrap());
    let usage = help.lines().find(|line| line.starts_with("Usage:")).expect("help has usage");
    assert!(usage.contains("<PG_VERSION> [HEADER]"), "{usage}");
    assert!(!help.contains("--pg-version"));

    let old_option = config.command().args(["list", "--pg-version", "pg18"]).output().unwrap();
    assert_eq!(old_option.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&old_option.stderr).contains("--pg-version"));
    assert!(old_option.stdout.is_empty());

    let unconfigured = config.command().args(["list", "pg17"]).output().unwrap();
    assert!(!unconfigured.status.success());
    assert!(String::from_utf8_lossy(&unconfigured.stderr).contains("pg17"));
    assert!(unconfigured.stdout.is_empty());
}

#[test]
fn resolves_pg_config_path_override_and_rejects_version_mismatch_and_malformed_cppflags() {
    let config = TestConfig::new();
    std::fs::remove_file(config.home.join("config.toml")).unwrap();
    let pg_config = config.home.join("pg_config");
    let selected = config
        .command()
        .env("PGRX_PG_CONFIG_PATH", &pg_config)
        .args(["list", "pg18", "--format", "names", "--name", "POSTGRES_FIXTURE_SERVER"])
        .output()
        .unwrap();
    assert_eq!(successful_stdout(selected), "POSTGRES_FIXTURE_SERVER\n");

    let mismatch = config
        .command()
        .env("PGRX_PG_CONFIG_PATH", &pg_config)
        .args(["list", "pg17"])
        .output()
        .unwrap();
    assert!(!mismatch.status.success());
    assert!(mismatch.stdout.is_empty());
    assert!(String::from_utf8_lossy(&mismatch.stderr).contains("pg17"));

    let malformed = config
        .command()
        .env("PGRX_PG_CONFIG_PATH", &pg_config)
        .env("PGRX_C_MACROS_TEST_CPPFLAGS", "-DUNTERMINATED='")
        .args(["list", "pg18"])
        .output()
        .unwrap();
    assert!(!malformed.status.success());
    assert!(malformed.stdout.is_empty());
    assert!(String::from_utf8_lossy(&malformed.stderr).contains("CPPFLAGS"));
}

#[test]
fn lists_definitions_and_applies_exact_name_kind_and_source_filters() {
    let definitions = successful_stdout(list("definitions.h", &[]));
    assert!(definitions.lines().all(|line| line.starts_with("#define ")));
    assert!(definitions.contains("#define INCLUDED_VALUE 41"));
    assert!(definitions.contains("#define FUNCTION("));
    assert!(definitions.contains("#define OBJECT_SPACED ("));
    assert!(definitions.contains("/* retained comment */"));
    assert_eq!(
        definitions.lines().filter(|line| line.starts_with("#define REDEFINED ")).count(),
        2
    );
    assert!(!definitions.contains("#define __STDC__ "));

    let names = successful_stdout(list(
        "definitions.h",
        &["--format", "names", "--function-like", "--main-file-only"],
    ));
    assert_eq!(
        names.lines().collect::<Vec<_>>(),
        [
            "COMMENTED",
            "EMPTY_FUNCTION",
            "FUNCTION",
            "GNU_VARIADIC",
            "IN_BODY",
            "MULTILINE",
            "STRINGIFY",
            "TOKEN_PASTE",
            "VARIADIC",
        ]
    );

    let exact = successful_stdout(list(
        "definitions.h",
        &["--format", "names", "--name", "FUNCTION", "--name", "OBJECT_PARENS", "--object-like"],
    ));
    assert_eq!(exact, "OBJECT_PARENS\n");
    let exact_function =
        successful_stdout(list("definitions.h", &["--format", "names", "--name", "FUNCTION"]));
    assert_eq!(exact_function, "FUNCTION\n");
    let unknown = successful_stdout(list(
        "definitions.h",
        &["--format", "names", "--name", "DOES_NOT_EXIST"],
    ));
    assert!(unknown.is_empty());
}

#[test]
fn emits_json_with_owned_macro_tokens_locations_and_definition_history() {
    let output = successful_stdout(list(
        "definitions.h",
        &[
            "--format",
            "json",
            "--name",
            "COMMENTED",
            "--name",
            "INCLUDED_VALUE",
            "--name",
            "REDEFINED",
        ],
    ));
    let inventory: serde_json::Value = serde_json::from_str(&output).expect("CLI JSON must parse");
    assert_eq!(inventory["diagnostics"], serde_json::json!([]));
    let macros = inventory["macros"].as_array().expect("JSON inventory has a macros array");
    assert_eq!(
        macros.iter().map(|definition| definition["name"].as_str().unwrap()).collect::<Vec<_>>(),
        ["COMMENTED", "INCLUDED_VALUE", "REDEFINED", "REDEFINED"]
    );
    assert_eq!(macros[0]["kind"], "function_like");
    assert_eq!(macros[1]["kind"], "object_like");
    assert!(macros.iter().all(|definition| definition["builtin"] == false));
    let included_file =
        macros[1]["location"]["file"].as_str().expect("included macro has a filename");
    assert_eq!(
        Path::new(included_file).canonicalize().unwrap(),
        fixture("included.h").canonicalize().unwrap()
    );
    assert_eq!(macros[1]["location"]["line"], 1);
    assert_eq!(macros[1]["location"]["column"], 9);
    assert_eq!(macros[1]["location"]["offset"], 8);
    assert!(macros[0]["tokens"].as_array().unwrap().iter().any(|token| {
        token["kind"] == "comment" && token["spelling"] == "/* retained comment */"
    }));
    assert_eq!(macros[2]["tokens"].as_array().unwrap().last().unwrap()["spelling"], "1");
    assert_eq!(macros[3]["tokens"].as_array().unwrap().last().unwrap()["spelling"], "2");

    let builtins = successful_stdout(list(
        "definitions.h",
        &["--format", "json", "--name", "__STDC__", "--include-builtins"],
    ));
    let builtins: serde_json::Value = serde_json::from_str(&builtins).unwrap();
    assert_eq!(builtins["macros"].as_array().unwrap().len(), 1);
    assert_eq!(builtins["macros"][0]["builtin"], true);
}

#[test]
fn passes_clang_arguments_and_reports_diagnostics_without_partial_output() {
    let enabled = successful_stdout(list(
        "configured.h",
        &[
            "--format",
            "names",
            "--name",
            "ENABLED_VALUE",
            "--name",
            "DISABLED_VALUE",
            "--name",
            "COMMAND_LINE_VALUE",
            "--",
            "-DENABLE_BRANCH",
            "-DCOMMAND_LINE_VALUE=73",
        ],
    ));
    assert_eq!(enabled, "COMMAND_LINE_VALUE\nENABLED_VALUE\n");
    let undefined = successful_stdout(list(
        "configured.h",
        &[
            "--format",
            "names",
            "--name",
            "DISABLED_VALUE",
            "--",
            "-DENABLE_BRANCH",
            "-UENABLE_BRANCH",
        ],
    ));
    assert_eq!(undefined, "DISABLED_VALUE\n");

    for (argument, expected_message) in [
        ("-DTEST_ERROR", "expected macro scanner failure"),
        ("-DTEST_MISSING_INCLUDE", "pgrx_macro_fixture_missing_header.h"),
    ] {
        let output = list("diagnostics.h", &["--format", "json", "--", argument]);
        assert!(!output.status.success());
        assert!(output.stdout.is_empty(), "errors must not emit a partial inventory");
        assert!(String::from_utf8_lossy(&output.stderr).contains(expected_message));
    }
    let missing_header = list("missing.h", &[]);
    assert!(!missing_header.status.success());
    assert!(missing_header.stdout.is_empty());
    assert!(!missing_header.stderr.is_empty());

    let warned = list("diagnostics.h", &["--format", "json", "--", "-DTEST_WARNING"]);
    assert!(String::from_utf8_lossy(&warned.stderr).contains("expected macro scanner warning"));
    let warned: serde_json::Value = serde_json::from_str(&successful_stdout(warned)).unwrap();
    assert!(warned["diagnostics"].as_array().unwrap().iter().any(|diagnostic| {
        diagnostic["severity"] == "warning"
            && diagnostic["message"].as_str().unwrap().contains("expected macro scanner warning")
    }));

    let conflicting_filters = list("definitions.h", &["--function-like", "--object-like"]);
    assert!(!conflicting_filters.status.success());
    assert!(conflicting_filters.stdout.is_empty());
}
