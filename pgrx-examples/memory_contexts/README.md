# memory_contexts

Runnable examples of `PgMemoryContext` lifecycle and how memory contexts
interact with SRFs and bgworkers.

## Files

| File | What it shows |
|---|---|
| `basics.rs` | Create a child `PgMemoryContexts`, `switch_to` for scoped allocation, `reset` to reclaim, anti-pattern (use-after-reset) explained in module docs. |
| `srf_per_call.rs` | Two SRFs — one streaming (`SetOfIterator`), one materialized (`TableIterator`) — with notes on `multi_call_memory_ctx` vs `per_query_ctx` and what pgrx handles for you. |
| `bgworker_state.rs` | Bgworker that allocates its long-lived counter under `TopMemoryContext`. Not exercised by `pg_test`; run manually with `cargo pgrx run pg17 memory_contexts` after adding the crate to `shared_preload_libraries`. |

## Running

```bash
cargo pgrx test pg17 -p memory_contexts
```

The bgworker example does not run under `cargo pgrx test`. To exercise it:

1. Add to `${PGRX_HOME}/data-17/postgresql.conf`:
   ```
   shared_preload_libraries = 'memory_contexts'
   ```
2. `cargo pgrx run pg17 memory_contexts`
3. Watch the postmaster log for `memory_contexts demo worker tick N` lines.
