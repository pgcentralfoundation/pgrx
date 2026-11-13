//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use crate::CommandExecute;
use crate::cargo::CargoProfile;
use crate::command::get::get_property;
use crate::command::install::{
    format_display_path, install_extension, shared_library_filename, warn_if_pg_bench_enabled,
};
use crate::command::migrate::UnreleasedFragmentMode;
use crate::manifest::{PgVersionSource, display_version_info};
use crate::target::{TargetOs, target_arch};
use cargo_toml::Manifest;
use eyre::{WrapErr, eyre};
use owo_colors::OwoColorize;
use pgrx_pg_config::{PgConfig, Pgrx};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Create an installation package directory.
#[derive(clap::Args, Debug)]
#[clap(author)]
pub(crate) struct Package {
    /// Package to build (see `cargo help pkgid`)
    #[clap(long, short)]
    pub(crate) package: Option<String>,
    /// Path to Cargo.toml
    #[clap(long, value_parser)]
    pub(crate) manifest_path: Option<PathBuf>,
    /// Compile for debug mode (default is release)
    #[clap(long, short)]
    pub(crate) debug: bool,
    /// Specific profile to use (conflicts with `--debug`)
    #[clap(long)]
    pub(crate) profile: Option<String>,
    /// Build in test mode (for `cargo pgrx test`)
    #[clap(long)]
    pub(crate) test: bool,
    /// The `pg_config` path (default is first in $PATH)
    #[clap(long, short = 'c', value_parser)]
    pub(crate) pg_config: Option<PathBuf>,
    /// The directory to output the package (default is `./target/[debug|release]/extname-pgXX/`)
    #[clap(long, value_parser)]
    pub(crate) out_dir: Option<PathBuf>,
    #[clap(flatten)]
    pub(crate) features: clap_cargo::Features,
    #[clap(long)]
    pub(crate) target: Option<String>,
    /// Assemble unreleased SQL fragments into the package instead of refusing to build it
    #[clap(long, conflicts_with = "no_assemble_unreleased")]
    pub(crate) assemble_unreleased: bool,
    /// Build the package without unreleased SQL fragments instead of refusing to build it
    #[clap(long)]
    pub(crate) no_assemble_unreleased: bool,
    /// Skip the RPM and DEB packages that `[package.metadata.rpm]` and `[package.metadata.deb]` declare
    #[clap(long)]
    pub(crate) no_distrib_packages: bool,
    /// Extra cargo flags forwarded to every `cargo` invocation. Repeatable and split on whitespace: `--cargo=--config=foo` or `--cargo "--offline --frozen"`.
    #[clap(long = "cargo", value_name = "FLAG", allow_hyphen_values = true)]
    pub(crate) cargo: Vec<String>,
    #[clap(from_global, action = ArgAction::Count)]
    pub(crate) verbose: u8,
    /// The directory within out_dir to put extension assets (default is based on pg_config path)
    #[clap(long, value_parser)]
    pub(crate) prefix_dir: Option<PathBuf>,
}

impl Package {
    pub(crate) fn unreleased_mode(&self) -> UnreleasedFragmentMode {
        if self.assemble_unreleased {
            UnreleasedFragmentMode::Assemble
        } else if self.no_assemble_unreleased {
            UnreleasedFragmentMode::Skip
        } else {
            UnreleasedFragmentMode::Disallow
        }
    }

