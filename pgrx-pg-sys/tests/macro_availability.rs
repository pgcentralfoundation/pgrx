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
//! A gate that this build compiles out, through a file-level `#![cfg]` or a `#[cfg]` written
//! directly on the gate, wraps no test here and is not collected.

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
    /// Every selected candidate and its emission result.
    macros: Vec<Candidate>,
}

/// Require a report from completed inspection.
#[derive(serde::Deserialize)]
#[serde(rename_all = "snake_case")]
enum ReportStatus {
    /// Inspection ran and each candidate records its own result.
    Generated,
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
    let mut emissions = BTreeMap::new();
    for candidate in report.macros {
        let name = candidate.analysis.name;
        assert!(
            emissions.insert(name.clone(), candidate.status).is_none(),
            "PG{major} report lists {name} more than once"
        );
    }
    let gated = gated_names(&Build { major, target_os: std::env::consts::OS });
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

/// Windows CI tests PostgreSQL 15 and 17, where every instr_time gate is compiled out: their
/// tests compare Unix timing representations. Collecting them there would fail the guard
/// although no test disappears, because PostgreSQL 15's WIN32 MILLISEC and MICROSEC use
/// floating-point literals that the generator refuses. Every other configuration keeps them.
#[test]
fn gates_compiled_out_by_cfg_are_not_collected() {
    let time = ["INSTR_TIME_GET_DOUBLE", "INSTR_TIME_GET_MILLISEC", "INSTR_TIME_GET_MICROSEC"];
    for major in 15..=19 {
        for target_os in ["linux", "macos", "windows"] {
            let gated = gated_names(&Build { major, target_os });
            for name in time {
                let expected = target_os != "windows" || major == 19;
                assert_eq!(gated.contains_key(name), expected, "{name} on {target_os} PG{major}");
            }
            // generated_c_macro_tests.rs is compiled out on Windows by its file-level cfg.
            assert_eq!(
                gated.contains_key("pgstat_count_heap_scan"),
                target_os != "windows",
                "{target_os} PG{major}"
            );
        }
    }
}

/// A predicate the guard does not model fails instead of silently keeping or dropping a gate.
#[test]
#[should_panic(expected = "cannot evaluate cfg predicate")]
fn unknown_gate_cfgs_fail() {
    cfg_holds("feature = \"cshim\"", &Build { major: 18, target_os: "linux" });
}

/// A file without gates hides no test, so its cfgs need not be ones the guard understands.
#[test]
fn files_without_gates_need_no_understood_cfgs() {
    let build = Build { major: 18, target_os: "linux" };
    assert!(gates("#![cfg(unix)]\nfn helper() {}\n", &build).is_empty());
    assert!(gates("#![cfg(target_family = \"unix\")]\n", &build).is_empty());
}

/// Only the attribute lines directly above a gate apply to it. A cfg on an earlier one-line
/// item belongs to that item, so it must not compile the gate out.
#[test]
fn cfgs_of_preceding_items_do_not_apply_to_a_gate() {
    let unrelated = format!(
        "#[cfg(feature = \"pg19\")]\n#[allow(dead_code)] fn helper() {{}}\n{}",
        gate("FOO")
    );
    let direct =
        format!("#[cfg(feature = \"pg19\")]\n// Only PostgreSQL 19 has FOO.\n{}", gate("FOO"));
    for major in [18, 19] {
        let build = Build { major, target_os: "linux" };
        assert_eq!(gates(&unrelated, &build), [("FOO", true)], "PG{major}");
        assert_eq!(gates(&direct, &build), [("FOO", major == 19)], "PG{major}");
    }
}

/// Spell an availability gate for `name` as test input, without writing one in this file that
/// the guard's own scan would collect.
fn gate(name: &str) -> String {
    format!("pg::__pgrx_c_classify! {{ {} {name} {{\n}} }}\n", "@if_available")
}

/// The configuration that decides which gates compile: the `pg<major>` feature and the target
/// operating system, spelled as Rust's `target_os` cfg spells it.
struct Build {
    /// Selected PostgreSQL major version.
    major: u16,
    /// Target operating system.
    target_os: &'static str,
}

/// Map each macro name used by an availability gate that `build` compiles to the test files
/// that use it.
fn gated_names(build: &Build) -> BTreeMap<String, Vec<PathBuf>> {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let mut gated = BTreeMap::<String, Vec<PathBuf>>::new();
    for root in ["pgrx-pg-sys/tests", "pgrx-unit-tests/src/tests"] {
        let mut files = Vec::new();
        collect_sources(&workspace.join(root), &mut files);
        let mut gates_found = 0;
        for file in files {
            let source = std::fs::read_to_string(&file).expect("read gated test source");
            let relative = file.strip_prefix(workspace).unwrap().to_owned();
            for (name, compiled) in gates(&source, build) {
                gates_found += 1;
                if !compiled {
                    continue;
                }
                let users = gated.entry(name.to_owned()).or_default();
                if !users.contains(&relative) {
                    users.push(relative.clone());
                }
            }
        }
        // An empty scan would make this guard pass vacuously after tests move.
        assert_ne!(gates_found, 0, "found no availability gates below {root}");
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

/// Return each availability gate in `source` with whether `build` compiles it.
fn gates<'a>(source: &'a str, build: &Build) -> Vec<(&'a str, bool)> {
    let names = gate_names(source);
    // A file without gates hides no test, so its cfgs are not evaluated.
    let file = !names.is_empty() && file_cfgs(source).all(|predicate| cfg_holds(predicate, build));
    names
        .into_iter()
        .map(|(name, line_start)| {
            let gate =
                gate_cfgs(&source[..line_start]).all(|predicate| cfg_holds(predicate, build));
            (name, file && gate)
        })
        .collect()
}

/// Return each macro name gated by a classifier availability arm or an `if_c_macro!`
/// invocation, with the offset of the line that starts the gate.
fn gate_names(source: &str) -> Vec<(&str, usize)> {
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
                let line_start = source[..index].rfind('\n').map_or(0, |newline| newline + 1);
                names.push((&rest[..end], line_start));
            }
        }
    }
    names
}

