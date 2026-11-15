//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use crate::CommandExecute;
use crate::bundle::Bundle;
use crate::cargo::CargoProfile;
use crate::command::get::{find_control_file, get_property};
use crate::command::migrate::{UnreleasedFragmentMode, plan_unreleased_fragments};
use crate::command::sudo_install::SudoInstall;
use crate::manifest::PgrxManifestExt;
use crate::manifest::{PgVersionSource, display_version_info, program_pg_config};
use crate::target::TargetOs;
use cargo_metadata::{CrateType, Message as CargoMessage};
use cargo_toml::Manifest;
use eyre::{WrapErr, eyre};
use owo_colors::OwoColorize;
use pgrx_pg_config::{PgConfig, Pgrx};
use std::collections::HashMap;
use std::fs;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::{Arc, Mutex, OnceLock};

/// Type used for memoizing expensive to calculate values.
/// `Arc<Mutex>` is needed to get around compiler safety checks.
type MemoizeKeyValue = Arc<Mutex<HashMap<PathBuf, String>>>;

/// Install the crate as an extension into the Postgres specified by `pg_config`
#[derive(clap::Args, Debug)]
#[clap(author)]
pub(crate) struct Install {
    /// Package to build (see `cargo help pkgid`)
    #[clap(long, short)]
    pub(crate) package: Option<String>,
    /// Path to Cargo.toml
    #[clap(long, value_parser)]
    pub(crate) manifest_path: Option<PathBuf>,
    /// Compile for release mode (default is debug)
    #[clap(long, short)]
    pub(crate) release: bool,
    /// Specific profile to use (conflicts with `--release`)
    #[clap(long)]
    pub(crate) profile: Option<String>,
    /// Build in test mode (for `cargo pgrx test`)
    #[clap(long)]
    pub(crate) test: bool,
    /// The `pg_config` path (default is first in $PATH)
    #[clap(long, short = 'c')]
    pub(crate) pg_config: Option<String>,
    /// Use `sudo` to install the extension artifacts
    #[clap(long, short = 's')]
    sudo: bool,
    #[clap(flatten)]
    pub(crate) features: clap_cargo::Features,
    /// Don't assemble unreleased SQL fragments into the installed extension
    #[clap(long, env = "PGRX_NO_ASSEMBLE_UNRELEASED")]
    pub(crate) no_assemble_unreleased: bool,
    /// Extra cargo flags forwarded to every `cargo` invocation. Repeatable and split on whitespace: `--cargo=--config=foo` or `--cargo "--offline --frozen"`.
    #[clap(long = "cargo", value_name = "FLAG", allow_hyphen_values = true)]
    pub(crate) cargo: Vec<String>,
    #[clap(from_global, action = ArgAction::Count)]
    pub(crate) verbose: u8,
}

impl CommandExecute for Install {
    #[tracing::instrument(level = "error", skip(self))]
    fn execute(mut self) -> eyre::Result<()> {
        warn_if_pg_bench_enabled(&self.features, "install");
        if self.sudo {
            // user wishes to use `sudo` to install the extension
            // so we re-route through the `SudoInstall` type
            let sudo_install = SudoInstall::from(self);
            return sudo_install.execute();
        }

        let cargo_flags = std::mem::take(&mut self.cargo);
        let metadata =
            crate::metadata::metadata(&self.features, self.manifest_path.as_deref(), &cargo_flags)
                .wrap_err("couldn't get cargo metadata")?;
        crate::metadata::validate(self.manifest_path.as_deref(), &metadata)?;
        let package_manifest_path =
            crate::manifest::manifest_path(&metadata, self.package.as_deref())
                .wrap_err("Couldn't get manifest path")?;
        let package_manifest =
            Manifest::from_path(&package_manifest_path).wrap_err("Couldn't parse manifest")?;

        let pg_config = program_pg_config(self.pg_config.take().map(PathBuf::from));
        let pg_version = format!("pg{}", pg_config.major_version()?);
        let profile = CargoProfile::from_flags(
            self.profile.as_deref(),
            if self.release { CargoProfile::Release } else { CargoProfile::Dev },
        )?;

        crate::manifest::modify_features_for_version(
            // Only the feature names are needed, so no `config.toml` is read.
            &Pgrx::default(),
            Some(&mut self.features),
            &package_manifest,
            &PgVersionSource::PgConfig(pg_version),
            self.test,
        );

        display_version_info(&pg_config, &PgVersionSource::PgConfig(pg_config.label()?));
        let unreleased_mode = UnreleasedFragmentMode::for_development(self.no_assemble_unreleased);
        install_extension(
            self.manifest_path.as_deref(),
            self.package.as_deref(),
            &package_manifest_path,
            &profile,
            self.test,
            None,
            &self.features,
            Platform::Host(&pg_config),
            unreleased_mode,
            metadata.target_directory.as_std_path(),
            &cargo_flags,
            None,
        )?;
        Ok(())
    }
}

