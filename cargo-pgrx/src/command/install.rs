//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use crate::command::get::{find_control_file, get_property};
use crate::command::sudo_install::SudoInstall;
use crate::manifest::{display_version_info, PgVersionSource};
use crate::profile::CargoProfile;
use crate::CommandExecute;
use cargo_metadata::{Message as CargoMessage, CompilerMessage, Artifact};
use cargo_toml::Manifest;
use eyre::{eyre, WrapErr};
use owo_colors::OwoColorize;
use pgrx_pg_config::{cargo::PgrxManifestExt, get_target_dir, PgConfig, Pgrx};
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
    #[clap(from_global, action = ArgAction::Count)]
    pub(crate) verbose: u8,
}

impl CommandExecute for Install {
    #[tracing::instrument(level = "error", skip(self))]
    fn execute(mut self) -> eyre::Result<()> {
        if self.sudo {
            // user wishes to use `sudo` to install the extension
            // so we re-route through the `SudoInstall` type
            let sudo_install = SudoInstall::from(self);
            return sudo_install.execute();
        }

        let metadata = crate::metadata::metadata(&self.features, self.manifest_path.as_ref())
            .wrap_err("couldn't get cargo metadata")?;
        crate::metadata::validate(self.manifest_path.as_ref(), &metadata)?;
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
            if self.release { CargoProfile::Release } else { CargoProfile::Dev },
        )?;

        crate::manifest::modify_features_for_version(
            &Pgrx::from_config()?,
            Some(&mut self.features),
            &package_manifest,
            &PgVersionSource::PgConfig(pg_version),
            self.test,
        );

        display_version_info(&pg_config, &PgVersionSource::PgConfig(pg_config.label()?));
        install_extension(
            self.manifest_path.as_ref(),
            self.package.as_ref(),
            package_manifest_path,
            &pg_config,
            &profile,
            self.test,
            None,
            &self.features,
        )?;
        Ok(())
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
) -> eyre::Result<Vec<PathBuf>> {
    let mut output_tracking = Vec::new();
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
        build_extension(user_manifest_path.as_ref(), user_package, profile, features)?;
    let build_command_bytes = build_command_output.stdout;
    // eprintln!("{}", String::from_utf8_lossy(&build_command_bytes));
    let build_command_reader = BufReader::new(build_command_bytes.as_slice());
    let build_command_stream = CargoMessage::parse_stream(build_command_reader);
    let build_command_messages =
        build_command_stream.collect::<Result<Vec<_>, std::io::Error>>()?;

    println!("{} extension", "  Installing".bold().green());
    let pkgdir = make_relative(pg_config.pkglibdir()?);
    let extdir = make_relative(pg_config.extension_dir()?);
    let shlibpath = find_library_file(&manifest, &build_command_messages)?;

    {
        let mut dest = base_directory.clone();
        dest.push(&extdir);
        dest.push(
            control_file
                .file_name()
                .ok_or_else(|| eyre!("Could not get filename for `{}`", control_file.display()))?,
        );
        copy_file(
            &control_file,
            dest,
            "control file",
            true,
            &package_manifest_path,
            &mut output_tracking,
            pg_config,
        )?;
    }

    {
        let mut dest = base_directory.clone();
        dest.push(&pkgdir);

        let so_name = if versioned_so {
            let extver = get_version(&package_manifest_path)?;
            // note: versioned so-name format must agree with pgrx-utils
            format!("{extname}-{extver}")
        } else {
            extname.clone()
        };
        // Since Postgres 16, the shared library extension on macOS is `dylib`, not `so`.
        // Ref https://github.com/postgres/postgres/commit/b55f62abb2c2e07dfae99e19a2b3d7ca9e58dc1a
        let so_extension = if cfg!(target_os = "macos") && pg_config.major_version().unwrap() >= 16
        {
            "dylib"
        } else {
            "so"
        };
        dest.push(format!("{so_name}.{so_extension}"));

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
            &package_manifest_path,
            &mut output_tracking,
            pg_config,
        )?;
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
        &mut output_tracking,
    )?;

    println!("{} installing {}", "    Finished".bold().green(), extname);
    Ok(output_tracking)
}

