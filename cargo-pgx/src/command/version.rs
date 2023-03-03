use pgx_pg_config::{PgConfig, Pgx};

pub(crate) fn pgx_default(supported_major_versions: &[u16]) -> eyre::Result<Pgx> {
    let mut pgx = Pgx::default();
    rss::PostgreSQLVersionRss::new(supported_major_versions)?
        .into_iter()
        .for_each(|version| pgx.push(PgConfig::from(version)));

    Ok(pgx)
}

mod rss {
    use eyre::WrapErr;
    use owo_colors::OwoColorize;
    use pgx_pg_config::PgVersion;
    use serde_derive::Deserialize;
    use url::Url;

    use crate::command::build_agent_for_url;

    pub(super) struct PostgreSQLVersionRss;

    impl PostgreSQLVersionRss {
        pub(super) fn new(supported_major_versions: &[u16]) -> eyre::Result<Vec<PgVersion>> {
            static VERSIONS_RSS_URL: &str = "https://www.postgresql.org/versions.rss";

            let http_client = build_agent_for_url(VERSIONS_RSS_URL)?;
            let response = http_client
                .get(VERSIONS_RSS_URL)
                .call()
                .wrap_err_with(|| format!("unable to retrieve {}", VERSIONS_RSS_URL))?;

            let rss: Rss = match serde_xml_rs::from_str(&response.into_string()?) {
                Ok(rss) => rss,
                Err(e) => return Err(e.into()),
            };

            let mut versions = Vec::new();
            for item in rss.channel.item {
                let title = item.title.trim();
                let mut parts = title.split('.');
                let major = parts.next();
                let minor = parts.next();

                // if we don't have major/minor versions or if they don't parse correctly
                // we'll just assume zero for them and eventually skip them
                let major = major.unwrap().parse::<u16>().unwrap_or_default();
                let minor = minor.unwrap().parse::<u16>().unwrap_or_default();

                if supported_major_versions.contains(&major) {
                    versions.push(
                        PgVersion::new(
                            major,
                            minor,
                            Url::parse(
                                &format!("https://ftp.postgresql.org/pub/source/v{major}.{minor}/postgresql-{major}.{minor}.tar.bz2",
                                         major = major, minor = minor)
                            ).expect("invalid url")
                        ),
                    )
                }
            }

            println!(
                "{} Postgres {}",
                "  Discovered".white().bold(),
                versions.iter().map(|ver| format!("v{ver}")).collect::<Vec<_>>().join(", ")
            );

            Ok(versions)
        }
    }

    #[derive(Deserialize)]
    struct Rss {
        channel: Channel,
    }

    #[derive(Deserialize)]
    struct Channel {
        item: Vec<Item>,
    }

    #[derive(Deserialize)]
    struct Item {
        title: String,
    }
}
