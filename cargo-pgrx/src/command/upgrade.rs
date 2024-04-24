use std::{fs::OpenOptions, io::Read, path::PathBuf};
use eyre::eyre;
use toml_edit::{Document, DocumentMut};

use crate::{manifest, CommandExecute};


/// Upgrade pgrx crate versions in `Cargo.toml`.
/// Defaults to latest.
#[derive(clap::Args, Debug)]
#[clap(author)]
pub(crate) struct Upgrade {
    /// Specify a version to upgrade to, rather than defaulting to the latest
    /// version.
    #[clap(long, short)]
    pub(crate) target_version: Option<String>, //TODO: typed version not string

    /// Path to the manifest file (usually Cargo.toml). Defaults to 
    /// "./Cargo.toml" in the working directory.
    #[clap(long, short)]
    pub(crate) path: Option<PathBuf>,
}

// Locate the version property for an item in a Cargo.toml document. 
fn find_version_property<'a>(cargo_toml: &'a mut DocumentMut, target_crate: &'static str, section: &'static str) -> eyre::Result<Option<&'a mut toml_edit::Item>> {
    if let Some(deps_table) = cargo_toml.get_mut(section).map(|item| {
            item.as_table_like_mut().ok_or(eyre!("The Cargo.toml section [{section}] is not a table."))
        })
    {
        let deps_table = deps_table?;
        Ok(deps_table.get_mut(target_crate))
    }
    else {
        Ok(None)
    }
}

impl CommandExecute for Upgrade {
    #[tracing::instrument(level = "error", skip(self))]
    fn execute(self) -> eyre::Result<()> {
        const PGRX_STR: &'static str = "pgrx";
        let path = self.path.unwrap_or(PathBuf::from("./Cargo.toml"));
        let mut cargo_toml_string = String::new();
        let mut file = OpenOptions::new().read(true).open(path)?;
        file.read_to_string(&mut cargo_toml_string)?;
        // Assume working directory
        let mut manifest = cargo_toml_string.parse::<DocumentMut>()?;
        let table = manifest.as_table_mut();
        println!("Table is: {:#?}", table);
        
        let pgrx_property = find_version_property(&mut manifest, "pgrx", "dependencies")?;

        println!("pgrx dependency is: {:#?}", pgrx_property);
        Ok(())
    }
}
