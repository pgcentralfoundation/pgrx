//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use crate::cargo::CargoProfile;
use crate::command::get::get_property;
use crate::command::install::{install_extension, warn_if_pg_bench_enabled};
use crate::manifest::{display_version_info, PgVersionSource};
use crate::CommandExecute;
use cargo_toml::Manifest;
use eyre::{eyre, WrapErr};
use pgrx_pg_config::{get_target_dir, PgConfig, Pgrx};
use std::path::{Path, PathBuf};
use owo_colors::OwoColorize;

/// Create an installation package directory.
#[derive(clap::Args, Debug)]
#[clap(author)]
pub(crate) struct Package {
    /// Package to build (see `cargo help pkgid`)
    #[clap(long, short)]
    pub(crate) package: Option<String>,
    /// Path to Cargo.toml
    #[clap(long, value_parser)]
    pub(crate) manifest_path: Option<PathBuf>,
    /// Compile for debug mode (default is release)
    #[clap(long, short)]
    pub(crate) debug: bool,
    /// Specific profile to use (conflicts with `--debug`)
    #[clap(long)]
    pub(crate) profile: Option<String>,
    /// Build in test mode (for `cargo pgrx test`)
    #[clap(long)]
    pub(crate) test: bool,
    /// The `pg_config` path (default is first in $PATH)
    #[clap(long, short = 'c', value_parser)]
    pub(crate) pg_config: Option<PathBuf>,
    /// The directory to output the package (default is `./target/[debug|release]/extname-pgXX/`)
    #[clap(long, value_parser)]
    pub(crate) out_dir: Option<PathBuf>,
    #[clap(flatten)]
    pub(crate) features: clap_cargo::Features,
    #[clap(long)]
    pub(crate) target: Option<String>,
    #[clap(from_global, action = ArgAction::Count)]
    pub(crate) verbose: u8,
}

impl Package {
    pub(crate) fn perform(mut self) -> eyre::Result<(PathBuf, Vec<PathBuf>)> {
        warn_if_pg_bench_enabled(&self.features, "package");
        let metadata = crate::metadata::metadata(&self.features, self.manifest_path.as_deref())
            .wrap_err("couldn't get cargo metadata")?;
        crate::metadata::validate(self.manifest_path.as_deref(), &metadata)?;
        let package_manifest_path =
            crate::manifest::manifest_path(&metadata, self.package.as_deref())
                .wrap_err("Couldn't get manifest path")?;
        let package_manifest =
            Manifest::from_path(&package_manifest_path).wrap_err("Couldn't parse manifest")?;

        let pg_config = match self.pg_config {
            None => PgConfig::from_path(),
            Some(config) => PgConfig::new_with_defaults(config),
        };
        let pg_version = format!("pg{}", pg_config.major_version()?);

        crate::manifest::modify_features_for_version(
            &Pgrx::from_config()?,
            Some(&mut self.features),
            &package_manifest,
            &PgVersionSource::PgConfig(pg_version),
            false,
        );
        let profile = CargoProfile::from_flags(
            self.profile.as_deref(),
            // NB:  `cargo pgrx package` defaults to "--release" whereas all other commands default to "debug"
            if self.debug { CargoProfile::Dev } else { CargoProfile::Release },
        )?;
        let out_dir = if let Some(out_dir) = self.out_dir {
            out_dir
        } else {
            build_base_path(&pg_config, &package_manifest_path, &profile, self.target.as_deref())?
        };

        let output_files = package_extension(
            self.manifest_path.as_deref(),
            self.package.as_deref(),
            &package_manifest_path,
            &pg_config,
            out_dir.clone(),
            &profile,
            self.test,
            &self.features,
            self.target.as_deref(),
        )?;

        if rpm(&pg_config, &package_manifest_path, out_dir.clone())? {
            eprintln!("{} RPM package in {out_dir:?}", "     Writing".bold().green());
        }
        Ok((out_dir, output_files))
    }
}

impl CommandExecute for Package {
    #[tracing::instrument(level = "error", skip(self))]
    fn execute(self) -> eyre::Result<()> {
        self.perform()?;
        Ok(())
    }
}

