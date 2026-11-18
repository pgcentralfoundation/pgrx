//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! Package two example extensions for another platform from a target bundle, and check the
//! packages without running any of their code.
//!
//! The bundle comes from the target platform: `cargo pgrx package --bundle-generate`, or
//! `tools/pgrx-collect-partial-bundle.sh` for a partial bundle. The packages must be laid out
//! for the directories the bundle's `pg_config` reported, and their libraries must be the
//! target's.

use std::collections::BTreeMap;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;

use eyre::{WrapErr, bail, ensure, eyre};

use crate::process;
use crate::workspace::Workspace;

/// An example extension, and what packaging it must produce.
pub(crate) struct Example {
    pub(crate) name: &'static str,
    version: &'static str,
    /// The library's file name without its suffix, and the name the SQL loads it by.
    library: &'static str,
    sql_library: &'static str,
    /// Whether its manifest has RPM and DEB sections, built for a Linux target.
    distributions: bool,
}

pub(crate) const EXAMPLES: [Example; 2] = [
    Example {
        name: "versioned_so",
        version: "0.0.0",
        library: "versioned_so-0.0.0",
        sql_library: "versioned_so-0.0.0",
        distributions: false,
    },
    Example {
        name: "packaging",
        version: "0.1.0",
        library: "packaging",
        sql_library: "MODULE_PATHNAME",
        distributions: true,
    },
];

/// Build this checkout's cargo-pgrx, package each example from `bundle`, and report whether
/// the packages are what the bundle's platform expects.
pub(crate) fn run(bundle: &Path) -> eyre::Result<bool> {
    let name = bundle
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| eyre!("{} is not a bundle file", bundle.display()))?;
    let (major, target) = bundle_domain(name)
        .ok_or_else(|| {
            eyre!("{name} is not named pgrx-<pgrx>-pg<major>-<target>.tar.gz or pgrx-pg<major>-<target>.partial.tar.gz")
        })?;
    let platform = Platform::of(target)
        .ok_or_else(|| eyre!("{target} is not a target this check knows how to read"))?;
    let layout = Layout::of(bundle)?;

    let workspace = Workspace::load()?;
    let target_dir = workspace.target_dir("target");
    if !process::run(Command::new("cargo").args(["build", "--locked", "--package", "cargo-pgrx"]))?
    {
        return Ok(false);
    }
    let cargo_pgrx = target_dir.join(format!("debug/cargo-pgrx{}", std::env::consts::EXE_SUFFIX));

    // No pgrx configuration exists for packaging to fall back on.
    let scratch = target_dir.join("cross-package");
    if scratch.exists() {
        std::fs::remove_dir_all(&scratch).wrap_err_with(|| format!("couldn't remove {scratch}"))?;
    }
    let home = scratch.join("pgrx-home");
    std::fs::create_dir_all(&home).wrap_err_with(|| format!("couldn't create {home}"))?;
    for example in &EXAMPLES {
        let package = scratch.join("packages").join(example.name);
        let mut command = Command::new(&cargo_pgrx);
        command
            .args(["pgrx", "package", "--package", example.name, "--debug"])
            .arg("--bundle")
            .arg(bundle)
            .arg("--out-dir")
            .arg(&package)
            .arg("--cargo=--locked")
            .env("PGRX_HOME", &home);
        if !process::run(&mut command)? {
            return Ok(false);
        }
        ensure!(!home.join("config.toml").exists(), "packaging wrote a pgrx configuration");
        check_package(package.as_std_path(), example, major, platform, &layout)
            .wrap_err_with(|| format!("{} is not packaged for {target}", example.name))?;
        process::log(format!("{} is packaged for PostgreSQL {major} on {target}", example.name));
    }
    Ok(true)
}

