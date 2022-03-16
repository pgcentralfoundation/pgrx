use cargo_metadata::Metadata;
use cargo_toml::Manifest;
use eyre::eyre;
use std::path::PathBuf;
use pgx_utils::SUPPORTED_MAJOR_VERSIONS;

#[tracing::instrument(skip_all)]
pub(crate) fn manifest_path(metadata: &Metadata, package_name: Option<&String>) -> eyre::Result<PathBuf> {
    let manifest_path = if let Some(package_name) = package_name {
        let found = metadata.packages.iter()
            .find(|v| v.name == *package_name)
            .ok_or_else(|| eyre!("Could not find package `{}`", package_name))?;
        tracing::debug!(manifest_path = %found.manifest_path, "Found workspace package");
        found.manifest_path.clone().into_std_path_buf()
    } else {
        let root = metadata
            .root_package()
            .ok_or(eyre!("`pgx` requires a root package in a workspace when `--package` is not specified."))?;
        tracing::debug!(manifest_path = %root.manifest_path, "Found root package");
        root.manifest_path.clone().into_std_path_buf()
    };
    Ok(manifest_path)
}

pub(crate) fn default_pg_version(manifest: &Manifest) -> Option<String> {
    let default_features = manifest.features.get("default")?;
    for default_feature in default_features {
        for major_version in SUPPORTED_MAJOR_VERSIONS {
            let potential_feature = format!("pg{}", major_version);
            if *default_feature == format!("pg{}", major_version) {
                return Some(potential_feature);
            }
        }
    }
    None
}

pub(crate) fn features_for_version(
    mut features: clap_cargo::Features,
    manifest: &Manifest,
    pg_version: &String,
) -> clap_cargo::Features {
    let default_features = manifest.features.get("default");

    match default_features {
        Some(default_features) => {
            if default_features.contains(&pg_version) {
                return features;
            }
            let default_features = default_features
                .iter()
                .filter(|default_feature| {
                    for supported_major in SUPPORTED_MAJOR_VERSIONS {
                        if **default_feature == format!("pg{}", supported_major) {
                            return false;
                        }
                    }
                    true
                })
                .cloned()
                .collect::<Vec<_>>();
            features.no_default_features = true;
            features.features.extend(default_features);
            if features.features.iter().all(|f| f != pg_version) {
                features.features.push(pg_version.clone());
            }
        }
        None => (),
    };

    features
}
