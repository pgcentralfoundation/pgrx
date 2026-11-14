//LICENSE Portions Copyright 2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.

//! A target bundle: one file, made on the platform an extension will run on, that lets any
//! other machine package extensions for that platform's PostgreSQL.
//!
//! The file is a gzipped tar archive holding:
//!
//! - `bundle.json`, the [`Description`]: the Rust target and what `pg_config` printed there;
//! - `target-info/`, what the `pgrx-pg-sys` build script generated there, which it verifies
//!   itself when a build imports it;
//! - `lib/postgres.lib`, for MSVC targets, whose extensions link against it.

use eyre::{WrapErr, bail, ensure, eyre};
use pgrx_pg_config::PgConfig;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::File;
use std::path::{Path, PathBuf};

const DESCRIPTION: &str = "bundle.json";
const TARGET_INFO: &str = "target-info";
const LIBRARIES: &str = "lib";
const IMPORT_LIBRARY: &str = "postgres.lib";

/// The on-disk contract of [`Description`] and the archive's layout.
const FORMAT: u32 = 1;

/// The `pg_config` properties a bundle records, which are the ones packaging reads.
const PROPERTIES: [&str; 3] = ["version", "pkglibdir", "sharedir"];

/// What a bundle says about where it came from.
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Description {
    format: u32,
    /// The Rust target the bundle was generated on, which is the one it builds for.
    target: String,
    /// What that machine's `pg_config` printed, by option name without its dashes.
    pg_config: BTreeMap<String, String>,
}

/// The file name of the bundle for one PostgreSQL major version and Rust target.
pub(crate) fn file_name(major_version: u16, target: &str) -> String {
    format!("pgrx-pg{major_version}-{target}.tar.gz")
}

/// Write the bundle for `pg_config`'s PostgreSQL into `out_dir`, from the `target_info` that a
/// build on this machine just exported. Returns the file's path.
pub(crate) fn write(
    pg_config: &PgConfig,
    target_info: &Path,
    out_dir: &Path,
) -> eyre::Result<PathBuf> {
    let major_version = pg_config.major_version()?;
    let target = exported_target(target_info, major_version)?;
    let description = Description {
        format: FORMAT,
        pg_config: BTreeMap::from([
            ("version".to_owned(), format!("PostgreSQL {}", pg_config.version()?)),
            ("pkglibdir".to_owned(), utf8(&pg_config.pkglibdir()?)?),
            ("sharedir".to_owned(), utf8(&pg_config.sharedir()?)?),
        ]),
        target,
    };

    std::fs::create_dir_all(out_dir)
        .wrap_err_with(|| format!("couldn't create {}", out_dir.display()))?;
    let path = out_dir.join(file_name(major_version, &description.target));
    let file =
        File::create(&path).wrap_err_with(|| format!("couldn't create {}", path.display()))?;
    let mut archive =
        tar::Builder::new(flate2::write::GzEncoder::new(file, flate2::Compression::default()));

    let json = serde_json::to_vec_pretty(&description)?;
    let mut header = tar::Header::new_gnu();
    header.set_size(json.len() as u64);
    header.set_mode(0o644);
    header.set_cksum();
    archive.append_data(&mut header, DESCRIPTION, json.as_slice())?;
    archive.append_dir_all(TARGET_INFO, target_info)?;
    if description.target.ends_with("-msvc") {
        let library = pg_config.lib_dir()?.join(IMPORT_LIBRARY);
        archive
            .append_path_with_name(&library, Path::new(LIBRARIES).join(IMPORT_LIBRARY))
            .wrap_err_with(|| format!("couldn't read {}", library.display()))?;
    }
    archive.into_inner()?.finish()?;
    Ok(path)
}

/// The Rust target that the build script recorded in the bundle it exported.
fn exported_target(target_info: &Path, major_version: u16) -> eyre::Result<String> {
    let manifest = target_info.join(format!("pg{major_version}_target_artifact.json"));
    let json = std::fs::read(&manifest).wrap_err_with(|| {
        format!("the build wrote no bundle manifest at {}; does this crate build pgrx-pg-sys for pg{major_version}?", manifest.display())
    })?;
    let json: serde_json::Value = serde_json::from_slice(&json)
        .wrap_err_with(|| format!("couldn't parse {}", manifest.display()))?;
    json["domain"]["rust_target"]
        .as_str()
        .map(str::to_owned)
        .ok_or_else(|| eyre!("{} names no Rust target", manifest.display()))
}

/// A bundle unpacked for a build on this machine.
#[derive(Debug)]
pub(crate) struct Bundle {
    /// The Rust target to build for.
    pub(crate) target: String,
    /// The target's PostgreSQL, as its `pg_config` described it when the bundle was made.
    pub(crate) pg_config: PgConfig,
    /// The generated sources and native archive for the `pgrx-pg-sys` build to import.
    pub(crate) target_info: PathBuf,
}

