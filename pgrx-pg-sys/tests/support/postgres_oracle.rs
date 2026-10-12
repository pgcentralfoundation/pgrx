//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Compile independent observations from the original PostgreSQL headers selected by this build.
//!
//! Integration tests read only the invocation facts from the fresh OUT_DIR audit report and pass
//! them unchanged to the bounded C oracle. Expected values and types come from executing C, never
//! from generated Rust, analyzed macro bodies, or the documentation-only checked-in bindings.

/// Own bounded compiler processes and temporary files for independent original-header execution.
#[path = "../../../pgrx-c-macros/tests/support/oracle.rs"]
mod oracle;

use std::path::PathBuf;

/// Read only the selected build's version and compiler invocation, ignoring large macro bodies.
#[derive(serde::Deserialize)]
struct BuildReport {
    /// Version whose original installed headers must supply the independent expectations.
    postgres_major_version: u16,
    /// Verified invocation that supplied the generated Rust under test.
    profile: OriginalProfile,
}

/// Keep the original preprocessing and target environment identical to binding generation.
#[derive(serde::Deserialize)]
struct OriginalProfile {
    /// Original all-header wrapper used to discover this build's PostgreSQL macros.
    header: PathBuf,
    /// Compiler executable selected and verified during frontend inspection.
    compiler: OriginalCompiler,
    /// Exact target and preprocessing flags recorded during this binding build.
    arguments: Vec<String>,
}

/// Identify the actual compiler rather than choosing a potentially incompatible executable.
#[derive(serde::Deserialize)]
struct OriginalCompiler {
    /// Verified Clang driver whose original headers and ABI match the generated macros.
    executable: PathBuf,
}

/// Select the version feature whose fresh macro report describes this test's compiled exports.
pub fn postgres_major_version() -> u16 {
    pgrx_pg_sys::PG_MAJORVERSION_NUM.try_into().expect("PostgreSQL major version fits in u16")
}

/// Run original C macro invocations with this build's verified profile and return exact stdout.
pub fn run(source: &str) -> String {
    let major = postgres_major_version();
    let path = PathBuf::from(env!("OUT_DIR")).join(format!("pg{major}_macro_report.json"));
    let report: BuildReport = serde_json::from_reader(std::io::BufReader::new(
        std::fs::File::open(&path).expect("open this build's C macro report"),
    ))
    .expect("read this build's original compiler profile");
    assert_eq!(report.postgres_major_version, major);
    let arguments = report.profile.arguments;
    // These codegen flags discard unused backend routines from the all-header wrapper on Linux
    // without changing the preprocessing environment or target ABI.
    #[cfg(target_os = "linux")]
    let arguments = {
        let mut arguments = arguments;
        arguments.extend(["-ffunction-sections".into(), "-fdata-sections".into()]);
        arguments
    };
    let arguments = arguments.iter().map(String::as_str).collect::<Vec<_>>();
    oracle::run_c(
        &report.profile.compiler.executable,
        &report.profile.header,
        source,
        &arguments,
        true,
    )
}