/// The PostgreSQL major version and Rust target in a bundle's file name. A full bundle's name
/// starts with the pgrx version that generated it, which can itself contain `-`.
fn bundle_domain(name: &str) -> Option<(u16, &str)> {
    let stem = name.strip_prefix("pgrx-")?.strip_suffix(".tar.gz")?;
    let stem = match stem.strip_suffix(".partial") {
        Some(stem) => stem,
        None => &stem[stem.find("-pg")? + 1..],
    };
    let (major, target) = stem.strip_prefix("pg")?.split_once('-')?;
    Some((major.parse().ok()?, target))
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Os {
    Linux,
    Macos,
    Windows,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Arch {
    X86_64,
    Aarch64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Platform {
    os: Os,
    arch: Arch,
}

impl Platform {
    fn of(target: &str) -> Option<Self> {
        let mut parts = target.split('-');
        let arch = match parts.next()? {
            "x86_64" => Arch::X86_64,
            "aarch64" => Arch::Aarch64,
            _ => return None,
        };
        let os = match (parts.next()?, parts.next()?) {
            (_, "linux") => Os::Linux,
            ("apple", "darwin") => Os::Macos,
            (_, "windows") => Os::Windows,
            _ => return None,
        };
        Some(Self { os, arch })
    }

    /// The installed library's suffix. PostgreSQL loaded `.so` files on macOS until 16.
    fn library_suffix(self, major: u16) -> &'static str {
        match self.os {
            Os::Linux => ".so",
            Os::Macos if major < 16 => ".so",
            Os::Macos => ".dylib",
            Os::Windows => ".dll",
        }
    }

    fn rpm_arch(self) -> &'static str {
        match self.arch {
            Arch::X86_64 => "x86_64",
            Arch::Aarch64 => "aarch64",
        }
    }

    fn deb_arch(self) -> &'static str {
        match self.arch {
            Arch::X86_64 => "amd64",
            Arch::Aarch64 => "arm64",
        }
    }
}

/// Where the bundle's PostgreSQL loads libraries and extension files from.
#[derive(Debug, PartialEq)]
struct Layout {
    pkglibdir: String,
    sharedir: String,
}

impl Layout {
    /// Read the layout out of a full bundle's `bundle.json`, or a partial bundle's
    /// `pg_config.txt`, where the last line for a name counts.
    fn of(bundle: &Path) -> eyre::Result<Self> {
        let file = std::fs::File::open(bundle)
            .wrap_err_with(|| format!("couldn't open {}", bundle.display()))?;
        let mut archive = tar::Archive::new(flate2::read::GzDecoder::new(file));
        for entry in archive.entries()? {
            let mut entry = entry?;
            let path = entry.path()?.into_owned();
            let mut text = String::new();
            if path == Path::new("bundle.json") {
                entry.read_to_string(&mut text)?;
                let description: serde_json::Value = serde_json::from_str(&text)?;
                let property = |name: &str| {
                    description["pg_config"][name]
                        .as_str()
                        .map(str::to_owned)
                        .ok_or_else(|| eyre!("bundle.json records no {name}"))
                };
                return Ok(Self {
                    pkglibdir: property("pkglibdir")?,
                    sharedir: property("sharedir")?,
                });
            }
            if path == Path::new("pg_config.txt") {
                entry.read_to_string(&mut text)?;
                return Self::reported(&text);
            }
        }
        bail!("{} has neither bundle.json nor pg_config.txt", bundle.display())
    }

    fn reported(text: &str) -> eyre::Result<Self> {
        let reported: BTreeMap<_, _> = text
            .lines()
            .filter_map(|line| line.split_once(" = "))
            .map(|(name, value)| (name.trim(), value.trim()))
            .collect();
        let property = |name: &str| {
            reported
                .get(name)
                .map(|value| (*value).to_owned())
                .ok_or_else(|| eyre!("pg_config.txt reports no {name}"))
        };
        Ok(Self { pkglibdir: property("PKGLIBDIR")?, sharedir: property("SHAREDIR")? })
    }

    /// The package's library and extension directories, relative to its root. Windows packages
    /// always use `lib` and `share/extension`.
    fn directories(&self, os: Os) -> (PathBuf, PathBuf) {
        if os == Os::Windows {
            return ("lib".into(), "share/extension".into());
        }
        let relative = |path: &str| PathBuf::from(path.trim_start_matches('/'));
        (relative(&self.pkglibdir), relative(&self.sharedir).join("extension"))
    }
}