pub(crate) fn pg_bench_feature_enabled(features: &clap_cargo::Features) -> bool {
    features.all_features || features.features.iter().any(|feature| feature == "pg_bench")
}

pub(crate) fn warn_if_pg_bench_enabled(features: &clap_cargo::Features, command: &str) {
    if !pg_bench_feature_enabled(features) {
        return;
    }

    eprintln!(
        "{} building with feature `pg_bench`\nbenchmark functions and helper dependencies will be included in this `cargo pgrx {command}` build\nthis is usually not intended for packaged releases",
        "WARNING:".red().bold()
    );
}

#[tracing::instrument(skip_all, fields(
    pg_version = %platform.pg_config().version()?,
    profile = ?profile,
    test = is_test,
    base_directory = tracing::field::Empty,
    features = ?features.features,
))]
pub(crate) fn install_extension(
    user_manifest_path: Option<&Path>,
    user_package: Option<&str>,
    package_manifest_path: &Path,
    profile: &CargoProfile,
    is_test: bool,
    base_directory: Option<PathBuf>,
    features: &clap_cargo::Features,
    platform: Platform<'_>,
    unreleased_mode: UnreleasedFragmentMode,
    target_dir: &Path,
    cargo_flags: &[String],
    prefix_directory: Option<PathBuf>,
) -> eyre::Result<Vec<PathBuf>> {
    let mut output_tracking = Vec::new();
    let (pg_config, target) = (platform.pg_config(), platform.target());

    let manifest = Manifest::from_path(package_manifest_path)?;
    let (control_file, extname) = find_control_file(package_manifest_path)?;

    let unreleased_fragments = plan_unreleased_fragments(
        &manifest,
        package_manifest_path,
        &extname,
        unreleased_mode,
        cargo_flags,
    )?;
    let effective_version =
        unreleased_fragments.effective_version(package_manifest_path, cargo_flags)?;

    let build_manifest_path =
        manifest_path_for_build(user_manifest_path, user_package, package_manifest_path);
    let build_command_output = build_extension(
        platform,
        build_manifest_path,
        user_package,
        profile,
        features,
        EffectiveExtVersion(&effective_version),
        cargo_flags,
    )?;
    let build_command_bytes = build_command_output.stdout;
    let build_command_reader = BufReader::new(build_command_bytes.as_slice());
    let build_command_stream = CargoMessage::parse_stream(build_command_reader);
    let build_command_messages =
        build_command_stream.collect::<Result<Vec<_>, std::io::Error>>()?;

    println!("{} extension", "  Installing".bold().green());
    let shlibpath =
        find_library_file(&manifest, package_manifest_path, &build_command_messages, target)?;

    let relative_extdir = if let Some(prefix_directory) = prefix_directory.as_ref() {
        make_relative_extdir(prefix_directory.to_path_buf(), target)
    } else {
        make_relative_extdir(pg_config.extension_dir()?, target)
    };

    let relative_pkglibdir = if let Some(prefix_directory) = prefix_directory.as_ref() {
        make_relative_pkglibdir(prefix_directory.to_path_buf(), target)
    } else {
        make_relative_pkglibdir(pg_config.pkglibdir()?, target)
    };

    let extdir = if let Some(base_directory) = base_directory.as_ref() {
        base_directory.join(relative_extdir)
    } else {
        pg_config.extension_dir()?
    };

    let pkglibdir = if let Some(base_directory) = base_directory.as_ref() {
        base_directory.join(relative_pkglibdir)
    } else {
        pg_config.pkglibdir()?
    };

    {
        let filename = control_file
            .file_name()
            .ok_or_else(|| eyre!("Could not get filename for `{}`", control_file.display()))?;
        let dest = extdir.join(filename);
        copy_file(
            &control_file,
            dest,
            "control file",
            true,
            package_manifest_path,
            &mut output_tracking,
            target_dir,
            cargo_flags,
        )?;
    }

    {
        let dest =
            pkglibdir.join(shared_library_filename(package_manifest_path, platform, cargo_flags)?);

        // Remove the existing shared libraries if present. This is a workaround for an
        // issue highlighted by the following apple documentation:
        // https://developer.apple.com/documentation/security/updating_mac_software
        //
        // for Linux, dlopen(2) will use mmap to load the .so.
        // if update the file in place, the modification will pass into the running
        // process which will mash up all pointers in the .TEXT segment.
        // this simulate linux's install(1) behavior
        if dest.exists() {
            fs::remove_file(&dest)
                .wrap_err_with(|| format!("unable to remove existing file {}", dest.display()))?;
        }

        copy_file(
            &shlibpath,
            dest,
            "shared library",
            false,
            package_manifest_path,
            &mut output_tracking,
            target_dir,
            cargo_flags,
        )?;
    }

    // The schema comes from the library Cargo reported building, wherever Cargo put it.
    copy_sql_files(
        &manifest,
        package_manifest_path,
        &shlibpath,
        is_test,
        target,
        target_dir,
        &extdir,
        &mut output_tracking,
        cargo_flags,
    )?;

    unreleased_fragments.apply(
        package_manifest_path,
        &extname,
        &extdir,
        target_dir,
        cargo_flags,
        &mut output_tracking,
    )?;

    println!("{} installing {}", "    Finished".bold().green(), extname);
    Ok(output_tracking)
}