    pub(crate) fn perform(mut self) -> eyre::Result<(PathBuf, Vec<PathBuf>)> {
        let unreleased_mode = self.unreleased_mode();
        warn_if_pg_bench_enabled(&self.features, "package");
        let cargo_flags = std::mem::take(&mut self.cargo);
        let metadata =
            crate::metadata::metadata(&self.features, self.manifest_path.as_deref(), &cargo_flags)
                .wrap_err("couldn't get cargo metadata")?;
        crate::metadata::validate(self.manifest_path.as_deref(), &metadata)?;
        let package_manifest_path =
            crate::manifest::manifest_path(&metadata, self.package.as_deref())
                .wrap_err("Couldn't get manifest path")?;
        let package_manifest =
            Manifest::from_path(&package_manifest_path).wrap_err("Couldn't parse manifest")?;

        let pg_config = match self.pg_config {
            None => PgConfig::from_path(),
            Some(config) => PgConfig::new_with_defaults(config),
        };
        let pg_version = format!("pg{}", pg_config.major_version()?);

        crate::manifest::modify_features_for_version(
            &Pgrx::from_config()?,
            Some(&mut self.features),
            &package_manifest,
            &PgVersionSource::PgConfig(pg_version),
            false,
        );
        let profile = CargoProfile::from_flags(
            self.profile.as_deref(),
            // NB:  `cargo pgrx package` defaults to "--release" whereas all other commands default to "debug"
            if self.debug { CargoProfile::Dev } else { CargoProfile::Release },
        )?;
        let out_dir = if let Some(out_dir) = self.out_dir {
            out_dir
        } else {
            build_base_path(
                &pg_config,
                &package_manifest_path,
                &profile,
                self.target.as_deref(),
                metadata.target_directory.as_std_path(),
            )?
        };

        let output_files = package_extension(
            self.manifest_path.as_deref(),
            self.package.as_deref(),
            &package_manifest_path,
            &pg_config,
            out_dir.clone(),
            &profile,
            self.test,
            &self.features,
            self.target.as_deref(),
            unreleased_mode,
            metadata.target_directory.as_std_path(),
            &cargo_flags,
            self.prefix_dir.clone(),
        )?;

        if !self.no_distrib_packages {
            build_distrib_packages(
                &pg_config,
                &package_manifest_path,
                metadata.workspace_root.as_std_path(),
                &out_dir,
                self.target.as_deref(),
                metadata.target_directory.as_std_path(),
                &cargo_flags,
            )?;
        }

        Ok((out_dir, output_files))
    }
}

impl CommandExecute for Package {
    #[tracing::instrument(level = "error", skip(self))]
    fn execute(self) -> eyre::Result<()> {
        self.perform()?;
        Ok(())
    }
}

#[tracing::instrument(level = "error", skip_all, fields(
    pg_version = %pg_config.version()?,
    profile = ?profile,
    test = is_test,
))]
pub(crate) fn package_extension(
    user_manifest_path: Option<&Path>,
    user_package: Option<&str>,
    package_manifest_path: &Path,
    pg_config: &PgConfig,
    out_dir: PathBuf,
    profile: &CargoProfile,
    is_test: bool,
    features: &clap_cargo::Features,
    target: Option<&str>,
    unreleased_mode: UnreleasedFragmentMode,
    target_dir: &Path,
    cargo_flags: &[String],
    prefix_dir: Option<PathBuf>,
) -> eyre::Result<Vec<PathBuf>> {
    let out_dir_exists = out_dir.try_exists().wrap_err_with(|| {
        format!("failed to access {} while packaging extension", out_dir.display())
    })?;
    if !out_dir_exists {
        std::fs::create_dir_all(&out_dir)?;
    }

    display_version_info(pg_config, &PgVersionSource::PgConfig(pg_config.label()?));
    install_extension(
        user_manifest_path,
        user_package,
        package_manifest_path,
        pg_config,
        profile,
        is_test,
        Some(out_dir),
        features,
        target,
        unreleased_mode,
        target_dir,
        cargo_flags,
        prefix_dir,
    )
}

pub(crate) fn build_base_path(
    pg_config: &PgConfig,
    manifest_path: &Path,
    profile: &CargoProfile,
    target: Option<&str>,
    target_dir: &Path,
) -> eyre::Result<PathBuf> {
    let mut target_dir = target_dir.to_path_buf();
    let pgver = pg_config.major_version()?;
    let extname = get_property(manifest_path, "extname")?
        .ok_or_else(|| eyre!("could not determine extension name"))?;
    if let Some(target) = target {
        target_dir.push(target);
    }
    target_dir.push(profile.target_subdir());
    target_dir.push(format!("{extname}-pg{pgver}"));
    Ok(target_dir)
}

