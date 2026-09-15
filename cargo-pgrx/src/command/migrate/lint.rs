//LICENSE Portions Copyright 2026-2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use super::fragment::{
    FragmentDependency, FragmentId, MigrationFragment, topological_sort_fragments,
};
use super::resolve_unreleased_dir;
use crate::CommandExecute;
use crate::manifest::get_package_manifest;
use eyre::{Context, eyre};
use owo_colors::OwoColorize;
use regex::Regex;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::LazyLock;

static LINE_COMMENT_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"--[^\n]*").unwrap());
static BLOCK_COMMENT_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"/\*[\s\S]*?\*/").unwrap());
static BACKSLASH_CMD_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\\\w+[^\n]*").unwrap());
static STMT_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)^(CREATE(?:\s+OR\s+REPLACE)?|ALTER|DROP)\s+([A-Z\s]+?)\s+(?:IF\s+EXISTS\s+)?([^\s(]+)(?:\s*\((.*?)\))?",
    )
    .unwrap()
});
static DEFAULT_ARG_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)\s+(DEFAULT|=)\s+.*$").unwrap());

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

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    clap::ValueEnum,
    Default,
    serde::Serialize,
    serde::Deserialize
)]
#[serde(rename_all = "lowercase")]
pub enum LintFormat {
    #[default]
    Text,
    Json,
    Github,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LintRule {
    DagCycle,
    UndeclaredDependency,
    MixedObjects,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DiagnosticIssue {
    pub file: PathBuf,
    pub filename: String,
    pub rule: LintRule,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SqlStatement {
    pub verb: String,
    pub signature: String,
    pub raw: String,
}

/// Extracts (verb, normalized_signature, raw_stmt) from SQL content.
pub fn extract_statements_and_objects(content: &str) -> Vec<SqlStatement> {
    let no_line_comments = LINE_COMMENT_RE.replace_all(content, "");
    let no_block_comments = BLOCK_COMMENT_RE.replace_all(&no_line_comments, "");
    let cleaned = BACKSLASH_CMD_RE.replace_all(&no_block_comments, "");

    let mut statements = Vec::new();
    for piece in cleaned.split(';') {
        let stmt = piece.split_whitespace().collect::<Vec<_>>().join(" ");
        if stmt.is_empty() {
            continue;
        }

        if let Some(caps) = STMT_RE.captures(&stmt) {
            let verb_match = caps.get(1).map(|m| m.as_str()).unwrap_or("");
            let obj_type_match = caps.get(2).map(|m| m.as_str()).unwrap_or("");
            let name_match = caps.get(3).map(|m| m.as_str()).unwrap_or("");
            let args_match = caps.get(4).map(|m| m.as_str());

            let verb = verb_match.split_whitespace().collect::<Vec<_>>().join(" ").to_uppercase();
            let obj_type =
                obj_type_match.split_whitespace().collect::<Vec<_>>().join(" ").to_uppercase();
            let name = name_match.replace('"', "").to_lowercase();

            let signature = if let Some(args) = args_match
                && matches!(obj_type.as_str(), "FUNCTION" | "AGGREGATE" | "PROCEDURE")
            {
                let mut arg_types = Vec::new();
                for raw_arg in args.split(',') {
                    let stripped = DEFAULT_ARG_RE.replace(raw_arg.trim(), "");
                    let tokens: Vec<&str> = stripped.split_whitespace().collect();
                    if let Some(&last) = tokens.last() {
                        let cleaned_type = last.replace('"', "").to_lowercase();
                        if !cleaned_type.is_empty() {
                            arg_types.push(cleaned_type);
                        }
                    }
                }
                format!("{obj_type} {name}({})", arg_types.join(", "))
            } else {
                format!("{obj_type} {name}")
            };

            statements.push(SqlStatement { verb, signature, raw: stmt });
        }
    }

    statements
}

/// Catalogs all objects created or replaced in unreleased migration fragments.
pub fn catalog_unreleased_objects(fragments: &[MigrationFragment]) -> BTreeMap<String, FragmentId> {
    let mut catalog = BTreeMap::new();
    for fragment in fragments {
        for stmt in extract_statements_and_objects(&fragment.sql_content) {
            let v = stmt.verb.to_uppercase();
            if v.contains("CREATE") || v.contains("REPLACE") {
                catalog.entry(stmt.signature).or_insert_with(|| fragment.id.clone());
            }
        }
    }
    catalog
}

/// Identifies fragment files added or modified in git relative to `base_ref`.
pub fn get_changed_fragment_files(
    repo_root: &Path,
    unreleased_dir: &Path,
    base_ref: &str,
) -> eyre::Result<Vec<PathBuf>> {
    let merge_base_out =
        Command::new("git").args(["merge-base", base_ref, "HEAD"]).current_dir(repo_root).output();

    let diff_ref = match merge_base_out {
        Ok(out) if out.status.success() => {
            let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if s.is_empty() { base_ref.to_string() } else { s }
        }
        _ => base_ref.to_string(),
    };

    let diff_out = Command::new("git")
        .args(["diff", "--name-only", "--diff-filter=AM", &format!("{diff_ref}...HEAD")])
        .current_dir(repo_root)
        .output()
        .wrap_err("failed to invoke git diff")?;

    if !diff_out.status.success() {
        return Err(eyre!("git diff failed: {}", String::from_utf8_lossy(&diff_out.stderr)));
    }

    let stdout = String::from_utf8_lossy(&diff_out.stdout);
    let mut changed = Vec::new();
    let unreleased_canonical =
        unreleased_dir.canonicalize().unwrap_or_else(|_| unreleased_dir.to_path_buf());

    for line in stdout.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || !trimmed.ends_with(".sql") {
            continue;
        }
        let full_path = repo_root.join(trimmed);
        if let Ok(canon) = full_path.canonicalize() {
            if canon.parent() == Some(&unreleased_canonical) {
                changed.push(canon);
            }
        } else if full_path.parent() == Some(unreleased_dir) {
            changed.push(full_path);
        }
    }

    Ok(changed)
}

pub struct LintConfig {
    pub deny_mixed_objects: bool,
    pub base_ref: Option<String>,
}

pub fn run_lint_checks(
    repo_root: &Path,
    unreleased_dir: &Path,
    config: &LintConfig,
) -> eyre::Result<Vec<DiagnosticIssue>> {
    if !unreleased_dir.exists() {
        return Ok(Vec::new());
    }

    let mut raw_fragments = Vec::new();
    for entry in std::fs::read_dir(unreleased_dir)
        .wrap_err_with(|| format!("failed to read directory `{}`", unreleased_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if name.starts_with('.') || !name.ends_with(".sql") {
            continue;
        }
        raw_fragments.push(super::fragment::load_fragment(&path)?);
    }

    let mut issues = Vec::new();

    // 1. DAG cycle check
    if let Err(err) = topological_sort_fragments(raw_fragments.clone()) {
        issues.push(DiagnosticIssue {
            file: unreleased_dir.to_path_buf(),
            filename: "unreleased".to_string(),
            rule: LintRule::DagCycle,
            message: err.to_string(),
        });
        return Ok(issues);
    }

    let catalog = catalog_unreleased_objects(&raw_fragments);

    // 2. Determine which fragments to lint (filtered by git diff if base_ref provided)
    let fragments_to_lint: Vec<&MigrationFragment> = if let Some(base_ref) = &config.base_ref {
        let changed_paths = get_changed_fragment_files(repo_root, unreleased_dir, base_ref)?;
        let changed_filenames: BTreeSet<String> = changed_paths
            .into_iter()
            .filter_map(|p| p.file_name().and_then(|f| f.to_str()).map(|s| s.to_string()))
            .collect();

        raw_fragments.iter().filter(|f| changed_filenames.contains(&f.filename)).collect()
    } else {
        raw_fragments.iter().collect()
    };

    // 3. Lint each targeted fragment
    for fragment in fragments_to_lint {
        let statements = extract_statements_and_objects(&fragment.sql_content);
        let mut detected_unreleased_deps = BTreeSet::new();
        let mut touched_released_objects = Vec::new();

        for stmt in &statements {
            if let Some(owner_id) = catalog.get(&stmt.signature) {
                if owner_id != &fragment.id {
                    detected_unreleased_deps.insert(owner_id.clone());
                }
            } else {
                let v = stmt.verb.to_uppercase();
                if v.contains("ALTER") || v.contains("DROP") || v.contains("REPLACE") {
                    touched_released_objects.push(stmt.signature.clone());
                }
            }
        }

        // Check for undeclared dependencies on unreleased objects
        for dep_id in &detected_unreleased_deps {
            let declared = fragment.dependencies.iter().any(|d| match d {
                FragmentDependency::Id(id) => id == dep_id,
                FragmentDependency::Filename(fname) => {
                    super::fragment::parse_fragment_id(fname) == *dep_id
                }
            });

            if !declared {
                issues.push(DiagnosticIssue {
                    file: fragment.path.clone(),
                    filename: fragment.filename.clone(),
                    rule: LintRule::UndeclaredDependency,
                    message: format!(
                        "Fragment touches unreleased object(s) from fragment `{dep_id}` but does not declare '-- depends-on: {dep_id}'"
                    ),
                });
            }
        }

        // Check for mixing released and unreleased objects
        if config.deny_mixed_objects
            && !touched_released_objects.is_empty()
            && !detected_unreleased_deps.is_empty()
        {
            let owners_str = detected_unreleased_deps
                .iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            issues.push(DiagnosticIssue {
                file: fragment.path.clone(),
                filename: fragment.filename.clone(),
                rule: LintRule::MixedObjects,
                message: format!(
                    "Fragment mixes modifications to already-released objects ({}) with unreleased objects from fragment(s) [{owners_str}]",
                    touched_released_objects.join(", ")
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
        let features = clap_cargo::Features::default();
        let (package_manifest, package_manifest_path) = get_package_manifest(
            &features,
            self.package.as_deref(),
            self.manifest_path.as_deref(),
        )?;

        let crate_dir = package_manifest_path.parent().unwrap_or_else(|| Path::new("."));
        let unreleased_dir = resolve_unreleased_dir(&package_manifest, &package_manifest_path);

        let config =
            LintConfig { deny_mixed_objects: self.deny_mixed_objects, base_ref: self.base };

        let issues = run_lint_checks(crate_dir, &unreleased_dir, &config)?;
        render_lint_diagnostics(&issues, self.format)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_statements_and_signatures() {
        let sql = r#"
-- comment
\echo loading
CREATE FUNCTION my_func(a int, b text DEFAULT 'x') RETURNS void AS $$ $$ LANGUAGE sql;
CREATE OR REPLACE AGGREGATE my_agg(integer) (SFUNC = int4_add);
ALTER TABLE "Customers" ADD COLUMN age int;
DROP TYPE IF EXISTS status_t;
"#;
        let stmts = extract_statements_and_objects(sql);
        assert_eq!(stmts.len(), 4);

        assert_eq!(stmts[0].verb, "CREATE");
        assert_eq!(stmts[0].signature, "FUNCTION my_func(int, text)");

        assert_eq!(stmts[1].verb, "CREATE OR REPLACE");
        assert_eq!(stmts[1].signature, "AGGREGATE my_agg(integer)");

        assert_eq!(stmts[2].verb, "ALTER");
        assert_eq!(stmts[2].signature, "TABLE customers");

        assert_eq!(stmts[3].verb, "DROP");
        assert_eq!(stmts[3].signature, "TYPE status_t");
    }

    #[test]
    fn catalogs_unreleased_objects_correctly() {
        let f1 = MigrationFragment {
            path: PathBuf::from("101.sql"),
            filename: "101.sql".to_string(),
            id: FragmentId::Numeric(101),
            dependencies: vec![],
            sql_content:
                "CREATE TABLE t1 (id int); CREATE FUNCTION f1() RETURNS void AS $$ $$ LANGUAGE sql;"
                    .to_string(),
        };

        let catalog = catalog_unreleased_objects(&[f1]);
        assert_eq!(catalog.get("TABLE t1"), Some(&FragmentId::Numeric(101)));
        assert_eq!(catalog.get("FUNCTION f1()"), Some(&FragmentId::Numeric(101)));
    }

    #[test]
    fn detects_undeclared_dependency() {
        let temp = tempfile::tempdir().unwrap();
        let unreleased = temp.path().join("sql/unreleased");
        std::fs::create_dir_all(&unreleased).unwrap();

        std::fs::write(unreleased.join("101.create.sql"), "CREATE TABLE t1 (id int);").unwrap();

        // 102 modifies t1 without declaring dependency on 101
        std::fs::write(unreleased.join("102.modify.sql"), "ALTER TABLE t1 ADD COLUMN name text;")
            .unwrap();

        let config = LintConfig { deny_mixed_objects: false, base_ref: None };

        let issues = run_lint_checks(temp.path(), &unreleased, &config).unwrap();
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].rule, LintRule::UndeclaredDependency);
        assert!(issues[0].message.contains("touches unreleased object(s) from fragment `101`"));
    }

    #[test]
    fn allows_declared_dependency() {
        let temp = tempfile::tempdir().unwrap();
        let unreleased = temp.path().join("sql/unreleased");
        std::fs::create_dir_all(&unreleased).unwrap();

        std::fs::write(unreleased.join("101.create.sql"), "CREATE TABLE t1 (id int);").unwrap();

        std::fs::write(
            unreleased.join("102.modify.sql"),
            "-- depends-on: 101\nALTER TABLE t1 ADD COLUMN name text;",
        )
        .unwrap();

        let config = LintConfig { deny_mixed_objects: false, base_ref: None };

        let issues = run_lint_checks(temp.path(), &unreleased, &config).unwrap();
        assert!(issues.is_empty());
    }

    #[test]
    fn detects_mixed_objects_when_flag_enabled() {
        let temp = tempfile::tempdir().unwrap();
        let unreleased = temp.path().join("sql/unreleased");
        std::fs::create_dir_all(&unreleased).unwrap();

        std::fs::write(
            unreleased.join("101.create.sql"),
            "CREATE TABLE unreleased_table (id int);",
        )
        .unwrap();

        // 102 modifies unreleased_table (from 101) AND modifies an existing released table (released_table)
        std::fs::write(
            unreleased.join("102.mixed.sql"),
            "-- depends-on: 101\nALTER TABLE unreleased_table ADD COLUMN a int;\nALTER TABLE released_table ADD COLUMN b text;",
        )
        .unwrap();

        let config = LintConfig { deny_mixed_objects: true, base_ref: None };

        let issues = run_lint_checks(temp.path(), &unreleased, &config).unwrap();
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].rule, LintRule::MixedObjects);
        assert!(
            issues[0]
                .message
                .contains("mixes modifications to already-released objects (TABLE released_table)")
        );
    }
}
