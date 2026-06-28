# notify — LISTEN/NOTIFY cache invalidation example

Demonstrates safe `NOTIFY` / `LISTEN` / `UNLISTEN` wrappers (in `src/notify.rs`, over the `commands/async.h` bindings) driving a real-world cache-invalidation broadcast. An `AFTER INSERT OR UPDATE OR DELETE` trigger on `products` emits a `NOTIFY cache_invalidation, '<id>'` whenever a row changes. An external application that `LISTEN`s on that channel can drop the matching cache entry in real time instead of polling.

## Try it

```sql
-- session 1: subscribe
LISTEN cache_invalidation;

-- session 2: change data
INSERT INTO products (name) VALUES ('widget');   -- id 1
UPDATE products SET name = 'gadget' WHERE id = 1;
DELETE FROM products WHERE id = 1;
```

Session 1 receives three asynchronous notifications, each with the changed row's id as the payload:

```
Asynchronous notification "cache_invalidation" with payload "1" received from server process ...
Asynchronous notification "cache_invalidation" with payload "1" received from server process ...
Asynchronous notification "cache_invalidation" with payload "1" received from server process ...
```

The same wrappers are exposed for direct use via the `pgrx_notify(channel, payload)` function.

## The hard case: coalesced invalidation 

The per-row trigger above is fine for single-row changes, but a bulk `UPDATE products SET ... WHERE category = 5` touching a million rows would try to queue a million notifications and overrun the async queue (`max_notify_queue_pages`). Notifying once per row is the wrong granularity.

`src/coalesced.rs` shows the correct pattern on an `inventory(id, sku,category)`
table:

1. A row-level trigger does **not** notify. It only accumulates the affected `category` into a per-transaction set in backend-local memory.
2. A single `PreCommit` transaction callback (registered via pgrx's safe `register_xact_callback`) emits **one** `NOTIFY category_invalidation, '<category>'`per distinct dirtied category, just before commit.
3. An `Abort` callback discards the set if the transaction rolls back.

A million row changes in one category collapse into a single notification:

```sql
-- session 1: subscribe
LISTEN category_invalidation;

-- session 2: one statement, 500 rows across 2 categories
INSERT INTO inventory (sku, category)
SELECT 'sku' || g, g % 2 FROM generate_series(1, 500) g;
```

Session 1 receives exactly **two** notifications (payloads `0` and `1`), not 500.This needs Rust/C-level access hooking the transaction lifecycle and holding state across trigger invocations is impossible from a plain SQL trigger.

