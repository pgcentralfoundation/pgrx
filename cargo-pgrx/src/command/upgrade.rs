use std::path::{Path, PathBuf};
use cargo_edit::Dependency;
use eyre::eyre;
use toml_edit::KeyMut;
use tracing::{debug, error, info, warn};

use crate::CommandExecute;

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
trait DependencySource {
    fn set_version<A: AsRef<str>>(&mut self, new_version: A);
    fn get_version<'a>(&'a self) -> Option<&'a String> ;
}
impl DependencySource for cargo_edit::Source {
    fn set_version<A: AsRef<str>>(&mut self, new_version: A) {
        match self {
            cargo_edit::Source::Registry(reg) => {
                reg.version = new_version.as_ref().to_string()
            },
            cargo_edit::Source::Path(path) => {
                path.version = Some(new_version.as_ref().to_string())
            },
            cargo_edit::Source::Git(git) => {
                git.version = Some(new_version.as_ref().to_string())
            },
            cargo_edit::Source::Workspace(_ws) => {
                error!("Cannot upgrade the version of \
                    a package because it is set in the \
                    workspace, please run `cargo pgrx \
                    upgrade` in the workspace directory.");
            },
        }
    }
    fn get_version<'a>(&'a self) -> Option<&'a String> {
        match self {
            cargo_edit::Source::Registry(reg) => Some(&reg.version),
            cargo_edit::Source::Path(path) => path.version.as_ref(),
            cargo_edit::Source::Git(git) => git.version.as_ref(),
            cargo_edit::Source::Workspace(_ws) => {
                error!("Cannot fetch the version of \
                    a package because it is set in the \
                    workspace, please run `cargo pgrx \
                    upgrade` in the workspace directory.");
                None
            },
        }
    }
}
fn replace_version<S>(new_version: &str, crate_root: &Path, key: &mut KeyMut, dep: &mut toml_edit::Item, mut parsed_dep: Dependency, mut source: S) -> eyre::Result<()> where S : Clone + DependencySource, cargo_edit::Source: From<S> {
    let dep_name = key.get();
    let ver_maybe = source.get_version();
    match ver_maybe {
        Some(v) => {
            debug!("{dep_name} version is {v}")
        },
        None => return Err(eyre!("No version field for {dep_name}, cannot upgrade.")),
    }
    source.set_version(new_version);
    parsed_dep = parsed_dep.set_source(source);

    parsed_dep.update_toml(crate_root, key, dep);
    Ok(())
}

impl CommandExecute for Upgrade {
    #[tracing::instrument(level = "error", skip(self))]
    fn execute(self) -> eyre::Result<()> {
        const RELEVANT_PACKAGES: [&'static str; 3] = ["pgrx",
            "pgrx-macros",
            "pgrx-test"];
        // Canonicalize because cargo-edit does not accept relative paths.
        let path = std::fs::canonicalize(
            self.path.unwrap_or(PathBuf::from("./Cargo.toml"))
        )?;

        let mut manifest = cargo_edit::LocalManifest::find(Some(&path))
            .map_err(|e| eyre!("Error opening manifest: {e}"))?;

        for dep_table in manifest.get_dependency_tables_mut() {
            for dep_name in RELEVANT_PACKAGES {
                let decor = dep_table.key_decor(dep_name).cloned();
                if let Some((mut key, dep)) = dep_table.get_key_value_mut(dep_name) {
                    let parsed_dep: Dependency = match Dependency::from_toml(
                            path.as_path(),
                            &key,
                            dep) {
                        Ok(dependency) => dependency, 
                        Err(e) => return Err(eyre!("Could not parse dependency \
                            entry for {dep_name} due to error: {e}")),
                    };
                    if let Some(source) = parsed_dep.source().cloned() {
                        debug!("Found dependency {dep_name} with current \
                            source {source:#?}");
                        if let cargo_edit::Source::Workspace(_) = &source {
                                error!("Cannot upgrade the version of \
                                    {dep_name} because it is set in the \
                                    workspace, please run `cargo pgrx \
                                    upgrade` in the workspace directory.");
                        }
                        else if source.get_version().is_some() {
                            replace_version("test", path.as_path(), &mut key, dep, parsed_dep, source)?;
                            // Workaround since update_toml() doesn't preserve comments
                            dep_table.key_decor_mut(dep_name).map(|dec| {
                                if let Some(prefix) = decor.as_ref().and_then(|val| val.prefix().cloned()) {
                                    dec.set_prefix(prefix)
                                }
                                if let Some(suffix) = decor.as_ref().and_then(|val| val.suffix().cloned()) { 
                                    dec.set_suffix(suffix)
                                }
                            });
                            
                        }
                        else {
                            info!("No version specified for {dep_name}, not upgrading.");
                        }
                    }
                }
                else {
                    debug!("Manifest does not contain a dependency entry for \
                        {dep_name}");
                }
            }
        }
        manifest.write().map_err(|err| { 
            eyre!("Unable to write the updated Cargo.toml to disk: {err}")
        })?;
        Ok(())
    }
}
