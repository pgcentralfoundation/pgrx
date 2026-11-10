//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Compare the C macros that pgrx-pg-sys generates under different Clang versions.
//!
//! A macro that one libclang cannot analyze is skipped, and a test run under that libclang
//! cannot see a macro that isn't there. Each cell of the workflow's `clang` job therefore
//! builds pgrx-pg-sys for every PostgreSQL version and keeps a [`Summary`] of each build's
//! `pgNN_macro_report.json`: the reports are about 100 MB each, and a summary is well under
//! 1 MB. The `clang-compare` job then requires every Clang major to agree, per architecture
//! and PostgreSQL version, on which macros exist and how they are called.
//!
//! Architectures are never compared with each other, because their headers differ. Neither
//! is the Rust text of a macro, whose integer type spellings follow the compiler. Comparing
//! text would need a normalized form of it in [`Outcome::Emitted`] and [`Outcome::agrees`].

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::io::BufReader;
use std::ops::RangeInclusive;
use std::path::Path;
use std::process::{Command, Stdio};

use cargo_metadata::Message;
use cargo_metadata::camino::Utf8PathBuf;
use eyre::{WrapErr, bail, ensure, eyre};
use serde::{Deserialize, Serialize};

use crate::process::{self, log};
use crate::workspace::Workspace;

/// A difference between Clang majors that the comparison accepts: the majors in `clang` skip
/// `name` with the skip code `skip`, and every other major emits it.
///
/// Only a skip can be listed. Majors that emit a macro must always agree on how it is called.
struct Expected {
    name: &'static str,
    /// The first component of the target triple, such as `x86_64`.
    arch: &'static str,
    postgres: RangeInclusive<u16>,
    clang: RangeInclusive<u32>,
    skip: &'static str,
}

/// The accepted differences. Each entry needs a comment that says why the generator cannot
/// produce the macro under those Clang majors. An entry that stops matching fails the
/// comparison, so the list cannot outlive its reasons.
const EXPECTED: &[Expected] = &[];

/// What one build generated for each macro PostgreSQL's headers define.
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Summary {
    postgres: u16,
    /// The first component of the build's target triple.
    arch: String,
    /// The major version of the libclang and Clang driver that the build used.
    clang: u32,
    /// The libclang version as it describes itself.
    libclang: String,
    macros: BTreeMap<String, Outcome>,
}

/// What a build made of one macro.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
enum Outcome {
    Emitted {
        caller_unsafe: bool,
        /// How callers must write the invocation, such as `parenthesized_scalar_expressions`.
        invocation: String,
        /// Why the macro calls the original inline function instead of a translation of it.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        fallback: Option<Reason>,
    },
    Skipped {
        reason: Reason,
    },
}

/// Why the generator skipped a macro or fell back to calling the C function.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Reason {
    code: String,
    message: String,
}

impl Outcome {
    /// Whether two Clang majors generated the same thing: both nothing, or both a macro with
    /// the same safety, invocation contract and form. Why each one skipped or fell back may
    /// differ.
    fn agrees(left: Option<&Outcome>, right: Option<&Outcome>) -> bool {
        match (left, right) {
            (
                Some(Outcome::Emitted { caller_unsafe: unsafe_a, invocation: call_a, fallback: a }),
                Some(Outcome::Emitted { caller_unsafe: unsafe_b, invocation: call_b, fallback: b }),
            ) => unsafe_a == unsafe_b && call_a == call_b && a.is_some() == b.is_some(),
            (Some(Outcome::Emitted { .. }), _) | (_, Some(Outcome::Emitted { .. })) => false,
            _ => true,
        }
    }

