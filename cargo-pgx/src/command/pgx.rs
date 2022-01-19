use crate::CommandExecute;

#[derive(clap::Args, Debug)]
#[clap(about, author)]
pub(crate) struct Pgx {
    #[clap(subcommand)]
    subcommand: CargoPgxSubCommands,
    #[clap(from_global, parse(from_occurrences))]
    verbose: usize,
}

impl CommandExecute for Pgx {
    fn execute(self) -> eyre::Result<()> {
        self.subcommand.execute()
    }
}

#[derive(clap::Subcommand, Debug)]
enum CargoPgxSubCommands {
    Init(super::init::Init),
    Start(super::start::Start),
    Stop(super::stop::Stop),
    Status(super::status::Status),
    New(super::new::New),
    Install(super::install::Install),
    Package(super::package::Package),
    Schema(super::schema::Schema),
    Run(super::run::Run),
    Connect(super::connect::Connect),
    Test(super::test::Test),
    Get(super::get::Get),
}

impl CommandExecute for CargoPgxSubCommands {
    fn execute(self) -> eyre::Result<()> {
        use CargoPgxSubCommands::*;
        match self {
            Init(c) => c.execute(),
            Start(c) => c.execute(),
            Stop(c) => c.execute(),
            Status(c) => c.execute(),
            New(c) => c.execute(),
            Install(c) => c.execute(),
            Package(c) => c.execute(),
            Schema(c) => c.execute(),
            Run(c) => c.execute(),
            Connect(c) => c.execute(),
            Test(c) => c.execute(),
            Get(c) => c.execute(),
        }
    }
}
