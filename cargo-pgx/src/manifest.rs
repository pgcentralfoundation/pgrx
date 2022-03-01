use cargo_metadata::Metadata;
use cargo_toml::Manifest;
use eyre::eyre;
use pgx_utils::SUPPORTED_MAJOR_VERSIONS;

pub(crate) fn manifest(metadata: &Metadata) -> eyre::Result<Manifest> {
    let root = metadata
        .root_package()
        .ok_or(eyre!("`pgx` requires a root package."))?;
    let manifest = Manifest::from_path(&root.manifest_path)?;
    Ok(manifest)
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
