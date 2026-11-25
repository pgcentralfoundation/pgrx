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
use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{env, process};

use eyre::WrapErr;

/// Configuration for building a cargo execution
#[derive(Default, Clone, Debug)]
pub struct Cargo {
    subcmd: String,
    features: clap_cargo::Features,
    stdio: [Stdio; 3],
    manifest: Option<PathBuf>,
    package: String,
    log_level: Option<String>,
    profile: CargoProfile,
    // use a BTreeMap for deterministic order of iteration
    more_args: BTreeMap<String, Vec<String>>,
    // Extra arguments passed after `cargo rustc --`.
    rustc_args: Vec<String>,
    // `--cargo` passthrough flags, forwarded verbatim (after whitespace-splitting).
    cargo_flags: Vec<String>,
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

    pub fn features(mut self, features: clap_cargo::Features) -> Self {
        self.features = features;
        self
    }

    /// `--cargo` passthrough flags, forwarded to this cargo invocation.
    pub fn cargo_flags(mut self, flags: &[String]) -> Self {
        self.cargo_flags = crate::metadata::split_cargo_flags(flags);
        self
    }

    pub fn rustc_args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.rustc_args.extend(args.into_iter().map(Into::into));
        self
    }

    #[track_caller]
    pub fn into_command(self) -> process::Command {
        if !self.rustc_args.is_empty() && self.subcmd != "rustc" {
            panic!("`Cargo::rustc_args` requires the `rustc` subcommand, was: {self:?}");
        }

        let mut cmd = cargo();

        // subcommand *must* go first among subcommand-relative args
        if !self.subcmd.is_empty() {
            cmd.arg(&self.subcmd);
        } else {
            panic!("`Cargo::into_command` requires a subcommand to be set, was: {self:?}")
        }

        let Self {
            features,
            stdio,
            manifest,
            log_level,
            profile,
            package,
            subcmd: _,
            more_args,
            rustc_args,
            cargo_flags,
        } = self;

        // set most-interesting flags first, like profile and manifest-path
        // so that when we read dumped command lines we can see that info first
        cmd.args(profile.cargo_args());

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

        // And now the miscellaneous build flags!, `PGRX_BUILD_FLAGS` stays build-only.
        let flags = env::var("PGRX_BUILD_FLAGS").unwrap_or_default();
        for arg in flags.split_ascii_whitespace() {
            cmd.arg(arg);
        }

        // `--cargo` passthrough: forward each (already whitespace-split) token verbatim.
        for arg in cargo_flags {
            cmd.arg(arg);
        }

        // set envs
        if let Some(log_level) = log_level {
            cmd.env("RUST_LOG", log_level);
        }

        for (flag, args) in more_args {
            cmd.arg(flag).args(args);
        }

        if !rustc_args.is_empty() {
            cmd.arg("--").args(rustc_args);
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
            Self::Inherit => Some(process::Stdio::inherit()),
            Self::Null => Some(process::Stdio::null()),
            Self::Default => None,
        }
    }
}

pub(crate) fn cargo() -> std::process::Command {
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let mut command = std::process::Command::new(cargo);
    // `cargo pgrx` often runs under `cargo run` or a test binary.
    pgrx_pg_config::cargo::remove_package_env(&mut command);
    command
}

/// Keep the schema records in ELF cdylibs without retaining unrelated code.
/// `#[used]` preserves the records through compilation, but not linker GC.
/// Only final extension links receive this script; Cargo's configured rustflags
/// and test executable links are left alone. The existing opt-out also disables
/// this narrower retention rule. cargo-pgrx cannot tell which linker rustc will
/// run, and mold and gold fail on the script, so their users need that opt-out.
pub(crate) fn pgrx_cdylib_rustc_args(
    target: Option<&str>,
    target_dir: &Path,
) -> eyre::Result<Vec<String>> {
    if std::env::var_os("CARGO_PGRX_DISABLE_GC_SECTIONS_WORKAROUND").is_some()
        || !target_uses_gnuish_ld(target)
    {
        return Ok(Vec::new());
    }

    const SCRIPT: &str = include_str!("pgrx-schema.ld");
    let mut hash = DefaultHasher::new();
    SCRIPT.hash(&mut hash);
    let directory = std::path::absolute(target_dir)?.join(".pgrx");
    let path = directory.join(format!("schema-{:016x}.ld", hash.finish()));

    // Stable arguments let Cargo reuse the build, so a script already in place is left alone
    // and a new script gets a new path. A file with other contents is replaced instead of
    // reported: a system crash can leave the name without its data, and every later build
    // would otherwise fail until someone deleted the file.
    if std::fs::read(&path).ok().as_deref() != Some(SCRIPT.as_bytes()) {
        write_by_rename(&directory, &path, SCRIPT.as_bytes())
            .wrap_err_with(|| format!("couldn't write {}", path.display()))?;
    }
    let path = path.to_str().ok_or_else(|| eyre::eyre!("non-UTF-8 linker script path"))?;

    // -Wl splits on commas, including any commas in the target directory.
    Ok(vec![
        "-C".into(),
        "link-arg=-Xlinker".into(),
        "-C".into(),
        format!("link-arg=--script={path}"),
    ])
}

