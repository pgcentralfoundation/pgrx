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

        // Stand in for the `pgrx-pg-sys` build script: record what it is told about PostgreSQL,
        // export what a bundle reads, and check what a bundle hands back.
        fs::write(
            root.join("build.rs"),
            r###"
const EXPORT: &str = "PGRX_PG_SYS_EXTRA_TARGET_INFO_PATH";
const IMPORT: &str = "PGRX_TARGET_INFO_PATH_PG18";

fn main() {
    // Like the real script, run again when the program, the bundle, or a described value that
    // the build reads changes.
    for name in [
        "PGRX_PG_CONFIG_PATH",
        "PGRX_PG_CONFIG_AS_ENV",
        "PGRX_PG_CONFIG_VERSION",
        "PGRX_PG_CONFIG_INCLUDEDIR-SERVER",
        "PGRX_PG_CONFIG_CPPFLAGS",
        "PGRX_PG_CONFIG_CFLAGS",
        EXPORT,
        IMPORT,
    ] {
        println!("cargo:rerun-if-env-changed={name}");
    }
    let mut seen: Vec<_> = std::env::vars()
        .filter(|(name, _)| {
            ["PGRX_PG_CONFIG_", "PGRX_INCLUDEDIR", "PGRX_PKGINCLUDEDIR"]
                .iter()
                .any(|prefix| name.starts_with(prefix))
                || name.contains("TARGET_INFO")
        })
        .collect();
    seen.sort();
    let seen: Vec<_> = seen.iter().map(|(name, value)| format!("{name}={value}\n")).collect();
    std::fs::write(std::env::var_os("PGRX_TEST_BUILD_LOG").unwrap(), seen.concat()).unwrap();

    if let Some(directory) = std::env::var_os(EXPORT) {
        let directory = std::path::PathBuf::from(directory);
        let target = std::env::var("TARGET").unwrap();
        let version = std::env::var("CARGO_PKG_VERSION").unwrap();
        let manifest = format!(
            r##"{{"domain": {{"major": 18, "rust_target": "{target}", "generator_version": "{version}"}}}}"##
        );
        std::fs::write(directory.join("pg18_target_artifact.json"), manifest).unwrap();
        std::fs::write(directory.join("generated.rs"), "// generated").unwrap();
    }
    if let Some(directory) = std::env::var_os(IMPORT) {
        let generated = std::path::Path::new(&directory).join("generated.rs");
        assert_eq!(std::fs::read_to_string(generated).unwrap(), "// generated");
    }
}
"###,
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
        // Only an MSVC bundle asks, for the import library.
        Some("--libdir") => println!("{}", std::env::var("PGRX_TEST_PG_LIBDIR").unwrap()),
        // The collection script asks for everything at once.
        None => {
            println!("INCLUDEDIR-SERVER = {}", std::env::var("PGRX_TEST_PG_HEADERS").unwrap());
            println!("PKGLIBDIR = /native/pg/lib");
            println!("SHAREDIR = /native/pg/share");
            println!("CPPFLAGS = -DCOLLECTED=1");
            println!("CFLAGS = -O2 -fwrapv");
            println!("VERSION = PostgreSQL 18.7");
        }
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
        fs::create_dir(root.join("pglib")).unwrap();
        fs::write(root.join("pglib/postgres.lib"), "import library").unwrap();
        fs::create_dir(root.join("headers")).unwrap();
        fs::write(root.join("headers/pg_config.h"), "/* configured on the target */").unwrap();
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
            .env("PGRX_TEST_PG_LIBDIR", root.join("pglib"))
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
            .env("PGRX_PG_CONFIG_SHAREDIR", "/environment/pg/share")
            .env("PGRX_INCLUDEDIR_SERVER_PG18", root.join("stale-headers"))
            .env("PGRX_PKGINCLUDEDIR", root.join("stale-headers"))
            .env("PGRX_PG_SYS_EXTRA_TARGET_INFO_PATH", root.join("stale-export"))
            .env("PGRX_TARGET_INFO_PATH_PG18", root.join("stale-import"));
        command
    }

    fn assert_package(&self, prefix: &str) {
        self.assert_package_for(None, prefix);
    }

    /// The staged package of a build for `target`, or for this machine without one.
    fn assert_package_for(&self, target: Option<&str>, prefix: &str) {
        let root = self.directory.path();
        assert!(!root.join("pgrx-home/config.toml").exists());
        let package = root
            .join("target")
            .join(target.unwrap_or_default())
            .join("debug/package_environment_test-pg18");
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

    /// The PostgreSQL and bundle variables the crate's build script was given, sorted by name.
    fn build_saw(&self) -> Vec<(String, String)> {
        let seen = fs::read_to_string(self.directory.path().join("build-saw")).unwrap();
        seen.lines()
            .map(|line| line.split_once('=').unwrap())
            .map(|(name, value)| (name.to_owned(), value.to_owned()))
            .collect()
    }

    /// The build must be told to use this fixture's `pg_config` program. The metadata left in
    /// the environment may still be visible to it, but not the switch that makes it count.
    fn assert_build_used_program(&self) {
        let seen = self.build_saw();
        let program = seen.iter().find(|(name, _)| name == "PGRX_PG_CONFIG_PATH");
        let (_, program) = program.unwrap_or_else(|| panic!("no program in {seen:?}"));
        // A search of `PATH` on Windows can spell the extension differently.
        assert_eq!(
            fs::canonicalize(program).unwrap(),
            fs::canonicalize(&self.pg_config).unwrap(),
            "{seen:?}"
        );
        assert!(seen.iter().all(|(name, _)| name != "PGRX_PG_CONFIG_AS_ENV"), "{seen:?}");
        // Bundle variables left in the environment do not reach an ordinary build.
        assert!(seen.iter().all(|(name, _)| !name.contains("TARGET_INFO")), "{seen:?}");
    }

    fn assert_no_pg_config_calls(&self) {
        assert!(!self.directory.path().join("pg_config-called").exists());
    }
}

