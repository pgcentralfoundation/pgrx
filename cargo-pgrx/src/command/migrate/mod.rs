//LICENSE Portions Copyright 2026-2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
//! SQL migration fragments: upgrade SQL kept as one small file per change, and assembled into a
//! versioned upgrade script when a release is cut.
//!
//! Fragments are opt-in. A package enables them by setting `unreleased-sql-dir` in
//! `[package.metadata.pgrx]`. Without that key nothing here touches the disk, and every `cargo pgrx`
//! command behaves as if this module didn't exist.
pub(crate) mod assemble;
pub(crate) mod fragment;
pub(crate) mod info;
pub(crate) mod lint;
pub(crate) mod sql;
pub(crate) mod version;

use crate::CommandExecute;
use crate::command::get::find_control_file;
use crate::command::install::{filter_contents, format_display_path, get_version};
use crate::manifest::get_package_manifest;
use cargo_toml::Manifest;
use eyre::{Context, eyre};
use fragment::MigrationFragment;
use owo_colors::OwoColorize;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use version::{ExtVersion, UpgradeScript};

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

const UNRELEASED_SQL_DIR_KEY: &str = "unreleased-sql-dir";

/// Finds the fragment directory the package opted into, relative to its `Cargo.toml`.
pub(crate) fn resolve_unreleased_dir(
    manifest: &Manifest,
    manifest_path: &Path,
) -> eyre::Result<Option<PathBuf>> {
    let value = manifest
        .package
        .as_ref()
        .and_then(|package| package.metadata.as_ref())
        .and_then(|metadata| metadata.get("pgrx"))
        .and_then(|pgrx| pgrx.get(UNRELEASED_SQL_DIR_KEY));

    match value {
        None => Ok(None),
        Some(cargo_toml::Value::String(dir)) => {
            let crate_dir = manifest_path.parent().unwrap_or_else(|| Path::new("."));
            Ok(Some(crate_dir.join(dir)))
        }
        Some(other) => Err(eyre!(
            "`package.metadata.pgrx.{UNRELEASED_SQL_DIR_KEY}` must be a path string, not `{other}`"
        )),
    }
}

/// The version `cargo pgrx install` gives the extension before any fragments are considered.
fn current_version(manifest_path: &Path, cargo_flags: &[String]) -> eyre::Result<ExtVersion> {
    let version = get_version(manifest_path, cargo_flags)?;
    ExtVersion::parse(&version).ok_or_else(|| {
        eyre!("the extension version `{version}` is not one cargo-pgrx can order for fragments")
    })
}

/// What the `migrate` subcommands need to know about a package that opted in.
pub(crate) struct MigrateContext {
    extname: String,
    manifest_path: PathBuf,
    control_file: PathBuf,
    sql_dir: PathBuf,
    unreleased_dir: PathBuf,
    current_version: ExtVersion,
}

impl MigrateContext {
    fn load(package: Option<&str>, manifest_path: Option<&Path>) -> eyre::Result<Self> {
        let features = clap_cargo::Features::default();
        let (manifest, manifest_path, _) =
            get_package_manifest(&features, package, manifest_path, &[])?;

        let unreleased_dir =
            resolve_unreleased_dir(&manifest, &manifest_path)?.ok_or_else(|| {
                eyre!(
                    "SQL migration fragments are not enabled for this package. To opt in, add this to `{}`:\n\n\
                     [package.metadata.pgrx]\n\
                     {UNRELEASED_SQL_DIR_KEY} = \"sql/unreleased\"",
                    manifest_path.display()
                )
            })?;

        let (control_file, extname) = find_control_file(&manifest_path)?;
        let sql_dir = manifest_path.parent().unwrap_or_else(|| Path::new(".")).join("sql");
        let current_version = current_version(&manifest_path, &[])?;

        Ok(Self { extname, manifest_path, control_file, sql_dir, unreleased_dir, current_version })
    }
}

/// How an installing command treats unreleased fragments.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum UnreleasedFragmentMode {
    /// Assemble them into an ephemeral upgrade script in the destination directory
    Assemble,
    /// Refuse to continue if any exist: the default for `cargo pgrx package`
    Disallow,
    /// Leave them out of the destination directory
    Skip,
}

impl UnreleasedFragmentMode {
    /// The mode for a development command with a `--no-assemble-unreleased` flag.
    pub(crate) fn for_development(no_assemble_unreleased: bool) -> Self {
        if no_assemble_unreleased { Self::Skip } else { Self::Assemble }
    }
}

