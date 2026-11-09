//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Choose the jobs that a change needs.
//!
//! A change to a workspace package must test that package and every package that depends on
//! it. Cargo's graph supplies most edges; [`RUNTIME_EDGES`] and [`TEST_EDGES`] add the ones it
//! cannot see. Files outside the packages select every job, no job, or the lint or packaging
//! job alone, and a file that no rule names selects every job.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::io::Write as _;
use std::process::Command;

use cargo_metadata::DependencyKind;
use eyre::{WrapErr, bail};

use crate::workspace::{self, Workspace};

/// Packages whose artifacts use other packages at run time. The pg_test harness builds and
/// installs every extension with `cargo pgrx install`.
const RUNTIME_EDGES: &[(&str, &str)] = &[("pgrx-tests", "cargo-pgrx")];

/// Packages whose tests, but not their artifacts, use other packages. cargo-pgrx's install
/// regression test builds pgrx-unit-tests, and pgrx-bindgen's harness tests build crates that
/// use pgrx-macros.
const TEST_EDGES: &[(&str, &str)] =
    &[("cargo-pgrx", "pgrx-unit-tests"), ("pgrx-bindgen", "pgrx-macros")];

/// Packages whose changes select every job, because they decide what every job runs.
const DRIVERS: &[&str] = &["pgrx-ci"];

/// Paths outside the packages that select every job. A trailing `/` names a directory.
const SELECT_ALL: &[&str] = &[
    ".cargo/",
    ".gitattributes",
    ".github/workflows/",
    "Cargo.lock",
    "Cargo.toml",
    "rust-toolchain.toml",
    "rustfmt.toml",
    "tools/",
    "update-versions.sh",
];

/// Paths that no test reads.
const SELECT_NONE: &[&str] = &[
    ".github/FUNDING.yml",
    ".github/docker/",
    ".gitignore",
    "LICENSE",
    "art/",
    "articles/",
    "docs/",
    "prepare-release.sh",
    "publish.sh",
    "run-example.sh",
    "rust-analyzer.toml",
    "skills/",
    "upgrade-deps.sh",
];

/// Paths that only the lint job reads.
const SELECT_LINT: &[&str] = &["deny.toml"];

/// What a job tests, which decides whether a change needs it.
enum Subjects {
    /// The job runs these packages' tests.
    Packages(&'static [&'static str]),
    /// The job builds these packages' artifacts into something else and tests that.
    Artifacts(&'static [&'static str]),
    /// The job tests whatever a change touches.
    Any,
}

/// The optional jobs of `tests.yml`, by the output names that gate them.
const JOBS: &[(&str, Subjects)] = &[
    // Miri checks pointer provenance in pgrx-pg-sys's Datum and pgrx's fcinfo and varlena.
    ("miri", Subjects::Packages(&["pgrx-pg-sys", "pgrx"])),
    ("cmacros_clang14", Subjects::Packages(&["pgrx-c-macros"])),
    ("bindgen_harness", Subjects::Packages(&["pgrx-bindgen"])),
    ("ui", Subjects::Packages(&["pgrx-unit-tests"])),
    ("bench", Subjects::Packages(&["benching", "pgrx-bench"])),
    ("install_regression", Subjects::Packages(&["cargo-pgrx"])),
    ("runas", Subjects::Packages(&["arrays"])),
    // A new extension from the template, built against pgrx-tests, which runs cargo-pgrx.
    ("init", Subjects::Artifacts(&["pgrx-tests"])),
    ("variants", Subjects::Packages(&["pgrx-unit-tests", "custom_types"])),
    ("schema_release", Subjects::Packages(&["pgrx-unit-tests"])),
    ("beta", Subjects::Any),
    ("package", Subjects::Any),
];

/// The revisions whose difference a plan covers.
pub(crate) enum Source {
    /// Every job, as pushes to develop run.
    Everything,
    /// The jobs that the change from `base` to `head` needs.
    Diff { base: String, head: String },
}

