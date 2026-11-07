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
unreleased ones [can't be backported](./fragments.md#keeping-released-and-unreleased-changes-apart).
An object counts as unreleased only if some fragment creates it with a plain `CREATE`, because
`CREATE OR REPLACE` is also how a released function is changed.

## Options

- `--base <REF>` reports only problems that involve a fragment added or changed since this branch
  left `<REF>`, so old fragments don't fail unrelated pull requests. A conflict between a new
  fragment and an old one is still reported, even when it's the old file that needs the fix. It
  reads committed changes, so commit first when you run it locally.
- `--format <text|json|github>` chooses the output. `github` writes workflow annotations.

## What It Can't See

The check reads the object named at the start of each `CREATE`, `ALTER` and `DROP`, and the table
behind an index or trigger. It understands strings, comments and dollar quoting, so text inside a
function body never counts as a statement. That also means it's blind to:

- References inside function bodies, views and `DO` blocks, including dynamic SQL.
- Renames. An object is known by the name it was created with.
- Schemas. It compares unqualified names, because the `search_path` isn't known until the script
  runs. Objects with the same name in two schemas look like one object.
- A misspelled dependency between fragments that don't touch the same object. It looks like a
  fragment from an earlier release.

Passing the check means the order is declared. It doesn't mean the SQL works, which takes a
[real upgrade test](./testing.md).

## Protecting Released Scripts

Upgrade scripts should only ever be added. This command fails if a branch modifies or deletes one:

```console
$ git diff --exit-code --name-only --diff-filter=MD origin/main...HEAD -- 'sql/*--*--*.sql'
```