/// Write `contents` to `path` in `directory` by renaming a complete file into place, so a
/// concurrent build reads either the whole file or none of it. Renaming replaces a file
/// that is already there, which needs neither hard links nor `RENAME_NOREPLACE` from the
/// filesystem.
fn write_by_rename(directory: &Path, path: &Path, contents: &[u8]) -> std::io::Result<()> {
    std::fs::create_dir_all(directory)?;
    let mut file = tempfile::NamedTempFile::new_in(directory)?;
    file.write_all(contents)?;
    file.persist(path)?;
    Ok(())
}

fn target_uses_gnuish_ld(target: Option<&str>) -> bool {
    let Some(target) = target else {
        return cfg!(all(target_family = "unix", not(target_os = "macos")));
    };

    target.split('-').any(|part| {
        matches!(
            part,
            "android"
                | "dragonfly"
                | "freebsd"
                | "hurd"
                | "illumos"
                | "linux"
                | "netbsd"
                | "openbsd"
                | "solaris"
        )
    })
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
    pub fn from_flags(profile: Option<&str>, default: Self) -> eyre::Result<Self> {
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

#[cfg(test)]
mod tests {
    use super::{Cargo, pgrx_cdylib_rustc_args};

    #[test]
    fn cdylib_rustc_args_reuse_schema_linker_script() {
        if std::env::var_os("CARGO_PGRX_DISABLE_GC_SECTIONS_WORKAROUND").is_some() {
            return;
        }

        let directory = tempfile::tempdir().unwrap();
        let target_dir = directory.path().join("target with spaces,and commas");
        let args = pgrx_cdylib_rustc_args(Some("x86_64-unknown-linux-gnu"), &target_dir).unwrap();
        assert_eq!(&args[..3], ["-C", "link-arg=-Xlinker", "-C"]);
        let path = args[3].strip_prefix("link-arg=--script=").unwrap();
        assert_eq!(std::fs::read_to_string(path).unwrap(), include_str!("pgrx-schema.ld"));
        let file = std::fs::File::options().write(true).open(path).unwrap();
        file.set_modified(std::time::UNIX_EPOCH).unwrap();
        drop(file);
        assert_eq!(
            pgrx_cdylib_rustc_args(Some("x86_64-unknown-linux-gnu"), &target_dir).unwrap(),
            args
        );
        // Stat the path: a handle opened earlier would not see a file renamed over it.
        assert_eq!(std::fs::metadata(path).unwrap().modified().unwrap(), std::time::UNIX_EPOCH);
    }

    #[test]
    fn cdylib_rustc_args_replace_a_damaged_schema_linker_script() {
        if std::env::var_os("CARGO_PGRX_DISABLE_GC_SECTIONS_WORKAROUND").is_some() {
            return;
        }

        let directory = tempfile::tempdir().unwrap();
        let args =
            pgrx_cdylib_rustc_args(Some("x86_64-unknown-linux-gnu"), directory.path()).unwrap();
        let path = args[3].strip_prefix("link-arg=--script=").unwrap();
        // A crash soon after the first build can leave the name without its contents.
        std::fs::write(path, "").unwrap();
        assert_eq!(
            pgrx_cdylib_rustc_args(Some("x86_64-unknown-linux-gnu"), directory.path()).unwrap(),
            args
        );
        assert_eq!(std::fs::read_to_string(path).unwrap(), include_str!("pgrx-schema.ld"));
    }

    #[test]
    fn cdylib_rustc_args_skip_non_gnuish_targets() {
        let directory = tempfile::tempdir().unwrap();
        let target_dir = directory.path().join("unused");
        for target in ["aarch64-apple-darwin", "x86_64-pc-windows-msvc"] {
            assert!(pgrx_cdylib_rustc_args(Some(target), &target_dir).unwrap().is_empty());
        }
        assert!(!target_dir.exists());
    }

    #[test]
    fn cargo_rustc_args_are_not_cargo_config() {
        if std::env::var_os("CARGO_PGRX_DISABLE_GC_SECTIONS_WORKAROUND").is_some() {
            return;
        }

        let directory = tempfile::tempdir().unwrap();
        let rustc_args =
            pgrx_cdylib_rustc_args(Some("x86_64-unknown-linux-gnu"), directory.path()).unwrap();
        let command = Cargo::default()
            .subcommand("rustc")
            .flag("--lib")
            .rustc_args(rustc_args.clone())
            .into_command();
        let args = command.get_args().map(|arg| arg.to_string_lossy()).collect::<Vec<_>>();

        assert_eq!(args.first().map(|arg| arg.as_ref()), Some("rustc"));
        assert!(
            !args.iter().any(|arg| arg.as_ref() == "--config"),
            "the cdylib workaround must not override Cargo config rustflags: {args:?}"
        );
        let separator = args.iter().position(|arg| arg.as_ref() == "--").unwrap();
        assert_eq!(&args[separator + 1..], rustc_args.as_slice());
    }

    #[test]
    fn cargo_flags_are_forwarded_and_whitespace_split() {
        // `cargo pgrx schema` (and other builds) route `--cargo` through the
        // `Cargo` builder; both the single-token and quoted-string forms must
        // reach the spawned command as separate argv tokens.
        let command = Cargo::default()
            .subcommand("rustc")
            .cargo_flags(&["--offline".into(), "--config foo".into()])
            .into_command();
        let args =
            command.get_args().map(|arg| arg.to_string_lossy().into_owned()).collect::<Vec<_>>();

        assert!(args.contains(&"--offline".to_string()), "missing --offline: {args:?}");
        assert!(args.contains(&"--config".to_string()), "quoted form not split: {args:?}");
        assert!(args.contains(&"foo".to_string()), "quoted form not split: {args:?}");
    }
}