fn copy_file(
    src: &Path,
    dest: PathBuf,
    msg: &str,
    do_filter: bool,
    package_manifest_path: impl AsRef<Path>,
    output_tracking: &mut Vec<PathBuf>,
    pg_config: &PgConfig,
) -> eyre::Result<()> {
    let Some(dest_dir) = dest.parent() else {
        // what fresh hell could ever cause such an error?
        eyre::bail!("no directory to copy to: {}", dest.display())
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

    println!("{} {} to {}", "     Copying".bold().green(), msg, format_display_path(&dest)?.cyan());

    if do_filter {
        // we want to filter the contents of the file we're to copy
        let input = fs::read_to_string(src)
            .wrap_err_with(|| format!("failed to read `{}`", src.display()))?;
        let mut input = filter_contents(package_manifest_path, input)?;

        if src.display().to_string().ends_with(".control") {
            input = filter_out_fields_in_control(pg_config, input)?;
        }

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

pub(crate) fn build_extension(
    user_manifest_path: Option<impl AsRef<Path>>,
    user_package: Option<&String>,
    profile: &CargoProfile,
    features: &clap_cargo::Features,
) -> eyre::Result<std::process::Output> {
    let flags = std::env::var("PGRX_BUILD_FLAGS").unwrap_or_default();

    let mut command = crate::env::cargo();
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
    let command_str = format!("{command:?}");
    println!("{} extension with features {}", "    Building".bold().green(), features_arg.cyan());
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

fn get_target_sql_file(
    manifest_path: impl AsRef<Path>,
    extdir: &Path,
    base_directory: PathBuf,
) -> eyre::Result<PathBuf> {
    let mut dest = base_directory;
    dest.push(extdir);

    let (_, extname) = find_control_file(&manifest_path)?;
    let version = get_version(&manifest_path)?;
    dest.push(format!("{extname}--{version}.sql"));

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
    extdir: &Path,
    base_directory: &Path,
    skip_build: bool,
    output_tracking: &mut Vec<PathBuf>,
) -> eyre::Result<()> {
    let dest = get_target_sql_file(&package_manifest_path, extdir, base_directory.to_path_buf())?;
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
        output_tracking,
    )?;

    // now copy all the version upgrade files too
    if let Ok(dir) = fs::read_dir("sql/") {
        for sql in dir.flatten() {
            let filename = sql.file_name().into_string().unwrap();

            if filename.starts_with(&format!("{extname}--")) && filename.ends_with(".sql") {
                let mut dest = base_directory.to_path_buf();
                dest.push(extdir);
                dest.push(filename);

                copy_file(
                    &sql.path(),
                    dest,
                    "extension schema upgrade file",
                    true,
                    &package_manifest_path,
                    output_tracking,
                    pg_config,
                )?;
            }
        }
    }
    Ok(())
}

#[tracing::instrument(level = "error", skip_all)]
pub(crate) fn find_library_file(
    manifest: &Manifest,
    build_command_messages: &Vec<CargoMessage>,
) -> eyre::Result<PathBuf> {
    let target_name = manifest.target_name()?;
    let target_name_replaced = target_name.replace("-", "_");
    let so_ext = if cfg!(target_os = "macos") { "dylib" } else { "so" };
    
    let mut library_file = None;
    for message in build_command_messages {
        match message {
            CargoMessage::CompilerArtifact(artifact) => {
                let art_name = &artifact.target.name;
                
                if art_name.contains(&target_name) {
                    eprintln!("artifact {art_name} contains {target_name}");
                    eprintln!("artifact: {artifact:#?}");
                }
                if art_name.contains(&target_name_replaced) {
                    eprintln!("artifact {art_name} contains replaced {target_name}");
                    eprintln!("artifact: {artifact:#?}");
                }
                if target_name.contains(art_name) {
                    eprintln!("target {target_name} contains {art_name}");
                    eprintln!("artifact: {artifact:#?}");
                }
                if target_name_replaced.contains(art_name) {
                    eprintln!("replaced target {target_name_replaced} contains {art_name}");
                    eprintln!("artifact: {artifact:#?}");
                }
                if art_name != &target_name && art_name != &target_name_replaced {
                    continue;
                }
                for filename in &artifact.filenames {
                    if filename.extension() == Some(so_ext) {
                        library_file = Some(filename.to_string());
                        break;
                    }
                }
            }
            CargoMessage::CompilerMessage(notice) => {
                eprintln!("rustc: {notice:?}");
            },
            CargoMessage::TextLine(line) => {
                eprintln!("{line}");
            },
            CargoMessage::BuildScriptExecuted(_)
            | CargoMessage::BuildFinished(_)
            | _ => (),
        }
    }
    let library_file =
        library_file.ok_or_else(|| eyre!("Could not get shared object file `{target_name}.{so_ext}` from Cargo output."))?;
    let library_file_path = PathBuf::from(library_file);

    Ok(library_file_path)
}

static CARGO_VERSION: OnceLock<MemoizeKeyValue> = OnceLock::new();

pub(crate) fn get_version(manifest_path: impl AsRef<Path>) -> eyre::Result<String> {
    let path_string = manifest_path.as_ref().to_owned();

    if let Some(version) =
        CARGO_VERSION.get_or_init(Default::default).lock().unwrap().get(&path_string)
    {
        return Ok(version.clone());
    }

    let version = match get_property(&manifest_path, "default_version")? {
        Some(v) => {
            if v == "@CARGO_VERSION@" {
                let metadata = crate::metadata::metadata(&Default::default(), Some(&manifest_path))
                    .wrap_err("couldn't get cargo metadata")?;
                crate::metadata::validate(Some(manifest_path), &metadata)?;
                let manifest_path = crate::manifest::manifest_path(&metadata, None)
                    .wrap_err("Couldn't get manifest path")?;
                let manifest = Manifest::from_path(manifest_path)
                    .wrap_err("Couldn't parse manifest")?;

                manifest.package_version()?
            } else {
                v
            }
        },
        None => return Err(eyre!("cannot determine extension version number.  Is the `default_version` property declared in the control file?")),
    };

    CARGO_VERSION
        .get_or_init(Default::default)
        .lock()
        .unwrap()
        .insert(path_string, version.clone());
    Ok(version)
}

static GIT_HASH: OnceLock<MemoizeKeyValue> = OnceLock::new();

fn get_git_hash(manifest_path: impl AsRef<Path>) -> eyre::Result<String> {
    let path_string = manifest_path.as_ref().to_owned();

    if let Some(hash) = GIT_HASH.get_or_init(Default::default).lock().unwrap().get(&path_string) {
        Ok(hash.clone())
    } else {
        let hash = match get_property(manifest_path, "git_hash")? {
            Some(hash) => hash,
            None => return Err(eyre!(
                "unable to determine git hash.  Is git installed and is this project a git repository?"
            )),
        };

        GIT_HASH.get_or_init(Default::default).lock().unwrap().insert(path_string, hash.clone());

        Ok(hash)
    }
}

fn make_relative(path: PathBuf) -> PathBuf {
    if path.is_relative() {
        return path;
    }
    let mut relative = PathBuf::new();
    let mut components = path.components();
    components.next(); // skip the root
    for part in components {
        relative.push(part)
    }
    relative
}

pub(crate) fn format_display_path(path: impl AsRef<Path>) -> eyre::Result<String> {
    let path = path.as_ref();
    let out = path
        .strip_prefix(get_target_dir()?.parent().unwrap())
        .unwrap_or(path)
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

// remove fields in control for versions not supported
// `trusted`` in only supported in version 13 and above
fn filter_out_fields_in_control(pg_config: &PgConfig, mut input: String) -> eyre::Result<String> {
    if pg_config.major_version().unwrap() < 13 {
        input = input
            .lines()
            .filter(|line| !line.starts_with("trusted"))
            .collect::<Vec<_>>()
            .join("\n");
    }

    Ok(input)
}
