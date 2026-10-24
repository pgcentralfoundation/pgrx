//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use cargo_toml::{Dependency, DepsSet, Manifest};
use crates_index::{HashKind, SparseIndex, sparse};
use eyre::eyre;
use semver::VersionReq;
use std::collections::HashSet;
use std::sync::LazyLock;
use std::{fs, path::PathBuf};
use toml_edit::DocumentMut;
use tracing::info;

use crate::CommandExecute;

/// Upgrade pgrx crate versions in `Cargo.toml`.
/// Defaults to latest.
#[derive(clap::Args, Debug)]
#[clap(author)]
pub(crate) struct Upgrade {
    /// Specify a version requirement to upgrade to, rather than defaulting to the latest version. Accepts any Cargo version requirement syntax: bare versions (`0.16.1`, treated as caret), explicit operators (`^0.16`, `~0.16.1`, `=0.16.1`, `>=0.16, <0.17`), etc.
    #[clap(long)]
    pub(crate) to: Option<VersionReq>,

    /// Path to the manifest file (usually Cargo.toml). Defaults to
    /// "./Cargo.toml" in the working directory.
    #[clap(long = "manifest-path", short)]
    pub(crate) manifest_path: Option<PathBuf>,

    /// Flag for upgrading PGRX to pre-release versions.
    #[clap(long = "include-prereleases")]
    pub(crate) include_prereleases: bool,

    /// Select a package within the current workspace within which to upgrade
    /// pgrx package versions. Defaults to the root manifest in the working
    /// directory.
    #[clap(long, short)]
    pub(crate) package: Option<String>,

    /// Dry-run - if this flag is set, Cargo.toml will not be modified.
    /// Instead, this command will print the text of the new Cargo.toml
    /// that would have been generated if it was modified.
    #[clap(long = "dry-run", short = 'n')]
    pub(crate) dry_run: bool,
}

impl CommandExecute for Upgrade {
    #[tracing::instrument(level = "error", skip(self))]
    fn execute(self) -> eyre::Result<()> {
        let path = self.manifest_path.map_or_else(|| PathBuf::from("./Cargo.toml"), |p| p.clone());
        let path = path
            .canonicalize()
            .map_err(|e| eyre!("Error opening manifest {path:?}: {e} (see --manifest-path)"))?;
        let path = find_manifest_file(&path, self.package.as_ref())?;

        let version = if let Some(v) = self.to {
            TargetVersion::Match(v)
        } else {
            // crates-index defaults to the cache directory name Cargo used before 1.85.
            let index = SparseIndex::from_url_with_hash_kind(sparse::URL, &HashKind::Stable)?;
            if self.include_prereleases {
                TargetVersion::DiscoverPrerelease(index)
            } else {
                TargetVersion::DiscoverReleased(index)
            }
        };

        let updated = process_manifest_file(&path, &version)?;

        if self.dry_run {
            Ok(println!("{}", updated.to_string()))
        } else {
            fs::write(&path, updated.to_string())
                .map_err(|err| eyre!("Unable to write the updated Cargo.toml to disk: {err}"))
        }
    }
}

#[derive(Debug)]
enum TargetVersion {
    DiscoverPrerelease(SparseIndex),
    DiscoverReleased(SparseIndex),
    Match(VersionReq),
}

// Strategy for rewriting this dependency's version field. Encoding the two cases as a type lets each match arm own exactly one representation, instead of threading an Option<String> + tupl match across two stages.
#[derive(Debug)]
enum Rewrite {
    /// Overwrite the version field with this literal string,  Used when the user supplies a full Cargo version requirement.
    Replace(String),
    /// Substitute only the bare version number in the existing  requirement, preserving any leading operator (`^`, `=`, `~`).
    SwapBareVersion(String),
}

