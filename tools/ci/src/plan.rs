//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Choose the jobs that a change needs.
//!
//! A change to a workspace package must test that package and every package that depends on
//! it. Cargo's graph supplies most edges; [`RUNTIME_EDGES`] and [`TEST_EDGES`] add the ones it
//! cannot see, and so do the literal `../` paths with which one package's sources include or
//! open another package's files. A change under a package's [`TEST_DIRS`] reruns its tests but
//! leaves its artifact, and so its dependents, alone. Markdown selects only packaging, and only
//! for the published packages' readmes. Files outside the packages select every job, no job,
//! or the lint job alone, and a file that no rule names selects every job.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::io::Write as _;
use std::process::Command;

use cargo_metadata::DependencyKind;
use cargo_metadata::camino::Utf8Path;
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

/// Directories, relative to a package, that only its tests, benchmarks and examples build
/// from, so a change there leaves its artifact alone.
const TEST_DIRS: &[&str] = &["tests/", "benches/", "examples/"];

/// Packages whose changes select every job, because they decide what every job runs.
const DRIVERS: &[&str] = &["pgrx-ci"];

/// Packages that packaging runs: update-versions.sh runs pgrx-version-updater.
const PACKAGING_TOOLS: &[&str] = &["pgrx-version-updater"];

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
    /// The job packages the published packages, with their readmes, using [`PACKAGING_TOOLS`].
    Packaging,
    /// The job tests whatever a change touches.
    Any,
}

/// The optional jobs of `tests.yml`, by the output names that gate them.
const JOBS: &[(&str, Subjects)] = &[
    // Miri checks pointer provenance in pgrx-pg-sys's Datum and pgrx's fcinfo and varlena.
    ("miri", Subjects::Packages(&["pgrx-pg-sys", "pgrx"])),
    // Every supported Clang runs the generator's tests, generates pgrx-pg-sys's macros
    // through pgrx-bindgen's build script and runs pgrx-pg-sys's tests of them.
    ("clang", Subjects::Packages(&["pgrx-c-macros", "pgrx-bindgen", "pgrx-pg-sys"])),
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
    ("package", Subjects::Packaging),
    // The docs.rs builds of pgrx-pg-sys and pgrx, from their committed bindings.
    ("docsrs", Subjects::Artifacts(&["pgrx-pg-sys", "pgrx"])),
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
    pub(crate) published: bool,
    /// Its readme relative to the workspace root.
    pub(crate) readme: Option<String>,
    /// The workspace packages it depends on, and whether only its tests use each one.
    pub(crate) dependencies: Vec<(String, bool)>,
}

/// A path that a package's sources name outside the package, which Cargo's graph doesn't
/// show: a file one of its modules includes, or a file or directory one of its tests opens.
struct Read {
    /// A file, or a directory and everything under it, relative to the workspace root.
    path: String,
    reader: String,
    /// Whether only the reader's tests use the path, so a change there leaves its artifact
    /// alone.
    tests_only: bool,
}

impl Read {
    fn covers(&self, path: &str) -> bool {
        path.strip_prefix(self.path.as_str())
            .is_some_and(|rest| rest.is_empty() || rest.starts_with('/'))
    }
}

pub(crate) struct Graph {
    /// Package directories with a trailing `/`, deepest first, so a nested package wins.
    dirs: Vec<(String, String)>,
    needs_postgres: BTreeMap<String, bool>,
    published: BTreeSet<String>,
    /// The published packages' readmes, which packaging copies.
    readmes: BTreeSet<String>,
    artifact_dependents: BTreeMap<String, BTreeSet<String>>,
    test_dependents: BTreeMap<String, BTreeSet<String>>,
    reads: Vec<Read>,
}

