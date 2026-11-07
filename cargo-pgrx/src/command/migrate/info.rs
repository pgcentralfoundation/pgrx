//LICENSE Portions Copyright 2026-2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use super::MigrateContext;
use super::fragment::{assemble_sql_content, collect_fragments_from_dir};
use super::version::{
    ExtVersion, existing_upgrade_scripts, resolve_prev_version, resolve_target_version,
};
use crate::CommandExecute;
use owo_colors::OwoColorize;
use serde::Serialize;
use std::path::{Path, PathBuf};

/// Inspect extension migration plan without modifying disk state
#[derive(clap::Args, Debug)]
#[clap(author)]
pub(crate) struct Info {
    /// Target release version (e.g. 0.26.0). Defaults to the extension's current version if it's newer than every script in sql/, or else the next patch version
    pub(crate) target_version: Option<String>,

    /// Previous release version to upgrade from (e.g. 0.25.9). Defaults to the latest released version in sql/
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

/// Everything an assembly would do. `assemble`, `assemble --dry-run` and `info` all work from one
/// of these, so what they report can't drift from what's done.
#[derive(Debug, Serialize)]
pub(crate) struct MigratePlan {
    pub(crate) extension_name: String,
    pub(crate) target_version: ExtVersion,
    pub(crate) prev_version: ExtVersion,
    pub(crate) output_file: PathBuf,
    pub(crate) is_empty: bool,
    pub(crate) fragments: Vec<FragmentInfo>,
    /// The assembled upgrade script
    #[serde(skip)]
    pub(crate) script: String,
}

#[derive(Debug, Serialize)]
pub(crate) struct FragmentInfo {
    pub(crate) filename: String,
    pub(crate) id: String,
    pub(crate) dependencies: Vec<String>,
    #[serde(skip)]
    pub(crate) path: PathBuf,
}

pub(crate) fn build_migrate_plan(
    context: &MigrateContext,
    target_version: Option<&str>,
    prev_version: Option<&str>,
    output_dir: Option<&Path>,
) -> eyre::Result<MigratePlan> {
    let extname = &context.extname;
    let current = &context.current_version;
    let scripts = existing_upgrade_scripts(&context.sql_dir, extname)?;

    let target = resolve_target_version(&scripts, target_version, current)?;
    let prev = resolve_prev_version(&scripts, &target, prev_version, current)?;

    let fragments = collect_fragments_from_dir(&context.unreleased_dir)?;
    let script = assemble_sql_content(extname, &target.to_string(), &fragments);

    let out_dir = output_dir.unwrap_or(&context.sql_dir);
    Ok(MigratePlan {
        extension_name: extname.clone(),
        output_file: out_dir.join(format!("{extname}--{prev}--{target}.sql")),
        target_version: target,
        prev_version: prev,
        is_empty: fragments.is_empty(),
        fragments: fragments
            .into_iter()
            .map(|f| FragmentInfo {
                id: f.id.to_string(),
                dependencies: f.dependencies.iter().map(ToString::to_string).collect(),
                filename: f.filename,
                path: f.path,
            })
            .collect(),
        script,
    })
}

pub(crate) fn render_plan_text(plan: &MigratePlan) {
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
        let context = MigrateContext::load(self.package.as_deref(), self.manifest_path.as_deref())?;
        let plan = build_migrate_plan(
            &context,
            self.target_version.as_deref(),
            self.prev_version.as_deref(),
            self.output_dir.as_deref(),
        )?;

        if self.json {
            println!("{}", serde_json::to_string_pretty(&plan)?);
        } else {
            render_plan_text(&plan);
        }

        Ok(())
    }
}
