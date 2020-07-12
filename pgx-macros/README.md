# pgx-macros

Procedural macros for [`pgx`](https://crates.io/pgx/).

Provides:

    - #[pg_extern]
    - #[pg_guard]
    - #[pg_test]
    - #[derive(PostgresType)]
    - #[derive(PostgresEnum)]
    - #[derive(PostgresGucEnum)]
    
Using `pgx` as a dependency necessitates that `pgx-macros` also be a dependency