/// The package must hold exactly the library, the control file, the SQL and, for a Linux
/// target and an example with the sections, an RPM and a DEB, and each must be the target's.
fn check_package(
    package: &Path,
    example: &Example,
    major: u16,
    platform: Platform,
    layout: &Layout,
) -> eyre::Result<()> {
    let Example { name, version, .. } = example;
    let (libdir, extdir) = layout.directories(platform.os);
    let library = libdir.join(format!("{}{}", example.library, platform.library_suffix(major)));
    let control = extdir.join(format!("{name}.control"));
    let sql = extdir.join(format!("{name}--{version}.sql"));
    let distributions = example.distributions && platform.os == Os::Linux;
    let rpm = PathBuf::from(format!("{name}_{major}-{version}-1.{}.rpm", platform.rpm_arch()));
    let deb =
        PathBuf::from(format!("postgresql-{major}-{name}_{version}_{}.deb", platform.deb_arch()));

    let mut expected = vec![library.clone(), control.clone(), sql.clone()];
    if distributions {
        expected.extend([rpm.clone(), deb.clone()]);
    }
    expected.sort();
    let mut files = Vec::new();
    collect_files(package, package, &mut files)?;
    files.sort();
    ensure!(files == expected, "it holds {files:?}, not {expected:?}");

    let read = |file: &Path| {
        std::fs::read(package.join(file))
            .wrap_err_with(|| format!("couldn't read {}", file.display()))
    };
    let text = |file: &Path| -> eyre::Result<String> { Ok(String::from_utf8(read(file)?)?) };
    let control_text = text(&control)?;
    let default_version = format!("default_version = '{version}'");
    ensure!(control_text.contains(&default_version), "the control file lacks {default_version}");
    let schema = text(&sql)?;
    let words = schema.split_whitespace().collect::<Vec<_>>().join(" ");
    let function = format!("CREATE FUNCTION \"hello_{name}\"()");
    ensure!(words.contains(&function), "the SQL does not create hello_{name}");
    let loads = format!("AS '{}', 'hello_{name}_wrapper'", example.sql_library);
    ensure!(words.contains(&loads), "the SQL does not load {}", example.sql_library);
    let library_bytes = read(&library)?;
    check_library(&library_bytes, platform)
        .wrap_err_with(|| format!("{} is not the target's library", library.display()))?;

    if distributions {
        let payload = BTreeMap::from([
            (format!("lib/{}.so", example.library), library_bytes),
            (format!("share/extension/{name}.control"), control_text.into_bytes()),
            (format!("share/extension/{name}--{version}.sql"), schema.into_bytes()),
        ]);
        check_rpm(&package.join(&rpm), name, version, major, platform, &payload)
            .wrap_err_with(|| format!("{} is wrong", rpm.display()))?;
        check_deb(&package.join(&deb), name, version, major, platform, &payload)
            .wrap_err_with(|| format!("{} is wrong", deb.display()))?;
    }
    Ok(())
}

/// Every file below `directory`, relative to `root`.
fn collect_files(root: &Path, directory: &Path, files: &mut Vec<PathBuf>) -> eyre::Result<()> {
    let entries = std::fs::read_dir(directory)
        .wrap_err_with(|| format!("couldn't read {}", directory.display()))?;
    for entry in entries {
        let path = entry?.path();
        if path.is_dir() {
            collect_files(root, &path, files)?;
        } else {
            files.push(path.strip_prefix(root)?.to_owned());
        }
    }
    Ok(())
}

