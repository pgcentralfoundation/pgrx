/*
Portions Copyright 2019-2021 ZomboDB, LLC.
Portions Copyright 2021-2022 Technology Concepts & Design, Inc. <support@tcdi.com>

All rights reserved.

Use of this source code is governed by the MIT license that can be found in the LICENSE file.
*/
#pragma once
#include "postgres.h"

#define IS_PG_10 (PG_VERSION_NUM >= 100000 && PG_VERSION_NUM < 110000)
#define IS_PG_11 (PG_VERSION_NUM >= 110000 && PG_VERSION_NUM < 120000)
#define IS_PG_12 (PG_VERSION_NUM >= 120000 && PG_VERSION_NUM < 130000)
#define IS_PG_13 (PG_VERSION_NUM >= 130000 && PG_VERSION_NUM < 140000)

#include "access/htup.h"
#include "access/htup_details.h"
#include "catalog/pg_type.h"
#if IS_PG_10 || IS_PG_11
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
PGDLLEXPORT void pgx_elog(int32 level, const char *message);
PGDLLEXPORT void pgx_elog_error(const char *message);
PGDLLEXPORT void pgx_ereport(int level, int code, const char *message, const char *file, int lineno, int colno);
PGDLLEXPORT void pgx_SET_VARSIZE(struct varlena *ptr, int size);
PGDLLEXPORT void pgx_SET_VARSIZE_SHORT(struct varlena *ptr, int size);
PGDLLEXPORT Datum pgx_heap_getattr(HeapTupleData *tuple, int attnum, TupleDesc tupdesc, bool *isnull);
PGDLLEXPORT TransactionId pgx_HeapTupleHeaderGetXmin(HeapTupleHeader htup_header);
PGDLLEXPORT CommandId pgx_HeapTupleHeaderGetRawCommandId(HeapTupleHeader htup_header);
PGDLLEXPORT RangeTblEntry *pgx_planner_rt_fetch(Index index, PlannerInfo *plannerInfo);
PGDLLEXPORT void *pgx_list_nth(List *list, int nth);
PGDLLEXPORT int pgx_list_nth_int(List *list, int nth);
PGDLLEXPORT Oid pgx_list_nth_oid(List *list, int nth);
PGDLLEXPORT ListCell *pgx_list_nth_cell(List *list, int nth);
#if IS_PG_10 || IS_PG_11
PGDLLEXPORT Oid pgx_HeapTupleHeaderGetOid(HeapTupleHeader htup_header);
#endif
PGDLLEXPORT char *pgx_GETSTRUCT(HeapTuple tuple);
PGDLLEXPORT char *pgx_ARR_DATA_PTR(ArrayType *arr);
PGDLLEXPORT int pgx_ARR_NELEMS(ArrayType *arr);
PGDLLEXPORT bits8 *pgx_ARR_NULLBITMAP(ArrayType *arr);
PGDLLEXPORT int pgx_ARR_NDIM(ArrayType *arr);
PGDLLEXPORT bool pgx_ARR_HASNULL(ArrayType *arr);
PGDLLEXPORT int *pgx_ARR_DIMS(ArrayType *arr);
PGDLLEXPORT void pgx_SpinLockInit(volatile slock_t *lock);
PGDLLEXPORT void pgx_SpinLockAcquire(volatile slock_t *lock);
PGDLLEXPORT void pgx_SpinLockRelease(volatile slock_t *lock);
PGDLLEXPORT bool pgx_SpinLockFree(slock_t *lock);
