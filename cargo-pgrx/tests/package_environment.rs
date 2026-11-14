//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

use pgrx_sql_entity_graph::section::schema_section_sentinel_entry;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};
use tempfile::TempDir;

struct Fixture {
    directory: TempDir,
    pg_config: PathBuf,
}

impl Fixture {
    fn new() -> Self {
        let directory = tempfile::tempdir().unwrap();
        let root = directory.path();
        fs::create_dir(root.join("src")).unwrap();
        fs::create_dir(root.join("bin")).unwrap();
        fs::create_dir(root.join("pgrx-home")).unwrap();
        fs::write(
            root.join("Cargo.toml"),
            r#"
[package]
name = "package_environment_test"
version = "0.1.0"
edition = "2024"

[workspace]

[lib]
crate-type = ["cdylib"]

[features]
default = ["pg15", "keep-default"]
pg15 = []
pg18 = []
keep-default = []
"#,
        )
        .unwrap();
        fs::write(
            root.join("package_environment_test.control"),
            "comment = 'environment package regression'\ndefault_version = '@CARGO_VERSION@'\nmodule_pathname = '$libdir/package_environment_test'\nrelocatable = false\nsuperuser = false\n",
        )
        .unwrap();
        let sentinel = schema_section_sentinel_entry();
        fs::write(
            root.join("src/lib.rs"),
            format!(
                r#"
#[cfg(feature = "pg15")]
compile_error!("the default PostgreSQL feature must be replaced");
#[cfg(not(feature = "pg18"))]
compile_error!("the metadata must select PostgreSQL 18");
#[cfg(not(feature = "keep-default"))]
compile_error!("non-PostgreSQL default features must be retained");

#[used]
#[unsafe(no_mangle)]
#[cfg_attr(target_os = "macos", unsafe(link_section = "__DATA,__pgrxsc"))]
#[cfg_attr(not(target_os = "macos"), unsafe(link_section = ".pgrxsc"))]
pub static PGRX_SCHEMA: [u8; {len}] = {sentinel:?};
"#,
                len = sentinel.len(),
            ),
        )
        .unwrap();

        // This host executable tests native selection and records accidental environment fallbacks.
        // It never runs PostgreSQL or reads an installed PostgreSQL configuration.
        let pg_config = root.join("bin").join(format!("pg_config{}", std::env::consts::EXE_SUFFIX));
        let source = root.join("pg_config.rs");
        fs::write(
            &source,
            r#"
fn main() {
    std::fs::write(std::env::var_os("PGRX_TEST_PG_CONFIG_LOG").unwrap(), b"called").unwrap();
    match std::env::args().nth(1).as_deref() {
        Some("--version") => println!("PostgreSQL 18.7"),
        Some("--pkglibdir") => println!("/native/pg/lib"),
        Some("--sharedir") => println!("/native/pg/share"),
        option => panic!("unexpected pg_config query: {option:?}"),
    }
}
"#,
        )
        .unwrap();
        let output = Command::new(std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into()))
            .arg("--edition=2024")
            .arg(&source)
            .arg("-o")
            .arg(&pg_config)
            .output()
            .unwrap();
        assert!(output.status.success(), "{}", combined_output(&output));
        Self { directory, pg_config }
    }

    fn command(&self) -> Command {
        let root = self.directory.path();
        let mut command = Command::new(env!("CARGO_BIN_EXE_cargo-pgrx"));
        // Limit environment metadata to this test, without changing the test process's environment.
        for (name, _) in std::env::vars_os() {
            if name.to_string_lossy().starts_with("PGRX_PG_CONFIG_") {
                command.env_remove(name);
            }
        }
        let path = std::env::join_paths(
            std::iter::once(root.join("bin"))
                .chain(std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default())),
        )
        .unwrap();
        command
            .current_dir(root)
            .args(["pgrx", "package", "--debug", "--cargo=--offline"])
            .env("PATH", path)
            .env("PGRX_HOME", root.join("pgrx-home"))
            .env("PG_CONFIG", root.join("missing-pg-config"))
            .env("PGRX_PG_CONFIG_PATH", root.join("missing-pgrx-pg-config"))
            .env("PGRX_TEST_PG_CONFIG_LOG", root.join("pg_config-called"))
            .env("CARGO_TARGET_DIR", root.join("target"))
            .env_remove("CARGO_BUILD_TARGET")
            .env_remove("CARGO_BUILD_TARGET_DIR")
            .env_remove("PGRX_BUILD_FLAGS")
            .env_remove("RUSTFLAGS")
            .env_remove("CARGO_ENCODED_RUSTFLAGS")
            .env("PGRX_PG_CONFIG_AS_ENV", "true")
            .env("PGRX_PG_CONFIG_VERSION", "PostgreSQL 18.7")
            .env("PGRX_PG_CONFIG_PKGLIBDIR", "/environment/pg/lib")
            .env("PGRX_PG_CONFIG_SHAREDIR", "/environment/pg/share");
        command
    }

    fn assert_package(&self, prefix: &str) {
        let root = self.directory.path();
        assert!(!root.join("pgrx-home/config.toml").exists());
        let package = root.join("target/debug/package_environment_test-pg18");
        let (libdir, extdir) = if cfg!(windows) {
            (package.join("lib"), package.join("share/extension"))
        } else {
            (package.join(prefix).join("pg/lib"), package.join(prefix).join("pg/share/extension"))
        };
        assert!(
            libdir
                .join(format!("package_environment_test{}", std::env::consts::DLL_SUFFIX))
                .is_file()
        );
        let control = fs::read_to_string(extdir.join("package_environment_test.control")).unwrap();
        assert!(control.contains("default_version = '0.1.0'"), "{control}");
        let schema =
            fs::read_to_string(extdir.join("package_environment_test--0.1.0.sql")).unwrap();
        assert!(schema.contains("This file is auto generated by pgrx."), "{schema}");
    }

    fn assert_no_pg_config_calls(&self) {
        assert!(!self.directory.path().join("pg_config-called").exists());
    }
}

