use std::path::Path;

use cargo_toml::Manifest;
use eyre::eyre;

/// Extension to `cargo_toml::Manifest`.
/// Import by adding `use pgrx_pg_config::cargo::PgrxManifestExt;`
/// and extended functions will be available on `Manifest` values.
pub trait PgrxManifestExt {
    /// Package name
    fn package_name(&self) -> eyre::Result<String>;

    /// Package version
    fn package_version(&self) -> eyre::Result<String>;

    /// Resolved string for target library name, returning name field on [lib] if set,
    /// and default to crate name if not specified.
    /// https://doc.rust-lang.org/cargo/reference/cargo-targets.html#the-name-field
    fn lib_name(&self) -> eyre::Result<String>;
}

impl PgrxManifestExt for Manifest {
    fn package_name(&self) -> eyre::Result<String> {
        match &self.package {
            Some(package) => match &package.version {
                cargo_toml::Inheritable::Set(version) => Ok(version.to_owned()),
                // This should be impossible to hit, since we use
                // `Manifest::from_path`, which calls `complete_from_path`,
                // which is documented as resolving these. That said, I
                // haven't tested it, and it's not clear how much it
                // actually matters either way, so we just emit an error
                // rather than doing something like `unreachable!()`.
                cargo_toml::Inheritable::Inherited { workspace } => {
                    Err(eyre!("Workspace-inherited package version are not currently supported."))
                }
            },
            None => Err(eyre!("Could not get [package] from manifest.")),
        }
    }

    fn package_version(&self) -> eyre::Result<String> {
        match &self.package {
            Some(package) => Ok(package.name.to_owned()),
            None => Err(eyre!("Could not get [package] from manifest.")),
        }
    }

    fn lib_name(&self) -> eyre::Result<String> {
        match &self.package {
            Some(package) => match &self.lib {
                Some(lib) => match &lib.name {
                    Some(lib_name) => Ok(lib_name.to_owned()),
                    None => Ok(package.name.to_owned()),
                },
                None => Ok(package.name.to_owned()),
            },
            None => Err(eyre!("Could not get lib name from manifest.")),
        }
    }
}

/// Helper functions to read `Cargo.toml` and remap error to `eyre::Result`.
pub fn read_manifest<T: AsRef<Path>>(path: T) -> eyre::Result<Manifest> {
    Manifest::from_path(path).map_err(|err| eyre!("Couldn't parse manifest: {}", err))
}