impl Bundle {
    /// Unpack `file` into a directory of its own below `scratch`, replacing an earlier unpack.
    pub(crate) fn open(file: &Path, scratch: &Path) -> eyre::Result<Self> {
        let name = file.file_name().ok_or_else(|| eyre!("{} is not a file", file.display()))?;
        let root = scratch.join("pgrx-bundle").join(name);
        if root.exists() {
            std::fs::remove_dir_all(&root)
                .wrap_err_with(|| format!("couldn't remove {}", root.display()))?;
        }
        std::fs::create_dir_all(&root)
            .wrap_err_with(|| format!("couldn't create {}", root.display()))?;
        unpack(file, &root).wrap_err_with(|| format!("couldn't unpack {}", file.display()))?;

        let description = root.join(DESCRIPTION);
        let description: Description = serde_json::from_slice(
            &std::fs::read(&description)
                .wrap_err_with(|| format!("{} has no {DESCRIPTION}", file.display()))?,
        )
        .wrap_err_with(|| format!("couldn't parse {DESCRIPTION} in {}", file.display()))?;
        ensure!(
            description.format == FORMAT,
            "{} is a format {} bundle, and this cargo-pgrx reads format {FORMAT}; generate it again with this version",
            file.display(),
            description.format
        );
        let target_info = root.join(TARGET_INFO);
        ensure!(target_info.is_dir(), "{} has no {TARGET_INFO} directory", file.display());

        // The build links against this directory. It is empty except for MSVC targets.
        let libraries = root.join(LIBRARIES);
        std::fs::create_dir_all(&libraries)?;
        let mut properties = description.pg_config;
        for required in PROPERTIES {
            ensure!(
                properties.contains_key(required),
                "{} does not record `pg_config --{required}`",
                file.display()
            );
        }
        properties.insert("libdir".to_owned(), utf8(&libraries)?);
        Ok(Self {
            target: description.target,
            pg_config: PgConfig::from_properties(properties),
            target_info,
        })
    }
}

/// Unpack a bundle's regular files and directories. A bundle has nothing else, so any other
/// kind of entry, or one that would land outside `root`, is refused.
fn unpack(file: &Path, root: &Path) -> eyre::Result<()> {
    let file = File::open(file)?;
    let mut archive = tar::Archive::new(flate2::read::GzDecoder::new(file));
    for entry in archive.entries()? {
        let mut entry = entry?;
        let kind = entry.header().entry_type();
        let path = entry.path()?.into_owned();
        if !(kind.is_file() || kind.is_dir()) {
            bail!("{} is neither a file nor a directory", path.display());
        }
        if !entry.unpack_in(root)? {
            bail!("{} would be written outside the bundle", path.display());
        }
    }
    Ok(())
}