impl Graph {
    pub(crate) fn new(nodes: Vec<Node>) -> Self {
        let mut graph = Graph {
            dirs: Vec::new(),
            needs_postgres: BTreeMap::new(),
            published: BTreeSet::new(),
            readmes: BTreeSet::new(),
            artifact_dependents: BTreeMap::new(),
            test_dependents: BTreeMap::new(),
            reads: Vec::new(),
        };
        for node in &nodes {
            graph.dirs.push((format!("{}/", node.dir.trim_end_matches('/')), node.name.clone()));
            graph.needs_postgres.insert(node.name.clone(), node.needs_postgres);
            if node.published {
                graph.published.insert(node.name.clone());
                graph.readmes.extend(node.readme.clone());
            }
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

    pub(crate) fn from_workspace(workspace: &Workspace) -> eyre::Result<Self> {
        let root = workspace.root();
        let nodes = workspace
            .packages()
            .iter()
            .map(|package| {
                let dir = workspace::package_dir(package);
                let dir = dir.strip_prefix(root).unwrap_or(dir).as_str().replace('\\', "/");
                Node {
                    name: package.name.to_string(),
                    needs_postgres: workspace::needs_postgres(package),
                    // `publish = false` is an empty list of registries.
                    published: package.publish.as_ref().is_none_or(|to| !to.is_empty()),
                    readme: package.readme.as_ref().and_then(|path| resolve(&dir, path.as_str())),
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
                    dir,
                }
            })
            .collect();
        let mut graph = Self::new(nodes);
        graph.find_reads(&tracked_files(root)?, |file| {
            std::fs::read_to_string(root.join(file)).ok()
        });
        Ok(graph)
    }

    /// The package that holds `path`, and `path` relative to the package's directory.
    fn owner<'a>(&self, path: &'a str) -> Option<(&str, &'a str)> {
        self.dirs.iter().find_map(|(dir, name)| {
            path.strip_prefix(dir.as_str()).map(|relative| (name.as_str(), relative))
        })
    }

    /// Record the paths outside their packages that the packages' Rust sources name with string
    /// literals starting `../`. A literal can be relative to its source file, as `include!` and
    /// `#[path]` are, or to the package directory, as paths joined to `CARGO_MANIFEST_DIR`
    /// are, so both count when they name a tracked file or directory. A literal in a test
    /// directory or a doc comment, or naming another package's test directory, is test code.
    /// [`SELECT_NONE`] paths don't count, so test fixtures that happen to name one, such as
    /// `../LICENSE`, don't either. Paths that sources compute some other way stay invisible.
    fn find_reads(&mut self, tracked: &BTreeSet<String>, read: impl Fn(&str) -> Option<String>) {
        let exists = |path: &str| {
            let dir = format!("{path}/");
            tracked.contains(path)
                || tracked.range(dir.clone()..).next().is_some_and(|file| file.starts_with(&dir))
        };
        // Whether each path is read only by tests, per reader; any other read wins.
        let mut reads: BTreeMap<(String, String), bool> = BTreeMap::new();
        for file in tracked.iter().filter(|file| file.ends_with(".rs")) {
            let Some((reader, relative)) = self.owner(file) else { continue };
            let Some(text) = read(file) else { continue };
            let package_dir = &file[..file.len() - relative.len() - 1];
            let file_dir = file.rsplit_once('/').map_or("", |(dir, _)| dir);
            for (doc, literal) in parent_paths(&text) {
                for base in [file_dir, package_dir] {
                    let Some(path) = resolve(base, literal)
                        .filter(|path| exists(path) && !matches(path, SELECT_NONE))
                    else {
                        continue;
                    };
                    let dir = format!("{path}/");
                    let owner = self.owner(&path).or_else(|| self.owner(&dir));
                    if owner.is_some_and(|(name, _)| name == reader) {
                        continue;
                    }
                    // A library doesn't build from another package's tests, so naming them is
                    // test code too.
                    let tests_only = in_test_dir(relative)
                        || doc
                        || owner.is_some_and(|(_, relative)| in_test_dir(relative));
                    *reads.entry((path, reader.to_owned())).or_insert(true) &= tests_only;
                }
            }
        }
        self.reads = reads
            .into_iter()
            .map(|((path, reader), tests_only)| Read { path, reader, tests_only })
            .collect();
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

/// Whether a path relative to its package is in one of the package's [`TEST_DIRS`].
fn in_test_dir(relative: &str) -> bool {
    TEST_DIRS.iter().any(|dir| relative.starts_with(dir))
}

/// The string literals in Rust source that start with `../` or `/../`, without the leading
/// `/`, and whether each is in a doc comment, which only doctests compile. Literals with
/// braces or escapes are format strings or harder to read, and are skipped.
fn parent_paths(text: &str) -> impl Iterator<Item = (bool, &str)> {
    text.lines().flat_map(|line| {
        let comment = line.trim_start();
        let doc = comment.starts_with("///") || comment.starts_with("//!");
        line.match_indices('"').filter_map(move |(at, _)| {
            let literal = &line[at + 1..];
            let path = literal.strip_prefix('/').unwrap_or(literal);
            let end = path.find(['"', '{', '}', '\\'])?;
            (path.starts_with("../") && path[end..].starts_with('"')).then(|| (doc, &path[..end]))
        })
    })
}

/// `relative` joined to the directory `base`, both relative to the workspace root, or `None`
/// when it leaves the workspace or names its root.
fn resolve(base: &str, relative: &str) -> Option<String> {
    let mut parts: Vec<&str> = base.split('/').filter(|part| !part.is_empty()).collect();
    for part in relative.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                parts.pop()?;
            }
            part => parts.push(part),
        }
    }
    (!parts.is_empty()).then(|| parts.join("/"))
}

