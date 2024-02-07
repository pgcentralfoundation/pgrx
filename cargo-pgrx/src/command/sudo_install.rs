use std::path::{Path, PathBuf};
use std::process::Command;

use owo_colors::OwoColorize;

use crate::command::install::Install;
use crate::command::package::Package;
use crate::CommandExecute;

/// Like `cargo pgrx install`, but uses `sudo` to copy the extension files
#[derive(Debug, Clone)]
pub(crate) struct SudoInstall {
    package: Option<String>,
    manifest_path: Option<PathBuf>,
    release: bool,
    profile: Option<String>,
    test: bool,
    pg_config: Option<PathBuf>,
    out_dir: Option<PathBuf>,
    features: clap_cargo::Features,
    verbose: u8,
}

impl From<Install> for SudoInstall {
    fn from(value: Install) -> Self {
        Self {
            package: value.package,
            manifest_path: value.manifest_path,
            release: value.release,
            profile: value.profile,
            test: value.test,
            pg_config: value.pg_config.map(PathBuf::from),
            out_dir: None,
            features: value.features,
            verbose: value.verbose,
        }
    }
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
            let dest_abs = make_absolute(src.strip_prefix(&outdir)?);
            let dest = dest_abs.canonicalize().unwrap_or(dest_abs);

            // we're about to run `sudo` to copy some files, one at a time
            let mut command = Command::new("sudo"); // NB:  If we ever support Windows...
            command.arg("cp").arg(&src).arg(&dest);

            println!(
                "{} sudo cp {} {}",
                "       Running".bold().green(),
                src.display(),
                dest.display()
            );
            let mut child = command.spawn()?;

            let status = child.wait()?;
            if !status.success() {
                // sudo failed.  let the user know and get out now
                eprintln!("sudo command failed with status `{}`", format!("{status:?}").red());
                std::process::exit(status.code().unwrap_or(1));
            }
        }

        Ok(())
    }
}
