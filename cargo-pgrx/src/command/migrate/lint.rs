//LICENSE Portions Copyright 2026-2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use super::MigrateContext;
use super::fragment::{
    FragmentId, MigrationFragment, ShippedFragments, load_fragments, prerequisites,
    topological_sort_fragments, unresolved_dependencies,
};
use super::sql::{ObjectRef, Verb, touched_objects};
use super::version::{existing_upgrade_scripts, upgrade_script_target};
use crate::CommandExecute;
use eyre::{Context, eyre};
use owo_colors::OwoColorize;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;

/// Git revision used as the comparison base for `--base`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct BaseRef(pub(crate) String);

impl FromStr for BaseRef {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl fmt::Display for BaseRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Git revision used for release-branch fragment identity checks via `--upstream`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct UpstreamRef(pub(crate) String);

impl FromStr for UpstreamRef {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_string()))
    }
}

impl fmt::Display for UpstreamRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(clap::Args, Debug)]
#[clap(author)]
pub(crate) struct Check {
    /// Package to check (see `cargo help pkgid`)
    #[clap(long, short)]
    pub(crate) package: Option<String>,

    /// Path to Cargo.toml
    #[clap(long, value_parser)]
    pub(crate) manifest_path: Option<PathBuf>,

    /// Scope fragment lints to changes since `<REF>`, and reject edits or deletions of upgrade scripts in `sql/`
    #[clap(long = "base", alias = "diff", value_name = "REF")]
    pub(crate) base: Option<BaseRef>,

    /// Require every unreleased fragment to exist byte-for-byte identically on `<REF>` (e.g. `origin/main`)
    #[clap(long, value_name = "REF")]
    pub(crate) upstream: Option<UpstreamRef>,

    /// Error if any fragment touches both released and unreleased objects, or unreleased objects from multiple prerequisite fragments
    #[clap(long)]
    pub(crate) deny_mixed_objects: bool,

    /// Diagnostic output format
    #[clap(long, value_enum, default_value_t = LintFormat::Text)]
    pub(crate) format: LintFormat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum, Default)]
