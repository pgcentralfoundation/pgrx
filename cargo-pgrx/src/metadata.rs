use std::collections::BTreeMap;
//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use cargo_metadata::{Metadata, MetadataCommand};
use eyre::eyre;
use owo_colors::*;
use semver::VersionReq;
use std::path::Path;

pub(crate) fn metadata(
    features: &clap_cargo::Features,
    manifest_path: Option<&Path>,
    cargo_flags: &[String],
) -> eyre::Result<Metadata> {
    let mut metadata_command = MetadataCommand::new();
    if let Some(manifest_path) = manifest_path {
        metadata_command.manifest_path(manifest_path.to_owned());
    }

    // The `--cargo` passthrough must reach `cargo metadata` — the first cargo invocation, where config issues (private registries, etc.) fail first.
    let cargo_flags = split_cargo_flags(cargo_flags);
    if !cargo_flags.is_empty() {
        metadata_command.other_options(cargo_flags);
    }
    features.forward_metadata(&mut metadata_command);
    let metadata = metadata_command.exec()?;
    Ok(metadata)
}

/// Split each `--cargo` value on whitespace, so both the single-token form
/// (`--cargo=--config=child/.cargo/config.toml`) and the quoted-string form
/// (`--cargo "--config foo"`) work, forwarding each token as one argv element.
// ponytail: naive whitespace split, same as PGRX_BUILD_FLAGS in cargo.rs. This splits *every* value, including the single-token `--cargo=--config=x` form, so a flag value containing spaces (e.g. a config path with a space) is NOT supported by any form; use a space-free path, or pass config inline as `--cargo=--config=key="value"`. Upgrade path if that ever bites: shell-style tokenization that honors quotes instead of a raw whitespace split.
pub(crate) fn split_cargo_flags(cargo_flags: &[String]) -> Vec<String> {
    cargo_flags.iter().flat_map(|f| f.split_ascii_whitespace().map(str::to_owned)).collect()
}

#[tracing::instrument(level = "error", skip_all)]
pub fn validate(path: Option<&Path>, metadata: &Metadata) -> eyre::Result<()> {
    let cargo_pgrx_version = env!("CARGO_PKG_VERSION");
    let cargo_pgrx_version_req = VersionReq::parse(&format!("~{cargo_pgrx_version}"))?;

    let pgrx_packages = metadata.packages.iter().filter(|package| {
        package.name == "pgrx"
            || package.name == "pgrx-sql-entity-graph"
            || package.name == "pgrx-macros"
            || package.name == "pgrx-tests"
    });

    let mut mismatches = BTreeMap::new();
    let (mut unified, mut universion) = (true, None);
    for package in pgrx_packages {
        let package_semver = package.version.clone();
        if unified && universion.as_ref().is_some_and(|v| v != &package_semver) {
            unified = false;
        } else if universion.is_none() {
            universion = Some(package_semver.clone());
        };
        if !cargo_pgrx_version_req.matches(&package_semver) {
            mismatches.insert(package.name.clone(), package.version.clone());
        } else {
            tracing::trace!(
                "`{}-{}` is compatible with `cargo-pgrx-{}`.",
                package.name,
                package.version,
                cargo_pgrx_version,
            )
        }
    }

    if !mismatches.is_empty() {
        let many = mismatches.len();
        let mismatches = mismatches
            .into_iter()
            .map(|(p, v)| format!("{p} = {v}"))
            .collect::<Vec<_>>()
            .join(", ");

        let help = if let (true, Some(version)) = (unified, universion) {
            let version = version.clone();
            format!(
                "{prefix} cargo install cargo-pgrx --version {version} --locked",
                prefix = "help:".bold()
            )
        } else {
            String::new()
        };
        return Err(eyre!(
            r#"The installed cargo-pgrx {cargo_pgrx_version} is not compatible with the {} in {}:
{mismatches}
cargo-pgrx and pgrx library versions must be identical.
{help}"#,
            if many == 1 { "dependency" } else { "dependencies" },
            path.map(|p| p.display().to_string())
                .unwrap_or_else(|| "./Cargo.toml".to_string())
                .yellow(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::split_cargo_flags;

    #[test]
    fn split_cargo_flags_handles_both_forms() {
        // single-token form is passed through untouched
        assert_eq!(
            split_cargo_flags(&["--config=child/.cargo/config.toml".into()]),
            vec!["--config=child/.cargo/config.toml"]
        );
        // quoted-string form is split into separate argv tokens
        assert_eq!(
            split_cargo_flags(&["--config foo --offline".into()]),
            vec!["--config", "foo", "--offline"]
        );
        // repeatable `--cargo` and whitespace-splitting compose
        assert_eq!(
            split_cargo_flags(&["--offline".into(), "--config foo".into()]),
            vec!["--offline", "--config", "foo"]
        );
        // documented limitation: a value with an embedded space is split too,
        // even in the single-token form, so space-containing paths are unsupported
        assert_eq!(
            split_cargo_flags(&["--config=/my path/config.toml".into()]),
            vec!["--config=/my", "path/config.toml"]
        );
    }
}