/// Header overrides left in the environment, which would replace the PostgreSQL's headers,
/// never reach a build that makes or uses a bundle.
fn assert_no_header_overrides(seen: &[(String, String)]) {
    let overridden =
        |name: &str| ["PGRX_INCLUDEDIR", "PGRX_PKGINCLUDEDIR"].iter().any(|p| name.starts_with(p));
    assert!(seen.iter().all(|(name, _)| !overridden(name)), "{seen:?}");
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
        // Header overrides serve plain `cargo` builds, and a build for this machine keeps them.
        let seen = fixture.build_saw();
        assert!(seen.iter().any(|(name, _)| name == "PGRX_INCLUDEDIR_SERVER_PG18"), "{seen:?}");
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

fn host_triple() -> String {
    let output = Command::new(std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into()))
        .arg("-vV")
        .output()
        .unwrap();
    let version = String::from_utf8(output.stdout).unwrap();
    version.lines().find_map(|line| line.strip_prefix("host: ")).unwrap().trim().to_owned()
}

/// `--bundle-generate` writes one file for this platform, PostgreSQL and pgrx version, and
/// neither compiles the extension nor stages it.
/// `--bundle` then packages from that file alone: it names the target and describes its
/// PostgreSQL, and no `pg_config` program runs.
#[test]
fn a_generated_bundle_is_all_that_packaging_needs() {
    let fixture = Fixture::new();
    let root = fixture.directory.path();
    let host = host_triple();

    let output = fixture.command().arg("--bundle-generate").output().unwrap();
    assert!(output.status.success(), "{}", combined_output(&output));
    let bundle = root.join("target").join(format!("pgrx-0.1.0-pg18-{host}.tar.gz"));
    assert!(bundle.is_file(), "{}", combined_output(&output));
    assert!(!root.join("target/debug/package_environment_test-pg18").exists());
    let library = format!(
        "{}package_environment_test{}",
        std::env::consts::DLL_PREFIX,
        std::env::consts::DLL_SUFFIX
    );
    assert!(!root.join("target/debug").join(library).exists(), "the extension was compiled");
    let seen = fixture.build_saw();
    assert_no_header_overrides(&seen);
    let export = seen.iter().find(|(name, _)| name == "PGRX_PG_SYS_EXTRA_TARGET_INFO_PATH");
    let (_, export) = export.unwrap_or_else(|| panic!("no export in {seen:?}"));
    assert!(!export.contains("stale-export"), "{seen:?}");
    assert!(seen.iter().all(|(name, _)| !name.starts_with("PGRX_TARGET_INFO_PATH")), "{seen:?}");

    // The bundle can be written somewhere else.
    let output = fixture
        .command()
        .args(["--bundle-generate", "--out-dir"])
        .arg(root.join("bundles"))
        .output()
        .unwrap();
    assert!(output.status.success(), "{}", combined_output(&output));
    assert!(root.join("bundles").join(format!("pgrx-0.1.0-pg18-{host}.tar.gz")).is_file());

    fs::remove_file(root.join("pg_config-called")).unwrap();
    let output = fixture.command().arg("--bundle").arg(&bundle).output().unwrap();
    assert!(output.status.success(), "{}", combined_output(&output));
    fixture.assert_no_pg_config_calls();
    // The layout is the generating PostgreSQL's, under the bundle's target.
    fixture.assert_package_for(Some(&host), "native");
    let seen = fixture.build_saw();
    assert_no_header_overrides(&seen);
    let value = |name: &str| {
        let found = seen.iter().find(|(seen, _)| seen == name);
        found.unwrap_or_else(|| panic!("no {name} in {seen:?}")).1.as_str()
    };
    assert_eq!(value("PGRX_PG_CONFIG_AS_ENV"), "true");
    assert_eq!(value("PGRX_PG_CONFIG_VERSION"), "PostgreSQL 18.7");
    assert_eq!(value("PGRX_PG_CONFIG_PKGLIBDIR"), "/native/pg/lib");
    assert_eq!(value("PGRX_PG_CONFIG_SHAREDIR"), "/native/pg/share");
    assert!(std::path::Path::new(value("PGRX_PG_CONFIG_LIBDIR")).is_dir());
    assert!(
        std::path::Path::new(value("PGRX_TARGET_INFO_PATH_PG18")).join("generated.rs").is_file()
    );
    // Nothing else reaches the build: no program, and no export.
    assert_eq!(seen.len(), 6, "{seen:?}");
    if host.ends_with("-msvc") {
        let library = std::path::Path::new(value("PGRX_PG_CONFIG_LIBDIR")).join("postgres.lib");
        assert_eq!(fs::read_to_string(library).unwrap(), "import library");
    }
}