fn build_distrib_packages(
    pg_config: &PgConfig,
    manifest_path: &Path,
    workspace_root: &Path,
    out_dir: &Path,
    target: Option<&str>,
    target_dir: &Path,
    cargo_flags: &[String],
) -> eyre::Result<Vec<PathBuf>> {
    let cargo_toml = Manifest::<toml::Value>::from_path_with_metadata(manifest_path)
        .wrap_err("couldn't parse manifest metadata")?;
    let package = cargo_toml.package();

    // Quit right away if there's no metadata
    let Some(metadata) = package.metadata.as_ref() else {
        return Ok(vec![]);
    };

    let kinds: Vec<_> = DistribKind::ALL
        .into_iter()
        .filter_map(|k| Some((k, metadata.get(k.section())?)))
        .collect();
    if kinds.is_empty() {
        return Ok(vec![]);
    }

    if TargetOs::from_target(target) != TargetOs::Linux {
        eprintln!(
            "{} skipping RPM and DEB packages, which require a Linux target",
            "WARNING:".red().bold()
        );
        return Ok(vec![]);
    }

    // Cargo reports the manifest of a package by its absolute path.
    let manifest_dir = manifest_path.parent().expect("a manifest path names a file in a directory");

    // Built once and shared by every package kind
    let lib_file = shared_library_filename(manifest_path, pg_config, cargo_flags, target)?;
    let vars =
        Vars::new(pg_config, out_dir, manifest_dir, workspace_root, package, target, lib_file)?;

    kinds
        .into_iter()
        .map(|(kind, pkg_metadata)| {
            build_distrib_package(kind, pkg_metadata, &vars, manifest_dir, out_dir, target_dir)
        })
        .collect()
}

// The same Cargo.toml metadata builds one package per Postgres major version
// and distribution.
fn build_distrib_package(
    kind: DistribKind,
    metadata: &toml::Value,
    vars: &Vars,
    manifest_dir: &Path,
    out_dir: &Path,
    target_dir: &Path,
) -> eyre::Result<PathBuf> {
    let section = kind.section();
    let mut metadata = kind
        .with_defaults(metadata)
        .wrap_err_with(|| format!("in [package.metadata.{section}]"))?;
    vars.apply(&mut metadata).wrap_err_with(|| format!("in [package.metadata.{section}]"))?;

    let mut manifest: arx_pack::Manifest =
        metadata.try_into().wrap_err_with(|| format!("invalid [package.metadata.{section}]"))?;
    resolve_relative_sources(&mut manifest, manifest_dir)?;

    let path = match kind {
        DistribKind::Rpm => arx_pack::build_rpm(&manifest, out_dir),
        DistribKind::Deb => arx_pack::build_deb(&manifest, out_dir),
    }
    .map_err(|e| eyre!("{e:#}"))?;

    eprintln!(
        "{} {} package to {}",
        "     Writing".bold().green(),
        section.to_uppercase(),
        format_display_path(&path, target_dir).cyan()
    );

    Ok(path)
}

/// Resolves each relative host path in `manifest` against `manifest_dir`, the directory that
/// holds the crate's `Cargo.toml`, as Cargo resolves the paths in a manifest. arx-pack would
/// otherwise read them relative to whichever directory `cargo pgrx` runs in.
fn resolve_relative_sources(
    manifest: &mut arx_pack::Manifest,
    manifest_dir: &Path,
) -> eyre::Result<()> {
    let scripts = &mut manifest.scripts;
    let sources = manifest
        .files
        .iter_mut()
        .map(|file| &mut file.source)
        .chain(manifest.dirs.iter_mut().map(|dir| &mut dir.source))
        .chain(
            [&mut scripts.preinst, &mut scripts.postinst, &mut scripts.prerm, &mut scripts.postrm]
                .into_iter()
                .flatten(),
        );
    for source in sources {
        if Path::new(source.as_str()).is_relative() {
            *source = utf8_path(&manifest_dir.join(&*source))?;
        }
    }
    Ok(())
}

/// A distribution package format, built from `[package.metadata.<section>]`.
#[derive(Clone, Copy, Debug)]
enum DistribKind {
    Rpm,
    Deb,
}

impl DistribKind {
    const ALL: [Self; 2] = [Self::Rpm, Self::Deb];

