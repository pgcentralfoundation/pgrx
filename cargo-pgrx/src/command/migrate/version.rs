//LICENSE Portions Copyright 2026-2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use eyre::{Context, eyre};
use regex::{NoExpand, Regex};
use semver::Version;
use serde::{Serialize, Serializer};
use std::cmp::Ordering;
use std::fmt;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

/// An extension version.
///
/// Postgres treats version names as opaque labels, so `1.1` and `1.1.0` are unrelated versions to
/// it and an upgrade script only connects the exact labels in its filename. The label is therefore
/// kept exactly as written, and `semver` is used for nothing but ordering.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExtVersion {
    label: String,
    semver: Version,
}

impl ExtVersion {
    /// Reads a version name from a filename or the control file.
    pub(crate) fn parse(label: &str) -> Option<Self> {
        let label = label.trim();
        let bare = label.strip_prefix('v').unwrap_or(label);
        let (core, suffix) = bare.split_at(bare.find(['-', '+']).unwrap_or(bare.len()));
        let padding = match core.split('.').count() {
            1 => ".0.0",
            2 => ".0",
            _ => "",
        };
        let semver = Version::parse(&format!("{core}{padding}{suffix}")).ok()?;
        Some(Self { label: label.to_string(), semver })
    }

    /// Reads a version given on the command line, where `v0.26.0` is a tag's spelling of `0.26.0`.
    pub(crate) fn from_arg(arg: &str) -> eyre::Result<Self> {
        let arg = arg.trim();
        Self::parse(arg.strip_prefix('v').unwrap_or(arg))
            .ok_or_else(|| eyre!("`{arg}` is not a version cargo-pgrx can order"))
    }

    pub(crate) fn next_patch(&self) -> eyre::Result<Self> {
        let patch = self
            .semver
            .patch
            .checked_add(1)
            .ok_or_else(|| eyre!("there is no patch version after `{self}`"))?;
        let semver = Version::new(self.semver.major, self.semver.minor, patch);
        Ok(Self { label: semver.to_string(), semver })
    }

    fn is_prerelease(&self) -> bool {
        !self.semver.pre.is_empty()
    }
}

impl Ord for ExtVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        self.semver.cmp(&other.semver).then_with(|| self.label.cmp(&other.label))
    }
}

impl PartialOrd for ExtVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for ExtVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.label)
    }
}

