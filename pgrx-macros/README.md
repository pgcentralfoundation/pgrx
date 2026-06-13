# pgrx-macros

Procedural macros for [`pgrx`](https://crates.io/crates/pgrx/).

Provides:

    - #[pg_extern]
    - #[pg_guard]
    - #[pg_guc_hook]
    - #[pg_test]
    - #[derive(PostgresType)]
    - #[derive(PostgresEnum)]
    - #[derive(PostgresGucEnum)]
    
Using `pgrx` as a dependency necessitates that `pgrx-macros` also be a dependency