    fn describe(outcome: Option<&Outcome>) -> String {
        match outcome {
            None => "not in the report".to_owned(),
            Some(Outcome::Skipped { reason }) => format!("skipped: {reason}"),
            Some(Outcome::Emitted { caller_unsafe, invocation, fallback }) => {
                let safety = if *caller_unsafe { "caller unsafe" } else { "safe" };
                let form = match fallback {
                    Some(reason) => format!("calls the C function ({reason})"),
                    None => "translated".to_owned(),
                };
                format!("emitted: {safety}, {invocation}, {form}")
            }
        }
    }
}

impl std::fmt::Display for Reason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = self.message.split_whitespace().collect::<Vec<_>>().join(" ");
        write!(f, "{}: {message}", self.code)
    }
}

/// The parts of `pgNN_macro_report.json` that a summary keeps. `MacroReport` in
/// pgrx-bindgen's build script writes the file.
#[derive(Deserialize)]
struct Report {
    postgres_major_version: u16,
    profile: Profile,
    macros: Vec<ReportMacro>,
}

#[derive(Deserialize)]
struct Profile {
    compiler: Compiler,
    target: Target,
}

#[derive(Deserialize)]
struct Compiler {
    version: String,
    libclang_version: String,
}

#[derive(Deserialize)]
struct Target {
    triple: String,
}

#[derive(Deserialize)]
struct ReportMacro {
    analysis: Analysis,
    status: Status,
    fallback: Option<Reason>,
}

#[derive(Deserialize)]
struct Analysis {
    name: String,
    invocation: String,
}

#[derive(Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
enum Status {
    Emitted { caller_unsafe: bool },
    Skipped { reason: Reason },
}

/// The major version in a string such as `Ubuntu clang version 14.0.0-1ubuntu1.1`.
fn clang_major(version: &str) -> Option<u32> {
    let (_, number) = version.split_once("clang version ")?;
    let digits = number.find(|c: char| !c.is_ascii_digit()).unwrap_or(number.len());
    number[..digits].parse().ok()
}

impl Summary {
    /// Summarize a macro report, which must come from PostgreSQL `postgres` and from a
    /// libclang and Clang driver of major version `clang`. A build loads the newest libclang
    /// it finds unless told otherwise, so a job that means to test an older one must check.
    fn from_report(json: &[u8], postgres: u16, clang: u32) -> eyre::Result<Self> {
        let report: Report = serde_json::from_slice(json).wrap_err("parsing the macro report")?;
        ensure!(
            report.postgres_major_version == postgres,
            "the report is for PostgreSQL {}, not {postgres}",
            report.postgres_major_version
        );
        let compiler = &report.profile.compiler;
        for version in [&compiler.libclang_version, &compiler.version] {
            ensure!(
                clang_major(version) == Some(clang),
                "the build used `{version}`, not Clang {clang}"
            );
        }
        let triple = &report.profile.target.triple;
        let arch = triple.split('-').next().unwrap_or(triple).to_owned();
        let mut macros = BTreeMap::new();
        for candidate in report.macros {
            let name = candidate.analysis.name;
            let outcome = match candidate.status {
                Status::Emitted { caller_unsafe } => Outcome::Emitted {
                    caller_unsafe,
                    invocation: candidate.analysis.invocation,
                    fallback: candidate.fallback,
                },
                Status::Skipped { reason } => Outcome::Skipped { reason },
            };
            ensure!(
                macros.insert(name.clone(), outcome).is_none(),
                "the report lists {name} more than once"
            );
        }
        Ok(Self { postgres, arch, clang, libclang: compiler.libclang_version.clone(), macros })
    }
}

/// One macro that the Clang majors of a group disagree about.
#[derive(Debug)]
struct Difference {
    name: String,
    postgres: u16,
    arch: String,
    /// What each Clang major generated; `None` when its report doesn't list the macro.
    outcomes: BTreeMap<u32, Option<Outcome>>,
    /// Whether [`EXPECTED`] lists it.
    expected: bool,
}

