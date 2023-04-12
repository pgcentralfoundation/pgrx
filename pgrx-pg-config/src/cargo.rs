use std::path::Path;

use cargo_toml::Manifest;
use eyre::eyre;

/// Extension to `cargo_toml::Manifest`.
/// Import by adding `use pgrx_pg_config::cargo::PgrxManifestExt;`
/// and extended functions will be available on `Manifest` values.
pub trait PgrxManifestExt {
    /// Resolved string for target library name, returning name field on [lib] if set,
    /// and default to crate name if not specified.
    /// https://doc.rust-lang.org/cargo/reference/cargo-targets.html#the-name-field
    fn lib_name(&self) -> eyre::Result<String>;
}

impl PgrxManifestExt for Manifest {
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
