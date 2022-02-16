// Copyright 2020 ZomboDB, LLC <zombodb@gmail.com>. All rights reserved. Use of this source code is
// governed by the MIT license that can be found in the LICENSE file.

use crate::{
    command::get::{find_control_file, get_property},
    CommandExecute,
};
use cargo_metadata::MetadataCommand;
use colored::Colorize;
use eyre::{eyre, WrapErr};
use pgx_utils::get_target_dir;
use pgx_utils::pg_config::PgConfig;
use std::path::PathBuf;
use std::process::{Command, Stdio};

/// Install the extension from the current crate to the Postgres specified by whatever `pg_config` is currently on your $PATH
#[derive(clap::Args, Debug)]
#[clap(author)]
pub(crate) struct Install {
    /// Compile for release mode (default is debug)
    #[clap(env = "PROFILE", long, short)]
    release: bool,
    /// Don't regenerate the schema
    #[clap(long)]
    no_schema: bool,
    /// The `pg_config` path (default is first in $PATH)
    #[clap(long, short = 'c')]
    pg_config: Option<String>,
    #[clap(flatten)]
    features: clap_cargo::Features,
    #[clap(from_global, parse(from_occurrences))]
    verbose: usize,
}

impl CommandExecute for Install {
    #[tracing::instrument(level = "error", skip(self))]
    fn execute(self) -> eyre::Result<()> {
        let metadata = crate::metadata::metadata(&self.features)?;
        crate::metadata::validate(&metadata)?;
        let manifest = crate::manifest::manifest(&metadata)?;

        let pg_config = match self.pg_config {
            None => PgConfig::from_path(),
            Some(config) => PgConfig::new(PathBuf::from(config)),
        };
        let pg_version = format!("pg{}", pg_config.major_version()?);
        let features = crate::manifest::features_for_version(self.features, &manifest, &pg_version);

        install_extension(&manifest, &pg_config, self.release, self.no_schema, None, &features)
    }
}

#[tracing::instrument(skip_all, fields(
    pg_version = %pg_config.version()?,
    release = is_release,
    base_directory = tracing::field::Empty,
    features = ?features.features,
))]
pub(crate) fn install_extension(
    manifest: &cargo_toml::Manifest,
    pg_config: &PgConfig,
    is_release: bool,
    no_schema: bool,
    base_directory: Option<PathBuf>,
    features: &clap_cargo::Features,
) -> eyre::Result<()> {
    let base_directory = base_directory.unwrap_or("/".into());
    tracing::Span::current().record(
        "base_directory",
        &tracing::field::display(&base_directory.display()),
    );

    let (control_file, extname) = find_control_file()?;

    if get_property("relocatable")? != Some("false".into()) {
        return Err(eyre!(
            "{}:  The `relocatable` property MUST be `false`.  Please update your .control file.",
            control_file.display()
        ));
    }

    build_extension(is_release, &features)?;

    println!();
    println!("installing extension");
    let pkgdir = make_relative(pg_config.pkglibdir()?);
    let extdir = make_relative(pg_config.extension_dir()?);
    let shlibpath = find_library_file(&extname, is_release)?;

    {
        let mut dest = base_directory.clone();
        dest.push(&extdir);
        dest.push(&control_file);
        copy_file(&control_file, &dest, "control file", true)?;
    }

    {
        let mut dest = base_directory.clone();
        dest.push(&pkgdir);
        dest.push(format!("{}.so", extname));

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
        copy_file(&shlibpath, &dest, "shared library", false)?;
    }

    if !no_schema || !get_target_sql_file(&extdir, &base_directory)?.exists() {
        copy_sql_files(manifest, pg_config, is_release, features, &extdir, &base_directory)?;
    } else {
        println!("{} schema generation", "    Skipping".bold().yellow());
    }

    println!("{} installing {}", "    Finished".bold().green(), extname);
    Ok(())
}