/// A platform with PostgreSQL's headers but no Rust or Clang collects a partial bundle with a
/// shell script. `--bundle` then gives the build those headers and `pg_config`'s values to
/// generate from, and still runs no `pg_config` program.
///
/// The script takes those values from a `pg_config` program, from a file of what one prints,
/// or one at a time, and checks none of them: packaging from the bundle does.
#[cfg(unix)]
#[test]
fn a_collected_partial_bundle_supplies_headers_to_the_build() {
    let fixture = Fixture::new();
    let root = fixture.directory.path();
    let host = host_triple();
    let called = root.join("pg_config-called");

    // Run the script with `--name=value` arguments, and return the bundle it wrote in `out`.
    let collect = |out: &str, arguments: &[(&str, &std::ffi::OsStr)]| {
        let mut command = Command::new("sh");
        command.arg(concat!(env!("CARGO_MANIFEST_DIR"), "/../tools/pgrx-collect-bundle.sh"));
        let out = root.join(out);
        for (name, value) in arguments.iter().copied().chain([("--out-dir=", out.as_os_str())]) {
            let mut argument = std::ffi::OsString::from(name);
            argument.push(value);
            command.arg(argument);
        }
        let output = command
            .env("PGRX_TEST_PG_CONFIG_LOG", &called)
            .env("PGRX_TEST_PG_HEADERS", root.join("headers"))
            .output()
            .unwrap();
        let bundle = out.join(format!("pgrx-pg18-{host}.partial.tar.gz"));
        if output.status.success() {
            assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), bundle.to_str().unwrap());
            assert!(bundle.is_file());
        } else {
            assert!(!out.exists(), "a refused collection wrote {}", out.display());
        }
        (output, bundle)
    };
    // Package from a bundle, and return what the build was given.
    let package = |bundle: &std::path::Path| {
        let output = fixture.command().arg("--bundle").arg(bundle).output().unwrap();
        assert!(output.status.success(), "{}", combined_output(&output));
        fixture.assert_no_pg_config_calls();
        fixture.assert_package_for(Some(&host), "native");
        let seen = fixture.build_saw();
        assert_no_header_overrides(&seen);
        // The build generates for itself: nothing is imported, and no program is named.
        assert!(seen.iter().all(|(name, _)| !name.contains("TARGET_INFO")), "{seen:?}");
        assert!(seen.iter().all(|(name, _)| name != "PGRX_PG_CONFIG_PATH"), "{seen:?}");
        move |name: &str| {
            let found = seen.iter().find(|(seen, _)| seen == name);
            found.unwrap_or_else(|| panic!("no {name} in {seen:?}")).1.clone()
        }
    };
    let (target, headers) = (host.as_ref(), root.join("headers"));

    // From a `pg_config` program.
    let (output, bundle) =
        collect("program", &[("--target=", target), ("--pg-config=", fixture.pg_config.as_ref())]);
    assert!(output.status.success(), "{}", combined_output(&output));
    fs::remove_file(&called).unwrap();
    let value = package(&bundle);
    assert_eq!(value("PGRX_PG_CONFIG_AS_ENV"), "true");
    assert_eq!(value("PGRX_PG_CONFIG_VERSION"), "PostgreSQL 18.7");
    assert_eq!(value("PGRX_PG_CONFIG_CPPFLAGS"), "-DCOLLECTED=1");
    assert_eq!(value("PGRX_PG_CONFIG_CFLAGS"), "-O2 -fwrapv");
    // The build reads the collected headers, unpacked here, not a path on the target.
    let unpacked = std::path::PathBuf::from(value("PGRX_PG_CONFIG_INCLUDEDIR-SERVER"));
    assert!(!unpacked.starts_with(&headers));
    assert_eq!(
        fs::read_to_string(unpacked.join("pg_config.h")).unwrap(),
        "/* configured on the target */"
    );

    // From a file of what the target's `pg_config` printed, ending its lines as Windows does
    // and its last line not at all, with the headers somewhere else on this machine and one
    // value stated over the file's. No program runs.
    let saved = root.join("saved-pg_config.txt");
    fs::write(
        &saved,
        "INCLUDEDIR-SERVER = /on/the/target\r\nPKGLIBDIR = /native/pg/lib\r\nSHAREDIR = /native/pg/share\r\nCPPFLAGS = -DSAVED=1\r\nCFLAGS = -O1\r\nVERSION = PostgreSQL 18.7",
    )
    .unwrap();
    let (output, bundle) = collect(
        "file",
        &[
            ("--target=", target),
            ("--pg-config-file=", saved.as_ref()),
            ("--includedir-server=", headers.as_ref()),
            ("--cppflags=", "-DSTATED=1 -DNAME=\"a = b\"".as_ref()),
        ],
    );
    assert!(output.status.success(), "{}", combined_output(&output));
    assert!(!called.exists(), "a file and properties were given, so no pg_config should run");
    let value = package(&bundle);
    assert_eq!(value("PGRX_PG_CONFIG_CPPFLAGS"), "-DSTATED=1 -DNAME=\"a = b\"");
    assert_eq!(value("PGRX_PG_CONFIG_CFLAGS"), "-O1");

    // The script collects whatever it is told, and packaging says what is missing.
    let (output, bundle) = collect(
        "incomplete",
        &[
            ("--target=", target),
            ("--version=", "PostgreSQL 18.7".as_ref()),
            ("--includedir-server=", headers.as_ref()),
            ("--pkglibdir=", "/native/pg/lib".as_ref()),
        ],
    );
    assert!(output.status.success(), "{}", combined_output(&output));
    let output = fixture.command().arg("--bundle").arg(&bundle).output().unwrap();
    assert!(!output.status.success());
    let error = combined_output(&output);
    assert!(error.contains("does not say what `pg_config --sharedir` reports"), "{error}");

    // The script refuses only what stops it from collecting at all.
    for (arguments, problem) in [
        (vec![], "--target is required"),
        (vec![("--target", target)], "unknown argument: --target"),
        (vec![("--target=", target), ("--pkglibdir=", "/x".as_ref())], "nothing reports VERSION"),
        (
            vec![("--target=", target), ("--version=", "PostgreSQL 18.7".as_ref())],
            "nothing reports INCLUDEDIR-SERVER; pass --includedir-server=",
        ),
    ] {
        let (output, _) = collect("refused", &arguments);
        assert!(!output.status.success(), "{arguments:?}");
        assert!(combined_output(&output).contains(problem), "{}", combined_output(&output));
    }
}

