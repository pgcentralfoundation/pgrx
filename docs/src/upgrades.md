# Upgrading Extensions

Sooner or later you'll change your extension's schema while databases out there are running the
old one. This chapter covers how those databases get the change, and helps you pick between pgrx's
two ways of writing it down.

## Two Paths into a Database

A database gets your extension's schema in one of two ways:

- `CREATE EXTENSION` runs the base schema, `<ext>--<version>.sql`. pgrx generates this file from
  your Rust code every time it installs the extension.
- `ALTER EXTENSION ... UPDATE` runs upgrade scripts, `<ext>--<old>--<new>.sql`, in a chain from the
  version the database has to the version it's asked for, which defaults to `default_version` in
  the control file.

pgrx can't generate upgrade scripts. It knows what your schema looks like now, but it doesn't know
what the last release looked like or what should happen to the data in between. So every schema
change is made twice: once in your Rust code for new databases, and once as SQL in an upgrade
script for existing ones.

Postgres treats version names as opaque labels. It finds the shortest chain of scripts whose
filenames connect the two versions, which means `1.1` and `1.1.0` are unrelated names to it. A gap
in the chain fails with `extension "my_ext" has no update path from version "1.0" to version "1.2"`.
The [Postgres documentation](https://www.postgresql.org/docs/current/extend-extensions.html#EXTEND-EXTENSIONS-UPDATES)
has the details.

## Choosing a Workflow

Both workflows end with the same artifact: an ordinary `<ext>--<old>--<new>.sql` file in `sql/`.
Postgres and your users can't tell how it was made, and you can switch at any release, because
existing scripts keep working either way.

[Hand-written scripts](./upgrades/hand-written.md) need no setup and are the default. You edit one
file per release.

[SQL fragments](./upgrades/fragments.md) are optional, and nothing changes until a package opts in.
You add one small file per change, and pgrx assembles them into the upgrade script when you cut a
release.

### When Hand-Written Scripts Are Enough

Stay with hand-written scripts if most of these describe your project:

- Few people change the schema, and rarely at the same time.
- You release from one branch.
- You know which version a change will ship in when you write it.
- You'd rather review an upgrade as a single file.

### When Fragments Help

Consider fragments when:

- **Several people change the schema at once.** With a shared upgrade script, every pull request
  that touches SQL edits the same file. Each pull request adds its own fragment instead, so they
  don't conflict.
- **You don't know which release a change will ship in.** An upgrade script's filename commits to a
  version. A fragment has no version until it's assembled.
- **You backport fixes to stable branches.** A fragment cherry-picks cleanly as a single file, and
  the stable branch assembles it into whatever patch release comes next.
- **You want to try an upgrade before releasing.** `cargo pgrx install` assembles your fragments
  into a temporary upgrade script, so `ALTER EXTENSION ... UPDATE` works on a development database.
- **You want CI to check upgrade SQL.** `cargo pgrx migrate check` catches cycles and missing
  dependencies between fragments before they merge.

Fragments need version names `cargo-pgrx` can put in order: one to three numbers, with an optional
pre-release suffix, like `1`, `1.2`, `1.2.3` or `1.2.3-rc.1`.

## In This Chapter

- [Hand-Written Upgrade Scripts](./upgrades/hand-written.md)
- [SQL Fragments](./upgrades/fragments.md) covers enabling them and using them day to day.
- [Releasing with Fragments](./upgrades/releasing.md) covers release candidates, stable branches
  and packaging.
- [Checking Fragments in CI](./upgrades/ci.md)
- [Testing Upgrades](./upgrades/testing.md) applies to both workflows.

The [`migration_fragments` example](https://github.com/pgcentralfoundation/pgrx/tree/develop/pgrx-examples/migration_fragments)
is a complete extension you can build and run.
