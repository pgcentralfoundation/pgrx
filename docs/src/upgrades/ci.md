# Checking Fragments in CI

`cargo pgrx migrate check`, also known as `lint`, reads your fragments and fails if their order can't
be trusted. It needs no Postgres installation, so it's quick to run on every pull request:

```yaml
- uses: actions/checkout@v4
  with:
    fetch-depth: 0 # --base needs history to compare against

- name: Check SQL fragments
  run: |
    cargo pgrx migrate check \
      --base "origin/${{ github.base_ref }}" \
      --deny-mixed-objects \
      --format github
```

## What It Checks

**Dependency cycles.** Two fragments that depend on each other can't be ordered.

**Unresolved dependencies.** Every `-- depends-on` target must match either another unreleased
fragment or a `-- Fragment:` marker in an assembled script in `sql/`. That catches misspelled IDs on
`main` as well as backports to a release branch whose prerequisite hasn't been backported.

**Undeclared dependencies.** If fragment B touches an object fragment A creates, B has to declare
`-- depends-on: A`, directly or through another fragment. When A has the higher number, B would run
first and fail. When A has the lower number they run in the right order by luck, and a backport or a
renumbered pull request would change that. The check catches both.

```console
$ cargo pgrx migrate check
❌ 105.alter_vector_proc.sql: Fragment touches unreleased object(s) from fragment `102` but does not declare '-- depends-on: 102'

❌ Migration fragment lint failed with 1 error(s).
```

Two fragments that replace the same function also need an order, since the last one wins.

**Mixed fragments**, with `--deny-mixed-objects`. A fragment that touches both released objects and
unreleased ones, or unreleased objects from multiple prerequisite fragments,
[can't be backported cleanly](./fragments.md#keeping-released-and-unreleased-changes-apart). An
object counts as unreleased only if some fragment creates it with a plain `CREATE`, because
`CREATE OR REPLACE` is also how a released function is changed.

**Upstream identity**, with `--upstream <REF>`. On a stable release branch, pass
`--upstream origin/main` to require every unreleased fragment on the branch to exist at the same path
on `origin/main` with identical bytes.

**Released-script immutability**, with `--base <REF>`. Upgrade scripts in `sql/` should only ever be
added; modifying or deleting an existing `<ext>--<prev>--<target>.sql` script relative to `<REF>` is
rejected.

## Options

- `--base <REF>` scopes `undeclared_dependency` and `mixed_objects` diagnostics to fragments added
  or changed since this branch left `<REF>`, so old fragments don't fail unrelated pull requests, and
  rejects modifications or deletions of existing upgrade scripts in `sql/`. A conflict between a new
  fragment and an old one is still reported, even when it's the old file that needs the fix, and
  unresolved `depends-on` entries are always reported. It reads committed changes, so commit first
  when you run it locally.
- `--upstream <REF>` verifies that every fragment in the unreleased directory exists byte-for-byte
  identically on `<REF>`.
- `--format <text|json|github>` chooses the output. `github` writes workflow annotations.

## What It Can't See

The check reads the object named at the start of each `CREATE`, `ALTER` and `DROP`, and the table
behind an index or trigger. It understands strings, comments and dollar quoting, so text inside a
function body never counts as a statement. That also means it's blind to:

- References inside function bodies, views and `DO` blocks, including dynamic SQL.
- Renames. An object is known by the name it was created with.
- Schemas. It compares unqualified names, because the `search_path` isn't known until the script
  runs. Objects with the same name in two schemas look like one object.

Passing the check means the order is declared. It doesn't mean the SQL works, which takes a
[real upgrade test](./testing.md).