    fn section(self) -> &'static str {
        match self {
            Self::Rpm => "rpm",
            Self::Deb => "deb",
        }
    }

    /// The metadata used when `[package.metadata.<section>]` leaves a key out.
    /// The paths match the PGDG RPM layout and the Debian/PGDG apt layout.
    fn default_template(self) -> &'static str {
        match self {
            Self::Rpm => {
                r#"
name = "${NAME}_${PG_MAJOR_VERSION}"
version = "${VERSION}"
arch = "${ARCH}"
description = "${DESCRIPTION}"
license = "${LICENSE}"
maintainer = "${MAINTAINER}"
depends = ["postgresql${PG_MAJOR_VERSION}-server"]
files = [
  { source = "${BUILD_BASE_PATH}/${PG_PKGLIBDIR}/${LIB_FILE}", dest = "/usr/pgsql-${PG_MAJOR_VERSION}/lib/${LIB_FILE}", mode = "0755" },
]
dirs = [
  { source = "${BUILD_BASE_PATH}/${PG_SHAREDIR}/extension/", dest = "/usr/pgsql-${PG_MAJOR_VERSION}/share/extension/" },
]
"#
            }
            Self::Deb => {
                r#"
name = "postgresql-${PG_MAJOR_VERSION}-${DEB_NAME}"
version = "${VERSION}"
arch = "${ARCH}"
description = "${DESCRIPTION}"
license = "${LICENSE}"
maintainer = "${MAINTAINER}"
depends = ["postgresql-${PG_MAJOR_VERSION}"]
files = [
  { source = "${BUILD_BASE_PATH}/${PG_PKGLIBDIR}/${LIB_FILE}", dest = "/usr/lib/postgresql/${PG_MAJOR_VERSION}/lib/${LIB_FILE}", mode = "0644" },
]
dirs = [
  { source = "${BUILD_BASE_PATH}/${PG_SHAREDIR}/extension/", dest = "/usr/share/postgresql/${PG_MAJOR_VERSION}/extension/" },
]
"#
            }
        }
    }

    /// Overlays the user's section on the default template. Each top-level key
    /// the user sets replaces the default value as a whole, including arrays
    /// such as `files` and `depends`.
    fn with_defaults(self, user: &toml::Value) -> eyre::Result<toml::Value> {
        let user = user.as_table().ok_or_else(|| eyre!("expected a table"))?;
        let mut merged: toml::Table = toml::from_str(self.default_template())
            .expect("default package templates are valid TOML, see `default_templates_parse`");
        merged.extend(user.iter().map(|(k, v)| (k.clone(), v.clone())));
        Ok(toml::Value::Table(merged))
    }
}

/// The variables allowed in every `[package.metadata.<section>]` of [`DistribKind`].
///
/// A variable whose crate field is missing holds the reason instead of a value,
/// so it is an error only when the metadata actually uses it.
struct Vars(HashMap<&'static str, Result<String, &'static str>>);

impl Vars {
    fn new(
        pg_config: &PgConfig,
        out_dir: &Path,
        manifest_dir: &Path,
        workspace_root: &Path,
        package: &cargo_toml::Package<toml::Value>,
        target: Option<&str>,
        lib_file: String,
    ) -> eyre::Result<Self> {
        let optional = |value: Option<&str>, reason| value.map(str::to_owned).ok_or(reason);
        // Absolute, so that a relative `--out-dir` still names the directory below the one
        // `cargo pgrx` runs in after relative sources resolve against `manifest_dir`.
        let out_dir = std::path::absolute(out_dir)
            .wrap_err_with(|| format!("couldn't resolve {}", out_dir.display()))?;
        Ok(Self(HashMap::from([
            ("ARCH", Ok(target_arch(target).to_owned())),
            ("BUILD_BASE_PATH", Ok(utf8_path(&out_dir)?)),
            (
                "DESCRIPTION",
                optional(package.description(), "set `package.description` in Cargo.toml"),
            ),
            ("LICENSE", optional(package.license(), "set `package.license` in Cargo.toml")),
            (
                "MAINTAINER",
                optional(
                    package.authors().first().map(String::as_str),
                    "set `package.authors` in Cargo.toml",
                ),
            ),
            ("DEB_NAME", Ok(deb_name(package.name()))),
            ("LIB_FILE", Ok(lib_file)),
            ("MANIFEST_DIR", Ok(utf8_path(manifest_dir)?)),
            ("NAME", Ok(package.name().to_owned())),
            ("PG_MAJOR_VERSION", Ok(pg_config.major_version()?.to_string())),
            ("PG_PKGLIBDIR", Ok(utf8_path(&pg_config.pkglibdir()?)?)),
            ("PG_SHAREDIR", Ok(utf8_path(&pg_config.sharedir()?)?)),
            ("VERSION", Ok(package_version(&package.version().to_string()))),
            ("WORKSPACE_ROOT", Ok(utf8_path(workspace_root)?)),
        ])))
    }