pub(crate) enum LintFormat {
    #[default]
    Text,
    Json,
    Github,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
enum LintRule {
    DagCycle,
    UnresolvedDependency,
    UndeclaredDependency,
    MixedObjects,
    UpstreamDivergence,
    ReleasedScriptModified,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
struct DiagnosticIssue {
    file: PathBuf,
    filename: String,
    rule: LintRule,
    message: String,
}

fn verify_git_ref(dir: &Path, spec: &str) -> eyre::Result<()> {
    let out = Command::new("git")
        .args(["rev-parse", "--verify", "--quiet", spec])
        .current_dir(dir)
        .output()
        .wrap_err("failed to invoke git rev-parse")?;
    if !out.status.success() {
        return Err(eyre!("git ref `{spec}` could not be resolved"));
    }
    Ok(())
}

/// Names the fragments in `unreleased_dir` added or modified since they diverged from `base`.
fn changed_fragment_files(unreleased_dir: &Path, base: &BaseRef) -> eyre::Result<BTreeSet<String>> {
    if !unreleased_dir.exists() {
        return Ok(BTreeSet::new());
    }

    // `--relative` scopes the diff to the current directory and reports paths relative to it, which
    // holds however deep inside the repository the crate is.
    let diff_out = Command::new("git")
        .args(["diff", "--relative", "--name-only", "--no-renames", "--diff-filter=AM", "-z"])
        .arg(format!("{base}...HEAD"))
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

/// Reports any `<extname>--<prev>--<target>.sql` upgrade scripts modified or deleted since `base`.
fn modified_released_scripts(
    crate_dir: &Path,
    extname: &str,
    base: &BaseRef,
) -> eyre::Result<Vec<DiagnosticIssue>> {
    let diff_out = Command::new("git")
        .args([
            "diff",
            "--relative",
            "--name-status",
            "--no-renames",
            "--diff-filter=MD",
            "-z",
            &format!("{base}...HEAD"),
            "--",
            "sql",
        ])
        .current_dir(crate_dir)
        .output()
        .wrap_err("failed to invoke git diff for sql/")?;

    if !diff_out.status.success() {
        return Err(eyre!("git diff failed: {}", String::from_utf8_lossy(&diff_out.stderr)));
    }

    let text = String::from_utf8_lossy(&diff_out.stdout);
    let mut parts = text.split('\0').filter(|part| !part.is_empty());
    let mut issues = Vec::new();
    while let (Some(status), Some(rel_path)) = (parts.next(), parts.next()) {
        let Some(filename) = rel_path.strip_prefix("sql/") else {
            continue;
        };
        if filename.contains('/') {
            continue;
        }
        let Some(target) = upgrade_script_target(filename, extname) else {
            continue;
        };
        let action = if status == "D" { "deleted" } else { "modified" };
        issues.push(DiagnosticIssue {
            file: crate_dir.join(rel_path),
            filename: filename.to_string(),
            rule: LintRule::ReleasedScriptModified,
            message: format!(
                "Upgrade script `{filename}` (targeting `{target}`) was {action}; shipped upgrade scripts in `sql/` are immutable. Add new schema changes as a migration fragment instead."
            ),
        });
    }
    Ok(issues)
}

/// Verifies that every fragment in `fragments` exists at the same repository-relative path on
/// `upstream` with identical bytes.
fn upstream_divergences(
    unreleased_dir: &Path,
    fragments: &[MigrationFragment],
    upstream: &UpstreamRef,
) -> eyre::Result<Vec<DiagnosticIssue>> {
    if fragments.is_empty() {
        return Ok(Vec::new());
    }

    let prefix_out = Command::new("git")
        .args(["rev-parse", "--show-prefix"])
        .current_dir(unreleased_dir)
        .output()
        .wrap_err("failed to determine repository-relative path for unreleased fragments")?;
    if !prefix_out.status.success() {
        return Err(eyre!(
            "git rev-parse --show-prefix failed: {}",
            String::from_utf8_lossy(&prefix_out.stderr)
        ));
    }
    let prefix = String::from_utf8_lossy(&prefix_out.stdout);
    let prefix = prefix.trim();

    let mut ordered: Vec<&MigrationFragment> = fragments.iter().collect();
    ordered.sort_by(|a, b| a.filename.cmp(&b.filename));

    let mut issues = Vec::new();
    for fragment in ordered {
        let repo_path = format!("{prefix}{}", fragment.filename);
        let spec = format!("{upstream}:{repo_path}");
        let blob_out = Command::new("git")
            .args(["cat-file", "blob", &spec])
            .current_dir(unreleased_dir)
            .output()
            .wrap_err_with(|| format!("failed to read `{spec}` from git"))?;

        if !blob_out.status.success() {
            issues.push(DiagnosticIssue {
                file: fragment.path.clone(),
                filename: fragment.filename.clone(),
                rule: LintRule::UpstreamDivergence,
                message: format!(
                    "Fragment does not exist on `{upstream}` (`{repo_path}`); fragments on a release branch must keep the same filename as on `{upstream}`"
                ),
            });
            continue;
        }

        let local_bytes = fs::read(&fragment.path)
            .wrap_err_with(|| format!("failed to read `{}`", fragment.path.display()))?;
        if blob_out.stdout != local_bytes {
            issues.push(DiagnosticIssue {
                file: fragment.path.clone(),
                filename: fragment.filename.clone(),
                rule: LintRule::UpstreamDivergence,
                message: format!(
                    "Fragment differs from `{upstream}` (`{repo_path}`); fragments sharing a filename with `{upstream}` must be byte-for-byte identical"
                ),
            });
        }
    }

    Ok(issues)
}

struct LintConfig {
    deny_mixed_objects: bool,
    base_ref: Option<BaseRef>,
    upstream_ref: Option<UpstreamRef>,
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

fn lint_fragments(
    unreleased_dir: &Path,
    raw_fragments: Vec<MigrationFragment>,
    shipped: &ShippedFragments,
    changed: Option<&BTreeSet<String>>,
    deny_mixed_objects: bool,
) -> Vec<DiagnosticIssue> {
    let mut issues = Vec::new();
    for (fragment, dep) in unresolved_dependencies(&raw_fragments, shipped) {
        issues.push(DiagnosticIssue {
            file: fragment.path.clone(),
            filename: fragment.filename.clone(),
            rule: LintRule::UnresolvedDependency,
            message: format!(
                "Fragment declares '-- depends-on: {dep}', which was not found in unreleased fragments or assembled scripts in `sql/`"
            ),
        });
    }

    match topological_sort_fragments(raw_fragments) {
        Ok(fragments) => {
            issues.extend(lint_sorted(&fragments, changed, deny_mixed_objects));
        }
        Err(err) => {
            issues.push(DiagnosticIssue {
                file: unreleased_dir.to_path_buf(),
                filename: "unreleased".to_string(),
                rule: LintRule::DagCycle,
                message: err.to_string(),
            });
        }
    }

    issues
}

fn run_lint_checks(
    context: &MigrateContext,
    config: &LintConfig,
) -> eyre::Result<Vec<DiagnosticIssue>> {
    let crate_dir = context.manifest_path.parent().unwrap_or_else(|| Path::new("."));

    let mut issues = Vec::new();
    let changed = if let Some(ref base) = config.base_ref {
        verify_git_ref(crate_dir, &format!("{base}^{{commit}}"))?;
        issues.extend(modified_released_scripts(crate_dir, &context.extname, base)?);
        Some(changed_fragment_files(&context.unreleased_dir, base)?)
    } else {
        None
    };

    if let Some(ref upstream) = config.upstream_ref {
        verify_git_ref(crate_dir, &format!("{upstream}^{{tree}}"))?;
    }

    let scripts = existing_upgrade_scripts(&context.sql_dir, &context.extname)?;
    let shipped = ShippedFragments::from_scripts(
        scripts.iter().filter(|script| !script.target.is_prerelease()),
    )?;
    let raw_fragments = load_fragments(&context.unreleased_dir)?;

    if let Some(ref upstream) = config.upstream_ref {
        issues.extend(upstream_divergences(&context.unreleased_dir, &raw_fragments, upstream)?);
    }

    issues.extend(lint_fragments(
        &context.unreleased_dir,
        raw_fragments,
        &shipped,
        changed.as_ref(),
        config.deny_mixed_objects,
    ));

    Ok(issues)
}

/// Lints fragments in assembly order. With `changed`, an issue is reported only if it involves a
/// changed fragment, on either side.
fn lint_sorted(
    fragments: &[MigrationFragment],
    changed: Option<&BTreeSet<String>>,
    deny_mixed_objects: bool,
) -> Vec<DiagnosticIssue> {
    let touched: Vec<Vec<ObjectRef>> =
        fragments.iter().map(|fragment| touched_objects(&fragment.sql_content)).collect();

    // Every fragment that creates or replaces each object, wherever it falls in the order, so that
    // a fragment touching an object a later fragment creates is caught as surely as one touching an
    // earlier fragment's.
    let mut creators: BTreeMap<(&str, &str), Vec<(&ObjectRef, usize)>> = BTreeMap::new();
    for (idx, objects) in touched.iter().enumerate() {
        for object in objects.iter().filter(|o| matches!(o.verb, Verb::Create | Verb::Replace)) {
            creators.entry((object.kind, object.name.as_str())).or_default().push((object, idx));
        }
    }

    let is_changed =
        |idx: usize| changed.is_none_or(|changed| changed.contains(&fragments[idx].filename));
    let guaranteed_before = transitive_prerequisites(fragments);
    let mut issues = Vec::new();

    for (idx, fragment) in fragments.iter().enumerate() {
        let mut ordering_owners = BTreeSet::new();
        let mut unreleased_owners: BTreeSet<&FragmentId> = BTreeSet::new();
        let mut released_objects = Vec::new();

        for object in &touched[idx] {
            let same: Vec<(&ObjectRef, usize)> = creators
                .get(&(object.kind, object.name.as_str()))
                .into_iter()
                .flatten()
                .filter(|(creator, _)| creator.same_object(object))
                .copied()
                .collect();

            // Only a plain CREATE proves an object is new. CREATE OR REPLACE may be changing
            // something that already shipped, and a DROP before a later CREATE drops the released
            // object before recreating it.
            let created_in_unreleased = same.iter().any(|&(c, owner)| {
                c.verb == Verb::Create && (owner <= idx || object.verb == Verb::Alter)
            });
            if object.verb != Verb::Create && !created_in_unreleased {
                released_objects.push(object.to_string());
            }

            for &(creator, owner) in &same {
                if owner == idx || fragments[owner].id == fragment.id {
                    continue;
                }
                // An earlier creator has to stay earlier. A later one is a problem for an ALTER,
                // which needs the object to exist. A DROP before a later CREATE is how a released
                // object gets recreated, and two creations are reported from the later side.
                if owner < idx || object.verb == Verb::Alter {
                    ordering_owners.insert(owner);
                    if creator.verb == Verb::Create {
                        unreleased_owners.insert(&fragments[owner].id);
                    }
                }
            }
        }

        for &owner in &ordering_owners {
            if guaranteed_before[idx].contains(&owner) || !(is_changed(idx) || is_changed(owner)) {
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

        if deny_mixed_objects && is_changed(idx) && !unreleased_owners.is_empty() {
            let owners_str =
                unreleased_owners.iter().map(|id| id.to_string()).collect::<Vec<_>>().join(", ");
            if !released_objects.is_empty() {
                issues.push(DiagnosticIssue {
                    file: fragment.path.clone(),
                    filename: fragment.filename.clone(),
                    rule: LintRule::MixedObjects,
                    message: format!(
                        "Fragment mixes modifications to already-released objects ({}) with unreleased objects from fragment(s) [{owners_str}]",
                        released_objects.join(", ")
                    ),
                });
            } else if unreleased_owners.len() > 1 {
                issues.push(DiagnosticIssue {
                    file: fragment.path.clone(),
                    filename: fragment.filename.clone(),
                    rule: LintRule::MixedObjects,
                    message: format!(
                        "Fragment touches unreleased objects from multiple prerequisite fragments [{owners_str}]; split into one fragment per prerequisite so each can be backported independently"
                    ),
                });
            }
        }
    }

    issues
}

fn render_lint_diagnostics(issues: &[DiagnosticIssue], format: LintFormat) -> eyre::Result<()> {
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
        let config = LintConfig {
            deny_mixed_objects: self.deny_mixed_objects,
            base_ref: self.base,
            upstream_ref: self.upstream,
        };

        let issues = run_lint_checks(&context, &config)?;
        render_lint_diagnostics(&issues, self.format)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lint_with_shipped(
        files: &[(&str, &str)],
        shipped_sql: &str,
        deny_mixed_objects: bool,
    ) -> Vec<DiagnosticIssue> {
        let temp = tempfile::tempdir().unwrap();
        for (name, sql) in files {
            fs::write(temp.path().join(name), sql).unwrap();
        }
        let mut shipped = ShippedFragments::default();
        if !shipped_sql.is_empty() {
            shipped.record_sql(shipped_sql, Path::new("sql/my_ext--0.25.0--0.26.0.sql"));
        }
        let raw_fragments = load_fragments(temp.path()).unwrap();
        lint_fragments(temp.path(), raw_fragments, &shipped, None, deny_mixed_objects)
    }

    fn lint(files: &[(&str, &str)], deny_mixed_objects: bool) -> Vec<DiagnosticIssue> {
        lint_with_shipped(files, "", deny_mixed_objects)
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
            false,
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
            false,
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
            let issues = lint(&[("101.a.sql", a), ("102.b.sql", b)], false);
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
            false,
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
            let issues = lint(&[("101.a.sql", create), ("102.b.sql", touch)], false);
            assert_undeclared(&issues, "102.b.sql", "101");
        }
    }

    #[test]
    fn a_fragment_needing_a_later_one_is_caught() {
        // 98 sorts before 102, so without a declaration (or with a declaration naming an already-
        // shipped fragment 120 instead of 102) the index would be created first.
        for header in ["", "-- depends-on: 120\n"] {
            let issues = lint_with_shipped(
                &[
                    ("98.index.sql", &format!("{header}CREATE INDEX i ON greeting_log (x);")),
                    ("102.add_greeting_log.sql", "CREATE TABLE greeting_log (x int);"),
                ],
                "-- Fragment: 120.shipped.sql\n",
                false,
            );
            assert_undeclared(&issues, "98.index.sql", "102");
        }

        // If 120 never shipped either, both the unresolved `depends-on` and the missing 102 edge
        // are reported.
        let issues = lint(
            &[
                ("98.index.sql", "-- depends-on: 120\nCREATE INDEX i ON greeting_log (x);"),
                ("102.add_greeting_log.sql", "CREATE TABLE greeting_log (x int);"),
            ],
            false,
        );
        let rules: Vec<_> = issues.iter().map(|i| &i.rule).collect();
        assert_eq!(rules, [&LintRule::UnresolvedDependency, &LintRule::UndeclaredDependency]);

        let issues = lint(
            &[
                ("98.index.sql", "-- depends-on: 102\nCREATE INDEX i ON greeting_log (x);"),
                ("102.add_greeting_log.sql", "CREATE TABLE greeting_log (x int);"),
            ],
            false,
        );
        assert!(issues.is_empty(), "{issues:?}");
    }

    fn sorted(files: &[(&str, &str)]) -> Vec<MigrationFragment> {
        let temp = tempfile::tempdir().unwrap();
        for (name, sql) in files {
            fs::write(temp.path().join(name), sql).unwrap();
        }
        topological_sort_fragments(load_fragments(temp.path()).unwrap()).unwrap()
    }

    #[test]
    fn a_changed_fragment_is_reported_whichever_side_it_is_on() {
        let sql = "CREATE OR REPLACE FUNCTION f() RETURNS int;";
        let fragments = sorted(&[("98.new.sql", sql), ("102.existing.sql", sql)]);

        let changed = BTreeSet::from(["98.new.sql".to_string()]);
        assert_undeclared(
            &lint_sorted(&fragments, Some(&changed), false),
            "102.existing.sql",
            "98",
        );

        let unrelated = BTreeSet::from(["7.other.sql".to_string()]);
        assert!(lint_sorted(&fragments, Some(&unrelated), false).is_empty());
    }

    #[test]
    fn a_transitive_dependency_is_enough() {
        let issues = lint(
            &[
                ("101.a.sql", "CREATE TABLE t (id int);"),
                ("102.b.sql", "-- depends-on: 101\nALTER TABLE t ADD COLUMN x int;"),
                ("103.c.sql", "-- depends-on: 102\nALTER TABLE t ADD COLUMN y int;"),
            ],
            false,
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
            true,
        );
        assert!(issues.is_empty(), "{issues:?}");
    }

    #[test]
    fn replacing_the_same_function_twice_needs_an_order() {
        let sql = "CREATE OR REPLACE FUNCTION released() RETURNS int;";
        let issues = lint(&[("102.b.sql", sql), ("101.a.sql", sql)], false);
        assert_undeclared(&issues, "102.b.sql", "101");
    }

    #[test]
    fn fragments_sharing_an_id_are_ordered_by_filename() {
        let issues = lint(
            &[
                ("101.a.sql", "CREATE TABLE t (id int);"),
                ("101.b.sql", "ALTER TABLE t ADD COLUMN x int;"),
            ],
            false,
        );
        assert!(issues.is_empty(), "{issues:?}");
    }

    #[test]
    fn reports_cycles() {
        let issues = lint(
            &[("101.a.sql", "-- depends-on: 102\n"), ("102.b.sql", "-- depends-on: 101\n")],
            false,
        );
        assert_eq!(issues.len(), 1, "{issues:?}");
        assert_eq!(issues[0].rule, LintRule::DagCycle);
    }

    #[test]
    fn deny_mixed_objects_matrix() {
        struct Case<'a> {
            name: &'a str,
            files: &'a [(&'a str, &'a str)],
            expected_substring: Option<&'a str>,
        }

        let cases = [
            Case {
                name: "mixes released table alter with unreleased table alter",
                files: &[
                    ("101.create.sql", "CREATE TABLE unreleased_table (id int);"),
                    (
                        "102.mixed.sql",
                        "-- depends-on: 101\nALTER TABLE unreleased_table ADD COLUMN a int;\nALTER TABLE released_table ADD COLUMN b text;",
                    ),
                ],
                expected_substring: Some(
                    "mixes modifications to already-released objects (TABLE released_table)",
                ),
            },
            Case {
                name: "replacing a released function counts as touching released objects",
                files: &[
                    ("101.a.sql", "CREATE TABLE unreleased_t (id int);"),
                    (
                        "102.b.sql",
                        "-- depends-on: 101\nCREATE OR REPLACE FUNCTION released_fn() RETURNS int;\nALTER TABLE unreleased_t ADD COLUMN x int;",
                    ),
                ],
                expected_substring: Some("FUNCTION released_fn()"),
            },
            Case {
                name: "touches unreleased objects from multiple prerequisite fragments",
                files: &[
                    ("101.a.sql", "CREATE TABLE table_a (id int);"),
                    ("102.b.sql", "CREATE TABLE table_b (id int);"),
                    (
                        "103.both.sql",
                        "-- depends-on: 101, 102\nALTER TABLE table_a ADD COLUMN x int;\nALTER TABLE table_b ADD COLUMN y int;",
                    ),
                ],
                expected_substring: Some("multiple prerequisite fragments [101, 102]"),
            },
            Case {
                name: "own newly created objects plus a single prerequisite are allowed",
                files: &[
                    ("101.create.sql", "CREATE TABLE a (id int);"),
                    (
                        "102.own.sql",
                        "-- depends-on: 101\nCREATE TABLE b (id int);\nALTER TABLE b ADD COLUMN x int;\nALTER TABLE a ADD COLUMN y int;",
                    ),
                ],
                expected_substring: None,
            },
            Case {
                name: "multiple files sharing one PR fragment ID count as one prerequisite",
                files: &[
                    ("101.part1.sql", "CREATE TABLE a (id int);"),
                    ("101.part2.sql", "CREATE TABLE b (id int);"),
                    (
                        "102.use_both.sql",
                        "-- depends-on: 101\nALTER TABLE a ADD COLUMN x int;\nALTER TABLE b ADD COLUMN y int;",
                    ),
                ],
                expected_substring: None,
            },
            Case {
                name: "linear CREATE -> CREATE OR REPLACE -> ALTER on one unreleased function is allowed",
                files: &[
                    ("101.create.sql", "CREATE FUNCTION f() RETURNS int;"),
                    (
                        "102.replace.sql",
                        "-- depends-on: 101\nCREATE OR REPLACE FUNCTION f() RETURNS int;",
                    ),
                    ("103.alter.sql", "-- depends-on: 102\nALTER FUNCTION f() PARALLEL SAFE;"),
                ],
                expected_substring: None,
            },
            Case {
                name: "ordered CREATE OR REPLACE on the same released function is allowed",
                files: &[
                    ("101.first.sql", "CREATE OR REPLACE FUNCTION released_fn() RETURNS int;"),
                    (
                        "102.second.sql",
                        "-- depends-on: 101\nCREATE OR REPLACE FUNCTION released_fn() RETURNS int;",
                    ),
                ],
                expected_substring: None,
            },
        ];

        for case in cases {
            let issues = lint(case.files, true);
            match case.expected_substring {
                Some(sub) => {
                    assert_eq!(issues.len(), 1, "{}: {issues:?}", case.name);
                    assert_eq!(issues[0].rule, LintRule::MixedObjects, "{}", case.name);
                    assert!(
                        issues[0].message.contains(sub),
                        "{}: {}",
                        case.name,
                        issues[0].message
                    );
                }
                None => assert!(issues.is_empty(), "{}: {issues:?}", case.name),
            }
        }
    }
}