/// The file name of the extension's shared library in `pkglibdir`.
pub(crate) fn shared_library_filename(
    package_manifest_path: &Path,
    platform: Platform<'_>,
    cargo_flags: &[String],
) -> eyre::Result<String> {
    let (_, extname) = find_control_file(package_manifest_path)?;
    let so_name = if get_property(package_manifest_path, "module_pathname")?.is_none() {
        let extver = get_version(package_manifest_path, cargo_flags)?;
        // note: versioned so-name format must agree with pgrx-utils
        format!("{extname}-{extver}")
    } else {
        extname
    };
    // Since Postgres 16, the shared library extension on macOS is `dylib`, not `so`.
    // Ref https://github.com/postgres/postgres/commit/b55f62abb2c2e07dfae99e19a2b3d7ca9e58dc1a
    let target_os = TargetOs::from_target(platform.target());
    let extension = if target_os == TargetOs::Darwin && platform.pg_config().major_version()? < 16 {
        "so"
    } else {
        target_os.dll_extension()
    };
    Ok(format!("{so_name}.{extension}"))
}

fn copy_file(
    src: &Path,
    dest: PathBuf,
    msg: &str,
    do_filter: bool,
    package_manifest_path: &Path,
    output_tracking: &mut Vec<PathBuf>,
    target_dir: &Path,
    cargo_flags: &[String],
) -> eyre::Result<()> {
    let Some(dest_dir) = dest.parent() else {
        // what fresh hell could ever cause such an error?
        eyre::bail!("no directory to copy to: {}", dest.display());
    };
    match dest_dir.try_exists() {
        Ok(false) => fs::create_dir_all(dest_dir).wrap_err_with(|| {
            format!("failed to create destination directory {}", dest_dir.display())
        })?,
        Ok(true) => (),
        Err(e) => Err(e).wrap_err_with(|| {
            format!("failed to access {}, is it write-enabled?", dest_dir.display())
        })?,
    };

    println!(
        "{} {} to {}",
        "     Copying".bold().green(),
        msg,
        format_display_path(&dest, target_dir).cyan()
    );

    if do_filter {
        // we want to filter the contents of the file we're to copy
        let input = fs::read_to_string(src)
            .wrap_err_with(|| format!("failed to read `{}`", src.display()))?;
        let input = filter_contents(package_manifest_path, input, cargo_flags)?;

        fs::write(&dest, input).wrap_err_with(|| {
            format!("failed writing `{}` to `{}`", src.display(), dest.display())
        })?;
    } else {
        fs::copy(src, &dest).wrap_err_with(|| {
            format!("failed copying `{}` to `{}`", src.display(), dest.display())
        })?;
    }

    output_tracking.push(dest);

    Ok(())
}

