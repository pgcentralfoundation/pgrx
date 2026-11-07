# Testing Upgrades

Passing tests say nothing about your upgrade SQL. `cargo pgrx test` and `cargo pgrx regress` start
from `CREATE EXTENSION`, which runs the base schema generated from your Rust code. Neither
hand-written scripts nor fragments run on that path, so a broken one passes every test you have.

Only `ALTER EXTENSION ... UPDATE` runs them, on a database created by an older release.

## Testing Against the Previous Release

Run this in CI for every pull request that changes SQL:

1. Check out the previous release's tag and install it.

   ```console
   $ git checkout v0.25.0
   $ cargo pgrx install
   ```

   If that release used a different version of pgrx, install the matching `cargo-pgrx` for this
   step, since the two have to agree.

2. Create the extension in a new database and add data that resembles production.

   ```console
   $ createdb upgraded
   $ psql upgraded -c 'CREATE EXTENSION my_ext'
   ```

3. Check out your branch and install it. Both versions' files now sit side by side.

   ```console
   $ git checkout -
   $ cargo pgrx install
   ```

4. Upgrade, and check the result.

   ```sql
   ALTER EXTENSION my_ext UPDATE;
   SELECT extversion FROM pg_extension WHERE extname = 'my_ext';
   ```

Run your tests against that database. Data that survives the upgrade matters as much as the DDL
running without errors.

## Comparing an Upgrade to a Fresh Install

An upgrade can succeed and still leave a database different from a new one, usually because a change
reached the Rust code and not the upgrade SQL. Compare the upgraded database with a new one:

```console
$ createdb fresh
$ psql fresh -c 'CREATE EXTENSION my_ext'
$ psql upgraded -c '\dx+ my_ext' > upgraded.txt
$ psql fresh -c '\dx+ my_ext' > fresh.txt
$ diff upgraded.txt fresh.txt
```

`\dx+` lists the objects that belong to the extension, so a missing or extra object shows up. It
doesn't show definitions, though, so a function whose return type never changed on the upgrade
path looks the same. Compare the routines' definitions too:

```sql
SELECT pg_get_functiondef(p.oid)
  FROM pg_depend d
  JOIN pg_proc p ON p.oid = d.objid
 WHERE d.classid = 'pg_proc'::regclass
   AND d.refclassid = 'pg_extension'::regclass
   AND d.refobjid = (SELECT oid FROM pg_extension WHERE extname = 'my_ext')
   AND d.deptype = 'e'
   AND p.prokind <> 'a' -- pg_get_functiondef rejects aggregates
 ORDER BY 1;
```

Don't reach for `pg_dump --schema-only` here. It writes `CREATE EXTENSION` and leaves out every
object the extension owns, so its output is the same whatever the upgrade did.

## A Cheaper Test from One Checkout

Testing from a release tag takes two builds. A regression test can run fragments from a single
checkout, with one complication: `cargo pgrx install` generates
`my_ext--<version>.sql` from today's code, so that "old" version already has everything the fragments
add. Running them on top would fail with `already exists`.

The test has to take those objects back out first. From the
[`migration_fragments` example](https://github.com/pgcentralfoundation/pgrx/tree/develop/pgrx-examples/migration_fragments):

```sql
CREATE EXTENSION migration_fragments VERSION '1.1.0';
ALTER EXTENSION migration_fragments DROP FUNCTION shout(text);
DROP FUNCTION shout(text);
ALTER EXTENSION migration_fragments DROP TABLE greeting_log;
DROP TABLE greeting_log;

ALTER EXTENSION migration_fragments UPDATE;
```

Its `upgrade.sql` then compares the extension's members to a fresh install's. This runs your
fragments and proves they're in a workable order, but you write the rewind by hand, so it's only as
good as your memory of what the last release contained. It doesn't replace testing against a real
previous release.