#[tracing::instrument(level = "error", skip_all, fields(
    pg_version = %pg_config.version()?,
    profile = ?profile,
    test = is_test,
))]
pub(crate) fn package_extension(
    user_manifest_path: Option<&Path>,
    user_package: Option<&str>,
    package_manifest_path: &Path,
    pg_config: &PgConfig,
    out_dir: PathBuf,
    profile: &CargoProfile,
    is_test: bool,
    features: &clap_cargo::Features,
    target: Option<&str>,
) -> eyre::Result<Vec<PathBuf>> {
    let out_dir_exists = out_dir.try_exists().wrap_err_with(|| {
        format!("failed to access {} while packaging extension", out_dir.display())
    })?;
    if !out_dir_exists {
        std::fs::create_dir_all(&out_dir)?;
    }

    display_version_info(pg_config, &PgVersionSource::PgConfig(pg_config.label()?));
    install_extension(
        user_manifest_path,
        user_package,
        package_manifest_path,
        pg_config,
        profile,
        is_test,
        Some(out_dir),
        features,
        target,
    )
}

pub(crate) fn build_base_path(
    pg_config: &PgConfig,
    manifest_path: &Path,
    profile: &CargoProfile,
    target: Option<&str>,
) -> eyre::Result<PathBuf> {
    let mut target_dir = get_target_dir()?;
    let pgver = pg_config.major_version()?;
    let extname = get_property(manifest_path, "extname")?
        .ok_or(eyre!("could not determine extension name"))?;
    if let Some(target) = target {
        target_dir.push(target);
    }
    target_dir.push(profile.target_subdir());
    target_dir.push(format!("{extname}-pg{pgver}"));
    Ok(target_dir)
}

fn rpm(
    pg_config: &PgConfig,
    manifest_path: &Path,
    outdir: PathBuf,
) -> eyre::Result<bool> {
    use rpm;
    use cargo_toml::Manifest;


    let major_version = pg_config.major_version()?;
    let extname = get_property(manifest_path, "extname")?
        .ok_or(eyre!("could not determine extension name"))?;
    let package_name=format!("{extname}-pg{major_version}");


    let cargo_toml = Manifest::<toml::Value>::from_path_with_metadata(&manifest_path).unwrap();
    let package = cargo_toml.package();
    let description = package.description().unwrap_or("");
    let license = package.license().unwrap_or("None");
    let version = package.version(); 
    let homepage = package.homepage().unwrap_or("");

    let Some(metadata) = package.metadata.clone() else {
        return Ok(false);
    };

    let Some(rpm_metadata) = metadata.get("rpm") else {
        return Ok(false);
    };

    let vendor = rpm_metadata.get("vendor")
        .and_then(|v| v.as_str())
        .unwrap_or("Undefined");

    // this is based on the PGDG packages. 
    let dest_pgsql_dir = format!("pgsql-{major_version}");
    let libfile=format!("{extname}.so");
    let src_libfile_path=outdir.clone()
        .join(pg_config.pkglibdir()?.strip_prefix("/")?)
        .join(&libfile);
    let dest_libfile_path=PathBuf::new().join("/usr").join(dest_pgsql_dir).join("lib").join(&libfile);
    let libfile_options = rpm::FileOptions::new(dest_libfile_path.display().to_string()); 

    let src_extdir_path=outdir.clone()
        .join(pg_config.sharedir()?.strip_prefix("/")?)
        .join("extension");
    let dest_extdir_path=format!("/usr/pgsql-{major_version}/share/extension");
    
    let build_config = rpm::BuildConfig::default()
        .compression(rpm::CompressionType::Gzip)
        // TODO: fetch the timestamp of the last commit
        .source_date(1_600_000_000);
    let pkg = rpm::PackageBuilder::new(&package_name, version, license, "x86_64", description)
        .using_config(build_config)
        .with_file(src_libfile_path,libfile_options)?
        .with_dir(src_extdir_path,dest_extdir_path,|o| o.config())?
        .requires(rpm::Dependency::any(format!("postgresql${major_version}-server")))
        .vendor(vendor)
        .url(homepage)
        // TODO: fetch the last commit
        //.vcs("git:repo=example_repo:branch=example_branch:sha=example_sha")
        .build()?;

    let _ = pkg.write_to(outdir);
    Ok(true)
}