/// The result of comparing every architecture's and PostgreSQL version's summaries.
#[derive(Debug, Default)]
struct Comparison {
    /// The reports compared with each other, one line per architecture and PostgreSQL version.
    compared: Vec<String>,
    differences: Vec<Difference>,
    /// Reasons the comparison cannot pass besides a difference: reports that are missing, and
    /// expected differences that no longer occur.
    problems: Vec<String>,
}

impl Expected {
    fn covers(&self, arch: &str, postgres: u16) -> bool {
        self.arch == arch && self.postgres.contains(&postgres)
    }

    /// Whether this is the whole difference: its Clang majors skip with its code, and the
    /// others emit the same thing.
    fn matches(&self, difference: &Difference) -> bool {
        let mut emitted = difference
            .outcomes
            .iter()
            .filter(|(clang, _)| !self.clang.contains(clang))
            .map(|(_, outcome)| outcome.as_ref());
        let first = emitted.next().flatten();
        self.name == difference.name
            && self.covers(&difference.arch, difference.postgres)
            && matches!(first, Some(Outcome::Emitted { .. }))
            && emitted.all(|outcome| Outcome::agrees(first, outcome))
            && difference.outcomes.iter().filter(|(clang, _)| self.clang.contains(clang)).all(
                |(_, outcome)| {
                    matches!(outcome, Some(Outcome::Skipped { reason }) if reason.code == self.skip)
                },
            )
    }
}

/// Compare the summaries of each architecture and PostgreSQL version across Clang majors.
fn compare(summaries: &[Summary], expected: &[Expected]) -> eyre::Result<Comparison> {
    let mut groups: BTreeMap<(&str, u16), BTreeMap<u32, &Summary>> = BTreeMap::new();
    let mut clangs: BTreeMap<&str, BTreeSet<u32>> = BTreeMap::new();
    for summary in summaries {
        let group = groups.entry((&summary.arch, summary.postgres)).or_default();
        ensure!(
            group.insert(summary.clang, summary).is_none(),
            "two reports for {} PostgreSQL {} under Clang {}",
            summary.arch,
            summary.postgres,
            summary.clang
        );
        clangs.entry(&summary.arch).or_default().insert(summary.clang);
    }

    let mut comparison = Comparison::default();
    if groups.is_empty() {
        comparison.problems.push("there are no reports to compare".to_owned());
    }
    for (&(arch, postgres), group) in &groups {
        let majors: Vec<u32> = group.keys().copied().collect();
        let names: BTreeSet<&str> =
            group.values().flat_map(|summary| summary.macros.keys().map(String::as_str)).collect();
        comparison.compared.push(format!(
            "{arch}, PostgreSQL {postgres}: Clang {}, {} macros",
            ranges(&majors),
            names.len()
        ));
        let missing: Vec<u32> =
            clangs[arch].iter().copied().filter(|clang| !group.contains_key(clang)).collect();
        if !missing.is_empty() {
            comparison.problems.push(format!(
                "{arch}, PostgreSQL {postgres}: no report from Clang {}",
                ranges(&missing)
            ));
        } else if majors.len() < 2 {
            comparison.problems.push(format!(
                "{arch}, PostgreSQL {postgres}: only Clang {} reported, so nothing was compared",
                ranges(&majors)
            ));
        }

        let before = comparison.differences.len();
        for name in names {
            let outcomes: BTreeMap<u32, Option<&Outcome>> =
                group.iter().map(|(clang, summary)| (*clang, summary.macros.get(name))).collect();
            let first = outcomes.values().next().copied().flatten();
            if outcomes.values().all(|outcome| Outcome::agrees(first, *outcome)) {
                continue;
            }
            let mut difference = Difference {
                name: name.to_owned(),
                postgres,
                arch: arch.to_owned(),
                outcomes: outcomes.into_iter().map(|(clang, o)| (clang, o.cloned())).collect(),
                expected: false,
            };
            difference.expected = expected.iter().any(|entry| entry.matches(&difference));
            comparison.differences.push(difference);
        }
        // An entry is judged only where reports exist from both sides of its Clang range.
        let found = &comparison.differences[before..];
        for entry in expected.iter().filter(|entry| entry.covers(arch, postgres)) {
            let inside = majors.iter().filter(|clang| entry.clang.contains(clang)).count();
            if inside == 0 || inside == majors.len() {
                continue;
            }
            if !found.iter().any(|difference| entry.matches(difference)) {
                comparison.problems.push(format!(
                    "{arch}, PostgreSQL {postgres}: the expected difference for {} no longer \
                     occurs (Clang {}-{} skipping it with `{}`); remove it from EXPECTED",
                    entry.name,
                    entry.clang.start(),
                    entry.clang.end(),
                    entry.skip
                ));
            }
        }
    }
    Ok(comparison)
}

