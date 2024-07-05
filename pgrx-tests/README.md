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
