use cargo_metadata::{Metadata, MetadataCommand};
use eyre::eyre;
use semver::{Version, VersionReq};

pub fn metadata(features: &clap_cargo::Features) -> eyre::Result<Metadata> {
    let mut metadata_command = MetadataCommand::new();
    features.forward_metadata(&mut metadata_command);
    let metadata = metadata_command.exec()?;
    Ok(metadata)
}

#[tracing::instrument(level = "error", skip_all)]
pub fn validate(metadata: &Metadata) -> eyre::Result<()> {
    let cargo_pgx_version = env!("CARGO_PKG_VERSION");
    let cargo_pgx_version_req = VersionReq::parse(&format!("~{}", cargo_pgx_version))?;

    let pgx_packages = metadata.packages.iter().filter(|package| {
        package.name == "pgx"
            || package.name == "pgx-utils"
            || package.name == "pgx-macros"
            || package.name == "pgx-tests"
    });

    for package in pgx_packages {
        let package_semver = metadata_version_to_semver(package.version.clone());
        if !cargo_pgx_version_req.matches(&package_semver) {
            return Err(eyre!(
                r#"`{}-{}` shouldn't be used with `cargo-pgx-{}`, please use `{} = "~{}"` in your `Cargo.toml`."#,
                package.name,
                package.version,
                cargo_pgx_version,
                package.name,
                cargo_pgx_version,
            ));
        } else {
            tracing::trace!(
                "`{}-{}` is compatible with `cargo-pgx-{}`.",
                package.name,
                package.version,
                cargo_pgx_version,
            )
        }
    }

    Ok(())
}

fn metadata_version_to_semver(metadata_version: cargo_metadata::Version) -> semver::Version {
    Version {
        major: metadata_version.major,
        minor: metadata_version.minor,
        patch: metadata_version.patch,
        pre: metadata_version.pre,
        build: metadata_version.build,
    }
}
