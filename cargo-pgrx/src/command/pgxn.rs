//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

/// PGXN (PostgreSQL eXtension Network) support for cargo-pgrx
/// Generates META.json files and creates PGXN-compatible packages

use cargo_toml::Manifest;
use eyre::{WrapErr, eyre};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::io::Write;

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

/// Create a PGXN-compatible package including META.json
pub fn create_pgxn_package(
    out_dir: &PathBuf,
    manifest_path: &std::path::Path,
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
        create_zip(out_dir, extname)?;
    }

    Ok(())
}

/// Create a zip file with all PGXN package contents
fn create_zip(package_dir: &PathBuf, _extname: &str) -> eyre::Result<()> {
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