fn combined_output(output: &Output) -> String {
    format!(
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

#[test]
fn package_uses_environment_metadata_without_managed_postgres() {
    let fixture = Fixture::new();
    let output = fixture.command().output().unwrap();
    assert!(output.status.success(), "{}", combined_output(&output));
    assert!(combined_output(&output).contains("PGRX_PG_CONFIG_*"), "{}", combined_output(&output));
    fixture.assert_package("environment");
    fixture.assert_no_pg_config_calls();
}

#[test]
fn package_rejects_environment_metadata_with_explicit_pg_config() {
    let fixture = Fixture::new();
    let output = fixture.command().arg("--pg-config").arg(&fixture.pg_config).output().unwrap();
    assert!(!output.status.success());
    let output = combined_output(&output);
    assert!(
        output.contains("cannot combine --pg-config with PGRX_PG_CONFIG_AS_ENV=true"),
        "{output}"
    );
    fixture.assert_no_pg_config_calls();
}

#[test]
fn incomplete_environment_metadata_never_falls_back_to_pg_config() {
    let fixture = Fixture::new();
    for (property, option) in
        [("VERSION", "--version"), ("PKGLIBDIR", "--pkglibdir"), ("SHAREDIR", "--sharedir")]
    {
        let output =
            fixture.command().env_remove(format!("PGRX_PG_CONFIG_{property}")).output().unwrap();
        assert!(!output.status.success());
        let output = combined_output(&output);
        assert!(output.contains(&format!("has no known property named {option}")), "{output}");
        fixture.assert_no_pg_config_calls();
    }
}

#[test]
fn disabled_environment_metadata_preserves_explicit_pg_config() {
    let fixture = Fixture::new();
    for enabled in [None, Some("false")] {
        let mut command = fixture.command();
        command
            .arg("--pg-config")
            .arg(&fixture.pg_config)
            .env("PGRX_PG_CONFIG_VERSION", "PostgreSQL 15.1");
        match enabled {
            Some(value) => {
                command.env("PGRX_PG_CONFIG_AS_ENV", value);
            }
            None => {
                command.env_remove("PGRX_PG_CONFIG_AS_ENV");
            }
        }
        let output = command.output().unwrap();
        assert!(output.status.success(), "{}", combined_output(&output));
        assert!(combined_output(&output).contains(fixture.pg_config.to_str().unwrap()));
        fixture.assert_package("native");
        assert!(fixture.directory.path().join("pg_config-called").is_file());
    }
}
