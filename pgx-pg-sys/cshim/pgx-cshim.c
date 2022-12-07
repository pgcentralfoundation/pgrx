/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
#include "postgres.h"

#define IS_PG_11 (PG_VERSION_NUM >= 110000 && PG_VERSION_NUM < 120000)
#define IS_PG_12 (PG_VERSION_NUM >= 120000 && PG_VERSION_NUM < 130000)
#define IS_PG_13 (PG_VERSION_NUM >= 130000 && PG_VERSION_NUM < 140000)

#include "access/htup.h"
#include "access/htup_details.h"
#include "catalog/pg_type.h"
#if IS_PG_11
#include "nodes/relation.h"
#else
#include "nodes/pathnodes.h"
#endif
#include "nodes/pg_list.h"
#include "parser/parsetree.h"
#include "utils/memutils.h"
#include "utils/builtins.h"
#include "utils/array.h"
#include "storage/spin.h"


PGDLLEXPORT MemoryContext pgx_GetMemoryContextChunk(void *ptr);
MemoryContext pgx_GetMemoryContextChunk(void *ptr) {
    return GetMemoryChunkContext(ptr);
}

PGDLLEXPORT bool pgx_MemoryContextIsValid(MemoryContext context);
bool pgx_MemoryContextIsValid(MemoryContext context) {
    return MemoryContextIsValid(context);
}

PGDLLEXPORT void pgx_elog(int32 level, char *message);
void pgx_elog(int32 level, char *message) {
    elog(level, "%s", message);
}

PGDLLEXPORT void pgx_elog_error(char *message);
void pgx_elog_error(char *message) {
    elog(ERROR, "%s", message);
}

PGDLLEXPORT void pgx_errcontext_msg(char *message);
void pgx_errcontext_msg(char *message) {
    errcontext_msg("%s", message);
}

PGDLLEXPORT void pgx_ereport(const int level, int code, char *message, char *detail, char *funcname, char *file, int lineno, char *context);
void pgx_ereport(const int level, int code, char *message, char *detail, char *funcname, char *file, int lineno, char *context) {
// the below is basically Postgres' `#define ereport(...)` macro unrolled

#if (IS_PG_11 || IS_PG_12)
#   ifdef HAVE__BUILTIN_CONSTANT_P
        if (errstart(level, file, lineno, funcname, NULL)) {
            if (detail != NULL) {
                if (context != NULL) {
                    errfinish(level, errcode(code), errmsg("%s", message), errdetail("%s", detail), errcontext_msg("%s", context));
                } else {
                    errfinish(level, errcode(code), errmsg("%s", message), errdetail("%s", detail));
                }
            } else {
                if (context != NULL) {
                    errfinish(level, errcode(code), errmsg("%s", message), errcontext_msg("%s", context));
                } else {
                    errfinish(level, errcode(code), errmsg("%s", message));
                }
            }
        }
        if (__builtin_constant_p(level) && level >= ERROR) {
            pg_unreachable();
        }
#   else    /* !HAVE__BUILTIN_CONSTANT_P */
        if (errstart(level, file, lineno, funcname, NULL)) {
            if (detail != NULL) {
                if (context != NULL) {
                    errfinish(level, errcode(code), errmsg("%s", message), errdetail("%s", detail), errcontext_msg(context));
                } else {
                    errfinish(level, errcode(code), errmsg("%s", message), errdetail("%s", detail));
                }
            } else {
                if (context != NULL) {
                    errfinish(level, errcode(code), errmsg("%s", message), errcontext_msg("%s", context));
                } else {
                    errfinish(level, errcode(code), errmsg("%s", message));
                }
            }
        }
        if (level >= ERROR) {
            pg_unreachable();
        }
#   endif   /* HAVE__BUILTIN_CONSTANT_P */
#else   /* IS_PG_11 || IS_PG_12 */
#   ifdef HAVE__BUILTIN_CONSTANT_P
        pg_prevent_errno_in_scope();
        if (errstart(level, NULL)) {
            errcode(code);
            errmsg("%s", message);
            if (detail != NULL) {
                errdetail("%s", detail);
            }
            if (context != NULL) {
                errcontext_msg("%s", context);
            }
            errfinish(file, lineno, funcname);
        }
        if (__builtin_constant_p(level) && level >= ERROR) {
            pg_unreachable();
        }
#   else    /* !HAVE__BUILTIN_CONSTANT_P */
        pg_prevent_errno_in_scope();
        if (errstart(level, domain)) {
            errcode(code);
            errmsg("%s", message);
            if (detail != NULL) {
                errdetail("%s", detail);
            }
            if (context != NULL) {
                errcontext_msg("%s", context);
            }
            errfinish(file, lineno, funcname);
        }
        if (level >= ERROR) {
            pg_unreachable();
        }
#   endif   /* HAVE__BUILTIN_CONSTANT_P */
#endif  /* IS_PG_11 || IS_PG_12 */
}