/// The work [`plan_unreleased_fragments`] decided on, applied once the rest of the extension has
/// been copied into place.
pub(crate) enum UnreleasedFragments {
    /// The package didn't opt in.
    NotEnabled,
    /// The package opted in but this install has nothing to add.
    NothingToAssemble,
    Assemble(Box<EphemeralUpgrade>),
}

pub(crate) struct EphemeralUpgrade {
    /// The version of the base schema `cargo pgrx install` generates.
    current: ExtVersion,
    prev: ExtVersion,
    target: ExtVersion,
    fragments: Vec<MigrationFragment>,
}

/// Drops fragments that are already in a script for the latest version, such as ones kept with
/// `cargo pgrx migrate assemble --preserve-fragments`. Assembling them again would put the same
/// SQL on the update path twice.
fn retain_unassembled(
    fragments: &mut Vec<MigrationFragment>,
    scripts: &[UpgradeScript],
) -> eyre::Result<()> {
    let Some(latest) = scripts.last() else {
        return Ok(());
    };

    for script in scripts.iter().rev().take_while(|script| script.target == latest.target) {
        let sql = fs::read_to_string(&script.path)
            .wrap_err_with(|| format!("failed to read `{}`", script.path.display()))?;
        fragments.retain(|f| {
            let marker = fragment::fragment_marker(&f.filename);
            !sql.lines().any(|line| line == marker)
        });
    }
    Ok(())
}

/// Decides what to do with unreleased fragments before anything is built or copied, so that a bad
/// fragment or a refused package fails while the installed extension is still whole.
pub(crate) fn plan_unreleased_fragments(
    manifest: &Manifest,
    manifest_path: &Path,
    extname: &str,
    mode: UnreleasedFragmentMode,
    cargo_flags: &[String],
) -> eyre::Result<UnreleasedFragments> {
    let Some(unreleased_dir) = resolve_unreleased_dir(manifest, manifest_path)? else {
        return Ok(UnreleasedFragments::NotEnabled);
    };
    if mode == UnreleasedFragmentMode::Skip {
        return Ok(UnreleasedFragments::NothingToAssemble);
    }

    let sql_dir = manifest_path.parent().unwrap_or_else(|| Path::new(".")).join("sql");
    let scripts = version::existing_upgrade_scripts(&sql_dir, extname)?;

    let mut fragments = fragment::collect_fragments_from_dir(&unreleased_dir)?;
    retain_unassembled(&mut fragments, &scripts)?;
    if fragments.is_empty() {
        return Ok(UnreleasedFragments::NothingToAssemble);
    }

    if mode == UnreleasedFragmentMode::Disallow {
        return Err(eyre!(
            "found unreleased SQL migration fragments in `{}`.\n\
             Run `cargo pgrx migrate assemble` before packaging for release, pass `--assemble-unreleased` to include them, or pass `--no-assemble-unreleased` to leave them out.",
            unreleased_dir.display()
        ));
    }

    let current = current_version(manifest_path, cargo_flags)?;
    let (prev, target) = match scripts.last() {
        Some(latest) => (latest.target.clone(), version::next_version(&latest.target, &current)?),
        None => (current.clone(), current.next_patch()?),
    };

    Ok(UnreleasedFragments::Assemble(Box::new(EphemeralUpgrade {
        current,
        prev,
        target,
        fragments,
    })))
}

/// The first line of every file assembled for an installation only, which is how a later install
/// tells those files from released ones.
const EPHEMERAL_MARKER: &str =
    "-- Ephemeral: assembled by cargo-pgrx from unreleased SQL fragments.\n";

/// Finds what a previous install assembled in `extdir`. Those fragments may have changed, or be
/// gone entirely after a branch switch, and a stale script stays on the update path until it's
/// removed.
pub(crate) fn ephemeral_files(extname: &str, extdir: &Path) -> eyre::Result<Vec<PathBuf>> {
    let entries = match fs::read_dir(extdir) {
        Ok(entries) => entries,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(err) => {
            return Err(err)
                .wrap_err_with(|| format!("failed to read directory `{}`", extdir.display()));
        }
    };

    let prefix = format!("{extname}--");
    let mut found = Vec::new();
    for entry in entries {
        let path = entry?.path();
        let is_extension_sql = path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.starts_with(&prefix) && name.ends_with(".sql"));
        if !is_extension_sql {
            continue;
        }

        let mut head = Vec::with_capacity(EPHEMERAL_MARKER.len());
        fs::File::open(&path)
            .and_then(|file| file.take(EPHEMERAL_MARKER.len() as u64).read_to_end(&mut head))
            .wrap_err_with(|| format!("failed to read `{}`", path.display()))?;

        if head == EPHEMERAL_MARKER.as_bytes() {
            found.push(path);
        }
    }
    Ok(found)
}

