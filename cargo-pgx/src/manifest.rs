/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
use cargo_metadata::Metadata;
use cargo_toml::Manifest;
use clap_cargo::Features;
use eyre::{eyre, Context};
use pgx_pg_config::{PgConfig, Pgx};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub(crate) enum PgVersionSource {
    CliArgument(String),
    FeatureFlag(String),
    DefaultFeature(String),
    PgConfig(String),
}

impl From<PgVersionSource> for String {
    fn from(v: PgVersionSource) -> Self {
        match v {
            PgVersionSource::CliArgument(s) => s,
            PgVersionSource::FeatureFlag(s) => s,
            PgVersionSource::DefaultFeature(s) => s,
            PgVersionSource::PgConfig(s) => s,
        }
    }
}

impl PgVersionSource {
    fn label(&self) -> &String {
        match self {
            PgVersionSource::CliArgument(s) => s,
            PgVersionSource::FeatureFlag(s) => s,
            PgVersionSource::DefaultFeature(s) => s,
            PgVersionSource::PgConfig(s) => s,
        }
    }
}

#[tracing::instrument(skip_all)]
pub(crate) fn manifest_path(
    metadata: &Metadata,
    package_name: Option<&String>,
) -> eyre::Result<PathBuf> {
    let manifest_path = if let Some(package_name) = package_name {
        let found = metadata
            .packages
            .iter()
            .find(|v| v.name == *package_name)
            .ok_or_else(|| eyre!("Could not find package `{}`", package_name))?;
        tracing::debug!(manifest_path = %found.manifest_path, "Found workspace package");
        found.manifest_path.clone().into_std_path_buf()
    } else {
        let root = metadata.root_package().ok_or(eyre!(
            "`pgx` requires a root package in a workspace when `--package` is not specified."
        ))?;
        tracing::debug!(manifest_path = %root.manifest_path, "Found root package");
        root.manifest_path.clone().into_std_path_buf()
    };
    Ok(manifest_path)
}

pub(crate) fn modify_features_for_version(
    pgx: &Pgx,
    features: Option<&mut Features>,
    manifest: &Manifest,
    pg_version: &PgVersionSource,
    test: bool,
) {
    if let Some(features) = features {
        if let Some(default_features) = manifest.features.get("default") {
            if !features.no_default_features {
                // if the user didn't specify `--no-default-features`, which would otherwise indicate
                // they think they know what they're doing, we need to build an explicit set of features
                // to use and turn on `--no-default-features`

                features.no_default_features = true;
                features.features.extend(
                    default_features
                        .iter()
                        // only include default features that aren't known pgXX version features
                        .filter(|flag| !pgx.is_feature_flag(flag))
                        .cloned(),
                );
            }
        }

        // if we know we're running from the `pgx-tests/src/framework.rs`, remove any user-specified features
        // that aren't valid for the manifest
        if test {
            features.features.retain(|flag| {
                if manifest.features.contains_key(flag) {
                    true
                } else {
                    use owo_colors::OwoColorize;
                    println!(
                        "{} feature `{}`",
                        "    Ignoring".bold().yellow(),
                        flag.bold().white()
                    );
                    false
                }
            });
        }

        // no matter what, we need the postgres version we determined to be included in the
        // set of features to compile with
        if !features.features.contains(pg_version.label()) {
            features.features.push(pg_version.label().clone());
        }
    }
}

pub(crate) fn pg_config_and_version<'a>(
    pgx: &'a Pgx,
    manifest: &Manifest,
    specified_pg_version: Option<String>,
    user_features: Option<&mut Features>,
    verbose: bool,
) -> eyre::Result<(&'a PgConfig, PgVersionSource)> {
    let pg_version = {
        'outer: loop {
            if let Some(pg_version) = specified_pg_version {
                // the user gave us an explicit Postgres version to use, so we will
                break 'outer Some(PgVersionSource::CliArgument(pg_version));
            } else if let Some(features) = user_features.as_ref() {
                // the user did not give us an explicit Postgres version, so see if there's one in the set
                // of `--feature` flags they gave us
                for flag in &features.features {
                    if pgx.is_feature_flag(flag) {
                        // use the first feature flag that is a Postgres version we support
                        break 'outer Some(PgVersionSource::FeatureFlag(flag.clone()));
                    }
                }

                // user didn't give us a feature flag that is a Postgres version

                // if they didn't ask for `--no-default-features` lets see if we have a default
                // postgres version feature specified in the manifest
                if !features.no_default_features {
                    if let Some(default_features) = manifest.features.get("default") {
                        for flag in default_features {
                            if pgx.is_feature_flag(flag) {
                                break 'outer Some(PgVersionSource::DefaultFeature(flag.clone()));
                            }
                        }
                    }
                }
            } else {
                // lets check the manifest for a default feature
                if let Some(default_features) = manifest.features.get("default") {
                    for flag in default_features {
                        if pgx.is_feature_flag(flag) {
                            break 'outer Some(PgVersionSource::DefaultFeature(flag.clone()));
                        }
                    }
                }
            }

            // we cannot determine the Postgres version the user wants to use
            break 'outer None;
        }
    };

    match pg_version {
        Some(pg_version) => {
            // we have determined a Postgres version

            modify_features_for_version(pgx, user_features, manifest, &pg_version, false);
            let pg_config = pgx.get(&pg_version.label())?;

            if verbose {
                display_version_info(pg_config, &pg_version);
            }

            Ok((pg_config, pg_version))
        }
        None => Err(eyre!("Could not determine which Postgres version feature flag to use")),
    }
}

pub(crate) fn display_version_info(pg_config: &PgConfig, pg_version: &PgVersionSource) {
    use owo_colors::OwoColorize;
    eprintln!(
        "{} {:?} and `pg_config` from {}",
        "       Using".bold().green(),
        pg_version.bold().white(),
        pg_config.path().unwrap().display().cyan()
    );
}

pub(crate) fn get_package_manifest(
    features: &Features,
    package_nane: Option<&String>,
    manifest_path: Option<impl AsRef<std::path::Path>>,
) -> eyre::Result<(Manifest, PathBuf)> {
    let metadata = crate::metadata::metadata(&features, manifest_path.as_ref())
        .wrap_err("couldn't get cargo metadata")?;
    crate::metadata::validate(&metadata)?;
    let package_manifest_path = crate::manifest::manifest_path(&metadata, package_nane)
        .wrap_err("Couldn't get manifest path")?;

    Ok((
        Manifest::from_path(&package_manifest_path).wrap_err("Couldn't parse manifest")?,
        package_manifest_path,
    ))
}
