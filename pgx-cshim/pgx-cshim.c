#include "postgres.h"
#include "utils/memutils.h"

PGDLLEXPORT MemoryContext pgx_GetMemoryContextChunk(void *ptr);
PGDLLEXPORT void pgx_elog(int32 level, char *message);
PGDLLEXPORT void pgx_elog_error(char *message);
PGDLLEXPORT void pgx_ereport(int level, int code, char *message, char *file, int lineno, int colno);

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


