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
    let mut seen: Vec<_> =
        std::env::vars().filter(|(name, _)| name.starts_with("PGRX_PG_CONFIG_")).collect();
    seen.sort();
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
        // Start from this test's own metadata variables, without changing the test process's
        // environment. cargo-pgrx must not read the ones set below: they stand for values left
        // in a shell.
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

    /// The build must be told to use this fixture's `pg_config` program. The metadata left in
    /// the environment may still be visible to it, but not the switch that makes it count.
    fn assert_build_used_program(&self) {
        let seen = fs::read_to_string(self.directory.path().join("build-saw")).unwrap();
        let program = ("PGRX_PG_CONFIG_PATH", self.pg_config.to_str().unwrap());
        assert!(seen.contains(&format!("{program:?}")), "{seen}");
        assert!(!seen.contains("PGRX_PG_CONFIG_AS_ENV"), "{seen}");
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

/// A build for this machine uses a `pg_config` program, the first on `PATH` here, whether or
/// not environment metadata is set, and the build is told to use that same program.
#[test]
fn host_package_ignores_environment_metadata() {
    for environment in [true, false] {
        let fixture = Fixture::new();
        let mut command = fixture.command();
        if !environment {
            command.env_remove("PGRX_PG_CONFIG_AS_ENV");
        }
        let output = command.output().unwrap();
        assert!(output.status.success(), "{}", combined_output(&output));
        fixture.assert_package("native");
        fixture.assert_build_used_program();
    }
}

#[test]
fn explicit_pg_config_chooses_layout_and_build() {
    let fixture = Fixture::new();
    let output = fixture.command().arg("--pg-config").arg(&fixture.pg_config).output().unwrap();
    assert!(output.status.success(), "{}", combined_output(&output));
    assert!(combined_output(&output).contains(fixture.pg_config.to_str().unwrap()));
    fixture.assert_package("native");
    fixture.assert_build_used_program();
}

/// The values that describe the PostgreSQL of another target in these tests.
const DESCRIBED: [(&str, &str); 6] = [
    ("version", "PostgreSQL 18.7"),
    ("includedir-server", "/described/pg/include/server"),
    ("libdir", "/described/pg/lib"),
    ("pkglibdir", "/described/pg/lib"),
    ("sharedir", "/described/pg/share"),
    ("cppflags", "-I/described/pg/include -DDESCRIBED"),
];

fn described(command: &mut Command, except: Option<&str>) {
    for (name, value) in DESCRIBED {
        if Some(name) != except {
            command.arg(format!("--pg-config-{name}")).arg(value);
        }
    }
}

fn host_triple() -> String {
    let output = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()))
        .arg("-vV")
        .output()
        .unwrap();
    let version = String::from_utf8(output.stdout).unwrap();
    version.lines().find_map(|line| line.strip_prefix("host: ")).unwrap().trim().to_owned()
}

/// A `pg_config` program describes this machine, so another target needs the description.
#[test]
fn cross_target_requires_described_postgres() {
    let fixture = Fixture::new();
    let host = host_triple();
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

    // Neither `PATH` nor the metadata left in the environment stands in for the description.
    let output = fixture.command().args(["--target", cross]).output().unwrap();
    assert!(!output.status.success());
    let output = combined_output(&output);
    assert!(output.contains("pass --pg-config-version, --pg-config-includedir-server"), "{output}");

    for (missing, _) in DESCRIBED {
        let mut command = fixture.command();
        described(command.args(["--target", cross]), Some(missing));
        let output = command.output().unwrap();
        assert!(!output.status.success());
        let output = combined_output(&output);
        assert!(output.contains(&format!("pass --pg-config-{missing}")), "{output}");
        assert!(!output.contains("Compiling"), "{output}");
    }

    // A complete description reaches the build script, which runs on this machine even when
    // the target's standard library is not installed for the rest of the build. Nothing of
    // the metadata left in the environment comes with it.
    let mut command = fixture.command();
    described(command.args(["--target", cross]), None);
    let output = command.output().unwrap();
    let seen = fs::read_to_string(fixture.directory.path().join("build-saw"))
        .unwrap_or_else(|error| panic!("{error}: {}", combined_output(&output)));
    let mut expected: Vec<_> = DESCRIBED
        .iter()
        .map(|(name, value)| {
            (format!("PGRX_PG_CONFIG_{}", name.to_uppercase()), (*value).to_owned())
        })
        .chain([("PGRX_PG_CONFIG_AS_ENV".to_owned(), "true".to_owned())])
        .collect();
    expected.sort();
    assert_eq!(seen, format!("{expected:?}"));
    fixture.assert_no_pg_config_calls();
}

/// Giving any `--pg-config-*` option means describing PostgreSQL in full, on this machine too.
#[test]
fn described_postgres_is_all_or_nothing() {
    let fixture = Fixture::new();
    let output = fixture.command().args(["--pg-config-cflags", "-O2 -fwrapv"]).output().unwrap();
    assert!(!output.status.success());
    let output = combined_output(&output);
    assert!(
        output.contains("so all of them are needed; also pass --pg-config-version"),
        "{output}"
    );

    let mut command = fixture.command();
    described(command.arg("--pg-config").arg(&fixture.pg_config), None);
    let output = command.output().unwrap();
    assert!(!output.status.success());
    assert!(
        combined_output(&output).contains("cannot be used with"),
        "{}",
        combined_output(&output)
    );

    let mut command = fixture.command();
    described(&mut command, None);
    let output = command.output().unwrap();
    assert!(output.status.success(), "{}", combined_output(&output));
    assert!(combined_output(&output).contains("--pg-config-*"), "{}", combined_output(&output));
    fixture.assert_package("described");
    fixture.assert_no_pg_config_calls();
}

/// Only `package` builds for another target.
#[test]
fn install_has_no_target_option() {
    let fixture = Fixture::new();
    let output = fixture.cargo_pgrx(&["install", "--target", &host_triple()]).output().unwrap();
    assert!(!output.status.success());
    assert!(combined_output(&output).contains("--target"), "{}", combined_output(&output));
    fixture.assert_no_pg_config_calls();
}

/// `install --sudo` stages through the package command, so it chooses PostgreSQL the same way.
/// Environment metadata left set in the shell cannot redirect an install on this machine.
#[cfg(unix)]
#[test]
fn sudo_install_ignores_environment_metadata() {
    use std::os::unix::fs::PermissionsExt;
    for (explicit_pg_config, environment) in
        [(false, true), (true, true), (false, false), (true, false)]
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
        fixture.assert_build_used_program();
        let copies = fs::read_to_string(&log).unwrap();
        let mut destinations: Vec<_> =
            copies.lines().map(|copy| copy.rsplit(' ').next().unwrap()).collect();
        destinations.sort_unstable();
        assert_eq!(
            destinations,
            [
                format!("/native/pg/lib/package_environment_test{}", std::env::consts::DLL_SUFFIX),
                "/native/pg/share/extension/package_environment_test--0.1.0.sql".to_owned(),
                "/native/pg/share/extension/package_environment_test.control".to_owned(),
            ],
            "{copies}"
        );
    }
}
