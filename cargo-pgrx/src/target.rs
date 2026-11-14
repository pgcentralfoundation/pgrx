//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TargetOs {
    Darwin,
    Windows,
    Linux,
    Elf,
}

impl TargetOs {
    /// Read the platform out of a `--target` tuple, falling back to the host
    /// when no target was passed. Cross builds must pass cargo-pgrx's `--target`
    /// explicitly rather than relying on Cargo's target configuration.
    pub(crate) fn from_target(target: Option<&str>) -> Self {
        match target {
            Some(tuple) => Self::from_tuple(tuple),
            // The host is the one case where the build-time `cfg` is the answer.
            None if cfg!(target_vendor = "apple") => Self::Darwin,
            None if cfg!(target_os = "windows") => Self::Windows,
            None if cfg!(target_os = "linux") => Self::Linux,
            None => Self::Elf,
        }
    }

    /// A target tuple is `<arch>-<vendor>-<os>` with an optional `-<env>`, so
    /// matching the delimited vendor and os components avoids mistaking an
    /// architecture or environment that merely contains the same letters.
    fn from_tuple(tuple: &str) -> Self {
        let mut components = tuple.split('-');
        let _arch = components.next();
        match (components.next(), components.next()) {
            (Some("apple"), _) => Self::Darwin,
            (_, Some("windows")) => Self::Windows,
            (_, Some("linux")) => Self::Linux,
            _ => Self::Elf,
        }
    }

    /// The `cdylib` name prefix, as `std::env::consts::DLL_PREFIX` gives for the host.
    pub(crate) fn dll_prefix(self) -> &'static str {
        match self {
            Self::Darwin | Self::Linux | Self::Elf => "lib",
            Self::Windows => "",
        }
    }

    /// The `cdylib` extension without a leading dot, as `DLL_EXTENSION` gives.
    pub(crate) fn dll_extension(self) -> &'static str {
        match self {
            Self::Darwin => "dylib",
            Self::Windows => "dll",
            Self::Linux | Self::Elf => "so",
        }
    }

    /// The `cdylib` extension with its leading dot, as `DLL_SUFFIX` gives.
    pub(crate) fn dll_suffix(self) -> &'static str {
        match self {
            Self::Darwin => ".dylib",
            Self::Windows => ".dll",
            Self::Linux | Self::Elf => ".so",
        }
    }
}

pub(crate) fn target_arch(target: Option<&str>) -> &str {
    target.and_then(|tuple| tuple.split('-').next()).unwrap_or(std::env::consts::ARCH)
}

#[cfg(test)]
mod tests {
    use super::TargetOs;

    #[test]
    fn reads_the_os_out_of_target_tuples() {
        for tuple in ["x86_64-apple-darwin", "aarch64-apple-darwin"] {
            assert_eq!(TargetOs::from_target(Some(tuple)), TargetOs::Darwin, "{tuple}");
        }
        for tuple in ["x86_64-pc-windows-msvc", "x86_64-pc-windows-gnu"] {
            assert_eq!(TargetOs::from_target(Some(tuple)), TargetOs::Windows, "{tuple}");
        }
        for tuple in
            ["x86_64-unknown-linux-gnu", "aarch64-unknown-linux-gnu", "x86_64-unknown-linux-musl"]
        {
            assert_eq!(TargetOs::from_target(Some(tuple)), TargetOs::Linux, "{tuple}");
        }
        assert_eq!(TargetOs::from_target(Some("x86_64-unknown-freebsd")), TargetOs::Elf);
    }

    #[test]
    fn names_a_cdylib_per_platform() {
        assert_eq!(
            (TargetOs::Darwin.dll_prefix(), TargetOs::Darwin.dll_suffix()),
            ("lib", ".dylib")
        );
        assert_eq!((TargetOs::Windows.dll_prefix(), TargetOs::Windows.dll_suffix()), ("", ".dll"));
        assert_eq!((TargetOs::Elf.dll_prefix(), TargetOs::Elf.dll_suffix()), ("lib", ".so"));
        assert_eq!((TargetOs::Linux.dll_prefix(), TargetOs::Linux.dll_suffix()), ("lib", ".so"));
    }

    #[test]
    fn no_target_agrees_with_the_host_constants() {
        // whatever machine this runs on, the fallback must match what std says
        let host = TargetOs::from_target(None);
        assert_eq!(host.dll_prefix(), std::env::consts::DLL_PREFIX);
        assert_eq!(host.dll_suffix(), std::env::consts::DLL_SUFFIX);
        assert_eq!(host.dll_extension(), std::env::consts::DLL_EXTENSION);
    }
}
