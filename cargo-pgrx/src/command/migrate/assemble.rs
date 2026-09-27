//LICENSE Portions Copyright 2026-2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use super::fragment::{assemble_sql_content, collect_fragments_from_dir};
use super::resolve_unreleased_dir;
use super::version::{
    clean_version_string, parse_version_lossy, resolve_prev_version, resolve_target_version,
};
use crate::CommandExecute;
use crate::command::get::find_control_file;
use crate::manifest::get_package_manifest;
use eyre::{Context, eyre};
use owo_colors::OwoColorize;
use pgrx_pg_config::cargo::PgrxManifestExt;
use std::fs;
use std::path::{Path, PathBuf};

/// Assemble unreleased migration fragments into a versioned upgrade script
#[derive(clap::Args, Debug)]
#[clap(author)]
pub(crate) struct Assemble {
    /// Target release version (e.g. 0.26.0). Defaults to package.version in Cargo.toml if greater than existing sql/ releases, or derives the next patch version
    pub(crate) target_version: Option<String>,

    /// Previous release version to upgrade from (e.g. 0.25.9). Defaults to latest target version in sql/
    #[clap(long)]
    pub(crate) prev_version: Option<String>,

    /// Output directory for the assembled upgrade script (defaults to <crate>/sql/)
    #[clap(long, value_parser)]
    pub(crate) output_dir: Option<PathBuf>,

    /// Preserve fragments in the unreleased directory instead of deleting them after assembly
    #[clap(long)]
    pub(crate) preserve_fragments: bool,

    /// Package to build (see `cargo help pkgid`)
    #[clap(long, short)]
    pub(crate) package: Option<String>,

    /// Path to Cargo.toml
    #[clap(long, value_parser)]
    pub(crate) manifest_path: Option<PathBuf>,

    /// Allow assembling a stub upgrade script when no unreleased fragments exist
    #[clap(long)]
    pub(crate) allow_empty: bool,

    /// Update default_version in the extension .control file
    #[clap(long)]
    pub(crate) update_control: bool,

    /// Inspect the migration plan without generating files or modifying disk
    #[clap(long)]
    pub(crate) dry_run: bool,

    /// Output migration plan as JSON
    #[clap(long)]
    pub(crate) json: bool,

    #[clap(from_global, action = clap::ArgAction::Count)]
    pub(crate) verbose: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FragmentRetention {
    Consume,
    Preserve,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlFileUpdate {
    Skip,
    Update,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CrateDir<'a>(pub &'a Path);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SqlDir<'a>(pub &'a Path);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnreleasedDir<'a>(pub &'a Path);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OutputDir<'a>(pub &'a Path);

