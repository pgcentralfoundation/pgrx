/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
#include "pgx-cshim.h"

MemoryContext pgx_GetMemoryContextChunk(void *ptr) {
    return GetMemoryChunkContext(ptr);
}

void pgx_elog(int32 level, const char *message) {
    elog(level, "%s", message);
}

void pgx_elog_error(const char *message) {
    elog(ERROR, "%s", message);
}

void pgx_ereport(int level, int code, const char *message, const char *file, int lineno, int colno) {
    ereport(level,
            (errcode(code),
                    errmsg("%s", message), errcontext_msg("%s:%d:%d", file, lineno, colno)));
}

void pgx_SET_VARSIZE(struct varlena *ptr, int size) {
    SET_VARSIZE(ptr, size);
}

void pgx_SET_VARSIZE_SHORT(struct varlena *ptr, int size) {
    SET_VARSIZE_SHORT(ptr, size);
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

RangeTblEntry *pgx_planner_rt_fetch(Index index, PlannerInfo *root) {
    return planner_rt_fetch(index, root);
}

void *pgx_list_nth(List *list, int nth) {
    return list_nth(list, nth);
}

int pgx_list_nth_int(List *list, int nth) {
    return list_nth_int(list, nth);
}
Oid pgx_list_nth_oid(List *list, int nth) {
    return list_nth_oid(list, nth);
}

ListCell *pgx_list_nth_cell(List *list, int nth) {
    return list_nth_cell(list, nth);
}

#if IS_PG_10 || IS_PG_11
Oid pgx_HeapTupleHeaderGetOid(HeapTupleHeader htup_header) {
    return HeapTupleHeaderGetOid(htup_header);
}
#endif

char *pgx_GETSTRUCT(HeapTuple tuple) {
    return GETSTRUCT(tuple);
}

char *pgx_ARR_DATA_PTR(ArrayType *arr) {
    return ARR_DATA_PTR(arr);
}

int pgx_ARR_NELEMS(ArrayType *arr) {
    return ArrayGetNItems(arr->ndim, ARR_DIMS(arr));
}

bits8 *pgx_ARR_NULLBITMAP(ArrayType *arr) {
    return ARR_NULLBITMAP(arr);
}

int pgx_ARR_NDIM(ArrayType *arr) {
    return ARR_NDIM(arr);
}

bool pgx_ARR_HASNULL(ArrayType *arr) {
    return ARR_HASNULL(arr);
}

int *pgx_ARR_DIMS(ArrayType *arr){
    return ARR_DIMS(arr);
}

void pgx_SpinLockInit(volatile slock_t *lock) {
    SpinLockInit(lock);
}

void pgx_SpinLockAcquire(volatile slock_t *lock) {
    SpinLockAcquire(lock);
}

void pgx_SpinLockRelease(volatile slock_t *lock) {
    SpinLockRelease(lock);
}

bool pgx_SpinLockFree(slock_t *lock) {
    return SpinLockFree(lock);
}
