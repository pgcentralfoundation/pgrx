//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::{env, process};

/// Configuration for building a cargo execution
#[derive(Default, Clone, Debug)]
pub struct Cargo {
    subcmd: String,
    features: clap_cargo::Features,
    stdio: [Stdio; 3],
    manifest: Option<PathBuf>,
    package: String,
    log_level: Option<String>,
    target: Option<String>,
    profile: CargoProfile,
    // use a BTreeMap for deterministic order of iteration
    more_args: BTreeMap<String, Vec<String>>,
}

impl Cargo {
    pub fn subcommand(mut self, s: &str) -> Self {
        self.subcmd.clear();
        self.subcmd.push_str(s);
        self
    }

    pub fn std_streams(mut self, streams: [Stdio; 3]) -> Self {
        self.stdio = streams;
        self
    }

    pub fn manifest_path(mut self, path: Option<PathBuf>) -> Self {
        self.manifest = path;
        self
    }

    pub fn package(mut self, package: String) -> Self {
        self.package = package;
        self
    }

    pub fn target(mut self, target: Option<String>) -> Self {
        self.target = target;
        self
    }

    pub fn profile(mut self, profile: CargoProfile) -> Self {
        self.profile = profile;
        self
    }

    pub fn log_level(mut self, level: Option<String>) -> Self {
        self.log_level = level;
        self
    }

    pub fn flag(mut self, flag: impl Into<String>) -> Self {
        self.more_args.insert(flag.into(), Vec::new());
        self
    }

    pub fn flag_args(mut self, flag: impl Into<String>, args: Vec<String>) -> Self {
        self.more_args.insert(flag.into(), args);
        self
    }

    pub fn features(mut self, features: clap_cargo::Features) -> Self {
        self.features = features;
        self
    }

    #[track_caller]
    pub fn into_command(self) -> process::Command {
        let mut cmd = cargo();

        // subcommand *must* go first
        if self.subcmd != "" {
            cmd.arg(&self.subcmd);
        } else {
            panic!("`Cargo::into_command` requires a subcommand to be set, was: {self:?}")
        }

        let Cargo {
            features,
            stdio,
            manifest,
            log_level,
            target,
            profile,
            package,
            subcmd: _,
            more_args,
        } = self;

        // set most-interesting flags first, like profile, target, and manifest-path
        // so that when we read dumped command lines we can see that info first
        cmd.args(profile.cargo_args());

        if let Some(target) = target {
            cmd.arg("--target").arg(target);
        }
        if let Some(manifest) = manifest {
            cmd.arg("--manifest-path").arg(manifest);
        }
        if !package.is_empty() {
            cmd.arg("--package").arg(package);
        }

        // set std streams
        let [stdin, stdout, stderr] = stdio;
        if let Some(stdio) = stdin.into_stdio() {
            cmd.stdin(stdio);
        }
        if let Some(stdio) = stdout.into_stdio() {
            cmd.stdout(stdio);
        }
        if let Some(stdio) = stderr.into_stdio() {
            cmd.stderr(stdio);
        }

        // set features
        if features.no_default_features {
            cmd.arg("--no-default-features");
        }

        if features.all_features {
            cmd.arg("--all-features");
        }

        if !features.features.is_empty() {
            cmd.arg("--features");
            cmd.arg(features.features.join(" "));
        }

        // And now the miscellaneous build flags!
        let flags = env::var("PGRX_BUILD_FLAGS").unwrap_or_default();
        for arg in flags.split_ascii_whitespace() {
            cmd.arg(arg);
        }

        // set envs
        if let Some(log_level) = log_level {
            cmd.env("RUST_LOG", log_level);
        }

        for (flag, args) in more_args {
            cmd.arg(flag).args(args);
        }

        cmd
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Stdio {
    Inherit,
    Null,
    #[default]
    Default,
}

impl Stdio {
    fn into_stdio(self) -> Option<process::Stdio> {
        match self {
            Stdio::Inherit => Some(process::Stdio::inherit()),
            Stdio::Null => Some(process::Stdio::null()),
            Stdio::Default => None,
        }
    }
}

pub(crate) fn cargo() -> std::process::Command {
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    std::process::Command::new(cargo)
}

/// Set some environment variables for use downstream (in `pgrx-test` for
/// example). Does nothing if already set.
pub(crate) fn initialize() {
    match (std::env::var_os("CARGO_PGRX"), std::env::current_exe()) {
        (None, Ok(path)) => {
            unsafe {
                std::env::set_var("CARGO_PGRX", path);
            }
            // TODO: Should we set `CARGO_PGRX_{CARGO,RUSTC}` to `RUSTC`/`CARGO`
            // if unset, then prefer those? The issue with `RUSTC`/`CARGO` vars
            // is that they are unset if something invokes e.g. `cargo`
            // directly... This is probably eventually something we'll need, but
            // let's wait until that happens.
        }
        (Some(_), Ok(_)) => {
            // For now I guess we should just hope they're the same.
            // Canonicalizing here's tricky and not guaranteed to behave
            // right... although we could consider calling back into ourselves
            // so something that blindly invokes `cargo-pgrx` instead of
            // `CARGO_PGRX` will do the right thing.
            //
            // In either case if we ever get to the macos-linker-shim work this
            // will have to be slightly firmed up (if `cargo-pgrx` is still going
            // to act as the linker shim.)
        }
        //  bad but not much we can do.
        (_, Err(_)) => {}
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum CargoProfile {
    /// The default non-release profile, `[profile.dev]`
    #[default]
    Dev,
    /// The default release profile, `[profile.release]`
    Release,
    /// Some other profile, specified by name.
    Profile(String),
}

impl CargoProfile {
    pub fn from_flags(profile: Option<&str>, default: CargoProfile) -> eyre::Result<Self> {
        match profile {
            // Cargo treats `--profile release` the same as `--release`.
            Some("release") => Ok(Self::Release),
            // Cargo has two names for the debug profile, due to legacy
            // reasons...
            Some("debug") | Some("dev") => Ok(Self::Dev),
            Some(profile) => Ok(Self::Profile(profile.into())),
            None => Ok(default),
        }
    }

    pub fn cargo_args(&self) -> Vec<String> {
        match self {
            Self::Dev => vec![],
            Self::Release => vec!["--release".into()],
            Self::Profile(p) => vec!["--profile".into(), p.into()],
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Dev => "dev",
            Self::Release => "release",
            Self::Profile(p) => p,
        }
    }

    pub fn target_subdir(&self) -> &str {
        match self {
            Self::Dev => "debug",
            Self::Release => "release",
            Self::Profile(p) => p,
        }
    }
}
