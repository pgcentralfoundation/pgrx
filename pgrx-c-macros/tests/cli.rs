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
    include_dir: PathBuf,
}

impl TestConfig {
    fn new() -> Self {
        let nonce = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let home =
            std::env::temp_dir().join(format!("pgrx-c-macros-cli-{}-{nonce}", std::process::id()));
        std::fs::create_dir(&home).expect("isolated PGRX_HOME must be created");
        let config = Self { include_dir: home.join("include"), home };
        let wrapper = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("pgrx-pg-sys/include/pg18.h");
        let wrapper = std::fs::read_to_string(wrapper).unwrap();
        for include in wrapper.lines().filter_map(|line| {
            line.trim().strip_prefix("#include \"").and_then(|include| include.strip_suffix('"'))
        }) {
            let header = config.include_dir.join(include);
            std::fs::create_dir_all(header.parent().unwrap()).unwrap();
            std::fs::write(header, "").unwrap();
        }
        for name in ["postgres.h", "included.h", "definitions.h", "configured.h", "diagnostics.h"] {
            std::fs::copy(fixture(name), config.include_dir.join(name)).unwrap();
        }
        let late_header = config.include_dir.join("utils/varlena.h");
        assert!(late_header.exists(), "sentinel must be in a header present in pg18.h");
        std::fs::write(late_header, "#define LATE_WRAPPER_FUNCTION(value) ((value) + 99)\n")
            .unwrap();

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
            .env("PGRX_C_MACROS_TEST_INCLUDE_DIR", &self.include_dir)
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
        .arg(config.include_dir.join(header))
        .args(arguments)
        .output()
        .expect("macro discovery CLI must run")
}

fn successful_stdout(output: Output) -> String {
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    String::from_utf8(output.stdout).expect("CLI output must be UTF-8")
}

fn assert_json_provenance(definition: &serde_json::Value, file: &Path, start: u32, end: u32) {
    let provenance = &definition["provenance"];
    let filename = provenance["file"].as_str().expect("JSON provenance has a physical filename");
    assert!(Path::new(filename).is_absolute());
    assert_eq!(Path::new(filename).file_name(), file.file_name());
    assert_eq!(Path::new(filename).canonicalize().unwrap(), file.canonicalize().unwrap());
    assert_eq!(provenance["start_line"], start);
    assert_eq!(provenance["end_line"], end);
}

#[test]
fn requires_a_configured_version_and_lists_functions_from_the_complete_default_wrapper() {
    let config = TestConfig::new();
    for version in ["pg18", "18"] {
        let output =
            config.command().args(["list", version, "--format", "names"]).output().unwrap();
        assert_eq!(
            successful_stdout(output),
            "CPPFLAGS_FUNCTION\nINCLUDED_FUNCTION\nLATE_WRAPPER_FUNCTION\nPOSTGRES_FIXTURE_FUNCTION\n"
        );
    }
    let definitions = successful_stdout(config.command().args(["list", "pg18"]).output().unwrap());
    assert!(definitions.contains("#define LATE_WRAPPER_FUNCTION("));
    assert!(!definitions.contains("#define POSTGRES_FIXTURE_SERVER "));
    assert!(!definitions.contains("#define INCLUDED_VALUE "));
    assert!(!definitions.contains("#define PG_CONFIG_FIXTURE_VALUE "));

    let main_file = config
        .command()
        .args(["list", "pg18", "--format", "names", "--main-file-only"])
        .output()
        .unwrap();
    assert!(
        successful_stdout(main_file).is_empty(),
        "the canonical wrapper itself has no definitions"
    );

    let missing_version = config.command().arg("list").output().unwrap();
    assert_eq!(missing_version.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&missing_version.stderr).contains("<PG_VERSION>"));
    assert!(missing_version.stdout.is_empty());
    let help = successful_stdout(config.command().args(["list", "--help"]).output().unwrap());
    let usage = help.lines().find(|line| line.starts_with("Usage:")).expect("help has usage");
    assert!(usage.contains("<PG_VERSION> [HEADER]"), "{usage}");
    for option in ["--pg-version", "--function-like", "--object-like", "--include-builtins"] {
        assert!(!help.contains(option));
        let obsolete = config.command().args(["list", "pg18", option]).output().unwrap();
        assert_eq!(obsolete.status.code(), Some(2));
        assert!(String::from_utf8_lossy(&obsolete.stderr).contains(option));
        assert!(obsolete.stdout.is_empty());
    }
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
        .args(["list", "pg18", "--format", "names", "--name", "LATE_WRAPPER_FUNCTION"])
        .output()
        .unwrap();
    assert_eq!(successful_stdout(selected), "LATE_WRAPPER_FUNCTION\n");
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
fn lists_only_functions_from_custom_headers_with_exact_name_and_source_filters() {
    let definitions = successful_stdout(list("definitions.h", &[]));
    assert!(definitions.lines().all(|line| line.starts_with("#define ")));
    assert!(definitions.contains("#define INCLUDED_FUNCTION("));
    assert!(definitions.contains("#define FUNCTION("));
    assert!(definitions.contains("/* retained comment */"));
    assert_eq!(
        definitions.lines().filter(|line| line.starts_with("#define REDEFINED_FUNCTION(")).count(),
        2
    );
    assert!(!definitions.contains("#define OBJECT_"));
    assert!(!definitions.contains("#define INCLUDED_VALUE "));
    assert!(!definitions.contains("#define REDEFINED "));

    let names =
        successful_stdout(list("definitions.h", &["--format", "names", "--main-file-only"]));
    assert_eq!(
        names.lines().collect::<Vec<_>>(),
        [
            "COMMENTED",
            "EMPTY_FUNCTION",
            "FUNCTION",
            "GNU_VARIADIC",
            "IN_BODY",
            "MULTILINE",
            "REDEFINED_FUNCTION",
            "REDEFINED_FUNCTION",
            "STRINGIFY",
            "TOKEN_PASTE",
            "VARIADIC",
        ]
    );
    let exact = successful_stdout(list(
        "definitions.h",
        &["--format", "names", "--name", "FUNCTION", "--name", "OBJECT_PARENS"],
    ));
    assert_eq!(exact, "FUNCTION\n");
    let object =
        successful_stdout(list("definitions.h", &["--format", "names", "--name", "OBJECT_PARENS"]));
    assert!(object.is_empty());
    let unknown = successful_stdout(list(
        "definitions.h",
        &["--format", "names", "--name", "DOES_NOT_EXIST"],
    ));
    assert!(unknown.is_empty());
}

#[test]
fn emits_json_with_function_tokens_physical_provenance_and_definition_history() {
    let config = TestConfig::new();
    let output = successful_stdout(
        config
            .command()
            .current_dir(&config.include_dir)
            .args([
                "list",
                "pg18",
                "definitions.h",
                "--format",
                "json",
                "--name",
                "COMMENTED",
                "--name",
                "INCLUDED_FUNCTION",
                "--name",
                "MULTILINE",
                "--name",
                "REDEFINED_FUNCTION",
            ])
            .output()
            .unwrap(),
    );
    let inventory: serde_json::Value = serde_json::from_str(&output).expect("CLI JSON must parse");
    assert_eq!(inventory["diagnostics"], serde_json::json!([]));
    let macros = inventory["macros"].as_array().expect("JSON inventory has a macros array");
    assert_eq!(
        macros.iter().map(|definition| definition["name"].as_str().unwrap()).collect::<Vec<_>>(),
        ["COMMENTED", "INCLUDED_FUNCTION", "MULTILINE", "REDEFINED_FUNCTION", "REDEFINED_FUNCTION"]
    );
    assert!(
        macros
            .iter()
            .all(|definition| definition["kind"] == "function_like"
                && definition["builtin"] == false)
    );
    assert_json_provenance(&macros[0], &config.include_dir.join("definitions.h"), 11, 11);
    assert_json_provenance(&macros[1], &config.include_dir.join("included.h"), 2, 2);
    assert_json_provenance(&macros[2], &config.include_dir.join("definitions.h"), 8, 10);
    assert_json_provenance(&macros[3], &config.include_dir.join("definitions.h"), 27, 27);
    assert_json_provenance(&macros[4], &config.include_dir.join("definitions.h"), 29, 29);
    assert_eq!(macros[1]["location"]["line"], 2);
    assert_eq!(macros[1]["location"]["column"], 9);
    assert!(macros[0]["tokens"].as_array().unwrap().iter().any(|token| {
        token["kind"] == "comment" && token["spelling"] == "/* retained comment */"
    }));
    for (index, value) in [(3, "1"), (4, "2")] {
        assert!(
            macros[index]["tokens"]
                .as_array()
                .unwrap()
                .iter()
                .any(|token| token["kind"] == "literal" && token["spelling"] == value)
        );
    }
    let command_line = successful_stdout(list(
        "configured.h",
        &[
            "--format",
            "json",
            "--name",
            "FROM_COMMAND_LINE",
            "--",
            "-DFROM_COMMAND_LINE(value)=value",
        ],
    ));
    let command_line: serde_json::Value = serde_json::from_str(&command_line).unwrap();
    assert!(command_line["macros"].as_array().unwrap().is_empty());

    let builtin_object =
        successful_stdout(list("definitions.h", &["--format", "json", "--name", "__STDC__"]));
    let builtin_object: serde_json::Value = serde_json::from_str(&builtin_object).unwrap();
    assert!(builtin_object["macros"].as_array().unwrap().is_empty());
}

#[test]
fn excludes_external_macros_from_every_format_despite_custom_headers_and_include_options() {
    let config = TestConfig::new();
    let external = config.home.join("external");
    let sibling = config.home.join("include-extra");
    let extra_include = config.home.join("extra");
    for directory in [&external, &sibling, &extra_include] {
        std::fs::create_dir(directory).unwrap();
    }
    std::fs::write(
        config.include_dir.join("primary.h"),
        "#define PRIMARY_FUNCTION(value) ((value) + 1)\n#define PRIMARY_OBJECT 2\n",
    )
    .unwrap();
    let spoofed_filename =
        serde_json::to_string(config.include_dir.join("spoofed.h").to_str().unwrap()).unwrap();
    std::fs::write(
        external.join("outside.h"),
        format!("#line 700 {spoofed_filename}\n#define OUTSIDE_FUNCTION(value) (value)\n"),
    )
    .unwrap();
    std::fs::write(sibling.join("neighbor.h"), "#define SIBLING_FUNCTION(value) (value)\n")
        .unwrap();
    std::fs::write(extra_include.join("extra.h"), "#define EXTRA_FUNCTION(value) (value)\n")
        .unwrap();
    let wrapper = config.home.join("custom.h");
    std::fs::write(
        &wrapper,
        "#define WRAPPER_FUNCTION(value) (value)\n\
         #include \"primary.h\"\n\
         #include \"include/../external/outside.h\"\n\
         #include \"include-extra/neighbor.h\"\n\
         #include <extra.h>\n",
    )
    .unwrap();
    let extra_argument = format!("-I{}", extra_include.display());
    for format in ["names", "definitions", "json"] {
        let output = successful_stdout(
            config
                .command()
                .args(["list", "pg18"])
                .arg(&wrapper)
                .args(["--format", format, "--", &extra_argument])
                .output()
                .unwrap(),
        );
        for excluded in [
            "OUTSIDE_FUNCTION",
            "SIBLING_FUNCTION",
            "EXTRA_FUNCTION",
            "WRAPPER_FUNCTION",
            "PRIMARY_OBJECT",
        ] {
            assert!(!output.contains(excluded), "{format}: {output}");
        }
        match format {
            "names" => assert_eq!(output, "PRIMARY_FUNCTION\n"),
            "definitions" => assert!(output.starts_with("#define PRIMARY_FUNCTION(")),
            "json" => {
                let inventory: serde_json::Value = serde_json::from_str(&output).unwrap();
                assert!(inventory.get("context").is_none());
                let macros = inventory["macros"].as_array().unwrap();
                assert_eq!(macros.len(), 1);
                assert_eq!(macros[0]["name"], "PRIMARY_FUNCTION");
                assert_json_provenance(&macros[0], &config.include_dir.join("primary.h"), 1, 1);
            }
            _ => unreachable!(),
        }
    }
    for filter in ["--main-file-only", "--name"] {
        let mut command = config.command();
        command.args(["list", "pg18"]).arg(&wrapper).args(["--format", "names", filter]);
        if filter == "--name" {
            command.arg("OUTSIDE_FUNCTION");
        }
        let output = successful_stdout(command.args(["--", &extra_argument]).output().unwrap());
        assert!(output.is_empty(), "{filter}: {output}");
    }
}

#[test]
fn passes_clang_arguments_and_reports_diagnostics_without_partial_output() {
    let enabled = successful_stdout(list(
        "configured.h",
        &[
            "--format",
            "names",
            "--name",
            "ENABLED_FUNCTION",
            "--name",
            "DISABLED_FUNCTION",
            "--name",
            "COMMAND_LINE_FUNCTION",
            "--",
            "-DENABLE_BRANCH",
            "-DCOMMAND_LINE_VALUE=73",
        ],
    ));
    assert_eq!(enabled, "COMMAND_LINE_FUNCTION\nENABLED_FUNCTION\n");
    let undefined = successful_stdout(list(
        "configured.h",
        &[
            "--format",
            "names",
            "--name",
            "DISABLED_FUNCTION",
            "--",
            "-DENABLE_BRANCH",
            "-UENABLE_BRANCH",
        ],
    ));
    assert_eq!(undefined, "DISABLED_FUNCTION\n");

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
}