/// Return the predicates of the `#![cfg(...)]` attributes among the comments and inner
/// attributes that open a source file.
fn file_cfgs(source: &str) -> impl Iterator<Item = &str> {
    source
        .lines()
        .map(str::trim)
        .take_while(|line| line.is_empty() || line.starts_with("//") || line.starts_with("#!["))
        .filter_map(|line| line.strip_prefix("#![cfg(")?.strip_suffix(")]"))
}

/// Return the predicates of the single-line `#[cfg(...)]` attributes directly above a gate,
/// given the source before the gate's line. The walk crosses only comments and lines that end
/// with their attribute, so a line that also holds an item ends it and that item's cfg is not
/// taken for the gate's. A cfg spelled any other way is not seen, so the gate stays collected
/// and the guard can only become stricter.
fn gate_cfgs(before: &str) -> impl Iterator<Item = &str> {
    before
        .lines()
        .rev()
        .map(str::trim)
        .take_while(|line| {
            (line.starts_with("#[") && line.ends_with(']')) || line.starts_with("//")
        })
        .filter_map(|line| line.strip_prefix("#[cfg(")?.strip_suffix(")]"))
}

/// Evaluate a `cfg` predicate for `build`. Only the predicates that these tests use are
/// understood; any other fails the test, so a new kind of exclusion cannot drop a gate unseen.
fn cfg_holds(predicate: &str, build: &Build) -> bool {
    let predicate = predicate.trim();
    if let Some(predicates) = cfg_arguments(predicate, "all") {
        return predicates.iter().all(|predicate| cfg_holds(predicate, build));
    }
    if let Some(predicates) = cfg_arguments(predicate, "any") {
        return predicates.iter().any(|predicate| cfg_holds(predicate, build));
    }
    if let Some([predicate]) = cfg_arguments(predicate, "not").as_deref() {
        return !cfg_holds(predicate, build);
    }
    let value = |key: &str| {
        let (name, value) = predicate.split_once('=')?;
        (name.trim() == key).then(|| value.trim().strip_prefix('"')?.strip_suffix('"')).flatten()
    };
    if let Some(target_os) = value("target_os") {
        return target_os == build.target_os;
    }
    if let Some(major) =
        value("feature").and_then(|feature| feature.strip_prefix("pg")?.parse::<u16>().ok())
    {
        return build.major == major;
    }
    match predicate {
        // This guard compiles only without docsrs.
        "docsrs" => false,
        _ => panic!("availability gates cannot evaluate cfg predicate {predicate:?}"),
    }
}

/// Split `name(first, second, ...)` into its top-level predicates.
fn cfg_arguments<'a>(predicate: &'a str, name: &str) -> Option<Vec<&'a str>> {
    let inner = predicate.strip_prefix(name)?.trim_start().strip_prefix('(')?.strip_suffix(')')?;
    let mut arguments = Vec::new();
    let (mut depth, mut start) = (0_usize, 0);
    for (index, character) in inner.char_indices() {
        match character {
            '(' => depth += 1,
            ')' => depth -= 1,
            ',' if depth == 0 => {
                arguments.push(inner[start..index].trim());
                start = index + 1;
            }
            _ => {}
        }
    }
    arguments.push(inner[start..].trim());
    arguments.retain(|argument| !argument.is_empty());
    Some(arguments)
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
