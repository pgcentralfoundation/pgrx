/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use crate::command::get::{find_control_file, get_property};
use crate::manifest::{display_version_info, PgVersionSource};
use crate::profile::CargoProfile;
use crate::CommandExecute;
use cargo_toml::Manifest;
use eyre::{eyre, WrapErr};
use owo_colors::OwoColorize;
use pgx_pg_config::{get_target_dir, PgConfig, Pgx};
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Install the extension from the current crate to the Postgres specified by whatever `pg_config` is currently on your $PATH
#[derive(clap::Args, Debug)]
#[clap(author)]
pub(crate) struct Install {
    /// Package to build (see `cargo help pkgid`)
    #[clap(long, short)]
    package: Option<String>,
    /// Path to Cargo.toml
    #[clap(long, value_parser)]
    manifest_path: Option<PathBuf>,
    /// Compile for release mode (default is debug)
    #[clap(long, short)]
    release: bool,
    /// Specific profile to use (conflicts with `--release`)
    #[clap(long)]
    profile: Option<String>,
    /// Build in test mode (for `cargo pgx test`)
    #[clap(long)]
    test: bool,
    /// The `pg_config` path (default is first in $PATH)
    #[clap(long, short = 'c')]
    pg_config: Option<String>,
    #[clap(flatten)]
    features: clap_cargo::Features,
    #[clap(from_global, action = ArgAction::Count)]
    verbose: u8,
}

impl CommandExecute for Install {
    #[tracing::instrument(level = "error", skip(self))]
    fn execute(mut self) -> eyre::Result<()> {
        let metadata = crate::metadata::metadata(&self.features, self.manifest_path.as_ref())
            .wrap_err("couldn't get cargo metadata")?;
        crate::metadata::validate(&metadata)?;
        let package_manifest_path =
            crate::manifest::manifest_path(&metadata, self.package.as_ref())
                .wrap_err("Couldn't get manifest path")?;
        let package_manifest =
            Manifest::from_path(&package_manifest_path).wrap_err("Couldn't parse manifest")?;

        let pg_config = match self.pg_config {
            None => PgConfig::from_path(),
            Some(config) => PgConfig::new_with_defaults(PathBuf::from(config)),
        };
        let pg_version = format!("pg{}", pg_config.major_version()?);
        let profile = CargoProfile::from_flags(
            self.profile.as_deref(),
            self.release.then_some(CargoProfile::Release).unwrap_or(CargoProfile::Dev),
        )?;

        crate::manifest::modify_features_for_version(
            &Pgx::from_config()?,
            Some(&mut self.features),
            &package_manifest,
            &PgVersionSource::PgConfig(pg_version),
            self.test,
        );

        display_version_info(&pg_config, &PgVersionSource::PgConfig(pg_config.label()?.into()));
        install_extension(
            self.manifest_path.as_ref(),
            self.package.as_ref(),
            package_manifest_path,
            &pg_config,
            &profile,
            self.test,
            None,
            &self.features,
        )
    }
}

#[tracing::instrument(skip_all, fields(
    pg_version = %pg_config.version()?,
    profile = ?profile,
    test = is_test,
    base_directory = tracing::field::Empty,
    features = ?features.features,
))]
pub(crate) fn install_extension(
    user_manifest_path: Option<impl AsRef<Path>>,
    user_package: Option<&String>,
    package_manifest_path: impl AsRef<Path>,
    pg_config: &PgConfig,
    profile: &CargoProfile,
    is_test: bool,
    base_directory: Option<PathBuf>,
    features: &clap_cargo::Features,
) -> eyre::Result<()> {
    let base_directory = base_directory.unwrap_or_else(|| PathBuf::from("/"));
    tracing::Span::current()
        .record("base_directory", &tracing::field::display(&base_directory.display()));

    let manifest = Manifest::from_path(&package_manifest_path)?;
    let (control_file, extname) = find_control_file(&package_manifest_path)?;

    if get_property(&package_manifest_path, "relocatable")? != Some("false".into()) {
        return Err(eyre!(
            "{}:  The `relocatable` property MUST be `false`.  Please update your .control file.",
            control_file.display()
        ));
    }

    let versioned_so = get_property(&package_manifest_path, "module_pathname")?.is_none();

    let build_command_output =
        build_extension(user_manifest_path.as_ref(), user_package, &profile, &features)?;
    let build_command_bytes = build_command_output.stdout;
    let build_command_reader = BufReader::new(build_command_bytes.as_slice());
    let build_command_stream = cargo_metadata::Message::parse_stream(build_command_reader);
    let build_command_messages =
        build_command_stream.collect::<Result<Vec<_>, std::io::Error>>()?;

    println!("{} extension", "  Installing".bold().green(),);
    let pkgdir = make_relative(pg_config.pkglibdir()?);
    let extdir = make_relative(pg_config.extension_dir()?);
    let shlibpath = find_library_file(&manifest, &build_command_messages)?;

    {
        let mut dest = base_directory.clone();
        dest.push(&extdir);
        dest.push(
            &control_file
                .file_name()
                .ok_or_else(|| eyre!("Could not get filename for `{}`", control_file.display()))?,
        );
        copy_file(&control_file, &dest, "control file", true, &package_manifest_path)?;
    }

    {
        let mut dest = base_directory.clone();
        dest.push(&pkgdir);
        let so_name = if versioned_so {
            let extver = get_version(&package_manifest_path)?;
            // note: versioned so-name format must agree with pgx-utils
            format!("{}-{}", &extname, &extver)
        } else {
            extname.clone()
        };
        dest.push(format!("{}.so", so_name));

        if cfg!(target_os = "macos") {
            // Remove the existing .so if present. This is a workaround for an
            // issue highlighted by the following apple documentation:
            // https://developer.apple.com/documentation/security/updating_mac_software
            if dest.exists() {
                std::fs::remove_file(&dest).wrap_err_with(|| {
                    format!("unable to remove existing file {}", dest.display())
                })?;
            }
        }
        copy_file(&shlibpath, &dest, "shared library", false, &package_manifest_path)?;
    }

    copy_sql_files(
        user_manifest_path,
        user_package,
        &package_manifest_path,
        pg_config,
        profile,
        is_test,
        features,
        &extdir,
        &base_directory,
        true,
    )?;

    println!("{} installing {}", "    Finished".bold().green(), extname);
    Ok(())
}

