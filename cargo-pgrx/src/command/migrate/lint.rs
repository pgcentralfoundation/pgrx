//LICENSE Portions Copyright 2026-2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use super::MigrateContext;
use super::fragment::{
    MigrationFragment, load_fragments, prerequisites, topological_sort_fragments,
};
use super::sql::{ObjectRef, Verb, touched_objects};
use crate::CommandExecute;
use eyre::{Context, eyre};
use owo_colors::OwoColorize;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(clap::Args, Debug)]
#[clap(author)]
pub(crate) struct Check {
    /// Package to check (see `cargo help pkgid`)
    #[clap(long, short)]
    pub(crate) package: Option<String>,

    /// Path to Cargo.toml
    #[clap(long, value_parser)]
    pub(crate) manifest_path: Option<PathBuf>,

    /// Only lint fragments added or modified relative to <REF> (e.g. `main` or `HEAD~1`)
    #[clap(long = "base", alias = "diff")]
    pub(crate) base: Option<String>,

    /// Error if any fragment touches both existing (released) database objects and unreleased objects
    #[clap(long)]
    pub(crate) deny_mixed_objects: bool,

    /// Diagnostic output format
    #[clap(long, value_enum, default_value_t = LintFormat::Text)]
    pub(crate) format: LintFormat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum, Default)]
