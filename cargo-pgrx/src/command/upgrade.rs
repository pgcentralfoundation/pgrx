use crate::CommandExecute;


/// Upgrade pgrx crate versions in `Cargo.toml`.
/// Defaults to latest.
#[derive(clap::Args, Debug)]
#[clap(author)]
pub(crate) struct Upgrade {
    /// Specify a version to upgrade to, rather than defaulting to the latest
    /// version.
    #[clap(long, short)]
    pub(crate) version: Option<String>, //TODO: typed version not string
    /// Upgrade versions in all packages in this workspace, rather than only
    /// the top-level package.
    #[clap(long, short)]
    pub(crate) recursive: bool,
}

impl CommandExecute for Upgrade {
    #[tracing::instrument(level = "error", skip(self))]
    fn execute(self) -> eyre::Result<()> {
        println!("Hello, world!");
        Ok(())
    }
}