/// The `default_version` that the installation will write to `.control`, exposed to the extension
/// build as `CARGO_PGRX_EXTVERSION`.
#[derive(Debug, Clone, Copy)]
pub(crate) struct EffectiveExtVersion<'a>(pub(crate) &'a str);

/// The platform an extension is built for and its PostgreSQL, and where its `pgrx-pg-sys` gets
/// the sources it would otherwise generate from this machine's PostgreSQL headers.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Platform<'a> {
    /// This machine and the PostgreSQL that `pg_config` describes, generating as an ordinary
    /// build does.
    Host(&'a PgConfig),
    /// The same, writing what `pgrx-pg-sys` generates to this directory to go into a bundle,
    /// and checking the extension instead of building it.
    HostForBundle(&'a PgConfig, &'a Path),
    /// A bundle's platform and PostgreSQL. A full bundle's sources are unpacked for the build to
    /// import; a partial bundle has none, and the build generates from its headers.
    Bundle(&'a Bundle),
}

impl<'a> Platform<'a> {
    /// The PostgreSQL to build against, which also decides where the extension's files go.
    pub(crate) fn pg_config(self) -> &'a PgConfig {
        match self {
            Self::Host(pg_config) | Self::HostForBundle(pg_config, _) => pg_config,
            Self::Bundle(bundle) => &bundle.pg_config,
        }
    }

    /// The Rust target to build for, or `None` for this machine.
    pub(crate) fn target(self) -> Option<&'a str> {
        match self {
            Self::Host(_) | Self::HostForBundle(..) => None,
            Self::Bundle(bundle) => Some(&bundle.target),
        }
    }
}

/// Remove the variables this process inherited whose names start with one of `prefixes`, in
/// any case: Windows reads variable names without regard to case.
fn remove_inherited(command: &mut std::process::Command, prefixes: &[&str]) {
    for (name, _) in std::env::vars_os() {
        let upper = name.to_string_lossy().to_ascii_uppercase();
        if prefixes.iter().any(|prefix| upper.starts_with(prefix)) {
            command.env_remove(name);
        }
    }
}

/// Tell the `pgrx-pg-sys` build script what to do about a bundle. It reads these variables;
/// only cargo-pgrx sets them, so ones left in this process's environment are removed.
fn select_target_info(
    command: &mut std::process::Command,
    platform: Platform<'_>,
) -> eyre::Result<()> {
    const EXPORT: &str = "PGRX_PG_SYS_EXTRA_TARGET_INFO_PATH";
    const IMPORT: &str = "PGRX_TARGET_INFO_PATH";
    remove_inherited(command, &[EXPORT, IMPORT]);
    if !matches!(platform, Platform::Host(_)) {
        // The build script would read server headers from these in place of the ones the
        // PostgreSQL names, and a bundle's must come from its target. They serve plain `cargo`
        // builds, so builds for this machine leave them as they were.
        remove_inherited(command, &["PGRX_INCLUDEDIR_SERVER", "PGRX_PKGINCLUDEDIR"]);
    }
    match platform {
        Platform::Host(_) | Platform::Bundle(Bundle { generated: None, .. }) => {}
        Platform::HostForBundle(_, directory) => {
            command.env(EXPORT, directory);
        }
        Platform::Bundle(Bundle { generated: Some(generated), pg_config, .. }) => {
            command.env(format!("{IMPORT}_PG{}", pg_config.major_version()?), &generated.directory);
        }
    }
    Ok(())
}