/// Headers in a read-only tree, as in a Nix store, are collected. For an MSVC target the script
/// also leaves out the server headers nested in the public ones, which it must be able to delete.
#[cfg(unix)]
#[test]
fn read_only_headers_are_collected() {
    use std::os::unix::fs::PermissionsExt;
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path();
    let (include, lib) = (root.join("include"), root.join("lib"));
    fs::create_dir_all(include.join("server")).unwrap();
    fs::create_dir(&lib).unwrap();
    for file in [include.join("server/pg_config.h"), include.join("libpq-fe.h")] {
        fs::write(file, "").unwrap();
    }
    fs::write(lib.join("postgres.lib"), "import library").unwrap();
    let chmod = |mode: u32| {
        for directory in [include.join("server"), include.clone()] {
            fs::set_permissions(directory, fs::Permissions::from_mode(mode)).unwrap();
        }
    };
    chmod(0o555);
    let argument = |name: &str, value: &std::path::Path| {
        let mut argument = std::ffi::OsString::from(name);
        argument.push(value);
        argument
    };
    let output = Command::new("sh")
        .arg(concat!(env!("CARGO_MANIFEST_DIR"), "/../tools/pgrx-collect-bundle.sh"))
        .args(["--target=x86_64-pc-windows-msvc", "--version=PostgreSQL 18.7"])
        .arg(argument("--includedir-server=", &include.join("server")))
        .arg(argument("--pkgincludedir=", &include))
        .arg(argument("--libdir=", &lib))
        .arg(argument("--out-dir=", &root.join("out")))
        .output()
        .unwrap();
    chmod(0o755);
    assert!(output.status.success(), "{}", combined_output(&output));

    let bundle = root.join("out/pgrx-pg18-x86_64-pc-windows-msvc.partial.tar.gz");
    let mut archive =
        tar::Archive::new(flate2::read::GzDecoder::new(fs::File::open(bundle).unwrap()));
    let mut files: Vec<_> = archive
        .entries()
        .unwrap()
        .map(|entry| entry.unwrap())
        .filter(|entry| entry.header().entry_type().is_file())
        .map(|entry| entry.path().unwrap().display().to_string())
        .collect();
    files.sort();
    assert_eq!(
        files,
        [
            "include/pkg/libpq-fe.h",
            "include/server/pg_config.h",
            "lib/postgres.lib",
            "partial-bundle",
            "pg_config.txt",
            "target"
        ]
    );
}