pub struct MigrationDirectories<'a> {
    pub crate_dir: Option<CrateDir<'a>>,
    pub sql_dir: SqlDir<'a>,
    pub unreleased_dir: UnreleasedDir<'a>,
    pub output_dir: Option<OutputDir<'a>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TargetVersionStr<'a>(pub &'a str);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrevVersionStr<'a>(pub Option<&'a str>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ManifestVersionStr<'a>(pub Option<&'a str>);

pub struct VersionSpec<'a> {
    pub target_version: TargetVersionStr<'a>,
    pub prev_version: PrevVersionStr<'a>,
    pub manifest_version: ManifestVersionStr<'a>,
}

pub struct AssembleParams<'a> {
    pub extname: &'a str,
    pub dirs: MigrationDirectories<'a>,
    pub versions: VersionSpec<'a>,
    pub retention: FragmentRetention,
    pub allow_empty: bool,
    pub control_update: ControlFileUpdate,
}

pub fn assemble_release_upgrade_script(params: AssembleParams<'_>) -> eyre::Result<PathBuf> {
    let target_ver_str = clean_version_string(params.versions.target_version.0);
    let target_ver = parse_version_lossy(target_ver_str)
        .ok_or_else(|| eyre!("invalid semver for target version `{target_ver_str}`"))?;

    let prev_ver_str = resolve_prev_version(
        params.dirs.sql_dir.0,
        params.extname,
        &target_ver,
        params.versions.prev_version.0,
        params.versions.manifest_version.0,
    )?;

    let fragments = collect_fragments_from_dir(params.dirs.unreleased_dir.0)?;
    if fragments.is_empty() {
        if !params.allow_empty {
            return Err(eyre!(
                "no unreleased SQL migration fragments found in `{}`. Specify a target version or pass `--allow-empty` to generate a stub upgrade script.",
                params.dirs.unreleased_dir.0.display()
            ));
        }
        println!(
            "{} 0 SQL migration fragments; generating stub upgrade script",
            "  Assembling".bold().yellow()
        );
    } else {
        println!(
            "{} {} SQL migration fragment(s) from {}",
            "  Assembling".bold().green(),
            fragments.len().to_string().bold().white(),
            params.dirs.unreleased_dir.0.display().cyan()
        );
        for fragment in &fragments {
            println!("   - {}", fragment.filename);
        }
    }

    let out_dir = params.dirs.output_dir.map(|d| d.0).unwrap_or(params.dirs.sql_dir.0);
    if !out_dir.exists() {
        fs::create_dir_all(out_dir).wrap_err_with(|| {
            format!("failed to create output directory `{}`", out_dir.display())
        })?;
    }

    let output_filename = format!("{}--{}--{}.sql", params.extname, prev_ver_str, target_ver_str);
    let output_file = out_dir.join(&output_filename);

    let assembled = assemble_sql_content(params.extname, target_ver_str, &fragments);
    fs::write(&output_file, assembled).wrap_err_with(|| {
        format!("failed to write assembled upgrade script to `{}`", output_file.display())
    })?;

    println!(
        "{} assembled upgrade script to {}",
        "       Saved".bold().green(),
        output_file.display().cyan()
    );

    if params.control_update == ControlFileUpdate::Update {
        let control_filename = format!("{}.control", params.extname);
        let control_path = if let Some(out) = params.dirs.output_dir
            && out.0.join(&control_filename).exists()
        {
            out.0.join(&control_filename)
        } else if let Some(crate_dir) = params.dirs.crate_dir
            && crate_dir.0.join(&control_filename).exists()
        {
            crate_dir.0.join(&control_filename)
        } else {
            return Err(eyre!(
                "could not find `{control_filename}` to update default_version in `{}` or `{}`",
                params
                    .dirs
                    .output_dir
                    .map(|d| d.0.display().to_string())
                    .unwrap_or_else(|| "<none>".to_string()),
                params
                    .dirs
                    .crate_dir
                    .map(|d| d.0.display().to_string())
                    .unwrap_or_else(|| "<none>".to_string())
            ));
        };

        crate::command::migrate::version::update_control_file_default_version(
            &control_path,
            target_ver_str,
        )?;
        let display_control = crate::command::install::format_display_path(&control_path)?;
        println!(
            "{} default_version = '{}' in {}",
            "     Updated".bold().green(),
            target_ver_str.cyan(),
            display_control.cyan()
        );
    }

    if params.retention == FragmentRetention::Consume {
        for fragment in &fragments {
            if fragment.path.exists() {
                fs::remove_file(&fragment.path).wrap_err_with(|| {
                    format!("failed to remove consumed fragment `{}`", fragment.path.display())
                })?;
                println!(
                    "{} consumed fragment {}",
                    "     Removed".bold().yellow(),
                    fragment.filename
                );
            }
        }
    } else {
        println!(
            "{} unreleased fragments in {}",
            "   Preserved".bold().cyan(),
            params.dirs.unreleased_dir.0.display()
        );
    }

    Ok(output_file)
}

impl CommandExecute for Assemble {
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
        let target_ver_str = resolve_target_version(
            &sql_dir,
            &extname,
            self.target_version.as_deref(),
            manifest_pkg_ver.as_deref(),
        )?;

        if self.dry_run {
            let plan = super::info::build_migrate_plan(super::info::PlanRequest {
                extname: &extname,
                dirs: super::info::PlanDirectories {
                    sql_dir: super::info::SqlDir(&sql_dir),
                    unreleased_dir: super::info::UnreleasedDir(&unreleased_dir),
                    output_dir: self.output_dir.as_deref().map(super::info::OutputDir),
                },
                target_version: Some(&target_ver_str),
                prev_version: self.prev_version.as_deref(),
                manifest_version: manifest_pkg_ver.as_deref(),
            })?;

            if self.json {
                println!("{}", serde_json::to_string_pretty(&plan)?);
            } else {
                super::info::render_plan_text(&plan);
            }
            return Ok(());
        }

        let retention = if self.preserve_fragments {
            FragmentRetention::Preserve
        } else {
            FragmentRetention::Consume
        };

        let allow_empty = self.target_version.is_some() || self.allow_empty;
        let control_update =
            if self.update_control { ControlFileUpdate::Update } else { ControlFileUpdate::Skip };

        let params = AssembleParams {
            extname: &extname,
            dirs: MigrationDirectories {
                crate_dir: Some(CrateDir(crate_dir)),
                sql_dir: SqlDir(&sql_dir),
                unreleased_dir: UnreleasedDir(&unreleased_dir),
                output_dir: self.output_dir.as_deref().map(OutputDir),
            },
            versions: VersionSpec {
                target_version: TargetVersionStr(&target_ver_str),
                prev_version: PrevVersionStr(self.prev_version.as_deref()),
                manifest_version: ManifestVersionStr(manifest_pkg_ver.as_deref()),
            },
            retention,
            allow_empty,
            control_update,
        };

        assemble_release_upgrade_script(params)?;

        if self.json {
            let plan = super::info::build_migrate_plan(super::info::PlanRequest {
                extname: &extname,
                dirs: super::info::PlanDirectories {
                    sql_dir: super::info::SqlDir(&sql_dir),
                    unreleased_dir: super::info::UnreleasedDir(&unreleased_dir),
                    output_dir: self.output_dir.as_deref().map(super::info::OutputDir),
                },
                target_version: Some(&target_ver_str),
                prev_version: self.prev_version.as_deref(),
                manifest_version: manifest_pkg_ver.as_deref(),
            })?;
            println!("{}", serde_json::to_string_pretty(&plan)?);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assemble_release_upgrade_script_assembles_and_consumes() {
        let temp = tempfile::tempdir().unwrap();
        let sql_dir = temp.path().join("sql");
        let unreleased_dir = sql_dir.join("unreleased");
        fs::create_dir_all(&unreleased_dir).unwrap();

        fs::write(sql_dir.join("test_ext--0.1.0--0.2.0.sql"), "-- old").unwrap();
        let f1 = unreleased_dir.join("10.f1.sql");
        fs::write(&f1, "SELECT 1;").unwrap();

        let params = AssembleParams {
            extname: "test_ext",
            dirs: MigrationDirectories {
                crate_dir: None,
                sql_dir: SqlDir(&sql_dir),
                unreleased_dir: UnreleasedDir(&unreleased_dir),
                output_dir: None,
            },
            versions: VersionSpec {
                target_version: TargetVersionStr("0.3.0"),
                prev_version: PrevVersionStr(None),
                manifest_version: ManifestVersionStr(Some("0.3.0")),
            },
            retention: FragmentRetention::Consume,
            allow_empty: false,
            control_update: ControlFileUpdate::Skip,
        };

        let output_file = assemble_release_upgrade_script(params).unwrap();
        assert_eq!(output_file, sql_dir.join("test_ext--0.2.0--0.3.0.sql"));
        assert!(output_file.exists());

        let content = fs::read_to_string(&output_file).unwrap();
        assert!(content.contains("ALTER EXTENSION test_ext UPDATE TO '0.3.0'"));
        assert!(content.contains("SELECT 1;"));

        // Fragment was consumed (deleted)
        assert!(!f1.exists());
    }

    #[test]
    fn assemble_release_upgrade_script_preserves_fragments_when_requested() {
        let temp = tempfile::tempdir().unwrap();
        let sql_dir = temp.path().join("sql");
        let unreleased_dir = sql_dir.join("unreleased");
        fs::create_dir_all(&unreleased_dir).unwrap();

        fs::write(sql_dir.join("test_ext--0.1.0--0.2.0.sql"), "-- old").unwrap();
        let f1 = unreleased_dir.join("10.f1.sql");
        fs::write(&f1, "SELECT 1;").unwrap();

        let params = AssembleParams {
            extname: "test_ext",
            dirs: MigrationDirectories {
                crate_dir: None,
                sql_dir: SqlDir(&sql_dir),
                unreleased_dir: UnreleasedDir(&unreleased_dir),
                output_dir: None,
            },
            versions: VersionSpec {
                target_version: TargetVersionStr("0.3.0"),
                prev_version: PrevVersionStr(None),
                manifest_version: ManifestVersionStr(Some("0.3.0")),
            },
            retention: FragmentRetention::Preserve,
            allow_empty: false,
            control_update: ControlFileUpdate::Skip,
        };

        let output_file = assemble_release_upgrade_script(params).unwrap();
        assert!(output_file.exists());
        // Fragment was preserved
        assert!(f1.exists());
    }

    #[test]
    fn assemble_errors_when_no_fragments_and_not_allowed() {
        let temp = tempfile::tempdir().unwrap();
        let sql_dir = temp.path().join("sql");
        let unreleased_dir = sql_dir.join("unreleased");
        fs::create_dir_all(&unreleased_dir).unwrap();

        let params = AssembleParams {
            extname: "test_ext",
            dirs: MigrationDirectories {
                crate_dir: None,
                sql_dir: SqlDir(&sql_dir),
                unreleased_dir: UnreleasedDir(&unreleased_dir),
                output_dir: None,
            },
            versions: VersionSpec {
                target_version: TargetVersionStr("0.3.0"),
                prev_version: PrevVersionStr(Some("0.2.0")),
                manifest_version: ManifestVersionStr(None),
            },
            retention: FragmentRetention::Consume,
            allow_empty: false,
            control_update: ControlFileUpdate::Skip,
        };

        let res = assemble_release_upgrade_script(params);
        assert!(res.is_err());
        assert!(
            res.unwrap_err().to_string().contains("no unreleased SQL migration fragments found")
        );
    }

    #[test]
    fn assemble_generates_stub_when_allow_empty() {
        let temp = tempfile::tempdir().unwrap();
        let sql_dir = temp.path().join("sql");
        let unreleased_dir = sql_dir.join("unreleased");
        fs::create_dir_all(&unreleased_dir).unwrap();

        let params = AssembleParams {
            extname: "test_ext",
            dirs: MigrationDirectories {
                crate_dir: None,
                sql_dir: SqlDir(&sql_dir),
                unreleased_dir: UnreleasedDir(&unreleased_dir),
                output_dir: None,
            },
            versions: VersionSpec {
                target_version: TargetVersionStr("0.3.0"),
                prev_version: PrevVersionStr(Some("0.2.0")),
                manifest_version: ManifestVersionStr(None),
            },
            retention: FragmentRetention::Consume,
            allow_empty: true,
            control_update: ControlFileUpdate::Skip,
        };

        let output_file = assemble_release_upgrade_script(params).unwrap();
        assert_eq!(output_file, sql_dir.join("test_ext--0.2.0--0.3.0.sql"));
        assert!(output_file.exists());

        let content = fs::read_to_string(&output_file).unwrap();
        assert_eq!(
            content.trim(),
            "\\echo Use \"ALTER EXTENSION test_ext UPDATE TO '0.3.0'\" to load this file. \\quit"
        );
    }

    #[test]
    fn assemble_updates_control_file_when_requested() {
        let temp = tempfile::tempdir().unwrap();
        let crate_dir = temp.path().join("crate");
        let sql_dir = crate_dir.join("sql");
        let unreleased_dir = sql_dir.join("unreleased");
        fs::create_dir_all(&unreleased_dir).unwrap();

        let control_path = crate_dir.join("test_ext.control");
        fs::write(&control_path, "default_version = '0.2.0'\ncomment = 'test'\n").unwrap();

        fs::write(sql_dir.join("test_ext--0.1.0--0.2.0.sql"), "-- old").unwrap();
        let f1 = unreleased_dir.join("10.f1.sql");
        fs::write(&f1, "SELECT 1;").unwrap();

        let params = AssembleParams {
            extname: "test_ext",
            dirs: MigrationDirectories {
                crate_dir: Some(CrateDir(&crate_dir)),
                sql_dir: SqlDir(&sql_dir),
                unreleased_dir: UnreleasedDir(&unreleased_dir),
                output_dir: None,
            },
            versions: VersionSpec {
                target_version: TargetVersionStr("0.3.0"),
                prev_version: PrevVersionStr(None),
                manifest_version: ManifestVersionStr(Some("0.3.0")),
            },
            retention: FragmentRetention::Consume,
            allow_empty: false,
            control_update: ControlFileUpdate::Update,
        };

        let output_file = assemble_release_upgrade_script(params).unwrap();
        assert!(output_file.exists());

        let control_content = fs::read_to_string(&control_path).unwrap();
        assert!(control_content.contains("default_version = '0.3.0'"));
    }
}
