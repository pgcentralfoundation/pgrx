//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Keep availability-gated macro tests from disappearing without a failure.
//!
//! The classifier's availability arm and `if_c_macro!` expand to nothing when this build did
//! not emit the named macro. A generator regression would therefore delete the tests they wrap
//! while `cargo test` still passes. This test collects every gated name from the pg-sys
//! integration tests and the pgrx-unit-tests backend tests, then requires this build's macro
//! report to show it as emitted unless a listed profile difference explains its absence.

#![cfg(not(docsrs))]

/// Ask this build's original compiler profile whether assertions are enabled.
#[path = "support/postgres_oracle.rs"]
mod postgres_oracle;

use std::collections::BTreeMap;
use std::ops::RangeInclusive;
use std::path::{Path, PathBuf};

/// Read only the availability facts from this build's macro report.
#[derive(serde::Deserialize)]
struct Report {
    /// Version whose installed headers produced the report.
    postgres_major_version: u16,
    /// Whether inspection ran; any other recorded status fails deserialization.
    status: ReportStatus,
    /// Recorded cause of an unavailable report.
    reason: Option<String>,
    /// Every selected candidate and its emission result; absent from unavailable reports.
    #[serde(default)]
    macros: Vec<Candidate>,
}

/// Distinguish completed inspection from a build that omitted every macro.
#[derive(serde::Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
enum ReportStatus {
    /// Inspection ran and each candidate records its own result.
    Generated,
    /// The build published no macros and recorded why.
    Unavailable,
}

/// Identify one selected macro and its emission result.
#[derive(serde::Deserialize)]
struct Candidate {
    /// Analysis facts, of which only the name is needed here.
    analysis: Analysis,
    /// Structured emission or skip result.
    status: Emission,
}

/// Name a selected macro without reading its large analysis body.
#[derive(serde::Deserialize)]
struct Analysis {
    /// Original C macro name, which is also the generated Rust macro name.
    name: String,
}

/// Distinguish emitted macros from structured skips.
#[derive(serde::Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
enum Emission {
    /// The generated Rust macro exists in this build.
    Emitted,
    /// The generator refused the macro for a recorded reason.
    Skipped {
        /// Structured refusal.
        reason: SkipReason,
    },
}

/// Machine-readable skip cause recorded by the generator.
#[derive(serde::Deserialize)]
struct SkipReason {
    /// Stable snake_case skip code.
    code: String,
}

/// A known profile difference under which a gated macro may be missing from this build.
struct Allowance {
    /// Gated macro name.
    name: &'static str,
    /// PostgreSQL major versions to which the difference applies.
    majors: RangeInclusive<u16>,
    /// Exact skip code, or `None` when the installed headers do not define the macro.
    skip: Option<&'static str>,
    /// Whether the difference exists only in assertion-enabled C profiles.
    assertions_only: bool,
}

/// Every allowed way for a gated macro to be missing. Anything else fails, including a new skip
/// code for a listed name, so each addition here must name the profile difference it accepts.
const ALLOWANCES: &[Allowance] = &[
    // macro_api.rs gates on an undefined name to check the unavailable branch.
    Allowance {
        name: "__PGRX_REVIEW_MACRO_THAT_DOES_NOT_EXIST",
        majors: 0..=u16::MAX,
        skip: None,
        assertions_only: false,
    },
    // PostgreSQL 16 introduced soft error contexts.
    Allowance { name: "SOFT_ERROR_OCCURRED", majors: 15..=15, skip: None, assertions_only: false },
    // PostgreSQL 15's timespec conversions use floating-point literals.
    Allowance {
        name: "INSTR_TIME_GET_DOUBLE",
        majors: 15..=15,
        skip: Some("unsupported_literal"),
        assertions_only: false,
    },
    Allowance {
        name: "INSTR_TIME_GET_MILLISEC",
        majors: 15..=15,
        skip: Some("unsupported_literal"),
        assertions_only: false,
    },
    // AssertMacro expands IsA's token pasting only when assertions are enabled.
    Allowance {
        name: "GetCTETargetList",
        majors: 0..=u16::MAX,
        skip: Some("dynamic_builtin"),
        assertions_only: true,
    },
];