fn utf8(path: &Path) -> eyre::Result<String> {
    path.to_str().map(str::to_owned).ok_or_else(|| eyre!("{} is not UTF-8", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A stand-in for what the build script exports, with the manifest fields a bundle reads.
    fn target_info(directory: &Path, target: &str) -> PathBuf {
        let target_info = directory.join("exported");
        std::fs::create_dir_all(target_info.join("cmacros/pg18")).unwrap();
        std::fs::write(
            target_info.join("pg18_target_artifact.json"),
            format!(r#"{{"domain": {{"major": 18, "rust_target": "{target}"}}}}"#),
        )
        .unwrap();
        std::fs::write(target_info.join("cmacros/pg18/mod.rs"), "// generated").unwrap();
        target_info
    }

    fn postgres(properties: &[(&str, &str)]) -> PgConfig {
        PgConfig::from_properties(properties.iter().copied())
    }

    #[test]
    fn a_bundle_round_trips_its_target_and_postgres() {
        let directory = tempfile::tempdir().unwrap();
        let directory = directory.path();
        let pg_config = postgres(&[
            ("version", "PostgreSQL 18.1"),
            ("pkglibdir", "/usr/lib/postgresql/18/lib"),
            ("sharedir", "/usr/share/postgresql/18"),
        ]);
        let exported = target_info(directory, "aarch64-unknown-linux-gnu");
        let file = write(&pg_config, &exported, &directory.join("out")).unwrap();
        assert_eq!(
            file.file_name().unwrap(),
            "pgrx-pg18-aarch64-unknown-linux-gnu.tar.gz",
            "the name says which PostgreSQL and platform the bundle is for"
        );

        let bundle = Bundle::open(&file, &directory.join("scratch")).unwrap();
        assert_eq!(bundle.target, "aarch64-unknown-linux-gnu");
        assert_eq!(bundle.pg_config.major_version().unwrap(), 18);
        assert_eq!(
            bundle.pg_config.pkglibdir().unwrap(),
            Path::new("/usr/lib/postgresql/18/lib"),
            "package directories are the target's, not this machine's"
        );
        assert_eq!(bundle.pg_config.sharedir().unwrap(), Path::new("/usr/share/postgresql/18"));
        assert!(bundle.pg_config.lib_dir().unwrap().is_dir(), "the link directory is local");
        assert_eq!(
            std::fs::read_to_string(bundle.target_info.join("cmacros/pg18/mod.rs")).unwrap(),
            "// generated"
        );
        // Opening it again replaces the first unpack.
        Bundle::open(&file, &directory.join("scratch")).unwrap();
    }

    #[test]
    fn an_msvc_bundle_carries_the_import_library() {
        let directory = tempfile::tempdir().unwrap();
        let directory = directory.path();
        let libdir = directory.join("pg/lib");
        std::fs::create_dir_all(&libdir).unwrap();
        let pg_config = |libdir: &Path| {
            postgres(&[
                ("version", "PostgreSQL 18.1"),
                ("pkglibdir", "C:/PostgreSQL/18/lib"),
                ("sharedir", "C:/PostgreSQL/18/share"),
                ("libdir", libdir.to_str().unwrap()),
            ])
        };
        let exported = target_info(directory, "x86_64-pc-windows-msvc");
        let error = write(&pg_config(&libdir), &exported, &directory.join("out")).unwrap_err();
        assert!(format!("{error:#}").contains("postgres.lib"), "{error:#}");

        std::fs::write(libdir.join("postgres.lib"), "import library").unwrap();
        let file = write(&pg_config(&libdir), &exported, &directory.join("out")).unwrap();
        let bundle = Bundle::open(&file, &directory.join("scratch")).unwrap();
        let library = bundle.pg_config.lib_dir().unwrap().join("postgres.lib");
        assert_eq!(std::fs::read_to_string(library).unwrap(), "import library");
    }

    #[test]
    fn a_build_that_exported_nothing_is_reported() {
        let directory = tempfile::tempdir().unwrap();
        let pg_config = postgres(&[("version", "PostgreSQL 18.1")]);
        let error = write(&pg_config, directory.path(), directory.path()).unwrap_err();
        assert!(error.to_string().contains("wrote no bundle manifest"), "{error}");
    }

    #[test]
    fn other_archives_and_formats_are_refused() {
        let directory = tempfile::tempdir().unwrap();
        let directory = directory.path();
        let archive =
            |name: &str, build: &dyn Fn(&mut tar::Builder<flate2::write::GzEncoder<File>>)| {
                let path = directory.join(name);
                let file = File::create(&path).unwrap();
                let mut archive = tar::Builder::new(flate2::write::GzEncoder::new(
                    file,
                    flate2::Compression::default(),
                ));
                build(&mut archive);
                archive.into_inner().unwrap().finish().unwrap();
                path
            };
        let data = |archive: &mut tar::Builder<_>, name: &str, bytes: &[u8]| {
            let mut header = tar::Header::new_gnu();
            header.set_size(bytes.len() as u64);
            header.set_mode(0o644);
            header.set_cksum();
            archive.append_data(&mut header, name, bytes).unwrap();
        };

        let newer = archive("newer.tar.gz", &|archive| {
            data(archive, DESCRIPTION, br#"{"format": 2, "target": "t", "pg_config": {}}"#);
        });
        let error = Bundle::open(&newer, &directory.join("scratch")).unwrap_err();
        assert!(error.to_string().contains("format 2 bundle"), "{error}");

        let link = archive("link.tar.gz", &|archive| {
            let mut header = tar::Header::new_gnu();
            header.set_entry_type(tar::EntryType::Symlink);
            header.set_size(0);
            archive.append_link(&mut header, "target-info", "/etc").unwrap();
        });
        let error = Bundle::open(&link, &directory.join("scratch")).unwrap_err();
        assert!(format!("{error:#}").contains("neither a file nor a directory"), "{error:#}");

        let empty = archive("empty.tar.gz", &|_| {});
        let error = Bundle::open(&empty, &directory.join("scratch")).unwrap_err();
        assert!(error.to_string().contains("has no bundle.json"), "{error}");

        let incomplete = archive("incomplete.tar.gz", &|archive| {
            data(
                archive,
                DESCRIPTION,
                br#"{"format": 1, "target": "t", "pg_config": {"version": "PostgreSQL 18.1"}}"#,
            );
            data(archive, "target-info/placeholder", b"");
        });
        let error = Bundle::open(&incomplete, &directory.join("scratch")).unwrap_err();
        assert!(error.to_string().contains("pg_config --pkglibdir"), "{error}");
    }
}
