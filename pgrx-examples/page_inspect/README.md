# page_inspect — `PgBuffer` demo extension

A minimal extension that demonstrates `pgrx::buffer::PgBuffer` against a real
running Postgres. Each SQL function exercises a different part of the wrapper:

| SQL function | `PgBuffer` API exercised | Demonstrates |
| --- | --- | --- |
| `page_stats(relname text, blockno int4)` | `PgBuffer::read` + `read_lock` + `Page::header` + `Page::max_offset` + `Page::item` | RAII pin, share-locked page traversal, bounds-checked item iteration |
| `page_mark_dirty_try(relname text, blockno int4)` | `PgBuffer::try_write_lock` + `BufferWriteGuard::mark_dirty` | Non-blocking exclusive lock and mutating write |
| `page_is_dirty(relname text, blockno int4)` *(pg17+)* | `PgBuffer::as_raw` → `pg_sys::BufferIsDirty` | Intentional escape hatch for FFI the wrapper hasn't surfaced |

## Run it

```sh
cd pgrx-examples/page_inspect
cargo pgrx run pg18
```

```sql
CREATE EXTENSION page_inspect;

CREATE TABLE t(v int);
INSERT INTO t VALUES (1), (2), (3);

-- Inspect block 0
SELECT * FROM page_stats('t', 0);
--  pd_lower | pd_upper | pd_special | max_offset | valid_items
-- ----------+----------+------------+------------+-------------
--        36 |     8128 |       8192 |          3 |           3

-- Acquire an exclusive lock and mark the page dirty
SELECT page_mark_dirty_try('t', 0);     -- t

-- (pg17+) Confirm the page is now dirty
SELECT page_is_dirty('t', 0);           -- t
```

## Test it

```sh
cargo pgrx test pg18
```

Expected: all `#[pg_test]` cases pass (3 on pg13–pg16, 4 on pg17+ — the extra
case covers the pg17-only `page_is_dirty` round-trip).

## Why this matters

`PgBuffer` turns the most error-prone path in BufferManager programming —
pairing `ReadBuffer` with `ReleaseBuffer` and `LockBuffer(SHARE/EXCLUSIVE)`
with `LockBuffer(UNLOCK)` across every early return, `?`, and panic path —
into a borrow-checked RAII contract:

- Forgetting to release a pin or unlock is impossible: `Drop` does it.
- Holding two lock guards on the same buffer is impossible: borrow checker.
- Reading a page after its lock guard ends is impossible: lifetime bounds.
- Reading past `BLCKSZ` from a corrupt item is impossible: pure-Rust bounds
  check returns `None`.

The `as_raw()` method exists exactly for cases like `page_is_dirty` here —
when an extension needs an FFI the wrapper hasn't yet surfaced, the pin is
still owned by `PgBuffer`, so the raw call is sound for the duration of the
borrow.