    /// Expands every `${VAR}` in `src` in a single pass: substituted text is
    /// never rescanned, so the result does not depend on substitution order.
    fn expand(&self, src: &str) -> eyre::Result<String> {
        let mut out = String::with_capacity(src.len());
        let mut rest = src;
        while let Some(start) = rest.find("${") {
            out.push_str(&rest[..start]);
            let after = &rest[start + 2..];
            let end = after.find('}').ok_or_else(|| eyre!("unterminated `${{` in {src:?}"))?;
            let key = &after[..end];
            let val = self.0.get(key).ok_or_else(|| {
                eyre!("unknown variable `${{{key}}}` in {src:?}; known variables: {}", self.known())
            })?;
            let val = val.as_ref().map_err(|reason| {
                eyre!("`${{{key}}}` has no value: {reason}, or override the key that uses it")
            })?;
            out.push_str(val);
            rest = &after[end + 1..];
        }
        out.push_str(rest);
        Ok(out)
    }

    fn known(&self) -> String {
        let mut keys: Vec<_> = self.0.keys().copied().collect();
        keys.sort_unstable();
        keys.join(", ")
    }

    /// Substitutes inside string values only; keys and other value types are left untouched.
    fn apply(&self, value: &mut toml::Value) -> eyre::Result<()> {
        match value {
            toml::Value::String(s) => *s = self.expand(s)?,
            toml::Value::Array(a) => a.iter_mut().try_for_each(|v| self.apply(v))?,
            toml::Value::Table(t) => t.iter_mut().try_for_each(|(_, v)| self.apply(v))?,
            _ => {}
        }
        Ok(())
    }
}

/// RPM rejects `-` in a version, and Debian reads it as a revision that sorts
/// after the final release. Both formats sort `~` before the final release.
fn package_version(crate_version: &str) -> String {
    crate_version.replace('-', "~")
}

/// Debian package names allow only lowercase letters, digits, `+`, `-` and `.`.
/// Crate names are ASCII letters, digits, `_` and `-`, so lowercasing and
/// replacing `_` is enough.
fn deb_name(crate_name: &str) -> String {
    crate_name.to_ascii_lowercase().replace('_', "-")
}

