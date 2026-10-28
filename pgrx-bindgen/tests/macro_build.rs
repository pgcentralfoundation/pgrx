//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Exercise the integrated C macro binding build: invalidation, refusals and cross builds.
//!
//! Isolated consumers use synthetic PostgreSQL configuration and headers, so changes to
//! flags, optional includes, include-path shadowing and target metadata can be observed
//! without touching a developer's installation. The tests check both emitted behavior and
//! modification times: identical builds must reuse Cargo's result and preserve unchanged
//! generated files. Ordinary bindings retain their own argv; historical CFLAGS disagreements
//! must refuse generated macro callers. A complete target bundle also links original native
//! wrappers without inspecting unavailable headers.
//!
//! Generation is required. A missing or incompatible Clang, a signed-overflow policy runtime
//! support does not model, or a server header Clang rejects fails the build, and the error
//! names the cause. A `PGRX_INCLUDEDIR_SERVER` override owns the macros its headers define.
//! A cross build for a target whose standard library is installed accepts only server headers
//! configured for that target, compiles native support for it, and applies only PostgreSQL's
//! functional CFLAGS from a host `pg_config`.

#![cfg(unix)]

use std::collections::BTreeMap;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

/// Allocate unique isolated consumer paths without sharing Cargo outputs between tests.
static NEXT: AtomicU64 = AtomicU64::new(0);

/// Own an isolated consumer or header tree whose generated artifacts can be inspected without
/// mutating the repository.
struct Fixture(
    /// Owned fixture path used for isolated inputs and cleanup.
    PathBuf,
);