fn copy_file(src: &PathBuf, dest: &PathBuf, msg: &str, do_filter: bool) -> eyre::Result<()> {
    if !dest.parent().unwrap().exists() {
        std::fs::create_dir_all(dest.parent().unwrap()).wrap_err_with(|| {
            format!(
                "failed to create destination directory {}",
                dest.parent().unwrap().display()
            )
        })?;
    }

    println!(
        "{} {} to `{}`",
        "     Copying".bold().green(),
        msg,
        format_display_path(&dest)?
    );

    if do_filter {
        // we want to filter the contents of the file we're to copy
        let input = std::fs::read_to_string(&src)
            .wrap_err_with(|| format!("failed to read `{}`", src.display()))?;
        let input = filter_contents(input)?;

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
    is_release: bool,
    features: &clap_cargo::Features,
) -> eyre::Result<()> {
    let flags = std::env::var("PGX_BUILD_FLAGS").unwrap_or_default();

    let mut command = Command::new("cargo");
    command.arg("build");
    if is_release {
        command.arg("--release");
    }

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

    for arg in flags.split_ascii_whitespace() {
        command.arg(arg);
    }

    let command = command.stdout(Stdio::inherit()).stderr(Stdio::inherit());
    let command_str = format!("{:?}", command);
    println!(
        "building extension with features `{}`\n{}",
        features_arg, command_str
    );
    let status = command
        .status()
        .wrap_err_with(|| format!("failed to spawn cargo: {}", command_str))?;
    if !status.success() {
        Err(eyre!("failed to build extension"))
    } else {
        Ok(())
    }
}

fn get_target_sql_file(extdir: &PathBuf, base_directory: &PathBuf) -> eyre::Result<PathBuf> {
    let mut dest = base_directory.clone();
    dest.push(extdir);

    let (_, extname) = crate::command::get::find_control_file()?;
    let version = get_version()?;
    dest.push(format!("{}--{}.sql", extname, version));

    Ok(dest)
}

fn copy_sql_files(
    manifest: &cargo_toml::Manifest,
    pg_config: &PgConfig,
    is_release: bool,
    features: &clap_cargo::Features,
    extdir: &PathBuf,
    base_directory: &PathBuf,
) -> eyre::Result<()> {
    let dest = get_target_sql_file(extdir, base_directory)?;
    let (_, extname) = crate::command::get::find_control_file()?;

    crate::command::schema::generate_schema(
        manifest,
        pg_config,
        is_release,
        features,
        &dest,
        Option::<String>::None,
        None,
        false,
        true,
        true,
    )?;
    copy_file(&dest, &dest, "extension schema file", true)?;

    // now copy all the version upgrade files too
    if let Ok(dir) = std::fs::read_dir("sql/") {
        for sql in dir {
            if let Ok(sql) = sql {
                let filename = sql.file_name().into_string().unwrap();

                if filename.starts_with(&format!("{}--", extname)) && filename.ends_with(".sql") {
                    let mut dest = base_directory.clone();
                    dest.push(extdir);
                    dest.push(filename);

                    copy_file(&sql.path(), &dest, "extension schema upgrade file", true)?;
                }
            }
        }
    }
    Ok(())
}

pub(crate) fn find_library_file(extname: &str, is_release: bool) -> eyre::Result<PathBuf> {
    let mut target_dir = get_target_dir()?;
    target_dir.push(if is_release { "release" } else { "debug" });

    if !target_dir.exists() {
        return Err(eyre!(
            "target directory does not exist: {}",
            target_dir.display()
        ));
    }

    for f in std::fs::read_dir(&target_dir)
        .wrap_err_with(|| format!("Unable to read {}", target_dir.display()))?
    {
        if let Ok(f) = f {
            let filename = f.file_name().into_string().unwrap();

            if filename.contains(extname)
                && filename.starts_with("lib")
                && (filename.ends_with(".so")
                    || filename.ends_with(".dylib")
                    || filename.ends_with(".dll"))
            {
                return Ok(f.path());
            }
        }
    }

    if extname.contains('-') {
        Err(eyre!("
            library file not found in: `{}`.  It looks like your extension/crate name contains a dash (`-`).  The allowed set of characters is `{}`. Try renaming things, including your `{}.control` file",
            target_dir.display(), "[a-z0-9_]".green(),
            extname
        ))
    } else {
        Err(eyre!(
            "library file not found in: `{}`",
            target_dir.display()
        ))
    }
}

pub(crate) fn get_version() -> eyre::Result<String> {
    match get_property("default_version")? {
        Some(v) => {
            if v == "@CARGO_VERSION@" {
                let metadata = MetadataCommand::new().exec()?;
                let root_package = metadata.root_package().ok_or(eyre!("no root package found"))?;
                Ok(root_package.version.to_string())
            } else {
                Ok(v)
            }
        },
        None => Err(eyre!("cannot determine extension version number.  Is the `default_version` property declared in the control file?")),
    }
}

fn get_git_hash() -> eyre::Result<String> {
    match get_property("git_hash")? {
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

fn format_display_path(path: &PathBuf) -> eyre::Result<String> {
    let out = path
        .strip_prefix(get_target_dir()?.parent().unwrap())
        .unwrap_or(&path)
        .display()
        .to_string();
    Ok(out)
}

fn filter_contents(mut input: String) -> eyre::Result<String> {
    if input.contains("@GIT_HASH@") {
        // avoid doing this if we don't actually have the token
        // the project might not be a git repo so running `git`
        // would fail
        input = input.replace("@GIT_HASH@", &get_git_hash()?);
    }

    input = input.replace("@CARGO_VERSION@", &get_version()?);

    Ok(input)
}
