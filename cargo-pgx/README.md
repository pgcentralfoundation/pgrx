# cargo-pgx

`cargo-pgx` is a Cargo subcommand for managing `pgx`-based Postgres extensions.

You'll want to use `cargo pgx` during your extension development process. It automates the process of creating new Rust crate projects, auto-generating the SQL schema for your extension, installing your extension locally for testing with Postgres, and running your test suite against one or more versions of Postgres.

A video walkthrough of its abilities can be found here: https://www.twitch.tv/videos/684087991

## Installing

Install via crates.io:

```shell script
$ cargo install cargo-pgx
```

As new versions of `pgx` are released, you'll want to make sure you run this command again to update it.

## Usage

```shell script
$ cargo pgx --help
cargo-pgx 0.4.2
ZomboDB, LLC <zombodb@gmail.com>
Cargo subcommand for 'pgx' to make Postgres extension development easy

USAGE:
    cargo pgx [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -v, --verbose    Enable info logs, -vv for debug, -vvv for trace
    -V, --version    Print version information

SUBCOMMANDS:
    connect    Connect, via psql, to a Postgres instance
    get        Get a property from the extension control file
    help       Print this message or the help of the given subcommand(s)
    init       Initialize pgx development environment for the first time
    install    Install the extension from the current crate to the Postgres specified by
                   whatever `pg_config` is currently on your $PATH
    new        Create a new extension crate
    package    Create an installation package directory
    run        Compile/install extension to a pgx-managed Postgres instance and start psql
    schema     Generate extension schema files
    start      Start a pgx-managed Postgres instance
    status     Is a pgx-managed Postgres instance running?
    stop       Stop a pgx-managed Postgres instance
    test       Run the test suite for this crate
```

## Environment Variables