fn remove_ephemeral_files(extname: &str, extdir: &Path) -> eyre::Result<()> {
    for path in ephemeral_files(extname, extdir)? {
        fs::remove_file(&path)
            .wrap_err_with(|| format!("failed to remove `{}`", path.display()))?;
    }
    Ok(())
}

impl UnreleasedFragments {
    pub(crate) fn apply(
        self,
        manifest_path: &Path,
        extname: &str,
        extdir: &Path,
        target_dir: &Path,
        cargo_flags: &[String],
        output_tracking: &mut Vec<PathBuf>,
    ) -> eyre::Result<()> {
        match self {
            Self::NotEnabled => Ok(()),
            Self::NothingToAssemble => remove_ephemeral_files(extname, extdir),
            Self::Assemble(upgrade) => {
                remove_ephemeral_files(extname, extdir)?;
                upgrade.install(
                    manifest_path,
                    extname,
                    extdir,
                    target_dir,
                    cargo_flags,
                    output_tracking,
                )
            }
        }
    }
}

impl EphemeralUpgrade {
    fn install(
        &self,
        manifest_path: &Path,
        extname: &str,
        extdir: &Path,
        target_dir: &Path,
        cargo_flags: &[String],
        output_tracking: &mut Vec<PathBuf>,
    ) -> eyre::Result<()> {
        let Self { current, prev, target, fragments } = self;
        let target_label = target.to_string();

        let script_path = extdir.join(format!("{extname}--{prev}--{target}.sql"));
        let script = fragment::assemble_sql_content(extname, &target_label, fragments);
        // A released script gets this when `cargo pgrx install` copies it out of `sql/`.
        let script = filter_contents(manifest_path, script, cargo_flags)?;
        fs::write(&script_path, format!("{EPHEMERAL_MARKER}{script}")).wrap_err_with(|| {
            format!("failed to write ephemeral upgrade script `{}`", script_path.display())
        })?;
        output_tracking.push(script_path.clone());
        println!(
            "{} unreleased fragments for {} -> {} into {}",
            "  Assembling".bold().green(),
            prev.cyan(),
            target.cyan(),
            format_display_path(&script_path, target_dir).cyan()
        );

        // Without a base schema of its own, `CREATE EXTENSION` would install `current` and then
        // run the upgrade script on top of it. The generated schema already has today's objects,
        // so that fails with "already exists".
        if current != target {
            let current_schema = extdir.join(format!("{extname}--{current}.sql"));
            let target_schema = extdir.join(format!("{extname}--{target}.sql"));
            let schema = fs::read_to_string(&current_schema).wrap_err_with(|| {
                format!("failed to read generated schema `{}`", current_schema.display())
            })?;
            fs::write(&target_schema, format!("{EPHEMERAL_MARKER}{schema}")).wrap_err_with(
                || format!("failed to write base schema `{}`", target_schema.display()),
            )?;
            output_tracking.push(target_schema.clone());
            println!(
                "{} base schema for {} at {}",
                "   Installed".bold().green(),
                target.cyan(),
                format_display_path(&target_schema, target_dir).cyan()
            );

            let control = extdir.join(format!("{extname}.control"));
            version::update_control_file_default_version(&control, &target_label)?;
            println!(
                "{} default_version = '{}' in {}",
                "     Updated".bold().green(),
                target.cyan(),
                format_display_path(&control, target_dir).cyan()
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ENABLED: &str = "\
[package]
name = \"my_ext\"
version = \"0.25.0\"

[package.metadata.pgrx]
unreleased-sql-dir = \"sql/unreleased\"
";

    const NOT_ENABLED: &str = "\
[package]
name = \"my_ext\"
version = \"0.25.0\"
";

    /// A crate on disk, with an extension directory holding what `cargo pgrx install` had already
    /// put there when fragments are applied.
    struct Fixture {
        temp: tempfile::TempDir,
        manifest: Manifest,
        manifest_path: PathBuf,
        extdir: PathBuf,
    }

    impl Fixture {
        fn new(toml: &str, fragments: &[(&str, &str)]) -> Self {
            let temp = tempfile::tempdir().unwrap();
            let crate_dir = temp.path().join("my_ext");
            let unreleased_dir = crate_dir.join("sql/unreleased");
            let extdir = temp.path().join("share/extension");
            fs::create_dir_all(&unreleased_dir).unwrap();
            fs::create_dir_all(crate_dir.join("src")).unwrap();
            fs::create_dir_all(&extdir).unwrap();

            let manifest_path = crate_dir.join("Cargo.toml");
            fs::write(&manifest_path, toml).unwrap();
            fs::write(crate_dir.join("src/lib.rs"), "").unwrap();
            fs::write(crate_dir.join("my_ext.control"), "default_version = '@CARGO_VERSION@'\n")
                .unwrap();
            fs::write(extdir.join("my_ext.control"), "default_version = '0.25.0'\ncomment = 'c'\n")
                .unwrap();
            fs::write(extdir.join("my_ext--0.25.0.sql"), "-- full base schema").unwrap();
            for (name, sql) in fragments {
                fs::write(unreleased_dir.join(name), sql).unwrap();
            }

            let manifest = Manifest::from_str(toml).unwrap();
            Self { temp, manifest, manifest_path, extdir }
        }

        fn sql_dir(&self) -> PathBuf {
            self.manifest_path.parent().unwrap().join("sql")
        }

        fn plan(&self, mode: UnreleasedFragmentMode) -> eyre::Result<UnreleasedFragments> {
            plan_unreleased_fragments(&self.manifest, &self.manifest_path, "my_ext", mode, &[])
        }

        fn install(&self, mode: UnreleasedFragmentMode) -> Vec<PathBuf> {
            let mut tracking = Vec::new();
            self.plan(mode)
                .unwrap()
                .apply(
                    &self.manifest_path,
                    "my_ext",
                    &self.extdir,
                    self.temp.path(),
                    &[],
                    &mut tracking,
                )
                .unwrap();
            tracking
        }

        fn installed(&self) -> Vec<String> {
            let mut names: Vec<_> = fs::read_dir(&self.extdir)
                .unwrap()
                .map(|e| e.unwrap().file_name().into_string().unwrap())
                .collect();
            names.sort();
            names
        }
    }

    const FRAGMENT: &[(&str, &str)] = &[("101.add_func.sql", "CREATE FUNCTION f() RETURNS void;")];

    #[test]
    fn the_directory_comes_from_metadata() {
        let toml = ENABLED.replace("sql/unreleased", "custom/migrations");
        let manifest = Manifest::from_str(&toml).unwrap();
        assert_eq!(
            resolve_unreleased_dir(&manifest, Path::new("/some/dir/Cargo.toml")).unwrap(),
            Some(PathBuf::from("/some/dir/custom/migrations"))
        );
    }

    #[test]
    fn without_metadata_the_feature_is_off() {
        let manifest = Manifest::from_str(NOT_ENABLED).unwrap();
        assert_eq!(
            resolve_unreleased_dir(&manifest, Path::new("/some/dir/Cargo.toml")).unwrap(),
            None
        );
    }

    #[test]
    fn a_malformed_key_is_an_error_rather_than_quietly_off() {
        let toml = ENABLED.replace("\"sql/unreleased\"", "true");
        let manifest = Manifest::from_str(&toml).unwrap();
        assert!(resolve_unreleased_dir(&manifest, Path::new("/some/dir/Cargo.toml")).is_err());
    }

    #[test]
    fn fragments_are_ignored_until_the_package_opts_in() {
        let fixture = Fixture::new(NOT_ENABLED, FRAGMENT);
        for mode in [
            UnreleasedFragmentMode::Assemble,
            UnreleasedFragmentMode::Disallow,
            UnreleasedFragmentMode::Skip,
        ] {
            assert!(matches!(fixture.plan(mode).unwrap(), UnreleasedFragments::NotEnabled));
        }
        assert!(fixture.install(UnreleasedFragmentMode::Assemble).is_empty());
        assert_eq!(fixture.installed(), ["my_ext--0.25.0.sql", "my_ext.control"]);
    }

    #[test]
    fn disallow_refuses_fragments() {
        let fixture = Fixture::new(ENABLED, FRAGMENT);
        let Err(err) = fixture.plan(UnreleasedFragmentMode::Disallow) else {
            panic!("expected fragments to be refused");
        };
        let message = err.to_string();
        assert!(message.contains("found unreleased SQL migration fragments"));
        assert!(message.contains("cargo pgrx migrate assemble"));
        assert!(message.contains("--no-assemble-unreleased"));
    }

    #[test]
    fn skip_does_not_read_fragments() {
        // A cycle would fail if the fragments were read.
        let fixture = Fixture::new(
            ENABLED,
            &[("101.a.sql", "-- depends-on: 102\n"), ("102.b.sql", "-- depends-on: 101\n")],
        );
        assert!(fixture.plan(UnreleasedFragmentMode::Assemble).is_err());
        assert!(fixture.install(UnreleasedFragmentMode::Skip).is_empty());
    }

    #[test]
    fn assembles_upgrade_script_base_schema_and_control() {
        let fixture = Fixture::new(ENABLED, FRAGMENT);
        fs::write(fixture.sql_dir().join("my_ext--0.24.0--0.25.0.sql"), "-- old").unwrap();

        let tracking = fixture.install(UnreleasedFragmentMode::Assemble);

        let script = fixture.extdir.join("my_ext--0.25.0--0.25.1.sql");
        let base = fixture.extdir.join("my_ext--0.25.1.sql");
        assert_eq!(tracking, [script.clone(), base.clone()]);

        let sql = fs::read_to_string(&script).unwrap();
        assert!(sql.contains("ALTER EXTENSION my_ext UPDATE TO '0.25.1'"));
        assert!(sql.contains("CREATE FUNCTION f()"));
        assert!(fs::read_to_string(&base).unwrap().ends_with("-- full base schema"));

        let control = fs::read_to_string(fixture.extdir.join("my_ext.control")).unwrap();
        assert!(control.contains("default_version = '0.25.1'"));
    }

    #[test]
    fn a_version_already_ahead_of_sql_needs_no_second_base_schema() {
        let fixture = Fixture::new(&ENABLED.replace("0.25.0", "0.26.0"), FRAGMENT);
        fs::write(fixture.sql_dir().join("my_ext--0.24.0--0.25.0.sql"), "-- old").unwrap();
        fs::write(fixture.extdir.join("my_ext--0.26.0.sql"), "-- full base schema").unwrap();

        let tracking = fixture.install(UnreleasedFragmentMode::Assemble);

        assert_eq!(tracking, [fixture.extdir.join("my_ext--0.25.0--0.26.0.sql")]);
    }

    #[test]
    fn the_generated_schema_follows_the_control_file_not_the_crate() {
        // A hard-coded default_version names the generated schema. Using the crate version here
        // would skip the base schema copy, and CREATE EXTENSION would then fail.
        let fixture = Fixture::new(&ENABLED.replace("0.25.0", "0.26.0"), FRAGMENT);
        let control = fixture.manifest_path.parent().unwrap().join("my_ext.control");
        fs::write(control, "default_version = '0.25.0'\n").unwrap();
        fs::write(fixture.sql_dir().join("my_ext--0.24.0--0.25.0.sql"), "-- old").unwrap();

        fixture.install(UnreleasedFragmentMode::Assemble);

        assert!(fixture.extdir.join("my_ext--0.25.0--0.25.1.sql").exists());
        assert!(fixture.extdir.join("my_ext--0.25.1.sql").exists());
    }

    #[test]
    fn placeholders_are_filled_in_like_a_released_script() {
        let fixture = Fixture::new(ENABLED, &[("101.v.sql", "SELECT '@CARGO_VERSION@';")]);
        fixture.install(UnreleasedFragmentMode::Assemble);
        let sql = fs::read_to_string(fixture.extdir.join("my_ext--0.25.0--0.25.1.sql")).unwrap();
        assert!(!sql.contains("@CARGO_VERSION@"), "{sql}");
    }

    #[test]
    fn fragments_already_in_the_latest_script_are_not_assembled_again() {
        let fixture = Fixture::new(ENABLED, FRAGMENT);
        let released = fragment::assemble_sql_content(
            "my_ext",
            "0.25.0",
            &fragment::collect_fragments_from_dir(&fixture.sql_dir().join("unreleased")).unwrap(),
        );
        fs::write(fixture.sql_dir().join("my_ext--0.24.0--0.25.0.sql"), released).unwrap();

        assert!(matches!(
            fixture.plan(UnreleasedFragmentMode::Disallow).unwrap(),
            UnreleasedFragments::NothingToAssemble
        ));
        assert!(fixture.install(UnreleasedFragmentMode::Assemble).is_empty());
    }

    #[test]
    fn a_later_install_removes_what_an_earlier_one_assembled() {
        let fixture = Fixture::new(ENABLED, FRAGMENT);
        fixture.install(UnreleasedFragmentMode::Assemble);
        assert!(fixture.extdir.join("my_ext--0.25.0--0.25.1.sql").exists());

        fs::remove_file(fixture.sql_dir().join("unreleased/101.add_func.sql")).unwrap();
        fs::write(fixture.extdir.join("my_ext--0.24.0--0.25.0.sql"), "-- released").unwrap();
        fixture.install(UnreleasedFragmentMode::Assemble);

        assert_eq!(
            fixture.installed(),
            ["my_ext--0.24.0--0.25.0.sql", "my_ext--0.25.0.sql", "my_ext.control"]
        );
    }
}
