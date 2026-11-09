//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
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

#[cfg(test)]
mod tests {
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
