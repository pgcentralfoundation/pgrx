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

        // Record what a build script is told about PostgreSQL, as `pgrx-pg-sys`'s would be.
        fs::write(
            root.join("build.rs"),
            r#"
fn main() {
    println!("cargo:rerun-if-env-changed=PGRX_PG_CONFIG_PATH");
    println!("cargo:rerun-if-env-changed=PGRX_PG_CONFIG_AS_ENV");
    let seen = ["PGRX_PG_CONFIG_PATH", "PGRX_PG_CONFIG_AS_ENV"].map(|name| std::env::var(name).ok());
    std::fs::write(std::env::var_os("PGRX_TEST_BUILD_LOG").unwrap(), format!("{seen:?}")).unwrap();
}
"#,
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
        self.cargo_pgrx(&["package", "--debug"])
    }

    fn cargo_pgrx(&self, subcommand: &[&str]) -> Command {
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
            .arg("pgrx")
            .args(subcommand)
            .arg("--cargo=--offline")
            .env("PATH", path)
            .env("PGRX_HOME", root.join("pgrx-home"))
            .env("PG_CONFIG", root.join("missing-pg-config"))
            .env("PGRX_PG_CONFIG_PATH", root.join("missing-pgrx-pg-config"))
            .env("PGRX_TEST_PG_CONFIG_LOG", root.join("pg_config-called"))
            .env("PGRX_TEST_BUILD_LOG", root.join("build-saw"))
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

    /// The build must be told to use this fixture's `pg_config` program, not the environment.
    fn assert_build_used_program(&self) {
        let seen = fs::read_to_string(self.directory.path().join("build-saw")).unwrap();
        let expected = [Some(self.pg_config.to_str().unwrap().to_owned()), None];
        assert_eq!(seen, format!("{expected:?}"));
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
    let build_saw = fs::read_to_string(fixture.directory.path().join("build-saw")).unwrap();
    assert!(build_saw.ends_with(r#"Some("true")]"#), "{build_saw}");
}

#[test]
fn explicit_pg_config_overrides_environment_metadata() {
    let fixture = Fixture::new();
    let output = fixture.command().arg("--pg-config").arg(&fixture.pg_config).output().unwrap();
    assert!(output.status.success(), "{}", combined_output(&output));
    assert!(combined_output(&output).contains(fixture.pg_config.to_str().unwrap()));
    fixture.assert_package("native");
    assert!(fixture.directory.path().join("pg_config-called").is_file());
    fixture.assert_build_used_program();
}

/// With no `--pg-config` and no environment metadata, the first `pg_config` on `PATH` chooses
/// the layout, and the build is told to use that same program.
#[test]
fn path_pg_config_reaches_the_build() {
    let fixture = Fixture::new();
    let output = fixture.command().env_remove("PGRX_PG_CONFIG_AS_ENV").output().unwrap();
    assert!(output.status.success(), "{}", combined_output(&output));
    fixture.assert_package("native");
    fixture.assert_build_used_program();
}

/// A `pg_config` program describes this machine, so another target needs environment metadata.
#[test]
fn cross_target_requires_environment_metadata() {
    let fixture = Fixture::new();
    let output = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()))
        .arg("-vV")
        .output()
        .unwrap();
    let version = String::from_utf8(output.stdout).unwrap();
    let host = version.lines().find_map(|line| line.strip_prefix("host: ")).unwrap().trim();
    let cross = if host == "x86_64-unknown-linux-gnu" {
        "aarch64-unknown-linux-gnu"
    } else {
        "x86_64-unknown-linux-gnu"
    };

    let output = fixture
        .command()
        .args(["--target", cross, "--pg-config"])
        .arg(&fixture.pg_config)
        .output()
        .unwrap();
    assert!(!output.status.success());
    let output = combined_output(&output);
    assert!(output.contains("--pg-config names a program that describes this machine"), "{output}");

    let output = fixture
        .command()
        .args(["--target", cross])
        .env_remove("PGRX_PG_CONFIG_AS_ENV")
        .output()
        .unwrap();
    assert!(!output.status.success());
    let output = combined_output(&output);
    assert!(output.contains("set PGRX_PG_CONFIG_AS_ENV=true"), "{output}");
    fixture.assert_no_pg_config_calls();

    // Naming this machine's own triple is not a cross build.
    let output = fixture
        .command()
        .args(["--target", host, "--pg-config"])
        .arg(&fixture.pg_config)
        .output()
        .unwrap();
    assert!(output.status.success(), "{}", combined_output(&output));
    fixture.assert_build_used_program();
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
        fixture.assert_build_used_program();
    }
}

/// `install` chooses its PostgreSQL as `package` does, and needs no `config.toml` to do it.
#[test]
fn install_uses_environment_metadata() {
    let fixture = Fixture::new();
    let installed = fixture.directory.path().join("installed");
    let output = fixture
        .cargo_pgrx(&["install"])
        .env("PGRX_PG_CONFIG_PKGLIBDIR", installed.join("lib"))
        .env("PGRX_PG_CONFIG_SHAREDIR", installed.join("share"))
        .output()
        .unwrap();
    assert!(output.status.success(), "{}", combined_output(&output));
    assert!(
        installed
            .join(format!("lib/package_environment_test{}", std::env::consts::DLL_SUFFIX))
            .is_file()
    );
    assert!(installed.join("share/extension/package_environment_test.control").is_file());
    assert!(installed.join("share/extension/package_environment_test--0.1.0.sql").is_file());
    fixture.assert_no_pg_config_calls();
}

/// `install --sudo` stages through the package command, so it follows the same order:
/// `--pg-config`, then environment metadata, then the `pg_config` on `PATH`.
#[cfg(unix)]
#[test]
fn sudo_install_selects_postgres_as_package_does() {
    use std::os::unix::fs::PermissionsExt;
    for (explicit_pg_config, environment, prefix) in
        [(false, true, "environment"), (true, true, "native"), (false, false, "native")]
    {
        let fixture = Fixture::new();
        let root = fixture.directory.path();
        // Record each copy instead of running it as root.
        let sudo = root.join("bin/sudo");
        fs::write(&sudo, "#!/bin/sh\nprintf '%s\\n' \"$*\" >> \"$PGRX_TEST_SUDO_LOG\"\n").unwrap();
        fs::set_permissions(&sudo, fs::Permissions::from_mode(0o755)).unwrap();
        let log = root.join("sudo-called");
        let mut command = fixture.cargo_pgrx(&["install", "--sudo"]);
        command.env("PGRX_TEST_SUDO_LOG", &log);
        if explicit_pg_config {
            command.arg("--pg-config").arg(&fixture.pg_config);
        }
        if !environment {
            command.env_remove("PGRX_PG_CONFIG_AS_ENV");
        }
        let output = command.output().unwrap();
        assert!(output.status.success(), "{}", combined_output(&output));
        assert_eq!(root.join("pg_config-called").is_file(), prefix == "native");
        if prefix == "native" {
            fixture.assert_build_used_program();
        }
        let copies = fs::read_to_string(&log).unwrap();
        let mut destinations: Vec<_> =
            copies.lines().map(|copy| copy.rsplit(' ').next().unwrap()).collect();
        destinations.sort_unstable();
        assert_eq!(
            destinations,
            [
                format!(
                    "/{prefix}/pg/lib/package_environment_test{}",
                    std::env::consts::DLL_SUFFIX
                ),
                format!("/{prefix}/pg/share/extension/package_environment_test--0.1.0.sql"),
                format!("/{prefix}/pg/share/extension/package_environment_test.control"),
            ],
            "{copies}"
        );
    }
}
