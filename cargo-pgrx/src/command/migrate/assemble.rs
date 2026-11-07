//LICENSE Portions Copyright 2026-2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use super::MigrateContext;
use super::info::{MigratePlan, build_migrate_plan, render_plan_text};
use super::version::update_control_file_default_version;
use crate::CommandExecute;
use crate::command::get::get_property;
use eyre::{Context, eyre};
use owo_colors::OwoColorize;
use std::fs;
use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};

/// Assemble unreleased migration fragments into a versioned upgrade script
#[derive(clap::Args, Debug)]
#[clap(author)]
pub(crate) struct Assemble {
    /// Target release version (e.g. 0.26.0). Defaults to the extension's current version if it's newer than every script in sql/, or else the next patch version
    pub(crate) target_version: Option<String>,

    /// Previous release version to upgrade from (e.g. 0.25.9). Defaults to the latest released version in sql/
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

    /// Set default_version in the extension .control file to the target version. A control file using `@CARGO_VERSION@` keeps it, and Cargo.toml must already have the target version
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

/// What `--update-control` means for this package.
#[derive(Debug, PartialEq, Eq)]
enum ControlUpdate {
    /// `--update-control` wasn't passed.
    None,
    /// `default_version` is `@CARGO_VERSION@`, which already resolves to the target. Writing a
    /// literal over it would stop the extension version following `Cargo.toml` from now on.
    FollowsCargo,
    Rewrite,
}

impl Assemble {
    fn control_update(
        &self,
        context: &MigrateContext,
        plan: &MigratePlan,
    ) -> eyre::Result<ControlUpdate> {
        if !self.update_control {
            return Ok(ControlUpdate::None);
        }

        let default_version = get_property(&context.manifest_path, "default_version")?;
        if default_version.as_deref() != Some("@CARGO_VERSION@") {
            return Ok(ControlUpdate::Rewrite);
        }

        if context.current_version != plan.target_version {
            return Err(eyre!(
                "`{}` uses `@CARGO_VERSION@`, so `default_version` follows `Cargo.toml`, which is at `{}`.\n\
                 Set the package version to `{}` first.",
                context.control_file.display(),
                context.current_version,
                plan.target_version
            ));
        }
        Ok(ControlUpdate::FollowsCargo)
    }

    /// Everything that can be known to fail, checked before anything is written or deleted.
    fn validate(&self, context: &MigrateContext, plan: &MigratePlan) -> eyre::Result<()> {
        if plan.is_empty && !self.allow_empty {
            return Err(eyre!(
                "no unreleased SQL migration fragments found in `{}`. Pass `--allow-empty` to generate a stub upgrade script.",
                context.unreleased_dir.display()
            ));
        }
        if plan.output_file.exists() {
            return Err(already_exists(&plan.output_file));
        }
        Ok(())
    }
}

fn already_exists(path: &Path) -> eyre::Report {
    eyre!(
        "`{}` already exists, and assembling would replace it. Delete it first if you mean to rebuild it, or choose another target version.",
        path.display()
    )
}

fn write_script(plan: &MigratePlan) -> eyre::Result<()> {
    let path = &plan.output_file;
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)
            .wrap_err_with(|| format!("failed to create output directory `{}`", dir.display()))?;
    }

    // `create_new` so that a script can't be lost to a race with `validate`: the fragments it was
    // assembled from are deleted once it's written.
    let mut file = fs::File::create_new(path).map_err(|err| match err.kind() {
        ErrorKind::AlreadyExists => already_exists(path),
        _ => eyre!(err).wrap_err(format!("failed to create `{}`", path.display())),
    })?;
    file.write_all(plan.script.as_bytes())
        .wrap_err_with(|| format!("failed to write `{}`", path.display()))
}

/// Progress goes to stderr so that stdout is only ever the plan, which `--json` callers parse.
fn apply_plan(
    context: &MigrateContext,
    plan: &MigratePlan,
    control_update: &ControlUpdate,
    preserve_fragments: bool,
) -> eyre::Result<()> {
    if plan.is_empty {
        eprintln!(
            "{} 0 SQL migration fragments; generating stub upgrade script",
            "  Assembling".bold().yellow()
        );
    } else {
        eprintln!(
            "{} {} SQL migration fragment(s) from {}",
            "  Assembling".bold().green(),
            plan.fragments.len().bold().white(),
            context.unreleased_dir.display().cyan()
        );
        for fragment in &plan.fragments {
            eprintln!("   - {}", fragment.filename);
        }
    }

    write_script(plan)?;
    eprintln!(
        "{} assembled upgrade script to {}",
        "       Saved".bold().green(),
        plan.output_file.display().cyan()
    );

    match control_update {
        ControlUpdate::None => {}
        ControlUpdate::FollowsCargo => eprintln!(
            "{} default_version follows Cargo.toml in {}",
            "    Unchanged".bold().green(),
            context.control_file.display().cyan()
        ),
        ControlUpdate::Rewrite => {
            let target = plan.target_version.to_string();
            update_control_file_default_version(&context.control_file, &target)?;
            eprintln!(
                "{} default_version = '{}' in {}",
                "     Updated".bold().green(),
                target.cyan(),
                context.control_file.display().cyan()
            );
        }
    }

    if preserve_fragments {
        eprintln!(
            "{} unreleased fragments in {}",
            "   Preserved".bold().cyan(),
            context.unreleased_dir.display()
        );
        return Ok(());
    }

    for fragment in &plan.fragments {
        fs::remove_file(&fragment.path).wrap_err_with(|| {
            format!("failed to remove consumed fragment `{}`", fragment.path.display())
        })?;
        eprintln!("{} consumed fragment {}", "     Removed".bold().yellow(), fragment.filename);
    }

    Ok(())
}

impl CommandExecute for Assemble {
    #[tracing::instrument(level = "error", skip(self))]
    fn execute(self) -> eyre::Result<()> {
        let context = MigrateContext::load(self.package.as_deref(), self.manifest_path.as_deref())?;
        let plan = build_migrate_plan(
            &context,
            self.target_version.as_deref(),
            self.prev_version.as_deref(),
            self.output_dir.as_deref(),
        )?;

        let control_update = self.control_update(&context, &plan)?;
        self.validate(&context, &plan)?;

        if !self.dry_run {
            apply_plan(&context, &plan, &control_update, self.preserve_fragments)?;
        }

        if self.json {
            println!("{}", serde_json::to_string_pretty(&plan)?);
        } else if self.dry_run {
            render_plan_text(&plan);
        }

        Ok(())
    }
}
