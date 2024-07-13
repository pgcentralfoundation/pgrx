# pgrx-tests

Test framework for [`pgrx`](https://crates.io/crates/pgrx/).

Meant to be used as one of your `[dev-dependencies]` when using `pgrx`.

## Running tests

When running tests defined in this crate, you will have to pass along featurs as you would normally pass to `pgrx`.

For example if you simply want to run a test by name on PG16:

```console
cargo test --features=pg16 name_of_your_test
```

A slightly more complicated example which runs al tests with start with `test_pgrx_chrono_roundtrip` and enables the required features for those tests to run:

```console
cargo test --features "pg16,proptest,chrono" test_pgrx_chrono_roundtrip
```

## FAQ / Common Issues

### Different `cargo pgrx` version

In local testing, if the version of `cargo-pgrx` differs from the ones the tests attempt to use, an error results.

If you run into this issue, make sure to install the *local* version of `cargo-pgrx` rather than the officially released version, temporarily while you run your tests.

From the `pgrx-tests` folder, you would run:

```console
cargo install --path ../cargo-pgrx
```

### `The specified pg_config binary, ... does not exist`

If you get this error, and were trying to test against PG16 (as in the example from the [running tests section](#running-tests) above) you should re-initialize pgrx:

```console
cargo pgrx init --pg16 download
```
