#include "postgres.h"
#include "utils/memutils.h"

PGDLLEXPORT MemoryContext pg_rs_bridge_GetMemoryContextChunk(void *ptr);
PGDLLEXPORT void pg_rs_bridge_elog(int32 level, char *message);

MemoryContext pg_rs_bridge_GetMemoryContextChunk(void *ptr) {
    return GetMemoryChunkContext(ptr);
}

void pg_rs_bridge_elog(int32 level, char *message) {
    elog(level, "%s", message);
}