/// One workspace package, as the planner sees it.
pub(crate) struct Node {
    pub(crate) name: String,
    /// The package's directory relative to the workspace root.
    pub(crate) dir: String,
    pub(crate) needs_postgres: bool,
    /// The workspace packages it depends on, and whether only its tests use each one.
    pub(crate) dependencies: Vec<(String, bool)>,
}

pub(crate) struct Graph {
    /// Package directories with a trailing `/`, deepest first, so a nested package wins.
    dirs: Vec<(String, String)>,
    needs_postgres: BTreeMap<String, bool>,
    artifact_dependents: BTreeMap<String, BTreeSet<String>>,
    test_dependents: BTreeMap<String, BTreeSet<String>>,
}

impl Graph {
    pub(crate) fn new(nodes: Vec<Node>) -> Self {
        let mut graph = Graph {
            dirs: Vec::new(),
            needs_postgres: BTreeMap::new(),
            artifact_dependents: BTreeMap::new(),
            test_dependents: BTreeMap::new(),
        };
        for node in &nodes {
            graph.dirs.push((format!("{}/", node.dir.trim_end_matches('/')), node.name.clone()));
            graph.needs_postgres.insert(node.name.clone(), node.needs_postgres);
            graph.artifact_dependents.entry(node.name.clone()).or_default();
            graph.test_dependents.entry(node.name.clone()).or_default();
        }
        let known = |name: &str| graph.needs_postgres.contains_key(name);
        let mut artifact_edges = Vec::new();
        let mut test_edges = Vec::new();
        for node in &nodes {
            for (dependency, tests_only) in &node.dependencies {
                if known(dependency) {
                    let edges = if *tests_only { &mut test_edges } else { &mut artifact_edges };
                    edges.push((node.name.clone(), dependency.clone()));
                }
            }
        }
        for (dependent, dependency) in RUNTIME_EDGES {
            if known(dependent) && known(dependency) {
                artifact_edges.push(((*dependent).to_owned(), (*dependency).to_owned()));
            }
        }
        for (dependent, dependency) in TEST_EDGES {
            if known(dependent) && known(dependency) {
                test_edges.push(((*dependent).to_owned(), (*dependency).to_owned()));
            }
        }
        for (dependent, dependency) in artifact_edges {
            graph.artifact_dependents.entry(dependency).or_default().insert(dependent);
        }
        for (dependent, dependency) in test_edges {
            graph.test_dependents.entry(dependency).or_default().insert(dependent);
        }
        graph.dirs.sort_by_key(|(dir, _)| std::cmp::Reverse(dir.len()));
        graph
    }

    pub(crate) fn from_workspace(workspace: &Workspace) -> Self {
        let root = workspace.root();
        let nodes = workspace
            .packages()
            .iter()
            .map(|package| {
                let dir = workspace::package_dir(package);
                Node {
                    name: package.name.to_string(),
                    dir: dir.strip_prefix(root).unwrap_or(dir).as_str().replace('\\', "/"),
                    needs_postgres: workspace::needs_postgres(package),
                    dependencies: package
                        .dependencies
                        .iter()
                        .filter(|dependency| dependency.path.is_some())
                        .map(|dependency| {
                            (
                                dependency.name.clone(),
                                dependency.kind == DependencyKind::Development,
                            )
                        })
                        .collect(),
                }
            })
            .collect();
        Self::new(nodes)
    }

    fn owner(&self, path: &str) -> Option<&str> {
        self.dirs
            .iter()
            .find(|(dir, _)| path.starts_with(dir.as_str()))
            .map(|(_, name)| name.as_str())
    }

    /// The packages whose artifacts a change to `changed` alters.
    fn artifacts(&self, changed: &BTreeSet<String>) -> BTreeSet<String> {
        let mut artifacts = BTreeSet::new();
        let mut pending: Vec<&str> = changed.iter().map(String::as_str).collect();
        while let Some(name) = pending.pop() {
            if artifacts.insert(name.to_owned()) {
                pending.extend(self.artifact_dependents[name].iter().map(String::as_str));
            }
        }
        artifacts
    }

