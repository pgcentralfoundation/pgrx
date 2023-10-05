use std::path::{Path, PathBuf};
use std::process::Command;

use owo_colors::OwoColorize;

use crate::command::package::Package;
use crate::CommandExecute;

/// Like `cargo pgrx install`, but uses `sudo` to copy the extension files
#[derive(clap::Args, Debug, Clone)]
#[clap(author)]
pub(crate) struct SudoInstall {
    /// Package to build (see `cargo help pkgid`)
    #[clap(long, short)]
    package: Option<String>,
    /// Path to Cargo.toml
    #[clap(long, value_parser)]
    manifest_path: Option<PathBuf>,
    /// Compile for release mode (default is debug)
    #[clap(long, short)]
    release: bool,
    /// Specific profile to use (conflicts with `--debug`)
    #[clap(long)]
    profile: Option<String>,
    /// Build in test mode (for `cargo pgrx test`)
    #[clap(long)]
    test: bool,
    /// The `pg_config` path (default is first in $PATH)
    #[clap(long, short = 'c', value_parser)]
    pg_config: Option<PathBuf>,
    /// The directory to output the package (default is `./target/[debug|release]/extname-pgXX/`)
    #[clap(long, value_parser)]
    out_dir: Option<PathBuf>,
    #[clap(flatten)]
    features: clap_cargo::Features,
    #[clap(from_global, action = ArgAction::Count)]
    verbose: u8,
}

impl From<SudoInstall> for Package {
    fn from(value: SudoInstall) -> Self {
        Package {
            package: value.package,
            manifest_path: value.manifest_path,
            debug: !value.release,
            profile: value.profile,
            test: value.test,
            pg_config: value.pg_config,
            out_dir: value.out_dir,
            features: value.features,
            verbose: value.verbose,
        }
    }
}

impl CommandExecute for SudoInstall {
    fn execute(self) -> eyre::Result<()> {
        fn make_absolute(p: &Path) -> PathBuf {
            PathBuf::from_iter(vec![PathBuf::from("/").as_path(), p])
        }

        // even tho we're an "install" command, directly use the `package` command to build
        // the extension as we want it to build out the directory structure in `./target/...`
        // from there we'll use sudo to copy each file created to the actual location on disk
        let package: Package = self.clone().into();
        let (outdir, output_files) = package.perform()?;

        eprintln!();
        eprintln!("Using sudo to copy extension files from {}", outdir.display().cyan());
        for src in output_files {
            let src = src.canonicalize()?;
            let dest = make_absolute(src.strip_prefix(&outdir)?).canonicalize()?;

            // we're about to run `sudo` to copy some files, one at a time
            let mut command = Command::new("sudo"); // NB:  If we ever support Windows...
            command.arg("cp").arg(src).arg(dest);

            println!("{} {:?}", "     Running".bold().green(), command);
            let output = command.output()?;
            if !output.status.success() {
                // it didn't work, so print out sudo's stdout and stderr streams
                println!("{}", String::from_utf8_lossy(&output.stdout));
                eprintln!("{}", String::from_utf8_lossy(&output.stderr));

                // and just exit now
                std::process::exit(output.status.code().unwrap_or(1));
            }
        }

        Ok(())
    }
}