/// Starting at path, search for the Cargo manifest containing package.
#[tracing::instrument(level = "error")]
fn find_manifest_file(path: &PathBuf, package: Option<&String>) -> eyre::Result<PathBuf> {
    let (manifest_dirpath, manifest_filepath) = if fs::metadata(&path)?.is_dir() {
        (path.clone(), path.join("Cargo.toml"))
    } else {
        (path.parent().expect("parent").to_path_buf(), path.clone())
    };

    let input = Manifest::from_path(&manifest_filepath)
        .map_err(|e| eyre!("Error opening manifest: {e}"))?;

    match (package, &input.workspace) {
        // Without a package argument, the manifest argument is already correct.
        (None, _) => Ok(manifest_filepath),

        // With a package argument and no workspace, check the name in the manifest.
        (Some(name), None) => {
            if let Some(input_package) = &input.package
                && input_package.name().to_lowercase() == name.to_lowercase()
            {
                Ok(manifest_filepath)
            } else {
                Err(eyre!("No package {name:?} in {:?}", manifest_filepath.file_name()))
            }
        }

        // With a package argument in a workspace, search among the workspace members.
        // Report errors from each member when the package is not found.
        (Some(name), Some(workspace)) => {
            let mut errs = Vec::with_capacity(workspace.members.len());

            for dir in &workspace.members {
                match find_manifest_file(&manifest_dirpath.join(dir), package) {
                    Ok(v) => return Ok(v),
                    Err(e) => errs.push(format!("- {dir}: {e}")),
                }
            }

            Err(eyre!("No package {name:?} in {manifest_filepath:?}:\n{}", errs.join("\n")))
        }
    }
}

/// Load the Cargo manifest at path and return a copy with updated dependency versions.
#[tracing::instrument(level = "error")]
fn process_manifest_file(path: &PathBuf, version: &TargetVersion) -> eyre::Result<DocumentMut> {
    let input = Manifest::from_path(&path).map_err(|e| eyre!("Error opening manifest: {e}"))?;

    let mut output: DocumentMut = fs::read_to_string(&path)
        .map_err(|e| eyre!("Error opening manifest: {e}"))?
        .parse()
        .map_err(|e| eyre!("Error parsing manifest: {e}"))?;

    update_manifest(&input, &mut output, &version)?;

    Ok(output)
}

/// Update versions in the "dependencies", "dev-dependencies", and "workspace.dependencies" sections
/// of a Cargo manifest loaded into source and sink.
#[tracing::instrument(level = "error")]
fn update_manifest(
    source: &Manifest,
    sink: &mut DocumentMut,
    target: &TargetVersion,
) -> eyre::Result<()> {
    if !source.dependencies.is_empty() {
        let section = sink["dependencies"].as_table_like_mut();
        let section = section.expect("source and sink diverged");

        update_manifest_section(section, &source.dependencies, target)?;
    }

    if !source.dev_dependencies.is_empty() {
        let section = sink["dev-dependencies"].as_table_like_mut();
        let section = section.expect("source and sink diverged");

        update_manifest_section(section, &source.dev_dependencies, target)?;
    }

    if let Some(workspace) = &source.workspace
        && !workspace.dependencies.is_empty()
    {
        let section = sink["workspace"]["dependencies"].as_table_like_mut();
        let section = section.expect("source and sink diverged");

        update_manifest_section(section, &workspace.dependencies, target)?;
    }

    Ok(())
}

