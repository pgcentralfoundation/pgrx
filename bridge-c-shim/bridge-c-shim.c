#include "postgres.h"
#include "utils/memutils.h"

PGDLLEXPORT MemoryContext pg_rs_bridge_GetMemoryContextChunk(void *ptr);
PGDLLEXPORT void pg_rs_bridge_elog(int32 level, char *message);
PGDLLEXPORT void pg_rs_bridge_elog_error(char *message);
PGDLLEXPORT void pg_rs_bridge_ereport(int level, int code, char *message, char *file, int lineno, int colno);

MemoryContext pg_rs_bridge_GetMemoryContextChunk(void *ptr) {
    return GetMemoryChunkContext(ptr);
}

void pg_rs_bridge_elog(int32 level, char *message) {
    elog(level, "%s", message);
}

void pg_rs_bridge_elog_error(char *message) {
    elog(ERROR, "%s", message);
}

void pg_rs_bridge_ereport(int level, int code, char *message, char *file, int lineno, int colno) {
    ereport(level,
            (errcode(code),
                    errmsg("%s", message), errcontext_msg("%s:%d:%d", file, lineno, colno)));
}


