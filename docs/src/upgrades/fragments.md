# SQL Fragments

A fragment is a small SQL file holding the upgrade SQL for one change. Fragments wait in a directory
until you cut a release, when `cargo pgrx migrate assemble` turns them into an ordinary upgrade
script.

## Enabling Fragments

Add this to your extension's `Cargo.toml`:

```toml
[package.metadata.pgrx]
unreleased-sql-dir = "sql/unreleased"
```

Then create the directory. Git doesn't track empty directories, so add a file to keep it, which is
also a good home for your project's fragment rules:

```console
$ mkdir sql/unreleased
$ touch sql/unreleased/.gitkeep
```

This key is the whole opt-in. Without it, `cargo pgrx` ignores `sql/unreleased/` entirely and the
`cargo pgrx migrate` commands tell you how to enable fragments. The path is relative to `Cargo.toml`
and can be any directory.

## Writing a Fragment

Say a pull request adds a function:

```rust
#[pg_extern]
fn shout(message: &str) -> String {
    message.to_uppercase()
}
```

New databases get `shout` from the generated base schema. For existing databases, run
`cargo pgrx schema`, copy the statement it generates, and save it under the pull request's number:

```sql
-- sql/unreleased/101.add_shout.sql
CREATE FUNCTION "shout"("message" TEXT) RETURNS TEXT STRICT LANGUAGE c AS 'MODULE_PATHNAME', 'shout_wrapper';
```

A fragment is named `<number>.<slug>.sql`, where the number is usually the pull request. Its ID is
the number, so this one is `101`. A fragment can also be named `<slug>.sql`, with the slug as its
ID. Numbers make the best IDs: pull requests never share one, so two branches can't add the same
file, and they read as a link to the change that introduced the fragment.

Some changes need more than one statement. Postgres can't change a function's return type in place,
so that fragment drops the function and creates it again:

```sql
-- sql/unreleased/215.index_stats_returns_table.sql
DROP FUNCTION IF EXISTS index_stats(regclass);
CREATE FUNCTION "index_stats"("index" regclass)
RETURNS TABLE ("segment" TEXT, "row_count" bigint)
STRICT LANGUAGE c AS 'MODULE_PATHNAME', 'index_stats_wrapper';
```

End each statement with a semicolon. Fragments are concatenated, so a missing one joins a statement
to the first statement of the next fragment.

## Declaring Dependencies

Fragments run in ID order, numbers first, unless a fragment declares that it needs another:

```sql
-- sql/unreleased/98.index_shouted_greetings.sql
-- depends-on: 102
CREATE INDEX greeting_log_shouted ON greeting_log (shouted);
```

Fragment 98 runs after fragment 102 despite its lower number, because 102 creates the table. Use a
comma-separated list for several dependencies, and either ID or filename: `-- depends-on: 101,
add_vector_index, 102.add_greeting_log.sql`. Dependencies carry through, so a fragment that depends
on 102 doesn't need to repeat what 102 depends on.

A dependency that isn't in the directory is assumed to have shipped in an earlier release, which
means a misspelled ID is silently ignored too. [`cargo pgrx migrate check`](./ci.md) still catches
the mistake whenever the two fragments touch the same object.

## Keeping Released and Unreleased Changes Apart

Suppose a fragment on your main branch changes a function that has already shipped, and also changes
one that an unreleased fragment added. That fragment can't be cherry-picked to a stable branch,
which has never heard of the second function.

Split it in two. If you know a change may be backported, write one fragment for the released objects
and a second, depending on the unreleased work, for the rest:

- `230.parallel_safe.sql` touches released objects only, and can go to any branch.
- `230.parallel_safe_index_stats.sql` starts with `-- depends-on: 215` and stays on the main
  branch.

`cargo pgrx migrate check --deny-mixed-objects` flags fragments that mix the two. Create new objects
with a plain `CREATE`, which is also what `cargo pgrx schema` generates: the check reads
`CREATE OR REPLACE` as possibly changing an object that has already shipped.

## Trying an Upgrade During Development

`cargo pgrx install` assembles your fragments automatically, and so do `run`, `test`, `regress` and
`bench`:

```console
$ cargo pgrx install
  ...
  Assembling unreleased fragments for 1.1.0 -> 1.1.1 into .../extension/my_ext--1.1.0--1.1.1.sql
   Installed base schema for 1.1.1 at .../extension/my_ext--1.1.1.sql
     Updated default_version = '1.1.1' in .../extension/my_ext.control
```

pgrx writes three files to Postgres's extension directory and leaves your checkout alone:

1. An upgrade script from the latest version in `sql/` to a temporary version.
2. A copy of the base schema under the temporary version's name, so `CREATE EXTENSION` installs
   today's schema directly.
3. A control file with `default_version` set to the temporary version.

Now a development database can take the upgrade path:

```sql
ALTER EXTENSION my_ext UPDATE;
```

The temporary version is your extension's version if that's newer than everything in `sql/`, and
otherwise the next patch after the latest script. For an extension at `1.1.0` whose newest script
targets `1.1.0`, that's `1.1.1`.

The next install replaces these files, and removes them if the fragments have gone, for instance
after you switch branches.

### Limits of Temporary Versions

`ALTER EXTENSION` runs a script once. A database that reached `1.1.1` stays there when you add
another fragment, and `ALTER EXTENSION ... UPDATE` reports that there's nothing to do, because the
new fragment joined a script the database already ran. To try it, drop the extension and recreate
it from the previous release.

Once fragments ship, a development database can be stranded. If it's on `1.1.1` and the release
turns out to be `1.2.0`, `sql/` has a script from `1.1.0` to `1.2.0` and nothing from `1.1.1`. That
database needs `DROP EXTENSION` and a fresh `CREATE EXTENSION`, so keep the extension in databases
you can rebuild.

A database created from a development install also reports the temporary version. Its
`pg_extension.extversion` is `1.1.1`, not the version in `Cargo.toml`, and that includes the
databases `cargo pgrx test` creates. Code that compares `extversion` with
`env!("CARGO_PKG_VERSION")`, for example to wait until `ALTER EXTENSION ... UPDATE` has run, sees a
mismatch in every one of them. Compare `extversion` with `default_version` from
`pg_available_extensions` instead, which names the temporary version in a development install and
the released version in a released one. That view reads every control file in the extension
directory, so look it up once rather than on every pass of a loop.

### Leaving Fragments Out

Every installing command takes `--no-assemble-unreleased`. It also skips reading the fragments, so
it works while one of them is broken. Setting `PGRX_NO_ASSEMBLE_UNRELEASED=true` has the same effect
on `cargo pgrx install`, and `cargo pgrx test --no-assemble-unreleased` uses it to reach the install
that the test framework runs.
