//LICENSE Portions Copyright 2026-2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use super::fragment::collect_fragments_from_dir;
use super::resolve_unreleased_dir;
use super::version::{parse_version_lossy, resolve_prev_version, resolve_target_version};
use crate::CommandExecute;
use crate::command::get::find_control_file;
use crate::manifest::get_package_manifest;
use eyre::eyre;
use owo_colors::OwoColorize;
use pgrx_pg_config::cargo::PgrxManifestExt;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::{Path, PathBuf};

/// Inspect extension migration plan without modifying disk state
#[derive(clap::Args, Debug)]
#[clap(author)]
pub(crate) struct Info {
    /// Target release version (e.g. 0.26.0). Defaults to package.version in Cargo.toml if greater than existing sql/ releases, or derives the next patch version
    pub(crate) target_version: Option<String>,

    /// Previous release version to upgrade from (e.g. 0.25.9). Defaults to latest target version in sql/
    #[clap(long)]
    pub(crate) prev_version: Option<String>,

    /// Output directory for the assembled upgrade script (defaults to <crate>/sql/)
    #[clap(long, value_parser)]
    pub(crate) output_dir: Option<PathBuf>,

    /// Package to build (see `cargo help pkgid`)
    #[clap(long, short)]
    pub(crate) package: Option<String>,

    /// Path to Cargo.toml
    #[clap(long, value_parser)]
    pub(crate) manifest_path: Option<PathBuf>,

    /// Output migration plan as JSON
    #[clap(long)]
    pub(crate) json: bool,

    #[clap(from_global, action = clap::ArgAction::Count)]
    pub(crate) verbose: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExtensionName(pub String);

impl fmt::Display for ExtensionName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TargetVersion(pub String);

impl fmt::Display for TargetVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PredecessorVersion(pub String);

impl fmt::Display for PredecessorVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SqlDir<'a>(pub &'a Path);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnreleasedDir<'a>(pub &'a Path);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutputDir<'a>(pub &'a Path);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlanDirectories<'a> {
    pub sql_dir: SqlDir<'a>,
    pub unreleased_dir: UnreleasedDir<'a>,
    pub output_dir: Option<OutputDir<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigratePlan {
    pub extension_name: ExtensionName,
    pub target_version: TargetVersion,
    pub prev_version: PredecessorVersion,
    pub output_file: PathBuf,
    pub is_empty: bool,
    pub fragments: Vec<FragmentInfo>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FragmentInfo {
    pub filename: String,
    pub id: String,
    pub dependencies: Vec<String>,
}

pub struct PlanRequest<'a> {
    pub extname: &'a str,
    pub dirs: PlanDirectories<'a>,
    pub target_version: Option<&'a str>,
    pub prev_version: Option<&'a str>,
    pub manifest_version: Option<&'a str>,
}

pub fn build_migrate_plan(req: PlanRequest<'_>) -> eyre::Result<MigratePlan> {
    let target_ver_str = resolve_target_version(
        req.dirs.sql_dir.0,
        req.extname,
        req.target_version,
        req.manifest_version,
    )?;

    let target_ver = parse_version_lossy(&target_ver_str)
        .ok_or_else(|| eyre!("invalid semver for target version `{target_ver_str}`"))?;

    let prev_ver_str = resolve_prev_version(
        req.dirs.sql_dir.0,
        req.extname,
        &target_ver,
        req.prev_version,
        req.manifest_version,
    )?;

    let fragments = collect_fragments_from_dir(req.dirs.unreleased_dir.0)?;
    let is_empty = fragments.is_empty();

    let out_dir = req.dirs.output_dir.map(|d| d.0).unwrap_or(req.dirs.sql_dir.0);
    let output_file =
        out_dir.join(format!("{}--{}--{}.sql", req.extname, prev_ver_str, target_ver_str));

    let fragment_infos = fragments
        .into_iter()
        .map(|f| FragmentInfo {
            filename: f.filename,
            id: f.id.to_string(),
            dependencies: f.dependencies.into_iter().map(|d| d.to_string()).collect(),
        })
        .collect();

    Ok(MigratePlan {
        extension_name: ExtensionName(req.extname.to_string()),
        target_version: TargetVersion(target_ver_str.to_string()),
        prev_version: PredecessorVersion(prev_ver_str),
        output_file,
        is_empty,
        fragments: fragment_infos,
    })
}

pub fn render_plan_text(plan: &MigratePlan) {
    println!(
        "{} {} ({} -> {})",
        "Extension:".bold(),
        plan.extension_name.cyan(),
        plan.prev_version.yellow(),
        plan.target_version.green()
    );
    println!("{} {}", "Output:".bold(), plan.output_file.display());
    if plan.is_empty {
        println!("{} none (stub upgrade script)", "Fragments:".bold());
    } else {
        println!("{} ({}):", "Fragments:".bold(), plan.fragments.len());
        for f in &plan.fragments {
            if f.dependencies.is_empty() {
                println!("  - {} (id: {})", f.filename.cyan(), f.id);
            } else {
                println!(
                    "  - {} (id: {}, depends on: [{}])",
                    f.filename.cyan(),
                    f.id,
                    f.dependencies.join(", ").yellow()
                );
            }
        }
    }
}