fn copy_file(
    src: &PathBuf,
    dest: &PathBuf,
    msg: &str,
    do_filter: bool,
    package_manifest_path: impl AsRef<Path>,
) -> eyre::Result<()> {
    if !dest.parent().unwrap().exists() {
        std::fs::create_dir_all(dest.parent().unwrap()).wrap_err_with(|| {
            format!("failed to create destination directory {}", dest.parent().unwrap().display())
        })?;
    }

    println!("{} {} to {}", "     Copying".bold().green(), msg, format_display_path(&dest)?.cyan());

    if do_filter {
        // we want to filter the contents of the file we're to copy
        let input = std::fs::read_to_string(&src)
            .wrap_err_with(|| format!("failed to read `{}`", src.display()))?;
        let input = filter_contents(package_manifest_path, input)?;

        std::fs::write(&dest, &input).wrap_err_with(|| {
            format!("failed writing `{}` to `{}`", src.display(), dest.display())
        })?;
    } else {
        std::fs::copy(&src, &dest).wrap_err_with(|| {
            format!("failed copying `{}` to `{}`", src.display(), dest.display())
        })?;
    }

    Ok(())
}

pub(crate) fn build_extension(
    user_manifest_path: Option<impl AsRef<Path>>,
    user_package: Option<&String>,
    profile: &CargoProfile,
    features: &clap_cargo::Features,
) -> eyre::Result<std::process::Output> {
    let flags = std::env::var("PGX_BUILD_FLAGS").unwrap_or_default();

    let mut command = Command::new("cargo");
    command.arg("build");

    if let Some(user_manifest_path) = user_manifest_path {
        command.arg("--manifest-path");
        command.arg(user_manifest_path.as_ref());
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

    let command = command.stderr(Stdio::inherit());
    let command_str = format!("{:?}", command);
    println!("{} extension with features {}", "    Building".bold().green(), features_arg.cyan());
    println!("{} command {}", "     Running".bold().green(), command_str.cyan());
    let cargo_output =
        command.output().wrap_err_with(|| format!("failed to spawn cargo: {}", command_str))?;
    if !cargo_output.status.success() {
        // We explicitly do not want to return a spantraced error here.
        std::process::exit(1)
    } else {
        Ok(cargo_output)
    }
}

fn get_target_sql_file(
    manifest_path: impl AsRef<Path>,
    extdir: &PathBuf,
    base_directory: &PathBuf,
) -> eyre::Result<PathBuf> {
    let mut dest = base_directory.clone();
    dest.push(extdir);

    let (_, extname) = find_control_file(&manifest_path)?;
    let version = get_version(&manifest_path)?;
    dest.push(format!("{}--{}.sql", extname, version));

    Ok(dest)
}

fn copy_sql_files(
    user_manifest_path: Option<impl AsRef<Path>>,
    user_package: Option<&String>,
    package_manifest_path: impl AsRef<Path>,
    pg_config: &PgConfig,
    profile: &CargoProfile,
    is_test: bool,
    features: &clap_cargo::Features,
    extdir: &PathBuf,
    base_directory: &PathBuf,
    skip_build: bool,
) -> eyre::Result<()> {
    let dest = get_target_sql_file(&package_manifest_path, extdir, base_directory)?;
    let (_, extname) = find_control_file(&package_manifest_path)?;

    crate::command::schema::generate_schema(
        pg_config,
        user_manifest_path,
        user_package,
        &package_manifest_path,
        profile,
        is_test,
        features,
        Some(&dest),
        Option::<String>::None,
        None,
        skip_build,
    )?;

    // now copy all the version upgrade files too
    if let Ok(dir) = std::fs::read_dir("sql/") {
        for sql in dir {
            if let Ok(sql) = sql {
                let filename = sql.file_name().into_string().unwrap();

                if filename.starts_with(&format!("{}--", extname)) && filename.ends_with(".sql") {
                    let mut dest = base_directory.clone();
                    dest.push(extdir);
                    dest.push(filename);

                    copy_file(
                        &sql.path(),
                        &dest,
                        "extension schema upgrade file",
                        true,
                        &package_manifest_path,
                    )?;
                }
            }
        }
    }
    Ok(())
}

#[tracing::instrument(level = "error", skip_all)]
pub(crate) fn find_library_file(
    manifest: &cargo_toml::Manifest,
    build_command_messages: &Vec<cargo_metadata::Message>,
) -> eyre::Result<PathBuf> {
    let crate_name = if let Some(ref package) = manifest.package {
        &package.name
    } else {
        return Err(eyre!("Could not get crate name from manifest."));
    };

    let mut library_file = None;
    for message in build_command_messages {
        match message {
            cargo_metadata::Message::CompilerArtifact(artifact) => {
                if artifact.target.name != *crate_name {
                    continue;
                }
                for filename in &artifact.filenames {
                    let so_extension = if cfg!(target_os = "macos") { "dylib" } else { "so" };
                    if filename.extension() == Some(so_extension) {
                        library_file = Some(filename.to_string());
                        break;
                    }
                }
            }
            cargo_metadata::Message::CompilerMessage(_)
            | cargo_metadata::Message::BuildScriptExecuted(_)
            | cargo_metadata::Message::BuildFinished(_)
            | _ => (),
        }
    }
    let library_file =
        library_file.ok_or(eyre!("Could not get shared object file from Cargo output."))?;
    let library_file_path = PathBuf::from(library_file);

    Ok(library_file_path)
}

pub(crate) fn get_version(manifest_path: impl AsRef<Path>) -> eyre::Result<String> {
    match get_property(&manifest_path, "default_version")? {
        Some(v) => {
            if v == "@CARGO_VERSION@" {
                let metadata = crate::metadata::metadata(&Default::default(), Some(&manifest_path))
                    .wrap_err("couldn't get cargo metadata")?;
                crate::metadata::validate(&metadata)?;
                let manifest_path = crate::manifest::manifest_path(&metadata, None)
                    .wrap_err("Couldn't get manifest path")?;
                let manifest = Manifest::from_path(&manifest_path)
                    .wrap_err("Couldn't parse manifest")?;

                let version = manifest.package.ok_or(eyre!("no `[package]` section found"))?.version;
                Ok(version)
            } else {
                Ok(v)
            }
        },
        None => Err(eyre!("cannot determine extension version number.  Is the `default_version` property declared in the control file?")),
    }
}

fn get_git_hash(manifest_path: impl AsRef<Path>) -> eyre::Result<String> {
    match get_property(manifest_path, "git_hash")? {
        Some(hash) => Ok(hash),
        None => Err(eyre!(
            "unable to determine git hash.  Is git installed and is this project a git repository?"
        )),
    }
}

fn make_relative(path: PathBuf) -> PathBuf {
    if path.is_relative() {
        return path;
    }
    let mut relative = PathBuf::new();
    let mut components = path.components();
    components.next(); // skip the root
    while let Some(part) = components.next() {
        relative.push(part)
    }
    relative
}

pub(crate) fn format_display_path(path: impl AsRef<Path>) -> eyre::Result<String> {
    let path = path.as_ref();
    let out = path
        .strip_prefix(get_target_dir()?.parent().unwrap())
        .unwrap_or(&path)
        .display()
        .to_string();
    Ok(out)
}

fn filter_contents(manifest_path: impl AsRef<Path>, mut input: String) -> eyre::Result<String> {
    if input.contains("@GIT_HASH@") {
        // avoid doing this if we don't actually have the token
        // the project might not be a git repo so running `git`
        // would fail
        input = input.replace("@GIT_HASH@", &get_git_hash(&manifest_path)?);
    }

    input = input.replace("@CARGO_VERSION@", &get_version(&manifest_path)?);

    Ok(input)
}