/// Sorted version numbers with runs joined, such as `14-16, 18`.
fn ranges(versions: &[u32]) -> String {
    let mut text = String::new();
    let mut index = 0;
    while index < versions.len() {
        let mut end = index;
        while end + 1 < versions.len() && versions[end + 1] == versions[end] + 1 {
            end += 1;
        }
        if !text.is_empty() {
            text.push_str(", ");
        }
        let _ = write!(text, "{}", versions[index]);
        if end > index {
            let _ = write!(text, "-{}", versions[end]);
        }
        index = end + 1;
    }
    text
}

impl Comparison {
    fn passed(&self) -> bool {
        self.problems.is_empty() && self.differences.iter().all(|difference| difference.expected)
    }

    /// A table of the differences, one row for each distinct result of a macro, then the
    /// problems.
    fn render(&self) -> String {
        let mut text = String::from("Compared the C macros generated under each Clang:\n");
        for line in &self.compared {
            let _ = writeln!(text, "  {line}");
        }
        for (expected, title) in [(false, "Differences"), (true, "Expected differences")] {
            let mut rows = Vec::new();
            for difference in self.differences.iter().filter(|d| d.expected == expected) {
                let mut results: BTreeMap<String, Vec<u32>> = BTreeMap::new();
                for (clang, outcome) in &difference.outcomes {
                    results.entry(Outcome::describe(outcome.as_ref())).or_default().push(*clang);
                }
                let mut results: Vec<_> = results.into_iter().collect();
                results.sort_by(|a, b| a.1.cmp(&b.1));
                for (result, majors) in results {
                    rows.push([
                        difference.name.clone(),
                        difference.postgres.to_string(),
                        difference.arch.clone(),
                        ranges(&majors),
                        result,
                    ]);
                }
            }
            if rows.is_empty() {
                continue;
            }
            let _ = writeln!(text, "\n{title}:");
            rows.insert(0, ["macro", "PG", "arch", "Clang", "result"].map(str::to_owned));
            let width = |column: usize| rows.iter().map(|row| row[column].len()).max().unwrap_or(0);
            let widths = [width(0), width(1), width(2), width(3)];
            for row in &rows {
                let _ = writeln!(
                    text,
                    "  {:<w0$}  {:<w1$}  {:<w2$}  {:<w3$}  {}",
                    row[0],
                    row[1],
                    row[2],
                    row[3],
                    row[4],
                    w0 = widths[0],
                    w1 = widths[1],
                    w2 = widths[2],
                    w3 = widths[3],
                );
            }
        }
        if !self.problems.is_empty() {
            text.push_str("\nProblems:\n");
            for problem in &self.problems {
                let _ = writeln!(text, "  {problem}");
            }
        }
        let unexpected = self.differences.iter().filter(|d| !d.expected).count();
        let _ = writeln!(
            text,
            "\nunexpected differences: {unexpected}, expected differences: {}, problems: {}",
            self.differences.len() - unexpected,
            self.problems.len()
        );
        text
    }
}

