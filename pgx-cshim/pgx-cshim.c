#include "postgres.h"
#include "access/htup.h"
#include "access/htup_details.h"
#include "utils/memutils.h"
#include "catalog/pg_type.h"
#include "utils/builtins.h"

#define IS_PG_10 (PG_VERSION_NUM >= 100000 && PG_VERSION_NUM < 110000)
#define IS_PG_11 (PG_VERSION_NUM >= 110000 && PG_VERSION_NUM < 120000)
#define IS_PG_12 (PG_VERSION_NUM >= 120000 && PG_VERSION_NUM < 130000)

PGDLLEXPORT MemoryContext pgx_GetMemoryContextChunk(void *ptr);
MemoryContext pgx_GetMemoryContextChunk(void *ptr) {
    return GetMemoryChunkContext(ptr);
}

PGDLLEXPORT void pgx_elog(int32 level, char *message);
void pgx_elog(int32 level, char *message) {
    elog(level, "%s", message);
}

PGDLLEXPORT void pgx_elog_error(char *message);
void pgx_elog_error(char *message) {
    elog(ERROR, "%s", message);
}

PGDLLEXPORT void pgx_ereport(int level, int code, char *message, char *file, int lineno, int colno);
void pgx_ereport(int level, int code, char *message, char *file, int lineno, int colno) {
    ereport(level,
            (errcode(code),
                    errmsg("%s", message), errcontext_msg("%s:%d:%d", file, lineno, colno)));
}

PGDLLEXPORT void pgx_SET_VARSIZE(struct varlena *ptr, int size);
void pgx_SET_VARSIZE(struct varlena *ptr, int size) {
    SET_VARSIZE(ptr, size);
}

PGDLLEXPORT Datum pgx_heap_getattr(HeapTupleData *tuple, int attnum, TupleDesc tupdesc, bool *isnull);
Datum pgx_heap_getattr(HeapTupleData *tuple, int attnum, TupleDesc tupdesc, bool *isnull) {
    return heap_getattr(tuple, attnum, tupdesc, isnull);
}

PGDLLEXPORT TransactionId pgx_HeapTupleHeaderGetXmin(HeapTupleHeader htup_header);
TransactionId pgx_HeapTupleHeaderGetXmin(HeapTupleHeader htup_header) {
    return HeapTupleHeaderGetXmin(htup_header);
}

PGDLLEXPORT CommandId pgx_HeapTupleHeaderGetRawCommandId(HeapTupleHeader htup_header);
CommandId pgx_HeapTupleHeaderGetRawCommandId(HeapTupleHeader htup_header) {
    return HeapTupleHeaderGetRawCommandId(htup_header);
}

#if IS_PG_10 || IS_PG_11
PGDLLEXPORT Oid pgx_HeapTupleHeaderGetOid(HeapTupleHeader htup_header);
Oid pgx_HeapTupleHeaderGetOid(HeapTupleHeader htup_header) {
    return HeapTupleHeaderGetOid(htup_header);
}
#endif

PGDLLEXPORT char *pgx_GETSTRUCT(HeapTuple tuple);
char *pgx_GETSTRUCT(HeapTuple tuple) {
    return GETSTRUCT(tuple);
}
