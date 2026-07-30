# pbox_slice — `MemCx::alloc_varlena` shown in a real workload

A teaching extension that demonstrates `MemCx::alloc_varlena` and `PBox<RawVarlena>` in code that mirrors a real use case rather than a synthetic benchmark.

The shape is a **minimal vector-embedding store**: pack a SQL `real[]` into a 4-byte-per-dimension binary `bytea` (matching pgvector's binary convention), and provide the distance / arithmetic primitives a smallRAG or k-NN application would actually run.

| Function | What it does |
|----------|-------------|
| `embedding_pack(real[]) -> bytea` | Pack a float array as binary embedding (4 bytes/dim). NULL → 0.0. |
| `embedding_dims(bytea) -> int` | Number of dimensions (or `-1` if layout is invalid). |
| `embedding_l2_squared(bytea, bytea) -> float8` | Squared Euclidean distance — skip the sqrt because nearest-neighbour ordering only needs the monotonic transform. |
| `embedding_dot(bytea, bytea) -> float8` | Dot product. For unit-length vectors this equals cosine similarity. |
| `embedding_cosine(bytea, bytea) -> float8` | Cosine similarity. NaN on dim mismatch or zero norm. |
| `embedding_mean(bytea, bytea) -> bytea` | Element-wise mean. Useful for centroids and ensembling. |
| `embedding_normalize(bytea) -> bytea` | L2-normalize to unit length. Lets downstream cosine become a plain dot product. |
| `embedding_seq_mean(bytea, int) -> bytea` | Mean-pool a packed sequence of N embeddings. **Demonstrates child-MemCx scratch + `'mcx` lifetime protection** via `PBox::from_iter_in`. |
| `build_bytea(int) -> bytea` | Smoke-test fixture for SQL-level checks; unrelated to embeddings. |

## Example workflow

```sql
CREATE TABLE documents (
    id   bigserial PRIMARY KEY,
    text text,
    emb  bytea          -- packed embedding from external embedder
);

-- Insert: external embedder returns real[]; we pack to compact binary.
INSERT INTO documents (text, emb)
VALUES (
    'rust is fun',
    embedding_pack(ARRAY[0.13, 0.42, 0.78, /* ... */]::real[])
);

-- Verify the encoding round-trips.
SELECT embedding_dims(emb) FROM documents LIMIT 1;

-- Top-K nearest neighbours by L2 distance.
SELECT id, text,
       embedding_l2_squared(emb, $query_emb) AS dist
FROM documents
ORDER BY dist
LIMIT 10;

-- Pre-normalize so cosine = dot, run faster searches downstream.
UPDATE documents SET emb = embedding_normalize(emb);
```

## Why this matters for the API surface

Every function above is a real workload an extension author would
write. They exercise the new APIs in the shapes that matter:

- **`embedding_pack`, `_mean`, `_normalize`**: build a new bytea from
  scratch with exact size known up-front. No intermediate `Vec<u8>`,
  no `memcpy` from Rust heap into a varlena. The output goes straight
  to Postgres.
- **`embedding_l2_squared`, `_dot`, `_cosine`, `_dims`**: read existing
  byteas via `&[u8]` (zero-copy in) and produce a scalar. No
  allocation needed; tests confirm dimension-mismatch / zero-norm
  edge cases return NaN rather than crashing.

For embeddings sized 384–1536 floats (≈1.5–6 KB each) over millions of
rows, eliminating one `Vec<u8>` allocation + memcpy per query pays off
visibly. That is the point of `MemCx::alloc_varlena`.

## Build & test

```bash
cd pgrx-examples/pbox_slice
cargo pgrx test pg18
```