/// Build pgrx-pg-sys for one PostgreSQL version and return the build script's output
/// directory, or `None` when the build fails.
fn build(workspace: &Workspace, postgres: u16) -> eyre::Result<Option<Utf8PathBuf>> {
    let mut command = Command::new("cargo");
    command.args([
        "build",
        "--locked",
        "--package",
        "pgrx-pg-sys",
        "--no-default-features",
        "--features",
        &format!("pg{postgres}"),
        "--message-format=json-render-diagnostics",
    ]);
    log(format_args!("+ {}", process::describe(&command)));
    let mut child = command.stdout(Stdio::piped()).spawn().wrap_err("starting `cargo build`")?;
    let stdout = child.stdout.take().expect("stdout was piped");
    let mut out_dir = None;
    for message in Message::parse_stream(BufReader::new(stdout)) {
        if let Message::BuildScriptExecuted(script) =
            message.wrap_err("reading Cargo's messages")?
            && workspace.by_id(&script.package_id).is_some_and(|p| p.name.as_str() == "pgrx-pg-sys")
        {
            out_dir = Some(script.out_dir);
        }
    }
    let built = child.wait().wrap_err("waiting for `cargo build`")?.success();
    Ok(out_dir.filter(|_| built))
}

/// Build pgrx-pg-sys for each PostgreSQL version in `versions` and write a summary of each
/// build's macro report to `out/pgNN.json`. Returns whether every version built with a
/// libclang and Clang driver of major version `clang`.
pub(crate) fn summarize(clang: u32, out: &Path, versions: &[u16]) -> eyre::Result<bool> {
    ensure!(!versions.is_empty(), "no PostgreSQL versions to summarize");
    let workspace = Workspace::load()?;
    std::fs::create_dir_all(out).wrap_err_with(|| format!("creating {}", out.display()))?;
    let mut passed = true;
    for &postgres in versions {
        let Some(out_dir) = build(&workspace, postgres)? else {
            log(format_args!("building pgrx-pg-sys for PostgreSQL {postgres} failed"));
            passed = false;
            continue;
        };
        let report = out_dir.join(format!("pg{postgres}_macro_report.json"));
        let json = std::fs::read(&report).wrap_err_with(|| format!("reading {report}"))?;
        match Summary::from_report(&json, postgres, clang).wrap_err_with(|| report.to_string()) {
            Ok(summary) => {
                let path = out.join(format!("pg{postgres}.json"));
                std::fs::write(&path, serde_json::to_vec(&summary)?)
                    .wrap_err_with(|| format!("writing {}", path.display()))?;
                let emitted = summary
                    .macros
                    .values()
                    .filter(|outcome| matches!(outcome, Outcome::Emitted { .. }))
                    .count();
                log(format_args!(
                    "PostgreSQL {postgres} on {} with {}: {emitted} of {} macros emitted",
                    summary.arch,
                    summary.libclang,
                    summary.macros.len()
                ));
            }
            Err(error) => {
                log(format_args!("{error:?}"));
                passed = false;
            }
        }
    }
    Ok(passed)
}