/// Require every gated test's macro to exist in this build unless a listed difference applies.
#[test]
fn every_availability_gated_macro_is_emitted() {
    let major = postgres_oracle::postgres_major_version();
    let path = PathBuf::from(env!("OUT_DIR")).join(format!("pg{major}_macro_report.json"));
    let report: Report = serde_json::from_reader(std::io::BufReader::new(
        std::fs::File::open(&path).expect("open this build's C macro report"),
    ))
    .expect("read this build's C macro report");
    assert_eq!(report.postgres_major_version, major);
    if report.status == ReportStatus::Unavailable {
        // Only an explicit opt-out may remove every gated test.
        assert_eq!(
            report.reason.as_deref(),
            Some("disabled by PGRX_C_MACROS=0"),
            "PG{major} C macro generation was unavailable, so every gated test was removed"
        );
        return;
    }
    let mut emissions = BTreeMap::new();
    for candidate in report.macros {
        let name = candidate.analysis.name;
        assert!(
            emissions.insert(name.clone(), candidate.status).is_none(),
            "PG{major} report lists {name} more than once"
        );
    }
    let gated = gated_names();
    let mut assertions = None;
    let mut missing = Vec::new();
    for (name, files) in &gated {
        let skip = match emissions.get(name) {
            Some(Emission::Emitted) => continue,
            Some(Emission::Skipped { reason }) => Some(reason.code.as_str()),
            None => None,
        };
        let allowed = ALLOWANCES.iter().any(|allowance| {
            allowance.name == name
                && allowance.majors.contains(&major)
                && allowance.skip == skip
                && (!allowance.assertions_only
                    || *assertions.get_or_insert_with(assertions_enabled))
        });
        if !allowed {
            missing.push(format!("{name} ({}) in {files:?}", skip.unwrap_or("not in report")));
        }
    }
    assert!(
        missing.is_empty(),
        "PG{major} did not emit macros whose gated tests would silently disappear:\n{}",
        missing.join("\n")
    );
}

/// Map each macro name used by an availability gate to the test files that use it.
fn gated_names() -> BTreeMap<String, Vec<PathBuf>> {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let mut gated = BTreeMap::<String, Vec<PathBuf>>::new();
    for root in ["pgrx-pg-sys/tests", "pgrx-unit-tests/src/tests"] {
        let mut files = Vec::new();
        collect_sources(&workspace.join(root), &mut files);
        let mut gates = 0;
        for file in files {
            let source = std::fs::read_to_string(&file).expect("read gated test source");
            // These test modules never compile on Windows, so their gates remove nothing there.
            if cfg!(windows) && source.contains("#![cfg(not(target_os = \"windows\"))]") {
                continue;
            }
            let relative = file.strip_prefix(workspace).unwrap().to_owned();
            for name in gate_names(&source) {
                gates += 1;
                let users = gated.entry(name.to_owned()).or_default();
                if !users.contains(&relative) {
                    users.push(relative.clone());
                }
            }
        }
        // An empty scan would make this guard pass vacuously after tests move.
        assert_ne!(gates, 0, "found no availability gates below {root}");
    }
    gated
}

/// Recursively collect Rust sources below a test root.
fn collect_sources(directory: &Path, files: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(directory).expect("read test source directory") {
        let path = entry.expect("read test source entry").path();
        if path.is_dir() {
            collect_sources(&path, files);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            files.push(path);
        }
    }
}

/// Return the macro names gated by classifier availability arms and `if_c_macro!` invocations.
fn gate_names(source: &str) -> Vec<&str> {
    let mut names = Vec::new();
    for (marker, delimited) in [("@if_available", false), ("if_c_macro!", true)] {
        for (index, _) in source.match_indices(marker) {
            let rest = source[index + marker.len()..].trim_start();
            let rest = if delimited {
                let Some(rest) = rest.strip_prefix(['{', '(', '[']) else { continue };
                rest.trim_start()
            } else {
                rest
            };
            let end = rest
                .find(|character: char| !(character.is_ascii_alphanumeric() || character == '_'))
                .unwrap_or(rest.len());
            if end != 0 && rest[end..].trim_start().starts_with('{') {
                names.push(&rest[..end]);
            }
        }
    }
    names
}

/// Report whether this build's original C profile defines USE_ASSERT_CHECKING.
fn assertions_enabled() -> bool {
    let output = postgres_oracle::run(
        "#include <stdio.h>\n#undef printf\nint main(void) {\n#ifdef USE_ASSERT_CHECKING\n    printf(\"1\\n\");\n#else\n    printf(\"0\\n\");\n#endif\n    return 0;\n}\n",
    );
    match output.trim() {
        "1" => true,
        "0" => false,
        other => panic!("unexpected assertion probe output: {other:?}"),
    }
}
