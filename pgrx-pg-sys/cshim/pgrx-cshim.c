//LICENSE Portions Copyright 2019-2021 ZomboDB, LLC.
//LICENSE
//LICENSE Portions Copyright 2021-2023 Technology Concepts & Design, Inc.
//LICENSE
//LICENSE Portions Copyright 2023-2023 PgCentral Foundation, Inc. <contact@pgcentral.org>
//LICENSE
//LICENSE All rights reserved.
//LICENSE
//LICENSE Use of this source code is governed by the MIT license that can be found in the LICENSE file.


#include "postgres.h"

#define IS_PG_12 (PG_VERSION_NUM >= 120000 && PG_VERSION_NUM < 130000)
#define IS_PG_13 (PG_VERSION_NUM >= 130000 && PG_VERSION_NUM < 140000)

#include "access/tableam.h"
#include "executor/executor.h"
#include "executor/tuptable.h"
#include "nodes/pathnodes.h"
#include "nodes/pg_list.h"
#include "parser/parsetree.h"
#include "storage/spin.h"
#include "storage/bufpage.h"

PGDLLEXPORT RangeTblEntry *pgrx_planner_rt_fetch(Index index, PlannerInfo *plannerInfo);
RangeTblEntry *pgrx_planner_rt_fetch(Index index, PlannerInfo *root) {
    return planner_rt_fetch(index, root);
}

PGDLLEXPORT void *pgrx_list_nth(List *list, int nth);
void *pgrx_list_nth(List *list, int nth) {
    return list_nth(list, nth);
}

PGDLLEXPORT int pgrx_list_nth_int(List *list, int nth);
int pgrx_list_nth_int(List *list, int nth) {
    return list_nth_int(list, nth);
}

PGDLLEXPORT Oid pgrx_list_nth_oid(List *list, int nth);
Oid pgrx_list_nth_oid(List *list, int nth) {
    return list_nth_oid(list, nth);
}

PGDLLEXPORT ListCell *pgrx_list_nth_cell(List *list, int nth);
ListCell *pgrx_list_nth_cell(List *list, int nth) {
    return list_nth_cell(list, nth);
}

PGDLLEXPORT void pgrx_SpinLockInit(volatile slock_t *lock);
void pgrx_SpinLockInit(volatile slock_t *lock) {
    SpinLockInit(lock);
}

PGDLLEXPORT void pgrx_SpinLockAcquire(volatile slock_t *lock);
void pgrx_SpinLockAcquire(volatile slock_t *lock) {
    SpinLockAcquire(lock);
}

PGDLLEXPORT void pgrx_SpinLockRelease(volatile slock_t *lock);
void pgrx_SpinLockRelease(volatile slock_t *lock) {
    SpinLockRelease(lock);
}

PGDLLEXPORT bool pgrx_SpinLockFree(slock_t *lock);
bool pgrx_SpinLockFree(slock_t *lock) {
    return SpinLockFree(lock);
}

PGDLLEXPORT char * pgrx_PageGetSpecialPointer(Page page);
char * pgrx_PageGetSpecialPointer(Page page) {
    return PageGetSpecialPointer(page);
}

PGDLLEXPORT TableScanDesc pgrx_table_beginscan_strat(Relation relation, Snapshot snapshot, int nkeys, struct ScanKeyData * key, bool allow_strat, bool allow_sync);
TableScanDesc pgrx_table_beginscan_strat(Relation relation, Snapshot snapshot, int nkeys, struct ScanKeyData * key, bool allow_strat, bool allow_sync) {
    return table_beginscan_strat(relation, snapshot, nkeys, key, allow_strat, allow_sync);
}

PGDLLEXPORT void pgrx_table_endscan(TableScanDesc scan);
void pgrx_table_endscan(TableScanDesc scan) {
    return table_endscan(scan);
}

PGDLLEXPORT bool pgrx_ExecQual(ExprState * state, ExprContext * econtext);
bool pgrx_ExecQual(ExprState * state, ExprContext * econtext) {
    return ExecQual(state, econtext);
}

PGDLLEXPORT HeapTuple pgrx_ExecCopySlotHeapTuple(TupleTableSlot * slot);
HeapTuple pgrx_ExecCopySlotHeapTuple(TupleTableSlot * slot) {
    return ExecCopySlotHeapTuple(slot);
}