pub enum LintFormat {
    #[default]
    Text,
    Json,
    Github,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LintRule {
    DagCycle,
    UndeclaredDependency,
    MixedObjects,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct DiagnosticIssue {
    pub file: PathBuf,
    pub filename: String,
    pub rule: LintRule,
    pub message: String,
}

/// Names the fragments in `unreleased_dir` added or modified since they diverged from `base_ref`.
fn changed_fragment_files(unreleased_dir: &Path, base_ref: &str) -> eyre::Result<BTreeSet<String>> {
    // `--relative` scopes the diff to the current directory and reports paths relative to it, which
    // holds however deep inside the repository the crate is.
    let diff_out = Command::new("git")
        .args(["diff", "--relative", "--name-only", "--no-renames", "--diff-filter=AM", "-z"])
        .arg(format!("{base_ref}...HEAD"))
        .current_dir(unreleased_dir)
        .output()
        .wrap_err("failed to invoke git diff")?;

    if !diff_out.status.success() {
        return Err(eyre!("git diff failed: {}", String::from_utf8_lossy(&diff_out.stderr)));
    }

    Ok(String::from_utf8_lossy(&diff_out.stdout)
        .split('\0')
        .filter(|path| path.ends_with(".sql") && !path.contains('/'))
        .map(str::to_string)
        .collect())
}

pub struct LintConfig {
    pub deny_mixed_objects: bool,
    pub base_ref: Option<String>,
}

/// For each fragment of a topologically sorted list, every fragment that is guaranteed to run
/// before it, directly declared or not.
fn transitive_prerequisites(sorted: &[MigrationFragment]) -> Vec<BTreeSet<usize>> {
    let mut transitive: Vec<BTreeSet<usize>> = Vec::with_capacity(sorted.len());
    for direct in prerequisites(sorted) {
        let inherited: Vec<usize> =
            direct.iter().flat_map(|&idx| transitive[idx].iter().copied()).collect();
        let mut all = direct;
        all.extend(inherited);
        transitive.push(all);
    }
    transitive
}

pub fn run_lint_checks(
    unreleased_dir: &Path,
    config: &LintConfig,
) -> eyre::Result<Vec<DiagnosticIssue>> {
    let fragments = match topological_sort_fragments(load_fragments(unreleased_dir)?) {
        Ok(fragments) => fragments,
        Err(err) => {
            return Ok(vec![DiagnosticIssue {
                file: unreleased_dir.to_path_buf(),
                filename: "unreleased".to_string(),
                rule: LintRule::DagCycle,
                message: err.to_string(),
            }]);
        }
    };

    // Every fragment feeds the catalog below, but only changed ones are reported.
    let changed = config
        .base_ref
        .as_deref()
        .map(|base_ref| changed_fragment_files(unreleased_dir, base_ref))
        .transpose()?;

    let guaranteed_before = transitive_prerequisites(&fragments);
    let mut issues = Vec::new();

    // Objects created so far, by kind and name, and the index of the fragment creating each.
    // Walking in assembly order means a fragment can only depend on one that runs before it, so a
    // fragment that drops an object a later fragment recreates is not told to run after it.
    let mut created: BTreeMap<(&str, String), Vec<(ObjectRef, usize)>> = BTreeMap::new();

    for (idx, fragment) in fragments.iter().enumerate() {
        let mut owners = BTreeSet::new();
        let mut released_objects = Vec::new();

        for object in touched_objects(&fragment.sql_content) {
            let same_name = created.entry((object.kind, object.name.clone())).or_default();
            let earlier: Vec<usize> = same_name
                .iter()
                .filter(|(other, _)| other.same_object(&object))
                .map(|&(_, owner)| owner)
                .collect();

            if earlier.is_empty() && object.verb != Verb::Create {
                released_objects.push(object.to_string());
            }
            owners.extend(earlier.into_iter().filter(|&owner| fragments[owner].id != fragment.id));

            if object.verb == Verb::Create {
                same_name.push((object, idx));
            }
        }

        if changed.as_ref().is_some_and(|changed| !changed.contains(&fragment.filename)) {
            continue;
        }

        for &owner in &owners {
            if guaranteed_before[idx].contains(&owner) {
                continue;
            }
            let dep_id = &fragments[owner].id;
            issues.push(DiagnosticIssue {
                file: fragment.path.clone(),
                filename: fragment.filename.clone(),
                rule: LintRule::UndeclaredDependency,
                message: format!(
                    "Fragment touches unreleased object(s) from fragment `{dep_id}` but does not declare '-- depends-on: {dep_id}'"
                ),
            });
        }

        if config.deny_mixed_objects && !released_objects.is_empty() && !owners.is_empty() {
            let owners_str = owners
                .iter()
                .map(|&owner| fragments[owner].id.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            issues.push(DiagnosticIssue {
                file: fragment.path.clone(),
                filename: fragment.filename.clone(),
                rule: LintRule::MixedObjects,
                message: format!(
                    "Fragment mixes modifications to already-released objects ({}) with unreleased objects from fragment(s) [{owners_str}]",
                    released_objects.join(", ")
                ),
            });
        }
    }

    Ok(issues)
}

pub fn render_lint_diagnostics(issues: &[DiagnosticIssue], format: LintFormat) -> eyre::Result<()> {
    match format {
        LintFormat::Text => {
            if issues.is_empty() {
                println!("{} All migration fragments passed lint checks.", "✅".green());
            } else {
                for issue in issues {
                    println!("{} {}: {}", "❌".red(), issue.filename.bold(), issue.message);
                }
                println!(
                    "\n{} Migration fragment lint failed with {} error(s).",
                    "❌".red(),
                    issues.len()
                );
            }
        }
        LintFormat::Github => {
            for issue in issues {
                println!("::error file={}::{}", issue.file.display(), issue.message);
                eprintln!("❌ {}: {}", issue.filename, issue.message);
            }
            if issues.is_empty() {
                println!("✅ All migration fragments passed lint checks.");
            } else {
                eprintln!("\n❌ Migration fragment lint failed with {} error(s).", issues.len());
            }
        }
        LintFormat::Json => {
            println!("{}", serde_json::to_string_pretty(issues)?);
        }
    }

    if !issues.is_empty() {
        eyre::bail!("migration lint failed with {} error(s)", issues.len());
    }

    Ok(())
}

impl CommandExecute for Check {
    #[tracing::instrument(level = "error", skip(self))]
    fn execute(self) -> eyre::Result<()> {
        let context = MigrateContext::load(self.package.as_deref(), self.manifest_path.as_deref())?;
        let config =
            LintConfig { deny_mixed_objects: self.deny_mixed_objects, base_ref: self.base };

        let issues = run_lint_checks(&context.unreleased_dir, &config)?;
        render_lint_diagnostics(&issues, self.format)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NO_FLAGS: LintConfig = LintConfig { deny_mixed_objects: false, base_ref: None };

    fn lint(files: &[(&str, &str)], config: &LintConfig) -> Vec<DiagnosticIssue> {
        let temp = tempfile::tempdir().unwrap();
        for (name, sql) in files {
            std::fs::write(temp.path().join(name), sql).unwrap();
        }
        run_lint_checks(temp.path(), config).unwrap()
    }

    fn assert_undeclared(issues: &[DiagnosticIssue], filename: &str, owner: &str) {
        assert_eq!(issues.len(), 1, "{issues:?}");
        assert_eq!(issues[0].rule, LintRule::UndeclaredDependency);
        assert_eq!(issues[0].filename, filename);
        assert!(
            issues[0].message.contains(&format!("from fragment `{owner}`")),
            "{}",
            issues[0].message
        );
    }

    #[test]
    fn detects_undeclared_dependency() {
        let issues = lint(
            &[
                ("101.create.sql", "CREATE TABLE t1 (id int);"),
                ("102.modify.sql", "ALTER TABLE t1 ADD COLUMN name text;"),
            ],
            &NO_FLAGS,
        );
        assert_undeclared(&issues, "102.modify.sql", "101");
    }

    #[test]
    fn allows_declared_dependency() {
        let issues = lint(
            &[
                ("101.create.sql", "CREATE TABLE t1 (id int);"),
                ("102.modify.sql", "-- depends-on: 101\nALTER TABLE t1 ADD COLUMN name text;"),
            ],
            &NO_FLAGS,
        );
        assert!(issues.is_empty(), "{issues:?}");
    }

    #[test]
    fn independent_objects_do_not_collide() {
        for (a, b) in [
            ("CREATE TABLE IF NOT EXISTS a (id int);", "CREATE TABLE IF NOT EXISTS b (id int);"),
            ("CREATE MATERIALIZED VIEW a AS SELECT 1;", "CREATE MATERIALIZED VIEW b AS SELECT 2;"),
            ("CREATE UNIQUE INDEX a ON t1 (x);", "CREATE UNIQUE INDEX b ON t2 (y);"),
            ("CREATE FUNCTION f(int) RETURNS int;", "CREATE FUNCTION f(text) RETURNS int;"),
        ] {
            let issues = lint(&[("101.a.sql", a), ("102.b.sql", b)], &NO_FLAGS);
            assert!(issues.is_empty(), "{a} / {b}: {issues:?}");
        }
    }

    #[test]
    fn statements_inside_function_bodies_are_not_objects() {
        let body = "AS $$ BEGIN PERFORM 1; CREATE TABLE scratch (id int); END $$ LANGUAGE plpgsql;";
        let issues = lint(
            &[
                ("101.a.sql", &format!("CREATE FUNCTION g() RETURNS void {body}")),
                ("102.b.sql", &format!("CREATE FUNCTION h() RETURNS void {body}")),
            ],
            &NO_FLAGS,
        );
        assert!(issues.is_empty(), "{issues:?}");
    }

    #[test]
    fn finds_dependencies_through_awkward_spellings() {
        for (create, touch) in [
            ("CREATE TABLE IF NOT EXISTS t (id int);", "ALTER TABLE t ADD COLUMN x int;"),
            ("CREATE TABLE t (id int);", "ALTER TABLE ONLY t ADD COLUMN x int;"),
            ("CREATE TABLE t (id int);", "CREATE INDEX i ON t (id);"),
            ("CREATE FUNCTION f(a numeric(10,2)) RETURNS int;", "DROP FUNCTION f(numeric);"),
            ("CREATE FUNCTION f(a integer) RETURNS int;", "DROP FUNCTION f(int);"),
            ("CREATE FUNCTION s.f() RETURNS int;", "DROP FUNCTION f();"),
            ("CREATE FUNCTION f(int) RETURNS int;", "DROP FUNCTION f;"),
            ("COMMENT ON TABLE t IS 'a -- b';\nCREATE TABLE u (id int);", "DROP TABLE u;"),
        ] {
            let issues = lint(&[("101.a.sql", create), ("102.b.sql", touch)], &NO_FLAGS);
            assert_undeclared(&issues, "102.b.sql", "101");
        }
    }

    #[test]
    fn a_transitive_dependency_is_enough() {
        let issues = lint(
            &[
                ("101.a.sql", "CREATE TABLE t (id int);"),
                ("102.b.sql", "-- depends-on: 101\nALTER TABLE t ADD COLUMN x int;"),
                ("103.c.sql", "-- depends-on: 102\nALTER TABLE t ADD COLUMN y int;"),
            ],
            &NO_FLAGS,
        );
        assert!(issues.is_empty(), "{issues:?}");
    }

    #[test]
    fn a_drop_followed_by_a_recreate_is_already_ordered() {
        // Asking 101 to depend on 102 would run the DROP last and lose the function.
        let issues = lint(
            &[
                ("101.a.sql", "DROP FUNCTION f(int);"),
                ("102.b.sql", "CREATE FUNCTION f(int) RETURNS text;"),
            ],
            &NO_FLAGS,
        );
        assert!(issues.is_empty(), "{issues:?}");
    }

    #[test]
    fn replacing_the_same_function_twice_needs_an_order() {
        let sql = "CREATE OR REPLACE FUNCTION released() RETURNS int;";
        let issues = lint(&[("102.b.sql", sql), ("101.a.sql", sql)], &NO_FLAGS);
        assert_undeclared(&issues, "102.b.sql", "101");
    }

    #[test]
    fn fragments_sharing_an_id_are_ordered_by_filename() {
        let issues = lint(
            &[
                ("101.a.sql", "CREATE TABLE t (id int);"),
                ("101.b.sql", "ALTER TABLE t ADD COLUMN x int;"),
            ],
            &NO_FLAGS,
        );
        assert!(issues.is_empty(), "{issues:?}");
    }

    #[test]
    fn reports_cycles() {
        let issues = lint(
            &[("101.a.sql", "-- depends-on: 102\n"), ("102.b.sql", "-- depends-on: 101\n")],
            &NO_FLAGS,
        );
        assert_eq!(issues.len(), 1, "{issues:?}");
        assert_eq!(issues[0].rule, LintRule::DagCycle);
    }

    #[test]
    fn detects_mixed_objects_when_flag_enabled() {
        let config = LintConfig { deny_mixed_objects: true, base_ref: None };
        let issues = lint(
            &[
                ("101.create.sql", "CREATE TABLE unreleased_table (id int);"),
                (
                    "102.mixed.sql",
                    "-- depends-on: 101\nALTER TABLE unreleased_table ADD COLUMN a int;\nALTER TABLE released_table ADD COLUMN b text;",
                ),
            ],
            &config,
        );
        assert_eq!(issues.len(), 1, "{issues:?}");
        assert_eq!(issues[0].rule, LintRule::MixedObjects);
        assert!(
            issues[0]
                .message
                .contains("mixes modifications to already-released objects (TABLE released_table)")
        );
    }

    #[test]
    fn a_fragments_own_objects_are_not_released() {
        let config = LintConfig { deny_mixed_objects: true, base_ref: None };
        let issues = lint(
            &[
                ("101.create.sql", "CREATE TABLE a (id int);"),
                (
                    "102.own.sql",
                    "-- depends-on: 101\nCREATE TABLE b (id int);\nALTER TABLE b ADD COLUMN x int;\nALTER TABLE a ADD COLUMN y int;",
                ),
            ],
            &config,
        );
        assert!(issues.is_empty(), "{issues:?}");
    }
}
