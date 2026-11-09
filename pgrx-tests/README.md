# pgrx-tests

Test harness for [`pgrx`](https://crates.io/crates/pgrx/) extension tests.

Use this as a `[dev-dependencies]` crate when your extension uses `#[pg_test]`.

## The `client` feature

A `#[pg_test]` function runs in one backend. A test that needs more sessions,
such as a test of notifications or lock waits, can open more connections to the
test server with the [`postgres`](https://crates.io/crates/postgres) crate,
which the `client` feature re-exports as `pgrx_tests::postgres`. The harness
doesn't need the feature, and it's off by default because `postgres` brings in
`tokio`.

`#[pg_test]` functions are compiled into the extension's library, which can't
use dev-dependencies. So also list `pgrx-tests` under `[dependencies]`, as an
optional dependency that `pg_test` turns on. Forward the PostgreSQL features
with `pgrx-tests?/`, because `pgrx-tests/` would turn the optional dependency on
in every build:

```toml
[features]
pg18 = ["pgrx/pg18", "pgrx-tests?/pg18"] # and the same for the other pgNN features
pg_test = ["dep:pgrx-tests"]

[dependencies]
pgrx-tests = { version = "=0.20.0", features = ["client"], optional = true }

[dev-dependencies]
pgrx-tests = "=0.20.0"
```

In the test, read the server's address with SPI and connect:

```rust
#[pg_test]
fn test_two_sessions() {
    use pgrx_tests::postgres::{Config, NoTls};

    let setting = |sql: &str| Spi::get_one::<String>(sql).unwrap().unwrap();
    let socket_dir = setting("SHOW unix_socket_directories");
    let host = match socket_dir.split(',').next().map(str::trim) {
        Some(dir) if !cfg!(windows) && !dir.is_empty() => dir,
        _ => "127.0.0.1",
    };
    let mut other = Config::new()
        .host(host)
        .port(setting("SHOW port").parse().unwrap())
        .user(&setting("SELECT current_user::text"))
        .dbname(&setting("SELECT current_database()::text"))
        .connect(NoTls)
        .unwrap();
    // `other` is a separate session, so it doesn't see this test's uncommitted changes.
}
```

`pgrx_tests::client()` doesn't work inside a `#[pg_test]`, because only the test
process knows the port the harness chose. From an ordinary `#[test]`, after
`pgrx_tests::run_test` has started the server, it returns a connected client and
the connection's session ID.