/// Compare every summary in `dir` and the directories directly inside it, which is how
/// `actions/download-artifact` lays out one artifact per cell. Returns whether the Clang
/// majors agree, apart from [`EXPECTED`].
pub(crate) fn run_compare(dir: &Path) -> eyre::Result<bool> {
    let mut files = Vec::new();
    let mut dirs = vec![(dir.to_owned(), true)];
    while let Some((dir, descend)) = dirs.pop() {
        let entries =
            std::fs::read_dir(&dir).wrap_err_with(|| format!("reading {}", dir.display()))?;
        for entry in entries {
            let path = entry.wrap_err_with(|| format!("reading {}", dir.display()))?.path();
            if path.is_dir() {
                if descend {
                    dirs.push((path, false));
                }
            } else if path.extension().is_some_and(|extension| extension == "json") {
                files.push(path);
            }
        }
    }
    files.sort();
    let mut summaries = Vec::new();
    for file in &files {
        let json = std::fs::read(file).wrap_err_with(|| format!("reading {}", file.display()))?;
        let summary: Summary = serde_json::from_slice(&json)
            .map_err(|error| eyre!("{} is not a macro summary: {error}", file.display()))?;
        summaries.push(summary);
    }
    if summaries.is_empty() {
        bail!("{} holds no macro summaries", dir.display());
    }
    let comparison = compare(&summaries, EXPECTED)?;
    print!("{}", comparison.render());
    Ok(comparison.passed())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn emitted(caller_unsafe: bool) -> Outcome {
        Outcome::Emitted {
            caller_unsafe,
            invocation: "parenthesized_scalar_expressions".to_owned(),
            fallback: None,
        }
    }

    fn skipped(code: &str) -> Outcome {
        Outcome::Skipped { reason: Reason { code: code.to_owned(), message: "why".to_owned() } }
    }

    fn summary(arch: &str, postgres: u16, clang: u32, macros: &[(&str, Outcome)]) -> Summary {
        Summary {
            postgres,
            arch: arch.to_owned(),
            clang,
            libclang: format!("clang version {clang}.0.0"),
            macros: macros
                .iter()
                .map(|(name, outcome)| ((*name).to_owned(), outcome.clone()))
                .collect(),
        }
    }

    fn found(comparison: &Comparison) -> Vec<(&str, u16, &str, bool)> {
        comparison
            .differences
            .iter()
            .map(|d| (d.name.as_str(), d.postgres, d.arch.as_str(), d.expected))
            .collect()
    }

    const LIBCLANG_15: Expected =
        Expected { name: "B", arch: "x86_64", postgres: 17..=18, clang: 14..=15, skip: "call" };

    #[test]
    fn agreeing_reports_pass() {
        let macros = [("A", emitted(true)), ("B", skipped("statement"))];
        // Majors that skip for different reasons, or don't list the macro, generate the same.
        let other = [("A", emitted(true)), ("B", skipped("variadic"))];
        let absent = [("A", emitted(true))];
        let comparison = compare(
            &[
                summary("x86_64", 18, 14, &macros),
                summary("x86_64", 18, 15, &other),
                summary("x86_64", 18, 16, &absent),
            ],
            &[],
        )
        .unwrap();
        assert!(comparison.passed(), "{}", comparison.render());
        assert_eq!(comparison.compared, ["x86_64, PostgreSQL 18: Clang 14-16, 2 macros"]);
    }

    #[test]
    fn each_kind_of_difference_is_found() {
        let calls_c = Outcome::Emitted {
            caller_unsafe: true,
            invocation: "parenthesized_scalar_expressions".to_owned(),
            fallback: Some(Reason { code: "call".to_owned(), message: "no binding".to_owned() }),
        };
        let statements = Outcome::Emitted {
            caller_unsafe: true,
            invocation: "statements".to_owned(),
            fallback: None,
        };
        let comparison = compare(
            &[
                summary(
                    "x86_64",
                    18,
                    14,
                    &[
                        ("FORM", calls_c),
                        ("INVOCATION", statements),
                        ("SAFETY", emitted(false)),
                        ("SAME", emitted(true)),
                        ("SKIP", skipped("unsupported_type")),
                    ],
                ),
                summary(
                    "x86_64",
                    18,
                    16,
                    &[
                        ("ABSENT", emitted(true)),
                        ("FORM", emitted(true)),
                        ("INVOCATION", emitted(true)),
                        ("SAFETY", emitted(true)),
                        ("SAME", emitted(true)),
                        ("SKIP", emitted(true)),
                    ],
                ),
            ],
            &[],
        )
        .unwrap();
        assert!(!comparison.passed());
        let names: Vec<&str> = comparison.differences.iter().map(|d| d.name.as_str()).collect();
        assert_eq!(names, ["ABSENT", "FORM", "INVOCATION", "SAFETY", "SKIP"]);
        let table = comparison.render();
        for row in [
            "  ABSENT      18  x86_64  14     not in the report",
            "  FORM        18  x86_64  14     emitted: caller unsafe, \
             parenthesized_scalar_expressions, calls the C function (call: no binding)",
            "  SAFETY      18  x86_64  16     emitted: caller unsafe, \
             parenthesized_scalar_expressions, translated",
            "  SKIP        18  x86_64  14     skipped: unsupported_type: why",
            "unexpected differences: 5, expected differences: 0, problems: 0",
        ] {
            assert!(table.lines().any(|line| line == row), "no `{row}` in\n{table}");
        }
    }

    #[test]
    fn architectures_and_postgres_versions_are_compared_separately() {
        let comparison = compare(
            &[
                summary("x86_64", 17, 14, &[("A", emitted(true))]),
                summary("x86_64", 17, 21, &[("A", emitted(true))]),
                summary("x86_64", 18, 14, &[("A", skipped("call"))]),
                summary("x86_64", 18, 21, &[("A", emitted(true))]),
                summary("aarch64", 18, 14, &[("A", skipped("call"))]),
                summary("aarch64", 18, 21, &[("A", skipped("call"))]),
            ],
            &[],
        )
        .unwrap();
        assert_eq!(found(&comparison), [("A", 18, "x86_64", false)]);
    }

    #[test]
    fn an_expected_difference_passes_only_as_listed() {
        let reports = |old: Outcome, new: Outcome| {
            [
                summary("x86_64", 18, 14, &[("B", old.clone())]),
                summary("x86_64", 18, 15, &[("B", old)]),
                summary("x86_64", 18, 16, &[("B", new.clone())]),
                summary("x86_64", 18, 21, &[("B", new)]),
            ]
        };
        let comparison = compare(&reports(skipped("call"), emitted(true)), &[LIBCLANG_15]).unwrap();
        assert_eq!(found(&comparison), [("B", 18, "x86_64", true)]);
        assert!(comparison.passed(), "{}", comparison.render());
        assert!(comparison.render().contains("Expected differences:\n"));

        // Another skip code, another Clang major or another architecture is a new difference.
        let comparison =
            compare(&reports(skipped("statement"), emitted(true)), &[LIBCLANG_15]).unwrap();
        assert_eq!(found(&comparison), [("B", 18, "x86_64", false)]);
        let mut wider = reports(skipped("call"), emitted(true));
        wider[2] = summary("x86_64", 18, 16, &[("B", skipped("call"))]);
        let comparison = compare(&wider, &[LIBCLANG_15]).unwrap();
        assert_eq!(found(&comparison), [("B", 18, "x86_64", false)]);
        let mut arm = reports(skipped("call"), emitted(true));
        for summary in &mut arm {
            summary.arch = "aarch64".to_owned();
        }
        let comparison = compare(&arm, &[LIBCLANG_15]).unwrap();
        assert_eq!(found(&comparison), [("B", 18, "aarch64", false)]);
        // The majors that emit it must still agree with each other.
        let mut split = reports(skipped("call"), emitted(true));
        split[3] = summary("x86_64", 18, 21, &[("B", emitted(false))]);
        let comparison = compare(&split, &[LIBCLANG_15]).unwrap();
        assert_eq!(found(&comparison), [("B", 18, "x86_64", false)]);
    }

    #[test]
    fn an_expected_difference_that_no_longer_occurs_is_stale() {
        let fixed = [
            summary("x86_64", 17, 15, &[("B", skipped("call"))]),
            summary("x86_64", 17, 16, &[("B", emitted(true))]),
            summary("x86_64", 18, 15, &[("B", emitted(true))]),
            summary("x86_64", 18, 16, &[("B", emitted(true))]),
        ];
        let comparison = compare(&fixed, &[LIBCLANG_15]).unwrap();
        assert_eq!(found(&comparison), [("B", 17, "x86_64", true)]);
        assert!(!comparison.passed());
        assert_eq!(
            comparison.problems,
            ["x86_64, PostgreSQL 18: the expected difference for B no longer occurs (Clang \
                 14-15 skipping it with `call`); remove it from EXPECTED"]
        );
        // Reports from one side of the entry's Clang range cannot show the difference.
        let one_side = [
            summary("x86_64", 18, 16, &[("B", emitted(true))]),
            summary("x86_64", 18, 21, &[("B", emitted(true))]),
        ];
        assert!(compare(&one_side, &[LIBCLANG_15]).unwrap().passed());
    }

    #[test]
    fn missing_and_repeated_reports_do_not_pass() {
        let macros = [("A", emitted(true))];
        let comparison = compare(
            &[
                summary("x86_64", 17, 14, &macros),
                summary("x86_64", 17, 15, &macros),
                summary("x86_64", 18, 14, &macros),
                summary("aarch64", 18, 21, &macros),
            ],
            &[],
        )
        .unwrap();
        assert!(!comparison.passed());
        assert_eq!(
            comparison.problems,
            [
                "aarch64, PostgreSQL 18: only Clang 21 reported, so nothing was compared",
                "x86_64, PostgreSQL 18: no report from Clang 15",
            ]
        );
        assert!(!compare(&[], &[]).unwrap().passed());
        let twice = [summary("x86_64", 18, 14, &macros), summary("x86_64", 18, 14, &macros)];
        assert!(compare(&twice, &[]).is_err());
    }

    #[test]
    fn a_report_becomes_a_summary_and_back() {
        let report = br#"{
            "postgres_major_version": 18,
            "profile": {
                "compiler": {
                    "executable": "/usr/bin/clang-14",
                    "version": "Ubuntu clang version 14.0.0-1ubuntu1.1",
                    "libclang_version": "Ubuntu clang version 14.0.0-1ubuntu1.1"
                },
                "target": { "triple": "x86_64-pc-linux-gnu", "pointer_bits": 64 }
            },
            "macros": [
                {
                    "analysis": { "name": "A", "invocation": "statements", "parameters": [] },
                    "status": {
                        "status": "emitted",
                        "rust": "macro_rules! A {}",
                        "const_capability": "runtime_only",
                        "caller_unsafe": true
                    },
                    "fallback": { "code": "call", "message": "no\n  binding", "spans": [] }
                },
                {
                    "analysis": { "name": "B", "invocation": "not_established" },
                    "status": {
                        "status": "skipped",
                        "reason": { "code": "variadic", "message": "why", "tokens": null }
                    }
                }
            ]
        }"#;
        let summary = Summary::from_report(report, 18, 14).unwrap();
        assert_eq!((summary.arch.as_str(), summary.postgres, summary.clang), ("x86_64", 18, 14));
        assert_eq!(
            Outcome::describe(summary.macros.get("A")),
            "emitted: caller unsafe, statements, calls the C function (call: no binding)"
        );
        assert_eq!(summary.macros["B"], skipped("variadic"));
        let json = serde_json::to_vec(&summary).unwrap();
        let read: Summary = serde_json::from_slice(&json).unwrap();
        assert_eq!(read.macros, summary.macros);

        // A build that loaded another libclang, or another PostgreSQL's headers, is refused.
        assert!(Summary::from_report(report, 18, 15).is_err());
        assert!(Summary::from_report(report, 17, 14).is_err());
    }

    #[test]
    fn clang_majors_and_version_ranges() {
        assert_eq!(clang_major("Ubuntu clang version 14.0.0-1ubuntu1.1"), Some(14));
        assert_eq!(clang_major("Apple clang version 21.0.0 (clang-2100.3.34.2)"), Some(21));
        assert_eq!(clang_major("clang version 23"), Some(23));
        assert_eq!(clang_major("gcc 13"), None);
        assert_eq!(ranges(&[14, 15, 16, 18, 20, 21]), "14-16, 18, 20-21");
        assert_eq!(ranges(&[]), "");
    }
}