    /// The packages whose tests use one of `artifacts`. A test-only edge reruns the
    /// dependent's tests, but the dependent's artifact doesn't change, so it goes no further.
    fn tested(&self, artifacts: &BTreeSet<String>) -> BTreeSet<String> {
        let mut tested = artifacts.clone();
        for name in artifacts {
            tested.extend(self.test_dependents[name].iter().cloned());
        }
        tested
    }

    fn all(&self) -> BTreeSet<String> {
        self.needs_postgres.keys().cloned().collect()
    }
}

/// The jobs a change needs.
pub(crate) struct Plan {
    everything: bool,
    reasons: Vec<String>,
    changed: BTreeSet<String>,
    /// The packages whose artifacts the change alters.
    artifacts: BTreeSet<String>,
    /// The packages whose tests must run.
    affected: BTreeSet<String>,
    lint: bool,
    markdown: bool,
}

impl Plan {
    /// Plan for the changed `files`, or for every job when there are none to compare.
    pub(crate) fn new(graph: &Graph, files: Option<&[String]>) -> Self {
        let mut plan = Plan {
            everything: files.is_none(),
            reasons: Vec::new(),
            changed: BTreeSet::new(),
            artifacts: BTreeSet::new(),
            affected: BTreeSet::new(),
            lint: false,
            markdown: false,
        };
        for path in files.unwrap_or_default() {
            if path.ends_with(".md") {
                // No code includes a Markdown file, but packaging copies the manifests'
                // `readme` files.
                plan.markdown = true;
            } else if let Some(owner) = graph.owner(path) {
                if DRIVERS.contains(&owner) {
                    plan.everything = true;
                    plan.reasons.push(format!("{path} changes the CI driver"));
                }
                plan.changed.insert(owner.to_owned());
            } else if matches(path, SELECT_ALL) {
                plan.everything = true;
                plan.reasons.push(format!("{path} selects every job"));
            } else if matches(path, SELECT_LINT) {
                plan.lint = true;
            } else if !matches(path, SELECT_NONE) {
                plan.everything = true;
                plan.reasons.push(format!("no rule names {path}, so it selects every job"));
            }
        }
        plan.artifacts = if plan.everything { graph.all() } else { graph.artifacts(&plan.changed) };
        plan.affected = graph.tested(&plan.artifacts);
        plan.lint |= !plan.affected.is_empty();
        plan
    }

    fn packages(&self, graph: &Graph, postgres: bool) -> Vec<&str> {
        self.affected
            .iter()
            .filter(|name| graph.needs_postgres[name.as_str()] == postgres)
            .map(String::as_str)
            .collect()
    }

    fn job(&self, subjects: &Subjects) -> bool {
        match subjects {
            Subjects::Packages(names) => names.iter().any(|name| self.affected.contains(*name)),
            Subjects::Artifacts(names) => names.iter().any(|name| self.artifacts.contains(*name)),
            Subjects::Any => !self.affected.is_empty(),
        }
    }

    /// The `name=value` lines that gate the workflow's jobs.
    pub(crate) fn outputs(&self, graph: &Graph) -> Vec<(&'static str, String)> {
        let mut outputs = vec![
            ("lint", self.lint.to_string()),
            ("pg", self.packages(graph, true).join(" ")),
            ("crates", self.packages(graph, false).join(" ")),
        ];
        for (name, subjects) in JOBS {
            let selected = self.job(subjects) || (*name == "package" && self.markdown);
            outputs.push((name, selected.to_string()));
        }
        outputs
    }

    fn describe(&self) -> String {
        let mut text = String::new();
        let _ = writeln!(text, "everything: {}", self.everything);
        for reason in &self.reasons {
            let _ = writeln!(text, "  {reason}");
        }
        let _ = writeln!(text, "changed packages: {:?}", self.changed);
        let _ = writeln!(text, "affected packages: {:?}", self.affected);
        text
    }
}

