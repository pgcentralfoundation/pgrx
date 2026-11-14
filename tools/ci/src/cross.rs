//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Package an example extension for another platform from a target bundle, and check the
//! package without running any of it.
//!
//! The bundle comes from a job on the target platform, `cargo pgrx package --bundle-generate`.
//! This job has no PostgreSQL, no server headers and no Clang: the bundle and a linker for
//! the target are all it is given.

use std::path::{Path, PathBuf};
use std::process::Command;

use eyre::{WrapErr, bail, ensure, eyre};

use crate::process;
use crate::workspace::Workspace;

/// The example to package: its library is named for its version, so the SQL has to name it.
const EXTENSION: &str = "versioned_so";
const VERSION: &str = "0.0.0";

/// Build this checkout's cargo-pgrx, package [`EXTENSION`] from `bundle`, and report whether
/// the package is what the bundle's platform expects.
pub(crate) fn run(bundle: &Path) -> eyre::Result<bool> {
    let name = bundle
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| eyre!("{} is not a bundle file", bundle.display()))?;
    let (major, target) = bundle_domain(name)
        .ok_or_else(|| eyre!("{name} is not named pgrx-pg<major>-<target>.tar.gz"))?;
    let machine = elf_machine(target)
        .ok_or_else(|| eyre!("{target} is not a Linux target this check knows how to read"))?;

    let workspace = Workspace::load()?;
    let target_dir = workspace.target_dir("target");
    if !process::run(Command::new("cargo").args(["build", "--locked", "--package", "cargo-pgrx"]))?
    {
        return Ok(false);
    }

    // No pgrx configuration exists for packaging to fall back on.
    let scratch = target_dir.join("cross-package");
    if scratch.exists() {
        std::fs::remove_dir_all(&scratch).wrap_err_with(|| format!("couldn't remove {scratch}"))?;
    }
    let (home, package) = (scratch.join("pgrx-home"), scratch.join("package"));
    std::fs::create_dir_all(&home).wrap_err_with(|| format!("couldn't create {home}"))?;
    let packaged = process::run(
        Command::new(target_dir.join("debug/cargo-pgrx"))
            .args(["pgrx", "package", "--package", EXTENSION, "--debug", "--no-distrib-packages"])
            .arg("--bundle")
            .arg(bundle)
            .arg("--out-dir")
            .arg(&package)
            .arg("--cargo=--locked")
            .env("PGRX_HOME", &home),
    )?;
    if !packaged {
        return Ok(false);
    }

    check_package(package.as_std_path(), major, machine)?;
    process::log(format!("{EXTENSION} is packaged for PostgreSQL {major} on {target}"));
    Ok(true)
}

/// The PostgreSQL major version and Rust target in a bundle's file name.
fn bundle_domain(name: &str) -> Option<(u16, &str)> {
    let (major, target) = name.strip_prefix("pgrx-pg")?.strip_suffix(".tar.gz")?.split_once('-')?;
    Some((major.parse().ok()?, target))
}

/// The ELF `e_machine` of a Linux target's architecture.
fn elf_machine(target: &str) -> Option<u16> {
    let mut parts = target.split('-');
    let (arch, _vendor, os) = (parts.next()?, parts.next()?, parts.next()?);
    match (arch, os) {
        ("x86_64", "linux") => Some(62),
        ("aarch64", "linux") => Some(183),
        _ => None,
    }
}

/// The package must hold exactly the library, the control file and the SQL, with the SQL
/// naming the versioned library, and the library must be a shared object for `machine`.
fn check_package(package: &Path, major: u16, machine: u16) -> eyre::Result<()> {
    let mut files = Vec::new();
    collect_files(package, &mut files)?;
    let named = |name: &str| -> eyre::Result<&PathBuf> {
        files
            .iter()
            .find(|file| file.file_name().is_some_and(|file| file == name))
            .ok_or_else(|| eyre!("the package has no {name}: {files:?}"))
    };
    let library = named(&format!("{EXTENSION}-{VERSION}.so"))?;
    let control = named(&format!("{EXTENSION}.control"))?;
    let sql = named(&format!("{EXTENSION}--{VERSION}.sql"))?;
    ensure!(files.len() == 3, "the package has other files: {files:?}");

    // The layout is the generating PostgreSQL's, which names its major version.
    for file in [library, control, sql] {
        let path = file.strip_prefix(package).unwrap_or(file);
        ensure!(
            path.components().any(|part| part.as_os_str() == major.to_string().as_str()),
            "{} is not laid out for PostgreSQL {major}",
            path.display()
        );
    }
    let read = |file: &Path| {
        std::fs::read_to_string(file).wrap_err_with(|| format!("couldn't read {}", file.display()))
    };
    let default_version = format!("default_version = '{VERSION}'");
    ensure!(read(control)?.contains(&default_version), "the control file lacks {default_version}");
    let function = format!("AS '{EXTENSION}-{VERSION}', 'hello_{EXTENSION}_wrapper'");
    ensure!(read(sql)?.contains(&function), "the SQL does not name the packaged library");

    let header =
        std::fs::read(library).wrap_err_with(|| format!("couldn't read {}", library.display()))?;
    check_shared_object(&header, machine)
        .wrap_err_with(|| format!("{} is not the target's library", library.display()))
}

