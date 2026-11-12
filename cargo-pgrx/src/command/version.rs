//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use eyre::WrapErr;
use owo_colors::OwoColorize;
use pgrx_pg_config::{PgConfig, PgMinorVersion, PgVersion, Pgrx, SUPPORTED_VERSIONS};
use serde::Deserialize;
use std::collections::BTreeMap;

/// postgresql.org's list of PostgreSQL major versions and their latest minor releases.
const VERSIONS_URL: &str = "https://www.postgresql.org/versions.json";

pub(crate) fn pgrx_default() -> eyre::Result<Pgrx> {
    let http_client = crate::command::http_agent();
    let body = crate::command::download_with_retries(VERSIONS_URL, || {
        Ok(http_client.get(VERSIONS_URL).call()?.body_mut().read_to_vec()?)
    })
    .wrap_err_with(|| format!("unable to retrieve {VERSIONS_URL}"))?;
    let releases: Vec<Release> = serde_json::from_slice(&body)
        .wrap_err_with(|| format!("unable to parse {VERSIONS_URL}"))?;
    let versions = with_latest_releases(&SUPPORTED_VERSIONS(), &releases, |major, minor| {
        source_release_exists(&http_client, major, minor)
    })?;

    println!(
        "{} Postgres {}",
        "   Discovered".white().bold(),
        versions.iter().map(|version| format!("v{version}")).collect::<Vec<_>>().join(", ")
    );

    let mut pgrx = Pgrx::default();
    versions.into_iter().for_each(|version| pgrx.push(PgConfig::from(version)));
    Ok(pgrx)
}

/// An entry in [`VERSIONS_URL`], such as `{"major": "18", "latestMinor": "6", ...}`.
#[derive(Deserialize)]
struct Release {
    major: String,
    #[serde(rename = "latestMinor")]
    latest_minor: String,
}

/// `supported`, in major version order, where each version that asks for the latest minor
/// release gets the one `releases` names and its download URL. `release_exists` reports
/// whether a `major.minor` release exists, which restores a minor whose trailing zeros
/// `releases` dropped.
fn with_latest_releases(
    supported: &[PgVersion],
    releases: &[Release],
    mut release_exists: impl FnMut(u16, u16) -> eyre::Result<bool>,
) -> eyre::Result<Vec<PgVersion>> {
    let mut versions: BTreeMap<u16, PgVersion> =
        supported.iter().map(|version| (version.major, version.clone())).collect();
    for release in releases {
        // Majors before PostgreSQL 10, such as "9.6", are not numbers and not supported.
        let (Ok(major), Ok(mut minor)) =
            (release.major.parse(), release.latest_minor.parse::<u16>())
        else {
            continue;
        };
        if let Some(version) = versions.get_mut(&major)
            && matches!(version.minor, PgMinorVersion::Latest)
        {
            // postgresql.org has published a minor with its trailing zero dropped, such as
            // "2" for 15.20. Minor releases only increase, so when `{minor}0` exists,
            // `minor` is not the latest one.
            while minor > 0
                && let Some(longer) = minor.checked_mul(10)
                && release_exists(major, longer)?
            {
                minor = longer;
            }
            version.minor = PgMinorVersion::Release(minor);
            version.url = Some(download_url(major, minor));
        }
    }
    Ok(versions.into_values().collect())
}

/// Whether postgresql.org publishes the source of PostgreSQL `major.minor`. Windows downloads
/// binaries from EnterpriseDB, which can lag a release by days, so this checks the source.
fn source_release_exists(http_client: &ureq::Agent, major: u16, minor: u16) -> eyre::Result<bool> {
    let url = source_url(major, minor);
    crate::command::download_with_retries(&url, || match http_client.head(&url).call() {
        Ok(_) => Ok(true),
        Err(ureq::Error::StatusCode(404)) => Ok(false),
        Err(error) => Err(error.into()),
    })
    .wrap_err_with(|| format!("unable to check for {url}"))
}

fn source_url(major: u16, minor: u16) -> String {
    format!(
        "https://ftp.postgresql.org/pub/source/v{major}.{minor}/postgresql-{major}.{minor}.tar.gz"
    )
}

#[cfg(not(target_os = "windows"))]
fn download_url(major: u16, minor: u16) -> String {
    source_url(major, minor)
}

#[cfg(target_os = "windows")]
fn download_url(major: u16, minor: u16) -> String {
    format!(
        "https://get.enterprisedb.com/postgresql/postgresql-{major}.{minor}-1-windows-x64-binaries.zip"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn latest_releases_fill_in_versions_that_ask_for_them() {
        let releases: Vec<Release> = serde_json::from_str(
            r#"[
                {"current": false, "latestMinor": "2", "major": "6.3", "supported": false},
                {"current": false, "latestMinor": "11", "major": "17", "supported": true},
                {"current": true, "latestMinor": "6", "major": "18", "supported": true}
            ]"#,
        )
        .unwrap();
        let rc = PgVersion::new(19, PgMinorVersion::Rc(1), Some("https://example/19rc1".into()));
        let supported = [
            rc.clone(),
            PgVersion::new(18, PgMinorVersion::Latest, None),
            PgVersion::new(17, PgMinorVersion::Latest, None),
            PgVersion::new(16, PgMinorVersion::Latest, None),
        ];

        let versions = with_latest_releases(&supported, &releases, |_, _| Ok(false)).unwrap();

        let expected = [
            PgVersion::new(16, PgMinorVersion::Latest, None),
            PgVersion::new(17, PgMinorVersion::Release(11), Some(download_url(17, 11))),
            PgVersion::new(18, PgMinorVersion::Release(6), Some(download_url(18, 6))),
            rc,
        ];
        assert_eq!(versions, expected);
    }

    /// On 2026-11-12, versions.json listed PostgreSQL 15's latest minor as "2" for 15.20.
    #[test]
    fn latest_releases_restore_dropped_trailing_zeros() {
        let releases: Vec<Release> = serde_json::from_str(
            r#"[
                {"latestMinor": "2", "major": "15"},
                {"latestMinor": "16", "major": "16"},
                {"latestMinor": "0", "major": "18"}
            ]"#,
        )
        .unwrap();
        let supported =
            [15, 16, 18].map(|major| PgVersion::new(major, PgMinorVersion::Latest, None));
        let mut checked = Vec::new();

        let versions = with_latest_releases(&supported, &releases, |major, minor| {
            checked.push((major, minor));
            Ok((major, minor) == (15, 20))
        })
        .unwrap();

        let expected = [
            PgVersion::new(15, PgMinorVersion::Release(20), Some(download_url(15, 20))),
            PgVersion::new(16, PgMinorVersion::Release(16), Some(download_url(16, 16))),
            PgVersion::new(18, PgMinorVersion::Release(0), Some(download_url(18, 0))),
        ];
        assert_eq!(versions, expected);
        // A ".0" release has no zero to restore, so 18 is never checked.
        assert_eq!(checked, [(15, 20), (15, 200), (16, 160)]);
    }
}
