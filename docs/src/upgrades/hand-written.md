# Hand-Written Upgrade Scripts

`cargo pgrx new` creates a `sql/` directory, and this is where upgrade scripts go:

```console
$ tree
.
├── Cargo.toml
├── my_ext.control
├── sql
│   └── my_ext--1.0.0--1.1.0.sql
└── src
    └── lib.rs
```

Name each script `<ext>--<old>--<new>.sql`. Every command that installs an extension
(`cargo pgrx test`, `run`, `install`, `package` and `bench`) copies these files, and only these
files, from `sql/` to where `pg_config` says they belong.

## Writing a Script

A script holds whatever SQL moves a database from the old version to the new one, such as
`ALTER TABLE`, `CREATE OR REPLACE FUNCTION` or a data migration. To add a function, run
`cargo pgrx schema` and copy the statement it generates:

```rust
#[pg_extern]
fn farewell(name: &str) -> String {
    format!("Goodbye, {name}!")
}
```

```sql
-- sql/my_ext--1.0.0--1.1.0.sql
CREATE FUNCTION "farewell"("name" TEXT) RETURNS TEXT STRICT LANGUAGE c AS 'MODULE_PATHNAME', 'farewell_wrapper';
```

Postgres replaces `MODULE_PATHNAME` with the `module_pathname` from the control file. pgrx replaces
two more tokens as it copies a script:

| Token | Replaced with |
|---|---|
| `@CARGO_VERSION@` | The extension's version |
| `@GIT_HASH@` | The current git commit |

An extension without `module_pathname`, which pgrx builds as a versioned shared library, can use
`'$libdir/my_ext-@CARGO_VERSION@'` where others use `'MODULE_PATHNAME'`.

## Rules for Scripts

Never edit a script after releasing it. Databases that already upgraded won't run it again, so a
fix belongs in the next release's script.

Keep the chain unbroken. Each release needs a script that starts from the previous release, or
users on the older version are stuck.

[Test the upgrade](./testing.md) before you release it: `cargo pgrx test` doesn't run these scripts.
