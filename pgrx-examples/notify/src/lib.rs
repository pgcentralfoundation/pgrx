//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.
use pgrx::prelude::*;

mod coalesced;
mod notify;

pgrx::pg_module_magic!(name, version);

/// Thin SQL-callable wrapper over [`notify::notify`], used to demonstrate (and test) error handling on over-length payloads.
#[pg_extern]
fn pgrx_notify(channel: &str, payload: &str) -> Result<(), String> {
    notify::notify(channel, payload).map_err(|e| e.to_string())
}

/// SQL-callable `LISTEN <channel>` (server-side subscription).
#[pg_extern]
fn pgrx_listen(channel: &str) -> Result<(), String> {
    notify::listen(channel).map_err(|e| e.to_string())
}

/// SQL-callable `UNLISTEN <channel>`.
#[pg_extern]
fn pgrx_unlisten(channel: &str) -> Result<(), String> {
    notify::unlisten(channel).map_err(|e| e.to_string())
}

/// SQL-callable `UNLISTEN *`.
#[pg_extern]
fn pgrx_unlisten_all() {
    notify::unlisten_all()
}

/// whenever a row in `products` changes,  broadcast its id on the `cache_invalidation` channel so external listeners can drop the corresponding cache entry.
#[pg_trigger]
fn products_notify<'a>(
    trigger: &'a pgrx::PgTrigger<'a>,
) -> Result<Option<PgHeapTuple<'a, impl WhoAllocated>>, Box<dyn std::error::Error>> {
    // On DELETE the changed row is `old`; otherwise it is `new`.
    let row = match trigger.new().or_else(|| trigger.old()) {
        Some(r) => r.into_owned(),
        None => return Ok(None),
    };
    let id: Option<i64> = row.get_by_name("id")?;
    if let Some(id) = id {
        notify::notify("cache_invalidation", &id.to_string())?;
    }
    Ok(Some(row))
}

extension_sql!(
    r#"
CREATE TABLE products (
    id    bigserial NOT NULL PRIMARY KEY,
    name  text NOT NULL
);

CREATE TRIGGER products_notify
    AFTER INSERT OR UPDATE OR DELETE ON products
    FOR EACH ROW EXECUTE PROCEDURE products_notify();
"#,
    name = "create_products_trigger",
    requires = [products_notify]
);

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use pgrx::prelude::*;
    use std::time::Duration;

    /// Open a fresh client connection (a separate backend) to the running test server. `pgrx_tests::client()` can't be used from inside a `#[pg_test]` body — its `TEST_PORT` cell is unset there — so discover the live address from the server's own GUCs and connect directly.
    fn open_client() -> postgres::Client {
        let port: String = Spi::get_one("SHOW port").expect("SHOW port").expect("port set");
        let host = if cfg!(windows) {
            "127.0.0.1".to_string()
        } else {
            let host: String =
                Spi::get_one("SHOW unix_socket_directories").expect("SHOW usd").expect("usd set");
            let host = host.split(',').next().unwrap_or("").trim();
            if host.is_empty() { "127.0.0.1".to_string() } else { host.to_string() }
        };
        let user = Spi::get_one::<String>("SELECT current_user::text")
            .expect("current_user")
            .expect("user");
        let dbname = Spi::get_one::<String>("SELECT current_database()::text")
            .expect("current_database")
            .expect("db");
        postgres::Config::new()
            .host(&host)
            .port(port.parse().expect("port"))
            .user(&user)
            .dbname(&dbname)
            .connect(postgres::NoTls)
            .expect("client connect")
    }

    #[pg_test]
    fn notify_rejects_interior_nul() {
        assert!(crate::notify::notify("chan\0nul", "payload").is_err());
        assert!(crate::notify::notify("chan", "pay\0load").is_err());
    }

    #[pg_test]
    fn listen_rejects_interior_nul() {
        assert!(crate::notify::listen("chan\0nul").is_err());
    }

    /// End-to-end: a row change fires the trigger, which NOTIFYs, and a separate listening connection receives the payload after the writer commits.
    #[pg_test]
    fn trigger_delivers_cache_invalidation() {
        // Connection A: subscribe.
        let mut listener = open_client();
        listener.batch_execute("LISTEN cache_invalidation").expect("LISTEN");

        // Connection B: autocommit write that fires the trigger and commits.  Use RETURNING so the assertion is robust to the table's current serial.
        let mut writer = open_client();
        let row = writer
            .query_one("INSERT INTO products (name) VALUES ('widget') RETURNING id", &[])
            .expect("INSERT");
        let id: i64 = row.get(0);

        // Pump the protocol on A so libpq surfaces the async notification.
        listener.batch_execute("SELECT 1").expect("pump");
        let mut notifications = listener.notifications();
        let got = {
            use postgres::fallible_iterator::FallibleIterator;
            notifications
                .timeout_iter(Duration::from_secs(5))
                .next()
                .expect("notification iter")
                .expect("a notification within 5s")
        };

        assert_eq!(got.channel(), "cache_invalidation");
        assert_eq!(got.payload(), id.to_string());
    }

    /// Coalescing: a single statement touching 500 rows across 2 categories must
    /// produce exactly 2 notifications (one per category), not 500.
    #[pg_test]
    fn bulk_change_coalesces_to_one_per_category() {
        use postgres::fallible_iterator::FallibleIterator;

        let mut listener = open_client();
        listener.batch_execute("LISTEN category_invalidation").expect("LISTEN");

        // 500 row changes across exactly two categories, in one autocommitted statement.
        let mut writer = open_client();
        writer
            .batch_execute(
                "INSERT INTO inventory (sku, category) \
                 SELECT 'sku' || g, g % 2 FROM generate_series(1, 500) g",
            )
            .expect("bulk INSERT");

        // Drain every notification that arrives within the window.
        listener.batch_execute("SELECT 1").expect("pump");
        let mut payloads = std::collections::BTreeSet::new();
        let mut count = 0usize;
        let mut notifications = listener.notifications();
        let mut iter = notifications.timeout_iter(Duration::from_secs(2));
        while let Some(n) = iter.next().expect("notification iter") {
            count += 1;
            payloads.insert(n.payload().to_string());
        }

        assert_eq!(count, 2, "expected 2 coalesced notifications, got {count}");
        let expected: std::collections::BTreeSet<String> =
            ["0".to_string(), "1".to_string()].into_iter().collect();
        assert_eq!(payloads, expected);
    }

    /// Over-length payloads are reported by PostgreSQL as an error.
    #[pg_test(error = "payload string too long")]
    fn oversize_payload_errors() {
        let big = "x".repeat(9000);
        Spi::run(&format!("SELECT pgrx_notify('chan', '{big}')")).unwrap();
    }
}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {}
    pub fn postgresql_conf_options() -> Vec<&'static str> {
        vec![]
    }
}