/// Have the `pgrx-pg-sys` build script describe the same PostgreSQL as `pg_config`, which
/// chose the feature flags and decides where the extension's files go.
fn select_build_postgres(command: &mut std::process::Command, pg_config: &PgConfig) {
    const METADATA: &str = "PGRX_PG_CONFIG_";
    let Some(program) = pg_config.path() else {
        // A described installation reaches the build script as its environment metadata, with
        // nothing left over from this process's own environment.
        let Some(properties) = pg_config.properties() else { return };
        remove_inherited(command, &[METADATA]);
        command.env("PGRX_PG_CONFIG_AS_ENV", "true");
        for (name, value) in properties {
            command.env(format!("{METADATA}{}", name.to_uppercase()), value);
        }
        return;
    };
    // The build script would prefer metadata in the environment to this program.
    command.env_remove("PGRX_PG_CONFIG_AS_ENV");
    let same_file = |other: PathBuf| match (program.canonicalize(), other.canonicalize()) {
        (Ok(program), Ok(other)) => program == other,
        _ => program == other,
    };
    // Leave the variable alone when the build script finds this program by itself, as it does
    // for an installation in `config.toml`: changing it reruns the script, which regenerates
    // the bindings, in every build that alternates with one that did not set it.
    let found_by_build_script = pg_config
        .label()
        .and_then(|label| Pgrx::from_config()?.get(&label))
        .ok()
        .and_then(|found| found.path())
        .is_some_and(same_file);
    if !found_by_build_script {
        // The build script runs in the crate's directory, not this one.
        let program = if program.components().count() > 1 {
            std::path::absolute(&program).unwrap_or(program)
        } else {
            program
        };
        command.env("PGRX_PG_CONFIG_PATH", program);
    }
}

pub(crate) fn build_extension(
    platform: Platform<'_>,
    manifest_path: Option<&Path>,
    user_package: Option<&str>,
    profile: &CargoProfile,
    features: &clap_cargo::Features,
    extversion: EffectiveExtVersion<'_>,
    cargo_flags: &[String],
) -> eyre::Result<std::process::Output> {
    let flags = std::env::var("PGRX_BUILD_FLAGS").unwrap_or_default();

    let mut command = crate::cargo::cargo();
    command.env("CARGO_PGRX_EXTVERSION", extversion.0);
    select_build_postgres(&mut command, platform.pg_config());
    select_target_info(&mut command, platform)?;
    // A bundle needs only what `pgrx-pg-sys`'s build script generates. Checking the library
    // runs every build script with the features a build would use, without compiling the
    // extension itself.
    let exporting = matches!(platform, Platform::HostForBundle(..));
    command.arg(if exporting { "check" } else { "rustc" });
    command.arg("--lib");

    if let Some(manifest_path) = manifest_path {
        command.arg("--manifest-path");
        command.arg(manifest_path);
    }

    if let Some(user_package) = user_package {
        command.arg("--package");
        command.arg(user_package);
    }
    command.args(profile.cargo_args());

    let features_arg = features.features.join(" ");
    if !features_arg.trim().is_empty() {
        command.arg("--features");
        command.arg(&features_arg);
    }

    if features.no_default_features {
        command.arg("--no-default-features");
    }

    if features.all_features {
        command.arg("--all-features");
    }

    command.arg("--message-format=json-render-diagnostics");

    for arg in flags.split_ascii_whitespace() {
        command.arg(arg);
    }

    for arg in crate::metadata::split_cargo_flags(cargo_flags) {
        command.arg(arg);
    }

    let target = platform.target();
    if let Some(target) = target {
        command.arg("--target");
        command.arg(target);
    }

    let rustc_args = crate::cargo::pgrx_cdylib_rustc_args(target);
    if !exporting && !rustc_args.is_empty() {
        command.arg("--");
        command.args(rustc_args);
    }

    let command = command.stderr(Stdio::inherit());
    let command_str = format!("{command:?}");
    let verb = if exporting { "    Checking" } else { "    Building" };
    println!("{} extension with features {}", verb.bold().green(), features_arg.cyan());
    println!("{} command {}", "     Running".bold().green(), command_str.cyan());
    let cargo_output =
        command.output().wrap_err_with(|| format!("failed to spawn cargo: {command_str}"))?;
    if !cargo_output.status.success() {
        // We explicitly do not want to return a spantraced error here.
        std::process::exit(1)
    } else {
        Ok(cargo_output)
    }
}