impl CommandExecute for Info {
    #[tracing::instrument(level = "error", skip(self))]
    fn execute(self) -> eyre::Result<()> {
        let features = clap_cargo::Features::default();
        let (package_manifest, package_manifest_path) = get_package_manifest(
            &features,
            self.package.as_deref(),
            self.manifest_path.as_deref(),
        )?;

        let (_, extname) = find_control_file(&package_manifest_path)?;
        let crate_dir = package_manifest_path.parent().unwrap_or_else(|| Path::new("."));
        let sql_dir = crate_dir.join("sql");
        let unreleased_dir = resolve_unreleased_dir(&package_manifest, &package_manifest_path);
        let manifest_pkg_ver = package_manifest.package_version().ok();

        let plan = build_migrate_plan(PlanRequest {
            extname: &extname,
            dirs: PlanDirectories {
                sql_dir: SqlDir(&sql_dir),
                unreleased_dir: UnreleasedDir(&unreleased_dir),
                output_dir: self.output_dir.as_deref().map(OutputDir),
            },
            target_version: self.target_version.as_deref(),
            prev_version: self.prev_version.as_deref(),
            manifest_version: manifest_pkg_ver.as_deref(),
        })?;

        if self.json {
            println!("{}", serde_json::to_string_pretty(&plan)?);
        } else {
            render_plan_text(&plan);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn build_migrate_plan_serializes_json_properly() {
        let temp = tempfile::tempdir().unwrap();
        let sql_dir = temp.path().join("sql");
        let unreleased_dir = sql_dir.join("unreleased");
        fs::create_dir_all(&unreleased_dir).unwrap();

        fs::write(sql_dir.join("test_ext--0.25.0--0.25.9.sql"), "-- old").unwrap();
        fs::write(unreleased_dir.join("5903.rename_test.sql"), "SELECT 1;").unwrap();
        fs::write(unreleased_dir.join("6221.unfielded.sql"), "-- depends-on: 5903\nSELECT 2;")
            .unwrap();

        let plan = build_migrate_plan(PlanRequest {
            extname: "test_ext",
            dirs: PlanDirectories {
                sql_dir: SqlDir(&sql_dir),
                unreleased_dir: UnreleasedDir(&unreleased_dir),
                output_dir: None,
            },
            target_version: Some("0.26.0"),
            prev_version: None,
            manifest_version: Some("0.26.0"),
        })
        .unwrap();

        assert_eq!(plan.extension_name.0, "test_ext");
        assert_eq!(plan.prev_version.0, "0.25.9");
        assert_eq!(plan.target_version.0, "0.26.0");
        assert!(!plan.is_empty);
        assert_eq!(plan.fragments.len(), 2);
        assert_eq!(plan.fragments[0].filename, "5903.rename_test.sql");
        assert_eq!(plan.fragments[1].filename, "6221.unfielded.sql");
        assert_eq!(plan.fragments[1].dependencies, vec!["5903"]);

        let json = serde_json::to_string(&plan).unwrap();
        assert!(json.contains(r#""extension_name":"test_ext""#));
        assert!(json.contains(r#""prev_version":"0.25.9""#));
        assert!(json.contains(r#""target_version":"0.26.0""#));
        assert!(json.contains(r#""is_empty":false"#));
    }

    #[test]
    fn build_migrate_plan_defaults_target_version_when_manifest_is_older() {
        let temp = tempfile::tempdir().unwrap();
        let sql_dir = temp.path().join("sql");
        let unreleased_dir = sql_dir.join("unreleased");
        fs::create_dir_all(&unreleased_dir).unwrap();

        fs::write(sql_dir.join("test_ext--0.25.0--0.25.9.sql"), "-- old").unwrap();
        fs::write(unreleased_dir.join("1.test.sql"), "SELECT 1;").unwrap();

        // When manifest is 0.25.6 and sql has 0.25.9, default target version should be 0.25.10
        let plan = build_migrate_plan(PlanRequest {
            extname: "test_ext",
            dirs: PlanDirectories {
                sql_dir: SqlDir(&sql_dir),
                unreleased_dir: UnreleasedDir(&unreleased_dir),
                output_dir: None,
            },
            target_version: None,
            prev_version: None,
            manifest_version: Some("0.25.6"),
        })
        .unwrap();

        assert_eq!(plan.target_version.0, "0.25.10");
        assert_eq!(plan.prev_version.0, "0.25.9");
    }
}
