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
use crates_index::SparseIndex;
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
        let path = find_manifest_file(&path.canonicalize()?, self.package.as_ref())?;

        let version = if let Some(v) = self.to {
            TargetVersion::Match(v)
        } else if self.include_prereleases {
            TargetVersion::DiscoverPrerelease
        } else {
            TargetVersion::DiscoverReleased
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
    DiscoverPrerelease,
    DiscoverReleased,
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

        let index = match dependency {
            Dependency::Inherited(_) => continue,
            Dependency::Simple(_) => SparseIndex::new_cargo_default()?,
            Dependency::Detailed(detail) => {
                if let Some(registry) = &detail.registry {
                    unimplemented!("custom registry {registry:?}")
                } else {
                    SparseIndex::new_cargo_default()?
                }
            }
        };

        let strategy: Rewrite = match target {
            TargetVersion::Match(req) => Rewrite::Replace(req.to_string()),
            TargetVersion::DiscoverPrerelease => {
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
            TargetVersion::DiscoverReleased => {
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
}
