/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
use cargo_metadata::{Metadata, MetadataCommand};
use eyre::eyre;
use semver::VersionReq;
use std::path::Path;

pub fn metadata(
    features: &clap_cargo::Features,
    manifest_path: Option<impl AsRef<Path>>,
) -> eyre::Result<Metadata> {
    let mut metadata_command = MetadataCommand::new();
    if let Some(manifest_path) = manifest_path {
        metadata_command.manifest_path(manifest_path.as_ref().to_owned());
    }
    features.forward_metadata(&mut metadata_command);
    let metadata = metadata_command.exec()?;
    Ok(metadata)
}

#[tracing::instrument(level = "error", skip_all)]
pub fn validate(metadata: &Metadata) -> eyre::Result<()> {
    let cargo_pgrx_version = env!("CARGO_PKG_VERSION");
    let cargo_pgrx_version_req = VersionReq::parse(&format!("~{}", cargo_pgrx_version))?;

    let pgrx_packages = metadata.packages.iter().filter(|package| {
        package.name == "pgrx"
            || package.name == "pgrx-sql-entity-graph"
            || package.name == "pgrx-macros"
            || package.name == "pgrx-tests"
    });

    for package in pgrx_packages {
        let package_semver = package.version.clone();
        if !cargo_pgrx_version_req.matches(&package_semver) {
            return Err(eyre!(
                r#"`{}-{}` shouldn't be used with `cargo-pgrx-{}`, please use `{} = "~{}"` in your `Cargo.toml`."#,
                package.name,
                package.version,
                cargo_pgrx_version,
                package.name,
                cargo_pgrx_version,
            ));
        } else {
            tracing::trace!(
                "`{}-{}` is compatible with `cargo-pgrx-{}`.",
                package.name,
                package.version,
                cargo_pgrx_version,
            )
        }
    }

    Ok(())
}