/// With Cargo's build target set, Cargo puts the library in a target directory of its own; the
/// SQL still comes from that library, not from an older one where a plain build would be.
#[test]
fn the_schema_comes_from_the_library_cargo_built() {
    let fixture = Fixture::new();
    let output = fixture.command().env("CARGO_BUILD_TARGET", host_triple()).output().unwrap();
    assert!(output.status.success(), "{}", combined_output(&output));
    fixture.assert_package("native");
}

/// When `config.toml` names the program a package selects, the build script finds it there by
/// itself, and the build is not told it again: setting the variable in some builds and not in
/// others would make each rerun the script.
#[test]
fn a_program_in_config_toml_is_not_named_again() {
    let fixture = Fixture::new();
    let home = fixture.directory.path().join("pgrx-home");
    fs::create_dir_all(&home).unwrap();
    let program = fixture.pg_config.to_str().unwrap().replace('\\', "/");
    fs::write(home.join("config.toml"), format!("[configs]\npg18 = \"{program}\"\n")).unwrap();
    let output = fixture
        .command()
        .arg("--pg-config")
        .arg(&fixture.pg_config)
        .env_remove("PGRX_PG_CONFIG_PATH")
        .output()
        .unwrap();
    assert!(output.status.success(), "{}", combined_output(&output));
    let seen = fixture.build_saw();
    let named = ["PGRX_PG_CONFIG_PATH", "PGRX_PG_CONFIG_AS_ENV"];
    assert!(seen.iter().all(|(name, _)| !named.contains(&name.as_str())), "{seen:?}");
}

/// A bundle supplies the target and its PostgreSQL, so nothing else on the command line can.
#[test]
fn only_a_bundle_names_another_platform() {
    let fixture = Fixture::new();
    let refused = |command: &mut Command, expected: &str| {
        let output = command.output().unwrap();
        assert!(!output.status.success());
        let output = combined_output(&output);
        assert!(output.contains(expected), "{expected}: {output}");
    };
    refused(
        fixture
            .command()
            .args(["--bundle", "bundle.tar.gz", "--pg-config"])
            .arg(&fixture.pg_config),
        "cannot be used with",
    );
    refused(
        fixture.command().args(["--bundle", "bundle.tar.gz", "--bundle-generate"]),
        "cannot be used with",
    );
    refused(fixture.command().args(["--bundle", "missing.tar.gz"]), "couldn't unpack");
    // Making a bundle packages nothing, so options about the package are refused.
    refused(fixture.command().args(["--bundle-generate", "--test"]), "cannot be used with");
    refused(fixture.command().args(["--target", &host_triple()]), "--target");
    refused(&mut fixture.cargo_pgrx(&["install", "--target", &host_triple()]), "--target");
    refused(&mut fixture.cargo_pgrx(&["cross", "pgrx-target"]), "cross");
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