impl Serialize for ExtVersion {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

/// An `<ext>--<prev>--<target>.sql` script in the crate's `sql/` directory.
#[derive(Debug, Clone)]
pub(crate) struct UpgradeScript {
    pub(crate) target: ExtVersion,
    pub(crate) path: PathBuf,
}

/// Finds the upgrade scripts in `sql_dir`, oldest target first.
pub(crate) fn existing_upgrade_scripts(
    sql_dir: &Path,
    extname: &str,
) -> eyre::Result<Vec<UpgradeScript>> {
    let entries = match fs::read_dir(sql_dir) {
        Ok(entries) => entries,
        Err(err) if err.kind() == ErrorKind::NotFound => return Ok(Vec::new()),
        Err(err) => {
            return Err(err).wrap_err_with(|| format!("failed to read `{}`", sql_dir.display()));
        }
    };

    let prefix = format!("{extname}--");
    let mut scripts = Vec::new();
    for entry in entries {
        let path = entry?.path();
        let target = path
            .file_name()
            .and_then(|name| name.to_str())
            .and_then(|name| name.strip_prefix(&prefix)?.strip_suffix(".sql")?.rsplit_once("--"))
            .and_then(|(_, target)| ExtVersion::parse(target));

        if let Some(target) = target
            && path.is_file()
        {
            scripts.push(UpgradeScript { target, path });
        }
    }

    scripts.sort_by(|a, b| a.target.cmp(&b.target));
    Ok(scripts)
}

/// The version after `latest`: `current` once it has moved past `latest`, or else the next patch.
pub(crate) fn next_version(latest: &ExtVersion, current: &ExtVersion) -> eyre::Result<ExtVersion> {
    if current > latest { Ok(current.clone()) } else { latest.next_patch() }
}

pub(crate) fn resolve_target_version(
    scripts: &[UpgradeScript],
    explicit: Option<&str>,
    current: &ExtVersion,
) -> eyre::Result<ExtVersion> {
    match (explicit, scripts.last()) {
        (Some(target), _) => ExtVersion::from_arg(target),
        (None, Some(latest)) => next_version(&latest.target, current),
        (None, None) => Ok(current.clone()),
    }
}

/// Picks the version an upgrade script to `target` starts from.
///
/// Pre-releases are skipped because a script for a later version repeats their fragments: going
/// through one would apply the same SQL twice.
pub(crate) fn resolve_prev_version(
    scripts: &[UpgradeScript],
    target: &ExtVersion,
    explicit: Option<&str>,
    current: &ExtVersion,
) -> eyre::Result<ExtVersion> {
    let prev = match explicit {
        Some(prev) => ExtVersion::from_arg(prev)?,
        None => scripts
            .iter()
            .rev()
            .map(|script| &script.target)
            .find(|version| !version.is_prerelease() && *version < target)
            .or((current < target).then_some(current))
            .cloned()
            .ok_or_else(|| {
                eyre!(
                    "could not determine which release upgrades to `{target}`: `sql/` has no earlier upgrade script and the extension is at `{current}`.\n\
                     Pass `--prev-version <VERSION>` with the release this script upgrades from."
                )
            })?,
    };

    if prev >= *target {
        eyre::bail!("the previous version `{prev}` must be older than the target `{target}`");
    }
    Ok(prev)
}

pub(crate) fn update_control_file_default_version(
    control_file_path: &Path,
    target_version: &str,
) -> eyre::Result<()> {
    let content = fs::read_to_string(control_file_path).wrap_err_with(|| {
        format!("failed to read extension control file `{}`", control_file_path.display())
    })?;

    let line = format!("default_version = '{target_version}'");
    let re = Regex::new(r"(?m)^default_version\s*=.*$").unwrap();
    let updated = if re.is_match(&content) {
        re.replace(&content, NoExpand(&line)).to_string()
    } else {
        format!("{content}\n{line}\n")
    };

    fs::write(control_file_path, updated).wrap_err_with(|| {
        format!("failed to update default_version in `{}`", control_file_path.display())
    })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn version(label: &str) -> ExtVersion {
        ExtVersion::parse(label).unwrap()
    }

    fn scripts(dir: &Path, names: &[&str]) -> Vec<UpgradeScript> {
        for name in names {
            fs::write(dir.join(name), "-- old").unwrap();
        }
        existing_upgrade_scripts(dir, "my_ext").unwrap()
    }

    #[test]
    fn keeps_labels_exactly_as_written() {
        for label in ["1.1", "1", "0.26.0-rc.1", "1.0-rc.1", "2.0.0+build.5"] {
            assert_eq!(version(label).to_string(), label);
        }
    }

    #[test]
    fn command_line_versions_lose_a_tag_prefix() {
        assert_eq!(ExtVersion::from_arg("v0.26.0").unwrap().to_string(), "0.26.0");
        assert_eq!(ExtVersion::from_arg("v0.26.0-rc.1").unwrap().to_string(), "0.26.0-rc.1");
        assert!(ExtVersion::from_arg("nonsense").is_err());
    }

    #[test]
    fn orders_by_semver_not_text() {
        assert!(version("0.9.0") < version("0.10.0"));
        assert!(version("1.1") < version("1.1.1"));
        assert!(version("0.26.0-rc.1") < version("0.26.0"));
        assert!(version("0.25.9") < version("0.26.0-rc.1"));
        assert_ne!(version("1.1"), version("1.1.0"));
    }

    #[test]
    fn next_version_prefers_a_current_version_ahead_of_sql() {
        let latest = version("0.25.0");
        assert_eq!(next_version(&latest, &version("0.26.0")).unwrap(), version("0.26.0"));
        assert_eq!(next_version(&latest, &version("0.25.0")).unwrap(), version("0.25.1"));
        assert_eq!(
            next_version(&version("0.25.9"), &version("0.25.6")).unwrap(),
            version("0.25.10")
        );
    }

    #[test]
    fn the_largest_patch_has_no_successor() {
        let last = version("0.1.18446744073709551615");
        assert!(next_version(&last, &last).is_err());
    }

    #[test]
    fn finds_scripts_oldest_first_and_ignores_other_files() {
        let dir = tempfile::tempdir().unwrap();
        let found = scripts(
            dir.path(),
            &[
                "my_ext--0.9.0--0.10.0.sql",
                "my_ext--0.8.0--0.9.0.sql",
                "my_ext--0.10.0.sql",
                "other--0.9.0--9.9.9.sql",
                "my_ext--0.1.0--0.2.0.txt",
            ],
        );
        let targets: Vec<_> = found.iter().map(|s| s.target.to_string()).collect();
        assert_eq!(targets, ["0.9.0", "0.10.0"]);
    }

    #[test]
    fn a_missing_sql_directory_has_no_scripts() {
        let dir = tempfile::tempdir().unwrap();
        assert!(existing_upgrade_scripts(&dir.path().join("sql"), "my_ext").unwrap().is_empty());
    }

    #[test]
    fn resolves_target_versions() {
        let dir = tempfile::tempdir().unwrap();
        let current = version("0.25.0");
        assert_eq!(resolve_target_version(&[], None, &current).unwrap(), current);
        assert_eq!(
            resolve_target_version(&[], Some("v0.27.0"), &current).unwrap(),
            version("0.27.0")
        );

        let found = scripts(dir.path(), &["my_ext--0.25.0--0.25.9.sql"]);
        assert_eq!(
            resolve_target_version(&found, None, &version("0.26.0")).unwrap(),
            version("0.26.0")
        );
        assert_eq!(
            resolve_target_version(&found, None, &version("0.25.6")).unwrap(),
            version("0.25.10")
        );
    }

    #[test]
    fn previous_version_keeps_the_spelling_on_disk() {
        let dir = tempfile::tempdir().unwrap();
        let found = scripts(dir.path(), &["my_ext--1.0--1.1.sql"]);
        let prev =
            resolve_prev_version(&found, &version("1.2.0"), None, &version("1.2.0")).unwrap();
        assert_eq!(prev.to_string(), "1.1");
    }

    #[test]
    fn previous_version_skips_pre_releases() {
        let dir = tempfile::tempdir().unwrap();
        let found =
            scripts(dir.path(), &["my_ext--0.24.0--0.25.9.sql", "my_ext--0.25.9--0.26.0-rc.1.sql"]);
        let prev =
            resolve_prev_version(&found, &version("0.26.0"), None, &version("0.26.0")).unwrap();
        assert_eq!(prev, version("0.25.9"));
    }

    #[test]
    fn previous_version_falls_back_to_the_current_version() {
        let prev = resolve_prev_version(&[], &version("0.2.0"), None, &version("0.1.0")).unwrap();
        assert_eq!(prev, version("0.1.0"));
    }

    #[test]
    fn a_first_release_needs_an_explicit_previous_version() {
        let current = version("0.2.0");
        let err = resolve_prev_version(&[], &current, None, &current).unwrap_err();
        assert!(err.to_string().contains("--prev-version"), "{err}");

        let prev = resolve_prev_version(&[], &current, Some("0.1.0"), &current).unwrap();
        assert_eq!(prev, version("0.1.0"));
    }

    #[test]
    fn refuses_a_previous_version_that_is_not_older() {
        let current = version("0.26.0");
        assert!(resolve_prev_version(&[], &version("0.25.0"), None, &current).is_err());
        assert!(resolve_prev_version(&[], &current, Some("0.26.0"), &current).is_err());
        assert!(resolve_prev_version(&[], &version("0.25.0"), Some("0.26.0"), &current).is_err());
    }

    #[test]
    fn updates_control_file_content() {
        let temp_dir = tempfile::tempdir().unwrap();
        let control_path = temp_dir.path().join("test.control");

        fs::write(
            &control_path,
            "comment = 'test ext'\ndefault_version = '0.25.0'\nrelocatable = false\n",
        )
        .unwrap();

        update_control_file_default_version(&control_path, "0.26.0").unwrap();

        let updated = fs::read_to_string(&control_path).unwrap();
        assert!(updated.contains("default_version = '0.26.0'"));
        assert!(updated.contains("comment = 'test ext'"));
        assert!(updated.contains("relocatable = false"));
    }
}