fn collect_files(directory: &Path, files: &mut Vec<PathBuf>) -> eyre::Result<()> {
    let entries = std::fs::read_dir(directory)
        .wrap_err_with(|| format!("couldn't read {}", directory.display()))?;
    for entry in entries {
        let path = entry?.path();
        if path.is_dir() {
            collect_files(&path, files)?;
        } else {
            files.push(path);
        }
    }
    Ok(())
}

/// Read the start of an ELF file: a little-endian 64-bit shared object for `machine`.
fn check_shared_object(elf: &[u8], machine: u16) -> eyre::Result<()> {
    const SHARED_OBJECT: u16 = 3;
    let Some(header) = elf.get(..20) else { bail!("it is too short to be an ELF file") };
    ensure!(header[..4] == [0x7f, b'E', b'L', b'F'], "it is not an ELF file");
    ensure!(header[4] == 2 && header[5] == 1, "it is not little-endian 64-bit ELF");
    let field = |at: usize| u16::from_le_bytes([header[at], header[at + 1]]);
    ensure!(field(16) == SHARED_OBJECT, "its ELF type is {}, not a shared object", field(16));
    ensure!(field(18) == machine, "its ELF machine is {}, not {machine}", field(18));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_bundle_name_gives_the_major_and_target() {
        assert_eq!(
            bundle_domain("pgrx-pg18-aarch64-unknown-linux-gnu.tar.gz"),
            Some((18, "aarch64-unknown-linux-gnu"))
        );
        for name in ["pgrx-pg18.tar.gz", "pgrx-pgXX-x86_64-unknown-linux-gnu.tar.gz", "bundle.tgz"]
        {
            assert_eq!(bundle_domain(name), None, "{name}");
        }
    }

    #[test]
    fn only_linux_targets_have_a_machine_to_check() {
        assert_eq!(elf_machine("aarch64-unknown-linux-gnu"), Some(183));
        assert_eq!(elf_machine("x86_64-unknown-linux-musl"), Some(62));
        assert_eq!(elf_machine("aarch64-apple-darwin"), None);
        assert_eq!(elf_machine("x86_64-pc-windows-msvc"), None);
    }

    fn elf(class: u8, kind: u16, machine: u16) -> Vec<u8> {
        let mut header = vec![0x7f, b'E', b'L', b'F', class, 1, 1, 0];
        header.resize(16, 0);
        header.extend(kind.to_le_bytes());
        header.extend(machine.to_le_bytes());
        header
    }

    #[test]
    fn a_shared_object_for_another_machine_is_refused() {
        check_shared_object(&elf(2, 3, 183), 183).unwrap();
        for (file, problem) in [
            (elf(2, 3, 62), "ELF machine is 62"),
            (elf(2, 2, 183), "ELF type is 2"),
            (elf(1, 3, 183), "64-bit"),
            (b"MZ".to_vec(), "too short"),
            (vec![0; 20], "not an ELF file"),
        ] {
            let error = check_shared_object(&file, 183).unwrap_err().to_string();
            assert!(error.contains(problem), "{problem}: {error}");
        }
    }

    #[test]
    fn a_package_must_be_exactly_the_three_files() {
        let directory = std::env::temp_dir().join(format!("pgrx-ci-cross-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&directory);
        let lib = directory.join("usr/lib/postgresql/18/lib");
        let ext = directory.join("usr/share/postgresql/18/extension");
        std::fs::create_dir_all(&lib).unwrap();
        std::fs::create_dir_all(&ext).unwrap();
        std::fs::write(lib.join("versioned_so-0.0.0.so"), elf(2, 3, 183)).unwrap();
        std::fs::write(ext.join("versioned_so.control"), "default_version = '0.0.0'\n").unwrap();
        let sql = ext.join("versioned_so--0.0.0.sql");
        std::fs::write(&sql, "AS 'versioned_so-0.0.0', 'hello_versioned_so_wrapper'").unwrap();
        check_package(&directory, 18, 183).unwrap();

        let error = check_package(&directory, 17, 183).unwrap_err().to_string();
        assert!(error.contains("not laid out for PostgreSQL 17"), "{error}");
        std::fs::write(&sql, "AS 'MODULE_PATHNAME', 'hello_versioned_so_wrapper'").unwrap();
        let error = check_package(&directory, 18, 183).unwrap_err().to_string();
        assert!(error.contains("does not name the packaged library"), "{error}");
        std::fs::write(ext.join("extra.sql"), "").unwrap();
        let error = check_package(&directory, 18, 183).unwrap_err().to_string();
        assert!(error.contains("other files"), "{error}");
        std::fs::remove_dir_all(&directory).unwrap();
    }
}