// Package manifests are UTF-8 TOML, so a lossy conversion would silently
// produce wrong install paths.
fn utf8_path(path: &Path) -> eyre::Result<String> {
    path.to_str()
        .map(str::to_owned)
        .ok_or_else(|| eyre!("path is not valid UTF-8: {}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vars() -> Vars {
        Vars(HashMap::from([
            ("ARCH", Ok("x86_64".to_owned())),
            ("BUILD_BASE_PATH", Ok(r"C:\Users\me\target\my_ext-pg17".to_owned())),
            ("DEB_NAME", Ok("my-ext".to_owned())),
            ("DESCRIPTION", Ok("My extension".to_owned())),
            ("LIB_FILE", Ok("my_ext.so".to_owned())),
            ("LICENSE", Err("set `package.license` in Cargo.toml")),
            ("MAINTAINER", Ok("Me".to_owned())),
            ("MANIFEST_DIR", Ok("/src/my_ext".to_owned())),
            ("NAME", Ok("my_ext".to_owned())),
            ("PG_MAJOR_VERSION", Ok("17".to_owned())),
            ("PG_PKGLIBDIR", Ok("/usr/lib/postgresql/17/lib".to_owned())),
            ("PG_SHAREDIR", Ok("/odd/${NAME}/share".to_owned())),
            ("VERSION", Ok("1.0.0".to_owned())),
            ("WORKSPACE_ROOT", Ok("/src".to_owned())),
        ]))
    }

    #[test]
    fn expands_variables() {
        assert_eq!(vars().expand("${NAME}_${PG_MAJOR_VERSION}").unwrap(), "my_ext_17");
        assert_eq!(vars().expand("no variables").unwrap(), "no variables");
        assert_eq!(vars().expand("").unwrap(), "");
    }

    #[test]
    fn does_not_rescan_substituted_text() {
        assert_eq!(
            vars().expand("${PG_SHAREDIR}/extension").unwrap(),
            "/odd/${NAME}/share/extension"
        );
    }

    #[test]
    fn rejects_unknown_variable() {
        let err = vars().expand("/usr/pgsql-${PG_MAJ OR_VERSION}/lib").unwrap_err().to_string();
        assert!(err.contains("unknown variable `${PG_MAJ OR_VERSION}`"), "{err}");
        assert!(err.contains("NAME, PG_MAJOR_VERSION"), "{err}");
    }

    #[test]
    fn rejects_unterminated_variable() {
        let err = vars().expand("${NAME").unwrap_err().to_string();
        assert!(err.contains("unterminated"), "{err}");
    }

    #[test]
    fn missing_crate_field_explains_how_to_fix() {
        let err = vars().expand("${LICENSE}").unwrap_err().to_string();
        assert!(err.contains("set `package.license` in Cargo.toml"), "{err}");
    }

    #[test]
    fn substitutes_values_not_keys_and_keeps_windows_paths() {
        let mut value: toml::Value = toml::from_str(
            r#"
            "${NAME}" = "${NAME}"
            mode = 420
            files = [{ source = "${BUILD_BASE_PATH}/x.so", dest = "/lib/${NAME}.so" }]
            "#,
        )
        .unwrap();
        vars().apply(&mut value).unwrap();

        let table = value.as_table().unwrap();
        assert_eq!(table["${NAME}"].as_str(), Some("my_ext"));
        assert_eq!(table["mode"].as_integer(), Some(420));
        let file = &table["files"][0];
        assert_eq!(file["source"].as_str(), Some(r"C:\Users\me\target\my_ext-pg17/x.so"));
        assert_eq!(file["dest"].as_str(), Some("/lib/my_ext.so"));

        // The backslashes survive a TOML round trip because `toml` escapes them.
        let reparsed: toml::Value = toml::from_str(&toml::to_string(&value).unwrap()).unwrap();
        assert_eq!(reparsed, value);
    }

    #[test]
    fn default_templates_parse() {
        let empty = toml::Value::Table(toml::Table::new());
        let vars =
            Vars(vars().0.into_iter().map(|(k, v)| (k, Ok(v.unwrap_or_default()))).collect());
        for kind in DistribKind::ALL {
            let mut value = kind.with_defaults(&empty).unwrap();
            vars.apply(&mut value).unwrap();
            let _: arx_pack::Manifest = value.try_into().unwrap();
        }
    }

    #[test]
    fn user_keys_replace_defaults() {
        let user: toml::Value = toml::from_str(
            r#"
            license = "PostgreSQL"
            depends = ["postgresql17-server", "postgis35_17"]
            "#,
        )
        .unwrap();
        let mut value = DistribKind::Rpm.with_defaults(&user).unwrap();
        vars().apply(&mut value).unwrap();

        let table = value.as_table().unwrap();
        assert_eq!(table["license"].as_str(), Some("PostgreSQL"));
        assert_eq!(table["depends"].as_array().unwrap().len(), 2);
        assert_eq!(table["name"].as_str(), Some("my_ext_17"));
        assert_eq!(table["files"][0]["dest"].as_str(), Some("/usr/pgsql-17/lib/my_ext.so"));
    }

    #[test]
    fn deb_name_is_valid_for_debian() {
        assert_eq!(deb_name("My_Ext-2"), "my-ext-2");

        // The fixture leaves `${LICENSE}` unset, so the section must provide it.
        let user: toml::Value = toml::from_str(r#"license = "MIT""#).unwrap();
        let mut value = DistribKind::Deb.with_defaults(&user).unwrap();
        vars().apply(&mut value).unwrap();
        assert_eq!(value["name"].as_str(), Some("postgresql-17-my-ext"));
    }

    #[test]
    fn package_arch_follows_the_target() {
        assert_eq!(target_arch(Some("aarch64-unknown-linux-gnu")), "aarch64");
        assert_eq!(target_arch(None), std::env::consts::ARCH);
    }

    #[test]
    fn package_version_sorts_pre_releases_first() {
        assert_eq!(package_version("1.2.3"), "1.2.3");
        assert_eq!(package_version("1.2.3-rc.1"), "1.2.3~rc.1");
    }

    fn package_manifest(dir: &Path) -> PathBuf {
        let path = dir.join("Cargo.toml");
        std::fs::write(
            &path,
            r#"
            [package]
            name = "my-ext"
            version = "1.0.0"
            description = "Extension"
            license = "MIT"
            authors = ["Me"]
            [lib]
            path = "lib.rs"
            [package.metadata.rpm]
            [package.metadata.deb]
            "#,
        )
        .unwrap();
        path
    }

    #[test]
    fn non_linux_targets_skip_distribution_packages() {
        let dir = tempfile::tempdir().unwrap();
        let manifest = package_manifest(dir.path());
        // No staging tree or pg_config program is needed when the target cannot use these packages.
        for target in ["aarch64-apple-darwin", "x86_64-pc-windows-msvc", "x86_64-unknown-freebsd"] {
            let paths = build_distrib_packages(
                &PgConfig::default(),
                &manifest,
                dir.path(),
                dir.path(),
                Some(target),
                dir.path(),
                &[],
            )
            .unwrap();
            assert!(paths.is_empty(), "{target}: {paths:?}");
        }
    }

    #[cfg(unix)]
    #[test]
    fn linux_target_builds_distribution_packages_on_any_unix_host() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().unwrap();
        let dir = dir.path();
        let manifest = package_manifest(dir);
        std::fs::write(dir.join("my_ext.control"), "default_version = '1.0.0'\n").unwrap();
        let pg_config = dir.join("pg_config");
        std::fs::write(
            &pg_config,
            r#"#!/bin/sh
case "$1" in
    --version) echo 'PostgreSQL 18.1' ;;
    --pkglibdir) echo '/usr/lib/postgresql/18/lib' ;;
    --sharedir) echo '/usr/share/postgresql/18' ;;
    *) exit 1 ;;
