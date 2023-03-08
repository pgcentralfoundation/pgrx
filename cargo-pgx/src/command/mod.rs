/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/

use env_proxy::for_url_str;
use ureq::{Agent, AgentBuilder, Proxy};

pub(crate) mod connect;
pub(crate) mod cross;
pub(crate) mod get;
pub(crate) mod init;
pub(crate) mod install;
pub(crate) mod new;
pub(crate) mod package;
pub(crate) mod pgx;
pub(crate) mod run;
pub(crate) mod schema;
pub(crate) mod start;
pub(crate) mod status;
pub(crate) mod stop;
pub(crate) mod test;
pub(crate) mod version;

// Build a ureq::Agent by the given url. Requests from this agent are proxied if we have
// set the HTTPS_PROXY/HTTP_PROXY environment variables.
pub(self) fn build_agent_for_url(url: &str) -> eyre::Result<Agent> {
    if let Some(proxy_url) = for_url_str(url).to_string() {
        Ok(AgentBuilder::new().proxy(Proxy::new(proxy_url)?).build())
    } else {
        Ok(Agent::new())
    }
}
