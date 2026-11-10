//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use owo_colors::OwoColorize;
use std::time::Duration;
use ureq::Agent;
use ureq::tls::{RootCerts, TlsConfig};

pub(crate) mod bench;
pub(crate) mod connect;
pub(crate) mod cross;
pub(crate) mod get;
pub(crate) mod info;
pub(crate) mod init;
pub(crate) mod install;
pub(crate) mod migrate;
pub(crate) mod new;
pub(crate) mod package;
pub(crate) mod pgrx;
mod regress;
pub(crate) mod run;
pub(crate) mod schema;
pub(crate) mod start;
pub(crate) mod status;
pub(crate) mod stop;
pub(crate) mod sudo_install;
pub(crate) mod test;
pub(crate) mod upgrade;
pub(crate) mod version;

fn reject_removed_pg_version(label: Option<&str>) -> eyre::Result<()> {
    if let Some(label @ ("pg13" | "pg14")) = label {
        return Err(eyre::eyre!(
            "Postgres `{label}` is no longer supported by pgrx; use pg15 through pg19"
        ));
    }
    Ok(())
}

/// An HTTP agent that validates certificates against the platform's certificate store, which
/// keeps working behind proxies that inspect TLS with a corporate certificate authority. Like
/// any ureq agent, it sends requests through the proxy that `ALL_PROXY`, `HTTPS_PROXY` or
/// `HTTP_PROXY` names, unless `NO_PROXY` excludes the host.
fn http_agent() -> Agent {
    let tls_config = TlsConfig::builder().root_certs(RootCerts::PlatformVerifier).build();
    Agent::new_with_config(Agent::config_builder().tls_config(tls_config).build())
}

/// The pauses between attempts at a download, which make four attempts in all.
const DOWNLOAD_RETRY_DELAYS: [Duration; 3] =
    [Duration::from_secs(1), Duration::from_secs(2), Duration::from_secs(4)];

/// Runs `download` until it succeeds or has failed four times, warning before each retry.
/// `what` names the download in the warning. Every error is retried, because postgresql.org's
/// servers have answered CI with 403s, as well as dropped connections, that a later attempt got
/// past. Each call of `download` must start over, replacing anything an earlier one wrote.
fn download_with_retries<T>(
    what: &str,
    download: impl FnMut() -> eyre::Result<T>,
) -> eyre::Result<T> {
    retry(what, &DOWNLOAD_RETRY_DELAYS, download)
}

/// [`download_with_retries`], pausing for each of `delays` in turn.
fn retry<T>(
    what: &str,
    delays: &[Duration],
    mut download: impl FnMut() -> eyre::Result<T>,
) -> eyre::Result<T> {
    let attempts = delays.len() + 1;
    for (failed_attempt, delay) in (1..).zip(delays) {
        match download() {
            Ok(downloaded) => return Ok(downloaded),
            Err(error) => {
                println!(
                    "{} {what} download attempt {failed_attempt}/{attempts} failed; retrying in {}s: {}",
                    "      Warning".bold().yellow(),
                    delay.as_secs(),
                    download_error_summary(&error),
                );
                std::thread::sleep(*delay);
            }
        }
    }
    download()
}

/// The first line of `error`, which keeps a retry warning on one line.
fn download_error_summary(error: &eyre::Report) -> String {
    error.to_string().lines().next().unwrap_or("unknown download error").to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_download_is_retried_until_it_succeeds() {
        let mut calls = 0;
        let downloaded = retry("test", &[Duration::ZERO; 3], || {
            calls += 1;
            if calls < 3 { Err(eyre::eyre!("failure {calls}")) } else { Ok(calls) }
        });
        assert_eq!(downloaded.unwrap(), 3);
    }

    #[test]
    fn a_download_that_keeps_failing_returns_its_last_error() {
        let mut calls = 0;
        let downloaded: eyre::Result<()> = retry("test", &[Duration::ZERO; 3], || {
            calls += 1;
            Err(eyre::eyre!("failure {calls}"))
        });
        assert_eq!(downloaded.unwrap_err().to_string(), "failure 4");
        assert_eq!(calls, 4);
    }

    #[test]
    fn download_error_summary_keeps_retry_warning_single_line() {
        let error = eyre::eyre!("first line\nsecond line");

        assert_eq!(download_error_summary(&error), "first line");
    }

    #[test]
    fn retired_version_labels_are_rejected_without_reclassifying_other_arguments() {
        for label in ["pg13", "pg14"] {
            assert!(super::reject_removed_pg_version(Some(label)).is_err());
        }
        for label in [None, Some("pg15"), Some("pg19"), Some("pg99"), Some("my_database")] {
            assert!(super::reject_removed_pg_version(label).is_ok());
        }
    }
}

// TODO: Abstract over the repeated `fn perform`?
