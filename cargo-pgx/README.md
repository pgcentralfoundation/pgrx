# cargo-pgx

`cargo-pgx` is a Cargo subcommand for managing `pgx`-based Postgres extensions.

You'll want to use `cargo pgx` during your extension development process.  It automates the process
of creating new Rust crate projects, auto-generating the SQL schema for your extension, installing your 
extension locally for testing with Postgres, and running your test suite against one or more versions
of Postgres.

 
## Installing

Installing via crates.io is really easy:

```shell script
$ cargo install cargo-pgx
```

## Usage

```shell script
$ cargo pgx --help                
cargo-pgx-pgx 

USAGE:
    cargo-pgx pgx [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    get        get a property from the extension control file
    help       Prints this message or the help of the given subcommand(s)
    install    install the extension from the current crate
    new        create a new extension crate
    schema     generate a schema
    test       run the test suite for this crate
```

Note that for `cargo pgx install` to work properly, you need the `pg_config` tool in your `$PATH` for the
version of Postgres against which you wish to install and use your extension.