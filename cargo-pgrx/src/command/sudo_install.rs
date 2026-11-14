use std::path::{Path, PathBuf};
use std::process::Command;

use owo_colors::OwoColorize;

use crate::CommandExecute;
use crate::command::install::Install;
use crate::command::package::Package;

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
    target: Option<String>,
    no_assemble_unreleased: bool,
    verbose: u8,
    cargo: Vec<String>,
    prefix_dir: Option<PathBuf>,
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
            target: value.target,
            no_assemble_unreleased: value.no_assemble_unreleased,
            verbose: value.verbose,
            cargo: value.cargo,
            prefix_dir: None,
        }
    }
}

impl From<SudoInstall> for Package {
    fn from(value: SudoInstall) -> Self {
        Self {
            package: value.package,
            manifest_path: value.manifest_path,
            debug: !value.release,
            profile: value.profile,
            test: value.test,
            pg_config: value.pg_config,
            out_dir: value.out_dir,
            features: value.features,
            verbose: value.verbose,
            target: value.target,
            assemble_unreleased: !value.no_assemble_unreleased,
            no_assemble_unreleased: value.no_assemble_unreleased,
            cargo: value.cargo,
            prefix_dir: value.prefix_dir,
            // It stages files to copy into place and never builds RPM or DEB packages.
            no_distrib_packages: true,
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
        let outdir = outdir.canonicalize()?;
        let mut installed = Vec::new();
        for src in output_files {
            let src = src.canonicalize()?;
            let dest_abs = make_absolute(src.strip_prefix(&outdir)?);
            let dest = dest_abs.canonicalize().unwrap_or(dest_abs);

            if !cfg!(target_os = "windows") {
                println!(
                    "{} sudo cp {} {}",
                    "       Running".bold().green(),
                    src.display(),
                    dest.display()
                );

                // we're about to run `sudo` to copy some files, one at a time
                let mut command = Command::new("sudo"); // NB:  If we ever support Windows...
                command.arg("cp").arg(&src).arg(&dest);

                let mut child = command.spawn()?;

                let status = child.wait()?;
                if !status.success() {
                    // sudo failed.  let the user know and get out now
                    eprintln!("sudo command failed with status `{}`", format!("{status:?}").red());
                    std::process::exit(status.code().unwrap_or(1));
                }
            } else {
                println!(
                    "{} cp {} {}",
                    "       Running".bold().green(),
                    src.display(),
                    dest.display()
                );

                std::fs::copy(src, &dest)?;
            }
            installed.push(dest);
        }

        remove_stale_ephemeral_files(&installed)
    }
}

/// Removes upgrade scripts that an earlier install assembled from fragments and this one didn't
/// write again. The package directory was cleaned when it was built, but the real extension
/// directory still holds whatever previous `--sudo` installs put there.
fn remove_stale_ephemeral_files(installed: &[PathBuf]) -> eyre::Result<()> {
    let Some(control) = installed.iter().find(|path| path.extension() == Some("control".as_ref()))
    else {
        return Ok(());
    };
    let (Some(extdir), Some(extname)) = (control.parent(), control.file_stem()) else {
        return Ok(());
    };

    let extname = extname.to_string_lossy();
    for stale in crate::command::migrate::ephemeral_files(&extname, extdir)? {
        if installed.contains(&stale) {
            continue;
        }
        if cfg!(target_os = "windows") {
            println!("{} {}", "      Removing".bold().green(), stale.display());
            std::fs::remove_file(&stale)?;
            continue;
        }

        println!("{} sudo rm {}", "       Running".bold().green(), stale.display());
        let status = Command::new("sudo").arg("rm").arg("-f").arg(&stale).status()?;
        if !status.success() {
            eyre::bail!("`sudo rm {}` failed with status {status:?}", stale.display());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::migrate::UnreleasedFragmentMode;

    fn package_mode(no_assemble_unreleased: bool) -> UnreleasedFragmentMode {
        let install = SudoInstall {
            package: None,
            manifest_path: None,
            release: false,
            profile: None,
            test: false,
            pg_config: None,
            out_dir: None,
            features: Default::default(),
            target: None,
            no_assemble_unreleased,
            verbose: 0,
            cargo: Vec::new(),
            prefix_dir: None,
        };
        Package::from(install).unreleased_mode()
    }

    #[test]
    fn fragments_follow_the_install_flag() {
        assert_eq!(package_mode(false), UnreleasedFragmentMode::Assemble);
        assert_eq!(package_mode(true), UnreleasedFragmentMode::Skip);
    }
}