/// The jobs a change needs.
pub(crate) struct Plan {
    everything: bool,
    reasons: Vec<String>,
    /// The packages whose artifacts the change alters directly.
    changed: BTreeSet<String>,
    /// The packages whose tests, but not artifacts, the change alters.
    tests_changed: BTreeSet<String>,
    /// The packages whose artifacts the change alters.
    artifacts: BTreeSet<String>,
    /// The packages whose tests must run.
    affected: BTreeSet<String>,
    lint: bool,
    /// Whether a published package's readme changed.
    readme: bool,
}

impl Plan {
    /// Plan for the changed `files`, or for every job when there are none to compare.
    pub(crate) fn new(graph: &Graph, files: Option<&[String]>) -> Self {
        let mut plan = Plan {
            everything: files.is_none(),
            reasons: Vec::new(),
            changed: BTreeSet::new(),
            tests_changed: BTreeSet::new(),
            artifacts: BTreeSet::new(),
            affected: BTreeSet::new(),
            lint: false,
            readme: false,
        };
        for path in files.unwrap_or_default() {
            for read in graph.reads.iter().filter(|read| read.covers(path)) {
                let readers =
                    if read.tests_only { &mut plan.tests_changed } else { &mut plan.changed };
                readers.insert(read.reader.clone());
            }
            if path.ends_with(".md") {
                // Code that names a Markdown file is a read above.
                plan.readme |= graph.readmes.contains(path);
            } else if let Some((owner, relative)) = graph.owner(path) {
                if DRIVERS.contains(&owner) {
                    plan.everything = true;
                    plan.reasons.push(format!("{path} changes the CI driver"));
                }
                if in_test_dir(relative) {
                    plan.tests_changed.insert(owner.to_owned());
                } else {
                    plan.changed.insert(owner.to_owned());
                }
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
        plan.affected.extend(plan.tests_changed.iter().cloned());
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

    fn job(&self, graph: &Graph, subjects: &Subjects) -> bool {
        match subjects {
            Subjects::Packages(names) => names.iter().any(|name| self.affected.contains(*name)),
            Subjects::Artifacts(names) => names.iter().any(|name| self.artifacts.contains(*name)),
            Subjects::Packaging => {
                self.readme
                    || self.artifacts.iter().any(|name| {
                        graph.published.contains(name) || PACKAGING_TOOLS.contains(&name.as_str())
                    })
            }
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
            outputs.push((name, self.job(graph, subjects).to_string()));
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
        let _ = writeln!(text, "packages with only changed tests: {:?}", self.tests_changed);
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

/// The repository's tracked files, relative to the workspace root.
fn tracked_files(root: &Utf8Path) -> eyre::Result<BTreeSet<String>> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["ls-files", "-z"])
        .output()
        .wrap_err("running `git ls-files`")?;
    if !output.status.success() {
        bail!("`git ls-files` failed: {}", String::from_utf8_lossy(&output.stderr));
    }
    let files =
        String::from_utf8(output.stdout).wrap_err("`git ls-files` printed a non-UTF-8 path")?;
    Ok(files.split('\0').filter(|file| !file.is_empty()).map(str::to_owned).collect())
}

/// Plan for `source`, print the plan, and write the job gates to `$GITHUB_OUTPUT` when set.
pub(crate) fn run(source: &Source) -> eyre::Result<()> {
    let files = match source {
        Source::Everything => None,
        Source::Diff { base, head } => Some(changed_files(base, head)?),
    };
    let graph = Graph::from_workspace(&Workspace::load()?)?;
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
            published: !matches!(
                name,
                "pgrx-unit-tests" | "arrays" | "pgrx-ci" | "pgrx-version-updater"
            ),
            // pgrx's readme is the repository's.
            readme: Some(if name == "pgrx" {
                "README.md".into()
            } else {
                format!("{dir}/README.md")
            }),
            dependencies: dependencies
                .iter()
                .map(|(name, tests)| ((*name).into(), *tests))
                .collect(),
        }
    }

    /// A miniature of the workspace's shape.
    fn graph() -> Graph {
        let mut graph = Graph::new(vec![
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
        ]);
        let tracked = [
            "pgrx-c-macros/tests/support/oracle.rs",
            "pgrx-c-macros/tests/call_oracle.rs",
            "pgrx-pg-sys/src/c_macros/support.rs",
            "pgrx-pg-sys/tests/oracle.rs",
        ];
        // Escaped quotes keep the planner's scan of this file from reading these as its paths.
        let sources = [
            (
                "pgrx-c-macros/tests/call_oracle.rs",
                "let support = dir.join(\"../pgrx-pg-sys/src/c_macros/support.rs\");",
            ),
            (
                "pgrx-pg-sys/tests/oracle.rs",
                "#[path = \"../../pgrx-c-macros/tests/support/oracle.rs\"]",
            ),
        ];
        graph.find_reads(&tracked.iter().map(|file| (*file).to_owned()).collect(), |file| {
            sources.iter().find(|(name, _)| *name == file).map(|(_, text)| (*text).to_owned())
        });
        graph
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
        assert_eq!(output(&plan, &graph, "package"), "true");
        assert_eq!(output(&plan, &graph, "docsrs"), "true");
    }

    #[test]
    fn cargo_pgrx_reaches_every_pg_test_through_the_harness() {
        let graph = graph();
        let plan = Plan::new(&graph, Some(&files(&["cargo-pgrx/src/command/regress.rs"])));
        assert_eq!(output(&plan, &graph, "pg"), "arrays pgrx-tests pgrx-unit-tests");
        assert_eq!(output(&plan, &graph, "crates"), "cargo-pgrx");
        assert_eq!(output(&plan, &graph, "miri"), "false");
        assert_eq!(output(&plan, &graph, "clang"), "false");
        assert_eq!(output(&plan, &graph, "docsrs"), "false");
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
        // Examples aren't published.
        assert_eq!(output(&plan, &graph, "package"), "false");
        assert_eq!(output(&plan, &graph, "docsrs"), "false");
    }

    #[test]
    fn a_test_change_reruns_only_its_package_and_readers() {
        let graph = graph();
        let plan = Plan::new(&graph, Some(&files(&["pgrx-c-macros/tests/call_oracle.rs"])));
        assert_eq!(output(&plan, &graph, "pg"), "");
        assert_eq!(output(&plan, &graph, "crates"), "pgrx-c-macros");
        assert_eq!(output(&plan, &graph, "clang"), "true");
        assert_eq!(output(&plan, &graph, "package"), "false");
        // pgrx-pg-sys's tests include pgrx-c-macros' oracle support.
        let plan = Plan::new(&graph, Some(&files(&["pgrx-c-macros/tests/support/oracle.rs"])));
        assert_eq!(output(&plan, &graph, "pg"), "pgrx-pg-sys");
        assert_eq!(output(&plan, &graph, "crates"), "pgrx-c-macros");
        assert_eq!(output(&plan, &graph, "miri"), "true");
    }

    #[test]
    fn a_file_another_package_reads_reruns_that_package() {
        let graph = graph();
        // pgrx-c-macros' tests read pgrx-pg-sys's support module, but Cargo's graph has
        // pgrx-c-macros upstream of pgrx-pg-sys.
        let plan = Plan::new(&graph, Some(&files(&["pgrx-pg-sys/src/c_macros/support.rs"])));
        assert!(plan.affected.contains("pgrx-pg-sys"));
        assert_eq!(output(&plan, &graph, "crates"), "cargo-pgrx pgrx-c-macros");
        assert_eq!(output(&plan, &graph, "clang"), "true");
        assert_eq!(output(&plan, &graph, "bindgen_harness"), "false");
    }

    #[test]
    fn the_clang_jobs_follow_the_generator_and_the_bindings() {
        let graph = graph();
        for path in [
            "pgrx-pg-config/src/lib.rs",
            "pgrx-bindgen/src/build.rs",
            "pgrx-pg-sys/tests/oracle.rs",
            "pgrx-macros/src/lib.rs",
        ] {
            let plan = Plan::new(&graph, Some(&files(&[path])));
            assert_eq!(output(&plan, &graph, "clang"), "true", "{path}");
        }
        for path in ["pgrx/src/lib.rs", "pgrx-tests/src/lib.rs"] {
            let plan = Plan::new(&graph, Some(&files(&[path])));
            assert_eq!(output(&plan, &graph, "clang"), "false", "{path}");
        }
    }

    #[test]
    fn markdown_selects_packaging_only_for_published_readmes() {
        let graph = graph();
        let plan = Plan::new(&graph, Some(&files(&["README.md", "cargo-pgrx/README.md"])));
        assert_eq!(output(&plan, &graph, "pg"), "");
        assert_eq!(output(&plan, &graph, "crates"), "");
        assert_eq!(output(&plan, &graph, "lint"), "false");
        assert_eq!(output(&plan, &graph, "package"), "true");
        assert_eq!(output(&plan, &graph, "docsrs"), "false");
        assert_eq!(output(&plan, &graph, "beta"), "false");
        let plan = Plan::new(
            &graph,
            Some(&files(&["docs/src/x.md", "pgrx-examples/arrays/README.md", "CHANGELOG.md"])),
        );
        assert!(plan.outputs(&graph).iter().all(|(_, value)| value.is_empty() || value == "false"));
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
        assert_eq!(output(&plan, &graph, "package"), "true");
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

    #[test]
    fn reads_resolve_from_the_file_or_the_package_and_classify_test_code() {
        let mut graph = Graph::new(vec![node("a", "a", false, &[]), node("b", "b", false, &[])]);
        let tracked: BTreeSet<String> = [
            "a/src/lib.rs",
            "a/tests/it.rs",
            "b/data/x.h",
            "b/src/lib.rs",
            "b/src/support.rs",
            "b/tests/support/script.rs",
        ]
        .iter()
        .map(|file| (*file).to_owned())
        .collect();
        let sources = [
            (
                "a/src/lib.rs",
                concat!(
                    "/// let text = include_str!(\"../../b/src/lib.rs\");\n",
                    "const LIB: &str = include_str!(\"../../b/src/lib.rs\");\n",
                    "#[cfg(test)]\n#[path = \"../../b/tests/support/script.rs\"]\nmod script;\n",
                    "const OWN: &str = include_str!(\"../tests/it.rs\");\n",
                    "let name = format!(\"../{x}/b/src/support.rs\");\n",
                    "let missing = \"../../b/src/missing.rs\";\n",
                ),
            ),
            (
                "a/tests/it.rs",
                concat!(
                    "#[path = \"../../b/src/support.rs\"] mod support;\n",
                    "let headers = Path::new(env!(\"CARGO_MANIFEST_DIR\")).join(\"../b/data\");\n",
                ),
            ),
        ];
        graph.find_reads(&tracked, |file| {
            sources.iter().find(|(name, _)| *name == file).map(|(_, text)| (*text).to_owned())
        });
        let reads: Vec<(&str, &str, bool)> = graph
            .reads
            .iter()
            .map(|read| (read.path.as_str(), read.reader.as_str(), read.tests_only))
            .collect();
        assert_eq!(
            reads,
            [
                ("b/data", "a", true),
                // A doctest reads it, and so does the library itself.
                ("b/src/lib.rs", "a", false),
                ("b/src/support.rs", "a", true),
                ("b/tests/support/script.rs", "a", true),
            ]
        );
        let plan = Plan::new(&graph, Some(&files(&["b/data/x.h"])));
        assert_eq!(plan.changed, BTreeSet::from(["b".to_owned()]));
        assert_eq!(plan.tests_changed, BTreeSet::from(["a".to_owned()]));
    }

    #[test]
    fn resolve_stays_inside_the_workspace() {
        assert_eq!(resolve("a/tests", "../../b/./c.rs").as_deref(), Some("b/c.rs"));
        assert_eq!(resolve("a", "../../outside.rs"), None);
        assert_eq!(resolve("a", ".."), None);
    }
}