- `PGX_HOME` - If set, overrides `pgx`'s default directory of `~/.pgx/`
- `PGX_BUILD_FLAGS` - If set during `cargo pgx run/test/install`, these additional flags are passed to `cargo build` while building the extension
- `PGX_BUILD_VERBOSE` - Set to true to enable verbose "build.rs" output -- useful for debugging build issues
- `HTTPS_PROXY` - If set during `cargo pgx init`, it will download the Postgres sources using these proxy settings. For more details refer to the [env_proxy crate documentation](https://docs.rs/env_proxy/*/env_proxy/fn.for_url.html).

## First Time Initialization

```shell script
$ cargo pgx init
  Discovered Postgres v14.1, v13.5, v12.9, v11.14, v10.19
  Downloading Postgres v10.19 from https://ftp.postgresql.org/pub/source/v10.19/postgresql-10.19.tar.bz2
  Downloading Postgres v14.1 from https://ftp.postgresql.org/pub/source/v14.1/postgresql-14.1.tar.bz2
  Downloading Postgres v12.9 from https://ftp.postgresql.org/pub/source/v12.9/postgresql-12.9.tar.bz2
  Downloading Postgres v11.14 from https://ftp.postgresql.org/pub/source/v11.14/postgresql-11.14.tar.bz2
  Downloading Postgres v13.5 from https://ftp.postgresql.org/pub/source/v13.5/postgresql-13.5.tar.bz2
     Removing /home/yourself/.pgx/10.19
     Removing /home/yourself/.pgx/14.1
     Removing /home/yourself/.pgx/12.9
    Untarring Postgres v10.19 to /home/yourself/.pgx/10.19
    Untarring Postgres v14.1 to /home/yourself/.pgx/14.1
    Untarring Postgres v12.9 to /home/yourself/.pgx/12.9
     Removing /home/yourself/.pgx/11.14
    Untarring Postgres v11.14 to /home/yourself/.pgx/11.14
     Removing /home/yourself/.pgx/13.5
    Untarring Postgres v13.5 to /home/yourself/.pgx/13.5
  Configuring Postgres v10.19
  Configuring Postgres v12.9
  Configuring Postgres v14.1
  Configuring Postgres v11.14
  Configuring Postgres v13.5
    Compiling Postgres v10.19
    Compiling Postgres v14.1
    Compiling Postgres v12.9
    Compiling Postgres v11.14
    Compiling Postgres v13.5
   Installing Postgres v10.19 to /home/yourself/.pgx/10.19/pgx-install
   Installing Postgres v11.14 to /home/yourself/.pgx/11.14/pgx-install
   Installing Postgres v12.9 to /home/yourself/.pgx/12.9/pgx-install
   Installing Postgres v13.5 to /home/yourself/.pgx/13.5/pgx-install
   Installing Postgres v14.1 to /home/yourself/.pgx/14.1/pgx-install
   Validating /home/yourself/.pgx/10.19/pgx-install/bin/pg_config
   Validating /home/yourself/.pgx/11.14/pgx-install/bin/pg_config
   Validating /home/yourself/.pgx/12.9/pgx-install/bin/pg_config
   Validating /home/yourself/.pgx/13.5/pgx-install/bin/pg_config
   Validating /home/yourself/.pgx/14.1/pgx-install/bin/pg_config
```

`cargo pgx init` is required to be run once to properly configure the `pgx` development environment.

As shown by the screenshot above, it downloads the latest versions of Postgres v10, v11, v12, v13, configures them, compiles them, and installs them to `~/.pgx/`. Other `pgx` commands such as `run` and `test` will fully manage and otherwise use these Postgres installations for you.

`pgx` is designed to support multiple Postgres versions in such a way that during development, you'll know if you're trying to use a Postgres API that isn't common across all versions. It's also designed to make testing your extension against these versions easy. This is why it requires you to have all fully compiled and installed versions of Postgres during development.

If you want to use your operating system's package manager to install Postgres, `cargo pgx init` has optional arguments that allow you to specify where they're installed (see below).

What you're telling `cargo pgx init` is the full path to `pg_config` for each version.

For any version you specify, `cargo pgx init` will forego downloading/compiling/installing it. `pgx` will then use that locally-installed version just as it uses any version it downloads/compiles/installs itself.

However, if the "path to pg_config" is the literal string `download`, then `pgx` will download and compile that version of Postgres for you.

When the various `--pgXX` options are specified, these are the **only** versions of Postgres that `pgx` will manage for you.

You'll also want to make sure you have the "postgresql-server-dev" package installed for each version you want to manage yourself.

Once complete, `cargo pgx init` also creates a configuration file (`~/.pgx/config.toml`) that describes where to find each version's `pg_config` tool.

If a new minor Postgres version is released in the future you can simply run `cargo pgx init [args]` again, and your local version will be updated, preserving all existing databases and configuration.

```shell script
$ cargo pgx init --help
cargo-pgx-init 0.4.2
ZomboDB, LLC <zombodb@gmail.com>
Initialize pgx development environment for the first time

USAGE:
    cargo pgx init [OPTIONS]

OPTIONS:
    -h, --help           Print help information
        --pg10 <PG10>    [env: PG10_PG_CONFIG=]
        --pg11 <PG11>    If installed locally, the path to PG11's `pgconfig` tool, or `downLoad` to
                         have pgx download/compile/install it [env: PG11_PG_CONFIG=]
        --pg12 <PG12>    If installed locally, the path to PG12's `pgconfig` tool, or `downLoad` to
                         have pgx download/compile/install it [env: PG12_PG_CONFIG=]
        --pg13 <PG13>    If installed locally, the path to PG13's `pgconfig` tool, or `downLoad` to
                         have pgx download/compile/install it [env: PG13_PG_CONFIG=]
        --pg14 <PG14>    If installed locally, the path to PG14's `pgconfig` tool, or `downLoad` to
                         have pgx download/compile/install it [env: PG14_PG_CONFIG=]
    -v, --verbose        Enable info logs, -vv for debug, -vvv for trace
    -V, --version        Print version information
```

## Creating a new Extension

```rust
$ cargo pgx new example
$ ls example/
Cargo.toml  example.control  sql  src
```

`cargo pgx new <extname>` is an easy way to get started creating a new extension. It's similar to `cargo new <name>`, but does the additional things necessary to support building a Rust Postgres extension.

If you'd like to create a "background worker" instead, specify the `--bgworker` argument.

`cargo pgx new` does not initialize the directory as a git repo, but it does create a `.gitignore` file in case you decide to do so.

> **Workspace users:** `cargo pgx new $NAME` will create a `$NAME/.cargo/config`, you should move this into your workspace root as `.cargo./config`.
>
> If you don't, you may experience unnecessary rebuilds using tools like Rust-Analyzer, as it will use the wrong `rustflags` option.

```shell script
$ cargo pgx new --help
cargo-pgx-new 0.4.2
ZomboDB, LLC <zombodb@gmail.com>
Create a new extension crate

USAGE:
    cargo pgx new [OPTIONS] <NAME>

ARGS:
    <NAME>    The name of the extension

OPTIONS:
    -b, --bgworker    Create a background worker template
    -h, --help        Print help information
    -v, --verbose     Enable info logs, -vv for debug, -vvv for trace
    -V, --version     Print version information
```

## Managing Your Postgres Installations

```shell script
$ cargo pgx status all
Postgres v10 is stopped
Postgres v11 is stopped
Postgres v12 is stopped
Postgres v13 is stopped
Postgres v14 is stopped

$ cargo pgx start all
    Starting Postgres v10 on port 28810
    Starting Postgres v11 on port 28811
    Starting Postgres v12 on port 28812
    Starting Postgres v13 on port 28813
    Starting Postgres v14 on port 28814

$ cargo pgx status all
Postgres v10 is running
Postgres v11 is running
Postgres v12 is running
Postgres v13 is running
Postgres v14 is running

$ cargo pgx stop all
    Stopping Postgres v10
    Stopping Postgres v11
    Stopping Postgres v12
    Stopping Postgres v13
    Stopping Postgres v14
```

`cargo pgx` has three commands for managing each Postgres installation: `start`, `stop`, and `status`. Additionally, `cargo pgx run` (see below) will automatically start its target Postgres instance if not already running.

When starting a Postgres instance, `pgx` starts it on port `28800 + PG_MAJOR_VERSION`, so Postgres 10 runs on `28810`, 11 on `28811`, etc. Additionally, the first time any of these are started, it'll automaticaly initialize a `PGDATA` directory in `~/.pgx/data-[10 | 11 | 12]`. Doing so allows `pgx` to manage either Postgres versions it installed or ones already on your computer, and to make sure that in the latter case, `pgx` managed versions don't interfere with what might already be running.

`pgx` doesn't tear down these instances. While they're stored in a hidden directory in your home directory, `pgx` considers these important and permanent database installations.

Once started, you can connect to them using `psql` (if you have it on your $PATH) like so: `psql -p 28812`. However, you probably just want the `cargo pgx run` command.

## Compiling and Running Your Extension

```shell script
$ cargo pgx run pg13
building extension with features ``
"cargo" "build" "--message-format=json-render-diagnostics"
    Finished dev [unoptimized + debuginfo] target(s) in 0.06s

installing extension
     Copying control file to /home/ana/.pgx/13.5/pgx-install/share/postgresql/extension/strings.control
     Copying shared library to /home/ana/.pgx/13.5/pgx-install/lib/postgresql/strings.so
    Building for SQL generation with features ``
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
 Discovering SQL entities
  Discovered 6 SQL entities: 0 schemas (0 unique), 6 functions, 0 types, 0 enums, 0 sqls, 0 ords, 0 hashes, 0 aggregates
     Writing SQL entities to /home/ana/.pgx/13.5/pgx-install/share/postgresql/extension/strings--0.1.0.sql
    Finished installing strings
    Starting Postgres v13 on port 28813
    Re-using existing database strings
psql (13.5)
Type "help" for help.

strings=# DROP EXTENSION strings;
ERROR:  extension "strings" does not exist
strings=# CREATE EXTENSION strings;
CREATE EXTENSION
strings=# \df strings.*
                                      List of functions
 Schema  |     Name      | Result data type |           Argument data types            | Type
---------+---------------+------------------+------------------------------------------+------
 strings | append        | text             | input text, extra text                   | func
 strings | return_static | text             |                                          | func
 strings | split         | text[]           | input text, pattern text                 | func
 strings | split_set     | SETOF text       | input text, pattern text                 | func
 strings | substring     | text             | input text, start integer, "end" integer | func
 strings | to_lowercase  | text             | input text                               | func
(6 rows)

strings=# select strings.to_lowercase('PGX');
 to_lowercase
--------------
 pgx
(1 row)
```

`cargo pgx run <pg10 | pg11 | pg12 | pg13>` is the primary interface into compiling and interactively testing/using your extension during development.

The very first time you execute `cargo pgx run pgXX`, it needs to compile not only your extension, but pgx itself, along with all its dependencies. Depending on your computer, this could take a bit of time (`pgx` is nearly 200k lines of Rust when counting the generated bindings for Postgres). Afterwards, however (as seen in the above screenshot), it's fairly fast.

`cargo pgx run` compiles your extension, installs it to the specified Postgres installation as described by its `pg_config` tool, starts that Postgres instance using the same process as `cargo pgx start pgXX`, and drops you into a `psql` shell connected to a database, by default, named after your extension. From there, it's up to you to create your extension and use it.

This is also the stage where `pgx` automatically generates the SQL schema for your extension via the `sql-generator` binary.

When you exit `psql`, the Postgres instance continues to run in the background.

For Postgres installations which are already on your computer, `cargo pgx run` will need write permissions to the directories described by `pg_config --pkglibdir` and `pg_config --sharedir`. It's up to you to decide how to make that happen. While a single Postgres installation can be started multiple times on different ports and different data directories, it does not support multiple "extension library directories".

```shell script
$ cargo pgx run --help
cargo-pgx-run 0.4.2
ZomboDB, LLC <zombodb@gmail.com>
Compile/install extension to a pgx-managed Postgres instance and start psql

USAGE:
    cargo pgx run [OPTIONS] [ARGS]

ARGS:
    <PG_VERSION>    Do you want to run against Postgres `pg10`, `pg11`, `pg12`, `pg13`, `pg14`?
                    [env: PG_VERSION=]
    <DBNAME>        The database to connect to (and create if the first time).  Defaults to a
                    database with the same name as the current extension name

OPTIONS:
        --all-features
            Activate all available features

        --features <FEATURES>
            Space-separated list of features to activate

    -h, --help
            Print help information

        --manifest-path <MANIFEST_PATH>
            Path to Cargo.toml

        --no-default-features
            Do not activate the `default` feature

    -p, --package <PACKAGE>
            Package to build (see `cargo help pkgid`)

        --pgcli
            Use an existing `pgcli` on the $PATH [env: PGX_PGCLI=]

    -r, --release
            Compile for release mode (default is debug) [env: PROFILE=]

    -v, --verbose
            Enable info logs, -vv for debug, -vvv for trace

    -V, --version
            Print version information
```

## Connect to a Database

```shell script
$ cargo pgx connect
    Re-using existing database strings
psql (13.5)
Type "help" for help.

strings=# select strings.to_lowercase('PGX');
 to_lowercase
--------------
 pgx
(1 row)

strings=# 
```

If you'd simply like to connect to a managed version of Postgres without re-compiling and installing
your extension, use `cargo pgx connect <pg10 | pg11 | pg12 | pg13>`.

This command will use the default database named for your extension, or you can specify another
database name as the final argument.

If the specified database doesn't exist, `cargo pgx connect` will create it. Similarly, if
the specified version of Postgres isn't running, it'll be automatically started.

```shell script
$ cargo pgx connect --help
cargo-pgx-connect 0.4.2
ZomboDB, LLC <zombodb@gmail.com>
Connect, via psql, to a Postgres instance

USAGE:
    cargo pgx connect [OPTIONS] [ARGS]

ARGS:
    <PG_VERSION>    Do you want to run against Postgres `pg10`, `pg11`, `pg12`, `pg13`, `pg14`?
                    [env: PG_VERSION=]
    <DBNAME>        The database to connect to (and create if the first time).  Defaults to a
                    database with the same name as the current extension name [env: DBNAME=]

OPTIONS:
    -h, --help
            Print help information

        --manifest-path <MANIFEST_PATH>
            Path to Cargo.toml

    -p, --package <PACKAGE>
            Package to determine default `pg_version` with (see `cargo help pkgid`)

        --pgcli
            Use an existing `pgcli` on the $PATH [env: PGX_PGCLI=]

    -v, --verbose
            Enable info logs, -vv for debug, -vvv for trace

    -V, --version
            Print version information
```

## Installing Your Extension Locally

```shell script
$ cargo pgx install
building extension with features ``
"cargo" "build" "--message-format=json-render-diagnostics"
    Finished dev [unoptimized + debuginfo] target(s) in 0.06s

installing extension
     Copying control file to /usr/share/postgresql/13/extension/strings.control
     Copying shared library to /usr/lib/postgresql/13/lib/strings.so
    Building for SQL generation with features ``
    Finished dev [unoptimized + debuginfo] target(s) in 0.06s
 Discovering SQL entities
  Discovered 6 SQL entities: 0 schemas (0 unique), 6 functions, 0 types, 0 enums, 0 sqls, 0 ords, 0 hashes, 0 aggregates
     Writing SQL entities to /usr/share/postgresql/13/extension/strings--0.1.0.sql
    Finished installing strings
```

If for some reason `cargo pgx run <PG_VERSION>` isn't your style, you can use `cargo pgx install` to install your extension
to the Postgres installation described by the `pg_config` tool currently on your `$PATH`.

You'll need write permissions to the directories described by `pg_config --pkglibdir` and `pg_config --sharedir`.

By default, `cargo pgx install` builds your extension in debug mode. Specifying `--release` changes that.

```shell script
$ cargo pgx install --help
cargo-pgx-install 0.4.2
ZomboDB, LLC <zombodb@gmail.com>
Install the extension from the current crate to the Postgres specified by whatever `pg_config` is
currently on your $PATH

USAGE:
    cargo pgx install [OPTIONS]

OPTIONS:
        --all-features
            Activate all available features

    -c, --pg-config <PG_CONFIG>
            The `pg_config` path (default is first in $PATH)

        --features <FEATURES>
            Space-separated list of features to activate

    -h, --help
            Print help information

        --manifest-path <MANIFEST_PATH>
            Path to Cargo.toml

        --no-default-features
            Do not activate the `default` feature

    -p, --package <PACKAGE>
            Package to build (see `cargo help pkgid`)

    -r, --release
            Compile for release mode (default is debug) [env: PROFILE=]

        --test
            Build in test mode (for `cargo pgx test`)

    -v, --verbose
            Enable info logs, -vv for debug, -vvv for trace

    -V, --version
            Print version information
```

## Testing Your Extension

```shell script
$ cargo pgx test
"cargo" "test" "--features" " pg_test"
    Finished test [unoptimized + debuginfo] target(s) in 0.07s
     Running unittests (target/debug/deps/spi-312296af509607bc)

running 2 tests
building extension with features ` pg_test`
"cargo" "build" "--features" " pg_test" "--message-format=json-render-diagnostics"
    Finished dev [unoptimized + debuginfo] target(s) in 0.06s

installing extension
     Copying control file to /home/ana/.pgx/13.5/pgx-install/share/postgresql/extension/spi.control
     Copying shared library to /home/ana/.pgx/13.5/pgx-install/lib/postgresql/spi.so
    Building for SQL generation with features ` pg_test`
    Finished test [unoptimized + debuginfo] target(s) in 0.07s
 Discovering SQL entities
  Discovered 11 SQL entities: 1 schemas (1 unique), 8 functions, 0 types, 0 enums, 2 sqls, 0 ords, 0 hashes, 0 aggregates
     Writing SQL entities to /home/ana/.pgx/13.5/pgx-install/share/postgresql/extension/spi--0.0.0.sql
    Finished installing spi
test tests::pg_test_spi_query_by_id_direct ... ok
test tests::pg_test_spi_query_by_id_via_spi ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.61s

Stopping Postgres
```

`cargo pgx test [pg10 | pg11 | pg12 | pg13]` runs your `#[test]` and `#[pg_test]` annotated functions using cargo's test system.

During the testing process, `pgx` starts a tempory instance of Postgres with its `PGDATA` directory in `./target/pgx-test-data-PGVER/`. This Postgres instance is stopped as soon as the test framework has finished.

The output is standard "cargo test" output along with some Postgres log output. In the case of test failures, the failure report will include any Postgres log messages generated by that particular test.

Rust `#[test]` functions behave normally, while `#[pg_test]` functions are run **inside** the Postgres instance and have full access to all of Postgres internals. All tests are run in parallel, regardless of their type.

Additionally, a `#[pg_test]` function runs in a transaction that is aborted when the test is finished. As such, any changes it might
make to the database are not preserved.

```shell script
$ cargo pgx test --help
cargo-pgx-test 0.4.2
ZomboDB, LLC <zombodb@gmail.com>
Run the test suite for this crate

USAGE:
    cargo pgx test [OPTIONS] [ARGS]

ARGS:
    <PG_VERSION>    Do you want to run against Postgres `pg10`, `pg11`, `pg12`, `pg13`, `pg14`,
                    or `all`? [env: PG_VERSION=]
    <TESTNAME>      If specified, only run tests containing this string in their names

OPTIONS:
        --all-features
            Activate all available features

        --features <FEATURES>
            Space-separated list of features to activate

    -h, --help
            Print help information

        --manifest-path <MANIFEST_PATH>
            Path to Cargo.toml

    -n, --no-schema
            Don't regenerate the schema

        --no-default-features
            Do not activate the `default` feature

    -p, --package <PACKAGE>
            Package to build (see `cargo help pkgid`)

    -r, --release
            compile for release mode (default is debug) [env: PROFILE=]

    -v, --verbose
            Enable info logs, -vv for debug, -vvv for trace

    -V, --version
            Print version information
```

## Building an Installation Package

```shell script
$ cargo pgx package
building extension with features ``
"cargo" "build" "--release" "--message-format=json-render-diagnostics"
    Finished release [optimized] target(s) in 0.07s

installing extension
     Copying control file to target/release/spi-pg13/usr/share/postgresql/13/extension/spi.control
     Copying shared library to target/release/spi-pg13/usr/lib/postgresql/13/lib/spi.so
    Building for SQL generation with features ``
    Finished release [optimized] target(s) in 0.07s
 Discovering SQL entities
  Discovered 8 SQL entities: 0 schemas (0 unique), 6 functions, 0 types, 0 enums, 2 sqls, 0 ords, 0 hashes, 0 aggregates
     Writing SQL entities to target/release/spi-pg13/usr/share/postgresql/13/extension/spi--0.0.0.sql
    Finished installing spi
```

`cargo pgx package [--debug]` builds your extension, in `--release` mode, to a directory structure in
`./target/[debug | release]/extension_name-PGVER` using the Postgres installation path information from the `pg_config`
tool on your `$PATH`.

The intent is that you'd then change into that directory and build a tarball or a .deb or .rpm package.

The directory structure `cargo pgx package` creates starts at the root of the filesystem, as a package-manager installed
version of Postgres is likely to split `pg_config --pkglibdir` and `pg_config --sharedir` into different base paths.

(In the example screenshot above, `cargo pgx package` was used to build a directory structure using my manually installed
version of Postgres 12.)

This command could be useful from Dockerfiles, for example, to automate building installation packages for various Linux
distobutions or MacOS Postgres installations.

```shell script
$ cargo pgx package --help
cargo-pgx-package 0.4.2
ZomboDB, LLC <zombodb@gmail.com>
Create an installation package directory

USAGE:
    cargo pgx package [OPTIONS]

OPTIONS:
        --all-features
            Activate all available features

    -c, --pg-config <PG_CONFIG>
            The `pg_config` path (default is first in $PATH)

    -d, --debug
            Compile for debug mode (default is release) [env: PROFILE=]

        --features <FEATURES>
            Space-separated list of features to activate

    -h, --help
            Print help information

        --manifest-path <MANIFEST_PATH>
            Path to Cargo.toml

        --no-default-features
            Do not activate the `default` feature

        --out-dir <OUT_DIR>
            The directory to output the package (default is `./target/[debug|release]/extname-
            pgXX/`)

    -p, --package <PACKAGE>
            Package to build (see `cargo help pkgid`)

        --test
            Build in test mode (for `cargo pgx test`)

    -v, --verbose
            Enable info logs, -vv for debug, -vvv for trace

    -V, --version
            Print version information
```

## Inspect you Extension Schema

If you just want to look at the full extension schema that pgx will generate, use
`cargo pgx schema`.

```shell script
$ cargo pgx schema --help
cargo-pgx-schema 0.4.2
ZomboDB, LLC <zombodb@gmail.com>
Generate extension schema files

USAGE:
    cargo pgx schema [OPTIONS] [PG_VERSION]

ARGS:
    <PG_VERSION>    Do you want to run against Postgres `pg10`, `pg11`, `pg12`, `pg13`, `pg14`?

OPTIONS:
        --all-features
            Activate all available features

    -c, --pg-config <PG_CONFIG>
            The `pg_config` path (default is first in $PATH)

    -d, --dot <DOT>
            A path to output a produced GraphViz DOT file

        --features <FEATURES>
            Space-separated list of features to activate

    -h, --help
            Print help information

        --manifest-path <MANIFEST_PATH>
            Path to Cargo.toml

        --no-default-features
            Do not activate the `default` feature

    -o, --out <OUT>
            A path to output a produced SQL file (default is `stdout`)

    -p, --package <PACKAGE>
            Package to build (see `cargo help pkgid`)

    -r, --release
            Compile for release mode (default is debug) [env: PROFILE=]

        --skip-build
            Skip building a fresh extension shared object

        --test
            Build in test mode (for `cargo pgx test`)

    -v, --verbose
            Enable info logs, -vv for debug, -vvv for trace

    -V, --version
            Print version information
```