esac
"#,
        )
        .unwrap();
        std::fs::set_permissions(&pg_config, std::fs::Permissions::from_mode(0o755)).unwrap();
        let pg_config = PgConfig::new_with_defaults(pg_config);
        let libdir = dir.join("usr/lib/postgresql/18/lib");
        let extdir = dir.join("usr/share/postgresql/18/extension");
        std::fs::create_dir_all(&libdir).unwrap();
        std::fs::create_dir_all(&extdir).unwrap();
        std::fs::write(libdir.join("my_ext-1.0.0.so"), "library").unwrap();
        std::fs::copy(dir.join("my_ext.control"), extdir.join("my_ext.control")).unwrap();
        for (target, rpm_arch, deb_arch) in [
            ("x86_64-unknown-linux-gnu", "x86_64", "amd64"),
            ("aarch64-unknown-linux-gnu", "aarch64", "arm64"),
        ] {
            let paths =
                build_distrib_packages(&pg_config, &manifest, dir, dir, Some(target), dir, &[])
                    .unwrap();
            assert_eq!(
                paths,
                [
                    dir.join(format!("my-ext_18-1.0.0-1.{rpm_arch}.rpm")),
                    dir.join(format!("postgresql-18-my-ext_1.0.0_{deb_arch}.deb")),
                ]
            );
            assert!(std::fs::read(&paths[0]).unwrap().starts_with(b"\xed\xab\xee\xdb"));
            assert!(std::fs::read(&paths[1]).unwrap().starts_with(b"!<arch>\n"));
        }
    }

    // The library's file name differs from the crate name, as it does for a
    // hyphenated crate or a versioned shared library.
    #[cfg(unix)]
    #[test]
    fn builds_packages_from_staged_files() {
        let out_dir = tempfile::tempdir().unwrap();
        let out_dir = out_dir.path();
        let libdir = out_dir.join("usr/lib/postgresql/17/lib");
        let sharedir = out_dir.join("usr/share/postgresql/17");
        std::fs::create_dir_all(&libdir).unwrap();
        std::fs::create_dir_all(sharedir.join("extension")).unwrap();
        std::fs::write(libdir.join("my_ext-1.0.0.so"), "library").unwrap();
        std::fs::write(sharedir.join("extension/my_ext.control"), "control").unwrap();

        let mut vars = vars();
        vars.0.extend([
            ("BUILD_BASE_PATH", Ok(utf8_path(out_dir).unwrap())),
            ("DEB_NAME", Ok("my-ext".to_owned())),
            ("LIB_FILE", Ok("my_ext-1.0.0.so".to_owned())),
            ("LICENSE", Ok("MIT".to_owned())),
            ("NAME", Ok("my-ext".to_owned())),
            ("PG_SHAREDIR", Ok("/usr/share/postgresql/17".to_owned())),
            ("VERSION", Ok(package_version("1.0.0-rc.1"))),
        ]);

        let section = toml::Value::Table(toml::Table::new());
        for (kind, expected) in [
            (DistribKind::Rpm, "my-ext_17-1.0.0~rc.1-1.x86_64.rpm"),
            (DistribKind::Deb, "postgresql-17-my-ext_1.0.0~rc.1_amd64.deb"),
        ] {
            let path =
                build_distrib_package(kind, &section, &vars, out_dir, out_dir, out_dir).unwrap();
            assert_eq!(path, out_dir.join(expected));
            assert!(path.is_file(), "{}", path.display());
        }
    }

    #[test]
    fn resolves_relative_sources_against_the_manifest_dir() {
        // Paths such as `/src` are relative to the current drive on Windows, so build absolute
        // ones from the temporary directory.
        let manifest_dir = std::env::temp_dir().join("my_ext");
        let staged = utf8_path(&manifest_dir.join("staged").join("my_ext.so")).unwrap();
        // A TOML literal string keeps Windows backslashes as they are.
        let mut manifest = arx_pack::Manifest::from_toml_str(&format!(
            r#"
            name = "my-ext"
            version = "1.0.0"
            arch = "x86_64"
            maintainer = "Me"
            description = "My extension"
            license = "MIT"
            files = [
              {{ source = '{staged}', dest = "/lib/my_ext.so", mode = "0755" }},
              {{ source = "../LICENSE", dest = "/doc/copyright", mode = "0644" }},
            ]
            dirs = [{{ source = "sql/", dest = "/share/extension/" }}]
            scripts = {{ postinst = "packaging/postinst" }}
            "#,
        ))
        .unwrap();
        resolve_relative_sources(&mut manifest, &manifest_dir).unwrap();

        let resolved = |path: &str| utf8_path(&manifest_dir.join(path)).unwrap();
        assert_eq!(manifest.files[0].source, staged);
        assert_eq!(manifest.files[1].source, resolved("../LICENSE"));
        assert_eq!(manifest.dirs[0].source, resolved("sql/"));
        assert_eq!(manifest.scripts.postinst, Some(resolved("packaging/postinst")));
        assert_eq!(manifest.scripts.preinst, None);
    }

    // `cargo test` runs in the cargo-pgrx directory, which has no `copyright` file, so the
    // package builds only if the relative source is read from the manifest directory.
    #[cfg(unix)]
    #[test]
    fn reads_relative_sources_from_the_manifest_dir() {
        let out_dir = tempfile::tempdir().unwrap();
        let manifest_dir = tempfile::tempdir().unwrap();
        std::fs::write(out_dir.path().join("my_ext.so"), "library").unwrap();
        std::fs::write(manifest_dir.path().join("copyright"), "MIT").unwrap();

        let mut vars = vars();
        vars.0.extend([
            ("BUILD_BASE_PATH", Ok(utf8_path(out_dir.path()).unwrap())),
            ("LICENSE", Ok("MIT".to_owned())),
        ]);
        let section: toml::Value = toml::from_str(
            r#"
            files = [
              { source = "${BUILD_BASE_PATH}/${LIB_FILE}", dest = "/lib/${LIB_FILE}", mode = "0644" },
              { source = "copyright", dest = "/usr/share/doc/${NAME}/copyright", mode = "0644" },
            ]
            dirs = []
            "#,
        )
        .unwrap();
        let path = build_distrib_package(
            DistribKind::Deb,
            &section,
            &vars,
            manifest_dir.path(),
            out_dir.path(),
            out_dir.path(),
        )
        .unwrap();
        assert!(path.is_file(), "{}", path.display());
    }

    #[test]
    fn rejects_non_table_section() {
        let err = DistribKind::Deb.with_defaults(&toml::Value::Boolean(true)).unwrap_err();
        assert!(err.to_string().contains("expected a table"), "{err}");
    }
}
