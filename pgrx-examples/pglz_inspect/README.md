# pglz_inspect

A pgrx-based PostgreSQL extension that helps DBAs decide whether to enable PGLZ compression on a column. Samples real data, runs it through the in-tree `pglz_compress` implementation, and reports compression ratio, acceptance rate, estimated disk savings, and an actionable recommendation.
Supports PostgreSQL 13–18.

## Install

```bash
cargo pgrx install --package pglz_inspect
psql -c 'CREATE EXTENSION pglz_inspect;'
```

## Functions

| Function | Purpose |
|---|---|
| `pglz_size(bytea)` | Probe a single value: raw size, compressed size, ratio, accepted. |
| `pglz_analyze_column(regclass, text, sample_size, strategy)` | Sample N rows; return aggregate stats and estimated savings (uses `pg_class.reltuples`). |
| `pglz_ratio_histogram(regclass, text, sample_size)` | Distribution of per-row compression ratios across 5 buckets + `incompressible`. |
| `pglz_recommend(regclass, text, sample_size)` | One-line RECOMMEND / MARGINAL / SKIP verdict with ready-to-run DDL. |

## Demo

```sql
CREATE EXTENSION pglz_inspect;

CREATE TABLE events AS
SELECT i AS id,
       ('{"user":'||i||',"action":"click","meta":'||repeat('"x",',50)||'1}') AS payload
FROM generate_series(1, 10000) i;


SELECT * FROM pglz_ratio_histogram('events'::regclass, 'payload');
-- Example output (numbers vary by data):
--      bucket      | row_count
-- -----------------+-----------
--  0.0-0.2         |      9876
--  0.2-0.4         |       124
--  incompressible  |         0

SELECT pglz_recommend('events'::regclass, 'payload');
-- Example output (numbers vary by data):
-- RECOMMEND: PGLZ saves ~83% on events.payload (1000 of 1000 sampled rows accepted).
-- Run: ALTER TABLE events ALTER COLUMN payload SET COMPRESSION pglz;
```

## Histogram bucket semantics

`pglz_ratio_histogram` reports `compressed_size / raw_size` per row, bucketed:

```
┌───────────┬────────────────┐
│   ratio   │     bucket     │
├───────────┼────────────────┤
│ 0.00-0.20 │ 0.0-0.2        │  ← excellent: ~80%+ saved
├───────────┼────────────────┤
│ 0.20-0.40 │ 0.2-0.4        │  ← good
├───────────┼────────────────┤
│ 0.40-0.60 │ 0.4-0.6        │  ← moderate
├───────────┼────────────────┤
│ 0.60-0.80 │ 0.6-0.8        │  ← weak
├───────────┼────────────────┤
│ 0.80-1.00 │ 0.8-1.0        │  ← negligible
├───────────┼────────────────┤
│ PGLZ rejust │ incompressible │  ← rejected by PGLZ heuristics
└───────────┴────────────────┘
```

Highly repetitive data clusters in `0.0-0.2`; short / high-entropy data lands in `incompressible`. A two-peak histogram (`0.0-0.2` + `incompressible`) is common and means the column mixes very-compressible and very-random rows.

## Mixed-distribution example

The default demo data is too uniform to populate the middle buckets. To see a spread across all buckets, use semi-repetitive data:

```sql
DROP TABLE IF EXISTS mixed;

CREATE TABLE mixed AS
SELECT i AS id,
       CASE (i % 5)
         -- 0.0-0.2: heavy repetition
         WHEN 0 THEN repeat('aaaaa', 100)
         -- 0.2-0.4: structured JSON, repeated field names + varied values
         WHEN 1 THEN '{"user_id":'||i||',"event_type":"page_view","timestamp":"2024-01-'||(i%28+1)||'","session":"'||md5(i::text)||'"}'
         -- 0.4-0.6: short repeated fragment + variable suffix
         WHEN 2 THEN repeat(md5(i::text), 3) || md5((i+1)::text)
         -- 0.6-0.8: mostly random, small repeating prefix
         WHEN 3 THEN 'log_entry_' || md5(i::text) || md5((i+1)::text) || md5((i+2)::text)
         -- incompressible: pure randomness
         ELSE (SELECT string_agg(md5(random()::text), '') FROM generate_series(1, 4))
       END AS payload
FROM generate_series(1, 5000) i;

SELECT * FROM pglz_ratio_histogram('mixed'::regclass, 'payload', 5000);
```