/// Update dependency versions in a single section of a Cargo manifest.
fn update_manifest_section<T: toml_edit::TableLike + ?Sized>(
    section: &mut T,
    dependencies: &DepsSet,
    target: &TargetVersion,
) -> eyre::Result<()> {
    static RELEVANT_PACKAGES: LazyLock<HashSet<&str>> = LazyLock::new(|| {
        HashSet::from([
            "pgrx",
            "pgrx-bench",
            "pgrx-macros",
            "pgrx-pg-config",
            "pgrx-pg-sys",
            "pgrx-sql-entity-graph",
            "pgrx-tests",
        ])
    });

    // Regex to capture any operators in the manifest version requirement.
    static REQUIREMENT_REGEX: LazyLock<regex::Regex> =
        LazyLock::new(|| regex::Regex::new(r"^([^0-9.]*)([0-9.].*)$").unwrap());

    for (local_name, dependency) in dependencies {
        let crate_name = match dependency {
            Dependency::Inherited(_) => continue,
            Dependency::Simple(_) => local_name,
            Dependency::Detailed(detail) => {
                if let Some(crate_name) = &detail.package {
                    crate_name
                } else {
                    local_name
                }
            }
        };

        if !RELEVANT_PACKAGES.contains(crate_name.as_str()) {
            continue;
        }

        if dependency_inherits_workspace(section, local_name) {
            continue;
        }

        if let Dependency::Detailed(detail) = dependency
            && let Some(registry) = &detail.registry
        {
            unimplemented!("custom registry {registry:?}")
        }

        let strategy: Rewrite = match target {
            TargetVersion::Match(req) => Rewrite::Replace(req.to_string()),
            TargetVersion::DiscoverPrerelease(index) => {
                let krate = index.crate_from_cache(crate_name)?;
                let highest = krate.highest_version();
                let bare = (!highest.is_yanked())
                    .then_some(highest.version())
                    .ok_or_else(|| {
                        eyre!("Latest version {:?} for {crate_name:?} is yanked", highest.version())
                    })?
                    .to_owned();
                Rewrite::SwapBareVersion(bare)
            }
            TargetVersion::DiscoverReleased(index) => {
                let krate = index.crate_from_cache(crate_name)?;
                let highest = krate.highest_normal_version();
                let bare = highest
                    .ok_or_else(|| eyre!("No released version for {crate_name:?}"))?
                    .version()
                    .to_owned();
                Rewrite::SwapBareVersion(bare)
            }
        };

        let rewrite = |existing: &str| -> String {
            match &strategy {
                Rewrite::Replace(s) => s.to_string(),
                Rewrite::SwapBareVersion(v) => {
                    REQUIREMENT_REGEX.replace(existing, format!("{}{}", "${1}", v)).into_owned()
                }
            }
        };

        match dependency {
            Dependency::Inherited(_) => continue,
            Dependency::Simple(requirement) => {
                let next_requirement = rewrite(requirement);
                section.insert(&local_name, next_requirement.into());
            }
            Dependency::Detailed(detail) => {
                let Some(requirement) = &detail.version else {
                    info!("No version specified for {local_name}, not upgrading.");
                    continue;
                };
                let next_requirement = rewrite(requirement);
                let table = section.get_mut(&local_name).expect("source and sink diverged");
                let table = table.as_table_like_mut().expect("source and sink diverged");
                table.insert("version", next_requirement.into());
            }
        }
    }

    Ok(())
}