pub(crate) fn manifest_path_for_build<'a>(
    user_manifest_path: Option<&'a Path>,
    user_package: Option<&str>,
    package_manifest_path: &'a Path,
) -> Option<&'a Path> {
    match user_package {
        Some(_) => user_manifest_path,
        None => Some(package_manifest_path),
    }
}

/// Write the schema of the library just built, and copy the extension's upgrade scripts.
fn copy_sql_files(
    manifest: &Manifest,
    package_manifest_path: &Path,
    library: &Path,
    is_test: bool,
    target: Option<&str>,
    target_dir: &Path,
    extdir: &Path,
    output_tracking: &mut Vec<PathBuf>,
    cargo_flags: &[String],
) -> eyre::Result<()> {
    let (_, extname) = find_control_file(package_manifest_path)?;
    {
        let version = get_version(package_manifest_path, cargo_flags)?;
        let filename = format!("{extname}--{version}.sql");
        let dest = extdir.join(filename);

        crate::command::schema::generate_schema_implicit(
            package_manifest_path,
            library,
            is_test,
            target,
            Some(&dest),
            None,
            None,
            // install scripts run inside CREATE EXTENSION and auto-attach;
            // explicit ALTER EXTENSION would be redundant.
            false,
            output_tracking,
            manifest,
        )?;
    }

    // now copy all the version upgrade files too
    if let Ok(dir) = fs::read_dir(package_manifest_path.parent().unwrap().join("sql/")) {
        for sql in dir.flatten() {
            let filename = sql.file_name().into_string().unwrap();

            // match the required pattern `extension--old_version--target_version.sql`
            let re_update_script_name =
                regex::Regex::new(&format!(r"^{extname}--.+--.+\.sql$")).unwrap();

            if re_update_script_name.is_match(filename.as_str()) {
                copy_file(
                    &sql.path(),
                    extdir.join(filename),
                    "extension schema upgrade file",
                    true,
                    package_manifest_path,
                    output_tracking,
                    target_dir,
                    cargo_flags,
                )?;
            }
        }
    }
    Ok(())
}

#[tracing::instrument(level = "error", skip_all)]
pub(crate) fn find_library_file(
    manifest: &Manifest,
    manifest_path: &Path,
    build_command_messages: &[CargoMessage],
    target: Option<&str>,
) -> eyre::Result<PathBuf> {
    let dll_extension = TargetOs::from_target(target).dll_extension();

    let manifest_path = std::path::absolute(manifest_path)?;
    let lib_filename = manifest.lib_filename(target)?;

    // no hard and fast rule for the lib.so output filename exists, so we implement this routine
    // which is essentially a cope for cargo's disinterest in writing down any docs so far.
    // you might think this is being silly but they do periodically change outputs. these changes
    // often seem to be unintentional, but they're real, so...
    let library_file = build_command_messages
        .iter()
        .filter_map(|msg| match msg {
            CargoMessage::CompilerArtifact(artifact) => Some(artifact),
            _ => None,
        })
        // normalize being flattened and low to the ground
        .find(|artifact| {
            artifact.manifest_path == manifest_path
                && artifact.target.crate_types.iter().any(|s| *s == CrateType::CDyLib)
        })
        .and_then(|artifact| {
            artifact
                .filenames
                .iter()
                .find(|filename| filename.extension() == Some(dll_extension))
                .map(|filename| filename.to_string())
        })
        .ok_or_else(|| {
            eyre!("Could not get shared object file `{lib_filename}` from Cargo output.",)
        })?;
    let library_file_path = PathBuf::from(library_file);

    Ok(library_file_path)
}

