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
use crate::cargo::CargoProfile;
use crate::command::get::get_property;
use crate::command::install::install_extension;
use crate::manifest::{PgVersionSource, display_version_info};
use cargo_toml::Manifest;
use eyre::{WrapErr, eyre};
use pgrx_pg_config::{PgConfig, Pgrx, get_target_dir};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::io::Write;

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
    /// Create a PGXN-compatible package with META.json
    #[clap(long)]
    pub(crate) pgxn: bool,
    /// Package format: directory (default) or zip
    #[clap(long, value_parser = ["directory", "zip"])]
    pub(crate) format: Option<String>,
    #[clap(flatten)]
    pub(crate) features: clap_cargo::Features,
    #[clap(long)]
    pub(crate) target: Option<String>,
    #[clap(from_global, action = ArgAction::Count)]
    pub(crate) verbose: u8,
}

/// PGXN META.json structure
#[derive(Debug, Serialize, Deserialize)]
pub struct PgxnMeta {
    pub name: String,
    #[serde(rename = "abstract")]
    pub abstract_: String,
    pub description: String,
    pub version: String,
    pub maintainer: Vec<String>,
    pub license: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provides: Option<BTreeMap<String, ProvideEntry>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prereqs: Option<PrereqsMap>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<ResourcesMap>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "meta-spec")]
    pub meta_spec: Option<MetaSpec>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProvideEntry {
    #[serde(rename = "abstract")]
    pub abstract_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docfile: Option<String>,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrereqsMap {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<PrereqsRuntime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrereqsRuntime {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires: Option<BTreeMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommends: Option<BTreeMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResourcesMap {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bugtracker: Option<BugTracker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<Repository>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BugTracker {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web: Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MetaSpec {
    pub version: String,
    pub url: String,
}

impl Package {
    pub(crate) fn perform(mut self) -> eyre::Result<(PathBuf, Vec<PathBuf>)> {
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

        // Handle PGXN packaging if requested
        if self.pgxn {
            let extname = get_property(&package_manifest_path, "extname")?
                .ok_or(eyre!("could not determine extension name"))?;
            create_pgxn_package(
                &out_dir,
                &package_manifest_path,
                &extname,
                self.format.as_deref(),
            )?;
        }

        Ok((out_dir, output_files))
    }
}

impl CommandExecute for Package {
    #[tracing::instrument(level = "error", skip(self))]
    fn execute(self) -> eyre::Result<()> {
        let (out_dir, _) = self.perform()?;
        if self.pgxn {
            let is_zip = self.format.as_deref() == Some("zip");
            let msg = if is_zip {
                format!(
                    "PGXN package created successfully at: {}/pgxn-package.zip",
                    out_dir.display()
                )
            } else {
                format!(
                    "PGXN package created successfully in directory: {}",
                    out_dir.display()
                )
            };
            println!("{}", msg);
        }
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

/// Create a PGXN-compatible package including META.json
fn create_pgxn_package(
    out_dir: &PathBuf,
    manifest_path: &Path,
    extname: &str,
    format: Option<&str>,
) -> eyre::Result<()> {
    // Read and parse the Cargo.toml
    let manifest = Manifest::from_path(manifest_path).wrap_err("Couldn't parse manifest")?;
    let package = &manifest.package.as_ref().ok_or(eyre!("No package section in Cargo.toml"))?;

    // Create META.json structure
    let mut meta = PgxnMeta {
        name: package.name.clone(),
        abstract_: package
            .metadata
            .as_ref()
            .and_then(|m| m.get("pgxn"))
            .and_then(|pgxn| pgxn.get("abstract"))
            .and_then(|v| v.as_str())
            .unwrap_or(&package.description)
            .to_string(),
        description: package.description.clone().unwrap_or_default(),
        version: package.version.as_ref().unwrap_or(&"0.0.0".to_string()).clone(),
        maintainer: package
            .authors
            .clone()
            .unwrap_or_else(|| vec!["Unknown <unknown@example.com>".to_string()]),
        license: package.license.clone().unwrap_or_else(|| "postgresql".to_string()),
        provides: None,
        prereqs: None,
        resources: None,
        tags: package.keywords.clone(),
        generated_by: Some("cargo-pgrx".to_string()),
        meta_spec: Some(MetaSpec {
            version: "1.0.0".to_string(),
            url: "https://pgxn.org/meta/spec.txt".to_string(),
        }),
    };

    // Extract PGXN-specific metadata from Cargo.toml if available
    if let Some(toml::Value::Table(pgxn_meta)) = manifest.original.get("package")
        .and_then(|p| match p {
            toml::Value::Table(t) => t.get("metadata"),
            _ => None,
        })
        .and_then(|m| match m {
            toml::Value::Table(t) => t.get("pgxn"),
            _ => None,
        })
    {
        // Extract tags if present
        if let Some(toml::Value::Array(tags)) = pgxn_meta.get("tags") {
            meta.tags = Some(
                tags.iter()
                    .filter_map(|t| match t {
                        toml::Value::String(s) => Some(s.clone()),
                        _ => None,
                    })
                    .collect(),
            );
        }

        // Extract bugtracker if present
        if let Some(bugtracker_str) = pgxn_meta.get("bugtracker").and_then(|v| v.as_str()) {
            meta.resources = Some(ResourcesMap {
                bugtracker: Some(BugTracker {
                    web: Some(bugtracker_str.to_string()),
                }),
                repository: package
                    .repository
                    .as_ref()
                    .map(|url| Repository {
                        web: Some(url.clone()),
                        url: Some(format!("git://{}", url.replace("https://", ""))),
                        r#type: Some("git".to_string()),
                    }),
            });
        } else if let Some(repo) = &package.repository {
            meta.resources = Some(ResourcesMap {
                bugtracker: None,
                repository: Some(Repository {
                    web: Some(repo.clone()),
                    url: Some(format!("git://{}", repo.replace("https://", ""))),
                    r#type: Some("git".to_string()),
                }),
            });
        }
    } else if let Some(repo) = &package.repository {
        meta.resources = Some(ResourcesMap {
            bugtracker: None,
            repository: Some(Repository {
                web: Some(repo.clone()),
                url: Some(format!("git://{}", repo.replace("https://", ""))),
                r#type: Some("git".to_string()),
            }),
        });
    }

    // Add provides section
    let mut provides = BTreeMap::new();
    provides.insert(
        extname.to_string(),
        ProvideEntry {
            abstract_: meta.abstract_.clone(),
            file: Some(format!("share/postgresql/extension/{extname}-{}.sql", meta.version)),
            docfile: Some("README.md".to_string()),
            version: meta.version.clone(),
        },
    );
    meta.provides = Some(provides);

    // Write META.json
    let meta_json = serde_json::to_string_pretty(&meta)
        .wrap_err("Failed to serialize META.json")?;
    let meta_path = out_dir.join("META.json");
    fs::write(&meta_path, &meta_json)
        .wrap_err("Failed to write META.json")?;

    // Copy README.md if it exists
    let manifest_dir = manifest_path.parent().ok_or(eyre!("Invalid manifest path"))?;
    let readme_src = manifest_dir.join("README.md");
    if readme_src.exists() {
        let readme_dst = out_dir.join("README.md");
        fs::copy(&readme_src, &readme_dst)
            .wrap_err("Failed to copy README.md")?;
    }

    // Copy LICENSE if it exists
    let license_src = manifest_dir.join("LICENSE");
    if license_src.exists() {
        let license_dst = out_dir.join("LICENSE");
        fs::copy(&license_src, &license_dst)
            .wrap_err("Failed to copy LICENSE")?;
    }

    // Create zip file if requested
    if format == Some("zip") {
        create_pgxn_zip(&out_dir, extname)?;
    }

    Ok(())
}

/// Create a zip file with all PGXN package contents
fn create_pgxn_zip(package_dir: &PathBuf, extname: &str) -> eyre::Result<()> {
    use zip::ZipWriter;

    let zip_path = package_dir.join("pgxn-package.zip");
    let file = fs::File::create(&zip_path)
        .wrap_err("Failed to create zip file")?;
    let mut zip = ZipWriter::new(file);

    let options = zip::FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);

    // Add META.json
    let meta_path = package_dir.join("META.json");
    if meta_path.exists() {
        let content = fs::read(&meta_path)
            .wrap_err("Failed to read META.json")?;
        zip.start_file("META.json", options.clone())
            .wrap_err("Failed to add META.json to zip")?;
        zip.write_all(&content)
            .wrap_err("Failed to write META.json to zip")?;
    }

    // Add README.md
    let readme_path = package_dir.join("README.md");
    if readme_path.exists() {
        let content = fs::read(&readme_path)
            .wrap_err("Failed to read README.md")?;
        zip.start_file("README.md", options.clone())
            .wrap_err("Failed to add README.md to zip")?;
        zip.write_all(&content)
            .wrap_err("Failed to write README.md to zip")?;
    }

    // Add LICENSE
    let license_path = package_dir.join("LICENSE");
    if license_path.exists() {
        let content = fs::read(&license_path)
            .wrap_err("Failed to read LICENSE")?;
        zip.start_file("LICENSE", options.clone())
            .wrap_err("Failed to add LICENSE to zip")?;
        zip.write_all(&content)
            .wrap_err("Failed to write LICENSE to zip")?;
    }

    // Add lib and share directories
    add_dir_to_zip(&mut zip, package_dir, "lib", &options)
        .wrap_err("Failed to add lib directory to zip")?;
    add_dir_to_zip(&mut zip, package_dir, "share", &options)
        .wrap_err("Failed to add share directory to zip")?;

    zip.finish()
        .wrap_err("Failed to finalize zip file")?;

    Ok(())
}

/// Recursively add a directory to a zip file
fn add_dir_to_zip(
    zip: &mut zip::ZipWriter<std::fs::File>,
    base_dir: &PathBuf,
    dir_name: &str,
    options: &zip::FileOptions,
) -> eyre::Result<()> {
    let dir_path = base_dir.join(dir_name);
    if !dir_path.exists() {
        return Ok(());
    }

    for entry in fs::read_dir(&dir_path).wrap_err("Failed to read directory")? {
        let entry = entry.wrap_err("Failed to read directory entry")?;
        let path = entry.path();
        let file_name = entry
            .file_name()
            .to_string_lossy()
            .to_string();

        if path.is_dir() {
            // Recursively add subdirectories
            for sub_entry in fs::read_dir(&path).wrap_err("Failed to read subdirectory")? {
                let sub_entry = sub_entry.wrap_err("Failed to read subdirectory entry")?;
                let sub_path = sub_entry.path();
                let sub_file_name = sub_entry
                    .file_name()
                    .to_string_lossy()
                    .to_string();
                let zip_path = format!("{}/{}/{}", dir_name, file_name, sub_file_name);

                if sub_path.is_file() {
                    let content = fs::read(&sub_path)
                        .wrap_err("Failed to read file")?;
                    zip.start_file(zip_path, options.clone())
                        .wrap_err("Failed to add file to zip")?;
                    zip.write_all(&content)
                        .wrap_err("Failed to write file to zip")?;
                }
            }
        } else {
            let zip_path = format!("{}/{}", dir_name, file_name);
            let content = fs::read(&path)
                .wrap_err("Failed to read file")?;
            zip.start_file(zip_path, options.clone())
                .wrap_err("Failed to add file to zip")?;
            zip.write_all(&content)
                .wrap_err("Failed to write file to zip")?;
        }
    }

    Ok(())
}