/// Read a library's header: a shared library, not an executable, for the platform.
fn check_library(file: &[u8], platform: Platform) -> eyre::Result<()> {
    let u16_at = |at: usize| -> eyre::Result<u16> {
        let bytes = file.get(at..at + 2).ok_or_else(|| eyre!("it is truncated"))?;
        Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
    };
    let u32_at = |at: usize| -> eyre::Result<u32> {
        let bytes = file.get(at..at + 4).ok_or_else(|| eyre!("it is truncated"))?;
        Ok(u32::from_le_bytes(bytes.try_into()?))
    };
    let u64_at = |at: usize| -> eyre::Result<usize> {
        let bytes = file.get(at..at + 8).ok_or_else(|| eyre!("it is truncated"))?;
        Ok(u64::from_le_bytes(bytes.try_into()?).try_into()?)
    };
    match platform.os {
        Os::Linux => {
            const SHARED_OBJECT: u16 = 3;
            const PT_INTERP: u32 = 3;
            ensure!(file.starts_with(b"\x7fELF"), "it is not an ELF file");
            ensure!(file.get(4..6) == Some(&[2, 1]), "it is not little-endian 64-bit ELF");
            ensure!(
                u16_at(16)? == SHARED_OBJECT,
                "its ELF type is {}, not a shared object",
                u16_at(16)?
            );
            let machine = match platform.arch {
                Arch::X86_64 => 62,
                Arch::Aarch64 => 183,
            };
            ensure!(u16_at(18)? == machine, "its ELF machine is {}, not {machine}", u16_at(18)?);
            // A position-independent executable is also ET_DYN, but names an interpreter.
            let (offset, size, count) = (u64_at(32)?, usize::from(u16_at(54)?), u16_at(56)?);
            for header in 0..usize::from(count) {
                ensure!(
                    u32_at(offset + header * size)? != PT_INTERP,
                    "it names a program interpreter, so it is an executable"
                );
            }
        }
        Os::Macos => {
            const MH_MAGIC_64: u32 = 0xfeed_facf;
            const MH_DYLIB: u32 = 6;
            const LC_BUILD_VERSION: u32 = 0x32;
            const PLATFORM_MACOS: u32 = 1;
            ensure!(u32_at(0)? == MH_MAGIC_64, "it is not a 64-bit little-endian Mach-O file");
            let cpu = match platform.arch {
                Arch::X86_64 => 0x0100_0007,
                Arch::Aarch64 => 0x0100_000c,
            };
            ensure!(u32_at(4)? == cpu, "its Mach-O CPU type is {:#x}, not {cpu:#x}", u32_at(4)?);
            ensure!(
                u32_at(12)? == MH_DYLIB,
                "its Mach-O file type is {}, not a dylib",
                u32_at(12)?
            );
            let mut command = 32;
            let mut macos = false;
            for _ in 0..u32_at(16)? {
                if u32_at(command)? == LC_BUILD_VERSION {
                    macos |= u32_at(command + 8)? == PLATFORM_MACOS;
                }
                let size = usize::try_from(u32_at(command + 4)?)?;
                ensure!(size >= 8, "it has a Mach-O load command of {size} bytes");
                command += size;
            }
            ensure!(macos, "it does not say it is built for macOS");
        }
        Os::Windows => {
            const IMAGE_FILE_DLL: u16 = 0x2000;
            ensure!(file.starts_with(b"MZ"), "it is not a PE file");
            let pe = usize::try_from(u32_at(0x3c)?)?;
            ensure!(file.get(pe..pe + 4) == Some(b"PE\0\0"), "it has no PE signature");
            let machine = match platform.arch {
                Arch::X86_64 => 0x8664,
                Arch::Aarch64 => 0xaa64,
            };
            ensure!(
                u16_at(pe + 4)? == machine,
                "its PE machine is {:#x}, not {machine:#x}",
                u16_at(pe + 4)?
            );
            ensure!(u16_at(pe + 22)? & IMAGE_FILE_DLL != 0, "it is not a DLL");
        }
    }
    Ok(())
}

