This example builds RPM and DEB packages with `cargo pgrx package`.

Its `Cargo.toml` has an empty `[package.metadata.rpm]` section and an empty
`[package.metadata.deb]` section, so both packages use cargo-pgrx's default
templates. The package description, license and maintainer come from the
`description`, `license` and `authors` fields of `[package]`.

```console
$ cargo pgrx package --pg-config /path/to/pg_config
```

The packages are written to `target/release/packaging-pgXX/`, where `XX` is the
Postgres major version reported by `pg_config`. The RPM installs into the PGDG
layout (`/usr/pgsql-XX/`) and the DEB into the Debian layout
(`/usr/lib/postgresql/XX/` and `/usr/share/postgresql/XX/`).

See [cargo-pgrx's README](../../cargo-pgrx/README.md#building-rpm-and-deb-packages)
for the default templates, how to override them, and the available variables.
