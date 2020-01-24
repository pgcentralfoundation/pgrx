#include "postgres.h"
#include "access/htup.h"
#include "access/htup_details.h"
#include "utils/memutils.h"
#include "catalog/pg_type.h"
#include "utils/builtins.h"

PGDLLEXPORT MemoryContext pgx_GetMemoryContextChunk(void *ptr);
PGDLLEXPORT void pgx_elog(int32 level, char *message);
PGDLLEXPORT void pgx_elog_error(char *message);
PGDLLEXPORT void pgx_ereport(int level, int code, char *message, char *file, int lineno, int colno);
PGDLLEXPORT void pgx_SET_VARSIZE(struct varlena *ptr, int size);
PGDLLEXPORT Datum pgx_heap_getattr(HeapTupleData *tuple, int attnum, TupleDesc tupdesc, bool *isnull);
PGDLLEXPORT TransactionId pgx_HeapTupleHeaderGetXmin(HeapTupleHeader htup_header);
PGDLLEXPORT CommandId pgx_HeapTupleHeaderGetRawCommandId(HeapTupleHeader htup_header);
PGDLLEXPORT Oid pgx_HeapTupleHeaderGetOid(HeapTupleHeader htup_header);
PGDLLEXPORT char *pgx_GETSTRUCT(HeapTuple tuple);

MemoryContext pgx_GetMemoryContextChunk(void *ptr) {
    return GetMemoryChunkContext(ptr);
}

void pgx_elog(int32 level, char *message) {
    elog(level, "%s", message);
}

void pgx_elog_error(char *message) {
    elog(ERROR, "%s", message);
}

void pgx_ereport(int level, int code, char *message, char *file, int lineno, int colno) {
    ereport(level,
            (errcode(code),
                    errmsg("%s", message), errcontext_msg("%s:%d:%d", file, lineno, colno)));
}

void pgx_SET_VARSIZE(struct varlena *ptr, int size) {
    SET_VARSIZE(ptr, size);
}

Datum pgx_heap_getattr(HeapTupleData *tuple, int attnum, TupleDesc tupdesc, bool *isnull) {
    return heap_getattr(tuple, attnum, tupdesc, isnull);
}

TransactionId pgx_HeapTupleHeaderGetXmin(HeapTupleHeader htup_header) {
    return HeapTupleHeaderGetXmin(htup_header);
}

CommandId pgx_HeapTupleHeaderGetRawCommandId(HeapTupleHeader htup_header) {
    return HeapTupleHeaderGetRawCommandId(htup_header);
}

Oid pgx_HeapTupleHeaderGetOid(HeapTupleHeader htup_header) {
    return HeapTupleHeaderGetOid(htup_header);
}

char *pgx_GETSTRUCT(HeapTuple tuple) {
    return GETSTRUCT(tuple);
}
