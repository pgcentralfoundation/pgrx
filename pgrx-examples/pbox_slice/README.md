# pbox_slice — `MemCx` slice + varlena demos

Demonstrates the safe, lifetime-bound allocation APIs added in support of
[issue #2217](https://github.com/pgcentralfoundation/pgrx/issues/2217).

All four `#[pg_extern]` functions return a `PBox<RawVarlena>` directly to
Postgres — no intermediate `Vec<u8>` allocated or copied.

| Function | Demonstrates |
|----------|--------------|
| `build_bytea(int)` | Bare-bones varlena allocation + payload write |
| `xor_bytea(bytea, bytea)` | Common bytea-in / bytea-out transformation |
| `pack_i32_array(int[])` | Exact-size varlena alloc + structured binary serialization |
| `hash_chain(bytea, int)` | Pairing `PBox<[T]>` scratch buffer with `VarlenaBuf` output |

The slice-returning analogue (`PBox<[T]>` as a SQL array) is deferred to a
follow-up; it requires `BoxRet` + `SqlTranslatable` impls that bridge `[T]`
to Postgres's `ArrayType` representation.

## Build & test

```bash
cd pgrx-examples/pbox_slice
cargo pgrx test pg18
```