/// The RPM must name the package for the target and hold exactly the staged files, under the
/// PGDG prefix.
fn check_rpm(
    file: &Path,
    name: &str,
    version: &str,
    major: u16,
    platform: Platform,
    payload: &BTreeMap<String, Vec<u8>>,
) -> eyre::Result<()> {
    let package = rpm::Package::open(file)?;
    let metadata = &package.metadata;
    let found = [
        metadata.get_name()?,
        metadata.get_version()?,
        metadata.get_release()?,
        metadata.get_arch()?,
    ];
    let package_name = format!("{name}_{major}");
    let expected = [package_name.as_str(), version, "1", platform.rpm_arch()];
    ensure!(
        found == expected,
        "its name, version, release and arch are {found:?}, not {expected:?}"
    );
    let mut files = BTreeMap::new();
    for entry in package.files()? {
        let entry = entry?;
        let path = entry.metadata.path().to_string_lossy().trim_start_matches('/').to_owned();
        files.insert(path, entry.content);
    }
    let prefix = format!("usr/pgsql-{major}");
    let expected = payload.iter().map(|(path, bytes)| (format!("{prefix}/{path}"), bytes.clone()));
    ensure!(files == expected.collect(), "it holds {:?}", files.keys().collect::<Vec<_>>());
    Ok(())
}

/// The DEB must be a Debian 2.0 archive whose control names the package for the target, and
/// whose data holds exactly the staged files in Debian's directories.
fn check_deb(
    file: &Path,
    name: &str,
    version: &str,
    major: u16,
    platform: Platform,
    payload: &BTreeMap<String, Vec<u8>>,
) -> eyre::Result<()> {
    let mut archive = ar::Archive::new(std::fs::File::open(file)?);
    let mut members = Vec::new();
    while let Some(entry) = archive.next_entry() {
        let mut entry = entry?;
        let member = String::from_utf8_lossy(entry.header().identifier()).into_owned();
        let mut bytes = Vec::new();
        entry.read_to_end(&mut bytes)?;
        members.push((member, bytes));
    }
    let names: Vec<_> = members.iter().map(|(member, _)| member.as_str()).collect();
    ensure!(names == ["debian-binary", "control.tar.gz", "data.tar.gz"], "it holds {names:?}");
    ensure!(members[0].1 == b"2.0\n", "it is not a Debian 2.0 archive");

    let control = gzipped_files(&members[1].1)?;
    let control = String::from_utf8(control.get("control").cloned().unwrap_or_default())?;
    for line in [
        format!("Package: postgresql-{major}-{name}"),
        format!("Version: {version}"),
        format!("Architecture: {}", platform.deb_arch()),
    ] {
        ensure!(
            control.lines().any(|found| found == line),
            "its control lacks `{line}`:\n{control}"
        );
    }

    let files = gzipped_files(&members[2].1)?;
    let expected: BTreeMap<_, _> =
        payload.iter().map(|(path, bytes)| (debian_path(path, major), bytes.clone())).collect();
    ensure!(files == expected, "its data holds {:?}", files.keys().collect::<Vec<_>>());
    Ok(())
}

/// Where Debian's PostgreSQL packages put a file staged at `lib/...` or `share/...`.
fn debian_path(staged: &str, major: u16) -> String {
    match staged.strip_prefix("lib/") {
        Some(library) => format!("usr/lib/postgresql/{major}/lib/{library}"),
        None => format!("usr/share/postgresql/{major}/{}", staged.trim_start_matches("share/")),
    }
}

