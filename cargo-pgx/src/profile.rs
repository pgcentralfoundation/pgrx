/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

/// Represents a selected cargo profile
///
/// Generally chosen from flags like `--release`, `--profile <profile name>`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CargoProfile {
    /// The default non-release profile, `[profile.dev]`
    Dev,
    /// The default release profile, `[profile.release]`
    Release,
    /// Some other profile, specified by name.
    Profile(String),
}

impl CargoProfile {
    pub fn from_flags(release: bool, profile: Option<&str>) -> eyre::Result<Self> {
        match (profile, release) {
            (Some(profile), true) => {
                eyre::bail!("conflicting usage of --profile={:?} and --release", profile);
            }
            // Cargo treats `--profile release` the same as `--release`.
            (Some("release"), false) => Ok(Self::Release),
            // Cargo has two names for the debug profile, due to legacy
            // reasons...
            (Some("debug"), false) | (Some("dev"), false) => Ok(Self::Dev),
            (Some(profile), false) => Ok(Self::Profile(profile.into())),
            (None, true) => Ok(Self::Release),
            (None, false) => Ok(Self::Dev),
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
