//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use std::path::Path;
use std::process::Command;

use eyre::eyre;
use serde::Deserialize;

/// The name of the library target in the `Cargo.toml` at `manifest_path`: its `[lib] name`, or
/// else the package name with each hyphen replaced by an underscore, as Cargo names it.
/// <https://doc.rust-lang.org/cargo/reference/cargo-targets.html#the-name-field>
pub fn lib_name(manifest_path: impl AsRef<Path>) -> eyre::Result<String> {
    #[derive(Deserialize)]
    struct Manifest {
        package: Option<Package>,
        lib: Option<Lib>,
    }
    #[derive(Deserialize)]
    struct Package {
        name: String,
    }
    #[derive(Deserialize)]
    struct Lib {
        name: Option<String>,
    }

    let path = manifest_path.as_ref();
    let manifest = std::fs::read_to_string(path)
        .map_err(|err| eyre!("Couldn't read manifest `{}`: {err}", path.display()))?;
    let manifest: Manifest =
        toml::from_str(&manifest).map_err(|err| eyre!("Couldn't parse manifest: {err}"))?;
    let package =
        manifest.package.ok_or_else(|| eyre!("Could not get [package] from manifest."))?;
    Ok(manifest.lib.and_then(|lib| lib.name).unwrap_or_else(|| package.name.replace('-', "_")))
}

/// Keep a Cargo command from inheriting the variables that `cargo run` and `cargo test` set to
/// describe the running package, such as `CARGO_MANIFEST_DIR`, `CARGO_PKG_NAME` and `OUT_DIR`.
///
/// Cargo checks a build script's `rerun-if-env-changed` variables against its own environment.
/// Started from a test binary, a nested Cargo would see that binary's package variables, which
/// differ for every test binary, so a build script that tracks one (`ring`'s does) reruns and
/// its crate and every crate above it rebuild on each call. Cargo sets these variables itself
/// for each crate it builds, so the nested build never needs the inherited values.
pub fn remove_package_env(command: &mut Command) {
    for (name, _) in std::env::vars_os() {
        if name.to_str().is_some_and(is_package_env) {
            command.env_remove(name);
        }
    }
}

/// Whether Cargo sets this variable to describe one package or binary rather than to configure
/// Cargo itself.
fn is_package_env(name: &str) -> bool {
    matches!(
        name,
        "CARGO_MANIFEST_DIR"
            | "CARGO_MANIFEST_PATH"
            | "CARGO_MANIFEST_LINKS"
            | "CARGO_CRATE_NAME"
            | "CARGO_BIN_NAME"
            | "CARGO_PRIMARY_PACKAGE"
            | "CARGO_TARGET_TMPDIR"
            | "CARGO_RUSTC_CURRENT_DIR"
            | "OUT_DIR"
    ) || name.starts_with("CARGO_PKG_")
        || name.starts_with("CARGO_BIN_EXE_")
}

#[cfg(test)]
mod tests {
    use super::{is_package_env, lib_name};

    #[test]
    fn lib_name_prefers_the_lib_section_over_the_package_name() {
        let dir = std::env::temp_dir().join(format!("pgrx-lib-name-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let manifest = dir.join("Cargo.toml");

        std::fs::write(&manifest, "[package]\nname = \"my-ext\"\nversion = \"0.1.0\"\n").unwrap();
        assert_eq!(lib_name(&manifest).unwrap(), "my_ext");

        std::fs::write(&manifest, "[package]\nname = \"my-ext\"\n\n[lib]\nname = \"other\"\n")
            .unwrap();
        assert_eq!(lib_name(&manifest).unwrap(), "other");

        std::fs::write(&manifest, "[workspace]\nmembers = []\n").unwrap();
        assert!(lib_name(&manifest).is_err());
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn package_env_excludes_cargo_configuration() {
        for name in [
            "CARGO_MANIFEST_DIR",
            "CARGO_PKG_NAME",
            "CARGO_PKG_VERSION_MAJOR",
            "CARGO_CRATE_NAME",
            "CARGO_BIN_EXE_cargo-pgrx",
            "CARGO_TARGET_TMPDIR",
            "OUT_DIR",
        ] {
            assert!(is_package_env(name), "{name}");
        }
        // Configuration that a nested Cargo must keep.
        for name in [
            "CARGO",
            "CARGO_HOME",
            "CARGO_TARGET_DIR",
            "CARGO_INCREMENTAL",
            "CARGO_PROFILE_DEV_DEBUG",
            "CARGO_BUILD_TARGET",
            "CARGO_TERM_COLOR",
            "CARGO_NET_OFFLINE",
        ] {
            assert!(!is_package_env(name), "{name}");
        }
    }
}