PGDLLEXPORT void pgx_SET_VARSIZE(struct varlena *ptr, int size);
void pgx_SET_VARSIZE(struct varlena *ptr, int size) {
    SET_VARSIZE(ptr, size);
}

PGDLLEXPORT void pgx_SET_VARSIZE_SHORT(struct varlena *ptr, int size);
void pgx_SET_VARSIZE_SHORT(struct varlena *ptr, int size) {
    SET_VARSIZE_SHORT(ptr, size);
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

PGDLLEXPORT RangeTblEntry *pgx_planner_rt_fetch(Index index, PlannerInfo *plannerInfo);
RangeTblEntry *pgx_planner_rt_fetch(Index index, PlannerInfo *root) {
    return planner_rt_fetch(index, root);
}

PGDLLEXPORT void *pgx_list_nth(List *list, int nth);
void *pgx_list_nth(List *list, int nth) {
    return list_nth(list, nth);
}

PGDLLEXPORT int pgx_list_nth_int(List *list, int nth);
int pgx_list_nth_int(List *list, int nth) {
    return list_nth_int(list, nth);
}

PGDLLEXPORT Oid pgx_list_nth_oid(List *list, int nth);
Oid pgx_list_nth_oid(List *list, int nth) {
    return list_nth_oid(list, nth);
}

PGDLLEXPORT ListCell *pgx_list_nth_cell(List *list, int nth);
ListCell *pgx_list_nth_cell(List *list, int nth) {
    return list_nth_cell(list, nth);
}

#if IS_PG_11
PGDLLEXPORT Oid pgx_HeapTupleHeaderGetOid(HeapTupleHeader htup_header);
Oid pgx_HeapTupleHeaderGetOid(HeapTupleHeader htup_header) {
    return HeapTupleHeaderGetOid(htup_header);
}
#endif

PGDLLEXPORT char *pgx_GETSTRUCT(HeapTuple tuple);
char *pgx_GETSTRUCT(HeapTuple tuple) {
    return GETSTRUCT(tuple);
}

PGDLLEXPORT char *pgx_ARR_DATA_PTR(ArrayType *arr);
char *pgx_ARR_DATA_PTR(ArrayType *arr) {
    return ARR_DATA_PTR(arr);
}

PGDLLEXPORT int pgx_ARR_NELEMS(ArrayType *arr);
int pgx_ARR_NELEMS(ArrayType *arr) {
    return ArrayGetNItems(arr->ndim, ARR_DIMS(arr));
}

PGDLLEXPORT bits8 *pgx_ARR_NULLBITMAP(ArrayType *arr);
bits8 *pgx_ARR_NULLBITMAP(ArrayType *arr) {
    return ARR_NULLBITMAP(arr);
}

PGDLLEXPORT int pgx_ARR_NDIM(ArrayType *arr);
int pgx_ARR_NDIM(ArrayType *arr) {
    return ARR_NDIM(arr);
}

PGDLLEXPORT bool pgx_ARR_HASNULL(ArrayType *arr);
bool pgx_ARR_HASNULL(ArrayType *arr) {
    return ARR_HASNULL(arr);
}

PGDLLEXPORT int *pgx_ARR_DIMS(ArrayType *arr);
int *pgx_ARR_DIMS(ArrayType *arr){
    return ARR_DIMS(arr);
}

PGDLLEXPORT void pgx_SpinLockInit(volatile slock_t *lock);
void pgx_SpinLockInit(volatile slock_t *lock) {
    SpinLockInit(lock);
}

PGDLLEXPORT void pgx_SpinLockAcquire(volatile slock_t *lock);
void pgx_SpinLockAcquire(volatile slock_t *lock) {
    SpinLockAcquire(lock);
}

PGDLLEXPORT void pgx_SpinLockRelease(volatile slock_t *lock);
void pgx_SpinLockRelease(volatile slock_t *lock) {
    SpinLockRelease(lock);
}

PGDLLEXPORT bool pgx_SpinLockFree(slock_t *lock);
bool pgx_SpinLockFree(slock_t *lock) {
    return SpinLockFree(lock);
}