fn matches(path: &str, patterns: &[&str]) -> bool {
    patterns.iter().any(|pattern| {
        if pattern.ends_with('/') { path.starts_with(pattern) } else { path == *pattern }
    })
}

fn changed_files(base: &str, head: &str) -> eyre::Result<Vec<String>> {
    let output = Command::new("git")
        .args(["diff", "--name-only", "--no-renames", base, head])
        .output()
        .wrap_err("running `git diff`")?;
    if !output.status.success() {
        bail!("`git diff {base} {head}` failed: {}", String::from_utf8_lossy(&output.stderr));
    }
    let files = String::from_utf8(output.stdout).wrap_err("`git diff` printed a non-UTF-8 path")?;
    Ok(files.lines().map(str::to_owned).collect())
}

/// Plan for `source`, print the plan, and write the job gates to `$GITHUB_OUTPUT` when set.
pub(crate) fn run(source: &Source) -> eyre::Result<()> {
    let files = match source {
        Source::Everything => None,
        Source::Diff { base, head } => Some(changed_files(base, head)?),
    };
    let graph = Graph::from_workspace(&Workspace::load()?);
    let plan = Plan::new(&graph, files.as_deref());
    print!("{}", plan.describe());

    let mut lines = String::new();
    for (name, value) in plan.outputs(&graph) {
        println!("{name}={value}");
        let _ = writeln!(lines, "{name}={value}");
    }
    if let Some(path) = std::env::var_os("GITHUB_OUTPUT") {
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(&path)
            .wrap_err("opening $GITHUB_OUTPUT")?;
        file.write_all(lines.as_bytes()).wrap_err("writing $GITHUB_OUTPUT")?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(name: &str, dir: &str, needs_postgres: bool, dependencies: &[(&str, bool)]) -> Node {
        Node {
            name: name.into(),
            dir: dir.into(),
            needs_postgres,
            dependencies: dependencies
                .iter()
                .map(|(name, tests)| ((*name).into(), *tests))
                .collect(),
        }
    }

    /// A miniature of the workspace's shape.
    fn graph() -> Graph {
        Graph::new(vec![
            node("pgrx-pg-config", "pgrx-pg-config", false, &[]),
            node("pgrx-c-macros", "pgrx-c-macros", false, &[("pgrx-pg-config", false)]),
            node("pgrx-bindgen", "pgrx-bindgen", false, &[("pgrx-c-macros", false)]),
            node("pgrx-macros", "pgrx-macros", false, &[]),
            node(
                "pgrx-pg-sys",
                "pgrx-pg-sys",
                true,
                &[("pgrx-bindgen", false), ("pgrx-macros", false)],
            ),
            node("pgrx", "pgrx", true, &[("pgrx-pg-sys", false), ("pgrx-macros", false)]),
            node("pgrx-tests", "pgrx-tests", true, &[("pgrx", false), ("pgrx-pg-config", false)]),
            node(
                "pgrx-unit-tests",
                "pgrx-unit-tests",
                true,
                &[("pgrx", false), ("pgrx-tests", false)],
            ),
            node("cargo-pgrx", "cargo-pgrx", false, &[("pgrx-pg-config", false)]),
            node("arrays", "pgrx-examples/arrays", true, &[("pgrx", false), ("pgrx-tests", true)]),
            node("pgrx-ci", "tools/ci", false, &[]),
            node("pgrx-version-updater", "tools/version-updater", false, &[]),
        ])
    }

    fn files(paths: &[&str]) -> Vec<String> {
        paths.iter().map(|path| (*path).to_owned()).collect()
    }

    fn output(plan: &Plan, graph: &Graph, name: &str) -> String {
        plan.outputs(graph).into_iter().find(|(key, _)| *key == name).unwrap().1
    }

    #[test]
    fn a_package_change_reaches_every_dependent() {
        let graph = graph();
        let plan = Plan::new(&graph, Some(&files(&["pgrx-c-macros/src/lib.rs"])));
        assert!(!plan.everything);
        assert_eq!(
            output(&plan, &graph, "pg"),
            "arrays pgrx pgrx-pg-sys pgrx-tests pgrx-unit-tests"
        );
        // cargo-pgrx's install regression test builds the changed pgrx-unit-tests.
        assert_eq!(output(&plan, &graph, "crates"), "cargo-pgrx pgrx-bindgen pgrx-c-macros");
        assert_eq!(output(&plan, &graph, "miri"), "true");
        assert_eq!(output(&plan, &graph, "init"), "true");
    }

    #[test]
    fn cargo_pgrx_reaches_every_pg_test_through_the_harness() {
        let graph = graph();
        let plan = Plan::new(&graph, Some(&files(&["cargo-pgrx/src/command/regress.rs"])));
        assert_eq!(output(&plan, &graph, "pg"), "arrays pgrx-tests pgrx-unit-tests");
        assert_eq!(output(&plan, &graph, "crates"), "cargo-pgrx");
        assert_eq!(output(&plan, &graph, "miri"), "false");
    }

    #[test]
    fn test_only_edges_stop_the_walk() {
        let graph = graph();
        // cargo-pgrx's tests build pgrx-unit-tests, but cargo-pgrx's artifact doesn't change,
        // so pgrx-tests, which runs cargo-pgrx, needs no new run.
        let plan = Plan::new(&graph, Some(&files(&["pgrx-unit-tests/src/lib.rs"])));
        assert_eq!(output(&plan, &graph, "pg"), "pgrx-unit-tests");
        assert_eq!(output(&plan, &graph, "crates"), "cargo-pgrx");
        assert_eq!(output(&plan, &graph, "install_regression"), "true");
        assert_eq!(output(&plan, &graph, "init"), "false");
    }

    #[test]
    fn an_example_change_tests_only_that_example() {
        let graph = graph();
        let plan = Plan::new(&graph, Some(&files(&["pgrx-examples/arrays/src/lib.rs"])));
        assert_eq!(output(&plan, &graph, "pg"), "arrays");
        assert_eq!(output(&plan, &graph, "crates"), "");
        assert_eq!(output(&plan, &graph, "runas"), "true");
    }

    #[test]
    fn documentation_selects_only_packaging() {
        let graph = graph();
        let plan =
            Plan::new(&graph, Some(&files(&["README.md", "pgrx/README.md", "docs/src/x.md"])));
        assert_eq!(output(&plan, &graph, "pg"), "");
        assert_eq!(output(&plan, &graph, "crates"), "");
        assert_eq!(output(&plan, &graph, "lint"), "false");
        assert_eq!(output(&plan, &graph, "package"), "true");
        assert_eq!(output(&plan, &graph, "beta"), "false");
    }

    #[test]
    fn configuration_drivers_and_unknown_files_select_everything() {
        let graph = graph();
        for path in
            ["Cargo.lock", ".github/workflows/tests.yml", "tools/ci/src/plan.rs", "new-file"]
        {
            let plan = Plan::new(&graph, Some(&files(&[path])));
            assert!(plan.everything, "{path}");
            assert_eq!(plan.affected, graph.all(), "{path}");
        }
        // A nested package's own directory still wins over tools/.
        let plan = Plan::new(&graph, Some(&files(&["tools/version-updater/src/main.rs"])));
        assert!(!plan.everything);
        assert_eq!(output(&plan, &graph, "crates"), "pgrx-version-updater");
    }

    #[test]
    fn ignored_and_lint_only_files() {
        let graph = graph();
        let plan = Plan::new(&graph, Some(&files(&["skills/x/SKILL", "LICENSE"])));
        assert!(plan.affected.is_empty());
        assert_eq!(output(&plan, &graph, "lint"), "false");
        let plan = Plan::new(&graph, Some(&files(&["deny.toml"])));
        assert_eq!(output(&plan, &graph, "lint"), "true");
        assert_eq!(output(&plan, &graph, "pg"), "");
    }
}
