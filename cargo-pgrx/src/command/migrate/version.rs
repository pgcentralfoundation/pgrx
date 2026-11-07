//LICENSE Portions Copyright 2026-2026 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use eyre::{Context, eyre};
use regex::Regex;
use semver::Version;
use std::fs;
use std::path::Path;

pub fn clean_version_string(ver: &str) -> &str {
    let ver = ver.trim();
    let ver = ver.strip_prefix('v').unwrap_or(ver);
    ver.split('-').next().unwrap_or(ver)
}

pub fn parse_version_lossy(ver: &str) -> Option<Version> {
    let cleaned = clean_version_string(ver);
    Version::parse(cleaned).ok().or_else(|| {
        let parts: Vec<&str> = cleaned.split('.').collect();
        if parts.len() == 2 {
            Version::parse(&format!("{cleaned}.0")).ok()
        } else if parts.len() == 1 {
            Version::parse(&format!("{cleaned}.0.0")).ok()
        } else {
            None
        }
    })
}

pub fn get_existing_sql_targets(sql_dir: &Path, extname: &str) -> Vec<Version> {
    let mut targets = Vec::new();
    let pattern = format!(r"^{}--.+--(.+)\.sql$", regex::escape(extname));
    let re = match Regex::new(&pattern) {
        Ok(re) => re,
        Err(_) => return targets,
    };

    let entries = match fs::read_dir(sql_dir) {
        Ok(entries) => entries,
        Err(_) => return targets,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let Some(file_name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };

        if let Some(caps) = re.captures(file_name) {
            let target_str = &caps[1];
            if let Some(v) = parse_version_lossy(target_str) {
                targets.push(v);
            }
        }
    }

    targets.sort();
    targets.dedup();
    targets
}

pub fn resolve_prev_version(
    sql_dir: &Path,
    extname: &str,
    target_version: &Version,
    explicit_prev: Option<&str>,
    manifest_version: Option<&str>,
) -> eyre::Result<String> {
    if let Some(prev) = explicit_prev {
        return Ok(clean_version_string(prev).to_string());
    }

    let existing_targets = get_existing_sql_targets(sql_dir, extname);
    let candidate = existing_targets.into_iter().filter(|v| v < target_version).max();

    if let Some(prev) = candidate {
        return Ok(prev.to_string());
    }

    if let Some(manifest_ver) = manifest_version {
        let cleaned = clean_version_string(manifest_ver);
        if let Some(v) = parse_version_lossy(cleaned)
            && &v < target_version
        {
            return Ok(v.to_string());
        }
        return Ok(cleaned.to_string());
    }

    Err(eyre!(
        "could not automatically determine previous version for target `{target_version}`. Specify `--prev-version <VERSION>` explicitly."
    ))
}

pub fn resolve_target_version(
    sql_dir: &Path,
    extname: &str,
    explicit_target: Option<&str>,
    manifest_version: Option<&str>,
) -> eyre::Result<String> {
    if let Some(target) = explicit_target {
        return Ok(clean_version_string(target).to_string());
    }

    let existing_targets = get_existing_sql_targets(sql_dir, extname);
    let latest_existing = existing_targets.into_iter().max();
    let manifest_parsed =
        manifest_version.map(clean_version_string).as_deref().and_then(parse_version_lossy);

    if let Some(latest) = latest_existing {
        if let Some(ref pkg_ver) = manifest_parsed
            && pkg_ver > &latest
        {
            return Ok(pkg_ver.to_string());
        }
        let ephemeral = derive_ephemeral_target_version(&latest, manifest_parsed.as_ref());
        return Ok(ephemeral.to_string());
    }

    if let Some(manifest_ver) = manifest_version {
        return Ok(clean_version_string(manifest_ver).to_string());
    }

    Err(eyre!("could not automatically determine target version from Cargo.toml or sql/."))
}

pub fn derive_ephemeral_target_version(
    prev_version: &Version,
    manifest_version: Option<&Version>,
) -> Version {
    if let Some(pkg_ver) = manifest_version
        && pkg_ver > prev_version
    {
        return pkg_ver.clone();
    }

    Version::new(prev_version.major, prev_version.minor, prev_version.patch + 1)
}

pub fn update_control_file_default_version(
    control_file_path: &Path,
    target_version: &str,
) -> eyre::Result<()> {
    let content = fs::read_to_string(control_file_path).wrap_err_with(|| {
        format!("failed to read extension control file `{}`", control_file_path.display())
    })?;

    let re = Regex::new(r"(?m)^default_version\s*=.*$").unwrap();
    let updated = if re.is_match(&content) {
        re.replace(&content, format!("default_version = '{target_version}'")).to_string()
    } else {
        format!("{content}\ndefault_version = '{target_version}'\n")
    };

    fs::write(control_file_path, updated).wrap_err_with(|| {
        format!("failed to update default_version in `{}`", control_file_path.display())
    })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cleans_version_strings() {
        assert_eq!(clean_version_string("v0.25.0"), "0.25.0");
        assert_eq!(clean_version_string("0.25.0-rc1"), "0.25.0");
        assert_eq!(clean_version_string("v1.2.3-beta.2"), "1.2.3");
    }

    #[test]
    fn derives_ephemeral_target_version_correctly() {
        let prev = Version::parse("0.25.0").unwrap();
        let pkg_higher = Version::parse("0.26.0").unwrap();
        assert_eq!(derive_ephemeral_target_version(&prev, Some(&pkg_higher)), pkg_higher);

        let pkg_equal = Version::parse("0.25.0").unwrap();
        assert_eq!(
            derive_ephemeral_target_version(&prev, Some(&pkg_equal)),
            Version::parse("0.25.1").unwrap()
        );

        assert_eq!(derive_ephemeral_target_version(&prev, None), Version::parse("0.25.1").unwrap());
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
    }

    #[test]
    fn resolves_target_version_correctly() {
        let temp_dir = tempfile::tempdir().unwrap();
        let sql_dir = temp_dir.path().join("sql");
        fs::create_dir_all(&sql_dir).unwrap();

        // 1. Explicit target always wins
        let target =
            resolve_target_version(&sql_dir, "my_ext", Some("0.27.0"), Some("0.25.0")).unwrap();
        assert_eq!(target, "0.27.0");

        // 2. Empty sql_dir uses manifest version
        let target = resolve_target_version(&sql_dir, "my_ext", None, Some("0.1.0")).unwrap();
        assert_eq!(target, "0.1.0");

        // 3. Existing sql target lower than manifest version uses manifest version
        fs::write(sql_dir.join("my_ext--0.24.0--0.25.0.sql"), "-- old").unwrap();
        let target = resolve_target_version(&sql_dir, "my_ext", None, Some("0.26.0")).unwrap();
        assert_eq!(target, "0.26.0");

        // 4. Existing sql target greater than or equal to manifest version derives ephemeral target
        fs::write(sql_dir.join("my_ext--0.25.0--0.25.9.sql"), "-- old").unwrap();
        let target = resolve_target_version(&sql_dir, "my_ext", None, Some("0.25.6")).unwrap();
        assert_eq!(target, "0.25.10");
    }
}
