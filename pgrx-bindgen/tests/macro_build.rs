//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Exercise Cargo invalidation for the integrated C macro binding build.
//!
//! An isolated consumer uses synthetic PostgreSQL configuration and headers, so
//! changes to flags, optional includes, and include-path shadowing can be observed
//! without touching a developer's installation. The test checks both emitted
//! behavior and modification times: identical builds must reuse Cargo's result
//! and preserve unchanged generated files.

#![cfg(unix)]

/// Keep catalogs, output maps, and observation sets deterministic for exact selection and
/// publication comparisons.
use std::collections::BTreeMap;
/// Read original fixtures and manage only the owned inputs and outputs used by generation
/// checks.
use std::fs;
/// Keep fixture and generated-output locations explicit so consumer builds remain independent
/// of the working directory.
use std::path::{Path, PathBuf};
/// Invoke independent compilers and consumers and inspect their actual exit status rather than
/// trusting generated source alone.
use std::process::{Command, Output};
/// Allocate unique fixture paths or record process-local effects across concurrent test
/// invocations.
use std::sync::atomic::{AtomicU64, Ordering};
/// Bound compiler processes and choose isolated temporary names without reusing prior oracle
/// artifacts.
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
    /// Create owned fixture storage for synthetic headers and a separate Cargo target tree.
    fn new() -> Self {
        let nonce = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let path = std::env::temp_dir().join(format!(
            "pgrx-macro-build-{}-{nonce}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }

    /// Return the isolated consumer checkout, keeping generated inputs separate from the
    /// repository.
    fn source(&self) -> PathBuf {
        self.0.join("source")
    }

    /// Configure an offline consumer build with isolated headers, PGRX_HOME, target output, and
    /// exact flag overrides.
    fn command(&self, flags: &str, env_flags: &str) -> Command {
        let mut command = Command::new("cargo");
        for (name, _) in std::env::vars_os() {
            if name.to_str().is_some_and(|name| {
                name.starts_with("PGRX_") || name.starts_with("BINDGEN_EXTRA_CLANG_ARGS")
            }) {
                command.env_remove(name);
            }
        }
        command
            .args(["run", "--offline", "--quiet", "--manifest-path"])
            .arg(self.source().join("Cargo.toml"))
            .args(["--features", "pg18"])
            .env("CARGO_TARGET_DIR", self.0.join("target"))
            .env("PGRX_HOME", self.0.join("pgrx-home"))
            .env("PGRX_PG_CONFIG_AS_ENV", "true")
            .env("PGRX_PG_CONFIG_VERSION", "PostgreSQL 18.4")
            .env("PGRX_PG_CONFIG_INCLUDEDIR-SERVER", self.source().join("server"))
            .env("PGRX_PG_CONFIG_CPPFLAGS", "")
            .env("PGRX_PG_CONFIG_CFLAGS", flags)
            .env("PGRX_PG_CONFIG_CONFIGURE", "")
            .env("PGRX_PG_CONFIG_LIBDIR", self.source().join("server"))
            .env("BINDGEN_EXTRA_CLANG_ARGS", env_flags)
            .env_remove("DOCS_RS")
            .env_remove("PGRX_PG_SYS_GENERATE_BINDINGS_FOR_RELEASE")
            .env_remove("PGRX_PG_CONFIG_PATH");
        command
    }

    /// Run the isolated binding consumer with one selected pair of recorded and environment
    /// compiler flags.
    fn build(&self, flags: &str, env_flags: &str) -> Output {
        self.command(flags, env_flags).output().expect("run the isolated binding build")
    }

    /// Locate the actual generated PG18 module index in the isolated consumer's OUT_DIR.
    fn macros(&self) -> PathBuf {
        let root = self.0.join("target/debug/build");
        fs::read_dir(root)
            .unwrap()
            .filter_map(Result::ok)
            .map(|entry| entry.path().join("out/cmacros/pg18/mod.rs"))
            .find(|path| path.exists())
            .expect("macro output must exist")
    }

    /// Resolve the consumer's build output root from its discovered macro index.
    fn out_dir(&self) -> PathBuf {
        self.macros().ancestors().nth(3).unwrap().to_path_buf()
    }
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
fn successful(output: Output) -> String {
    assert!(
        output.status.success(),
        "isolated binding build failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

/// Checks that generation tracks flags optional headers shadowing and unchanged builds.
#[test]
#[ignore = "compiles an isolated binding build repeatedly; requires native Clang and cached dependencies"]
fn generation_tracks_flags_optional_headers_shadowing_and_unchanged_builds() {
    let fixture = Fixture::new();
    let source = fixture.source();
    for directory in [
        source.join("src"),
        source.join("include"),
        source.join("server"),
        fixture.0.join("pgrx-home"),
    ] {
        fs::create_dir_all(directory).unwrap();
    }
    let bindgen = Path::new(env!("CARGO_MANIFEST_DIR"));
    let support = bindgen.parent().unwrap().join("pgrx-pg-sys/src/c_macros/support.rs");
    fs::write(source.join("Cargo.toml"), format!(
        "[package]\nname = \"macro-build-consumer\"\nversion = \"0.0.0\"\nedition = \"2024\"\n[workspace]\n[features]\npg18 = []\n[build-dependencies]\npgrx-bindgen = {{ path = {:?} }}\n", bindgen
    )).unwrap();
    fs::write(source.join("build.rs"), "fn main() { pgrx_bindgen::build::main().unwrap(); }\n")
        .unwrap();
    fs::write(source.join("pgrx-cshim.c"), "").unwrap();
    fs::write(source.join("include/pg18.h"), "#include <server.h>\n").unwrap();
    fs::write(
        source.join("server/server.h"),
        r#"
typedef unsigned int Oid;
typedef unsigned int TransactionId;
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
"#,
    )
    .unwrap();
    fs::write(source.join("server/choice.h"), "#define CHOICE 10\n").unwrap();
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
#[path = {support:?}] pub mod __pgrx_c_macros;
mod pg18 {{
    include!(concat!(env!("OUT_DIR"), "/pg18.rs"));
}}
pub use pg18::*;
pub mod cmacros {{
    include!(concat!(env!("OUT_DIR"), "/cmacros/pg18/mod.rs"));
}}
pub use cmacros::*;
"#
        ),
    )
    .unwrap();
    fs::write(source.join("src/main.rs"), "use macro_build_consumer as renamed;\nfn main() { println!(\"{}\", renamed::FROM_HEADER!(1).get()); }\n").unwrap();
    let flags = format!("-fwrapv -fsigned-char -I{}", source.join("shadow").display());
    assert_eq!(successful(fixture.build(&flags, "")), "11\n");
    let macros = fixture.macros();
    let modified = macro_timestamps(macros.parent().unwrap());
    let build_output = fixture.out_dir().parent().unwrap().join("output");
    let first_build = fs::metadata(&build_output).unwrap().modified().unwrap();
    assert_eq!(successful(fixture.build(&flags, "")), "11\n");
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
    fs::write(source.join("server/optional.h"), "#define OPTIONAL 100\n").unwrap();
    assert_eq!(successful(fixture.build(&flags, "")), "111\n");
    assert_ne!(
        fs::metadata(&build_output).unwrap().modified().unwrap(),
        first_build,
        "creating an optional header must rerun the build script"
    );
    fs::create_dir(source.join("shadow")).unwrap();
    fs::write(source.join("shadow/choice.h"), "#define CHOICE 20\n").unwrap();
    assert_eq!(successful(fixture.build(&flags, "")), "121\n");
    assert_eq!(successful(fixture.build(&format!("{flags} -DFLAG=1000"), "")), "1121\n");
    assert_eq!(
        successful(fixture.build(&format!("{flags} -DFLAG=1000"), "-UFLAG -DFLAG=2000")),
        "2121\n",
        "bindgen environment overrides must apply once after recorded flags"
    );
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
    assert_eq!(successful(output), "precomputed\n");
    let precomputed = macro_timestamps(macros.parent().unwrap());
    assert_eq!(precomputed.len(), 1, "precomputed builds remove stale macro leaves");
    assert!(precomputed.contains_key(Path::new("mod.rs")));
    assert!(syn::parse_file(&fs::read_to_string(&macros).unwrap()).unwrap().items.is_empty());
    let report: serde_json::Value = serde_json::from_slice(
        &fs::read(fixture.out_dir().join("pg18_macro_report.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(report["status"], "unavailable");
    assert!(report.get("profile").is_none(), "precomputed bindings must not infer a host profile");
    assert!(report["reason"].as_str().unwrap().contains("precomputed target bindings"));
}