/// The regular files in a gzipped tar archive, by path without a leading `./` or `/`.
fn gzipped_files(bytes: &[u8]) -> eyre::Result<BTreeMap<String, Vec<u8>>> {
    let mut archive = tar::Archive::new(flate2::read::GzDecoder::new(bytes));
    let mut files = BTreeMap::new();
    for entry in archive.entries()? {
        let mut entry = entry?;
        if entry.header().entry_type().is_file() {
            let path = entry.path()?.to_string_lossy().into_owned();
            let path = path.trim_start_matches("./").trim_start_matches('/').to_owned();
            let mut content = Vec::new();
            entry.read_to_end(&mut content)?;
            files.insert(path, content);
        }
    }
    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_bundle_name_gives_the_major_and_target() {
        for (name, target) in [
            ("pgrx-0.20.0-pg18-aarch64-unknown-linux-gnu.tar.gz", "aarch64-unknown-linux-gnu"),
            ("pgrx-0.21.0-beta.1-pg18-aarch64-apple-darwin.tar.gz", "aarch64-apple-darwin"),
            ("pgrx-pg18-x86_64-pc-windows-msvc.partial.tar.gz", "x86_64-pc-windows-msvc"),
        ] {
            assert_eq!(bundle_domain(name), Some((18, target)), "{name}");
        }
        for name in [
            "pgrx-0.20.0-pg18.tar.gz",
            "pgrx-0.20.0-pgXX-x86_64-unknown-linux-gnu.tar.gz",
            "pgrx-pg18-x86_64-unknown-linux-gnu.tar.gz",
            "bundle.tgz",
        ] {
            assert_eq!(bundle_domain(name), None, "{name}");
        }
    }

    #[test]
    fn targets_name_a_platform_this_check_reads() {
        let platform = |os, arch| Some(Platform { os, arch });
        assert_eq!(Platform::of("aarch64-unknown-linux-gnu"), platform(Os::Linux, Arch::Aarch64));
        assert_eq!(Platform::of("x86_64-unknown-linux-musl"), platform(Os::Linux, Arch::X86_64));
        assert_eq!(Platform::of("aarch64-apple-darwin"), platform(Os::Macos, Arch::Aarch64));
        assert_eq!(Platform::of("x86_64-pc-windows-msvc"), platform(Os::Windows, Arch::X86_64));
        assert_eq!(Platform::of("x86_64-unknown-freebsd"), None);
        assert_eq!(Platform::of("powerpc64le-unknown-linux-gnu"), None);

        let macos = Platform { os: Os::Macos, arch: Arch::Aarch64 };
        assert_eq!((macos.library_suffix(15), macos.library_suffix(16)), (".so", ".dylib"));
    }

    #[test]
    fn a_partial_bundle_layout_takes_the_last_line() {
        let layout = Layout::reported(
            "PKGLIBDIR = /usr/lib/postgresql/18/lib\r\nSHAREDIR = /usr/share/postgresql/18\nPKGLIBDIR = /opt/pg/lib\n",
        )
        .unwrap();
        assert_eq!(layout.pkglibdir, "/opt/pg/lib");
        assert_eq!(layout.directories(Os::Linux).1, Path::new("usr/share/postgresql/18/extension"));
        assert_eq!(layout.directories(Os::Windows).0, Path::new("lib"));
        assert!(Layout::reported("PKGLIBDIR = /l\n").is_err());
    }

    fn gzipped_tar(files: &[(&str, &[u8])]) -> Vec<u8> {
        let encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::fast());
        let mut archive = tar::Builder::new(encoder);
        for (path, bytes) in files {
            let mut header = tar::Header::new_gnu();
            header.set_size(bytes.len() as u64);
            header.set_mode(0o644);
            header.set_cksum();
            archive.append_data(&mut header, path, *bytes).unwrap();
        }
        archive.into_inner().unwrap().finish().unwrap()
    }

    /// A DEB laid out as Debian's PostgreSQL packages are, which `check_deb` must accept, and
    /// the same archive with one file moved, which it must refuse.
    #[test]
    fn a_deb_must_hold_the_staged_files_in_debians_directories() {
        let directory = tempfile_dir();
        let payload = BTreeMap::from([
            ("lib/packaging.so".to_owned(), b"library".to_vec()),
            ("share/extension/packaging.control".to_owned(), b"control".to_vec()),
        ]);
        let linux = Platform { os: Os::Linux, arch: Arch::Aarch64 };
        let control = gzipped_tar(&[(
            "./control",
            b"Package: postgresql-18-packaging\nVersion: 0.1.0\nArchitecture: arm64\n",
        )]);
        let write = |name: &str, data: &[(&str, &[u8])]| {
            let path = directory.join(name);
            let mut deb = ar::Builder::new(std::fs::File::create(&path).unwrap());
            for (member, bytes) in [
                ("debian-binary", b"2.0\n".to_vec()),
                ("control.tar.gz", control.clone()),
                ("data.tar.gz", gzipped_tar(data)),
            ] {
                let header = ar::Header::new(member.as_bytes().to_vec(), bytes.len() as u64);
                deb.append(&header, bytes.as_slice()).unwrap();
            }
            path
        };
        let good = write(
            "good.deb",
            &[
                ("./usr/lib/postgresql/18/lib/packaging.so", b"library"),
                ("./usr/share/postgresql/18/extension/packaging.control", b"control"),
            ],
        );
        check_deb(&good, "packaging", "0.1.0", 18, linux, &payload).unwrap();
        let moved = write(
            "moved.deb",
            &[
                ("./usr/lib/postgresql/18/lib/packaging.so", b"library"),
                ("./usr/share/postgresql/18/share/extension/packaging.control", b"control"),
            ],
        );
        let error = check_deb(&moved, "packaging", "0.1.0", 18, linux, &payload).unwrap_err();
        assert!(error.to_string().contains("its data holds"), "{error}");
        std::fs::remove_dir_all(directory).unwrap();
    }

    fn tempfile_dir() -> PathBuf {
        let directory = std::env::temp_dir().join(format!("pgrx-ci-deb-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&directory);
        std::fs::create_dir_all(&directory).unwrap();
        directory
    }

    fn elf(kind: u16, machine: u16, interpreter: bool) -> Vec<u8> {
        let mut file = vec![0x7f, b'E', b'L', b'F', 2, 1, 1, 0];
        file.resize(16, 0);
        file.extend(kind.to_le_bytes());
        file.extend(machine.to_le_bytes());
        file.resize(32, 0);
        file.extend(64u64.to_le_bytes()); // e_phoff
        file.resize(54, 0);
        file.extend(56u16.to_le_bytes()); // e_phentsize
        file.extend(1u16.to_le_bytes()); // e_phnum
        file.resize(64, 0);
        file.extend(if interpreter { 3u32 } else { 1u32 }.to_le_bytes());
        file.resize(64 + 56, 0);
        file
    }

    fn macho(cpu: u32, kind: u32, platform: u32) -> Vec<u8> {
        let mut file = Vec::new();
        for word in [0xfeed_facf, cpu, 0, kind, 1, 24, 0, 0] {
            file.extend(u32::to_le_bytes(word));
        }
        for word in [0x32, 24, platform, 0, 0, 0] {
            file.extend(u32::to_le_bytes(word));
        }
        file
    }

    fn pe(machine: u16, characteristics: u16) -> Vec<u8> {
        let mut file = b"MZ".to_vec();
        file.resize(0x3c, 0);
        file.extend(0x40u32.to_le_bytes());
        file.extend(b"PE\0\0");
        file.extend(machine.to_le_bytes());
        file.resize(0x40 + 22, 0);
        file.extend(characteristics.to_le_bytes());
        file
    }

    #[test]
    fn only_the_targets_shared_library_passes() {
        let linux = Platform { os: Os::Linux, arch: Arch::Aarch64 };
        let macos = Platform { os: Os::Macos, arch: Arch::Aarch64 };
        let windows = Platform { os: Os::Windows, arch: Arch::X86_64 };
        check_library(&elf(3, 183, false), linux).unwrap();
        check_library(&macho(0x0100_000c, 6, 1), macos).unwrap();
        check_library(&pe(0x8664, 0x2022), windows).unwrap();
        for (file, platform, problem) in [
            (elf(3, 62, false), linux, "ELF machine is 62"),
            (elf(2, 183, false), linux, "ELF type is 2"),
            (elf(3, 183, true), linux, "program interpreter"),
            (macho(0x0100_0007, 6, 1), macos, "CPU type is 0x1000007"),
            (macho(0x0100_000c, 2, 1), macos, "file type is 2"),
            (macho(0x0100_000c, 6, 2), macos, "built for macOS"),
            (pe(0xaa64, 0x2022), windows, "PE machine is 0xaa64"),
            (pe(0x8664, 0x0022), windows, "not a DLL"),
            (b"MZ".to_vec(), windows, "truncated"),
            (elf(3, 183, false), macos, "not a 64-bit little-endian Mach-O"),
        ] {
            let error = check_library(&file, platform).unwrap_err().to_string();
            assert!(error.contains(problem), "{problem}: {error}");
        }
    }
}
