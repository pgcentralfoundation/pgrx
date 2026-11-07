-- pgrx-regress-only pg17
-- `cargo pgrx regress` runs this test only on Postgres 17 and reports it as SKIP elsewhere
SELECT current_setting('server_version_num')::int / 10000 AS major;
