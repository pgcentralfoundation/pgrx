# pgrx-tests

Test harness for [`pgrx`](https://crates.io/crates/pgrx/) extension tests.

Use this as a `[dev-dependencies]` crate when your extension uses `#[pg_test]`.

## The `client` feature

A `#[pg_test]` function runs in one backend. A test that needs more sessions,
such as a test of notifications or lock waits, can open them with
`pgrx_tests::client()`, which the `client` feature adds. It returns a
[`postgres`](https://crates.io/crates/postgres) client, and the feature
re-exports that crate as `pgrx_tests::postgres`. The harness doesn't need the
feature, and it's off by default because `postgres` brings in `tokio`.

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

Then, in the test:

```rust
#[pg_test]
fn test_two_sessions() {
    let (mut other, _session_id) = pgrx_tests::client().unwrap();

    // `other` is a separate session, so it can't see this test's uncommitted table.
    Spi::run("CREATE TABLE tests.mine ()").unwrap();
    let row = other
        .query_one("SELECT to_regclass('tests.mine') IS NULL", &[])
        .unwrap();
    assert!(row.get::<_, bool>(0));
}
```

`client()` also returns the session's ID, the `%c` value that starts its lines
in the server log. It works from an ordinary `#[test]` too, once
`pgrx_tests::run_test` has started the server.