static CARGO_VERSION: OnceLock<MemoizeKeyValue> = OnceLock::new();

pub(crate) fn get_version(manifest_path: &Path, cargo_flags: &[String]) -> eyre::Result<String> {
    let path_string = manifest_path.to_owned();

    if let Some(version) =
        CARGO_VERSION.get_or_init(Default::default).lock().unwrap().get(&path_string)
    {
        return Ok(version.clone());
    }

    let version = match get_property(manifest_path, "default_version")? {
        Some(v) => {
            if v == "@CARGO_VERSION@" {
                let metadata = crate::metadata::metadata(
                    &Default::default(),
                    Some(manifest_path),
                    cargo_flags,
                )
                .wrap_err("couldn't get cargo metadata")?;
                crate::metadata::validate(Some(manifest_path), &metadata)?;
                let manifest_path = crate::manifest::manifest_path(&metadata, None)
                    .wrap_err("Couldn't get manifest path")?;
                let manifest =
                    Manifest::from_path(manifest_path).wrap_err("Couldn't parse manifest")?;

                manifest.package_version()?
            } else {
                v
            }
        }
        None => {
            return Err(eyre!(
                "cannot determine extension version number.  Is the `default_version` property declared in the control file?"
            ));
        }
    };

    CARGO_VERSION
        .get_or_init(Default::default)
        .lock()
        .unwrap()
        .insert(path_string, version.clone());
    Ok(version)
}

static GIT_HASH: OnceLock<MemoizeKeyValue> = OnceLock::new();

fn get_git_hash(manifest_path: &Path) -> eyre::Result<String> {
    let path_string = manifest_path.to_owned();

    let mut mutex = GIT_HASH.get_or_init(Default::default).lock().unwrap();
    if let Some(hash) = mutex.get(&path_string) {
        Ok(hash.clone())
    } else {
        let hash = match get_property(manifest_path, "git_hash")? {
            Some(hash) => hash,
            None => {
                return Err(eyre!(
                    "unable to determine git hash.  Is git installed and is this project a git repository?"
                ));
            }
        };

        mutex.insert(path_string, hash.clone());

        Ok(hash)
    }
}

fn make_relative_pkglibdir(path: PathBuf, target: Option<&str>) -> PathBuf {
    if TargetOs::from_target(target) == TargetOs::Windows {
        "lib".into()
    } else {
        make_relative_path(path)
    }
}

fn make_relative_extdir(path: PathBuf, target: Option<&str>) -> PathBuf {
    if TargetOs::from_target(target) == TargetOs::Windows {
        "share/extension".into()
    } else {
        make_relative_path(path)
    }
}

fn make_relative_path(path: PathBuf) -> PathBuf {
    use std::path::Component;
    // A Unix target's absolute path has a root but no drive prefix on a Windows host.
    if !path.has_root() {
        return path;
    }
    path.components()
        .skip_while(|x| matches!(x, Component::Prefix(_) | Component::RootDir))
        .collect()
}

pub(crate) fn format_display_path(path: &Path, target_dir: &Path) -> String {
    target_dir
        .parent()
        .and_then(|parent| path.strip_prefix(parent).ok())
        .unwrap_or(path)
        .display()
        .to_string()
}

