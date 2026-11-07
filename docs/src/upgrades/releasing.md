# Releasing with Fragments

Assembling fragments turns them into a permanent upgrade script. After that the script is an ordinary
file in `sql/`, and the fragments are gone.

## Cutting a Release

1. Set the new version in `Cargo.toml`. A control file with
   `default_version = '@CARGO_VERSION@'` follows along.

2. Preview the plan:

   ```console
   $ cargo pgrx migrate info
   Extension: my_ext (0.25.0 -> 0.26.0)
   Output: /path/to/my_ext/sql/my_ext--0.25.0--0.26.0.sql
   Fragments: (3):
     - 101.add_custom_index.sql (id: 101)
     - 102.new_vector_proc.sql (id: 102)
     - 105.alter_vector_proc.sql (id: 105, depends on: [102])
   ```

3. Assemble:

   ```console
   $ cargo pgrx migrate assemble
     Assembling 3 SQL migration fragment(s) from sql/unreleased
      - 101.add_custom_index.sql
      - 102.new_vector_proc.sql
      - 105.alter_vector_proc.sql
          Saved assembled upgrade script to sql/my_ext--0.25.0--0.26.0.sql
        Removed consumed fragment 101.add_custom_index.sql
        Removed consumed fragment 102.new_vector_proc.sql
        Removed consumed fragment 105.alter_vector_proc.sql
   ```

4. Commit the new script and the deleted fragments together.

The target defaults to your extension's version when that's newer than every script in `sql/`, and
to the next patch otherwise. Pass a version to choose: `cargo pgrx migrate assemble 0.26.0`. The
previous version defaults to the newest non-pre-release script older than the target.

`assemble` checks everything it can before it changes anything, and refuses to:

- Replace a script that already exists. The first run deleted the fragments, so a second run would
  have nothing left to write. Delete the script yourself if you mean to rebuild it.
- Write an upgrade with no fragments, unless you pass `--allow-empty`.
- Guess a previous version it can't determine.
- Include a fragment that already shipped, which it recognizes in any script on the path to the
  previous version.

`--dry-run` reports the same plan and the same errors without writing anything.

## Your First Release with Fragments

Assembly needs a previous version to name the script. If `sql/` has no upgrade scripts yet and
`Cargo.toml` already has the new version, there's nothing to read it from, so pass it:

```console
$ cargo pgrx migrate assemble --prev-version 0.1.0
```

Later releases find it in `sql/`.

## Extensions with a Literal `default_version`

If the control file names a version instead of `@CARGO_VERSION@`, pass `--update-control` to set it
to the target:

```console
$ cargo pgrx migrate assemble 0.26.0 --update-control
```

With `@CARGO_VERSION@` there's nothing to update, so the placeholder stays. `--update-control` then
checks that `Cargo.toml` already has the target version, and refuses if it doesn't.

## Release Candidates

Keep the fragments when you assemble a pre-release, because the final release needs them too:

```console
$ cargo pgrx migrate assemble 0.26.0-rc.1 --preserve-fragments
```

This writes `my_ext--0.25.0--0.26.0-rc.1.sql`. Assembling `0.26.0` later writes
`my_ext--0.25.0--0.26.0.sql` from the same fragments, because previous versions skip pre-releases.
Going through the release candidate would apply the same SQL twice.

There's no automatic path from `0.26.0-rc.1` to `0.26.0`. Testers on a release candidate need a
script you write by hand, or a fresh install.

While you test, `cargo pgrx install` notices fragments already in the newest script and doesn't
assemble them again, so they don't run twice on the update path. `cargo pgrx package` doesn't count
them as unreleased either.

## Patch Releases from a Stable Branch

To ship a fix from a `0.25.x` branch:

1. Merge the fix to your main branch with its fragment.
2. Cherry-pick the commit to `0.25.x`. The fragment arrives unchanged, and it has to stay
   byte-for-byte identical on both branches.
3. On `0.25.x`, set the patch version and run `cargo pgrx migrate assemble`, as for any release.
4. On the main branch, copy the new script into `sql/` and delete the fragments it consumed.
   `cargo pgrx migrate assemble` refuses to include them in the next release if you forget.

A fragment with a `depends-on` can only be backported once what it depends on is on that branch.
Nothing warns you otherwise: a missing dependency looks like one from an earlier release. A
[mixed fragment](./fragments.md#keeping-released-and-unreleased-changes-apart) can't be backported
at all.

If you release `0.25.7` after `0.26.0` is out, users on `0.25.7` need a script to `0.26.0` too.
Nothing assembles that one for you, so write it by hand.

## Releases with No Schema Changes

Postgres still needs a link in the chain. `--allow-empty` writes a script with nothing in it:

```console
$ cargo pgrx migrate assemble --allow-empty
```

## Packaging

`cargo pgrx package` refuses to build while unreleased fragments are waiting, so that they can't be
left out of a release by accident. It checks before compiling.

| Command | With unreleased fragments |
|---|---|
| `cargo pgrx package` | Fails |
| `cargo pgrx package --assemble-unreleased` | Includes them as a temporary upgrade, and sets `default_version` to the temporary version |
| `cargo pgrx package --no-assemble-unreleased` | Leaves them out |

If your main branch always has fragments waiting, use `--no-assemble-unreleased` for packages built
from it. `cargo pgrx install --sudo` builds a package internally, and behaves like `install`: it
assembles fragments unless you pass `--no-assemble-unreleased`.

## Scripting a Release

`cargo pgrx migrate info --json` prints the plan without touching the disk:

```json
{
  "extension_name": "my_ext",
  "target_version": "0.26.0",
  "prev_version": "0.25.0",
  "output_file": "/path/to/my_ext/sql/my_ext--0.25.0--0.26.0.sql",
  "is_empty": false,
  "fragments": [
    { "filename": "101.add_custom_index.sql", "id": "101", "dependencies": [] }
  ]
}
```

`cargo pgrx migrate assemble --json` prints the same plan after doing the work. Progress goes to
stderr, so stdout is always valid JSON. `--output-dir` writes the script somewhere other than
`sql/`, which suits a release job that doesn't commit to the branch.
