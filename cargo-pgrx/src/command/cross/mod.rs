use crate::CommandExecute;
pub(crate) mod pgrx_target;

/// Commands having to do with cross-compilation. (Experimental)
#[derive(clap::Args, Debug)]
#[clap(about, author)]
pub(crate) struct Cross {
    #[command(subcommand)]
    pub(crate) subcommand: CargoPgrxCrossSubCommands,
}

impl CommandExecute for Cross {
    fn execute(self) -> eyre::Result<()> {
        self.subcommand.execute()
    }
}

/// Subcommands relevant to cross-compilation.
#[derive(clap::Subcommand, Debug)]
pub(crate) enum CargoPgrxCrossSubCommands {
    PgrxTarget(pgrx_target::PgrxTarget),
}

impl CommandExecute for CargoPgrxCrossSubCommands {
    fn execute(self) -> eyre::Result<()> {
        use CargoPgrxCrossSubCommands::*;
        match self {
            PgrxTarget(target_info) => target_info.execute(),
        }
    }
}
