# generic_agg

A worked example proving why pgrx needs the `utils/datum.h` bindings, It implements one polymorphic aggregate:

```sql
count_changes(anyelement) -> bigint
```

which counts how many times the input value differs from the previous row.

## Run the tests

```bash
cargo pgrx test pg18 -p generic_agg
```

The tests cover pass-by-reference (`text`, `numeric`), pass-by-value (`int4`), NULL handling, and the empty-input case.

## Try it

```sql
SELECT count_changes(v ORDER BY ord)
FROM (VALUES (1,'a'),(2,'a'),(3,'b'),(4,'b'),(5,'b'),(6,'c')) t(ord, v);
-- 2
*/
```
