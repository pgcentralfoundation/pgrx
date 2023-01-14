use clap::Args;
use std::path::PathBuf;

#[derive(Args, Clone, Default, Debug)]
pub(crate) struct CrossBuildArgs {
    /// Cross-compilation target - passing this option will make cargo-pgx assume we are cross-compiling
    #[clap(long)]
    pub(crate) target: Option<String>,
    /// Cross-compilation sysroot
    #[clap(long)]
    pub(crate) sysroot: Option<PathBuf>,
    /// Host sysroot
    #[clap(long)]
    pub(crate) host_sysroot: Option<PathBuf>,
    /// Host pg_config
    #[clap(long)]
    pub(crate) host_pg_config: Option<PathBuf>,
}

impl CrossBuildArgs {
    pub(crate) fn is_cross_compiling(&self) -> bool {
        self.target.is_some()
    }

    pub(crate) fn to_build(&self) -> CrossBuild {
        if self.is_cross_compiling() {
            CrossBuild::Target {
                target: self.target.clone().unwrap(),
                sysroot: self.sysroot.clone(),
            }
        } else {
            CrossBuild::None
        }
    }

    pub(crate) fn to_host_build(&self) -> CrossBuild {
        if self.is_cross_compiling() {
            CrossBuild::Host {
                sysroot: self.host_sysroot.clone(),
                pg_config: self.host_pg_config.clone(),
            }
        } else {
            CrossBuild::None
        }
    }
}

#[derive(Debug)]
pub(crate) enum CrossBuild {
    None,
    Target { target: String, sysroot: Option<PathBuf> },
    Host { sysroot: Option<PathBuf>, pg_config: Option<PathBuf> },
}
