//LICENSE Portions Copyright 2026-2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
pub(crate) mod assemble;
pub(crate) mod fragment;
pub(crate) mod info;
pub(crate) mod lint;
pub(crate) mod version;

use crate::CommandExecute;
use cargo_toml::Manifest;
use std::path::{Path, PathBuf};

/// Manage extension SQL migrations and upgrade scripts
#[derive(clap::Args, Debug)]
#[clap(author)]
pub(crate) struct Migrate {
    #[clap(subcommand)]
    subcommand: MigrateSubcommands,
}

#[derive(clap::Subcommand, Debug)]
enum MigrateSubcommands {
    Assemble(assemble::Assemble),
    Info(info::Info),
    #[clap(alias = "lint")]
    Check(lint::Check),
}

impl CommandExecute for Migrate {
    fn execute(self) -> eyre::Result<()> {
        match self.subcommand {
            MigrateSubcommands::Assemble(c) => c.execute(),
            MigrateSubcommands::Info(c) => c.execute(),
            MigrateSubcommands::Check(c) => c.execute(),
        }
    }
}

/// Resolves the unreleased SQL directory from package manifest metadata or defaults to `<crate>/sql/unreleased`.
pub fn resolve_unreleased_dir(manifest: &Manifest, manifest_path: &Path) -> PathBuf {
    let base_dir = manifest_path.parent().unwrap_or_else(|| Path::new("."));
    if let Some(pkg) = &manifest.package
        && let Some(cargo_toml::Value::Table(metadata)) = &pkg.metadata
        && let Some(cargo_toml::Value::Table(pgrx_metadata)) = metadata.get("pgrx")
        && let Some(cargo_toml::Value::String(dir)) = pgrx_metadata
            .get("unreleased-sql-dir")
            .or_else(|| pgrx_metadata.get("unreleased_sql_dir"))
    {
        return base_dir.join(dir);
    }
    base_dir.join("sql/unreleased")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum UnreleasedFragmentMode {
    /// Ephemeral assembly directly into the installed extension directory
    AssembleEphemeral,
    /// Hermetic assembly directly into the package staging directory
    PackageAssemble,
    /// Disallow unreleased fragments (errors if any exist, used by `package` by default)
    Disallow,
    /// Skip unreleased fragments without erroring (e.g. `--no-assemble-unreleased`)
    Skip,
}

pub(crate) fn handle_unreleased_fragments(
    package_manifest: &Manifest,
    package_manifest_path: &Path,
    extname: &str,
    extdir: &Path,
    mode: UnreleasedFragmentMode,
    output_tracking: &mut Vec<PathBuf>,
) -> eyre::Result<()> {
    use eyre::{Context, eyre};
    use owo_colors::OwoColorize;
    use pgrx_pg_config::cargo::PgrxManifestExt;
    use std::fs;

    let unreleased_dir = resolve_unreleased_dir(package_manifest, package_manifest_path);
    let fragments = fragment::collect_fragments_from_dir(&unreleased_dir)?;
    if fragments.is_empty() {
        return Ok(());
    }

    match mode {
        UnreleasedFragmentMode::Skip => Ok(()),
        UnreleasedFragmentMode::Disallow => Err(eyre!(
            "found unreleased SQL migration fragments in `{}`.\n\
             Run `cargo pgrx migrate assemble` before packaging for release, or pass `--assemble-unreleased`.",
            unreleased_dir.display()
        )),
        UnreleasedFragmentMode::AssembleEphemeral | UnreleasedFragmentMode::PackageAssemble => {
            let crate_dir = package_manifest_path.parent().unwrap_or_else(|| Path::new("."));
            let sql_dir = crate_dir.join("sql");

            let existing_targets = version::get_existing_sql_targets(&sql_dir, extname);
            let manifest_ver_str = package_manifest.package_version().ok();
            let manifest_ver = manifest_ver_str.as_deref().and_then(version::parse_version_lossy);

            let (prev_ver, target_ver) = if let Some(latest_target) =
                existing_targets.into_iter().max()
            {
                let target =
                    version::derive_ephemeral_target_version(&latest_target, manifest_ver.as_ref());
                (latest_target, target)
            } else if let Some(pkg_ver) = manifest_ver {
                let target = semver::Version::new(pkg_ver.major, pkg_ver.minor, pkg_ver.patch + 1);
                (pkg_ver, target)
            } else {
                return Err(eyre!("could not determine version for ephemeral upgrade script"));
            };

            let prev_ver_str = prev_ver.to_string();
            let target_ver_str = target_ver.to_string();

            let filename = format!("{extname}--{prev_ver_str}--{target_ver_str}.sql");
            let dest_sql_file = extdir.join(&filename);

            if !extdir.exists() {
                fs::create_dir_all(extdir).wrap_err_with(|| {
                    format!("failed to create destination directory `{}`", extdir.display())
                })?;
            }

            let assembled = fragment::assemble_sql_content(extname, &target_ver_str, &fragments);
            fs::write(&dest_sql_file, assembled).wrap_err_with(|| {
                format!("failed to write ephemeral upgrade script `{}`", dest_sql_file.display())
            })?;
            output_tracking.push(dest_sql_file.clone());

            let display_dest = crate::command::install::format_display_path(&dest_sql_file)?;
            println!(
                "{} unreleased fragments for {} -> {} into {}",
                "  Assembling".bold().green(),
                prev_ver_str.cyan(),
                target_ver_str.cyan(),
                display_dest.cyan()
            );

            let dest_control = extdir.join(format!("{extname}.control"));
            if dest_control.exists() {
                version::update_control_file_default_version(&dest_control, &target_ver_str)?;
                let display_control = crate::command::install::format_display_path(&dest_control)?;
                println!(
                    "{} default_version = '{}' in {}",
                    "     Updated".bold().green(),
                    target_ver_str.cyan(),
                    display_control.cyan()
                );
            }

            // Ensure base schema matching target_ver_str exists in extdir so CREATE EXTENSION
            // can execute it directly instead of searching for upgrade paths from older base schemas.
            if let Some(pkg_ver) = manifest_ver_str.as_deref() {
                if pkg_ver != target_ver_str {
                    let generated_base = extdir.join(format!("{extname}--{pkg_ver}.sql"));
                    if generated_base.exists() {
                        let dest_base_sql = extdir.join(format!("{extname}--{target_ver_str}.sql"));
                        fs::copy(&generated_base, &dest_base_sql).wrap_err_with(|| {
                            format!(
                                "failed to copy generated base schema `{}` to `{}`",
                                generated_base.display(),
                                dest_base_sql.display()
                            )
                        })?;
                        output_tracking.push(dest_base_sql.clone());
                        let display_base =
                            crate::command::install::format_display_path(&dest_base_sql)?;
                        println!(
                            "{} base schema for {} at {}",
                            "    Installed".bold().green(),
                            target_ver_str.cyan(),
                            display_base.cyan()
                        );
                    }
                }
            }

            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn resolve_unreleased_dir_default() {
        let manifest = cargo_toml::Manifest::from_str(
            "\
[package]
name = \"my_ext\"
version = \"0.1.0\"
",
        )
        .unwrap();
        let path = Path::new("/some/dir/Cargo.toml");
        assert_eq!(
            resolve_unreleased_dir(&manifest, path),
            PathBuf::from("/some/dir/sql/unreleased")
        );
    }

    #[test]
    fn resolve_unreleased_dir_custom_metadata() {
        let manifest = cargo_toml::Manifest::from_str(
            "\
[package]
name = \"my_ext\"
version = \"0.1.0\"

[package.metadata.pgrx]
unreleased-sql-dir = \"custom/migrations\"
",
        )
        .unwrap();
        let path = Path::new("/some/dir/Cargo.toml");
        assert_eq!(
            resolve_unreleased_dir(&manifest, path),
            PathBuf::from("/some/dir/custom/migrations")
        );
    }

    #[test]
    fn handle_unreleased_fragments_disallow_errors() {
        let temp = tempfile::tempdir().unwrap();
        let crate_dir = temp.path().join("my_ext");
        let unreleased_dir = crate_dir.join("sql/unreleased");
        fs::create_dir_all(&unreleased_dir).unwrap();

        fs::write(
            unreleased_dir.join("101.add_func.sql"),
            "CREATE FUNCTION f() RETURNS void AS $$ $$ LANGUAGE sql;",
        )
        .unwrap();

        let manifest = cargo_toml::Manifest::from_str(
            "\
[package]
name = \"my_ext\"
version = \"0.25.0\"
",
        )
        .unwrap();
        let manifest_path = crate_dir.join("Cargo.toml");
        let extdir = temp.path().join("share/extension");
        let mut tracking = Vec::new();

        let res = handle_unreleased_fragments(
            &manifest,
            &manifest_path,
            "my_ext",
            &extdir,
            UnreleasedFragmentMode::Disallow,
            &mut tracking,
        );

        assert!(res.is_err());
        let err_msg = res.unwrap_err().to_string();
        assert!(err_msg.contains("found unreleased SQL migration fragments"));
        assert!(err_msg.contains("cargo pgrx migrate assemble"));
    }

    #[test]
    fn handle_unreleased_fragments_skip_does_nothing() {
        let temp = tempfile::tempdir().unwrap();
        let crate_dir = temp.path().join("my_ext");
        let unreleased_dir = crate_dir.join("sql/unreleased");
        fs::create_dir_all(&unreleased_dir).unwrap();

        fs::write(
            unreleased_dir.join("101.add_func.sql"),
            "CREATE FUNCTION f() RETURNS void AS $$ $$ LANGUAGE sql;",
        )
        .unwrap();

        let manifest = cargo_toml::Manifest::from_str(
            "\
[package]
name = \"my_ext\"
version = \"0.25.0\"
",
        )
        .unwrap();
        let manifest_path = crate_dir.join("Cargo.toml");
        let extdir = temp.path().join("share/extension");
        let mut tracking = Vec::new();

        let res = handle_unreleased_fragments(
            &manifest,
            &manifest_path,
            "my_ext",
            &extdir,
            UnreleasedFragmentMode::Skip,
            &mut tracking,
        );

        assert!(res.is_ok());
        assert!(tracking.is_empty());
    }

    #[test]
    fn handle_unreleased_fragments_assembles_and_updates_control() {
        let temp = tempfile::tempdir().unwrap();
        let crate_dir = temp.path().join("my_ext");
        let unreleased_dir = crate_dir.join("sql/unreleased");
        let released_sql_dir = crate_dir.join("sql");
        fs::create_dir_all(&unreleased_dir).unwrap();

        // Simulate a previously released upgrade file
        fs::write(released_sql_dir.join("my_ext--0.24.0--0.25.0.sql"), "-- old").unwrap();

        fs::write(
            unreleased_dir.join("101.add_func.sql"),
            "CREATE FUNCTION f() RETURNS void AS $$ $$ LANGUAGE sql;",
        )
        .unwrap();

        let manifest = cargo_toml::Manifest::from_str(
            "\
[package]
name = \"my_ext\"
version = \"0.25.0\"
",
        )
        .unwrap();
        let manifest_path = crate_dir.join("Cargo.toml");
        let extdir = temp.path().join("share/extension");
        fs::create_dir_all(&extdir).unwrap();

        let control_path = extdir.join("my_ext.control");
        fs::write(&control_path, "default_version = '0.25.0'\ncomment = 'my ext'\n").unwrap();

        let mut tracking = Vec::new();

        handle_unreleased_fragments(
            &manifest,
            &manifest_path,
            "my_ext",
            &extdir,
            UnreleasedFragmentMode::AssembleEphemeral,
            &mut tracking,
        )
        .unwrap();

        // Target version derived: 0.25.0 -> 0.25.1
        let expected_sql = extdir.join("my_ext--0.25.0--0.25.1.sql");
        assert!(expected_sql.exists());
        assert_eq!(tracking, vec![expected_sql.clone()]);

        let sql_content = fs::read_to_string(&expected_sql).unwrap();
        assert!(sql_content.contains("ALTER EXTENSION my_ext UPDATE TO '0.25.1'"));
        assert!(sql_content.contains("CREATE FUNCTION f()"));

        let control_content = fs::read_to_string(&control_path).unwrap();
        assert!(control_content.contains("default_version = '0.25.1'"));
    }

    #[test]
    fn handle_unreleased_fragments_copies_base_schema_when_present() {
        let temp = tempfile::tempdir().unwrap();
        let crate_dir = temp.path().join("my_ext");
        let unreleased_dir = crate_dir.join("sql/unreleased");
        let released_sql_dir = crate_dir.join("sql");
        fs::create_dir_all(&unreleased_dir).unwrap();

        fs::write(released_sql_dir.join("my_ext--0.24.0--0.25.0.sql"), "-- old").unwrap();
        fs::write(
            unreleased_dir.join("101.add_func.sql"),
            "CREATE FUNCTION f() RETURNS void AS $$ $$ LANGUAGE sql;",
        )
        .unwrap();

        let manifest = cargo_toml::Manifest::from_str(
            "\
[package]
name = \"my_ext\"
version = \"0.25.0\"
",
        )
        .unwrap();
        let manifest_path = crate_dir.join("Cargo.toml");
        let extdir = temp.path().join("share/extension");
        fs::create_dir_all(&extdir).unwrap();

        // Simulate generated base schema matching manifest version
        let generated_base = extdir.join("my_ext--0.25.0.sql");
        fs::write(&generated_base, "-- full base schema 0.25.0").unwrap();

        let control_path = extdir.join("my_ext.control");
        fs::write(&control_path, "default_version = '0.25.0'\ncomment = 'my ext'\n").unwrap();

        let mut tracking = Vec::new();

        handle_unreleased_fragments(
            &manifest,
            &manifest_path,
            "my_ext",
            &extdir,
            UnreleasedFragmentMode::AssembleEphemeral,
            &mut tracking,
        )
        .unwrap();

        // Target version derived: 0.25.0 -> 0.25.1
        let expected_upgrade_sql = extdir.join("my_ext--0.25.0--0.25.1.sql");
        let expected_base_sql = extdir.join("my_ext--0.25.1.sql");
        assert!(expected_upgrade_sql.exists());
        assert!(expected_base_sql.exists());
        assert_eq!(fs::read_to_string(&expected_base_sql).unwrap(), "-- full base schema 0.25.0");
        assert_eq!(tracking, vec![expected_upgrade_sql, expected_base_sql]);
    }
}
