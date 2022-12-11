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
