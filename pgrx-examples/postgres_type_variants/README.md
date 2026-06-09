# postgres_type_variants

Side-by-side examples of the four ways pgrx lets you ship a custom Postgres type,
plus `PostgresEnum` and the `PostgresEq`/`Ord`/`Hash` derive chain.

## Which variant should I use?

| File | Derive | I/O format | Pick when |
|---|---|---|---|
| `json_default.rs` | `#[derive(PostgresType, Serialize, Deserialize)]` | Serde JSON | The shape is JSON-friendly; you're prototyping; you don't need a custom textual form. |
| `inoutfuncs_custom.rs` | `#[derive(PostgresType)] #[inoutfuncs]` + `impl InOutFuncs` | Custom text via `&CStr` / `StringInfo` | The SQL surface should look like a domain syntax (`"3+4i"`, `"R5C7"`). |
| `varlena_zerocopy.rs` | `#[derive(PostgresType)] #[pgvarlena_inoutfuncs]` + `impl PgVarlenaInOutFuncs` | Fixed binary on disk; custom text at SQL boundary | High-volume reads; tight binary layout pays off; you can guarantee `#[repr(C)]`-stable fields. |
| `handrolled_datum.rs` | None — manual `FromDatum`/`IntoDatum` + `extension_sql!` | Whatever you write | You need full control over the SQL representation, alignment, or storage class; or you're studying what the derive generates. |

## Related derives

| File | Derive(s) | What you get |
|---|---|---|
| `enum_and_ord.rs` | `PostgresEnum` | Rust unit-enum → SQL `ENUM` type |
| `enum_and_ord.rs` | `PostgresEq` + `PostgresOrd` + `PostgresHash` | Working `=`, `<`, `ORDER BY`, btree/hash operator classes — i.e. you can `CREATE INDEX ... USING btree` on the type. |
| `composite_and_array.rs` | `PostgresType` on a record-shaped struct | `Vec<T>` returns and `Array<T>` arguments work transparently. |

## Running

```bash
cargo pgrx test pg17 -p postgres_type_variants
```

## Background

This crate exists to answer
[pgrx#1384](https://github.com/pgcentralfoundation/pgrx/issues/1384) — "what is
`PostgresType` actually for?" — by demonstrating each of its in/out paths in the
smallest possible runnable form.