fn dependency_inherits_workspace<T: toml_edit::TableLike + ?Sized>(
    section: &T,
    local_name: &str,
) -> bool {
    section
        .get(local_name)
        .and_then(|item| item.as_table_like())
        .and_then(|table| table.get("workspace"))
        .and_then(|workspace| workspace.as_bool())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use cargo_toml;
    use goldenfile;
    use std::fs;

    #[test]
    fn find_package_manifest() {
        let root_path = env!("CARGO_MANIFEST_DIR");
        let fixtures_path = format!("{root_path}/tests/fixtures/package");
        let manifest_path = format!("{fixtures_path}/expected-0.16.0.toml").into();

        let parsed = cargo_toml::Manifest::from_path(&manifest_path).expect("fixture exists");
        assert!(parsed.package.is_some() && parsed.workspace.is_none(), "package manifest");

        let found = super::find_manifest_file(&manifest_path, None).unwrap();
        assert_eq!(found, manifest_path);

        let found = super::find_manifest_file(&manifest_path, Some(&"package".to_owned())).unwrap();
        assert_eq!(found, manifest_path);

        let _ = super::find_manifest_file(&manifest_path, Some(&"other".to_owned())).unwrap_err();
    }

    #[test]
    fn find_package_manifest_in_workspace() {
        let root_path = env!("CARGO_MANIFEST_DIR");
        let fixtures_path = format!("{root_path}/tests/fixtures/workspace");
        let manifest_path = format!("{fixtures_path}/Cargo.toml").into();

        let parsed = cargo_toml::Manifest::from_path(&manifest_path).expect("fixture exists");
        assert!(parsed.package.is_none() && parsed.workspace.is_some(), "workspace manifest");

        let found = super::find_manifest_file(&manifest_path, None).unwrap();
        assert_eq!(found, manifest_path);

        let found = super::find_manifest_file(&manifest_path, Some(&"hello".to_owned())).unwrap();
        assert_eq!(found, format!("{fixtures_path}/hello/Cargo.toml"));

        let _ = super::find_manifest_file(&manifest_path, Some(&"other".to_owned())).unwrap_err();
    }

    #[test]
    fn process_package_manifest() {
        let root_path = env!("CARGO_MANIFEST_DIR");
        let fixtures_path = format!("{root_path}/tests/fixtures/package");
        let manifest_path = format!("{fixtures_path}/expected-0.16.0.toml");
        let mut golden = goldenfile::Mint::new(&fixtures_path);

        let parsed = cargo_toml::Manifest::from_path(&manifest_path).expect("fixture exists");
        assert!(parsed.package.is_some() && parsed.workspace.is_none(), "package manifest");

        let updated = super::process_manifest_file(
            &manifest_path.into(),
            &super::TargetVersion::Match(semver::VersionReq::parse("=0.16.1").unwrap()),
        )
        .unwrap();

        fs::write(golden.new_goldenpath("expected-0.16.1.toml").unwrap(), updated.to_string())
            .unwrap();
    }

    #[test]
    fn process_workspace_manifest() {
        let root_path = env!("CARGO_MANIFEST_DIR");
        let fixtures_path = format!("{root_path}/tests/fixtures/workspace");
        let manifest_path = format!("{fixtures_path}/Cargo.toml");
        let mut golden = goldenfile::Mint::new(&fixtures_path);

        let parsed = cargo_toml::Manifest::from_path(&manifest_path).expect("fixture exists");
        assert!(parsed.package.is_none() && parsed.workspace.is_some(), "workspace manifest");

        let updated = super::process_manifest_file(
            &manifest_path.into(),
            &super::TargetVersion::Match(semver::VersionReq::parse("=0.18.1").unwrap()),
        )
        .unwrap();

        fs::write(golden.new_goldenpath("expected-0.18.1.toml").unwrap(), updated.to_string())
            .unwrap();
    }

    #[test]
    fn process_workspace_package_manifest() {
        let root_path = env!("CARGO_MANIFEST_DIR");
        let fixtures_path = format!("{root_path}/tests/fixtures/workspace/hello");
        let manifest_path = format!("{fixtures_path}/Cargo.toml");
        let mut golden = goldenfile::Mint::new(&fixtures_path);

        let parsed = cargo_toml::Manifest::from_path(&manifest_path).expect("fixture exists");
        assert!(parsed.package.is_some() && parsed.workspace.is_none(), "package manifest");

        let updated = super::process_manifest_file(
            &manifest_path.into(),
            &super::TargetVersion::Match(semver::VersionReq::parse("=0.18.1").unwrap()),
        )
        .unwrap();

        fs::write(golden.new_goldenpath("expected-0.18.1.toml").unwrap(), updated.to_string())
            .unwrap();
    }

    use clap::Parser;
    use semver::VersionReq;

    #[derive(clap::Parser, Debug)]
    struct Wrap {
        #[clap(flatten)]
        inner: super::Upgrade,
    }

    #[test]
    fn parse_to_accepts_bare_version() {
        let parsed = Wrap::try_parse_from(["pgrx", "--to", "0.16.1"]).unwrap();
        assert_eq!(parsed.inner.to, Some(VersionReq::parse("0.16.1").unwrap()));
    }

    #[test]
    fn parse_to_accepts_partial_version() {
        // Cargo allows bare partial versions like `0.16`, treated as caret.
        let parsed = Wrap::try_parse_from(["pgrx", "--to", "0.16"]).unwrap();
        assert_eq!(parsed.inner.to, Some(VersionReq::parse("0.16").unwrap()));
    }

    #[test]
    fn parse_to_accepts_caret() {
        let parsed = Wrap::try_parse_from(["pgrx", "--to", "^0.16.1"]).unwrap();
        assert_eq!(parsed.inner.to, Some(VersionReq::parse("^0.16.1").unwrap()));
    }

    #[test]
    fn parse_to_accepts_tilde() {
        let parsed = Wrap::try_parse_from(["pgrx", "--to", "~0.16.1"]).unwrap();
        assert_eq!(parsed.inner.to, Some(VersionReq::parse("~0.16.1").unwrap()));
    }

    #[test]
    fn parse_to_accepts_exact_pin() {
        let parsed = Wrap::try_parse_from(["pgrx", "--to", "=0.16.1"]).unwrap();
        assert_eq!(parsed.inner.to, Some(VersionReq::parse("=0.16.1").unwrap()));
    }

    #[test]
    fn parse_to_accepts_range() {
        let parsed = Wrap::try_parse_from(["pgrx", "--to", ">=0.16, <0.17"]).unwrap();
        assert_eq!(parsed.inner.to, Some(VersionReq::parse(">=0.16, <0.17").unwrap()));
    }

    #[test]
    fn parse_to_accepts_prerelease() {
        let parsed = Wrap::try_parse_from(["pgrx", "--to", "=1.0.0-beta.1"]).unwrap();
        assert_eq!(parsed.inner.to, Some(VersionReq::parse("=1.0.0-beta.1").unwrap()));
    }

    #[test]
    fn parse_to_rejects_garbage() {
        let err = Wrap::try_parse_from(["pgrx", "--to", "foo"]).unwrap_err();
        let rendered = err.to_string();
        assert!(
            rendered.contains("invalid value 'foo'"),
            "expected clap error to mention the invalid value, got: {rendered}"
        );
    }

    #[test]
    fn parse_to_rejects_empty() {
        let err = Wrap::try_parse_from(["pgrx", "--to", ""]).unwrap_err();
        let rendered = err.to_string();
        assert!(
            rendered.contains("invalid value ''"),
            "expected clap error to mention the empty value, got: {rendered}"
        );
    }

    #[test]
    fn parse_to_defaults_to_none() {
        let parsed = Wrap::try_parse_from(["pgrx"]).unwrap();
        assert_eq!(parsed.inner.to, None);
    }

    #[test]
    fn parse_flags_default_to_off() {
        let upgrade = Wrap::try_parse_from(["pgrx"]).unwrap().inner;
        assert!(!upgrade.include_prereleases && !upgrade.dry_run);
        assert!(upgrade.manifest_path.is_none() && upgrade.package.is_none());
    }

    #[test]
    fn parse_accepts_every_flag() {
        let args = ["pgrx", "--include-prereleases", "-n", "-m", "a/Cargo.toml", "-p", "pkg"];
        let upgrade = Wrap::try_parse_from(args).unwrap().inner;
        assert!(upgrade.include_prereleases && upgrade.dry_run);
        assert_eq!(upgrade.manifest_path, Some("a/Cargo.toml".into()));
        assert_eq!(upgrade.package.as_deref(), Some("pkg"));
    }

    // Version discovery reads a fake crates.io index cache from a temporary Cargo home.
    use crate::CommandExecute;
    use crates_index::{HashKind, SparseIndex, sparse};
    use std::path::Path;

    const CURRENT_INDEX_DIR: &str = "index.crates.io-1949cf8c6b5b557f";
    const LEGACY_INDEX_DIR: &str = "index.crates.io-6f17d22bba15001f";

    /// 0.19.4 is yanked and 0.20.0-beta.1 is a prerelease, so 0.19.3 is the latest normal release.
    const RELEASES: &[(&str, bool)] = &[
        ("0.18.1", false),
        ("0.19.2", false),
        ("0.19.3", false),
        ("0.19.4", true),
        ("0.20.0-beta.1", false),
    ];

    const PGRX_CRATES: [&str; 5] =
        ["pgrx", "pgrx-macros", "pgrx-pg-sys", "pgrx-pg-config", "pgrx-tests"];

    const PGRX_ONLY: &str =
        "[package]\nname = \"ext\"\nversion = \"0.1.0\"\n\n[dependencies]\npgrx = \"=0.18.1\"\n";

    const FULL_MANIFEST: &str = r#"# keep this comment
[package]
name = "ext"
version = "0.1.0"

[dependencies]
pgrx = "=0.18.1"
pgrx-macros = "~0.18.1"
pgrx-pg-sys = { version = "0.18.1", default-features = false }
renamed = { package = "pgrx-pg-config", version = "^0.18" }
serde_json = "1.0.100"

[dev-dependencies]
pgrx-tests = "=0.18.1"
"#;

    /// Write `name`'s entry in the file format Cargo uses for its sparse index cache.
    fn write_index_entry(
        cargo_home: &Path,
        index_dir: &str,
        name: &str,
        releases: &[(&str, bool)],
    ) {
        let mut entry = vec![3u8]; // cache format version
        entry.extend_from_slice(&2u32.to_le_bytes()); // index format version
        entry.extend_from_slice(b"etag:test\0");
        for (vers, yanked) in releases {
            let cksum = "0".repeat(64);
            let json = format!(
                r#"{{"name":"{name}","vers":"{vers}","deps":[],"cksum":"{cksum}","features":{{}},"yanked":{yanked}}}"#
            );
            entry.extend_from_slice(format!("{vers}\0{json}\0").as_bytes());
        }
        let dir = cargo_home.join("registry/index").join(index_dir).join(".cache");
        let dir = dir.join(&name[..2]).join(&name[2..4]);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join(name), entry).unwrap();
    }

    /// A Cargo home whose `index_dir` index has `releases` of every pgrx crate.
    fn cargo_home_with(index_dir: &str, releases: &[(&str, bool)]) -> tempfile::TempDir {
        let home = tempfile::tempdir().unwrap();
        for name in PGRX_CRATES {
            write_index_entry(home.path(), index_dir, name, releases);
        }
        home
    }

    /// The index `cargo pgrx upgrade` opens, but rooted at `cargo_home`.
    fn index_in(cargo_home: &Path) -> SparseIndex {
        SparseIndex::with_path_and_hash_kind(cargo_home, sparse::URL, &HashKind::Stable).unwrap()
    }

    fn released_in(cargo_home: &Path) -> super::TargetVersion {
        super::TargetVersion::DiscoverReleased(index_in(cargo_home))
    }

    fn prerelease_in(cargo_home: &Path) -> super::TargetVersion {
        super::TargetVersion::DiscoverPrerelease(index_in(cargo_home))
    }

    fn upgraded(manifest: &str, target: &super::TargetVersion) -> eyre::Result<String> {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("Cargo.toml");
        fs::write(&path, manifest).unwrap();
        super::process_manifest_file(&path, target).map(|updated| updated.to_string())
    }

    /// Check that `FULL_MANIFEST` was rewritten using `RELEASES`, leaving unrelated lines alone.
    fn assert_upgraded_to_0_19_3(manifest: &str) {
        for expected in [
            r#"pgrx = "=0.19.3""#,
            r#"pgrx-macros = "~0.19.3""#,
            r#"pgrx-pg-sys = { version = "0.19.3", default-features = false }"#,
            r#"renamed = { package = "pgrx-pg-config", version = "^0.19.3" }"#,
            r#"pgrx-tests = "=0.19.3""#,
            r#"serde_json = "1.0.100""#,
            "# keep this comment",
        ] {
            assert!(manifest.contains(expected), "missing `{expected}` in:\n{manifest}");
        }
        for stale in ["0.18", "0.19.2", "0.19.4", "0.20.0", "beta"] {
            assert!(!manifest.contains(stale), "unexpected `{stale}` in:\n{manifest}");
        }
    }

    /// Check that `FULL_MANIFEST` was rewritten by `--to =0.19.3`, which replaces every operator.
    fn assert_pinned_to_0_19_3(manifest: &str) {
        for expected in [
            r#"pgrx = "=0.19.3""#,
            r#"pgrx-macros = "=0.19.3""#,
            r#"pgrx-pg-sys = { version = "=0.19.3", default-features = false }"#,
            r#"renamed = { package = "pgrx-pg-config", version = "=0.19.3" }"#,
            r#"pgrx-tests = "=0.19.3""#,
            r#"serde_json = "1.0.100""#,
            "# keep this comment",
        ] {
            assert!(manifest.contains(expected), "missing `{expected}` in:\n{manifest}");
        }
        assert!(!manifest.contains("0.18"), "unexpected `0.18` in:\n{manifest}");
    }

    #[test]
    fn discover_released_picks_the_highest_normal_release() {
        let home = cargo_home_with(CURRENT_INDEX_DIR, RELEASES);
        let updated = upgraded(FULL_MANIFEST, &released_in(home.path())).unwrap();
        assert_upgraded_to_0_19_3(&updated);
    }

    #[test]
    fn discover_prerelease_picks_the_highest_version() {
        let home = cargo_home_with(CURRENT_INDEX_DIR, RELEASES);
        let updated = upgraded(PGRX_ONLY, &prerelease_in(home.path())).unwrap();
        assert!(updated.contains(r#"pgrx = "=0.20.0-beta.1""#), "unexpected manifest:\n{updated}");
    }

    #[test]
    fn discover_prerelease_rejects_a_yanked_highest_version() {
        let home = cargo_home_with(CURRENT_INDEX_DIR, &[("0.19.3", false), ("0.19.4", true)]);

        let err = upgraded(PGRX_ONLY, &prerelease_in(home.path())).unwrap_err().to_string();
        assert!(err.contains("0.19.4") && err.contains("is yanked"), "unexpected error: {err}");

        let updated = upgraded(PGRX_ONLY, &released_in(home.path())).unwrap();
        assert!(updated.contains(r#"pgrx = "=0.19.3""#), "unexpected manifest:\n{updated}");
    }

    #[test]
    fn discover_released_requires_a_normal_release() {
        let home =
            cargo_home_with(CURRENT_INDEX_DIR, &[("0.20.0-beta.1", false), ("0.19.4", true)]);
        let err = upgraded(PGRX_ONLY, &released_in(home.path())).unwrap_err().to_string();
        assert!(err.contains("No released version"), "unexpected error: {err}");
    }

    #[test]
    fn discover_names_the_missing_cache_entry() {
        let home = tempfile::tempdir().unwrap();
        let err = upgraded(PGRX_ONLY, &released_in(home.path())).unwrap_err().to_string();
        assert!(err.contains(CURRENT_INDEX_DIR) && err.contains("pgrx"), "unexpected error: {err}");
    }

    #[test]
    fn discover_ignores_the_pre_1_85_cache_directory() {
        let home = cargo_home_with(LEGACY_INDEX_DIR, RELEASES);
        let err = upgraded(PGRX_ONLY, &released_in(home.path())).unwrap_err().to_string();
        assert!(err.contains(CURRENT_INDEX_DIR), "unexpected error: {err}");
        assert!(!err.contains(LEGACY_INDEX_DIR), "unexpected error: {err}");
    }

    #[test]
    fn discover_uses_the_current_directory_when_both_exist() {
        let home = cargo_home_with(CURRENT_INDEX_DIR, RELEASES);
        for name in PGRX_CRATES {
            write_index_entry(home.path(), LEGACY_INDEX_DIR, name, &[("0.18.1", false)]);
        }
        let updated = upgraded(FULL_MANIFEST, &released_in(home.path())).unwrap();
        assert_upgraded_to_0_19_3(&updated);
    }

    #[test]
    fn discover_rejects_a_corrupt_cache_entry() {
        let home = tempfile::tempdir().unwrap();
        write_index_entry(home.path(), CURRENT_INDEX_DIR, "pgrx", RELEASES);
        let entry = home.path().join("registry/index").join(CURRENT_INDEX_DIR);
        fs::write(entry.join(".cache/pg/rx/pgrx"), b"\x03not an index entry").unwrap();

        assert!(upgraded(PGRX_ONLY, &released_in(home.path())).is_err());
    }

    #[test]
    fn discover_updates_workspace_dependencies() {
        let home = cargo_home_with(CURRENT_INDEX_DIR, RELEASES);
        let manifest = "[workspace]\nmembers = []\n\n[workspace.dependencies]\npgrx = \"=0.18.1\"\npgrx-tests = \"^0.18\"\n";
        let updated = upgraded(manifest, &released_in(home.path())).unwrap();
        assert!(updated.contains(r#"pgrx = "=0.19.3""#), "unexpected manifest:\n{updated}");
        assert!(updated.contains(r#"pgrx-tests = "^0.19.3""#), "unexpected manifest:\n{updated}");
    }

    #[test]
    fn match_target_needs_no_cache() {
        let target = super::TargetVersion::Match(VersionReq::parse("=0.19.3").unwrap());
        let updated = upgraded(FULL_MANIFEST, &target).unwrap();
        assert_pinned_to_0_19_3(&updated);
    }

    fn upgrade_of(manifest: &Path, to: Option<&str>, dry_run: bool) -> super::Upgrade {
        super::Upgrade {
            to: to.map(|to| VersionReq::parse(to).unwrap()),
            manifest_path: Some(manifest.to_owned()),
            include_prereleases: false,
            package: None,
            dry_run,
        }
    }

    #[test]
    fn execute_writes_the_manifest_unless_dry_run() {
        let dir = tempfile::tempdir().unwrap();
        let manifest = dir.path().join("Cargo.toml");
        fs::write(&manifest, FULL_MANIFEST).unwrap();

        upgrade_of(&manifest, Some("=0.19.3"), true).execute().unwrap();
        assert_eq!(fs::read_to_string(&manifest).unwrap(), FULL_MANIFEST, "dry run must not write");

        upgrade_of(&manifest, Some("=0.19.3"), false).execute().unwrap();
        assert_pinned_to_0_19_3(&fs::read_to_string(&manifest).unwrap());
    }

    #[test]
    fn execute_names_a_missing_manifest() {
        let dir = tempfile::tempdir().unwrap();
        let manifest = dir.path().join("nope/Cargo.toml");

        let err = upgrade_of(&manifest, Some("=0.19.3"), true).execute().unwrap_err().to_string();
        assert!(err.contains("Error opening manifest") && err.contains("nope/Cargo.toml"), "{err}");
        assert!(err.contains("--manifest-path"), "{err}");
    }

    #[test]
    fn execute_selects_a_package_in_a_workspace() {
        let dir = tempfile::tempdir().unwrap();
        let member = dir.path().join("member");
        fs::create_dir_all(member.join("src")).unwrap();
        fs::write(dir.path().join("Cargo.toml"), "[workspace]\nmembers = [\"member\"]\n").unwrap();
        fs::write(member.join("src/lib.rs"), "").unwrap();
        fs::write(member.join("Cargo.toml"), FULL_MANIFEST.replace("\"ext\"", "\"member\""))
            .unwrap();

        let mut upgrade = upgrade_of(&dir.path().join("Cargo.toml"), Some("=0.19.3"), false);
        upgrade.package = Some("member".into());
        upgrade.execute().unwrap();

        assert_pinned_to_0_19_3(&fs::read_to_string(member.join("Cargo.toml")).unwrap());
    }
}