pub(crate) fn filter_contents(
    manifest_path: &Path,
    mut input: String,
    cargo_flags: &[String],
) -> eyre::Result<String> {
    if input.contains("@GIT_HASH@") {
        // avoid doing this if we don't actually have the token
        // the project might not be a git repo so running `git`
        // would fail
        input = input.replace("@GIT_HASH@", &get_git_hash(manifest_path)?);
    }

    input = input.replace("@CARGO_VERSION@", &get_version(manifest_path, cargo_flags)?);

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::{make_relative_extdir, make_relative_pkglibdir, manifest_path_for_build};
    use std::path::Path;

    #[test]
    fn auto_detected_package_builds_against_resolved_manifest() {
        let workspace_manifest = Path::new("/workspace/Cargo.toml");
        let package_manifest = Path::new("/workspace/postgres/Cargo.toml");

        assert_eq!(manifest_path_for_build(None, None, package_manifest), Some(package_manifest));
        assert_eq!(
            manifest_path_for_build(Some(workspace_manifest), None, package_manifest),
            Some(package_manifest)
        );
    }

    #[test]
    fn explicit_package_keeps_user_manifest_targeting() {
        let workspace_manifest = Path::new("/workspace/Cargo.toml");
        let package_manifest = Path::new("/workspace/postgres/Cargo.toml");

        assert_eq!(
            manifest_path_for_build(Some(workspace_manifest), Some("tin"), package_manifest),
            Some(workspace_manifest)
        );
        assert_eq!(manifest_path_for_build(None, Some("tin"), package_manifest), None);
    }

    #[test]
    fn staged_windows_paths_follow_target() {
        for target in ["x86_64-pc-windows-msvc", "x86_64-pc-windows-gnu"] {
            for path in [r"C:\Program Files\PostgreSQL\18\lib", "/usr/lib/postgresql/18/lib"] {
                assert_eq!(make_relative_pkglibdir(path.into(), Some(target)), Path::new("lib"));
            }
            for path in [
                r"C:\Program Files\PostgreSQL\18\share\extension",
                "/usr/share/postgresql/18/extension",
            ] {
                assert_eq!(
                    make_relative_extdir(path.into(), Some(target)),
                    Path::new("share/extension")
                );
            }
        }
    }

    #[test]
    fn staged_unix_paths_follow_target() {
        for target in ["x86_64-unknown-linux-gnu", "aarch64-apple-darwin"] {
            assert_eq!(
                make_relative_pkglibdir("/usr/lib/postgresql/18/lib".into(), Some(target)),
                Path::new("usr/lib/postgresql/18/lib")
            );
            assert_eq!(
                make_relative_extdir("/opt/postgresql/share/extension".into(), Some(target)),
                Path::new("opt/postgresql/share/extension")
            );
        }
    }

    #[test]
    fn staged_relative_prefixes_follow_target() {
        for target in ["x86_64-unknown-linux-gnu", "aarch64-apple-darwin"] {
            assert_eq!(
                make_relative_pkglibdir("relative/prefix".into(), Some(target)),
                Path::new("relative/prefix")
            );
            assert_eq!(
                make_relative_extdir("relative/prefix".into(), Some(target)),
                Path::new("relative/prefix")
            );
        }
        let target = Some("x86_64-pc-windows-msvc");
        assert_eq!(make_relative_pkglibdir("relative/prefix".into(), target), Path::new("lib"));
        assert_eq!(
            make_relative_extdir("relative/prefix".into(), target),
            Path::new("share/extension")
        );
    }

    #[test]
    fn staged_paths_default_to_host() {
        let (library_dir, extension_dir) = if cfg!(target_os = "windows") {
            ("lib", "share/extension")
        } else {
            ("usr/lib/postgresql/18/lib", "usr/share/postgresql/18/extension")
        };
        assert_eq!(
            make_relative_pkglibdir("/usr/lib/postgresql/18/lib".into(), None),
            Path::new(library_dir)
        );
        assert_eq!(
            make_relative_extdir("/usr/share/postgresql/18/extension".into(), None),
            Path::new(extension_dir)
        );
    }
}