/// Construct and inspect owned fixtures without changing repository snapshots or sharing
/// consumer build artifacts.
impl Fixture {
    /// Reserve owned fixture storage with canonical paths so extra include directories match
    /// PgConfig's canonical server root and bindgen's file allowlist.
    fn new() -> Self {
        let nonce = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let path = std::env::temp_dir().join(format!(
            "pgrx-macro-build-{}-{nonce}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(fs::canonicalize(&path).expect("canonicalize the reserved fixture directory"))
    }

    /// Return the isolated consumer checkout, keeping generated inputs separate from the
    /// repository.
    fn source(&self) -> PathBuf {
        self.0.join("source")
    }

    /// Run an offline Cargo subcommand on the consumer with an isolated PGRX_HOME and target
    /// output, and without inherited pgrx or bindgen settings.
    fn cargo(&self, subcommand: &str) -> Command {
        let mut command = Command::new("cargo");
        for (name, _) in std::env::vars_os() {
            if name.to_str().is_some_and(|name| {
                name.starts_with("PGRX_") || name.starts_with("BINDGEN_EXTRA_CLANG_ARGS")
            }) {
                command.env_remove(name);
            }
        }
        command
            .args([subcommand, "--offline", "--quiet", "--manifest-path"])
            .arg(self.source().join("Cargo.toml"))
            .args(["--features", "pg18"])
            .env("CARGO_TARGET_DIR", self.0.join("target"))
            .env("PGRX_HOME", self.0.join("pgrx-home"))
            .env_remove("DOCS_RS")
            .env_remove("PGRX_PG_SYS_GENERATE_BINDINGS_FOR_RELEASE");
        command
    }

    /// Configure an offline consumer run with isolated headers, PGRX_HOME, target output, and
    /// exact flag overrides.
    fn command(&self, flags: &str, env_flags: &str) -> Command {
        let mut command = self.cargo("run");
        command
            .env("PGRX_PG_CONFIG_AS_ENV", "true")
            .env("PGRX_PG_CONFIG_VERSION", "PostgreSQL 18.4")
            .env("PGRX_PG_CONFIG_INCLUDEDIR-SERVER", self.source().join("server"))
            .env("PGRX_PG_CONFIG_CPPFLAGS", "")
            .env("PGRX_PG_CONFIG_CFLAGS", flags)
            .env("PGRX_PG_CONFIG_CONFIGURE", "")
            .env("PGRX_PG_CONFIG_LIBDIR", self.source().join("server"))
            .env("BINDGEN_EXTRA_CLANG_ARGS", env_flags);
        command
    }

    /// Run the isolated binding consumer with one selected pair of recorded and environment
    /// compiler flags.
    fn build(&self, flags: &str, env_flags: &str) -> Output {
        self.command(flags, env_flags).output().expect("run the isolated binding build")
    }

    /// Locate the actual generated PG18 module index in the isolated consumer's OUT_DIR.
    fn macros(&self) -> PathBuf {
        self.out_dir_in("debug").join("cmacros/pg18/mod.rs")
    }

    /// Resolve the consumer's host build output root from its discovered macro index.
    fn out_dir(&self) -> PathBuf {
        self.out_dir_in("debug")
    }

    /// Find the build-script OUT_DIR holding the PG18 macro tree under a Cargo profile
    /// directory, such as `debug` or `<target>/debug`.
    fn out_dir_in(&self, profile: &str) -> PathBuf {
        fs::read_dir(self.0.join("target").join(profile).join("build"))
            .unwrap()
            .filter_map(Result::ok)
            .map(|entry| entry.path().join("out"))
            .find(|out| out.join("cmacros/pg18/mod.rs").exists())
            .expect("macro output must exist")
    }

    /// Read the current build's original-C facts and explicit per-root emission refusals.
    fn report(&self) -> serde_json::Value {
        report_in(&self.out_dir())
    }

    /// Write the consumer: a pgrx-bindgen build script, a wrapper including `server.h` through
    /// the selected server include directory, and a library exposing the generated macros.
    /// Each test supplies its own server headers and `main.rs`.
    fn consumer(&self) {
        let source = self.source();
        for directory in [source.join("src"), source.join("include"), self.0.join("pgrx-home")] {
            fs::create_dir_all(directory).unwrap();
        }
        let bindgen = Path::new(env!("CARGO_MANIFEST_DIR"));
        let macros = bindgen.parent().unwrap().join("pgrx-macros");
        let support = bindgen.parent().unwrap().join("pgrx-pg-sys/src/c_macros/support.rs");
        fs::write(source.join("Cargo.toml"), format!(
            "[package]\nname = \"macro-build-consumer\"\nversion = \"0.0.0\"\nedition = \"2024\"\n[workspace]\n[features]\npg18 = []\n[dependencies]\npgrx-macros = {{ path = {macros:?} }}\n[build-dependencies]\npgrx-bindgen = {{ path = {bindgen:?} }}\n"
        )).unwrap();
        fs::write(source.join("build.rs"), "fn main() { pgrx_bindgen::build::main().unwrap(); }\n")
            .unwrap();
        fs::write(source.join("pgrx-cshim.c"), "").unwrap();
        fs::write(source.join("include/pg18.h"), "#include <server.h>\n").unwrap();
        fs::write(
            source.join("src/lib.rs"),
            format!(
                r#"
#[derive(Clone, Copy)] pub struct Oid(u32);
impl Oid {{ pub fn to_u32(self) -> u32 {{ self.0 }} pub fn from_u32(value:u32)->Self {{ Self(value) }} }}
#[derive(Clone, Copy)] pub struct TransactionId(u32);
impl TransactionId {{ pub fn into_inner(self) -> u32 {{ self.0 }} pub fn from_inner(value:u32)->Self {{ Self(value) }} }}
pub type MultiXactId = TransactionId;
pub struct Datum;
pub trait PgNode {{}}
/// This fixture calls only stateless C integer functions, which cannot raise
/// PostgreSQL errors, invoke callbacks, or access backend state.
pub mod ffi {{
    /// Execute this fixture's pure native function; no backend guard is needed.
    ///
    /// # Safety
    /// The closure must call only the fixture's initialized scalar C operations.
    pub unsafe fn pg_guard_ffi_boundary<T, F: FnOnce() -> T>(f: F) -> T {{ f() }}
}}
#[path = {support:?}] pub mod __pgrx_c_macros;
mod pg18 {{
    include!(concat!(env!("OUT_DIR"), "/pg18.rs"));
}}
pub use pg18::*;
/// Expose the same ordinary binding namespace used by production macro support.
pub mod __pgrx_c_bindings {{
    pub use crate::pg18::*;
    pub use crate::{{Datum, MultiXactId, Oid, PgNode, TransactionId}};
}}
pub mod cmacros {{
    include!(concat!(env!("OUT_DIR"), "/cmacros/pg18/mod.rs"));
}}
pub use cmacros::*;
"#
            ),
        )
        .unwrap();
    }

    /// Write the server tree whose `server.h` defines `FROM_HEADER` from its fallback `CHOICE`,
    /// an optional header and a `FLAG` default, with a native inline function and metadata.
    fn macro_server(&self) {
        let source = self.source();
        fs::create_dir_all(source.join("server/fallback")).unwrap();
        fs::write(source.join("server/metadata.h"), include_str!("fixtures/metadata.h")).unwrap();
        fs::write(
            source.join("server/server.h"),
            r#"
typedef unsigned int Oid;
typedef unsigned int TransactionId;
#include "metadata.h"
#include <choice.h>
#if __has_include("optional.h")
#include "optional.h"
#else
#define OPTIONAL 0
#endif
#ifndef FLAG
#define FLAG 0
#endif
#define FROM_HEADER(x) ((x) + CHOICE + OPTIONAL + FLAG)
static inline int native_from_header(int value) { return FROM_HEADER(value); }
#define FROM_HEADER_NATIVE(value) native_from_header(value)
"#,
        )
        .unwrap();
        // Bindgen allowlists files under the server root. Keep both choices there, while
        // separating the default from the direct server include path so shadowing can win.
        fs::write(source.join("server/fallback/choice.h"), "#define CHOICE 10\n").unwrap();
        fs::write(source.join("src/main.rs"), MACRO_MAIN).unwrap();
    }
}

/// Call the generated macro, the original inline function through bindgen and through its
/// macro, and the native metadata, then print the macro's value.
const MACRO_MAIN: &str = r#"use macro_build_consumer as renamed;
fn main() {
    let expected = renamed::FROM_HEADER!(1_i32).get();
    // SAFETY: The original native inline function accepts and returns only C
    // int values, with no backend state, callbacks, pointer access, or errors.
    unsafe {
        assert_eq!(renamed::native_from_header(1), expected);
        assert_eq!(renamed::FROM_HEADER_NATIVE!(1_i32).get(), expected);
    }
    assert_eq!(renamed::__pgrx_module_magic_data().version, 37);
    let info = renamed::__pgrx_function_info_v1();
    assert_eq!(info.api_version, 23);
    assert!(core::ptr::eq(info, renamed::__pgrx_function_info_v1()));
    println!("{expected}");
}
"#;

/// Read a build's original-C facts and per-root emission results from its OUT_DIR.
fn report_in(out_dir: &Path) -> serde_json::Value {
    serde_json::from_slice(&fs::read(out_dir.join("pg18_macro_report.json")).unwrap()).unwrap()
}

/// Record generated Rust leaf modification times to detect unnecessary rewrites during an
/// unchanged build.
fn macro_timestamps(directory: &Path) -> BTreeMap<PathBuf, SystemTime> {
    let mut result = BTreeMap::new();
    let mut directories = vec![directory.to_path_buf()];
    while let Some(current) = directories.pop() {
        for entry in fs::read_dir(current).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let kind = entry.file_type().unwrap();
            if kind.is_dir() {
                directories.push(path);
            } else if kind.is_file() && path.extension().is_some_and(|extension| extension == "rs")
            {
                let name = path.strip_prefix(directory).unwrap().to_path_buf();
                assert!(
                    result.insert(name, fs::metadata(path).unwrap().modified().unwrap()).is_none()
                );
            }
        }
    }
    result
}

/// Release only temporary artifacts owned by this fixture, including on failed compiler or
/// assertion paths.
impl Drop for Fixture {
    /// Remove only this fixture's owned temporary storage after the test or oracle completes.
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// Require the isolated binding build to succeed before using its stdout as observed macro
/// behavior.
#[track_caller]
fn successful(output: Output) -> String {
    assert!(
        output.status.success(),
        "isolated binding build failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

/// Require an actual bindgen/C constant disagreement to refuse its macro caller with the exact
/// independently observed values, rather than accepting a failed tool invocation as coverage.
fn assert_constant_mismatch(
    fixture: &Fixture,
    output: Output,
    constant: &str,
    binding: i64,
    original: i64,
) {
    let report = fixture.report();
    let macro_report = report["macros"].as_array().and_then(|macros| {
        macros.iter().find(|candidate| candidate["analysis"]["name"] == "FROM_HEADER")
    });
    // Capture the actual published binding and analysis before asserting refusal, so an
    // unexpected successful consumer exposes whether the catalogs or the translation disagree.
    // Diagnostic text is bounded independently of report and generated-file size.
    let generated = fs::read_to_string(fixture.out_dir().join("pg18.rs"))
        .unwrap_or_else(|error| format!("cannot read generated pg18.rs: {error}"));
    let binding_lines = generated
        .lines()
        .filter(|line| {
            line.contains("pub const CHOICE:") || line.contains(&format!("pub const {constant}:"))
        })
        .take(4)
        .map(|line| line.chars().take(512).collect::<String>())
        .collect::<Vec<_>>();
    let macro_diagnostic = macro_report
        .map(|candidate| candidate["analysis"].to_string())
        .unwrap_or_else(|| "FROM_HEADER is absent from the report".into())
        .chars()
        .take(16 * 1024)
        .collect::<String>();
    let status_diagnostic = macro_report
        .map(|candidate| {
            format!(
                "status={}, reason={}",
                candidate["status"]["status"], candidate["status"]["reason"]
            )
        })
        .unwrap_or_else(|| "FROM_HEADER has no reported status".into())
        .chars()
        .take(4 * 1024)
        .collect::<String>();
    let stdout = String::from_utf8_lossy(&output.stdout).chars().take(4096).collect::<String>();
    assert!(
        !output.status.success(),
        "historical CFLAGS alone must refuse {constant} with bindgen={binding}, clang={original}; consumer stdout={stdout:?}\ngenerated bindings={binding_lines:?}\nFROM_HEADER status={status_diagnostic}\nFROM_HEADER analysis (first 16 KiB characters)={macro_diagnostic}"
    );
    assert_eq!(report["status"], "generated");
    let macro_report =
        macro_report.expect("the original macro caller remains in the generated report");
    assert_eq!(macro_report["status"]["status"], "skipped");
    let reason = &macro_report["status"]["reason"];
    assert_eq!(reason["code"], "binding_value_mismatch");
    assert!(
        reason["message"]
            .as_str()
            .unwrap()
            .contains(&format!("{constant}'s value; bindgen={binding}, clang={original}")),
        "unexpected constant refusal: {reason}"
    );
}

/// Track historical flags, shared header changes and bundle imports.
#[test]
#[ignore = "compiles an isolated binding build repeatedly; requires native Clang and cached dependencies"]
fn generation_tracks_flags_optional_headers_shadowing_and_unchanged_builds() {
    let fixture = Fixture::new();
    let source = fixture.source();
    fixture.consumer();
    fixture.macro_server();
    let flags = "-fwrapv -fsigned-char";
    let shadow_flags = format!("-I{}", source.join("server/shadow").display());
    let fallback_flags = format!("-I{}", source.join("server/fallback").display());
    let include_flags = format!("{shadow_flags} {fallback_flags}");
    assert_eq!(successful(fixture.build(flags, &include_flags)), "11\n");
    let choice = fs::read_to_string(fixture.out_dir().join("pg18.rs"))
        .unwrap()
        .lines()
        .find(|line| line.contains("pub const CHOICE:"))
        .map(|line| line.trim().to_owned());
    assert_eq!(
        choice.as_deref(),
        Some("pub const CHOICE: u32 = 10;"),
        "the first build must retain bindgen's CHOICE storage instead of supplementing the C profile's i32 constant"
    );
    let macros = fixture.macros();
    let modified = macro_timestamps(macros.parent().unwrap());
    let build_output = fixture.out_dir().parent().unwrap().join("output");
    let first_build = fs::metadata(&build_output).unwrap().modified().unwrap();
    assert_eq!(successful(fixture.build(flags, &include_flags)), "11\n");
    assert_eq!(
        macro_timestamps(macros.parent().unwrap()),
        modified,
        "an unchanged build must not rewrite generated macros"
    );
    assert_eq!(
        fs::metadata(&build_output).unwrap().modified().unwrap(),
        first_build,
        "an unchanged build must reuse the build-script result rather than regenerate unchanged bytes"
    );
    let flags = format!("{flags} -O2");
    assert_eq!(successful(fixture.build(&flags, &include_flags)), "11\n");
    let historical_build = fs::metadata(&build_output).unwrap().modified().unwrap();
    assert_ne!(historical_build, first_build, "historical CFLAGS changes must rerun generation");
    assert!(
        fixture.report()["profile"]["arguments"]
            .as_array()
            .unwrap()
            .iter()
            .any(|argument| argument == "-O2")
    );
    fs::write(source.join("server/optional.h"), "#define OPTIONAL 100\n").unwrap();
    assert_eq!(successful(fixture.build(&flags, &include_flags)), "111\n");
    assert_ne!(
        fs::metadata(&build_output).unwrap().modified().unwrap(),
        historical_build,
        "creating an optional header must rerun the build script"
    );
    fs::create_dir(source.join("server/shadow")).unwrap();
    fs::write(source.join("server/shadow/choice.h"), "#define CHOICE 20\n").unwrap();
    assert_constant_mismatch(
        &fixture,
        fixture.build(&format!("{flags} {shadow_flags}"), &fallback_flags),
        "CHOICE",
        10,
        20,
    );
    assert_eq!(successful(fixture.build(&flags, &include_flags)), "121\n");
    let defined_flags = format!("{flags} -DFLAG=1000");
    assert_constant_mismatch(
        &fixture,
        fixture.build(&defined_flags, &include_flags),
        "FLAG",
        0,
        1000,
    );
    assert_eq!(
        successful(fixture.build(&defined_flags, &format!("{include_flags} -DFLAG=1000"))),
        "1121\n"
    );
    let override_flags = format!("{include_flags} -UFLAG -DFLAG=2000");
    assert_eq!(
        successful(fixture.build(&defined_flags, &override_flags)),
        "2121\n",
        "shared bindgen environment overrides must apply once after historical flags"
    );
    let report = fixture.report();
    let arguments = report["profile"]["arguments"].as_array().unwrap();
    assert_eq!(arguments.iter().filter(|argument| *argument == "-DFLAG=2000").count(), 1);
    assert!(
        arguments.iter().position(|argument| argument == "-DFLAG=1000").unwrap()
            < arguments.iter().position(|argument| argument == "-UFLAG").unwrap()
    );
    let complete = fixture.0.join("complete-target-info");
    assert_eq!(
        successful(
            fixture
                .command(&defined_flags, &override_flags)
                .env("PGRX_PG_SYS_EXTRA_TARGET_INFO_PATH", &complete)
                .output()
                .unwrap()
        ),
        "2121\n"
    );
    for name in [
        "pg18_target_artifact.json",
        "pg18_raw_bindings.rs",
        "pg18_macro_report.json",
        "cmacros/pg18/mod.rs",
        "pgrx_c_macros_pg18.c",
        "libpgrx_c_macros_pg18.a",
    ] {
        assert!(complete.join(name).is_file(), "complete target bundle contains {name}");
    }
    let exported: BTreeMap<_, _> = macro_timestamps(macros.parent().unwrap());
    fs::rename(source.join("server"), source.join("server-unavailable")).unwrap();
    assert_eq!(
        successful(
            fixture
                .command("-include /deliberately/unavailable/target-header.h", "")
                .env("PGRX_TARGET_INFO_PATH_PG18", &complete)
                .env("CLANG_PATH", "/deliberately/unavailable/clang")
                .env("CARGO_TARGET_DIR", fixture.0.join("import-target"))
                .output()
                .unwrap()
        ),
        "2121\n",
        "a complete bundle supplies original native metadata and inline wrappers without Clang"
    );
    assert_eq!(
        macro_timestamps(macros.parent().unwrap()),
        exported,
        "an isolated import cannot rewrite the exporting build's generated tree"
    );
    fs::rename(source.join("server-unavailable"), source.join("server")).unwrap();
    let precomputed = macro_timestamps(macros.parent().unwrap());
    let target_info = fixture.0.join("target-info");
    fs::create_dir(&target_info).unwrap();
    fs::write(target_info.join("pg18_raw_bindings.rs"), "pub const PRECOMPUTED: i32 = 7;\n")
        .unwrap();
    fs::write(source.join("src/main.rs"), "fn main() { println!(\"precomputed\"); }\n").unwrap();
    let output = fixture
        .command("-include /deliberately/unavailable/target-header.h", "")
        .env("PGRX_TARGET_INFO_PATH_PG18", &target_info)
        .output()
        .unwrap();
    assert!(!output.status.success(), "raw-only bindings cannot satisfy macro callers");
    assert!(String::from_utf8_lossy(&output.stderr).contains("complete target bundle"));
    assert_eq!(
        macro_timestamps(macros.parent().unwrap()),
        precomputed,
        "failed raw-only imports cannot replace the previously published macro availability tree"
    );
}

/// Write an executable shell script.
fn script(path: &Path, body: &str) {
    fs::write(path, format!("#!/bin/sh\n{body}\n")).unwrap();
    fs::set_permissions(path, fs::Permissions::from_mode(0o755)).unwrap();
}

/// Require a failed build whose diagnostics contain every expected fragment.
#[track_caller]
fn refused(output: Output, expected: &[&str]) -> String {
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    assert!(!output.status.success(), "the build must fail:\n{stderr}");
    for fragment in expected {
        assert!(stderr.contains(fragment), "stderr lacks {fragment:?}:\n{stderr}");
    }
    stderr
}

/// Every generation failure stops the build and names its cause: a missing or incompatible
/// Clang, a signed-overflow policy runtime support does not model, and a server header that
/// Clang rejects. No native support is published by a refused build.
#[test]
#[ignore = "compiles an isolated binding build repeatedly; requires native Clang and cached dependencies"]
fn refused_generation_fails_and_names_the_cause() {
    let fixture = Fixture::new();
    let source = fixture.source();
    fixture.consumer();
    fixture.macro_server();
    let flags = "-fwrapv";
    let include_flags = format!("-I{}", source.join("server/fallback").display());
    let requirement = "pg18: pgrx-pg-sys requires generated C macros for ";
    refused(
        fixture
            .command(flags, &include_flags)
            .env("CLANG_PATH", "/deliberately/unavailable/clang")
            .output()
            .unwrap(),
        &[
            requirement,
            "could not find a compatible Clang executable: /deliberately/unavailable/clang",
        ],
    );
    let incompatible = fixture.0.join("clang");
    script(&incompatible, "echo 'clang version 1.0.0'");
    refused(
        fixture.command(flags, &include_flags).env("CLANG_PATH", &incompatible).output().unwrap(),
        &[
            requirement,
            &format!("{} is clang version 1.0.0; the loaded library is", incompatible.display()),
        ],
    );
    refused(
        fixture.command("-ftrapv", &include_flags).output().unwrap(),
        &[requirement, "runtime support does not yet model trapping signed overflow"],
    );
    fs::write(source.join("server/broken.h"), "int broken = ;\n").unwrap();
    let server = fs::read_to_string(source.join("server/server.h")).unwrap();
    fs::write(source.join("server/server.h"), format!("#include \"broken.h\"\n{server}")).unwrap();
    refused(
        fixture.command(flags, &include_flags).output().unwrap(),
        &[
            requirement,
            &format!("{}:1:14: expected expression", source.join("server/broken.h").display()),
        ],
    );
    let mut directories = vec![fixture.0.join("target")];
    while let Some(directory) = directories.pop() {
        for entry in fs::read_dir(directory).unwrap() {
            let path = entry.unwrap().path();
            assert!(
                !path.ends_with("libpgrx_c_macros_pg18.a"),
                "a refused build published {}",
                path.display()
            );
            if path.is_dir() {
                directories.push(path);
            }
        }
    }
}

/// A `PGRX_INCLUDEDIR_SERVER` override owns the macros its headers define, even though the
/// installation's own server directory is an empty decoy.
#[test]
#[ignore = "compiles an isolated binding build; requires native Clang and cached dependencies"]
fn include_server_override_owns_macros() {
    let fixture = Fixture::new();
    let source = fixture.source();
    fixture.consumer();
    fixture.macro_server();
    let decoy = source.join("decoy");
    fs::create_dir(&decoy).unwrap();
    let output = fixture
        .command("-fwrapv", &format!("-I{}", source.join("server/fallback").display()))
        .env("PGRX_PG_CONFIG_INCLUDEDIR-SERVER", &decoy)
        .env("PGRX_INCLUDEDIR_SERVER", source.join("server"))
        .output()
        .unwrap();
    assert_eq!(successful(output), "11\n");
    let from_header = fixture.report()["macros"]
        .as_array()
        .unwrap()
        .iter()
        .find(|candidate| candidate["analysis"]["name"] == "FROM_HEADER")
        .cloned()
        .expect("the override's macro is a generation candidate");
    assert_eq!(from_header["status"]["status"], "emitted", "{from_header}");
}

/// A Cargo target other than the host, with its standard library installed, and the
/// `PG_VERSION_STR` machines and host-only CFLAGS used to describe both.
struct Cross {
    /// Cargo's target.
    target: &'static str,
    /// The architecture that begins Clang's triple for the target.
    clang_arch: &'static str,
    /// The target's machine as configure records it.
    target_machine: &'static str,
    /// The host's machine as configure records it.
    host_machine: &'static str,
    /// A host packaging flag that does not apply to the target.
    host_only_flag: &'static str,
    /// The target's object-file machine: Mach-O `cputype` or ELF `e_machine`.
    object_machine: u32,
}

/// Choose the cross target for this host and require its standard library.
fn cross() -> Cross {
    const MACHO_X86_64: u32 = 0x0100_0007;
    const MACHO_ARM64: u32 = 0x0100_000c;
    const ELF_X86_64: u32 = 62;
    const ELF_AARCH64: u32 = 183;
    let cross = match (std::env::consts::ARCH, std::env::consts::OS) {
        ("aarch64", "macos") => Cross {
            target: "x86_64-apple-darwin",
            clang_arch: "x86_64",
            target_machine: "x86_64-apple-darwin24.0.0",
            host_machine: "aarch64-apple-darwin24.0.0",
            host_only_flag: "-mbranch-protection=standard",
            object_machine: MACHO_X86_64,
        },
        ("x86_64", "macos") => Cross {
            target: "aarch64-apple-darwin",
            clang_arch: "arm64",
            target_machine: "aarch64-apple-darwin24.0.0",
            host_machine: "x86_64-apple-darwin24.0.0",
            host_only_flag: "-fcf-protection",
            object_machine: MACHO_ARM64,
        },
        ("x86_64", "linux") => Cross {
            target: "aarch64-unknown-linux-gnu",
            clang_arch: "aarch64",
            target_machine: "aarch64-unknown-linux-gnu",
            host_machine: "x86_64-pc-linux-gnu",
            host_only_flag: "-fcf-protection",
            object_machine: ELF_AARCH64,
        },
        ("aarch64", "linux") => Cross {
            target: "x86_64-unknown-linux-gnu",
            clang_arch: "x86_64",
            target_machine: "x86_64-pc-linux-gnu",
            host_machine: "aarch64-unknown-linux-gnu",
            host_only_flag: "-mbranch-protection=standard",
            object_machine: ELF_X86_64,
        },
        (arch, os) => panic!("macro_build.rs names no cross target for {arch}-{os} hosts"),
    };
    let libdir = Command::new("rustc")
        .args(["--print", "target-libdir", "--target", cross.target])
        .output()
        .expect("run rustc");
    let libdir = PathBuf::from(String::from_utf8(libdir.stdout).unwrap().trim());
    assert!(
        libdir.is_dir(),
        "the cross-build test needs the {0} standard library; install it with `rustup target add {0}`",
        cross.target
    );
    cross
}

/// Read the machine of a Mach-O or ELF object file.
fn object_machine(object: &Path) -> u32 {
    let bytes = fs::read(object).unwrap();
    match bytes[..4] {
        [0xcf, 0xfa, 0xed, 0xfe] => u32::from_le_bytes(bytes[4..8].try_into().unwrap()),
        [0x7f, b'E', b'L', b'F'] => u16::from_le_bytes(bytes[18..20].try_into().unwrap()).into(),
        _ => panic!("{} is neither a 64-bit Mach-O nor an ELF object", object.display()),
    }
}

/// Write a freestanding LP64 server tree whose `pg_config.h` names `machine`.
fn lp64_server(directory: &Path, machine: &str) {
    fs::create_dir_all(directory).unwrap();
    fs::write(directory.join("metadata.h"), include_str!("fixtures/metadata.h")).unwrap();
    fs::write(
        directory.join("pg_config.h"),
        format!(
            r#"#define PG_MAJORVERSION_NUM 18
#define PG_VERSION_NUM 180004
#define PG_VERSION_STR "PostgreSQL 18.4 on {machine}, compiled by clang, 64-bit"
#define SIZEOF_VOID_P 8
#define SIZEOF_SIZE_T 8
#define SIZEOF_LONG 8
#define ALIGNOF_SHORT 2
#define ALIGNOF_INT 4
#define ALIGNOF_LONG 8
#define ALIGNOF_DOUBLE 8
#define MAXIMUM_ALIGNOF 8
"#
        ),
    )
    .unwrap();
    fs::write(
        directory.join("server.h"),
        r#"
typedef unsigned int Oid;
typedef unsigned int TransactionId;
#include "pg_config.h"
#include "metadata.h"
#define FROM_HEADER(x) ((x) + PG_MAJORVERSION_NUM)
static inline int native_from_header(int value) { return FROM_HEADER(value); }
#define FROM_HEADER_NATIVE(value) native_from_header(value)
"#,
    )
    .unwrap();
}

/// A cross build with a host `pg_config` refuses the host's server headers and names the
/// target-scoped override. With target headers it compiles native support for the target and
/// applies only PostgreSQL's functional flags from the host's CFLAGS.
#[test]
#[ignore = "cross-compiles an isolated binding build; requires Clang and the cross target's standard library"]
fn cross_builds_accept_only_target_headers() {
    let cross = cross();
    let fixture = Fixture::new();
    let source = fixture.source();
    fixture.consumer();
    fs::write(
        source.join("src/main.rs"),
        r#"use macro_build_consumer as renamed;
fn main() {
    let value = renamed::FROM_HEADER!(1_i32).get();
    // SAFETY: The original native inline function accepts and returns only C
    // int values, with no backend state, callbacks, pointer access, or errors.
    let native = unsafe { renamed::FROM_HEADER_NATIVE!(1_i32).get() };
    println!("{value} {native}");
}
"#,
    )
    .unwrap();
    let host_server = source.join("host-server");
    let target_server = source.join("target-server");
    lp64_server(&host_server, cross.host_machine);
    lp64_server(&target_server, cross.target_machine);
    let pg_config = fixture.0.join("pg_config");
    script(
        &pg_config,
        &format!(
            r#"case "$1" in
    --version) echo 'PostgreSQL 18.4' ;;
    --includedir-server) echo '{}' ;;
    --libdir) echo '{}' ;;
    --cppflags|--configure) echo ;;
    --cflags) echo '{} -fwrapv -fno-strict-aliasing' ;;
    *) echo "unsupported pg_config option $1" >&2; exit 1 ;;
esac"#,
            host_server.display(),
            host_server.display(),
            cross.host_only_flag,
        ),
    );
    // Checking expands the generated macros in the binary for the target without linking it,
    // while the build script still compiles and archives native support.
    let build = || {
        let mut command = fixture.cargo("check");
        command.args(["--target", cross.target]).env("PGRX_PG_CONFIG_PATH", &pg_config);
        command
    };
    let override_variable = format!("PGRX_INCLUDEDIR_SERVER_PG18_{}", cross.target);
    refused(
        build().output().unwrap(),
        &[
            &format!("pg_config.h was configured for {} (PG_VERSION_STR)", cross.host_machine),
            &override_variable,
            "PGRX_PG_CONFIG_AS_ENV",
            "PGRX_TARGET_INFO_PATH_PG18",
        ],
    );
    let output = build().env(&override_variable, &target_server).output().unwrap();
    assert!(
        output.status.success(),
        "target headers must build:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let out_dir = fixture.out_dir_in(&format!("{}/debug", cross.target));
    let report = report_in(&out_dir);
    assert_eq!(report["cflags_recorded"], true);
    assert_eq!(report["cflags_dropped"], serde_json::json!([cross.host_only_flag]));
    assert_eq!(report["profile"]["signed_overflow"], "wrapping");
    let triple = report["profile"]["target"]["triple"].as_str().unwrap();
    assert!(triple.starts_with(cross.clang_arch), "Clang target {triple} is not {}", cross.target);
    assert_eq!(object_machine(&out_dir.join("pgrx_c_macros_pg18.o")), cross.object_machine);
}
