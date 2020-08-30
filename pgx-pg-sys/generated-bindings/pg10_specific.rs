use crate as pg_sys;
use crate::common::*;
use pgx_macros::*;
#[doc = "\tQuery Tree"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Query {
    pub type_: NodeTag,
    pub commandType: CmdType,
    pub querySource: QuerySource,
    pub queryId: uint32,
    pub canSetTag: bool,
    pub utilityStmt: *mut Node,
    pub resultRelation: ::std::os::raw::c_int,
    pub hasAggs: bool,
    pub hasWindowFuncs: bool,
    pub hasTargetSRFs: bool,
    pub hasSubLinks: bool,
    pub hasDistinctOn: bool,
    pub hasRecursive: bool,
    pub hasModifyingCTE: bool,
    pub hasForUpdate: bool,
    pub hasRowSecurity: bool,
    pub cteList: *mut List,
    pub rtable: *mut List,
    pub jointree: *mut FromExpr,
    pub targetList: *mut List,
    pub override_: OverridingKind,
    pub onConflict: *mut OnConflictExpr,
    pub returningList: *mut List,
    pub groupClause: *mut List,
    pub groupingSets: *mut List,
    pub havingQual: *mut Node,
    pub windowClause: *mut List,
    pub distinctClause: *mut List,
    pub sortClause: *mut List,
    pub limitOffset: *mut Node,
    pub limitCount: *mut Node,
    pub rowMarks: *mut List,
    pub setOperations: *mut Node,
    pub constraintDeps: *mut List,
    pub withCheckOptions: *mut List,
    pub stmt_location: ::std::os::raw::c_int,
    pub stmt_len: ::std::os::raw::c_int,
}
#[pg_guard]
extern "C" {
    pub fn AllocSetContextCreate(
        parent: MemoryContext,
        name: *const ::std::os::raw::c_char,
        minContextSize: Size,
        initBlockSize: Size,
        maxBlockSize: Size,
    ) -> MemoryContext;
}
#[pg_guard]
extern "C" {
    pub fn AtEOXact_Files();
}
#[pg_guard]
extern "C" {
    pub fn BackgroundWorkerInitializeConnection(
        dbname: *mut ::std::os::raw::c_char,
        username: *mut ::std::os::raw::c_char,
    );
}
#[pg_guard]
extern "C" {
    pub fn BackgroundWorkerInitializeConnectionByOid(dboid: Oid, useroid: Oid);
}
#[pg_guard]
extern "C" {
    pub fn BasicOpenFile(
        fileName: FileName,
        fileFlags: ::std::os::raw::c_int,
        fileMode: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub fn BeginInternalSubTransaction(name: *mut ::std::os::raw::c_char);
}
#[pg_guard]
extern "C" {
    pub fn BuildTupleHashTable(
        numCols: ::std::os::raw::c_int,
        keyColIdx: *mut AttrNumber,
        eqfunctions: *mut FmgrInfo,
        hashfunctions: *mut FmgrInfo,
        nbuckets: ::std::os::raw::c_long,
        additionalsize: Size,
        tablecxt: MemoryContext,
        tempcxt: MemoryContext,
        use_variable_hash_iv: bool,
    ) -> TupleHashTable;
}
#[pg_guard]
extern "C" {
    pub fn CommuteRowCompareExpr(clause: *mut RowCompareExpr);
}
#[pg_guard]
extern "C" {
    pub fn ConditionVariableBroadcast(arg1: *mut ConditionVariable) -> ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub fn ConditionVariableInit(arg1: *mut ConditionVariable);
}
#[pg_guard]
extern "C" {
    pub fn ConditionVariablePrepareToSleep(arg1: *mut ConditionVariable);
}
#[pg_guard]
extern "C" {
    pub fn ConditionVariableSignal(arg1: *mut ConditionVariable) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn ConditionVariableSleep(arg1: *mut ConditionVariable, wait_event_info: uint32);
}
#[pg_guard]
extern "C" {
    pub fn CreateParallelContext(
        library_name: *const ::std::os::raw::c_char,
        function_name: *const ::std::os::raw::c_char,
        nworkers: ::std::os::raw::c_int,
    ) -> *mut ParallelContext;
}
#[pg_guard]
extern "C" {
    pub fn CreateTemplateTupleDesc(natts: ::std::os::raw::c_int, hasoid: bool) -> TupleDesc;
}
#[pg_guard]
extern "C" {
    pub fn CreateTrigger(
        stmt: *mut CreateTrigStmt,
        queryString: *const ::std::os::raw::c_char,
        relOid: Oid,
        refRelOid: Oid,
        constraintOid: Oid,
        indexOid: Oid,
        isInternal: bool,
    ) -> ObjectAddress;
}
#[pg_guard]
extern "C" {
    pub fn CreateTupleDesc(
        natts: ::std::os::raw::c_int,
        hasoid: bool,
        attrs: *mut Form_pg_attribute,
    ) -> TupleDesc;
}
#[pg_guard]
extern "C" {
    pub fn DatumGetAnyArray(d: Datum) -> *mut AnyArrayType;
}
#[pg_guard]
extern "C" {
    pub fn DefineSavepoint(name: *mut ::std::os::raw::c_char);
}
#[pg_guard]
extern "C" {
    pub fn EnableDisableTrigger(
        rel: Relation,
        tgname: *const ::std::os::raw::c_char,
        fires_when: ::std::os::raw::c_char,
        skip_system: bool,
    );
}
#[pg_guard]
extern "C" {
    pub fn EndTransactionBlock() -> bool;
}
#[pg_guard]
extern "C" {
    pub fn EvalPlanQual(
        estate: *mut EState,
        epqstate: *mut EPQState,
        relation: Relation,
        rti: Index,
        lockmode: ::std::os::raw::c_int,
        tid: ItemPointer,
        priorXmax: TransactionId,
    ) -> *mut TupleTableSlot;
}
#[pg_guard]
extern "C" {
    pub fn EvalPlanQualBegin(epqstate: *mut EPQState, parentestate: *mut EState);
}
#[pg_guard]
extern "C" {
    pub fn EvalPlanQualFetch(
        estate: *mut EState,
        relation: Relation,
        lockmode: ::std::os::raw::c_int,
        wait_policy: LockWaitPolicy,
        tid: ItemPointer,
        priorXmax: TransactionId,
    ) -> HeapTuple;
}
#[pg_guard]
extern "C" {
    pub fn EvalPlanQualFetchRowMarks(epqstate: *mut EPQState);
}
#[pg_guard]
extern "C" {
    pub fn EvalPlanQualGetTuple(epqstate: *mut EPQState, rti: Index) -> HeapTuple;
}
#[pg_guard]
extern "C" {
    pub fn EvalPlanQualInit(
        epqstate: *mut EPQState,
        estate: *mut EState,
        subplan: *mut Plan,
        auxrowmarks: *mut List,
        epqParam: ::std::os::raw::c_int,
    );
}
#[pg_guard]
extern "C" {
    pub fn EvalPlanQualSetTuple(epqstate: *mut EPQState, rti: Index, tuple: HeapTuple);
}
#[pg_guard]
extern "C" {
    pub fn EventTriggerSupportsGrantObjectType(objtype: GrantObjectType) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn ExecARInsertTriggers(
        estate: *mut EState,
        relinfo: *mut ResultRelInfo,
        trigtuple: HeapTuple,
        recheckIndexes: *mut List,
        transition_capture: *mut TransitionCaptureState,
    );
}
#[pg_guard]
extern "C" {
    pub fn ExecARUpdateTriggers(
        estate: *mut EState,
        relinfo: *mut ResultRelInfo,
        tupleid: ItemPointer,
        fdw_trigtuple: HeapTuple,
        newtuple: HeapTuple,
        recheckIndexes: *mut List,
        transition_capture: *mut TransitionCaptureState,
    );
}
#[pg_guard]
extern "C" {
    pub fn ExecAllocTableSlot(tupleTable: *mut *mut List) -> *mut TupleTableSlot;
}
#[pg_guard]
extern "C" {
    pub fn ExecAssignResultType(planstate: *mut PlanState, tupDesc: TupleDesc);
}
#[pg_guard]
extern "C" {
    pub fn ExecAssignResultTypeFromTL(planstate: *mut PlanState);
}
#[pg_guard]
extern "C" {
    pub fn ExecAssignScanTypeFromOuterPlan(scanstate: *mut ScanState);
}
#[pg_guard]
extern "C" {
    pub fn ExecBRDeleteTriggers(
        estate: *mut EState,
        epqstate: *mut EPQState,
        relinfo: *mut ResultRelInfo,
        tupleid: ItemPointer,
        fdw_trigtuple: HeapTuple,
    ) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn ExecBRInsertTriggers(
        estate: *mut EState,
        relinfo: *mut ResultRelInfo,
        slot: *mut TupleTableSlot,
    ) -> *mut TupleTableSlot;
}
#[pg_guard]
extern "C" {
    pub fn ExecBRUpdateTriggers(
        estate: *mut EState,
        epqstate: *mut EPQState,
        relinfo: *mut ResultRelInfo,
        tupleid: ItemPointer,
        fdw_trigtuple: HeapTuple,
        slot: *mut TupleTableSlot,
    ) -> *mut TupleTableSlot;
}
#[pg_guard]
extern "C" {
    pub fn ExecCleanTypeFromTL(targetList: *mut List, hasoid: bool) -> TupleDesc;
}
#[pg_guard]
extern "C" {
    pub fn ExecClearTuple(slot: *mut TupleTableSlot) -> *mut TupleTableSlot;
}
#[pg_guard]
extern "C" {
    pub fn ExecCloseScanRelation(scanrel: Relation);
}
#[pg_guard]
extern "C" {
    pub fn ExecContextForcesOids(planstate: *mut PlanState, hasoids: *mut bool) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn ExecCopySlot(
        dstslot: *mut TupleTableSlot,
        srcslot: *mut TupleTableSlot,
    ) -> *mut TupleTableSlot;
}
#[pg_guard]
extern "C" {
    pub fn ExecCopySlotMinimalTuple(slot: *mut TupleTableSlot) -> MinimalTuple;
}
#[pg_guard]
extern "C" {
    pub fn ExecCopySlotTuple(slot: *mut TupleTableSlot) -> HeapTuple;
}
#[pg_guard]
extern "C" {
    pub fn ExecFetchSlotMinimalTuple(slot: *mut TupleTableSlot) -> MinimalTuple;
}
#[pg_guard]
extern "C" {
    pub fn ExecFetchSlotTuple(slot: *mut TupleTableSlot) -> HeapTuple;
}
#[pg_guard]
extern "C" {
    pub fn ExecFetchSlotTupleDatum(slot: *mut TupleTableSlot) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn ExecFindPartition(
        resultRelInfo: *mut ResultRelInfo,
        pd: *mut PartitionDispatch,
        slot: *mut TupleTableSlot,
        estate: *mut EState,
    ) -> ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub fn ExecIRInsertTriggers(
        estate: *mut EState,
        relinfo: *mut ResultRelInfo,
        slot: *mut TupleTableSlot,
    ) -> *mut TupleTableSlot;
}
#[pg_guard]
extern "C" {
    pub fn ExecIRUpdateTriggers(
        estate: *mut EState,
        relinfo: *mut ResultRelInfo,
        trigtuple: HeapTuple,
        slot: *mut TupleTableSlot,
    ) -> *mut TupleTableSlot;
}
#[pg_guard]
extern "C" {
    pub fn ExecInitExtraTupleSlot(estate: *mut EState) -> *mut TupleTableSlot;
}
#[pg_guard]
extern "C" {
    pub fn ExecInitJunkFilter(
        targetList: *mut List,
        hasoid: bool,
        slot: *mut TupleTableSlot,
    ) -> *mut JunkFilter;
}
#[pg_guard]
extern "C" {
    pub fn ExecInitNullTupleSlot(estate: *mut EState, tupType: TupleDesc) -> *mut TupleTableSlot;
}
#[pg_guard]
extern "C" {
    pub fn ExecInitResultTupleSlot(estate: *mut EState, planstate: *mut PlanState);
}
#[pg_guard]
extern "C" {
    pub fn ExecInitScanTupleSlot(estate: *mut EState, scanstate: *mut ScanState);
}
#[pg_guard]
extern "C" {
    pub fn ExecInsertIndexTuples(
        slot: *mut TupleTableSlot,
        tupleid: ItemPointer,
        estate: *mut EState,
        noDupErr: bool,
        specConflict: *mut bool,
        arbiterIndexes: *mut List,
    ) -> *mut List;
}
#[pg_guard]
extern "C" {
    pub fn ExecLockNonLeafAppendTables(partitioned_rels: *mut List, estate: *mut EState);
}
#[pg_guard]
extern "C" {
    pub fn ExecMakeFunctionResultSet(
        fcache: *mut SetExprState,
        econtext: *mut ExprContext,
        isNull: *mut bool,
        isDone: *mut ExprDoneCond,
    ) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn ExecMaterializeSlot(slot: *mut TupleTableSlot) -> HeapTuple;
}
#[pg_guard]
extern "C" {
    pub fn ExecSetupPartitionTupleRouting(
        rel: Relation,
        resultRTindex: Index,
        estate: *mut EState,
        pd: *mut *mut PartitionDispatch,
        partitions: *mut *mut ResultRelInfo,
        tup_conv_maps: *mut *mut *mut TupleConversionMap,
        partition_tuple_slot: *mut *mut TupleTableSlot,
        num_parted: *mut ::std::os::raw::c_int,
        num_partitions: *mut ::std::os::raw::c_int,
    );
}
#[pg_guard]
extern "C" {
    pub fn ExecStoreTuple(
        tuple: HeapTuple,
        slot: *mut TupleTableSlot,
        buffer: Buffer,
        shouldFree: bool,
    ) -> *mut TupleTableSlot;
}
#[pg_guard]
extern "C" {
    pub fn ExecTypeFromTL(targetList: *mut List, hasoid: bool) -> TupleDesc;
}
#[pg_guard]
extern "C" {
    pub fn ExplainPropertyFloat(
        qlabel: *const ::std::os::raw::c_char,
        value: f64,
        ndigits: ::std::os::raw::c_int,
        es: *mut ExplainState,
    );
}
#[pg_guard]
extern "C" {
    pub fn ExplainPropertyInteger(
        qlabel: *const ::std::os::raw::c_char,
        value: ::std::os::raw::c_int,
        es: *mut ExplainState,
    );
}
#[pg_guard]
extern "C" {
    pub fn ExplainPropertyLong(
        qlabel: *const ::std::os::raw::c_char,
        value: ::std::os::raw::c_long,
        es: *mut ExplainState,
    );
}
#[pg_guard]
extern "C" {
    pub fn FileGetRawMode(file: File) -> ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub fn FileRead(
        file: File,
        buffer: *mut ::std::os::raw::c_char,
        amount: ::std::os::raw::c_int,
        wait_event_info: uint32,
    ) -> ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub fn FileSeek(file: File, offset: off_t, whence: ::std::os::raw::c_int) -> off_t;
}
#[pg_guard]
extern "C" {
    pub fn FileWrite(
        file: File,
        buffer: *mut ::std::os::raw::c_char,
        amount: ::std::os::raw::c_int,
        wait_event_info: uint32,
    ) -> ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub fn FindTupleHashEntry(
        hashtable: TupleHashTable,
        slot: *mut TupleTableSlot,
        eqfunctions: *mut FmgrInfo,
        hashfunctions: *mut FmgrInfo,
    ) -> TupleHashEntry;
}
#[pg_guard]
extern "C" {
    pub fn FormPartitionKeyDatum(
        pd: PartitionDispatch,
        slot: *mut TupleTableSlot,
        estate: *mut EState,
        values: *mut Datum,
        isnull: *mut bool,
    );
}
#[pg_guard]
extern "C" {
    pub fn FreeBulkInsertState(arg1: BulkInsertState);
}
#[pg_guard]
extern "C" {
    pub fn GetBulkInsertState() -> BulkInsertState;
}
#[pg_guard]
extern "C" {
    pub fn GetLockConflicts(
        locktag: *const LOCKTAG,
        lockmode: LOCKMODE,
    ) -> *mut VirtualTransactionId;
}
#[pg_guard]
extern "C" {
    pub fn GetNewTransactionId(isSubXact: bool) -> TransactionId;
}
#[pg_guard]
extern "C" {
    pub fn GetNextXidAndEpoch(xid: *mut TransactionId, epoch: *mut uint32);
}
#[pg_guard]
extern "C" {
    pub fn GetSysCacheOid(
        cacheId: ::std::os::raw::c_int,
        key1: Datum,
        key2: Datum,
        key3: Datum,
        key4: Datum,
    ) -> Oid;
}
#[pg_guard]
extern "C" {
    pub fn IndexBuildHeapRangeScan(
        heapRelation: Relation,
        indexRelation: Relation,
        indexInfo: *mut IndexInfo,
        allow_sync: bool,
        anyvisible: bool,
        start_blockno: BlockNumber,
        end_blockno: BlockNumber,
        callback: IndexBuildCallback,
        callback_state: *mut ::std::os::raw::c_void,
    ) -> f64;
}
#[pg_guard]
extern "C" {
    pub fn IndexBuildHeapScan(
        heapRelation: Relation,
        indexRelation: Relation,
        indexInfo: *mut IndexInfo,
        allow_sync: bool,
        callback: IndexBuildCallback,
        callback_state: *mut ::std::os::raw::c_void,
    ) -> f64;
}
#[pg_guard]
extern "C" {
    pub fn InitPostgres(
        in_dbname: *const ::std::os::raw::c_char,
        dboid: Oid,
        username: *const ::std::os::raw::c_char,
        useroid: Oid,
        out_dbname: *mut ::std::os::raw::c_char,
    );
}
#[pg_guard]
extern "C" {
    pub fn IsInTransactionChain(isTopLevel: bool) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn LWLockRegisterTranche(
        tranche_id: ::std::os::raw::c_int,
        tranche_name: *mut ::std::os::raw::c_char,
    );
}
#[pg_guard]
extern "C" {
    pub fn LookupAggWithArgs(agg: *mut ObjectWithArgs, noError: bool) -> Oid;
}
#[pg_guard]
extern "C" {
    pub fn LookupFuncName(
        funcname: *mut List,
        nargs: ::std::os::raw::c_int,
        argtypes: *const Oid,
        noError: bool,
    ) -> Oid;
}
#[pg_guard]
extern "C" {
    pub fn LookupFuncWithArgs(func: *mut ObjectWithArgs, noError: bool) -> Oid;
}
#[pg_guard]
extern "C" {
    pub fn MakeSingleTupleTableSlot(tupdesc: TupleDesc) -> *mut TupleTableSlot;
}
#[pg_guard]
extern "C" {
    pub fn MakeTupleTableSlot() -> *mut TupleTableSlot;
}
#[pg_guard]
extern "C" {
    pub fn MemoryContextCreate(
        tag: NodeTag,
        size: Size,
        methods: *mut MemoryContextMethods,
        parent: MemoryContext,
        name: *const ::std::os::raw::c_char,
    ) -> MemoryContext;
}
#[pg_guard]
extern "C" {
    pub fn OpenTransientFile(
        fileName: FileName,
        fileFlags: ::std::os::raw::c_int,
        fileMode: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub fn ParseFuncOrColumn(
        pstate: *mut ParseState,
        funcname: *mut List,
        fargs: *mut List,
        last_srf: *mut Node,
        fn_: *mut FuncCall,
        location: ::std::os::raw::c_int,
    ) -> *mut Node;
}
#[pg_guard]
extern "C" {
    pub fn PathNameOpenFile(
        fileName: FileName,
        fileFlags: ::std::os::raw::c_int,
        fileMode: ::std::os::raw::c_int,
    ) -> File;
}
#[pg_guard]
extern "C" {
    pub fn PrepareTransactionBlock(gid: *mut ::std::os::raw::c_char) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn PreventTransactionChain(isTopLevel: bool, stmtType: *const ::std::os::raw::c_char);
}
#[pg_guard]
extern "C" {
    pub fn RI_FKey_fk_upd_check_required(
        trigger: *mut Trigger,
        fk_rel: Relation,
        old_row: HeapTuple,
        new_row: HeapTuple,
    ) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn RI_FKey_pk_upd_check_required(
        trigger: *mut Trigger,
        pk_rel: Relation,
        old_row: HeapTuple,
        new_row: HeapTuple,
    ) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn RangeVarGetRelidExtended(
        relation: *const RangeVar,
        lockmode: LOCKMODE,
        missing_ok: bool,
        nowait: bool,
        callback: RangeVarGetRelidCallback,
        callback_arg: *mut ::std::os::raw::c_void,
    ) -> Oid;
}
#[pg_guard]
extern "C" {
    pub fn ReadNewTransactionId() -> TransactionId;
}
#[pg_guard]
extern "C" {
    pub fn RelationBuildLocalRelation(
        relname: *const ::std::os::raw::c_char,
        relnamespace: Oid,
        tupDesc: TupleDesc,
        relid: Oid,
        relfilenode: Oid,
        reltablespace: Oid,
        shared_relation: bool,
        mapped_relation: bool,
        relpersistence: ::std::os::raw::c_char,
        relkind: ::std::os::raw::c_char,
    ) -> Relation;
}
#[pg_guard]
extern "C" {
    pub fn RelationBuildPartitionDesc(relation: Relation);
}
#[pg_guard]
extern "C" {
    pub fn RelationGetOidIndex(relation: Relation) -> Oid;
}
#[pg_guard]
extern "C" {
    pub fn RelationGetPartitionDispatchInfo(
        rel: Relation,
        num_parted: *mut ::std::os::raw::c_int,
        leaf_part_oids: *mut *mut List,
    ) -> *mut PartitionDispatch;
}
#[pg_guard]
extern "C" {
    pub fn RelationGetPartitionQual(rel: Relation) -> *mut List;
}
#[pg_guard]
extern "C" {
    pub fn RelationSetIndexList(relation: Relation, indexIds: *mut List, oidIndex: Oid);
}
#[pg_guard]
extern "C" {
    pub fn RelationSetNewRelfilenode(
        relation: Relation,
        persistence: ::std::os::raw::c_char,
        freezeXid: TransactionId,
        minmulti: MultiXactId,
    );
}
#[pg_guard]
extern "C" {
    pub fn ReleaseBulkInsertStatePin(bistate: BulkInsertState);
}
#[pg_guard]
extern "C" {
    pub fn ReleaseSavepoint(options: *mut List);
}
#[pg_guard]
extern "C" {
    pub fn RemoveEventTriggerById(ctrigOid: Oid);
}
#[pg_guard]
extern "C" {
    pub fn RenameRelationInternal(
        myrelid: Oid,
        newrelname: *const ::std::os::raw::c_char,
        is_internal: bool,
    );
}
#[pg_guard]
extern "C" {
    pub fn RequireTransactionChain(isTopLevel: bool, stmtType: *const ::std::os::raw::c_char);
}
#[pg_guard]
extern "C" {
    pub fn RollbackToSavepoint(options: *mut List);
}
#[pg_guard]
extern "C" {
    pub fn SearchSysCacheList(
        cacheId: ::std::os::raw::c_int,
        nkeys: ::std::os::raw::c_int,
        key1: Datum,
        key2: Datum,
        key3: Datum,
        key4: Datum,
    ) -> *mut catclist;
}
#[pg_guard]
extern "C" {
    pub fn StandbyReleaseOldLocks(nxids: ::std::os::raw::c_int, xids: *mut TransactionId);
}
#[pg_guard]
extern "C" {
    pub fn SyncScanShmemInit();
}
#[pg_guard]
extern "C" {
    pub fn SyncScanShmemSize() -> Size;
}
#[pg_guard]
extern "C" {
    pub fn TemporalTransform(max_precis: int32, node: *mut Node) -> *mut Node;
}
#[pg_guard]
extern "C" {
    pub fn TupleDescGetSlot(tupdesc: TupleDesc) -> *mut TupleTableSlot;
}
#[pg_guard]
extern "C" {
    pub fn UserAbortTransactionBlock();
}
#[pg_guard]
extern "C" {
    pub fn WarnNoTransactionChain(isTopLevel: bool, stmtType: *const ::std::os::raw::c_char);
}
#[pg_guard]
extern "C" {
    pub fn XLogReaderAllocate(
        pagereadfunc: XLogPageReadCB,
        private_data: *mut ::std::os::raw::c_void,
    ) -> *mut XLogReaderState;
}
#[pg_guard]
extern "C" {
    pub fn XactLogAbortRecord(
        abort_time: TimestampTz,
        nsubxacts: ::std::os::raw::c_int,
        subxacts: *mut TransactionId,
        nrels: ::std::os::raw::c_int,
        rels: *mut RelFileNode,
        xactflags: ::std::os::raw::c_int,
        twophase_xid: TransactionId,
    ) -> XLogRecPtr;
}
#[pg_guard]
extern "C" {
    pub fn XactLogCommitRecord(
        commit_time: TimestampTz,
        nsubxacts: ::std::os::raw::c_int,
        subxacts: *mut TransactionId,
        nrels: ::std::os::raw::c_int,
        rels: *mut RelFileNode,
        nmsgs: ::std::os::raw::c_int,
        msgs: *mut SharedInvalidationMessage,
        relcacheInval: bool,
        forceSync: bool,
        xactflags: ::std::os::raw::c_int,
        twophase_xid: TransactionId,
    ) -> XLogRecPtr;
}
#[pg_guard]
extern "C" {
    pub fn abstime_date(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn abstime_finite(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn abstime_timestamp(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn abstime_timestamptz(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn abstimeeq(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn abstimege(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn abstimegt(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn abstimein(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn abstimele(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn abstimelt(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn abstimene(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn abstimeout(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn abstimerecv(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn abstimesend(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn aclcheck_error(
        aclerr: AclResult,
        objectkind: AclObjectKind,
        objectname: *const ::std::os::raw::c_char,
    );
}
#[pg_guard]
extern "C" {
    pub fn aclcheck_error_col(
        aclerr: AclResult,
        objectkind: AclObjectKind,
        objectname: *const ::std::os::raw::c_char,
        colname: *const ::std::os::raw::c_char,
    );
}
#[pg_guard]
extern "C" {
    pub fn acldefault(objtype: GrantObjectType, ownerId: Oid) -> *mut Acl;
}
#[pg_guard]
extern "C" {
    pub fn add_bool_reloption(
        kinds: bits32,
        name: *mut ::std::os::raw::c_char,
        desc: *mut ::std::os::raw::c_char,
        default_val: bool,
    );
}
#[pg_guard]
extern "C" {
    pub fn add_int_reloption(
        kinds: bits32,
        name: *mut ::std::os::raw::c_char,
        desc: *mut ::std::os::raw::c_char,
        default_val: ::std::os::raw::c_int,
        min_val: ::std::os::raw::c_int,
        max_val: ::std::os::raw::c_int,
    );
}
#[pg_guard]
extern "C" {
    pub fn add_real_reloption(
        kinds: bits32,
        name: *mut ::std::os::raw::c_char,
        desc: *mut ::std::os::raw::c_char,
        default_val: f64,
        min_val: f64,
        max_val: f64,
    );
}
#[pg_guard]
extern "C" {
    pub fn add_string_reloption(
        kinds: bits32,
        name: *mut ::std::os::raw::c_char,
        desc: *mut ::std::os::raw::c_char,
        default_val: *mut ::std::os::raw::c_char,
        validator: validate_string_relopt,
    );
}
#[pg_guard]
extern "C" {
    pub fn adjust_rowcompare_for_index(
        clause: *mut RowCompareExpr,
        index: *mut IndexOptInfo,
        indexcol: ::std::os::raw::c_int,
        indexcolnos: *mut *mut List,
        var_on_left_p: *mut bool,
    ) -> *mut Expr;
}
#[pg_guard]
extern "C" {
    pub fn and_clause(clause: *mut Node) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn array_map(fcinfo: FunctionCallInfo, retType: Oid, amstate: *mut ArrayMapState) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn begin_tup_output_tupdesc(
        dest: *mut DestReceiver,
        tupdesc: TupleDesc,
    ) -> *mut TupOutputState;
}
#[pg_guard]
extern "C" {
    pub fn btabstimecmp(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn btreltimecmp(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn bttintervalcmp(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn build_empty_join_rel(root: *mut PlannerInfo) -> *mut RelOptInfo;
}
#[pg_guard]
extern "C" {
    pub fn build_function_result_tupdesc_d(
        proallargtypes: Datum,
        proargmodes: Datum,
        proargnames: Datum,
    ) -> TupleDesc;
}
#[pg_guard]
extern "C" {
    pub fn calc_nestloop_required_outer(outer_path: *mut Path, inner_path: *mut Path) -> Relids;
}
#[pg_guard]
extern "C" {
    pub fn check_new_partition_bound(
        relname: *mut ::std::os::raw::c_char,
        parent: Relation,
        spec: *mut PartitionBoundSpec,
    );
}
#[pg_guard]
extern "C" {
    pub fn compute_parallel_worker(
        rel: *mut RelOptInfo,
        heap_pages: f64,
        index_pages: f64,
    ) -> ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub fn compute_semi_anti_join_factors(
        root: *mut PlannerInfo,
        outerrel: *mut RelOptInfo,
        innerrel: *mut RelOptInfo,
        jointype: JoinType,
        sjinfo: *mut SpecialJoinInfo,
        restrictlist: *mut List,
        semifactors: *mut SemiAntiJoinFactors,
    );
}
#[pg_guard]
extern "C" {
    pub fn cost_agg(
        path: *mut Path,
        root: *mut PlannerInfo,
        aggstrategy: AggStrategy,
        aggcosts: *const AggClauseCosts,
        numGroupCols: ::std::os::raw::c_int,
        numGroups: f64,
        input_startup_cost: Cost,
        input_total_cost: Cost,
        input_tuples: f64,
    );
}
#[pg_guard]
extern "C" {
    pub fn cost_group(
        path: *mut Path,
        root: *mut PlannerInfo,
        numGroupCols: ::std::os::raw::c_int,
        numGroups: f64,
        input_startup_cost: Cost,
        input_total_cost: Cost,
        input_tuples: f64,
    );
}
#[pg_guard]
extern "C" {
    pub fn cost_tableexprscan(
        path: *mut Path,
        root: *mut PlannerInfo,
        baserel: *mut RelOptInfo,
        param_info: *mut ParamPathInfo,
    );
}
#[pg_guard]
extern "C" {
    pub fn create_append_path(
        rel: *mut RelOptInfo,
        subpaths: *mut List,
        required_outer: Relids,
        parallel_workers: ::std::os::raw::c_int,
        partitioned_rels: *mut List,
    ) -> *mut AppendPath;
}
#[pg_guard]
extern "C" {
    pub fn create_group_path(
        root: *mut PlannerInfo,
        rel: *mut RelOptInfo,
        subpath: *mut Path,
        target: *mut PathTarget,
        groupClause: *mut List,
        qual: *mut List,
        numGroups: f64,
    ) -> *mut GroupPath;
}
#[pg_guard]
extern "C" {
    pub fn create_groupingsets_path(
        root: *mut PlannerInfo,
        rel: *mut RelOptInfo,
        subpath: *mut Path,
        target: *mut PathTarget,
        having_qual: *mut List,
        aggstrategy: AggStrategy,
        rollups: *mut List,
        agg_costs: *const AggClauseCosts,
        numGroups: f64,
    ) -> *mut GroupingSetsPath;
}
#[pg_guard]
extern "C" {
    pub fn create_hashjoin_path(
        root: *mut PlannerInfo,
        joinrel: *mut RelOptInfo,
        jointype: JoinType,
        workspace: *mut JoinCostWorkspace,
        extra: *mut JoinPathExtraData,
        outer_path: *mut Path,
        inner_path: *mut Path,
        restrict_clauses: *mut List,
        required_outer: Relids,
        hashclauses: *mut List,
    ) -> *mut HashPath;
}
#[pg_guard]
extern "C" {
    pub fn create_index_path(
        root: *mut PlannerInfo,
        index: *mut IndexOptInfo,
        indexclauses: *mut List,
        indexclausecols: *mut List,
        indexorderbys: *mut List,
        indexorderbycols: *mut List,
        pathkeys: *mut List,
        indexscandir: ScanDirection,
        indexonly: bool,
        required_outer: Relids,
        loop_count: f64,
        partial_path: bool,
    ) -> *mut IndexPath;
}
#[pg_guard]
extern "C" {
    pub fn create_modifytable_path(
        root: *mut PlannerInfo,
        rel: *mut RelOptInfo,
        operation: CmdType,
        canSetTag: bool,
        nominalRelation: Index,
        partitioned_rels: *mut List,
        resultRelations: *mut List,
        subpaths: *mut List,
        subroots: *mut List,
        withCheckOptionLists: *mut List,
        returningLists: *mut List,
        rowMarks: *mut List,
        onconflict: *mut OnConflictExpr,
        epqParam: ::std::os::raw::c_int,
    ) -> *mut ModifyTablePath;
}
#[pg_guard]
extern "C" {
    pub fn create_result_path(
        root: *mut PlannerInfo,
        rel: *mut RelOptInfo,
        target: *mut PathTarget,
        resconstantqual: *mut List,
    ) -> *mut ResultPath;
}
#[pg_guard]
extern "C" {
    pub fn create_tablexprscan_path(
        root: *mut PlannerInfo,
        rel: *mut RelOptInfo,
        pathkeys: *mut List,
        required_outer: Relids,
    ) -> *mut Path;
}
#[pg_guard]
extern "C" {
    pub fn create_windowagg_path(
        root: *mut PlannerInfo,
        rel: *mut RelOptInfo,
        subpath: *mut Path,
        target: *mut PathTarget,
        windowFuncs: *mut List,
        winclause: *mut WindowClause,
        winpathkeys: *mut List,
    ) -> *mut WindowAggPath;
}
#[pg_guard]
extern "C" {
    pub fn deconstruct_indexquals(path: *mut IndexPath) -> *mut List;
}
#[pg_guard]
extern "C" {
    pub fn do_convert_tuple(tuple: HeapTuple, map: *mut TupleConversionMap) -> HeapTuple;
}
#[pg_guard]
extern "C" {
    pub fn do_pg_start_backup(
        backupidstr: *const ::std::os::raw::c_char,
        fast: bool,
        starttli_p: *mut TimeLineID,
        labelfile: StringInfo,
        tblspcdir: *mut DIR,
        tablespaces: *mut *mut List,
        tblspcmapfile: StringInfo,
        infotbssize: bool,
        needtblspcmapfile: bool,
    ) -> XLogRecPtr;
}
#[pg_guard]
extern "C" {
    pub fn dsm_impl_can_resize() -> bool;
}
#[pg_guard]
extern "C" {
    pub fn dsm_remap(seg: *mut dsm_segment) -> *mut ::std::os::raw::c_void;
}
#[pg_guard]
extern "C" {
    pub fn dsm_resize(seg: *mut dsm_segment, size: Size) -> *mut ::std::os::raw::c_void;
}
#[pg_guard]
extern "C" {
    pub fn estimate_hash_bucketsize(
        root: *mut PlannerInfo,
        hashkey: *mut Node,
        nbuckets: f64,
    ) -> Selectivity;
}
#[pg_guard]
extern "C" {
    pub fn execTuplesHashPrepare(
        numCols: ::std::os::raw::c_int,
        eqOperators: *mut Oid,
        eqFunctions: *mut *mut FmgrInfo,
        hashFunctions: *mut *mut FmgrInfo,
    );
}
#[pg_guard]
extern "C" {
    pub fn execTuplesMatch(
        slot1: *mut TupleTableSlot,
        slot2: *mut TupleTableSlot,
        numCols: ::std::os::raw::c_int,
        matchColIdx: *mut AttrNumber,
        eqfunctions: *mut FmgrInfo,
        evalContext: MemoryContext,
    ) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn execTuplesMatchPrepare(
        numCols: ::std::os::raw::c_int,
        eqOperators: *mut Oid,
    ) -> *mut FmgrInfo;
}
#[pg_guard]
extern "C" {
    pub fn execTuplesUnequal(
        slot1: *mut TupleTableSlot,
        slot2: *mut TupleTableSlot,
        numCols: ::std::os::raw::c_int,
        matchColIdx: *mut AttrNumber,
        eqfunctions: *mut FmgrInfo,
        evalContext: MemoryContext,
    ) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn expand_indexqual_conditions(
        index: *mut IndexOptInfo,
        indexclauses: *mut List,
        indexclausecols: *mut List,
        indexquals_p: *mut *mut List,
        indexqualcols_p: *mut *mut List,
    );
}
#[pg_guard]
extern "C" {
    pub fn expression_returns_set_rows(clause: *mut Node) -> f64;
}
#[pg_guard]
extern "C" {
    pub fn find_childrel_appendrelinfo(
        root: *mut PlannerInfo,
        rel: *mut RelOptInfo,
    ) -> *mut AppendRelInfo;
}
#[pg_guard]
extern "C" {
    pub fn float4_cmp_internal(a: float4, b: float4) -> ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub fn float8_cmp_internal(a: float8, b: float8) -> ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub fn float8in_internal(
        num: *mut ::std::os::raw::c_char,
        endptr_p: *mut *mut ::std::os::raw::c_char,
        type_name: *const ::std::os::raw::c_char,
        orig_string: *const ::std::os::raw::c_char,
    ) -> f64;
}
#[pg_guard]
extern "C" {
    pub fn float8out_internal(num: f64) -> *mut ::std::os::raw::c_char;
}
#[pg_guard]
extern "C" {
    pub fn fmgr(procedureId: Oid, ...) -> *mut ::std::os::raw::c_char;
}
#[pg_guard]
extern "C" {
    pub fn format_type_with_typemod_qualified(
        type_oid: Oid,
        typemod: int32,
    ) -> *mut ::std::os::raw::c_char;
}
#[pg_guard]
extern "C" {
    pub fn generate_gather_paths(root: *mut PlannerInfo, rel: *mut RelOptInfo);
}
#[pg_guard]
extern "C" {
    pub fn genericcostestimate(
        root: *mut PlannerInfo,
        path: *mut IndexPath,
        loop_count: f64,
        qinfos: *mut List,
        costs: *mut GenericCosts,
    );
}
#[pg_guard]
extern "C" {
    pub fn get_attidentity(relid: Oid, attnum: AttrNumber) -> ::std::os::raw::c_char;
}
#[pg_guard]
extern "C" {
    pub fn get_attname(relid: Oid, attnum: AttrNumber) -> *mut ::std::os::raw::c_char;
}
#[pg_guard]
extern "C" {
    pub fn get_atttypmod(relid: Oid, attnum: AttrNumber) -> int32;
}
#[pg_guard]
extern "C" {
    pub fn get_catalog_object_by_oid(catalog: Relation, objectId: Oid) -> HeapTuple;
}
#[pg_guard]
extern "C" {
    pub fn get_float4_infinity() -> f32;
}
#[pg_guard]
extern "C" {
    pub fn get_float4_nan() -> f32;
}
#[pg_guard]
extern "C" {
    pub fn get_float8_infinity() -> f64;
}
#[pg_guard]
extern "C" {
    pub fn get_float8_nan() -> f64;
}
#[pg_guard]
extern "C" {
    pub fn get_func_cost(funcid: Oid) -> float4;
}
#[pg_guard]
extern "C" {
    pub fn get_func_rows(funcid: Oid) -> float4;
}
#[pg_guard]
extern "C" {
    pub fn get_leftop(clause: *const Expr) -> *mut Node;
}
#[pg_guard]
extern "C" {
    pub fn get_notclausearg(notclause: *mut Expr) -> *mut Expr;
}
#[pg_guard]
extern "C" {
    pub fn get_object_aclkind(class_id: Oid) -> AclObjectKind;
}
#[pg_guard]
extern "C" {
    pub fn get_partition_for_tuple(
        pd: *mut PartitionDispatch,
        slot: *mut TupleTableSlot,
        estate: *mut EState,
        failed_at: *mut *mut PartitionDispatchData,
        failed_slot: *mut *mut TupleTableSlot,
    ) -> ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub fn get_partition_parent(relid: Oid) -> Oid;
}
#[pg_guard]
extern "C" {
    pub fn get_partition_qual_relid(relid: Oid) -> *mut Expr;
}
#[pg_guard]
extern "C" {
    pub fn get_partitioned_child_rels(root: *mut PlannerInfo, rti: Index) -> *mut List;
}
#[pg_guard]
extern "C" {
    pub fn get_publication_name(pubid: Oid) -> *mut ::std::os::raw::c_char;
}
#[pg_guard]
extern "C" {
    pub fn get_qual_from_partbound(
        rel: Relation,
        parent: Relation,
        spec: *mut PartitionBoundSpec,
    ) -> *mut List;
}
#[pg_guard]
extern "C" {
    pub fn get_relid_attribute_name(relid: Oid, attnum: AttrNumber) -> *mut ::std::os::raw::c_char;
}
#[pg_guard]
extern "C" {
    pub fn get_rightop(clause: *const Expr) -> *mut Node;
}
#[pg_guard]
extern "C" {
    pub fn get_user_default_acl(objtype: GrantObjectType, ownerId: Oid, nsp_oid: Oid) -> *mut Acl;
}
#[pg_guard]
extern "C" {
    pub fn gist_box_compress(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn gist_box_decompress(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn gist_box_fetch(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn gtsquery_decompress(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn heap_abort_speculative(relation: Relation, tuple: HeapTuple);
}
#[pg_guard]
extern "C" {
    pub fn heap_attisnull(tup: HeapTuple, attnum: ::std::os::raw::c_int) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn heap_beginscan(
        relation: Relation,
        snapshot: Snapshot,
        nkeys: ::std::os::raw::c_int,
        key: ScanKey,
    ) -> HeapScanDesc;
}
#[pg_guard]
extern "C" {
    pub fn heap_beginscan_bm(
        relation: Relation,
        snapshot: Snapshot,
        nkeys: ::std::os::raw::c_int,
        key: ScanKey,
    ) -> HeapScanDesc;
}
#[pg_guard]
extern "C" {
    pub fn heap_beginscan_catalog(
        relation: Relation,
        nkeys: ::std::os::raw::c_int,
        key: ScanKey,
    ) -> HeapScanDesc;
}
#[pg_guard]
extern "C" {
    pub fn heap_beginscan_parallel(arg1: Relation, arg2: ParallelHeapScanDesc) -> HeapScanDesc;
}
#[pg_guard]
extern "C" {
    pub fn heap_beginscan_sampling(
        relation: Relation,
        snapshot: Snapshot,
        nkeys: ::std::os::raw::c_int,
        key: ScanKey,
        allow_strat: bool,
        allow_sync: bool,
        allow_pagemode: bool,
    ) -> HeapScanDesc;
}
#[pg_guard]
extern "C" {
    pub fn heap_beginscan_strat(
        relation: Relation,
        snapshot: Snapshot,
        nkeys: ::std::os::raw::c_int,
        key: ScanKey,
        allow_strat: bool,
        allow_sync: bool,
    ) -> HeapScanDesc;
}
#[pg_guard]
extern "C" {
    pub fn heap_delete(
        relation: Relation,
        tid: ItemPointer,
        cid: CommandId,
        crosscheck: Snapshot,
        wait: bool,
        hufd: *mut HeapUpdateFailureData,
    ) -> HTSU_Result;
}
#[pg_guard]
extern "C" {
    pub fn heap_endscan(scan: HeapScanDesc);
}
#[pg_guard]
extern "C" {
    pub fn heap_fetch(
        relation: Relation,
        snapshot: Snapshot,
        tuple: HeapTuple,
        userbuf: *mut Buffer,
        keep_buf: bool,
        stats_relation: Relation,
    ) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn heap_finish_speculative(relation: Relation, tuple: HeapTuple);
}
#[pg_guard]
extern "C" {
    pub fn heap_freeze_tuple(
        tuple: HeapTupleHeader,
        relfrozenxid: TransactionId,
        relminmxid: TransactionId,
        cutoff_xid: TransactionId,
        cutoff_multi: TransactionId,
    ) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn heap_get_latest_tid(relation: Relation, snapshot: Snapshot, tid: ItemPointer);
}
#[pg_guard]
extern "C" {
    pub fn heap_get_root_tuples(page: Page, root_offsets: *mut OffsetNumber);
}
#[pg_guard]
extern "C" {
    pub fn heap_getnext(scan: HeapScanDesc, direction: ScanDirection) -> HeapTuple;
}
#[pg_guard]
extern "C" {
    pub fn heap_hot_search(
        tid: ItemPointer,
        relation: Relation,
        snapshot: Snapshot,
        all_dead: *mut bool,
    ) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn heap_hot_search_buffer(
        tid: ItemPointer,
        relation: Relation,
        buffer: Buffer,
        snapshot: Snapshot,
        heapTuple: HeapTuple,
        all_dead: *mut bool,
        first_call: bool,
    ) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn heap_inplace_update(relation: Relation, tuple: HeapTuple);
}
#[pg_guard]
extern "C" {
    pub fn heap_insert(
        relation: Relation,
        tup: HeapTuple,
        cid: CommandId,
        options: ::std::os::raw::c_int,
        bistate: BulkInsertState,
    ) -> Oid;
}
#[pg_guard]
extern "C" {
    pub fn heap_lock_tuple(
        relation: Relation,
        tuple: HeapTuple,
        cid: CommandId,
        mode: LockTupleMode,
        wait_policy: LockWaitPolicy,
        follow_update: bool,
        buffer: *mut Buffer,
        hufd: *mut HeapUpdateFailureData,
    ) -> HTSU_Result;
}
#[pg_guard]
extern "C" {
    pub fn heap_multi_insert(
        relation: Relation,
        tuples: *mut HeapTuple,
        ntuples: ::std::os::raw::c_int,
        cid: CommandId,
        options: ::std::os::raw::c_int,
        bistate: BulkInsertState,
    );
}
#[pg_guard]
extern "C" {
    pub fn heap_open(relationId: Oid, lockmode: LOCKMODE) -> Relation;
}
#[pg_guard]
extern "C" {
    pub fn heap_openrv(relation: *const RangeVar, lockmode: LOCKMODE) -> Relation;
}
#[pg_guard]
extern "C" {
    pub fn heap_openrv_extended(
        relation: *const RangeVar,
        lockmode: LOCKMODE,
        missing_ok: bool,
    ) -> Relation;
}
#[pg_guard]
extern "C" {
    pub fn heap_page_prune(
        relation: Relation,
        buffer: Buffer,
        OldestXmin: TransactionId,
        report_stats: bool,
        latestRemovedXid: *mut TransactionId,
    ) -> ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub fn heap_page_prune_execute(
        buffer: Buffer,
        redirected: *mut OffsetNumber,
        nredirected: ::std::os::raw::c_int,
        nowdead: *mut OffsetNumber,
        ndead: ::std::os::raw::c_int,
        nowunused: *mut OffsetNumber,
        nunused: ::std::os::raw::c_int,
    );
}
#[pg_guard]
extern "C" {
    pub fn heap_page_prune_opt(relation: Relation, buffer: Buffer);
}
#[pg_guard]
extern "C" {
    pub fn heap_parallelscan_estimate(snapshot: Snapshot) -> Size;
}
#[pg_guard]
extern "C" {
    pub fn heap_parallelscan_initialize(
        target: ParallelHeapScanDesc,
        relation: Relation,
        snapshot: Snapshot,
    );
}
#[pg_guard]
extern "C" {
    pub fn heap_parallelscan_reinitialize(parallel_scan: ParallelHeapScanDesc);
}
#[pg_guard]
extern "C" {
    pub fn heap_rescan(scan: HeapScanDesc, key: ScanKey);
}
#[pg_guard]
extern "C" {
    pub fn heap_rescan_set_params(
        scan: HeapScanDesc,
        key: ScanKey,
        allow_strat: bool,
        allow_sync: bool,
        allow_pagemode: bool,
    );
}
#[pg_guard]
extern "C" {
    pub fn heap_setscanlimits(scan: HeapScanDesc, startBlk: BlockNumber, endBlk: BlockNumber);
}
#[pg_guard]
extern "C" {
    pub fn heap_sync(relation: Relation);
}
#[pg_guard]
extern "C" {
    pub fn heap_tuple_needs_eventual_freeze(tuple: HeapTupleHeader) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn heap_tuple_needs_freeze(
        tuple: HeapTupleHeader,
        cutoff_xid: TransactionId,
        cutoff_multi: MultiXactId,
        buf: Buffer,
    ) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn heap_update(
        relation: Relation,
        otid: ItemPointer,
        newtup: HeapTuple,
        cid: CommandId,
        crosscheck: Snapshot,
        wait: bool,
        hufd: *mut HeapUpdateFailureData,
        lockmode: *mut LockTupleMode,
    ) -> HTSU_Result;
}
#[pg_guard]
extern "C" {
    pub fn heap_update_snapshot(scan: HeapScanDesc, snapshot: Snapshot);
}
#[pg_guard]
extern "C" {
    pub fn heapgetpage(scan: HeapScanDesc, page: BlockNumber);
}
#[pg_guard]
extern "C" {
    pub fn index_build(
        heapRelation: Relation,
        indexRelation: Relation,
        indexInfo: *mut IndexInfo,
        isprimary: bool,
        isreindex: bool,
    );
}
#[pg_guard]
extern "C" {
    pub fn index_constraint_create(
        heapRelation: Relation,
        indexRelationId: Oid,
        indexInfo: *mut IndexInfo,
        constraintName: *const ::std::os::raw::c_char,
        constraintType: ::std::os::raw::c_char,
        deferrable: bool,
        initdeferred: bool,
        mark_as_primary: bool,
        update_pgindex: bool,
        remove_old_dependencies: bool,
        allow_system_table_mods: bool,
        is_internal: bool,
    ) -> ObjectAddress;
}
#[pg_guard]
extern "C" {
    pub fn index_create(
        heapRelation: Relation,
        indexRelationName: *const ::std::os::raw::c_char,
        indexRelationId: Oid,
        relFileNode: Oid,
        indexInfo: *mut IndexInfo,
        indexColNames: *mut List,
        accessMethodObjectId: Oid,
        tableSpaceId: Oid,
        collationObjectId: *mut Oid,
        classObjectId: *mut Oid,
        coloptions: *mut int16,
        reloptions: Datum,
        isprimary: bool,
        isconstraint: bool,
        deferrable: bool,
        initdeferred: bool,
        allow_system_table_mods: bool,
        skip_build: bool,
        concurrent: bool,
        is_internal: bool,
        if_not_exists: bool,
    ) -> Oid;
}
#[pg_guard]
extern "C" {
    pub fn index_drop(indexId: Oid, concurrent: bool);
}
#[pg_guard]
extern "C" {
    pub fn index_fetch_heap(scan: IndexScanDesc) -> HeapTuple;
}
#[pg_guard]
extern "C" {
    pub fn index_getnext(scan: IndexScanDesc, direction: ScanDirection) -> HeapTuple;
}
#[pg_guard]
extern "C" {
    pub fn inet_gist_decompress(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn initial_cost_hashjoin(
        root: *mut PlannerInfo,
        workspace: *mut JoinCostWorkspace,
        jointype: JoinType,
        hashclauses: *mut List,
        outer_path: *mut Path,
        inner_path: *mut Path,
        extra: *mut JoinPathExtraData,
    );
}
#[pg_guard]
extern "C" {
    pub fn interval_reltime(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn interval_transform(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn intinterval(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn is_dummy_plan(plan: *mut Plan) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn is_infinite(val: f64) -> ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub fn makeInteger(i: ::std::os::raw::c_long) -> *mut Value;
}
#[pg_guard]
extern "C" {
    pub fn make_greater_string(
        str_const: *const Const,
        ltproc: *mut FmgrInfo,
        collation: Oid,
    ) -> *mut Const;
}
#[pg_guard]
extern "C" {
    pub fn map_partition_varattnos(
        expr: *mut List,
        target_varno: ::std::os::raw::c_int,
        partrel: Relation,
        parent: Relation,
        found_whole_row: *mut bool,
    ) -> *mut List;
}
#[pg_guard]
extern "C" {
    pub fn mktinterval(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn namecpy(n1: Name, n2: Name) -> ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub fn not_clause(clause: *mut Node) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn numeric_transform(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn or_clause(clause: *mut Node) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn parse_real(value: *const ::std::os::raw::c_char, result: *mut f64) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn partition_bounds_equal(
        key: PartitionKey,
        p1: PartitionBoundInfo,
        p2: PartitionBoundInfo,
    ) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn pattern_fixed_prefix(
        patt: *mut Const,
        ptype: Pattern_Type,
        collation: Oid,
        prefix: *mut *mut Const,
        rest_selec: *mut Selectivity,
    ) -> Pattern_Prefix_Status;
}
#[pg_guard]
extern "C" {
    pub fn pgstat_init_function_usage(
        fcinfo: *mut FunctionCallInfoData,
        fcu: *mut PgStat_FunctionCallUsage,
    );
}
#[pg_guard]
extern "C" {
    pub fn planner(
        parse: *mut Query,
        cursorOptions: ::std::os::raw::c_int,
        boundParams: ParamListInfo,
    ) -> *mut PlannedStmt;
}
#[pg_guard]
extern "C" {
    pub fn point_dt(pt1: *mut Point, pt2: *mut Point) -> float8;
}
#[pg_guard]
extern "C" {
    pub fn point_sl(pt1: *mut Point, pt2: *mut Point) -> float8;
}
#[pg_guard]
extern "C" {
    pub fn pqStrerror(
        errnum: ::std::os::raw::c_int,
        strerrbuf: *mut ::std::os::raw::c_char,
        buflen: usize,
    ) -> *mut ::std::os::raw::c_char;
}
#[pg_guard]
extern "C" {
    pub fn process_equivalence(
        root: *mut PlannerInfo,
        restrictinfo: *mut RestrictInfo,
        below_outer_join: bool,
    ) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn range_gist_compress(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn range_gist_decompress(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn range_gist_fetch(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn reltime_interval(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn reltimeeq(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn reltimege(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn reltimegt(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn reltimein(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn reltimele(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn reltimelt(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn reltimene(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn reltimeout(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn reltimerecv(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn reltimesend(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn setLastTid(tid: ItemPointer);
}
#[pg_guard]
extern "C" {
    pub fn set_dummy_rel_pathlist(rel: *mut RelOptInfo);
}
#[pg_guard]
extern "C" {
    pub fn simple_heap_delete(relation: Relation, tid: ItemPointer);
}
#[pg_guard]
extern "C" {
    pub fn simple_heap_insert(relation: Relation, tup: HeapTuple) -> Oid;
}
#[pg_guard]
extern "C" {
    pub fn simple_heap_update(relation: Relation, otid: ItemPointer, tup: HeapTuple);
}
#[pg_guard]
extern "C" {
    pub fn slot_attisnull(slot: *mut TupleTableSlot, attnum: ::std::os::raw::c_int) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn slot_getallattrs(slot: *mut TupleTableSlot);
}
#[pg_guard]
extern "C" {
    pub fn slot_getattr(
        slot: *mut TupleTableSlot,
        attnum: ::std::os::raw::c_int,
        isnull: *mut bool,
    ) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn slot_getsomeattrs(slot: *mut TupleTableSlot, attnum: ::std::os::raw::c_int);
}
#[pg_guard]
extern "C" {
    pub fn slot_getsysattr(
        slot: *mut TupleTableSlot,
        attnum: ::std::os::raw::c_int,
        value: *mut Datum,
        isnull: *mut bool,
    ) -> bool;
}
#[pg_guard]
extern "C" {
    pub fn smgreq(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn smgrin(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn smgrne(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn smgrout(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn ss_get_location(rel: Relation, relnblocks: BlockNumber) -> BlockNumber;
}
#[pg_guard]
extern "C" {
    pub fn ss_report_location(rel: Relation, location: BlockNumber);
}
#[pg_guard]
extern "C" {
    pub fn stringToNode(str_: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_void;
}
#[pg_guard]
extern "C" {
    pub fn time_transform(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn timemi(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn timenow(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn timepl(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn timestamp_abstime(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn timestamp_izone_transform(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn timestamp_transform(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn timestamp_zone_transform(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn timestamptz_abstime(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn tintervalct(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn tintervalend(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn tintervaleq(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn tintervalge(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn tintervalgt(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn tintervalin(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn tintervalle(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn tintervalleneq(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn tintervallenge(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn tintervallengt(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn tintervallenle(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn tintervallenlt(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn tintervallenne(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn tintervallt(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn tintervalne(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn tintervalout(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn tintervalov(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn tintervalrecv(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn tintervalrel(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn tintervalsame(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn tintervalsend(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn tintervalstart(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn transformArraySubscripts(
        pstate: *mut ParseState,
        arrayBase: *mut Node,
        arrayType: Oid,
        elementType: Oid,
        arrayTypMod: int32,
        indirection: *mut List,
        assignFrom: *mut Node,
    ) -> *mut ArrayRef;
}
#[pg_guard]
extern "C" {
    pub fn transformArrayType(arrayType: *mut Oid, arrayTypmod: *mut int32) -> Oid;
}
#[pg_guard]
extern "C" {
    pub fn transformRelOptions(
        oldOptions: Datum,
        defList: *mut List,
        namspace: *mut ::std::os::raw::c_char,
        validnsps: *mut *mut ::std::os::raw::c_char,
        ignoreOids: bool,
        isReset: bool,
    ) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn tuplesort_begin_cluster(
        tupDesc: TupleDesc,
        indexRel: Relation,
        workMem: ::std::os::raw::c_int,
        randomAccess: bool,
    ) -> *mut Tuplesortstate;
}
#[pg_guard]
extern "C" {
    pub fn tuplesort_begin_datum(
        datumType: Oid,
        sortOperator: Oid,
        sortCollation: Oid,
        nullsFirstFlag: bool,
        workMem: ::std::os::raw::c_int,
        randomAccess: bool,
    ) -> *mut Tuplesortstate;
}
#[pg_guard]
extern "C" {
    pub fn tuplesort_begin_heap(
        tupDesc: TupleDesc,
        nkeys: ::std::os::raw::c_int,
        attNums: *mut AttrNumber,
        sortOperators: *mut Oid,
        sortCollations: *mut Oid,
        nullsFirstFlags: *mut bool,
        workMem: ::std::os::raw::c_int,
        randomAccess: bool,
    ) -> *mut Tuplesortstate;
}
#[pg_guard]
extern "C" {
    pub fn tuplesort_begin_index_btree(
        heapRel: Relation,
        indexRel: Relation,
        enforceUnique: bool,
        workMem: ::std::os::raw::c_int,
        randomAccess: bool,
    ) -> *mut Tuplesortstate;
}
#[pg_guard]
extern "C" {
    pub fn tuplesort_begin_index_hash(
        heapRel: Relation,
        indexRel: Relation,
        high_mask: uint32,
        low_mask: uint32,
        max_buckets: uint32,
        workMem: ::std::os::raw::c_int,
        randomAccess: bool,
    ) -> *mut Tuplesortstate;
}
#[pg_guard]
extern "C" {
    pub fn tuplesort_get_stats(
        state: *mut Tuplesortstate,
        sortMethod: *mut *const ::std::os::raw::c_char,
        spaceType: *mut *const ::std::os::raw::c_char,
        spaceUsed: *mut ::std::os::raw::c_long,
    );
}
#[pg_guard]
extern "C" {
    pub fn varbit_transform(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub fn varchar_transform(fcinfo: FunctionCallInfo) -> Datum;
}
#[pg_guard]
extern "C" {
    pub static mut ClientConnectionLost: bool;
}
#[pg_guard]
extern "C" {
    pub static mut IdleInTransactionSessionTimeoutPending: bool;
}
#[pg_guard]
extern "C" {
    pub static mut InterruptPending: bool;
}
#[pg_guard]
extern "C" {
    pub static mut MainLWLockNames: [*mut ::std::os::raw::c_char; 0usize];
}
#[pg_guard]
extern "C" {
    pub static mut ProcDiePending: bool;
}
#[pg_guard]
extern "C" {
    pub static mut QueryCancelPending: bool;
}
#[pg_guard]
extern "C" {
    pub static mut SPI_lastoid: Oid;
}
#[pg_guard]
extern "C" {
    pub static mut VacuumCostDelay: ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub static mut default_with_oids: bool;
}
#[pg_guard]
extern "C" {
    pub static mut extra_float_digits: ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub static mut no_such_variable: ::std::os::raw::c_int;
}
#[pg_guard]
extern "C" {
    pub static mut replacement_sort_tuples: ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct BackgroundWorker {
    pub bgw_name: [::std::os::raw::c_char; 64usize],
    pub bgw_flags: ::std::os::raw::c_int,
    pub bgw_start_time: BgWorkerStartTime,
    pub bgw_restart_time: ::std::os::raw::c_int,
    pub bgw_library_name: [::std::os::raw::c_char; 64usize],
    pub bgw_function_name: [::std::os::raw::c_char; 64usize],
    pub bgw_main_arg: Datum,
    pub bgw_extra: [::std::os::raw::c_char; 128usize],
    pub bgw_notify_pid: pid_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FormData_pg_attribute {
    pub attrelid: Oid,
    pub attname: NameData,
    pub atttypid: Oid,
    pub attstattarget: int32,
    pub attlen: int16,
    pub attnum: int16,
    pub attndims: int32,
    pub attcacheoff: int32,
    pub atttypmod: int32,
    pub attbyval: bool,
    pub attstorage: ::std::os::raw::c_char,
    pub attalign: ::std::os::raw::c_char,
    pub attnotnull: bool,
    pub atthasdef: bool,
    pub attidentity: ::std::os::raw::c_char,
    pub attisdropped: bool,
    pub attislocal: bool,
    pub attinhcount: int32,
    pub attcollation: Oid,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FormData_pg_class {
    pub relname: NameData,
    pub relnamespace: Oid,
    pub reltype: Oid,
    pub reloftype: Oid,
    pub relowner: Oid,
    pub relam: Oid,
    pub relfilenode: Oid,
    pub reltablespace: Oid,
    pub relpages: int32,
    pub reltuples: float4,
    pub relallvisible: int32,
    pub reltoastrelid: Oid,
    pub relhasindex: bool,
    pub relisshared: bool,
    pub relpersistence: ::std::os::raw::c_char,
    pub relkind: ::std::os::raw::c_char,
    pub relnatts: int16,
    pub relchecks: int16,
    pub relhasoids: bool,
    pub relhaspkey: bool,
    pub relhasrules: bool,
    pub relhastriggers: bool,
    pub relhassubclass: bool,
    pub relrowsecurity: bool,
    pub relforcerowsecurity: bool,
    pub relispopulated: bool,
    pub relreplident: ::std::os::raw::c_char,
    pub relispartition: bool,
    pub relfrozenxid: TransactionId,
    pub relminmxid: TransactionId,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FormData_pg_enum {
    pub enumtypid: Oid,
    pub enumsortorder: float4,
    pub enumlabel: NameData,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FormData_pg_event_trigger {
    pub evtname: NameData,
    pub evtevent: NameData,
    pub evtowner: Oid,
    pub evtfoid: Oid,
    pub evtenabled: ::std::os::raw::c_char,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FormData_pg_publication {
    pub pubname: NameData,
    pub pubowner: Oid,
    pub puballtables: bool,
    pub pubinsert: bool,
    pub pubupdate: bool,
    pub pubdelete: bool,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FormData_pg_type {
    pub typname: NameData,
    pub typnamespace: Oid,
    pub typowner: Oid,
    pub typlen: int16,
    pub typbyval: bool,
    pub typtype: ::std::os::raw::c_char,
    pub typcategory: ::std::os::raw::c_char,
    pub typispreferred: bool,
    pub typisdefined: bool,
    pub typdelim: ::std::os::raw::c_char,
    pub typrelid: Oid,
    pub typelem: Oid,
    pub typarray: Oid,
    pub typinput: regproc,
    pub typoutput: regproc,
    pub typreceive: regproc,
    pub typsend: regproc,
    pub typmodin: regproc,
    pub typmodout: regproc,
    pub typanalyze: regproc,
    pub typalign: ::std::os::raw::c_char,
    pub typstorage: ::std::os::raw::c_char,
    pub typnotnull: bool,
    pub typbasetype: Oid,
    pub typtypmod: int32,
    pub typndims: int32,
    pub typcollation: Oid,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FunctionCallInfoData {
    pub flinfo: *mut FmgrInfo,
    pub context: fmNodePtr,
    pub resultinfo: fmNodePtr,
    pub fncollation: Oid,
    pub isnull: bool,
    pub nargs: ::std::os::raw::c_short,
    pub arg: [Datum; 100usize],
    pub argnull: [bool; 100usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct HbaLine {
    pub linenumber: ::std::os::raw::c_int,
    pub rawline: *mut ::std::os::raw::c_char,
    pub conntype: ConnType,
    pub databases: *mut List,
    pub roles: *mut List,
    pub addr: sockaddr_storage,
    pub mask: sockaddr_storage,
    pub ip_cmp_method: IPCompareMethod,
    pub hostname: *mut ::std::os::raw::c_char,
    pub auth_method: UserAuth,
    pub usermap: *mut ::std::os::raw::c_char,
    pub pamservice: *mut ::std::os::raw::c_char,
    pub pam_use_hostname: bool,
    pub ldaptls: bool,
    pub ldapserver: *mut ::std::os::raw::c_char,
    pub ldapport: ::std::os::raw::c_int,
    pub ldapbinddn: *mut ::std::os::raw::c_char,
    pub ldapbindpasswd: *mut ::std::os::raw::c_char,
    pub ldapsearchattribute: *mut ::std::os::raw::c_char,
    pub ldapbasedn: *mut ::std::os::raw::c_char,
    pub ldapscope: ::std::os::raw::c_int,
    pub ldapprefix: *mut ::std::os::raw::c_char,
    pub ldapsuffix: *mut ::std::os::raw::c_char,
    pub clientcert: bool,
    pub krb_realm: *mut ::std::os::raw::c_char,
    pub include_realm: bool,
    pub compat_realm: bool,
    pub upn_username: bool,
    pub radiusservers: *mut List,
    pub radiusservers_s: *mut ::std::os::raw::c_char,
    pub radiussecrets: *mut List,
    pub radiussecrets_s: *mut ::std::os::raw::c_char,
    pub radiusidentifiers: *mut List,
    pub radiusidentifiers_s: *mut ::std::os::raw::c_char,
    pub radiusports: *mut List,
    pub radiusports_s: *mut ::std::os::raw::c_char,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct HeapScanDescData {
    pub rs_rd: Relation,
    pub rs_snapshot: Snapshot,
    pub rs_nkeys: ::std::os::raw::c_int,
    pub rs_key: ScanKey,
    pub rs_bitmapscan: bool,
    pub rs_samplescan: bool,
    pub rs_pageatatime: bool,
    pub rs_allow_strat: bool,
    pub rs_allow_sync: bool,
    pub rs_temp_snap: bool,
    pub rs_nblocks: BlockNumber,
    pub rs_startblock: BlockNumber,
    pub rs_numblocks: BlockNumber,
    pub rs_strategy: BufferAccessStrategy,
    pub rs_syncscan: bool,
    pub rs_inited: bool,
    pub rs_ctup: HeapTupleData,
    pub rs_cblock: BlockNumber,
    pub rs_cbuf: Buffer,
    pub rs_parallel: ParallelHeapScanDesc,
    pub rs_cindex: ::std::os::raw::c_int,
    pub rs_ntuples: ::std::os::raw::c_int,
    pub rs_vistuples: [OffsetNumber; 291usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PGPROC {
    pub links: SHM_QUEUE,
    pub procgloballist: *mut *mut PGPROC,
    pub sem: PGSemaphore,
    pub waitStatus: ::std::os::raw::c_int,
    pub procLatch: Latch,
    pub lxid: LocalTransactionId,
    pub pid: ::std::os::raw::c_int,
    pub pgprocno: ::std::os::raw::c_int,
    pub backendId: BackendId,
    pub databaseId: Oid,
    pub roleId: Oid,
    pub isBackgroundWorker: bool,
    pub recoveryConflictPending: bool,
    pub lwWaiting: bool,
    pub lwWaitMode: uint8,
    pub lwWaitLink: proclist_node,
    pub cvWaitLink: proclist_node,
    pub waitLock: *mut LOCK,
    pub waitProcLock: *mut PROCLOCK,
    pub waitLockMode: LOCKMODE,
    pub heldLocks: LOCKMASK,
    pub waitLSN: XLogRecPtr,
    pub syncRepState: ::std::os::raw::c_int,
    pub syncRepLinks: SHM_QUEUE,
    pub myProcLocks: [SHM_QUEUE; 16usize],
    pub subxids: XidCache,
    pub procArrayGroupMember: bool,
    pub procArrayGroupNext: pg_atomic_uint32,
    pub procArrayGroupMemberXid: TransactionId,
    pub wait_event_info: uint32,
    pub backendLock: LWLock,
    pub fpLockBits: uint64,
    pub fpRelId: [Oid; 16usize],
    pub fpVXIDLock: bool,
    pub fpLocalTransactionId: LocalTransactionId,
    pub lockGroupLeader: *mut PGPROC,
    pub lockGroupMembers: dlist_head,
    pub lockGroupLink: dlist_node,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PgBackendSSLStatus {
    pub ssl_bits: ::std::os::raw::c_int,
    pub ssl_compression: bool,
    pub ssl_version: [::std::os::raw::c_char; 64usize],
    pub ssl_cipher: [::std::os::raw::c_char; 64usize],
    pub ssl_clientdn: [::std::os::raw::c_char; 64usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PgBackendStatus {
    pub st_changecount: ::std::os::raw::c_int,
    pub st_procpid: ::std::os::raw::c_int,
    pub st_backendType: BackendType,
    pub st_proc_start_timestamp: TimestampTz,
    pub st_xact_start_timestamp: TimestampTz,
    pub st_activity_start_timestamp: TimestampTz,
    pub st_state_start_timestamp: TimestampTz,
    pub st_databaseid: Oid,
    pub st_userid: Oid,
    pub st_clientaddr: SockAddr,
    pub st_clienthostname: *mut ::std::os::raw::c_char,
    pub st_ssl: bool,
    pub st_sslstatus: *mut PgBackendSSLStatus,
    pub st_state: BackendState,
    pub st_appname: *mut ::std::os::raw::c_char,
    pub st_activity: *mut ::std::os::raw::c_char,
    pub st_progress_command: ProgressCommandType,
    pub st_progress_command_target: Oid,
    pub st_progress_param: [int64; 10usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Port {
    pub sock: pgsocket,
    pub noblock: bool,
    pub proto: ProtocolVersion,
    pub laddr: SockAddr,
    pub raddr: SockAddr,
    pub remote_host: *mut ::std::os::raw::c_char,
    pub remote_hostname: *mut ::std::os::raw::c_char,
    pub remote_hostname_resolv: ::std::os::raw::c_int,
    pub remote_hostname_errcode: ::std::os::raw::c_int,
    pub remote_port: *mut ::std::os::raw::c_char,
    pub canAcceptConnections: CAC_state,
    pub database_name: *mut ::std::os::raw::c_char,
    pub user_name: *mut ::std::os::raw::c_char,
    pub cmdline_options: *mut ::std::os::raw::c_char,
    pub guc_options: *mut List,
    pub hba: *mut HbaLine,
    pub SessionStartTime: TimestampTz,
    pub default_keepalives_idle: ::std::os::raw::c_int,
    pub default_keepalives_interval: ::std::os::raw::c_int,
    pub default_keepalives_count: ::std::os::raw::c_int,
    pub keepalives_idle: ::std::os::raw::c_int,
    pub keepalives_interval: ::std::os::raw::c_int,
    pub keepalives_count: ::std::os::raw::c_int,
    pub gss: *mut ::std::os::raw::c_void,
    pub ssl_in_use: bool,
    pub peer_cn: *mut ::std::os::raw::c_char,
    pub peer_cert_valid: bool,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SetExprState {
    pub type_: NodeTag,
    pub expr: *mut Expr,
    pub args: *mut List,
    pub elidedFuncState: *mut ExprState,
    pub func: FmgrInfo,
    pub funcResultStore: *mut Tuplestorestate,
    pub funcResultSlot: *mut TupleTableSlot,
    pub funcResultDesc: TupleDesc,
    pub funcReturnsTuple: bool,
    pub funcReturnsSet: bool,
    pub setArgsValid: bool,
    pub shutdown_reg: bool,
    pub fcinfo_data: FunctionCallInfoData,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct XLogReaderState {
    pub read_page: XLogPageReadCB,
    pub system_identifier: uint64,
    pub private_data: *mut ::std::os::raw::c_void,
    pub ReadRecPtr: XLogRecPtr,
    pub EndRecPtr: XLogRecPtr,
    pub decoded_record: *mut XLogRecord,
    pub main_data: *mut ::std::os::raw::c_char,
    pub main_data_len: uint32,
    pub main_data_bufsz: uint32,
    pub record_origin: RepOriginId,
    pub blocks: [DecodedBkpBlock; 33usize],
    pub max_block_id: ::std::os::raw::c_int,
    pub readBuf: *mut ::std::os::raw::c_char,
    pub readLen: uint32,
    pub readSegNo: XLogSegNo,
    pub readOff: uint32,
    pub readPageTLI: TimeLineID,
    pub latestPagePtr: XLogRecPtr,
    pub latestPageTLI: TimeLineID,
    pub currRecPtr: XLogRecPtr,
    pub currTLI: TimeLineID,
    pub currTLIValidUntil: XLogRecPtr,
    pub nextTLI: TimeLineID,
    pub readRecordBuf: *mut ::std::os::raw::c_char,
    pub readRecordBufSize: uint32,
    pub errormsg_buf: *mut ::std::os::raw::c_char,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union PgStat_Msg {
    pub msg_hdr: PgStat_MsgHdr,
    pub msg_dummy: PgStat_MsgDummy,
    pub msg_inquiry: PgStat_MsgInquiry,
    pub msg_tabstat: PgStat_MsgTabstat,
    pub msg_tabpurge: PgStat_MsgTabpurge,
    pub msg_dropdb: PgStat_MsgDropdb,
    pub msg_resetcounter: PgStat_MsgResetcounter,
    pub msg_resetsharedcounter: PgStat_MsgResetsharedcounter,
    pub msg_resetsinglecounter: PgStat_MsgResetsinglecounter,
    pub msg_autovacuum: PgStat_MsgAutovacStart,
    pub msg_vacuum: PgStat_MsgVacuum,
    pub msg_analyze: PgStat_MsgAnalyze,
    pub msg_archiver: PgStat_MsgArchiver,
    pub msg_bgwriter: PgStat_MsgBgWriter,
    pub msg_funcstat: PgStat_MsgFuncstat,
    pub msg_funcpurge: PgStat_MsgFuncpurge,
    pub msg_recoveryconflict: PgStat_MsgRecoveryConflict,
    pub msg_deadlock: PgStat_MsgDeadlock,
    _bindgen_union_align: [u64; 125usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union Value_ValUnion {
    pub ival: ::std::os::raw::c_long,
    pub str_: *mut ::std::os::raw::c_char,
    _bindgen_union_align: u64,
}
#[repr(C)]
#[derive(Debug)]
pub struct ParamListInfoData {
    pub paramFetch: ParamFetchHook,
    pub paramFetchArg: *mut ::std::os::raw::c_void,
    pub parserSetup: ParserSetupHook,
    pub parserSetupArg: *mut ::std::os::raw::c_void,
    pub numParams: ::std::os::raw::c_int,
    pub paramMask: *mut Bitmapset,
    pub params: __IncompleteArrayField<ParamExternData>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Agg {
    pub plan: Plan,
    pub aggstrategy: AggStrategy,
    pub aggsplit: AggSplit,
    pub numCols: ::std::os::raw::c_int,
    pub grpColIdx: *mut AttrNumber,
    pub grpOperators: *mut Oid,
    pub numGroups: ::std::os::raw::c_long,
    pub aggParams: *mut Bitmapset,
    pub groupingSets: *mut List,
    pub chain: *mut List,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AggState {
    pub ss: ScanState,
    pub aggs: *mut List,
    pub numaggs: ::std::os::raw::c_int,
    pub numtrans: ::std::os::raw::c_int,
    pub aggstrategy: AggStrategy,
    pub aggsplit: AggSplit,
    pub phase: AggStatePerPhase,
    pub numphases: ::std::os::raw::c_int,
    pub current_phase: ::std::os::raw::c_int,
    pub peragg: AggStatePerAgg,
    pub pertrans: AggStatePerTrans,
    pub hashcontext: *mut ExprContext,
    pub aggcontexts: *mut *mut ExprContext,
    pub tmpcontext: *mut ExprContext,
    pub curaggcontext: *mut ExprContext,
    pub curpertrans: AggStatePerTrans,
    pub input_done: bool,
    pub agg_done: bool,
    pub projected_set: ::std::os::raw::c_int,
    pub current_set: ::std::os::raw::c_int,
    pub grouped_cols: *mut Bitmapset,
    pub all_grouped_cols: *mut List,
    pub maxsets: ::std::os::raw::c_int,
    pub phases: AggStatePerPhase,
    pub sort_in: *mut Tuplesortstate,
    pub sort_out: *mut Tuplesortstate,
    pub sort_slot: *mut TupleTableSlot,
    pub pergroup: AggStatePerGroup,
    pub grp_firstTuple: HeapTuple,
    pub table_filled: bool,
    pub num_hashes: ::std::os::raw::c_int,
    pub perhash: AggStatePerHash,
    pub hash_pergroup: *mut AggStatePerGroup,
    pub combinedproj: *mut ProjectionInfo,
    pub curperagg: AggStatePerAgg,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AlterFunctionStmt {
    pub type_: NodeTag,
    pub func: *mut ObjectWithArgs,
    pub actions: *mut List,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AlterTableCmd {
    pub type_: NodeTag,
    pub subtype: AlterTableType,
    pub name: *mut ::std::os::raw::c_char,
    pub newowner: *mut RoleSpec,
    pub def: *mut Node,
    pub behavior: DropBehavior,
    pub missing_ok: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Append {
    pub plan: Plan,
    pub partitioned_rels: *mut List,
    pub appendplans: *mut List,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AppendPath {
    pub path: Path,
    pub partitioned_rels: *mut List,
    pub subpaths: *mut List,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AppendState {
    pub ps: PlanState,
    pub appendplans: *mut *mut PlanState,
    pub as_nplans: ::std::os::raw::c_int,
    pub as_whichplan: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ArrayCoerceExpr {
    pub xpr: Expr,
    pub arg: *mut Expr,
    pub elemfuncid: Oid,
    pub resulttype: Oid,
    pub resulttypmod: int32,
    pub resultcollid: Oid,
    pub isExplicit: bool,
    pub coerceformat: CoercionForm,
    pub location: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ArrayRef {
    pub xpr: Expr,
    pub refarraytype: Oid,
    pub refelemtype: Oid,
    pub reftypmod: int32,
    pub refcollid: Oid,
    pub refupperindexpr: *mut List,
    pub reflowerindexpr: *mut List,
    pub refexpr: *mut Expr,
    pub refassgnexpr: *mut Expr,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AttStatsSlot {
    pub staop: Oid,
    pub valuetype: Oid,
    pub values: *mut Datum,
    pub nvalues: ::std::os::raw::c_int,
    pub numbers: *mut float4,
    pub nnumbers: ::std::os::raw::c_int,
    pub values_arr: *mut ::std::os::raw::c_void,
    pub numbers_arr: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BitmapHeapScanState {
    pub ss: ScanState,
    pub bitmapqualorig: *mut ExprState,
    pub tbm: *mut TIDBitmap,
    pub tbmiterator: *mut TBMIterator,
    pub tbmres: *mut TBMIterateResult,
    pub exact_pages: ::std::os::raw::c_long,
    pub lossy_pages: ::std::os::raw::c_long,
    pub prefetch_iterator: *mut TBMIterator,
    pub prefetch_pages: ::std::os::raw::c_int,
    pub prefetch_target: ::std::os::raw::c_int,
    pub prefetch_maximum: ::std::os::raw::c_int,
    pub pscan_len: Size,
    pub initialized: bool,
    pub shared_tbmiterator: *mut TBMSharedIterator,
    pub shared_prefetch_iterator: *mut TBMSharedIterator,
    pub pstate: *mut ParallelBitmapHeapState,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BitmapIndexScanState {
    pub ss: ScanState,
    pub biss_result: *mut TIDBitmap,
    pub biss_ScanKeys: ScanKey,
    pub biss_NumScanKeys: ::std::os::raw::c_int,
    pub biss_RuntimeKeys: *mut IndexRuntimeKeyInfo,
    pub biss_NumRuntimeKeys: ::std::os::raw::c_int,
    pub biss_ArrayKeys: *mut IndexArrayKeyInfo,
    pub biss_NumArrayKeys: ::std::os::raw::c_int,
    pub biss_RuntimeKeysReady: bool,
    pub biss_RuntimeContext: *mut ExprContext,
    pub biss_RelationDesc: Relation,
    pub biss_ScanDesc: IndexScanDesc,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CachedPlanSource {
    pub magic: ::std::os::raw::c_int,
    pub raw_parse_tree: *mut RawStmt,
    pub query_string: *const ::std::os::raw::c_char,
    pub commandTag: *const ::std::os::raw::c_char,
    pub param_types: *mut Oid,
    pub num_params: ::std::os::raw::c_int,
    pub parserSetup: ParserSetupHook,
    pub parserSetupArg: *mut ::std::os::raw::c_void,
    pub cursor_options: ::std::os::raw::c_int,
    pub fixed_result: bool,
    pub resultDesc: TupleDesc,
    pub context: MemoryContext,
    pub query_list: *mut List,
    pub relationOids: *mut List,
    pub invalItems: *mut List,
    pub search_path: *mut OverrideSearchPath,
    pub query_context: MemoryContext,
    pub rewriteRoleId: Oid,
    pub rewriteRowSecurity: bool,
    pub dependsOnRLS: bool,
    pub gplan: *mut CachedPlan,
    pub is_oneshot: bool,
    pub is_complete: bool,
    pub is_saved: bool,
    pub is_valid: bool,
    pub generation: ::std::os::raw::c_int,
    pub next_saved: *mut CachedPlanSource,
    pub generic_cost: f64,
    pub total_custom_cost: f64,
    pub num_custom_plans: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ClusterStmt {
    pub type_: NodeTag,
    pub relation: *mut RangeVar,
    pub indexname: *mut ::std::os::raw::c_char,
    pub verbose: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CollectedCommand__bindgen_ty_1__bindgen_ty_7 {
    pub objtype: GrantObjectType,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ColumnDef {
    pub type_: NodeTag,
    pub colname: *mut ::std::os::raw::c_char,
    pub typeName: *mut TypeName,
    pub inhcount: ::std::os::raw::c_int,
    pub is_local: bool,
    pub is_not_null: bool,
    pub is_from_type: bool,
    pub is_from_parent: bool,
    pub storage: ::std::os::raw::c_char,
    pub raw_default: *mut Node,
    pub cooked_default: *mut Node,
    pub identity: ::std::os::raw::c_char,
    pub identitySequence: *mut RangeVar,
    pub collClause: *mut CollateClause,
    pub collOid: Oid,
    pub constraints: *mut List,
    pub fdwoptions: *mut List,
    pub location: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CommonTableExpr {
    pub type_: NodeTag,
    pub ctename: *mut ::std::os::raw::c_char,
    pub aliascolnames: *mut List,
    pub ctequery: *mut Node,
    pub location: ::std::os::raw::c_int,
    pub cterecursive: bool,
    pub cterefcount: ::std::os::raw::c_int,
    pub ctecolnames: *mut List,
    pub ctecoltypes: *mut List,
    pub ctecoltypmods: *mut List,
    pub ctecolcollations: *mut List,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Constraint {
    pub type_: NodeTag,
    pub contype: ConstrType,
    pub conname: *mut ::std::os::raw::c_char,
    pub deferrable: bool,
    pub initdeferred: bool,
    pub location: ::std::os::raw::c_int,
    pub is_no_inherit: bool,
    pub raw_expr: *mut Node,
    pub cooked_expr: *mut ::std::os::raw::c_char,
    pub generated_when: ::std::os::raw::c_char,
    pub keys: *mut List,
    pub exclusions: *mut List,
    pub options: *mut List,
    pub indexname: *mut ::std::os::raw::c_char,
    pub indexspace: *mut ::std::os::raw::c_char,
    pub access_method: *mut ::std::os::raw::c_char,
    pub where_clause: *mut Node,
    pub pktable: *mut RangeVar,
    pub fk_attrs: *mut List,
    pub pk_attrs: *mut List,
    pub fk_matchtype: ::std::os::raw::c_char,
    pub fk_upd_action: ::std::os::raw::c_char,
    pub fk_del_action: ::std::os::raw::c_char,
    pub old_conpfeqop: *mut List,
    pub old_pktable_oid: Oid,
    pub skip_validation: bool,
    pub initially_valid: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CopyStmt {
    pub type_: NodeTag,
    pub relation: *mut RangeVar,
    pub query: *mut Node,
    pub attlist: *mut List,
    pub is_from: bool,
    pub is_program: bool,
    pub filename: *mut ::std::os::raw::c_char,
    pub options: *mut List,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CreateFunctionStmt {
    pub type_: NodeTag,
    pub replace: bool,
    pub funcname: *mut List,
    pub parameters: *mut List,
    pub returnType: *mut TypeName,
    pub options: *mut List,
    pub withClause: *mut List,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CreateStatsStmt {
    pub type_: NodeTag,
    pub defnames: *mut List,
    pub stat_types: *mut List,
    pub exprs: *mut List,
    pub relations: *mut List,
    pub if_not_exists: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CreateStmt {
    pub type_: NodeTag,
    pub relation: *mut RangeVar,
    pub tableElts: *mut List,
    pub inhRelations: *mut List,
    pub partbound: *mut PartitionBoundSpec,
    pub partspec: *mut PartitionSpec,
    pub ofTypename: *mut TypeName,
    pub constraints: *mut List,
    pub options: *mut List,
    pub oncommit: OnCommitAction,
    pub tablespacename: *mut ::std::os::raw::c_char,
    pub if_not_exists: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CustomPathMethods {
    pub CustomName: *const ::std::os::raw::c_char,
    pub PlanCustomPath: ::std::option::Option<
        unsafe extern "C" fn(
            root: *mut PlannerInfo,
            rel: *mut RelOptInfo,
            best_path: *mut CustomPath,
            tlist: *mut List,
            clauses: *mut List,
            custom_plans: *mut List,
        ) -> *mut Plan,
    >,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DefineStmt {
    pub type_: NodeTag,
    pub kind: ObjectType,
    pub oldstyle: bool,
    pub defnames: *mut List,
    pub args: *mut List,
    pub definition: *mut List,
    pub if_not_exists: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EPQState {
    pub estate: *mut EState,
    pub planstate: *mut PlanState,
    pub origslot: *mut TupleTableSlot,
    pub plan: *mut Plan,
    pub arowMarks: *mut List,
    pub epqParam: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EState {
    pub type_: NodeTag,
    pub es_direction: ScanDirection,
    pub es_snapshot: Snapshot,
    pub es_crosscheck_snapshot: Snapshot,
    pub es_range_table: *mut List,
    pub es_plannedstmt: *mut PlannedStmt,
    pub es_sourceText: *const ::std::os::raw::c_char,
    pub es_junkFilter: *mut JunkFilter,
    pub es_output_cid: CommandId,
    pub es_result_relations: *mut ResultRelInfo,
    pub es_num_result_relations: ::std::os::raw::c_int,
    pub es_result_relation_info: *mut ResultRelInfo,
    pub es_root_result_relations: *mut ResultRelInfo,
    pub es_num_root_result_relations: ::std::os::raw::c_int,
    pub es_leaf_result_relations: *mut List,
    pub es_trig_target_relations: *mut List,
    pub es_trig_tuple_slot: *mut TupleTableSlot,
    pub es_trig_oldtup_slot: *mut TupleTableSlot,
    pub es_trig_newtup_slot: *mut TupleTableSlot,
    pub es_param_list_info: ParamListInfo,
    pub es_param_exec_vals: *mut ParamExecData,
    pub es_queryEnv: *mut QueryEnvironment,
    pub es_query_cxt: MemoryContext,
    pub es_tupleTable: *mut List,
    pub es_rowMarks: *mut List,
    pub es_processed: uint64,
    pub es_lastoid: Oid,
    pub es_top_eflags: ::std::os::raw::c_int,
    pub es_instrument: ::std::os::raw::c_int,
    pub es_finished: bool,
    pub es_exprcontexts: *mut List,
    pub es_subplanstates: *mut List,
    pub es_auxmodifytables: *mut List,
    pub es_per_tuple_exprcontext: *mut ExprContext,
    pub es_epqTuple: *mut HeapTuple,
    pub es_epqTupleSet: *mut bool,
    pub es_epqScanDone: *mut bool,
    pub es_query_dsa: *mut dsa_area,
    pub es_use_parallel_mode: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ExplainState {
    pub str_: StringInfo,
    pub verbose: bool,
    pub analyze: bool,
    pub costs: bool,
    pub buffers: bool,
    pub timing: bool,
    pub summary: bool,
    pub format: ExplainFormat,
    pub indent: ::std::os::raw::c_int,
    pub grouping_stack: *mut List,
    pub pstmt: *mut PlannedStmt,
    pub rtable: *mut List,
    pub rtable_names: *mut List,
    pub deparse_cxt: *mut List,
    pub printed_subplans: *mut Bitmapset,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ExprState {
    pub tag: Node,
    pub flags: uint8,
    pub resnull: bool,
    pub resvalue: Datum,
    pub resultslot: *mut TupleTableSlot,
    pub steps: *mut ExprEvalStep,
    pub evalfunc: ExprStateEvalFunc,
    pub expr: *mut Expr,
    pub steps_len: ::std::os::raw::c_int,
    pub steps_alloc: ::std::os::raw::c_int,
    pub innermost_caseval: *mut Datum,
    pub innermost_casenull: *mut bool,
    pub innermost_domainval: *mut Datum,
    pub innermost_domainnull: *mut bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FdwRoutine {
    pub type_: NodeTag,
    pub GetForeignRelSize: GetForeignRelSize_function,
    pub GetForeignPaths: GetForeignPaths_function,
    pub GetForeignPlan: GetForeignPlan_function,
    pub BeginForeignScan: BeginForeignScan_function,
    pub IterateForeignScan: IterateForeignScan_function,
    pub ReScanForeignScan: ReScanForeignScan_function,
    pub EndForeignScan: EndForeignScan_function,
    pub GetForeignJoinPaths: GetForeignJoinPaths_function,
    pub GetForeignUpperPaths: GetForeignUpperPaths_function,
    pub AddForeignUpdateTargets: AddForeignUpdateTargets_function,
    pub PlanForeignModify: PlanForeignModify_function,
    pub BeginForeignModify: BeginForeignModify_function,
    pub ExecForeignInsert: ExecForeignInsert_function,
    pub ExecForeignUpdate: ExecForeignUpdate_function,
    pub ExecForeignDelete: ExecForeignDelete_function,
    pub EndForeignModify: EndForeignModify_function,
    pub IsForeignRelUpdatable: IsForeignRelUpdatable_function,
    pub PlanDirectModify: PlanDirectModify_function,
    pub BeginDirectModify: BeginDirectModify_function,
    pub IterateDirectModify: IterateDirectModify_function,
    pub EndDirectModify: EndDirectModify_function,
    pub GetForeignRowMarkType: GetForeignRowMarkType_function,
    pub RefetchForeignRow: RefetchForeignRow_function,
    pub RecheckForeignScan: RecheckForeignScan_function,
    pub ExplainForeignScan: ExplainForeignScan_function,
    pub ExplainForeignModify: ExplainForeignModify_function,
    pub ExplainDirectModify: ExplainDirectModify_function,
    pub AnalyzeForeignTable: AnalyzeForeignTable_function,
    pub ImportForeignSchema: ImportForeignSchema_function,
    pub IsForeignScanParallelSafe: IsForeignScanParallelSafe_function,
    pub EstimateDSMForeignScan: EstimateDSMForeignScan_function,
    pub InitializeDSMForeignScan: InitializeDSMForeignScan_function,
    pub ReInitializeDSMForeignScan: ReInitializeDSMForeignScan_function,
    pub InitializeWorkerForeignScan: InitializeWorkerForeignScan_function,
    pub ShutdownForeignScan: ShutdownForeignScan_function,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ForeignKeyCacheInfo {
    pub type_: NodeTag,
    pub conrelid: Oid,
    pub confrelid: Oid,
    pub nkeys: ::std::os::raw::c_int,
    pub conkey: [AttrNumber; 32usize],
    pub confkey: [AttrNumber; 32usize],
    pub conpfeqop: [Oid; 32usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FuncCallContext {
    pub call_cntr: uint64,
    pub max_calls: uint64,
    pub slot: *mut TupleTableSlot,
    pub user_fctx: *mut ::std::os::raw::c_void,
    pub attinmeta: *mut AttInMetadata,
    pub multi_call_memory_ctx: MemoryContext,
    pub tuple_desc: TupleDesc,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Gather {
    pub plan: Plan,
    pub num_workers: ::std::os::raw::c_int,
    pub rescan_param: ::std::os::raw::c_int,
    pub single_copy: bool,
    pub invisible: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GatherMerge {
    pub plan: Plan,
    pub num_workers: ::std::os::raw::c_int,
    pub rescan_param: ::std::os::raw::c_int,
    pub numCols: ::std::os::raw::c_int,
    pub sortColIdx: *mut AttrNumber,
    pub sortOperators: *mut Oid,
    pub collations: *mut Oid,
    pub nullsFirst: *mut bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GatherMergeState {
    pub ps: PlanState,
    pub initialized: bool,
    pub gm_initialized: bool,
    pub need_to_scan_locally: bool,
    pub tupDesc: TupleDesc,
    pub gm_nkeys: ::std::os::raw::c_int,
    pub gm_sortkeys: SortSupport,
    pub pei: *mut ParallelExecutorInfo,
    pub nworkers_launched: ::std::os::raw::c_int,
    pub nreaders: ::std::os::raw::c_int,
    pub gm_slots: *mut *mut TupleTableSlot,
    pub reader: *mut *mut TupleQueueReader,
    pub gm_tuple_buffers: *mut GMReaderTupleBuffer,
    pub gm_heap: *mut binaryheap,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GatherState {
    pub ps: PlanState,
    pub initialized: bool,
    pub need_to_scan_locally: bool,
    pub funnel_slot: *mut TupleTableSlot,
    pub pei: *mut ParallelExecutorInfo,
    pub nworkers_launched: ::std::os::raw::c_int,
    pub nreaders: ::std::os::raw::c_int,
    pub nextreader: ::std::os::raw::c_int,
    pub reader: *mut *mut TupleQueueReader,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GrantStmt {
    pub type_: NodeTag,
    pub is_grant: bool,
    pub targtype: GrantTargetType,
    pub objtype: GrantObjectType,
    pub objects: *mut List,
    pub privileges: *mut List,
    pub grantees: *mut List,
    pub grant_option: bool,
    pub behavior: DropBehavior,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Group {
    pub plan: Plan,
    pub numCols: ::std::os::raw::c_int,
    pub grpColIdx: *mut AttrNumber,
    pub grpOperators: *mut Oid,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GroupState {
    pub ss: ScanState,
    pub eqfunctions: *mut FmgrInfo,
    pub grp_done: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Hash {
    pub plan: Plan,
    pub skewTable: Oid,
    pub skewColumn: AttrNumber,
    pub skewInherit: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HashJoin {
    pub join: Join,
    pub hashclauses: *mut List,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HashJoinState {
    pub js: JoinState,
    pub hashclauses: *mut ExprState,
    pub hj_OuterHashKeys: *mut List,
    pub hj_InnerHashKeys: *mut List,
    pub hj_HashOperators: *mut List,
    pub hj_HashTable: HashJoinTable,
    pub hj_CurHashValue: uint32,
    pub hj_CurBucketNo: ::std::os::raw::c_int,
    pub hj_CurSkewBucketNo: ::std::os::raw::c_int,
    pub hj_CurTuple: HashJoinTuple,
    pub hj_OuterTupleSlot: *mut TupleTableSlot,
    pub hj_HashTupleSlot: *mut TupleTableSlot,
    pub hj_NullOuterTupleSlot: *mut TupleTableSlot,
    pub hj_NullInnerTupleSlot: *mut TupleTableSlot,
    pub hj_FirstOuterTupleSlot: *mut TupleTableSlot,
    pub hj_JoinState: ::std::os::raw::c_int,
    pub hj_MatchedOuter: bool,
    pub hj_OuterNotEmpty: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HashPath {
    pub jpath: JoinPath,
    pub path_hashclauses: *mut List,
    pub num_batches: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HashState {
    pub ps: PlanState,
    pub hashtable: HashJoinTable,
    pub hashkeys: *mut List,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IndexAmRoutine {
    pub type_: NodeTag,
    pub amstrategies: uint16,
    pub amsupport: uint16,
    pub amcanorder: bool,
    pub amcanorderbyop: bool,
    pub amcanbackward: bool,
    pub amcanunique: bool,
    pub amcanmulticol: bool,
    pub amoptionalkey: bool,
    pub amsearcharray: bool,
    pub amsearchnulls: bool,
    pub amstorage: bool,
    pub amclusterable: bool,
    pub ampredlocks: bool,
    pub amcanparallel: bool,
    pub amkeytype: Oid,
    pub ambuild: ambuild_function,
    pub ambuildempty: ambuildempty_function,
    pub aminsert: aminsert_function,
    pub ambulkdelete: ambulkdelete_function,
    pub amvacuumcleanup: amvacuumcleanup_function,
    pub amcanreturn: amcanreturn_function,
    pub amcostestimate: amcostestimate_function,
    pub amoptions: amoptions_function,
    pub amproperty: amproperty_function,
    pub amvalidate: amvalidate_function,
    pub ambeginscan: ambeginscan_function,
    pub amrescan: amrescan_function,
    pub amgettuple: amgettuple_function,
    pub amgetbitmap: amgetbitmap_function,
    pub amendscan: amendscan_function,
    pub ammarkpos: ammarkpos_function,
    pub amrestrpos: amrestrpos_function,
    pub amestimateparallelscan: amestimateparallelscan_function,
    pub aminitparallelscan: aminitparallelscan_function,
    pub amparallelrescan: amparallelrescan_function,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IndexArrayKeyInfo {
    pub scan_key: ScanKey,
    pub array_expr: *mut ExprState,
    pub next_elem: ::std::os::raw::c_int,
    pub num_elems: ::std::os::raw::c_int,
    pub elem_values: *mut Datum,
    pub elem_nulls: *mut bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IndexInfo {
    pub type_: NodeTag,
    pub ii_NumIndexAttrs: ::std::os::raw::c_int,
    pub ii_KeyAttrNumbers: [AttrNumber; 32usize],
    pub ii_Expressions: *mut List,
    pub ii_ExpressionsState: *mut List,
    pub ii_Predicate: *mut List,
    pub ii_PredicateState: *mut ExprState,
    pub ii_ExclusionOps: *mut Oid,
    pub ii_ExclusionProcs: *mut Oid,
    pub ii_ExclusionStrats: *mut uint16,
    pub ii_UniqueOps: *mut Oid,
    pub ii_UniqueProcs: *mut Oid,
    pub ii_UniqueStrats: *mut uint16,
    pub ii_Unique: bool,
    pub ii_ReadyForInserts: bool,
    pub ii_Concurrent: bool,
    pub ii_BrokenHotChain: bool,
    pub ii_AmCache: *mut ::std::os::raw::c_void,
    pub ii_Context: MemoryContext,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IndexOnlyScanState {
    pub ss: ScanState,
    pub indexqual: *mut ExprState,
    pub ioss_ScanKeys: ScanKey,
    pub ioss_NumScanKeys: ::std::os::raw::c_int,
    pub ioss_OrderByKeys: ScanKey,
    pub ioss_NumOrderByKeys: ::std::os::raw::c_int,
    pub ioss_RuntimeKeys: *mut IndexRuntimeKeyInfo,
    pub ioss_NumRuntimeKeys: ::std::os::raw::c_int,
    pub ioss_RuntimeKeysReady: bool,
    pub ioss_RuntimeContext: *mut ExprContext,
    pub ioss_RelationDesc: Relation,
    pub ioss_ScanDesc: IndexScanDesc,
    pub ioss_VMBuffer: Buffer,
    pub ioss_HeapFetches: ::std::os::raw::c_long,
    pub ioss_PscanLen: Size,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IndexOptInfo {
    pub type_: NodeTag,
    pub indexoid: Oid,
    pub reltablespace: Oid,
    pub rel: *mut RelOptInfo,
    pub pages: BlockNumber,
    pub tuples: f64,
    pub tree_height: ::std::os::raw::c_int,
    pub ncolumns: ::std::os::raw::c_int,
    pub indexkeys: *mut ::std::os::raw::c_int,
    pub indexcollations: *mut Oid,
    pub opfamily: *mut Oid,
    pub opcintype: *mut Oid,
    pub sortopfamily: *mut Oid,
    pub reverse_sort: *mut bool,
    pub nulls_first: *mut bool,
    pub canreturn: *mut bool,
    pub relam: Oid,
    pub indexprs: *mut List,
    pub indpred: *mut List,
    pub indextlist: *mut List,
    pub indrestrictinfo: *mut List,
    pub predOK: bool,
    pub unique: bool,
    pub immediate: bool,
    pub hypothetical: bool,
    pub amcanorderbyop: bool,
    pub amoptionalkey: bool,
    pub amsearcharray: bool,
    pub amsearchnulls: bool,
    pub amhasgettuple: bool,
    pub amhasgetbitmap: bool,
    pub amcanparallel: bool,
    pub amcostestimate: ::std::option::Option<unsafe extern "C" fn()>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IndexPath {
    pub path: Path,
    pub indexinfo: *mut IndexOptInfo,
    pub indexclauses: *mut List,
    pub indexquals: *mut List,
    pub indexqualcols: *mut List,
    pub indexorderbys: *mut List,
    pub indexorderbycols: *mut List,
    pub indexscandir: ScanDirection,
    pub indextotalcost: Cost,
    pub indexselectivity: Selectivity,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IndexQualInfo {
    pub rinfo: *mut RestrictInfo,
    pub indexcol: ::std::os::raw::c_int,
    pub varonleft: bool,
    pub clause_op: Oid,
    pub other_operand: *mut Node,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IndexRuntimeKeyInfo {
    pub scan_key: ScanKey,
    pub key_expr: *mut ExprState,
    pub key_toastable: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IndexScanDescData {
    pub heapRelation: Relation,
    pub indexRelation: Relation,
    pub xs_snapshot: Snapshot,
    pub numberOfKeys: ::std::os::raw::c_int,
    pub numberOfOrderBys: ::std::os::raw::c_int,
    pub keyData: ScanKey,
    pub orderByData: ScanKey,
    pub xs_want_itup: bool,
    pub xs_temp_snap: bool,
    pub kill_prior_tuple: bool,
    pub ignore_killed_tuples: bool,
    pub xactStartedInRecovery: bool,
    pub opaque: *mut ::std::os::raw::c_void,
    pub xs_itup: IndexTuple,
    pub xs_itupdesc: TupleDesc,
    pub xs_hitup: HeapTuple,
    pub xs_hitupdesc: TupleDesc,
    pub xs_ctup: HeapTupleData,
    pub xs_cbuf: Buffer,
    pub xs_recheck: bool,
    pub xs_orderbyvals: *mut Datum,
    pub xs_orderbynulls: *mut bool,
    pub xs_recheckorderby: bool,
    pub xs_continue_hot: bool,
    pub parallel_scan: ParallelIndexScanDesc,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IndexScanState {
    pub ss: ScanState,
    pub indexqualorig: *mut ExprState,
    pub indexorderbyorig: *mut List,
    pub iss_ScanKeys: ScanKey,
    pub iss_NumScanKeys: ::std::os::raw::c_int,
    pub iss_OrderByKeys: ScanKey,
    pub iss_NumOrderByKeys: ::std::os::raw::c_int,
    pub iss_RuntimeKeys: *mut IndexRuntimeKeyInfo,
    pub iss_NumRuntimeKeys: ::std::os::raw::c_int,
    pub iss_RuntimeKeysReady: bool,
    pub iss_RuntimeContext: *mut ExprContext,
    pub iss_RelationDesc: Relation,
    pub iss_ScanDesc: IndexScanDesc,
    pub iss_ReorderQueue: *mut pairingheap,
    pub iss_ReachedEnd: bool,
    pub iss_OrderByValues: *mut Datum,
    pub iss_OrderByNulls: *mut bool,
    pub iss_SortSupport: SortSupport,
    pub iss_OrderByTypByVals: *mut bool,
    pub iss_OrderByTypLens: *mut int16,
    pub iss_PscanLen: Size,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IndexStmt {
    pub type_: NodeTag,
    pub idxname: *mut ::std::os::raw::c_char,
    pub relation: *mut RangeVar,
    pub accessMethod: *mut ::std::os::raw::c_char,
    pub tableSpace: *mut ::std::os::raw::c_char,
    pub indexParams: *mut List,
    pub options: *mut List,
    pub whereClause: *mut Node,
    pub excludeOpNames: *mut List,
    pub idxcomment: *mut ::std::os::raw::c_char,
    pub indexOid: Oid,
    pub oldNode: Oid,
    pub unique: bool,
    pub primary: bool,
    pub isconstraint: bool,
    pub deferrable: bool,
    pub initdeferred: bool,
    pub transformed: bool,
    pub concurrent: bool,
    pub if_not_exists: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IndexVacuumInfo {
    pub index: Relation,
    pub analyze_only: bool,
    pub estimated_count: bool,
    pub message_level: ::std::os::raw::c_int,
    pub num_heap_tuples: f64,
    pub strategy: BufferAccessStrategy,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct InlineCodeBlock {
    pub type_: NodeTag,
    pub source_text: *mut ::std::os::raw::c_char,
    pub langOid: Oid,
    pub langIsTrusted: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct InternalGrant {
    pub is_grant: bool,
    pub objtype: GrantObjectType,
    pub objects: *mut List,
    pub all_privs: bool,
    pub privileges: AclMode,
    pub col_privs: *mut List,
    pub grantees: *mut List,
    pub grant_option: bool,
    pub behavior: DropBehavior,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IntoClause {
    pub type_: NodeTag,
    pub rel: *mut RangeVar,
    pub colNames: *mut List,
    pub options: *mut List,
    pub onCommit: OnCommitAction,
    pub tableSpaceName: *mut ::std::os::raw::c_char,
    pub viewQuery: *mut Node,
    pub skipData: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LOCALLOCK {
    pub tag: LOCALLOCKTAG,
    pub lock: *mut LOCK,
    pub proclock: *mut PROCLOCK,
    pub hashcode: uint32,
    pub nLocks: int64,
    pub numLockOwners: ::std::os::raw::c_int,
    pub maxLockOwners: ::std::os::raw::c_int,
    pub holdsStrongLockCount: bool,
    pub lockCleared: bool,
    pub lockOwners: *mut LOCALLOCKOWNER,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LockRowsState {
    pub ps: PlanState,
    pub lr_arowMarks: *mut List,
    pub lr_epqstate: EPQState,
    pub lr_curtuples: *mut HeapTuple,
    pub lr_ntables: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MemoryContextData {
    pub type_: NodeTag,
    pub isReset: bool,
    pub allowInCritSection: bool,
    pub methods: *mut MemoryContextMethods,
    pub parent: MemoryContext,
    pub firstchild: MemoryContext,
    pub prevchild: MemoryContext,
    pub nextchild: MemoryContext,
    pub name: *mut ::std::os::raw::c_char,
    pub reset_cbs: *mut MemoryContextCallback,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MergeAppend {
    pub plan: Plan,
    pub partitioned_rels: *mut List,
    pub mergeplans: *mut List,
    pub numCols: ::std::os::raw::c_int,
    pub sortColIdx: *mut AttrNumber,
    pub sortOperators: *mut Oid,
    pub collations: *mut Oid,
    pub nullsFirst: *mut bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MergeAppendState {
    pub ps: PlanState,
    pub mergeplans: *mut *mut PlanState,
    pub ms_nplans: ::std::os::raw::c_int,
    pub ms_nkeys: ::std::os::raw::c_int,
    pub ms_sortkeys: SortSupport,
    pub ms_slots: *mut *mut TupleTableSlot,
    pub ms_heap: *mut binaryheap,
    pub ms_initialized: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ModifyTable {
    pub plan: Plan,
    pub operation: CmdType,
    pub canSetTag: bool,
    pub nominalRelation: Index,
    pub partitioned_rels: *mut List,
    pub resultRelations: *mut List,
    pub resultRelIndex: ::std::os::raw::c_int,
    pub rootResultRelIndex: ::std::os::raw::c_int,
    pub plans: *mut List,
    pub withCheckOptionLists: *mut List,
    pub returningLists: *mut List,
    pub fdwPrivLists: *mut List,
    pub fdwDirectModifyPlans: *mut Bitmapset,
    pub rowMarks: *mut List,
    pub epqParam: ::std::os::raw::c_int,
    pub onConflictAction: OnConflictAction,
    pub arbiterIndexes: *mut List,
    pub onConflictSet: *mut List,
    pub onConflictWhere: *mut Node,
    pub exclRelRTI: Index,
    pub exclRelTlist: *mut List,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ModifyTablePath {
    pub path: Path,
    pub operation: CmdType,
    pub canSetTag: bool,
    pub nominalRelation: Index,
    pub partitioned_rels: *mut List,
    pub resultRelations: *mut List,
    pub subpaths: *mut List,
    pub subroots: *mut List,
    pub withCheckOptionLists: *mut List,
    pub returningLists: *mut List,
    pub rowMarks: *mut List,
    pub onconflict: *mut OnConflictExpr,
    pub epqParam: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ModifyTableState {
    pub ps: PlanState,
    pub operation: CmdType,
    pub canSetTag: bool,
    pub mt_done: bool,
    pub mt_plans: *mut *mut PlanState,
    pub mt_nplans: ::std::os::raw::c_int,
    pub mt_whichplan: ::std::os::raw::c_int,
    pub resultRelInfo: *mut ResultRelInfo,
    pub rootResultRelInfo: *mut ResultRelInfo,
    pub mt_arowmarks: *mut *mut List,
    pub mt_epqstate: EPQState,
    pub fireBSTriggers: bool,
    pub mt_onconflict: OnConflictAction,
    pub mt_arbiterindexes: *mut List,
    pub mt_existing: *mut TupleTableSlot,
    pub mt_excludedtlist: *mut List,
    pub mt_conflproj: *mut TupleTableSlot,
    pub mt_partition_dispatch_info: *mut *mut PartitionDispatchData,
    pub mt_num_dispatch: ::std::os::raw::c_int,
    pub mt_num_partitions: ::std::os::raw::c_int,
    pub mt_partitions: *mut ResultRelInfo,
    pub mt_partition_tupconv_maps: *mut *mut TupleConversionMap,
    pub mt_partition_tuple_slot: *mut TupleTableSlot,
    pub mt_transition_capture: *mut TransitionCaptureState,
    pub mt_oc_transition_capture: *mut TransitionCaptureState,
    pub mt_transition_tupconv_maps: *mut *mut TupleConversionMap,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PROC_HDR {
    pub allProcs: *mut PGPROC,
    pub allPgXact: *mut PGXACT,
    pub allProcCount: uint32,
    pub freeProcs: *mut PGPROC,
    pub autovacFreeProcs: *mut PGPROC,
    pub bgworkerFreeProcs: *mut PGPROC,
    pub procArrayGroupFirst: pg_atomic_uint32,
    pub walwriterLatch: *mut Latch,
    pub checkpointerLatch: *mut Latch,
    pub spins_per_delay: ::std::os::raw::c_int,
    pub startupProc: *mut PGPROC,
    pub startupProcPid: ::std::os::raw::c_int,
    pub startupBufferPinWaitBufId: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ParallelContext {
    pub node: dlist_node,
    pub subid: SubTransactionId,
    pub nworkers: ::std::os::raw::c_int,
    pub nworkers_launched: ::std::os::raw::c_int,
    pub library_name: *mut ::std::os::raw::c_char,
    pub function_name: *mut ::std::os::raw::c_char,
    pub error_context_stack: *mut ErrorContextCallback,
    pub estimator: shm_toc_estimator,
    pub seg: *mut dsm_segment,
    pub private_memory: *mut ::std::os::raw::c_void,
    pub toc: *mut shm_toc,
    pub worker: *mut ParallelWorkerInfo,
    pub any_message_received: *mut bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PartitionBoundSpec {
    pub type_: NodeTag,
    pub strategy: ::std::os::raw::c_char,
    pub listdatums: *mut List,
    pub lowerdatums: *mut List,
    pub upperdatums: *mut List,
    pub location: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PartitionDescData {
    pub nparts: ::std::os::raw::c_int,
    pub oids: *mut Oid,
    pub boundinfo: PartitionBoundInfo,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PartitionDispatchData {
    pub reldesc: Relation,
    pub key: PartitionKey,
    pub keystate: *mut List,
    pub partdesc: PartitionDesc,
    pub tupslot: *mut TupleTableSlot,
    pub tupmap: *mut TupleConversionMap,
    pub indexes: *mut ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PartitionKeyData {
    pub strategy: ::std::os::raw::c_char,
    pub partnatts: int16,
    pub partattrs: *mut AttrNumber,
    pub partexprs: *mut List,
    pub partopfamily: *mut Oid,
    pub partopcintype: *mut Oid,
    pub partsupfunc: *mut FmgrInfo,
    pub partcollation: *mut Oid,
    pub parttypid: *mut Oid,
    pub parttypmod: *mut int32,
    pub parttyplen: *mut int16,
    pub parttypbyval: *mut bool,
    pub parttypalign: *mut ::std::os::raw::c_char,
    pub parttypcoll: *mut Oid,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PartitionedChildRelInfo {
    pub type_: NodeTag,
    pub parent_relid: Index,
    pub child_rels: *mut List,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PgStat_StatDBEntry {
    pub databaseid: Oid,
    pub n_xact_commit: PgStat_Counter,
    pub n_xact_rollback: PgStat_Counter,
    pub n_blocks_fetched: PgStat_Counter,
    pub n_blocks_hit: PgStat_Counter,
    pub n_tuples_returned: PgStat_Counter,
    pub n_tuples_fetched: PgStat_Counter,
    pub n_tuples_inserted: PgStat_Counter,
    pub n_tuples_updated: PgStat_Counter,
    pub n_tuples_deleted: PgStat_Counter,
    pub last_autovac_time: TimestampTz,
    pub n_conflict_tablespace: PgStat_Counter,
    pub n_conflict_lock: PgStat_Counter,
    pub n_conflict_snapshot: PgStat_Counter,
    pub n_conflict_bufferpin: PgStat_Counter,
    pub n_conflict_startup_deadlock: PgStat_Counter,
    pub n_temp_files: PgStat_Counter,
    pub n_temp_bytes: PgStat_Counter,
    pub n_deadlocks: PgStat_Counter,
    pub n_block_read_time: PgStat_Counter,
    pub n_block_write_time: PgStat_Counter,
    pub stat_reset_timestamp: TimestampTz,
    pub stats_timestamp: TimestampTz,
    pub tables: *mut HTAB,
    pub functions: *mut HTAB,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PlanState {
    pub type_: NodeTag,
    pub plan: *mut Plan,
    pub state: *mut EState,
    pub ExecProcNode: ExecProcNodeMtd,
    pub ExecProcNodeReal: ExecProcNodeMtd,
    pub instrument: *mut Instrumentation,
    pub worker_instrument: *mut WorkerInstrumentation,
    pub qual: *mut ExprState,
    pub lefttree: *mut PlanState,
    pub righttree: *mut PlanState,
    pub initPlan: *mut List,
    pub subPlan: *mut List,
    pub chgParam: *mut Bitmapset,
    pub ps_ResultTupleSlot: *mut TupleTableSlot,
    pub ps_ExprContext: *mut ExprContext,
    pub ps_ProjInfo: *mut ProjectionInfo,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PlannedStmt {
    pub type_: NodeTag,
    pub commandType: CmdType,
    pub queryId: uint32,
    pub hasReturning: bool,
    pub hasModifyingCTE: bool,
    pub canSetTag: bool,
    pub transientPlan: bool,
    pub dependsOnRole: bool,
    pub parallelModeNeeded: bool,
    pub planTree: *mut Plan,
    pub rtable: *mut List,
    pub resultRelations: *mut List,
    pub nonleafResultRelations: *mut List,
    pub rootResultRelations: *mut List,
    pub subplans: *mut List,
    pub rewindPlanIDs: *mut Bitmapset,
    pub rowMarks: *mut List,
    pub relationOids: *mut List,
    pub invalItems: *mut List,
    pub nParamExec: ::std::os::raw::c_int,
    pub utilityStmt: *mut Node,
    pub stmt_location: ::std::os::raw::c_int,
    pub stmt_len: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PlannerGlobal {
    pub type_: NodeTag,
    pub boundParams: ParamListInfo,
    pub subplans: *mut List,
    pub subroots: *mut List,
    pub rewindPlanIDs: *mut Bitmapset,
    pub finalrtable: *mut List,
    pub finalrowmarks: *mut List,
    pub resultRelations: *mut List,
    pub nonleafResultRelations: *mut List,
    pub rootResultRelations: *mut List,
    pub relationOids: *mut List,
    pub invalItems: *mut List,
    pub nParamExec: ::std::os::raw::c_int,
    pub lastPHId: Index,
    pub lastRowMarkId: Index,
    pub lastPlanNodeId: ::std::os::raw::c_int,
    pub transientPlan: bool,
    pub dependsOnRole: bool,
    pub parallelModeOK: bool,
    pub parallelModeNeeded: bool,
    pub maxParallelHazard: ::std::os::raw::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PlannerInfo {
    pub type_: NodeTag,
    pub parse: *mut Query,
    pub glob: *mut PlannerGlobal,
    pub query_level: Index,
    pub parent_root: *mut PlannerInfo,
    pub plan_params: *mut List,
    pub outer_params: *mut Bitmapset,
    pub simple_rel_array: *mut *mut RelOptInfo,
    pub simple_rel_array_size: ::std::os::raw::c_int,
    pub simple_rte_array: *mut *mut RangeTblEntry,
    pub all_baserels: Relids,
    pub nullable_baserels: Relids,
    pub join_rel_list: *mut List,
    pub join_rel_hash: *mut HTAB,
    pub join_rel_level: *mut *mut List,
    pub join_cur_level: ::std::os::raw::c_int,
    pub init_plans: *mut List,
    pub cte_plan_ids: *mut List,
    pub multiexpr_params: *mut List,
    pub eq_classes: *mut List,
    pub canon_pathkeys: *mut List,
    pub left_join_clauses: *mut List,
    pub right_join_clauses: *mut List,
    pub full_join_clauses: *mut List,
    pub join_info_list: *mut List,
    pub append_rel_list: *mut List,
    pub pcinfo_list: *mut List,
    pub rowMarks: *mut List,
    pub placeholder_list: *mut List,
    pub fkey_list: *mut List,
    pub query_pathkeys: *mut List,
    pub group_pathkeys: *mut List,
    pub window_pathkeys: *mut List,
    pub distinct_pathkeys: *mut List,
    pub sort_pathkeys: *mut List,
    pub initial_rels: *mut List,
    pub upper_rels: [*mut List; 6usize],
    pub upper_targets: [*mut PathTarget; 6usize],
    pub processed_tlist: *mut List,
    pub grouping_map: *mut AttrNumber,
    pub minmax_aggs: *mut List,
    pub planner_cxt: MemoryContext,
    pub total_table_pages: f64,
    pub tuple_fraction: f64,
    pub limit_tuples: f64,
    pub qual_security_level: Index,
    pub hasInheritedTarget: bool,
    pub hasJoinRTEs: bool,
    pub hasLateralRTEs: bool,
    pub hasDeletedRTEs: bool,
    pub hasHavingQual: bool,
    pub hasPseudoConstantQuals: bool,
    pub hasRecursion: bool,
    pub wt_param_id: ::std::os::raw::c_int,
    pub non_recursive_path: *mut Path,
    pub curOuterRels: Relids,
    pub curOuterParams: *mut List,
    pub join_search_private: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PortalData {
    pub name: *const ::std::os::raw::c_char,
    pub prepStmtName: *const ::std::os::raw::c_char,
    pub heap: MemoryContext,
    pub resowner: ResourceOwner,
    pub cleanup: ::std::option::Option<unsafe extern "C" fn(portal: Portal)>,
    pub createSubid: SubTransactionId,
    pub activeSubid: SubTransactionId,
    pub sourceText: *const ::std::os::raw::c_char,
    pub commandTag: *const ::std::os::raw::c_char,
    pub stmts: *mut List,
    pub cplan: *mut CachedPlan,
    pub portalParams: ParamListInfo,
    pub queryEnv: *mut QueryEnvironment,
    pub strategy: PortalStrategy,
    pub cursorOptions: ::std::os::raw::c_int,
    pub run_once: bool,
    pub status: PortalStatus,
    pub portalPinned: bool,
    pub queryDesc: *mut QueryDesc,
    pub tupDesc: TupleDesc,
    pub formats: *mut int16,
    pub holdStore: *mut Tuplestorestate,
    pub holdContext: MemoryContext,
    pub holdSnapshot: Snapshot,
    pub atStart: bool,
    pub atEnd: bool,
    pub portalPos: uint64,
    pub creation_time: TimestampTz,
    pub visible: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ProjectSetState {
    pub ps: PlanState,
    pub elems: *mut *mut Node,
    pub elemdone: *mut ExprDoneCond,
    pub nelems: ::std::os::raw::c_int,
    pub pending_srf_tuples: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RangeTblEntry {
    pub type_: NodeTag,
    pub rtekind: RTEKind,
    pub relid: Oid,
    pub relkind: ::std::os::raw::c_char,
    pub tablesample: *mut TableSampleClause,
    pub subquery: *mut Query,
    pub security_barrier: bool,
    pub jointype: JoinType,
    pub joinaliasvars: *mut List,
    pub functions: *mut List,
    pub funcordinality: bool,
    pub tablefunc: *mut TableFunc,
    pub values_lists: *mut List,
    pub ctename: *mut ::std::os::raw::c_char,
    pub ctelevelsup: Index,
    pub self_reference: bool,
    pub coltypes: *mut List,
    pub coltypmods: *mut List,
    pub colcollations: *mut List,
    pub enrname: *mut ::std::os::raw::c_char,
    pub enrtuples: f64,
    pub alias: *mut Alias,
    pub eref: *mut Alias,
    pub lateral: bool,
    pub inh: bool,
    pub inFromCl: bool,
    pub requiredPerms: AclMode,
    pub checkAsUser: Oid,
    pub selectedCols: *mut Bitmapset,
    pub insertedCols: *mut Bitmapset,
    pub updatedCols: *mut Bitmapset,
    pub securityQuals: *mut List,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RecursiveUnion {
    pub plan: Plan,
    pub wtParam: ::std::os::raw::c_int,
    pub numCols: ::std::os::raw::c_int,
    pub dupColIdx: *mut AttrNumber,
    pub dupOperators: *mut Oid,
    pub numGroups: ::std::os::raw::c_long,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RecursiveUnionState {
    pub ps: PlanState,
    pub recursing: bool,
    pub intermediate_empty: bool,
    pub working_table: *mut Tuplestorestate,
    pub intermediate_table: *mut Tuplestorestate,
    pub eqfunctions: *mut FmgrInfo,
    pub hashfunctions: *mut FmgrInfo,
    pub tempContext: MemoryContext,
    pub hashtable: TupleHashTable,
    pub tableContext: MemoryContext,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ReindexStmt {
    pub type_: NodeTag,
    pub kind: ReindexObjectType,
    pub relation: *mut RangeVar,
    pub name: *const ::std::os::raw::c_char,
    pub options: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RelOptInfo {
    pub type_: NodeTag,
    pub reloptkind: RelOptKind,
    pub relids: Relids,
    pub rows: f64,
    pub consider_startup: bool,
    pub consider_param_startup: bool,
    pub consider_parallel: bool,
    pub reltarget: *mut PathTarget,
    pub pathlist: *mut List,
    pub ppilist: *mut List,
    pub partial_pathlist: *mut List,
    pub cheapest_startup_path: *mut Path,
    pub cheapest_total_path: *mut Path,
    pub cheapest_unique_path: *mut Path,
    pub cheapest_parameterized_paths: *mut List,
    pub direct_lateral_relids: Relids,
    pub lateral_relids: Relids,
    pub relid: Index,
    pub reltablespace: Oid,
    pub rtekind: RTEKind,
    pub min_attr: AttrNumber,
    pub max_attr: AttrNumber,
    pub attr_needed: *mut Relids,
    pub attr_widths: *mut int32,
    pub lateral_vars: *mut List,
    pub lateral_referencers: Relids,
    pub indexlist: *mut List,
    pub statlist: *mut List,
    pub pages: BlockNumber,
    pub tuples: f64,
    pub allvisfrac: f64,
    pub subroot: *mut PlannerInfo,
    pub subplan_params: *mut List,
    pub rel_parallel_workers: ::std::os::raw::c_int,
    pub serverid: Oid,
    pub userid: Oid,
    pub useridiscurrent: bool,
    pub fdwroutine: *mut FdwRoutine,
    pub fdw_private: *mut ::std::os::raw::c_void,
    pub unique_for_rels: *mut List,
    pub non_unique_for_rels: *mut List,
    pub baserestrictinfo: *mut List,
    pub baserestrictcost: QualCost,
    pub baserestrict_min_security: Index,
    pub joininfo: *mut List,
    pub has_eclass_joins: bool,
    pub top_parent_relids: Relids,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RelationData {
    pub rd_node: RelFileNode,
    pub rd_smgr: *mut SMgrRelationData,
    pub rd_refcnt: ::std::os::raw::c_int,
    pub rd_backend: BackendId,
    pub rd_islocaltemp: bool,
    pub rd_isnailed: bool,
    pub rd_isvalid: bool,
    pub rd_indexvalid: ::std::os::raw::c_char,
    pub rd_statvalid: bool,
    pub rd_createSubid: SubTransactionId,
    pub rd_newRelfilenodeSubid: SubTransactionId,
    pub rd_rel: Form_pg_class,
    pub rd_att: TupleDesc,
    pub rd_id: Oid,
    pub rd_lockInfo: LockInfoData,
    pub rd_rules: *mut RuleLock,
    pub rd_rulescxt: MemoryContext,
    pub trigdesc: *mut TriggerDesc,
    pub rd_rsdesc: *mut RowSecurityDesc,
    pub rd_fkeylist: *mut List,
    pub rd_fkeyvalid: bool,
    pub rd_partkeycxt: MemoryContext,
    pub rd_partkey: *mut PartitionKeyData,
    pub rd_pdcxt: MemoryContext,
    pub rd_partdesc: *mut PartitionDescData,
    pub rd_partcheck: *mut List,
    pub rd_indexlist: *mut List,
    pub rd_oidindex: Oid,
    pub rd_pkindex: Oid,
    pub rd_replidindex: Oid,
    pub rd_statlist: *mut List,
    pub rd_indexattr: *mut Bitmapset,
    pub rd_keyattr: *mut Bitmapset,
    pub rd_pkattr: *mut Bitmapset,
    pub rd_idattr: *mut Bitmapset,
    pub rd_pubactions: *mut PublicationActions,
    pub rd_options: *mut bytea,
    pub rd_index: Form_pg_index,
    pub rd_indextuple: *mut HeapTupleData,
    pub rd_amhandler: Oid,
    pub rd_indexcxt: MemoryContext,
    pub rd_amroutine: *mut IndexAmRoutine,
    pub rd_opfamily: *mut Oid,
    pub rd_opcintype: *mut Oid,
    pub rd_support: *mut RegProcedure,
    pub rd_supportinfo: *mut FmgrInfo,
    pub rd_indoption: *mut int16,
    pub rd_indexprs: *mut List,
    pub rd_indpred: *mut List,
    pub rd_exclops: *mut Oid,
    pub rd_exclprocs: *mut Oid,
    pub rd_exclstrats: *mut uint16,
    pub rd_amcache: *mut ::std::os::raw::c_void,
    pub rd_indcollation: *mut Oid,
    pub rd_fdwroutine: *mut FdwRoutine,
    pub rd_toastoid: Oid,
    pub pgstat_info: *mut PgStat_TableStatus,
    pub rd_partcheckvalid: bool,
    pub rd_partcheckcxt: MemoryContext,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RestrictInfo {
    pub type_: NodeTag,
    pub clause: *mut Expr,
    pub is_pushed_down: bool,
    pub outerjoin_delayed: bool,
    pub can_join: bool,
    pub pseudoconstant: bool,
    pub leakproof: bool,
    pub security_level: Index,
    pub clause_relids: Relids,
    pub required_relids: Relids,
    pub outer_relids: Relids,
    pub nullable_relids: Relids,
    pub left_relids: Relids,
    pub right_relids: Relids,
    pub orclause: *mut Expr,
    pub parent_ec: *mut EquivalenceClass,
    pub eval_cost: QualCost,
    pub norm_selec: Selectivity,
    pub outer_selec: Selectivity,
    pub mergeopfamilies: *mut List,
    pub left_ec: *mut EquivalenceClass,
    pub right_ec: *mut EquivalenceClass,
    pub left_em: *mut EquivalenceMember,
    pub right_em: *mut EquivalenceMember,
    pub scansel_cache: *mut List,
    pub outer_is_left: bool,
    pub hashjoinoperator: Oid,
    pub left_bucketsize: Selectivity,
    pub right_bucketsize: Selectivity,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ResultPath {
    pub path: Path,
    pub quals: *mut List,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ResultRelInfo {
    pub type_: NodeTag,
    pub ri_RangeTableIndex: Index,
    pub ri_RelationDesc: Relation,
    pub ri_NumIndices: ::std::os::raw::c_int,
    pub ri_IndexRelationDescs: RelationPtr,
    pub ri_IndexRelationInfo: *mut *mut IndexInfo,
    pub ri_TrigDesc: *mut TriggerDesc,
    pub ri_TrigFunctions: *mut FmgrInfo,
    pub ri_TrigWhenExprs: *mut *mut ExprState,
    pub ri_TrigInstrument: *mut Instrumentation,
    pub ri_FdwRoutine: *mut FdwRoutine,
    pub ri_FdwState: *mut ::std::os::raw::c_void,
    pub ri_usesFdwDirectModify: bool,
    pub ri_WithCheckOptions: *mut List,
    pub ri_WithCheckOptionExprs: *mut List,
    pub ri_ConstraintExprs: *mut *mut ExprState,
    pub ri_junkFilter: *mut JunkFilter,
    pub ri_projectReturning: *mut ProjectionInfo,
    pub ri_onConflictSetProj: *mut ProjectionInfo,
    pub ri_onConflictSetWhere: *mut ExprState,
    pub ri_PartitionCheck: *mut List,
    pub ri_PartitionCheckExpr: *mut ExprState,
    pub ri_PartitionRoot: Relation,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SampleScanState {
    pub ss: ScanState,
    pub args: *mut List,
    pub repeatable: *mut ExprState,
    pub tsmroutine: *mut TsmRoutine,
    pub tsm_state: *mut ::std::os::raw::c_void,
    pub use_bulkread: bool,
    pub use_pagemode: bool,
    pub begun: bool,
    pub seed: uint32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScanState {
    pub ps: PlanState,
    pub ss_currentRelation: Relation,
    pub ss_currentScanDesc: HeapScanDesc,
    pub ss_ScanTupleSlot: *mut TupleTableSlot,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SetOp {
    pub plan: Plan,
    pub cmd: SetOpCmd,
    pub strategy: SetOpStrategy,
    pub numCols: ::std::os::raw::c_int,
    pub dupColIdx: *mut AttrNumber,
    pub dupOperators: *mut Oid,
    pub flagColIdx: AttrNumber,
    pub firstFlag: ::std::os::raw::c_int,
    pub numGroups: ::std::os::raw::c_long,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SetOpState {
    pub ps: PlanState,
    pub eqfunctions: *mut FmgrInfo,
    pub hashfunctions: *mut FmgrInfo,
    pub setop_done: bool,
    pub numOutput: ::std::os::raw::c_long,
    pub tempContext: MemoryContext,
    pub pergroup: SetOpStatePerGroup,
    pub grp_firstTuple: HeapTuple,
    pub hashtable: TupleHashTable,
    pub tableContext: MemoryContext,
    pub table_filled: bool,
    pub hashiter: TupleHashIterator,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SnapshotData {
    pub satisfies: SnapshotSatisfiesFunc,
    pub xmin: TransactionId,
    pub xmax: TransactionId,
    pub xip: *mut TransactionId,
    pub xcnt: uint32,
    pub subxip: *mut TransactionId,
    pub subxcnt: int32,
    pub suboverflowed: bool,
    pub takenDuringRecovery: bool,
    pub copied: bool,
    pub curcid: CommandId,
    pub speculativeToken: uint32,
    pub active_count: uint32,
    pub regd_count: uint32,
    pub ph_node: pairingheap_node,
    pub whenTaken: TimestampTz,
    pub lsn: XLogRecPtr,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SortState {
    pub ss: ScanState,
    pub randomAccess: bool,
    pub bounded: bool,
    pub bound: int64,
    pub sort_Done: bool,
    pub bounded_Done: bool,
    pub bound_Done: int64,
    pub tuplesortstate: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SubPlanState {
    pub type_: NodeTag,
    pub subplan: *mut SubPlan,
    pub planstate: *mut PlanState,
    pub parent: *mut PlanState,
    pub testexpr: *mut ExprState,
    pub args: *mut List,
    pub curTuple: HeapTuple,
    pub curArray: Datum,
    pub projLeft: *mut ProjectionInfo,
    pub projRight: *mut ProjectionInfo,
    pub hashtable: TupleHashTable,
    pub hashnulls: TupleHashTable,
    pub havehashrows: bool,
    pub havenullrows: bool,
    pub hashtablecxt: MemoryContext,
    pub hashtempcxt: MemoryContext,
    pub innerecontext: *mut ExprContext,
    pub keyColIdx: *mut AttrNumber,
    pub tab_hash_funcs: *mut FmgrInfo,
    pub tab_eq_funcs: *mut FmgrInfo,
    pub lhs_hash_funcs: *mut FmgrInfo,
    pub cur_eq_funcs: *mut FmgrInfo,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SysScanDescData {
    pub heap_rel: Relation,
    pub irel: Relation,
    pub scan: HeapScanDesc,
    pub iscan: IndexScanDesc,
    pub snapshot: Snapshot,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TransactionStmt {
    pub type_: NodeTag,
    pub kind: TransactionStmtKind,
    pub options: *mut List,
    pub gid: *mut ::std::os::raw::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TransitionCaptureState {
    pub tcs_delete_old_table: bool,
    pub tcs_update_old_table: bool,
    pub tcs_update_new_table: bool,
    pub tcs_insert_new_table: bool,
    pub tcs_map: *mut TupleConversionMap,
    pub tcs_original_insert_tuple: HeapTuple,
    pub tcs_private: *mut AfterTriggersTableData,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TriggerData {
    pub type_: NodeTag,
    pub tg_event: TriggerEvent,
    pub tg_relation: Relation,
    pub tg_trigtuple: HeapTuple,
    pub tg_newtuple: HeapTuple,
    pub tg_trigger: *mut Trigger,
    pub tg_trigtuplebuf: Buffer,
    pub tg_newtuplebuf: Buffer,
    pub tg_oldtable: *mut Tuplestorestate,
    pub tg_newtable: *mut Tuplestorestate,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TupleHashTableData {
    pub hashtab: *mut tuplehash_hash,
    pub numCols: ::std::os::raw::c_int,
    pub keyColIdx: *mut AttrNumber,
    pub tab_hash_funcs: *mut FmgrInfo,
    pub tab_eq_funcs: *mut FmgrInfo,
    pub tablecxt: MemoryContext,
    pub tempcxt: MemoryContext,
    pub entrysize: Size,
    pub tableslot: *mut TupleTableSlot,
    pub inputslot: *mut TupleTableSlot,
    pub in_hash_funcs: *mut FmgrInfo,
    pub cur_eq_funcs: *mut FmgrInfo,
    pub hash_iv: uint32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TupleTableSlot {
    pub type_: NodeTag,
    pub tts_isempty: bool,
    pub tts_shouldFree: bool,
    pub tts_shouldFreeMin: bool,
    pub tts_slow: bool,
    pub tts_tuple: HeapTuple,
    pub tts_tupleDescriptor: TupleDesc,
    pub tts_mcxt: MemoryContext,
    pub tts_buffer: Buffer,
    pub tts_nvalid: ::std::os::raw::c_int,
    pub tts_values: *mut Datum,
    pub tts_isnull: *mut bool,
    pub tts_mintuple: MinimalTuple,
    pub tts_minhdr: HeapTupleData,
    pub tts_off: ::std::os::raw::c_long,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TypeCacheEntry {
    pub type_id: Oid,
    pub typlen: int16,
    pub typbyval: bool,
    pub typalign: ::std::os::raw::c_char,
    pub typstorage: ::std::os::raw::c_char,
    pub typtype: ::std::os::raw::c_char,
    pub typrelid: Oid,
    pub btree_opf: Oid,
    pub btree_opintype: Oid,
    pub hash_opf: Oid,
    pub hash_opintype: Oid,
    pub eq_opr: Oid,
    pub lt_opr: Oid,
    pub gt_opr: Oid,
    pub cmp_proc: Oid,
    pub hash_proc: Oid,
    pub eq_opr_finfo: FmgrInfo,
    pub cmp_proc_finfo: FmgrInfo,
    pub hash_proc_finfo: FmgrInfo,
    pub tupDesc: TupleDesc,
    pub rngelemtype: *mut TypeCacheEntry,
    pub rng_collation: Oid,
    pub rng_cmp_proc_finfo: FmgrInfo,
    pub rng_canonical_finfo: FmgrInfo,
    pub rng_subdiff_finfo: FmgrInfo,
    pub domainData: *mut DomainConstraintCache,
    pub flags: ::std::os::raw::c_int,
    pub enumData: *mut TypeCacheEnumData,
    pub nextDomain: *mut TypeCacheEntry,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Unique {
    pub plan: Plan,
    pub numCols: ::std::os::raw::c_int,
    pub uniqColIdx: *mut AttrNumber,
    pub uniqOperators: *mut Oid,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UniqueState {
    pub ps: PlanState,
    pub eqfunctions: *mut FmgrInfo,
    pub tempContext: MemoryContext,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VacuumStmt {
    pub type_: NodeTag,
    pub options: ::std::os::raw::c_int,
    pub relation: *mut RangeVar,
    pub va_cols: *mut List,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WindowAgg {
    pub plan: Plan,
    pub winref: Index,
    pub partNumCols: ::std::os::raw::c_int,
    pub partColIdx: *mut AttrNumber,
    pub partOperators: *mut Oid,
    pub ordNumCols: ::std::os::raw::c_int,
    pub ordColIdx: *mut AttrNumber,
    pub ordOperators: *mut Oid,
    pub frameOptions: ::std::os::raw::c_int,
    pub startOffset: *mut Node,
    pub endOffset: *mut Node,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WindowAggPath {
    pub path: Path,
    pub subpath: *mut Path,
    pub winclause: *mut WindowClause,
    pub winpathkeys: *mut List,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WindowAggState {
    pub ss: ScanState,
    pub funcs: *mut List,
    pub numfuncs: ::std::os::raw::c_int,
    pub numaggs: ::std::os::raw::c_int,
    pub perfunc: WindowStatePerFunc,
    pub peragg: WindowStatePerAgg,
    pub partEqfunctions: *mut FmgrInfo,
    pub ordEqfunctions: *mut FmgrInfo,
    pub buffer: *mut Tuplestorestate,
    pub current_ptr: ::std::os::raw::c_int,
    pub spooled_rows: int64,
    pub currentpos: int64,
    pub frameheadpos: int64,
    pub frametailpos: int64,
    pub agg_winobj: *mut WindowObjectData,
    pub aggregatedbase: int64,
    pub aggregatedupto: int64,
    pub frameOptions: ::std::os::raw::c_int,
    pub startOffset: *mut ExprState,
    pub endOffset: *mut ExprState,
    pub startOffsetValue: Datum,
    pub endOffsetValue: Datum,
    pub partcontext: MemoryContext,
    pub aggcontext: MemoryContext,
    pub curaggcontext: MemoryContext,
    pub tmpcontext: *mut ExprContext,
    pub all_first: bool,
    pub all_done: bool,
    pub partition_spooled: bool,
    pub more_partitions: bool,
    pub framehead_valid: bool,
    pub frametail_valid: bool,
    pub first_part_slot: *mut TupleTableSlot,
    pub agg_row_slot: *mut TupleTableSlot,
    pub temp_slot_1: *mut TupleTableSlot,
    pub temp_slot_2: *mut TupleTableSlot,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WindowClause {
    pub type_: NodeTag,
    pub name: *mut ::std::os::raw::c_char,
    pub refname: *mut ::std::os::raw::c_char,
    pub partitionClause: *mut List,
    pub orderClause: *mut List,
    pub frameOptions: ::std::os::raw::c_int,
    pub startOffset: *mut Node,
    pub endOffset: *mut Node,
    pub winref: Index,
    pub copiedOrder: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct attrDefault {
    pub adnum: AttrNumber,
    pub adbin: *mut ::std::os::raw::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct constrCheck {
    pub ccname: *mut ::std::os::raw::c_char,
    pub ccbin: *mut ::std::os::raw::c_char,
    pub ccvalid: bool,
    pub ccnoinherit: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tupleConstr {
    pub defval: *mut AttrDefault,
    pub check: *mut ConstrCheck,
    pub num_defval: uint16,
    pub num_check: uint16,
    pub has_not_null: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tupleDesc {
    pub natts: ::std::os::raw::c_int,
    pub attrs: *mut Form_pg_attribute,
    pub constr: *mut TupleConstr,
    pub tdtypeid: Oid,
    pub tdtypmod: int32,
    pub tdhasoid: bool,
    pub tdrefcount: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xl_xact_parsed_abort {
    pub xact_time: TimestampTz,
    pub xinfo: uint32,
    pub nsubxacts: ::std::os::raw::c_int,
    pub subxacts: *mut TransactionId,
    pub nrels: ::std::os::raw::c_int,
    pub xnodes: *mut RelFileNode,
    pub twophase_xid: TransactionId,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xl_xact_parsed_commit {
    pub xact_time: TimestampTz,
    pub xinfo: uint32,
    pub dbId: Oid,
    pub tsId: Oid,
    pub nsubxacts: ::std::os::raw::c_int,
    pub subxacts: *mut TransactionId,
    pub nrels: ::std::os::raw::c_int,
    pub xnodes: *mut RelFileNode,
    pub nmsgs: ::std::os::raw::c_int,
    pub msgs: *mut SharedInvalidationMessage,
    pub twophase_xid: TransactionId,
    pub origin_lsn: XLogRecPtr,
    pub origin_timestamp: TimestampTz,
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct FormData_pg_index {
    pub indexrelid: Oid,
    pub indrelid: Oid,
    pub indnatts: int16,
    pub indisunique: bool,
    pub indisprimary: bool,
    pub indisexclusion: bool,
    pub indimmediate: bool,
    pub indisclustered: bool,
    pub indisvalid: bool,
    pub indcheckxmin: bool,
    pub indisready: bool,
    pub indislive: bool,
    pub indisreplident: bool,
    pub indkey: int2vector,
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct ParallelHeapScanDescData {
    pub phs_relid: Oid,
    pub phs_syncscan: bool,
    pub phs_nblocks: BlockNumber,
    pub phs_mutex: slock_t,
    pub phs_startblock: BlockNumber,
    pub phs_cblock: BlockNumber,
    pub phs_snapshot_data: __IncompleteArrayField<::std::os::raw::c_char>,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct AggClauseCosts {
    pub numAggs: ::std::os::raw::c_int,
    pub numOrderedAggs: ::std::os::raw::c_int,
    pub hasNonPartial: bool,
    pub hasNonSerial: bool,
    pub transCost: QualCost,
    pub finalCost: Cost,
    pub transitionSpace: Size,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct AutoVacOpts {
    pub enabled: bool,
    pub vacuum_threshold: ::std::os::raw::c_int,
    pub analyze_threshold: ::std::os::raw::c_int,
    pub vacuum_cost_delay: ::std::os::raw::c_int,
    pub vacuum_cost_limit: ::std::os::raw::c_int,
    pub freeze_min_age: ::std::os::raw::c_int,
    pub freeze_max_age: ::std::os::raw::c_int,
    pub freeze_table_age: ::std::os::raw::c_int,
    pub multixact_freeze_min_age: ::std::os::raw::c_int,
    pub multixact_freeze_max_age: ::std::os::raw::c_int,
    pub multixact_freeze_table_age: ::std::os::raw::c_int,
    pub log_min_duration: ::std::os::raw::c_int,
    pub vacuum_scale_factor: float8,
    pub analyze_scale_factor: float8,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct HeapUpdateFailureData {
    pub ctid: ItemPointerData,
    pub xmax: TransactionId,
    pub cmax: CommandId,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Instrumentation {
    pub need_timer: bool,
    pub need_bufusage: bool,
    pub running: bool,
    pub starttime: instr_time,
    pub counter: instr_time,
    pub firsttuple: f64,
    pub tuplecount: f64,
    pub bufusage_start: BufferUsage,
    pub startup: f64,
    pub total: f64,
    pub ntuples: f64,
    pub nloops: f64,
    pub nfiltered1: f64,
    pub nfiltered2: f64,
    pub bufusage: BufferUsage,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct JoinCostWorkspace {
    pub startup_cost: Cost,
    pub total_cost: Cost,
    pub run_cost: Cost,
    pub inner_run_cost: Cost,
    pub inner_rescan_run_cost: Cost,
    pub outer_rows: f64,
    pub inner_rows: f64,
    pub outer_skip_rows: f64,
    pub inner_skip_rows: f64,
    pub numbuckets: ::std::os::raw::c_int,
    pub numbatches: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct MemoryContextMethods {
    pub alloc: ::std::option::Option<
        unsafe extern "C" fn(context: MemoryContext, size: Size) -> *mut ::std::os::raw::c_void,
    >,
    pub free_p: ::std::option::Option<
        unsafe extern "C" fn(context: MemoryContext, pointer: *mut ::std::os::raw::c_void),
    >,
    pub realloc: ::std::option::Option<
        unsafe extern "C" fn(
            context: MemoryContext,
            pointer: *mut ::std::os::raw::c_void,
            size: Size,
        ) -> *mut ::std::os::raw::c_void,
    >,
    pub init: ::std::option::Option<unsafe extern "C" fn(context: MemoryContext)>,
    pub reset: ::std::option::Option<unsafe extern "C" fn(context: MemoryContext)>,
    pub delete_context: ::std::option::Option<unsafe extern "C" fn(context: MemoryContext)>,
    pub get_chunk_space: ::std::option::Option<
        unsafe extern "C" fn(context: MemoryContext, pointer: *mut ::std::os::raw::c_void) -> Size,
    >,
    pub is_empty: ::std::option::Option<unsafe extern "C" fn(context: MemoryContext) -> bool>,
    pub stats: ::std::option::Option<
        unsafe extern "C" fn(
            context: MemoryContext,
            level: ::std::os::raw::c_int,
            print: bool,
            totals: *mut MemoryContextCounters,
        ),
    >,
    pub check: ::std::option::Option<unsafe extern "C" fn(context: MemoryContext)>,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct PublicationActions {
    pub pubinsert: bool,
    pub pubupdate: bool,
    pub pubdelete: bool,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct StdRdOptions {
    pub vl_len_: int32,
    pub fillfactor: ::std::os::raw::c_int,
    pub autovacuum: AutoVacOpts,
    pub user_catalog_table: bool,
    pub parallel_workers: ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct VariableCacheData {
    pub nextOid: Oid,
    pub oidCount: uint32,
    pub nextXid: TransactionId,
    pub oldestXid: TransactionId,
    pub xidVacLimit: TransactionId,
    pub xidWarnLimit: TransactionId,
    pub xidStopLimit: TransactionId,
    pub xidWrapLimit: TransactionId,
    pub oldestXidDB: Oid,
    pub oldestCommitTsXid: TransactionId,
    pub newestCommitTsXid: TransactionId,
    pub latestCompletedXid: TransactionId,
    pub oldestClogXid: TransactionId,
}
#[repr(C)]
pub struct FormData_pg_trigger {
    pub tgrelid: Oid,
    pub tgname: NameData,
    pub tgfoid: Oid,
    pub tgtype: int16,
    pub tgenabled: ::std::os::raw::c_char,
    pub tgisinternal: bool,
    pub tgconstrrelid: Oid,
    pub tgconstrindid: Oid,
    pub tgconstraint: Oid,
    pub tgdeferrable: bool,
    pub tginitdeferred: bool,
    pub tgnargs: int16,
    pub tgattr: int2vector,
}
impl Default for ArrayRef {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for FunctionCallInfoData {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for HeapScanDescData {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for IndexQualInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for PartitionDescData {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for PartitionDispatchData {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for PartitionKeyData {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for PartitionedChildRelInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for ResultPath {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for attrDefault {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for constrCheck {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for tupleConstr {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl Default for tupleDesc {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub const ABSTIMEOID: u32 = 702;
pub const ACL_ALL_RIGHTS_NAMESPACE: u32 = 768;
pub const AT_REWRITE_ALTER_OID: u32 = 8;
pub const AclObjectKind_ACL_KIND_CLASS: AclObjectKind = 1;
pub const AclObjectKind_ACL_KIND_COLLATION: AclObjectKind = 12;
pub const AclObjectKind_ACL_KIND_COLUMN: AclObjectKind = 0;
pub const AclObjectKind_ACL_KIND_CONVERSION: AclObjectKind = 13;
pub const AclObjectKind_ACL_KIND_DATABASE: AclObjectKind = 3;
pub const AclObjectKind_ACL_KIND_EVENT_TRIGGER: AclObjectKind = 20;
pub const AclObjectKind_ACL_KIND_EXTENSION: AclObjectKind = 21;
pub const AclObjectKind_ACL_KIND_FDW: AclObjectKind = 18;
pub const AclObjectKind_ACL_KIND_FOREIGN_SERVER: AclObjectKind = 19;
pub const AclObjectKind_ACL_KIND_LANGUAGE: AclObjectKind = 7;
pub const AclObjectKind_ACL_KIND_LARGEOBJECT: AclObjectKind = 8;
pub const AclObjectKind_ACL_KIND_NAMESPACE: AclObjectKind = 9;
pub const AclObjectKind_ACL_KIND_OPCLASS: AclObjectKind = 10;
pub const AclObjectKind_ACL_KIND_OPER: AclObjectKind = 5;
pub const AclObjectKind_ACL_KIND_OPFAMILY: AclObjectKind = 11;
pub const AclObjectKind_ACL_KIND_PROC: AclObjectKind = 4;
pub const AclObjectKind_ACL_KIND_PUBLICATION: AclObjectKind = 22;
pub const AclObjectKind_ACL_KIND_SEQUENCE: AclObjectKind = 2;
pub const AclObjectKind_ACL_KIND_STATISTICS: AclObjectKind = 14;
pub const AclObjectKind_ACL_KIND_SUBSCRIPTION: AclObjectKind = 23;
pub const AclObjectKind_ACL_KIND_TABLESPACE: AclObjectKind = 15;
pub const AclObjectKind_ACL_KIND_TSCONFIGURATION: AclObjectKind = 17;
pub const AclObjectKind_ACL_KIND_TSDICTIONARY: AclObjectKind = 16;
pub const AclObjectKind_ACL_KIND_TYPE: AclObjectKind = 6;
pub const AclObjectKind_MAX_ACL_KIND: AclObjectKind = 24;
pub const AlterTableType_AT_AddConstraint: AlterTableType = 14;
pub const AlterTableType_AT_AddConstraintRecurse: AlterTableType = 15;
pub const AlterTableType_AT_AddIdentity: AlterTableType = 63;
pub const AlterTableType_AT_AddIndex: AlterTableType = 12;
pub const AlterTableType_AT_AddIndexConstraint: AlterTableType = 21;
pub const AlterTableType_AT_AddInherit: AlterTableType = 51;
pub const AlterTableType_AT_AddOf: AlterTableType = 53;
pub const AlterTableType_AT_AddOids: AlterTableType = 32;
pub const AlterTableType_AT_AddOidsRecurse: AlterTableType = 33;
pub const AlterTableType_AT_AlterColumnGenericOptions: AlterTableType = 26;
pub const AlterTableType_AT_AlterColumnType: AlterTableType = 25;
pub const AlterTableType_AT_AlterConstraint: AlterTableType = 17;
pub const AlterTableType_AT_AttachPartition: AlterTableType = 61;
pub const AlterTableType_AT_ChangeOwner: AlterTableType = 27;
pub const AlterTableType_AT_ClusterOn: AlterTableType = 28;
pub const AlterTableType_AT_DetachPartition: AlterTableType = 62;
pub const AlterTableType_AT_DisableRowSecurity: AlterTableType = 57;
pub const AlterTableType_AT_DisableRule: AlterTableType = 50;
pub const AlterTableType_AT_DisableTrig: AlterTableType = 42;
pub const AlterTableType_AT_DisableTrigAll: AlterTableType = 44;
pub const AlterTableType_AT_DisableTrigUser: AlterTableType = 46;
pub const AlterTableType_AT_DropCluster: AlterTableType = 29;
pub const AlterTableType_AT_DropColumn: AlterTableType = 10;
pub const AlterTableType_AT_DropColumnRecurse: AlterTableType = 11;
pub const AlterTableType_AT_DropConstraint: AlterTableType = 22;
pub const AlterTableType_AT_DropConstraintRecurse: AlterTableType = 23;
pub const AlterTableType_AT_DropIdentity: AlterTableType = 65;
pub const AlterTableType_AT_DropInherit: AlterTableType = 52;
pub const AlterTableType_AT_DropOf: AlterTableType = 54;
pub const AlterTableType_AT_DropOids: AlterTableType = 34;
pub const AlterTableType_AT_EnableAlwaysRule: AlterTableType = 48;
pub const AlterTableType_AT_EnableAlwaysTrig: AlterTableType = 40;
pub const AlterTableType_AT_EnableReplicaRule: AlterTableType = 49;
pub const AlterTableType_AT_EnableReplicaTrig: AlterTableType = 41;
pub const AlterTableType_AT_EnableRowSecurity: AlterTableType = 56;
pub const AlterTableType_AT_EnableRule: AlterTableType = 47;
pub const AlterTableType_AT_EnableTrig: AlterTableType = 39;
pub const AlterTableType_AT_EnableTrigAll: AlterTableType = 43;
pub const AlterTableType_AT_EnableTrigUser: AlterTableType = 45;
pub const AlterTableType_AT_ForceRowSecurity: AlterTableType = 58;
pub const AlterTableType_AT_GenericOptions: AlterTableType = 60;
pub const AlterTableType_AT_NoForceRowSecurity: AlterTableType = 59;
pub const AlterTableType_AT_ProcessedConstraint: AlterTableType = 20;
pub const AlterTableType_AT_ReAddComment: AlterTableType = 24;
pub const AlterTableType_AT_ReAddConstraint: AlterTableType = 16;
pub const AlterTableType_AT_ReAddIndex: AlterTableType = 13;
pub const AlterTableType_AT_ReplaceRelOptions: AlterTableType = 38;
pub const AlterTableType_AT_ReplicaIdentity: AlterTableType = 55;
pub const AlterTableType_AT_ResetOptions: AlterTableType = 8;
pub const AlterTableType_AT_ResetRelOptions: AlterTableType = 37;
pub const AlterTableType_AT_SetIdentity: AlterTableType = 64;
pub const AlterTableType_AT_SetLogged: AlterTableType = 30;
pub const AlterTableType_AT_SetOptions: AlterTableType = 7;
pub const AlterTableType_AT_SetRelOptions: AlterTableType = 36;
pub const AlterTableType_AT_SetStatistics: AlterTableType = 6;
pub const AlterTableType_AT_SetStorage: AlterTableType = 9;
pub const AlterTableType_AT_SetTableSpace: AlterTableType = 35;
pub const AlterTableType_AT_SetUnLogged: AlterTableType = 31;
pub const AlterTableType_AT_ValidateConstraint: AlterTableType = 18;
pub const AlterTableType_AT_ValidateConstraintRecurse: AlterTableType = 19;
pub const Anum_pg_attribute_attacl: u32 = 20;
pub const Anum_pg_attribute_attcollation: u32 = 19;
pub const Anum_pg_attribute_attfdwoptions: u32 = 22;
pub const Anum_pg_attribute_attidentity: u32 = 15;
pub const Anum_pg_attribute_attinhcount: u32 = 18;
pub const Anum_pg_attribute_attisdropped: u32 = 16;
pub const Anum_pg_attribute_attislocal: u32 = 17;
pub const Anum_pg_attribute_attoptions: u32 = 21;
pub const Anum_pg_class_relallvisible: u32 = 11;
pub const Anum_pg_class_relam: u32 = 6;
pub const Anum_pg_class_relchecks: u32 = 18;
pub const Anum_pg_class_relfilenode: u32 = 7;
pub const Anum_pg_class_relforcerowsecurity: u32 = 25;
pub const Anum_pg_class_relhasindex: u32 = 13;
pub const Anum_pg_class_relhasoids: u32 = 19;
pub const Anum_pg_class_relhaspkey: u32 = 20;
pub const Anum_pg_class_relhasrules: u32 = 21;
pub const Anum_pg_class_relhassubclass: u32 = 23;
pub const Anum_pg_class_relhastriggers: u32 = 22;
pub const Anum_pg_class_relispartition: u32 = 28;
pub const Anum_pg_class_relispopulated: u32 = 26;
pub const Anum_pg_class_relisshared: u32 = 14;
pub const Anum_pg_class_relkind: u32 = 16;
pub const Anum_pg_class_relname: u32 = 1;
pub const Anum_pg_class_relnamespace: u32 = 2;
pub const Anum_pg_class_relnatts: u32 = 17;
pub const Anum_pg_class_reloftype: u32 = 4;
pub const Anum_pg_class_relowner: u32 = 5;
pub const Anum_pg_class_relpages: u32 = 9;
pub const Anum_pg_class_relpersistence: u32 = 15;
pub const Anum_pg_class_relreplident: u32 = 27;
pub const Anum_pg_class_relrowsecurity: u32 = 24;
pub const Anum_pg_class_reltablespace: u32 = 8;
pub const Anum_pg_class_reltoastrelid: u32 = 12;
pub const Anum_pg_class_reltuples: u32 = 10;
pub const Anum_pg_class_reltype: u32 = 3;
pub const Anum_pg_enum_enumlabel: u32 = 3;
pub const Anum_pg_enum_enumsortorder: u32 = 2;
pub const Anum_pg_enum_enumtypid: u32 = 1;
pub const Anum_pg_event_trigger_evtenabled: u32 = 5;
pub const Anum_pg_event_trigger_evtevent: u32 = 2;
pub const Anum_pg_event_trigger_evtfoid: u32 = 4;
pub const Anum_pg_event_trigger_evtname: u32 = 1;
pub const Anum_pg_event_trigger_evtowner: u32 = 3;
pub const Anum_pg_event_trigger_evttags: u32 = 6;
pub const Anum_pg_index_indcheckxmin: u32 = 10;
pub const Anum_pg_index_indclass: u32 = 16;
pub const Anum_pg_index_indcollation: u32 = 15;
pub const Anum_pg_index_indexprs: u32 = 18;
pub const Anum_pg_index_indimmediate: u32 = 7;
pub const Anum_pg_index_indisclustered: u32 = 8;
pub const Anum_pg_index_indisexclusion: u32 = 6;
pub const Anum_pg_index_indislive: u32 = 12;
pub const Anum_pg_index_indisprimary: u32 = 5;
pub const Anum_pg_index_indisready: u32 = 11;
pub const Anum_pg_index_indisreplident: u32 = 13;
pub const Anum_pg_index_indisunique: u32 = 4;
pub const Anum_pg_index_indisvalid: u32 = 9;
pub const Anum_pg_index_indkey: u32 = 14;
pub const Anum_pg_index_indoption: u32 = 17;
pub const Anum_pg_index_indpred: u32 = 19;
pub const Anum_pg_publication_puballtables: u32 = 3;
pub const Anum_pg_publication_pubdelete: u32 = 6;
pub const Anum_pg_publication_pubinsert: u32 = 4;
pub const Anum_pg_publication_pubname: u32 = 1;
pub const Anum_pg_publication_pubowner: u32 = 2;
pub const Anum_pg_publication_pubupdate: u32 = 5;
pub const Anum_pg_trigger_tgargs: u32 = 14;
pub const Anum_pg_trigger_tgattr: u32 = 13;
pub const Anum_pg_trigger_tgconstraint: u32 = 9;
pub const Anum_pg_trigger_tgconstrindid: u32 = 8;
pub const Anum_pg_trigger_tgconstrrelid: u32 = 7;
pub const Anum_pg_trigger_tgdeferrable: u32 = 10;
pub const Anum_pg_trigger_tgenabled: u32 = 5;
pub const Anum_pg_trigger_tgfoid: u32 = 3;
pub const Anum_pg_trigger_tginitdeferred: u32 = 11;
pub const Anum_pg_trigger_tgisinternal: u32 = 6;
pub const Anum_pg_trigger_tgname: u32 = 2;
pub const Anum_pg_trigger_tgnargs: u32 = 12;
pub const Anum_pg_trigger_tgnewtable: u32 = 17;
pub const Anum_pg_trigger_tgoldtable: u32 = 16;
pub const Anum_pg_trigger_tgqual: u32 = 15;
pub const Anum_pg_trigger_tgrelid: u32 = 1;
pub const Anum_pg_trigger_tgtype: u32 = 4;
pub const Anum_pg_type_typacl: u32 = 30;
pub const Anum_pg_type_typalign: u32 = 21;
pub const Anum_pg_type_typanalyze: u32 = 20;
pub const Anum_pg_type_typarray: u32 = 13;
pub const Anum_pg_type_typbasetype: u32 = 24;
pub const Anum_pg_type_typbyval: u32 = 5;
pub const Anum_pg_type_typcategory: u32 = 7;
pub const Anum_pg_type_typcollation: u32 = 27;
pub const Anum_pg_type_typdefault: u32 = 29;
pub const Anum_pg_type_typdefaultbin: u32 = 28;
pub const Anum_pg_type_typdelim: u32 = 10;
pub const Anum_pg_type_typelem: u32 = 12;
pub const Anum_pg_type_typinput: u32 = 14;
pub const Anum_pg_type_typisdefined: u32 = 9;
pub const Anum_pg_type_typispreferred: u32 = 8;
pub const Anum_pg_type_typlen: u32 = 4;
pub const Anum_pg_type_typmodin: u32 = 18;
pub const Anum_pg_type_typmodout: u32 = 19;
pub const Anum_pg_type_typname: u32 = 1;
pub const Anum_pg_type_typnamespace: u32 = 2;
pub const Anum_pg_type_typndims: u32 = 26;
pub const Anum_pg_type_typnotnull: u32 = 23;
pub const Anum_pg_type_typoutput: u32 = 15;
pub const Anum_pg_type_typowner: u32 = 3;
pub const Anum_pg_type_typreceive: u32 = 16;
pub const Anum_pg_type_typrelid: u32 = 11;
pub const Anum_pg_type_typsend: u32 = 17;
pub const Anum_pg_type_typstorage: u32 = 22;
pub const Anum_pg_type_typtype: u32 = 6;
pub const Anum_pg_type_typtypmod: u32 = 25;
pub const BGW_MAXLEN: u32 = 64;
pub const BITS_PER_BITMAPWORD: u32 = 32;
pub const BUFFER_MAPPING_LWLOCK_OFFSET: u32 = 46;
pub const BuiltinTrancheIds_LWTRANCHE_ASYNC_BUFFERS: BuiltinTrancheIds = 51;
pub const BuiltinTrancheIds_LWTRANCHE_BUFFER_CONTENT: BuiltinTrancheIds = 54;
pub const BuiltinTrancheIds_LWTRANCHE_BUFFER_IO_IN_PROGRESS: BuiltinTrancheIds = 55;
pub const BuiltinTrancheIds_LWTRANCHE_BUFFER_MAPPING: BuiltinTrancheIds = 59;
pub const BuiltinTrancheIds_LWTRANCHE_CLOG_BUFFERS: BuiltinTrancheIds = 46;
pub const BuiltinTrancheIds_LWTRANCHE_COMMITTS_BUFFERS: BuiltinTrancheIds = 47;
pub const BuiltinTrancheIds_LWTRANCHE_FIRST_USER_DEFINED: BuiltinTrancheIds = 64;
pub const BuiltinTrancheIds_LWTRANCHE_LOCK_MANAGER: BuiltinTrancheIds = 60;
pub const BuiltinTrancheIds_LWTRANCHE_MXACTMEMBER_BUFFERS: BuiltinTrancheIds = 50;
pub const BuiltinTrancheIds_LWTRANCHE_MXACTOFFSET_BUFFERS: BuiltinTrancheIds = 49;
pub const BuiltinTrancheIds_LWTRANCHE_OLDSERXID_BUFFERS: BuiltinTrancheIds = 52;
pub const BuiltinTrancheIds_LWTRANCHE_PARALLEL_QUERY_DSA: BuiltinTrancheIds = 62;
pub const BuiltinTrancheIds_LWTRANCHE_PREDICATE_LOCK_MANAGER: BuiltinTrancheIds = 61;
pub const BuiltinTrancheIds_LWTRANCHE_PROC: BuiltinTrancheIds = 58;
pub const BuiltinTrancheIds_LWTRANCHE_REPLICATION_ORIGIN: BuiltinTrancheIds = 56;
pub const BuiltinTrancheIds_LWTRANCHE_REPLICATION_SLOT_IO_IN_PROGRESS: BuiltinTrancheIds = 57;
pub const BuiltinTrancheIds_LWTRANCHE_SUBTRANS_BUFFERS: BuiltinTrancheIds = 48;
pub const BuiltinTrancheIds_LWTRANCHE_TBM: BuiltinTrancheIds = 63;
pub const BuiltinTrancheIds_LWTRANCHE_WAL_INSERT: BuiltinTrancheIds = 53;
pub const CHECKPOINT_CAUSE_TIME: u32 = 128;
pub const CHECKPOINT_CAUSE_XLOG: u32 = 64;
pub const CHECKPOINT_REQUESTED: u32 = 256;
pub const ConstrType_CONSTR_ATTR_DEFERRABLE: ConstrType = 9;
pub const ConstrType_CONSTR_ATTR_DEFERRED: ConstrType = 11;
pub const ConstrType_CONSTR_ATTR_IMMEDIATE: ConstrType = 12;
pub const ConstrType_CONSTR_ATTR_NOT_DEFERRABLE: ConstrType = 10;
pub const ConstrType_CONSTR_CHECK: ConstrType = 4;
pub const ConstrType_CONSTR_EXCLUSION: ConstrType = 7;
pub const ConstrType_CONSTR_FOREIGN: ConstrType = 8;
pub const ConstrType_CONSTR_PRIMARY: ConstrType = 5;
pub const ConstrType_CONSTR_UNIQUE: ConstrType = 6;
pub const DEF_PGPORT: u32 = 28810;
pub const DEF_PGPORT_STR: &'static [u8; 6usize] = b"28810\0";
pub const DSM_IMPL_NONE: u32 = 0;
pub const DTK_CURRENT: u32 = 8;
pub const DTK_INVALID: u32 = 7;
pub const EXEC_FLAG_WITHOUT_OIDS: u32 = 64;
pub const EXEC_FLAG_WITH_NO_DATA: u32 = 128;
pub const EXEC_FLAG_WITH_OIDS: u32 = 32;
pub const FALSE: u32 = 0;
pub const FLOAT4PASSBYVAL: u32 = 1;
pub const FLOAT8PASSBYVAL: u32 = 1;
pub const FRAMEOPTION_BETWEEN: u32 = 8;
pub const FRAMEOPTION_DEFAULTS: u32 = 530;
pub const FRAMEOPTION_END_CURRENT_ROW: u32 = 512;
pub const FRAMEOPTION_END_UNBOUNDED_FOLLOWING: u32 = 128;
pub const FRAMEOPTION_END_UNBOUNDED_PRECEDING: u32 = 32;
pub const FRAMEOPTION_END_VALUE: u32 = 10240;
pub const FRAMEOPTION_END_VALUE_FOLLOWING: u32 = 8192;
pub const FRAMEOPTION_END_VALUE_PRECEDING: u32 = 2048;
pub const FRAMEOPTION_START_CURRENT_ROW: u32 = 256;
pub const FRAMEOPTION_START_UNBOUNDED_FOLLOWING: u32 = 64;
pub const FRAMEOPTION_START_UNBOUNDED_PRECEDING: u32 = 16;
pub const FRAMEOPTION_START_VALUE: u32 = 5120;
pub const FRAMEOPTION_START_VALUE_FOLLOWING: u32 = 4096;
pub const FRAMEOPTION_START_VALUE_PRECEDING: u32 = 1024;
pub const FirstBootstrapObjectId: u32 = 10000;
pub const FuncDetailCode_FUNCDETAIL_AGGREGATE: FuncDetailCode = 3;
pub const FuncDetailCode_FUNCDETAIL_COERCION: FuncDetailCode = 5;
pub const FuncDetailCode_FUNCDETAIL_WINDOWFUNC: FuncDetailCode = 4;
pub const GrantObjectType_ACL_OBJECT_COLUMN: GrantObjectType = 0;
pub const GrantObjectType_ACL_OBJECT_DATABASE: GrantObjectType = 3;
pub const GrantObjectType_ACL_OBJECT_DOMAIN: GrantObjectType = 4;
pub const GrantObjectType_ACL_OBJECT_FDW: GrantObjectType = 5;
pub const GrantObjectType_ACL_OBJECT_FOREIGN_SERVER: GrantObjectType = 6;
pub const GrantObjectType_ACL_OBJECT_FUNCTION: GrantObjectType = 7;
pub const GrantObjectType_ACL_OBJECT_LANGUAGE: GrantObjectType = 8;
pub const GrantObjectType_ACL_OBJECT_LARGEOBJECT: GrantObjectType = 9;
pub const GrantObjectType_ACL_OBJECT_NAMESPACE: GrantObjectType = 10;
pub const GrantObjectType_ACL_OBJECT_RELATION: GrantObjectType = 1;
pub const GrantObjectType_ACL_OBJECT_SEQUENCE: GrantObjectType = 2;
pub const GrantObjectType_ACL_OBJECT_TABLESPACE: GrantObjectType = 11;
pub const GrantObjectType_ACL_OBJECT_TYPE: GrantObjectType = 12;
pub const HAVE_DECL_SNPRINTF: u32 = 1;
pub const HAVE_DECL_SYS_SIGLIST: u32 = 1;
pub const HAVE_DECL_VSNPRINTF: u32 = 1;
pub const HAVE_SNPRINTF: u32 = 1;
pub const HAVE_STRERROR: u32 = 1;
pub const HAVE_STRONG_RANDOM: u32 = 1;
pub const HAVE_TOWLOWER: u32 = 1;
pub const HAVE_VSNPRINTF: u32 = 1;
pub const HAVE_WCSTOMBS: u32 = 1;
pub const HAVE__VA_ARGS: u32 = 1;
pub const HEAP_HASOID: u32 = 8;
pub const HEAP_INSERT_FROZEN: u32 = 4;
pub const HEAP_INSERT_NO_LOGICAL: u32 = 16;
pub const HEAP_INSERT_SKIP_FSM: u32 = 2;
pub const HEAP_INSERT_SKIP_WAL: u32 = 1;
pub const HEAP_INSERT_SPECULATIVE: u32 = 8;
pub const HTSU_Result_HeapTupleBeingUpdated: HTSU_Result = 4;
pub const HTSU_Result_HeapTupleInvisible: HTSU_Result = 1;
pub const HTSU_Result_HeapTupleMayBeUpdated: HTSU_Result = 0;
pub const HTSU_Result_HeapTupleSelfUpdated: HTSU_Result = 2;
pub const HTSU_Result_HeapTupleUpdated: HTSU_Result = 3;
pub const HTSU_Result_HeapTupleWouldBlock: HTSU_Result = 5;
pub const IndexAttrBitmapKind_INDEX_ATTR_BITMAP_ALL: IndexAttrBitmapKind = 0;
pub const IndexAttrBitmapKind_INDEX_ATTR_BITMAP_IDENTITY_KEY: IndexAttrBitmapKind = 3;
pub const IndexAttrBitmapKind_INDEX_ATTR_BITMAP_KEY: IndexAttrBitmapKind = 1;
pub const IndexAttrBitmapKind_INDEX_ATTR_BITMAP_PRIMARY_KEY: IndexAttrBitmapKind = 2;
pub const LOCK_MANAGER_LWLOCK_OFFSET: u32 = 174;
pub const NUM_FIXED_LWLOCKS: u32 = 206;
pub const NUM_INDIVIDUAL_LWLOCKS: u32 = 46;
pub const Natts_pg_attribute: u32 = 22;
pub const Natts_pg_enum: u32 = 3;
pub const Natts_pg_event_trigger: u32 = 6;
pub const Natts_pg_index: u32 = 19;
pub const Natts_pg_publication: u32 = 6;
pub const Natts_pg_trigger: u32 = 17;
pub const Natts_pg_type: u32 = 30;
pub const NodeTag_T_A_ArrayExpr: NodeTag = 342;
pub const NodeTag_T_A_Const: NodeTag = 337;
pub const NodeTag_T_A_Expr: NodeTag = 334;
pub const NodeTag_T_A_Indices: NodeTag = 340;
pub const NodeTag_T_A_Indirection: NodeTag = 341;
pub const NodeTag_T_A_Star: NodeTag = 339;
pub const NodeTag_T_AccessPriv: NodeTag = 367;
pub const NodeTag_T_Agg: NodeTag = 41;
pub const NodeTag_T_AggPath: NodeTag = 184;
pub const NodeTag_T_AggState: NodeTag = 86;
pub const NodeTag_T_Aggref: NodeTag = 102;
pub const NodeTag_T_AggrefExprState: NodeTag = 148;
pub const NodeTag_T_Alias: NodeTag = 95;
pub const NodeTag_T_AllocSetContext: NodeTag = 209;
pub const NodeTag_T_AlterCollationStmt: NodeTag = 333;
pub const NodeTag_T_AlterDatabaseSetStmt: NodeTag = 278;
pub const NodeTag_T_AlterDatabaseStmt: NodeTag = 277;
pub const NodeTag_T_AlterDefaultPrivilegesStmt: NodeTag = 234;
pub const NodeTag_T_AlterDomainStmt: NodeTag = 230;
pub const NodeTag_T_AlterEnumStmt: NodeTag = 300;
pub const NodeTag_T_AlterEventTrigStmt: NodeTag = 319;
pub const NodeTag_T_AlterExtensionContentsStmt: NodeTag = 317;
pub const NodeTag_T_AlterExtensionStmt: NodeTag = 316;
pub const NodeTag_T_AlterFdwStmt: NodeTag = 304;
pub const NodeTag_T_AlterForeignServerStmt: NodeTag = 306;
pub const NodeTag_T_AlterFunctionStmt: NodeTag = 246;
pub const NodeTag_T_AlterObjectDependsStmt: NodeTag = 291;
pub const NodeTag_T_AlterObjectSchemaStmt: NodeTag = 292;
pub const NodeTag_T_AlterOpFamilyStmt: NodeTag = 284;
pub const NodeTag_T_AlterOperatorStmt: NodeTag = 294;
pub const NodeTag_T_AlterOwnerStmt: NodeTag = 293;
pub const NodeTag_T_AlterPolicyStmt: NodeTag = 324;
pub const NodeTag_T_AlterPublicationStmt: NodeTag = 328;
pub const NodeTag_T_AlterRoleSetStmt: NodeTag = 279;
pub const NodeTag_T_AlterRoleStmt: NodeTag = 270;
pub const NodeTag_T_AlterSeqStmt: NodeTag = 263;
pub const NodeTag_T_AlterSubscriptionStmt: NodeTag = 330;
pub const NodeTag_T_AlterSystemStmt: NodeTag = 322;
pub const NodeTag_T_AlterTSConfigurationStmt: NodeTag = 302;
pub const NodeTag_T_AlterTSDictionaryStmt: NodeTag = 301;
pub const NodeTag_T_AlterTableCmd: NodeTag = 229;
pub const NodeTag_T_AlterTableMoveAllStmt: NodeTag = 311;
pub const NodeTag_T_AlterTableSpaceOptionsStmt: NodeTag = 310;
pub const NodeTag_T_AlterTableStmt: NodeTag = 228;
pub const NodeTag_T_AlterUserMappingStmt: NodeTag = 308;
pub const NodeTag_T_AlternativeSubPlan: NodeTag = 115;
pub const NodeTag_T_AlternativeSubPlanState: NodeTag = 152;
pub const NodeTag_T_Append: NodeTag = 12;
pub const NodeTag_T_AppendPath: NodeTag = 172;
pub const NodeTag_T_AppendRelInfo: NodeTag = 200;
pub const NodeTag_T_AppendState: NodeTag = 57;
pub const NodeTag_T_ArrayCoerceExpr: NodeTag = 120;
pub const NodeTag_T_ArrayExpr: NodeTag = 126;
pub const NodeTag_T_ArrayRef: NodeTag = 105;
pub const NodeTag_T_BaseBackupCmd: NodeTag = 386;
pub const NodeTag_T_BitString: NodeTag = 215;
pub const NodeTag_T_BitmapAnd: NodeTag = 15;
pub const NodeTag_T_BitmapAndPath: NodeTag = 163;
pub const NodeTag_T_BitmapAndState: NodeTag = 60;
pub const NodeTag_T_BitmapHeapPath: NodeTag = 162;
pub const NodeTag_T_BitmapHeapScan: NodeTag = 23;
pub const NodeTag_T_BitmapHeapScanState: NodeTag = 68;
pub const NodeTag_T_BitmapIndexScan: NodeTag = 22;
pub const NodeTag_T_BitmapIndexScanState: NodeTag = 67;
pub const NodeTag_T_BitmapOr: NodeTag = 16;
pub const NodeTag_T_BitmapOrPath: NodeTag = 164;
pub const NodeTag_T_BitmapOrState: NodeTag = 61;
pub const NodeTag_T_BoolExpr: NodeTag = 112;
pub const NodeTag_T_BooleanTest: NodeTag = 134;
pub const NodeTag_T_CaseExpr: NodeTag = 123;
pub const NodeTag_T_CaseTestExpr: NodeTag = 125;
pub const NodeTag_T_CaseWhen: NodeTag = 124;
pub const NodeTag_T_CheckPointStmt: NodeTag = 275;
pub const NodeTag_T_ClosePortalStmt: NodeTag = 235;
pub const NodeTag_T_ClusterStmt: NodeTag = 236;
pub const NodeTag_T_CoalesceExpr: NodeTag = 129;
pub const NodeTag_T_CoerceToDomain: NodeTag = 135;
pub const NodeTag_T_CoerceToDomainValue: NodeTag = 136;
pub const NodeTag_T_CoerceViaIO: NodeTag = 119;
pub const NodeTag_T_CollateClause: NodeTag = 346;
pub const NodeTag_T_CollateExpr: NodeTag = 122;
pub const NodeTag_T_ColumnDef: NodeTag = 355;
pub const NodeTag_T_ColumnRef: NodeTag = 335;
pub const NodeTag_T_CommentStmt: NodeTag = 242;
pub const NodeTag_T_CommonTableExpr: NodeTag = 377;
pub const NodeTag_T_CompositeTypeStmt: NodeTag = 297;
pub const NodeTag_T_Const: NodeTag = 100;
pub const NodeTag_T_Constraint: NodeTag = 357;
pub const NodeTag_T_ConstraintsSetStmt: NodeTag = 273;
pub const NodeTag_T_ConvertRowtypeExpr: NodeTag = 121;
pub const NodeTag_T_CopyStmt: NodeTag = 237;
pub const NodeTag_T_CreateAmStmt: NodeTag = 326;
pub const NodeTag_T_CreateCastStmt: NodeTag = 281;
pub const NodeTag_T_CreateConversionStmt: NodeTag = 280;
pub const NodeTag_T_CreateDomainStmt: NodeTag = 256;
pub const NodeTag_T_CreateEnumStmt: NodeTag = 298;
pub const NodeTag_T_CreateEventTrigStmt: NodeTag = 318;
pub const NodeTag_T_CreateExtensionStmt: NodeTag = 315;
pub const NodeTag_T_CreateFdwStmt: NodeTag = 303;
pub const NodeTag_T_CreateForeignServerStmt: NodeTag = 305;
pub const NodeTag_T_CreateForeignTableStmt: NodeTag = 313;
pub const NodeTag_T_CreateFunctionStmt: NodeTag = 245;
pub const NodeTag_T_CreateOpClassItem: NodeTag = 368;
pub const NodeTag_T_CreateOpClassStmt: NodeTag = 282;
pub const NodeTag_T_CreateOpFamilyStmt: NodeTag = 283;
pub const NodeTag_T_CreatePLangStmt: NodeTag = 268;
pub const NodeTag_T_CreatePolicyStmt: NodeTag = 323;
pub const NodeTag_T_CreatePublicationStmt: NodeTag = 327;
pub const NodeTag_T_CreateRangeStmt: NodeTag = 299;
pub const NodeTag_T_CreateReplicationSlotCmd: NodeTag = 387;
pub const NodeTag_T_CreateRoleStmt: NodeTag = 269;
pub const NodeTag_T_CreateSchemaStmt: NodeTag = 276;
pub const NodeTag_T_CreateSeqStmt: NodeTag = 262;
pub const NodeTag_T_CreateStatsStmt: NodeTag = 332;
pub const NodeTag_T_CreateStmt: NodeTag = 238;
pub const NodeTag_T_CreateSubscriptionStmt: NodeTag = 329;
pub const NodeTag_T_CreateTableAsStmt: NodeTag = 261;
pub const NodeTag_T_CreateTableSpaceStmt: NodeTag = 289;
pub const NodeTag_T_CreateTransformStmt: NodeTag = 325;
pub const NodeTag_T_CreateTrigStmt: NodeTag = 267;
pub const NodeTag_T_CreateUserMappingStmt: NodeTag = 307;
pub const NodeTag_T_CreatedbStmt: NodeTag = 257;
pub const NodeTag_T_CteScan: NodeTag = 29;
pub const NodeTag_T_CteScanState: NodeTag = 74;
pub const NodeTag_T_CurrentOfExpr: NodeTag = 138;
pub const NodeTag_T_CustomPath: NodeTag = 168;
pub const NodeTag_T_CustomScan: NodeTag = 33;
pub const NodeTag_T_CustomScanState: NodeTag = 78;
pub const NodeTag_T_DeallocateStmt: NodeTag = 287;
pub const NodeTag_T_DeclareCursorStmt: NodeTag = 288;
pub const NodeTag_T_DefElem: NodeTag = 358;
pub const NodeTag_T_DefineStmt: NodeTag = 239;
pub const NodeTag_T_DeleteStmt: NodeTag = 225;
pub const NodeTag_T_DiscardStmt: NodeTag = 266;
pub const NodeTag_T_DistinctExpr: NodeTag = 109;
pub const NodeTag_T_DoStmt: NodeTag = 247;
pub const NodeTag_T_DomainConstraintState: NodeTag = 153;
pub const NodeTag_T_DropOwnedStmt: NodeTag = 295;
pub const NodeTag_T_DropReplicationSlotCmd: NodeTag = 388;
pub const NodeTag_T_DropRoleStmt: NodeTag = 271;
pub const NodeTag_T_DropStmt: NodeTag = 240;
pub const NodeTag_T_DropSubscriptionStmt: NodeTag = 331;
pub const NodeTag_T_DropTableSpaceStmt: NodeTag = 290;
pub const NodeTag_T_DropUserMappingStmt: NodeTag = 309;
pub const NodeTag_T_DropdbStmt: NodeTag = 258;
pub const NodeTag_T_EState: NodeTag = 6;
pub const NodeTag_T_EquivalenceClass: NodeTag = 193;
pub const NodeTag_T_EquivalenceMember: NodeTag = 194;
pub const NodeTag_T_EventTriggerData: NodeTag = 393;
pub const NodeTag_T_ExecuteStmt: NodeTag = 286;
pub const NodeTag_T_ExplainStmt: NodeTag = 260;
pub const NodeTag_T_Expr: NodeTag = 98;
pub const NodeTag_T_ExprState: NodeTag = 147;
pub const NodeTag_T_ExtensibleNode: NodeTag = 220;
pub const NodeTag_T_FdwRoutine: NodeTag = 398;
pub const NodeTag_T_FetchStmt: NodeTag = 243;
pub const NodeTag_T_FieldSelect: NodeTag = 116;
pub const NodeTag_T_FieldStore: NodeTag = 117;
pub const NodeTag_T_Float: NodeTag = 213;
pub const NodeTag_T_ForeignKeyCacheInfo: NodeTag = 401;
pub const NodeTag_T_ForeignKeyOptInfo: NodeTag = 158;
pub const NodeTag_T_ForeignPath: NodeTag = 167;
pub const NodeTag_T_ForeignScan: NodeTag = 32;
pub const NodeTag_T_ForeignScanState: NodeTag = 77;
pub const NodeTag_T_FromExpr: NodeTag = 144;
pub const NodeTag_T_FuncCall: NodeTag = 338;
pub const NodeTag_T_FuncExpr: NodeTag = 106;
pub const NodeTag_T_FunctionParameter: NodeTag = 370;
pub const NodeTag_T_FunctionScan: NodeTag = 26;
pub const NodeTag_T_FunctionScanState: NodeTag = 71;
pub const NodeTag_T_Gather: NodeTag = 44;
pub const NodeTag_T_GatherMerge: NodeTag = 45;
pub const NodeTag_T_GatherMergePath: NodeTag = 178;
pub const NodeTag_T_GatherMergeState: NodeTag = 90;
pub const NodeTag_T_GatherPath: NodeTag = 177;
pub const NodeTag_T_GatherState: NodeTag = 89;
pub const NodeTag_T_GrantRoleStmt: NodeTag = 233;
pub const NodeTag_T_GrantStmt: NodeTag = 232;
pub const NodeTag_T_Group: NodeTag = 40;
pub const NodeTag_T_GroupPath: NodeTag = 182;
pub const NodeTag_T_GroupState: NodeTag = 85;
pub const NodeTag_T_GroupingFunc: NodeTag = 103;
pub const NodeTag_T_GroupingSet: NodeTag = 364;
pub const NodeTag_T_GroupingSetData: NodeTag = 206;
pub const NodeTag_T_GroupingSetsPath: NodeTag = 185;
pub const NodeTag_T_Hash: NodeTag = 46;
pub const NodeTag_T_HashJoin: NodeTag = 37;
pub const NodeTag_T_HashJoinState: NodeTag = 82;
pub const NodeTag_T_HashPath: NodeTag = 171;
pub const NodeTag_T_HashState: NodeTag = 91;
pub const NodeTag_T_IdentifySystemCmd: NodeTag = 385;
pub const NodeTag_T_ImportForeignSchemaStmt: NodeTag = 314;
pub const NodeTag_T_IndexAmRoutine: NodeTag = 399;
pub const NodeTag_T_IndexElem: NodeTag = 356;
pub const NodeTag_T_IndexOnlyScan: NodeTag = 21;
pub const NodeTag_T_IndexOnlyScanState: NodeTag = 66;
pub const NodeTag_T_IndexOptInfo: NodeTag = 157;
pub const NodeTag_T_IndexPath: NodeTag = 161;
pub const NodeTag_T_IndexScan: NodeTag = 20;
pub const NodeTag_T_IndexScanState: NodeTag = 65;
pub const NodeTag_T_IndexStmt: NodeTag = 244;
pub const NodeTag_T_InferClause: NodeTag = 375;
pub const NodeTag_T_InferenceElem: NodeTag = 140;
pub const NodeTag_T_InlineCodeBlock: NodeTag = 397;
pub const NodeTag_T_InsertStmt: NodeTag = 224;
pub const NodeTag_T_IntList: NodeTag = 218;
pub const NodeTag_T_Integer: NodeTag = 212;
pub const NodeTag_T_IntoClause: NodeTag = 146;
pub const NodeTag_T_Join: NodeTag = 34;
pub const NodeTag_T_JoinExpr: NodeTag = 143;
pub const NodeTag_T_JoinState: NodeTag = 79;
pub const NodeTag_T_Limit: NodeTag = 49;
pub const NodeTag_T_LimitPath: NodeTag = 192;
pub const NodeTag_T_LimitState: NodeTag = 94;
pub const NodeTag_T_List: NodeTag = 217;
pub const NodeTag_T_ListenStmt: NodeTag = 251;
pub const NodeTag_T_LoadStmt: NodeTag = 255;
pub const NodeTag_T_LockRows: NodeTag = 48;
pub const NodeTag_T_LockRowsPath: NodeTag = 190;
pub const NodeTag_T_LockRowsState: NodeTag = 93;
pub const NodeTag_T_LockStmt: NodeTag = 272;
pub const NodeTag_T_LockingClause: NodeTag = 371;
pub const NodeTag_T_Material: NodeTag = 38;
pub const NodeTag_T_MaterialPath: NodeTag = 175;
pub const NodeTag_T_MaterialState: NodeTag = 83;
pub const NodeTag_T_MemoryContext: NodeTag = 208;
pub const NodeTag_T_MergeAppend: NodeTag = 13;
pub const NodeTag_T_MergeAppendPath: NodeTag = 173;
pub const NodeTag_T_MergeAppendState: NodeTag = 58;
pub const NodeTag_T_MergeJoin: NodeTag = 36;
pub const NodeTag_T_MergeJoinState: NodeTag = 81;
pub const NodeTag_T_MergePath: NodeTag = 170;
pub const NodeTag_T_MinMaxAggInfo: NodeTag = 203;
pub const NodeTag_T_MinMaxAggPath: NodeTag = 186;
pub const NodeTag_T_MinMaxExpr: NodeTag = 130;
pub const NodeTag_T_ModifyTable: NodeTag = 11;
pub const NodeTag_T_ModifyTablePath: NodeTag = 191;
pub const NodeTag_T_ModifyTableState: NodeTag = 56;
pub const NodeTag_T_MultiAssignRef: NodeTag = 344;
pub const NodeTag_T_NamedArgExpr: NodeTag = 107;
pub const NodeTag_T_NamedTuplestoreScan: NodeTag = 30;
pub const NodeTag_T_NamedTuplestoreScanState: NodeTag = 75;
pub const NodeTag_T_NestLoop: NodeTag = 35;
pub const NodeTag_T_NestLoopParam: NodeTag = 50;
pub const NodeTag_T_NestLoopState: NodeTag = 80;
pub const NodeTag_T_NestPath: NodeTag = 169;
pub const NodeTag_T_NextValueExpr: NodeTag = 139;
pub const NodeTag_T_NotifyStmt: NodeTag = 250;
pub const NodeTag_T_Null: NodeTag = 216;
pub const NodeTag_T_NullIfExpr: NodeTag = 110;
pub const NodeTag_T_NullTest: NodeTag = 133;
pub const NodeTag_T_ObjectWithArgs: NodeTag = 366;
pub const NodeTag_T_OidList: NodeTag = 219;
pub const NodeTag_T_OnConflictClause: NodeTag = 376;
pub const NodeTag_T_OnConflictExpr: NodeTag = 145;
pub const NodeTag_T_OpExpr: NodeTag = 108;
pub const NodeTag_T_Param: NodeTag = 101;
pub const NodeTag_T_ParamPathInfo: NodeTag = 159;
pub const NodeTag_T_ParamRef: NodeTag = 336;
pub const NodeTag_T_PartitionBoundSpec: NodeTag = 382;
pub const NodeTag_T_PartitionCmd: NodeTag = 384;
pub const NodeTag_T_PartitionElem: NodeTag = 380;
pub const NodeTag_T_PartitionRangeDatum: NodeTag = 383;
pub const NodeTag_T_PartitionSpec: NodeTag = 381;
pub const NodeTag_T_PartitionedChildRelInfo: NodeTag = 201;
pub const NodeTag_T_Path: NodeTag = 160;
pub const NodeTag_T_PathKey: NodeTag = 195;
pub const NodeTag_T_PathTarget: NodeTag = 196;
pub const NodeTag_T_PlaceHolderInfo: NodeTag = 202;
pub const NodeTag_T_PlaceHolderVar: NodeTag = 198;
pub const NodeTag_T_Plan: NodeTag = 8;
pub const NodeTag_T_PlanInvalItem: NodeTag = 52;
pub const NodeTag_T_PlanRowMark: NodeTag = 51;
pub const NodeTag_T_PlanState: NodeTag = 53;
pub const NodeTag_T_PlannedStmt: NodeTag = 223;
pub const NodeTag_T_PlannerGlobal: NodeTag = 155;
pub const NodeTag_T_PlannerInfo: NodeTag = 154;
pub const NodeTag_T_PlannerParamItem: NodeTag = 204;
pub const NodeTag_T_PrepareStmt: NodeTag = 285;
pub const NodeTag_T_ProjectSet: NodeTag = 10;
pub const NodeTag_T_ProjectSetPath: NodeTag = 180;
pub const NodeTag_T_ProjectSetState: NodeTag = 55;
pub const NodeTag_T_ProjectionPath: NodeTag = 179;
pub const NodeTag_T_Query: NodeTag = 222;
pub const NodeTag_T_RangeFunction: NodeTag = 350;
pub const NodeTag_T_RangeSubselect: NodeTag = 349;
pub const NodeTag_T_RangeTableFunc: NodeTag = 352;
pub const NodeTag_T_RangeTableFuncCol: NodeTag = 353;
pub const NodeTag_T_RangeTableSample: NodeTag = 351;
pub const NodeTag_T_RangeTblEntry: NodeTag = 359;
pub const NodeTag_T_RangeTblFunction: NodeTag = 360;
pub const NodeTag_T_RangeTblRef: NodeTag = 142;
pub const NodeTag_T_RangeVar: NodeTag = 96;
pub const NodeTag_T_RawStmt: NodeTag = 221;
pub const NodeTag_T_ReassignOwnedStmt: NodeTag = 296;
pub const NodeTag_T_RecursiveUnion: NodeTag = 14;
pub const NodeTag_T_RecursiveUnionPath: NodeTag = 189;
pub const NodeTag_T_RecursiveUnionState: NodeTag = 59;
pub const NodeTag_T_RefreshMatViewStmt: NodeTag = 320;
pub const NodeTag_T_ReindexStmt: NodeTag = 274;
pub const NodeTag_T_RelOptInfo: NodeTag = 156;
pub const NodeTag_T_RelabelType: NodeTag = 118;
pub const NodeTag_T_RenameStmt: NodeTag = 248;
pub const NodeTag_T_ReplicaIdentityStmt: NodeTag = 321;
pub const NodeTag_T_ResTarget: NodeTag = 343;
pub const NodeTag_T_RestrictInfo: NodeTag = 197;
pub const NodeTag_T_Result: NodeTag = 9;
pub const NodeTag_T_ResultPath: NodeTag = 174;
pub const NodeTag_T_ResultRelInfo: NodeTag = 5;
pub const NodeTag_T_ResultState: NodeTag = 54;
pub const NodeTag_T_ReturnSetInfo: NodeTag = 394;
pub const NodeTag_T_RoleSpec: NodeTag = 378;
pub const NodeTag_T_RollupData: NodeTag = 205;
pub const NodeTag_T_RowCompareExpr: NodeTag = 128;
pub const NodeTag_T_RowExpr: NodeTag = 127;
pub const NodeTag_T_RowMarkClause: NodeTag = 372;
pub const NodeTag_T_RuleStmt: NodeTag = 249;
pub const NodeTag_T_SQLCmd: NodeTag = 391;
pub const NodeTag_T_SQLValueFunction: NodeTag = 131;
pub const NodeTag_T_SampleScan: NodeTag = 19;
pub const NodeTag_T_SampleScanState: NodeTag = 64;
pub const NodeTag_T_ScalarArrayOpExpr: NodeTag = 111;
pub const NodeTag_T_Scan: NodeTag = 17;
pub const NodeTag_T_ScanState: NodeTag = 62;
pub const NodeTag_T_SecLabelStmt: NodeTag = 312;
pub const NodeTag_T_SelectStmt: NodeTag = 227;
pub const NodeTag_T_SeqScan: NodeTag = 18;
pub const NodeTag_T_SeqScanState: NodeTag = 63;
pub const NodeTag_T_SetExprState: NodeTag = 150;
pub const NodeTag_T_SetOp: NodeTag = 47;
pub const NodeTag_T_SetOpPath: NodeTag = 188;
pub const NodeTag_T_SetOpState: NodeTag = 92;
pub const NodeTag_T_SetOperationStmt: NodeTag = 231;
pub const NodeTag_T_SetToDefault: NodeTag = 137;
pub const NodeTag_T_SlabContext: NodeTag = 210;
pub const NodeTag_T_Sort: NodeTag = 39;
pub const NodeTag_T_SortBy: NodeTag = 347;
pub const NodeTag_T_SortGroupClause: NodeTag = 363;
pub const NodeTag_T_SortPath: NodeTag = 181;
pub const NodeTag_T_SortState: NodeTag = 84;
pub const NodeTag_T_SpecialJoinInfo: NodeTag = 199;
pub const NodeTag_T_StartReplicationCmd: NodeTag = 389;
pub const NodeTag_T_StatisticExtInfo: NodeTag = 207;
pub const NodeTag_T_String: NodeTag = 214;
pub const NodeTag_T_SubLink: NodeTag = 113;
pub const NodeTag_T_SubPlan: NodeTag = 114;
pub const NodeTag_T_SubPlanState: NodeTag = 151;
pub const NodeTag_T_SubqueryScan: NodeTag = 25;
pub const NodeTag_T_SubqueryScanPath: NodeTag = 166;
pub const NodeTag_T_SubqueryScanState: NodeTag = 70;
pub const NodeTag_T_TIDBitmap: NodeTag = 396;
pub const NodeTag_T_TableFunc: NodeTag = 97;
pub const NodeTag_T_TableFuncScan: NodeTag = 28;
pub const NodeTag_T_TableFuncScanState: NodeTag = 72;
pub const NodeTag_T_TableLikeClause: NodeTag = 369;
pub const NodeTag_T_TableSampleClause: NodeTag = 361;
pub const NodeTag_T_TargetEntry: NodeTag = 141;
pub const NodeTag_T_TidPath: NodeTag = 165;
pub const NodeTag_T_TidScan: NodeTag = 24;
pub const NodeTag_T_TidScanState: NodeTag = 69;
pub const NodeTag_T_TimeLineHistoryCmd: NodeTag = 390;
pub const NodeTag_T_TransactionStmt: NodeTag = 253;
pub const NodeTag_T_TriggerData: NodeTag = 392;
pub const NodeTag_T_TriggerTransition: NodeTag = 379;
pub const NodeTag_T_TruncateStmt: NodeTag = 241;
pub const NodeTag_T_TsmRoutine: NodeTag = 400;
pub const NodeTag_T_TupleTableSlot: NodeTag = 7;
pub const NodeTag_T_TypeCast: NodeTag = 345;
pub const NodeTag_T_TypeName: NodeTag = 354;
pub const NodeTag_T_Unique: NodeTag = 43;
pub const NodeTag_T_UniquePath: NodeTag = 176;
pub const NodeTag_T_UniqueState: NodeTag = 88;
pub const NodeTag_T_UnlistenStmt: NodeTag = 252;
pub const NodeTag_T_UpdateStmt: NodeTag = 226;
pub const NodeTag_T_UpperUniquePath: NodeTag = 183;
pub const NodeTag_T_VacuumStmt: NodeTag = 259;
pub const NodeTag_T_Value: NodeTag = 211;
pub const NodeTag_T_ValuesScan: NodeTag = 27;
pub const NodeTag_T_ValuesScanState: NodeTag = 73;
pub const NodeTag_T_Var: NodeTag = 99;
pub const NodeTag_T_VariableSetStmt: NodeTag = 264;
pub const NodeTag_T_VariableShowStmt: NodeTag = 265;
pub const NodeTag_T_ViewStmt: NodeTag = 254;
pub const NodeTag_T_WindowAgg: NodeTag = 42;
pub const NodeTag_T_WindowAggPath: NodeTag = 187;
pub const NodeTag_T_WindowAggState: NodeTag = 87;
pub const NodeTag_T_WindowClause: NodeTag = 365;
pub const NodeTag_T_WindowDef: NodeTag = 348;
pub const NodeTag_T_WindowFunc: NodeTag = 104;
pub const NodeTag_T_WindowFuncExprState: NodeTag = 149;
pub const NodeTag_T_WindowObjectData: NodeTag = 395;
pub const NodeTag_T_WithCheckOption: NodeTag = 362;
pub const NodeTag_T_WithClause: NodeTag = 374;
pub const NodeTag_T_WorkTableScan: NodeTag = 31;
pub const NodeTag_T_WorkTableScanState: NodeTag = 76;
pub const NodeTag_T_XmlExpr: NodeTag = 132;
pub const NodeTag_T_XmlSerialize: NodeTag = 373;
pub const ObjectType_OBJECT_PUBLICATION: ObjectType = 28;
pub const ObjectType_OBJECT_PUBLICATION_REL: ObjectType = 29;
pub const ObjectType_OBJECT_ROLE: ObjectType = 30;
pub const ObjectType_OBJECT_RULE: ObjectType = 31;
pub const ObjectType_OBJECT_SCHEMA: ObjectType = 32;
pub const ObjectType_OBJECT_SEQUENCE: ObjectType = 33;
pub const ObjectType_OBJECT_STATISTIC_EXT: ObjectType = 35;
pub const ObjectType_OBJECT_SUBSCRIPTION: ObjectType = 34;
pub const ObjectType_OBJECT_TABCONSTRAINT: ObjectType = 36;
pub const ObjectType_OBJECT_TABLE: ObjectType = 37;
pub const ObjectType_OBJECT_TABLESPACE: ObjectType = 38;
pub const ObjectType_OBJECT_TRANSFORM: ObjectType = 39;
pub const ObjectType_OBJECT_TRIGGER: ObjectType = 40;
pub const ObjectType_OBJECT_TSCONFIGURATION: ObjectType = 41;
pub const ObjectType_OBJECT_TSDICTIONARY: ObjectType = 42;
pub const ObjectType_OBJECT_TSPARSER: ObjectType = 43;
pub const ObjectType_OBJECT_TSTEMPLATE: ObjectType = 44;
pub const ObjectType_OBJECT_TYPE: ObjectType = 45;
pub const ObjectType_OBJECT_USER_MAPPING: ObjectType = 46;
pub const ObjectType_OBJECT_VIEW: ObjectType = 47;
pub const PACKAGE_BUGREPORT: &'static [u8; 26usize] = b"pgsql-bugs@postgresql.org\0";
pub const PACKAGE_STRING: &'static [u8; 17usize] = b"PostgreSQL 10.13\0";
pub const PACKAGE_VERSION: &'static [u8; 6usize] = b"10.13\0";
pub const PGSTAT_NUM_PROGRESS_PARAM: u32 = 10;
pub const PG_BACKEND_VERSIONSTR: &'static [u8; 29usize] = b"postgres (PostgreSQL) 10.13\n\0";
pub const PG_MAJORVERSION: &'static [u8; 3usize] = b"10\0";
pub const PG_VERSION: &'static [u8; 6usize] = b"10.13\0";
pub const PG_VERSION_NUM: u32 = 100013;
pub const PG_VERSION_STR : & 'static [ u8 ; 115usize ] = b"PostgreSQL 10.13 on x86_64-apple-darwin19.0.0, compiled by Apple clang version 11.0.0 (clang-1100.0.33.12), 64-bit\0" ;
pub const PREDICATELOCK_MANAGER_LWLOCK_OFFSET: u32 = 190;
pub const ParseExprKind_EXPR_KIND_ALTER_COL_TRANSFORM: ParseExprKind = 31;
pub const ParseExprKind_EXPR_KIND_CHECK_CONSTRAINT: ParseExprKind = 25;
pub const ParseExprKind_EXPR_KIND_COLUMN_DEFAULT: ParseExprKind = 27;
pub const ParseExprKind_EXPR_KIND_DISTINCT_ON: ParseExprKind = 19;
pub const ParseExprKind_EXPR_KIND_DOMAIN_CHECK: ParseExprKind = 26;
pub const ParseExprKind_EXPR_KIND_EXECUTE_PARAMETER: ParseExprKind = 32;
pub const ParseExprKind_EXPR_KIND_FUNCTION_DEFAULT: ParseExprKind = 28;
pub const ParseExprKind_EXPR_KIND_GROUP_BY: ParseExprKind = 17;
pub const ParseExprKind_EXPR_KIND_INDEX_EXPRESSION: ParseExprKind = 29;
pub const ParseExprKind_EXPR_KIND_INDEX_PREDICATE: ParseExprKind = 30;
pub const ParseExprKind_EXPR_KIND_INSERT_TARGET: ParseExprKind = 14;
pub const ParseExprKind_EXPR_KIND_LIMIT: ParseExprKind = 20;
pub const ParseExprKind_EXPR_KIND_OFFSET: ParseExprKind = 21;
pub const ParseExprKind_EXPR_KIND_ORDER_BY: ParseExprKind = 18;
pub const ParseExprKind_EXPR_KIND_PARTITION_EXPRESSION: ParseExprKind = 35;
pub const ParseExprKind_EXPR_KIND_POLICY: ParseExprKind = 34;
pub const ParseExprKind_EXPR_KIND_RETURNING: ParseExprKind = 22;
pub const ParseExprKind_EXPR_KIND_SELECT_TARGET: ParseExprKind = 13;
pub const ParseExprKind_EXPR_KIND_TRIGGER_WHEN: ParseExprKind = 33;
pub const ParseExprKind_EXPR_KIND_UPDATE_SOURCE: ParseExprKind = 15;
pub const ParseExprKind_EXPR_KIND_UPDATE_TARGET: ParseExprKind = 16;
pub const ParseExprKind_EXPR_KIND_VALUES: ParseExprKind = 23;
pub const ParseExprKind_EXPR_KIND_VALUES_SINGLE: ParseExprKind = 24;
pub const Pattern_Prefix_Status_Pattern_Prefix_Exact: Pattern_Prefix_Status = 2;
pub const Pattern_Prefix_Status_Pattern_Prefix_None: Pattern_Prefix_Status = 0;
pub const Pattern_Prefix_Status_Pattern_Prefix_Partial: Pattern_Prefix_Status = 1;
pub const Pattern_Type_Pattern_Type_Like: Pattern_Type = 0;
pub const Pattern_Type_Pattern_Type_Like_IC: Pattern_Type = 1;
pub const Pattern_Type_Pattern_Type_Regex: Pattern_Type = 2;
pub const Pattern_Type_Pattern_Type_Regex_IC: Pattern_Type = 3;
pub const ProcessUtilityContext_PROCESS_UTILITY_SUBCOMMAND: ProcessUtilityContext = 2;
pub const QTW_DONT_COPY_QUERY: u32 = 32;
pub const QTW_EXAMINE_RTES: u32 = 16;
pub const RELTIMEOID: u32 = 703;
pub const RTMaxStrategyNumber: u32 = 27;
pub const RelOptKind_RELOPT_DEADREL: RelOptKind = 4;
pub const RelOptKind_RELOPT_UPPER_REL: RelOptKind = 3;
pub const SysCacheIdentifier_STATEXTNAMENSP: SysCacheIdentifier = 55;
pub const SysCacheIdentifier_STATEXTOID: SysCacheIdentifier = 56;
pub const SysCacheIdentifier_STATRELATTINH: SysCacheIdentifier = 57;
pub const SysCacheIdentifier_SUBSCRIPTIONNAME: SysCacheIdentifier = 58;
pub const SysCacheIdentifier_SUBSCRIPTIONOID: SysCacheIdentifier = 59;
pub const SysCacheIdentifier_SUBSCRIPTIONRELMAP: SysCacheIdentifier = 60;
pub const SysCacheIdentifier_TABLESPACEOID: SysCacheIdentifier = 61;
pub const SysCacheIdentifier_TRFOID: SysCacheIdentifier = 62;
pub const SysCacheIdentifier_TRFTYPELANG: SysCacheIdentifier = 63;
pub const SysCacheIdentifier_TSCONFIGMAP: SysCacheIdentifier = 64;
pub const SysCacheIdentifier_TSCONFIGNAMENSP: SysCacheIdentifier = 65;
pub const SysCacheIdentifier_TSCONFIGOID: SysCacheIdentifier = 66;
pub const SysCacheIdentifier_TSDICTNAMENSP: SysCacheIdentifier = 67;
pub const SysCacheIdentifier_TSDICTOID: SysCacheIdentifier = 68;
pub const SysCacheIdentifier_TSPARSERNAMENSP: SysCacheIdentifier = 69;
pub const SysCacheIdentifier_TSPARSEROID: SysCacheIdentifier = 70;
pub const SysCacheIdentifier_TSTEMPLATENAMENSP: SysCacheIdentifier = 71;
pub const SysCacheIdentifier_TSTEMPLATEOID: SysCacheIdentifier = 72;
pub const SysCacheIdentifier_TYPENAMENSP: SysCacheIdentifier = 73;
pub const SysCacheIdentifier_TYPEOID: SysCacheIdentifier = 74;
pub const SysCacheIdentifier_USERMAPPINGOID: SysCacheIdentifier = 75;
pub const SysCacheIdentifier_USERMAPPINGUSERSERVER: SysCacheIdentifier = 76;
pub const TINTERVALOID: u32 = 704;
pub const TRUE: u32 = 1;
pub const TYPECACHE_DOMAIN_INFO: u32 = 4096;
pub const TableLikeOption_CREATE_TABLE_LIKE_COMMENTS: TableLikeOption = 32;
pub const TableLikeOption_CREATE_TABLE_LIKE_DEFAULTS: TableLikeOption = 1;
pub const TableLikeOption_CREATE_TABLE_LIKE_IDENTITY: TableLikeOption = 4;
pub const TableLikeOption_CREATE_TABLE_LIKE_INDEXES: TableLikeOption = 8;
pub const TableLikeOption_CREATE_TABLE_LIKE_STATISTICS: TableLikeOption = 64;
pub const TableLikeOption_CREATE_TABLE_LIKE_STORAGE: TableLikeOption = 16;
pub const TypeFuncClass_TYPEFUNC_OTHER: TypeFuncClass = 3;
pub const TypeFuncClass_TYPEFUNC_RECORD: TypeFuncClass = 2;
pub const UpperRelationKind_UPPERREL_DISTINCT: UpperRelationKind = 3;
pub const UpperRelationKind_UPPERREL_FINAL: UpperRelationKind = 5;
pub const UpperRelationKind_UPPERREL_GROUP_AGG: UpperRelationKind = 1;
pub const UpperRelationKind_UPPERREL_ORDERED: UpperRelationKind = 4;
pub const UpperRelationKind_UPPERREL_WINDOW: UpperRelationKind = 2;
pub const VacuumOption_VACOPT_ANALYZE: VacuumOption = 2;
pub const VacuumOption_VACOPT_DISABLE_PAGE_SKIPPING: VacuumOption = 128;
pub const VacuumOption_VACOPT_FREEZE: VacuumOption = 8;
pub const VacuumOption_VACOPT_FULL: VacuumOption = 16;
pub const VacuumOption_VACOPT_NOWAIT: VacuumOption = 32;
pub const VacuumOption_VACOPT_SKIPTOAST: VacuumOption = 64;
pub const VacuumOption_VACOPT_VACUUM: VacuumOption = 1;
pub const VacuumOption_VACOPT_VERBOSE: VacuumOption = 4;
pub const WaitEventActivity_WAIT_EVENT_LOGICAL_APPLY_MAIN: WaitEventActivity = 83886086;
pub const WaitEventActivity_WAIT_EVENT_LOGICAL_LAUNCHER_MAIN: WaitEventActivity = 83886085;
pub const WaitEventIO_WAIT_EVENT_WAL_SYNC_METHOD_ASSIGN: WaitEventIO = 167772225;
pub const WaitEventIO_WAIT_EVENT_WAL_WRITE: WaitEventIO = 167772226;
pub const WaitEventIPC_WAIT_EVENT_EXECUTE_GATHER: WaitEventIPC = 134217731;
pub const WaitEventIPC_WAIT_EVENT_LOGICAL_SYNC_DATA: WaitEventIPC = 134217732;
pub const WaitEventIPC_WAIT_EVENT_LOGICAL_SYNC_STATE_CHANGE: WaitEventIPC = 134217733;
pub const WaitEventIPC_WAIT_EVENT_MQ_INTERNAL: WaitEventIPC = 134217734;
pub const WaitEventIPC_WAIT_EVENT_MQ_PUT_MESSAGE: WaitEventIPC = 134217735;
pub const WaitEventIPC_WAIT_EVENT_MQ_RECEIVE: WaitEventIPC = 134217736;
pub const WaitEventIPC_WAIT_EVENT_MQ_SEND: WaitEventIPC = 134217737;
pub const WaitEventIPC_WAIT_EVENT_PARALLEL_BITMAP_SCAN: WaitEventIPC = 134217739;
pub const WaitEventIPC_WAIT_EVENT_PARALLEL_FINISH: WaitEventIPC = 134217738;
pub const WaitEventIPC_WAIT_EVENT_PROCARRAY_GROUP_UPDATE: WaitEventIPC = 134217740;
pub const WaitEventIPC_WAIT_EVENT_REPLICATION_ORIGIN_DROP: WaitEventIPC = 134217741;
pub const WaitEventIPC_WAIT_EVENT_REPLICATION_SLOT_DROP: WaitEventIPC = 134217742;
pub const WaitEventIPC_WAIT_EVENT_SAFE_SNAPSHOT: WaitEventIPC = 134217743;
pub const WaitEventIPC_WAIT_EVENT_SYNC_REP: WaitEventIPC = 134217744;
pub const XACT_FLAGS_ACCESSEDTEMPNAMESPACE: u32 = 4;
pub const XACT_FLAGS_ACCESSEDTEMPREL: u32 = 1;
pub const XLOG_SEG_SIZE: u32 = 16777216;
pub const dsm_op_DSM_OP_DESTROY: dsm_op = 4;
pub const dsm_op_DSM_OP_RESIZE: dsm_op = 3;
pub const tuplehash_status_tuplehash_EMPTY: tuplehash_status = 0;
pub const tuplehash_status_tuplehash_IN_USE: tuplehash_status = 1;
pub type AclObjectKind = ::std::os::raw::c_uint;
pub type AttrDefault = attrDefault;
pub type BoolPtr = *mut bool;
pub type BulkInsertState = *mut BulkInsertStateData;
pub type ConstrCheck = constrCheck;
pub type DatumPtr = *mut Datum;
pub type ExplainOneQuery_hook_type = ::std::option::Option<
    unsafe extern "C" fn(
        query: *mut Query,
        cursorOptions: ::std::os::raw::c_int,
        into: *mut IntoClause,
        es: *mut ExplainState,
        queryString: *const ::std::os::raw::c_char,
        params: ParamListInfo,
    ),
>;
pub type FileName = *mut ::std::os::raw::c_char;
pub type FunctionCallInfo = *mut FunctionCallInfoData;
pub type GetForeignUpperPaths_function = ::std::option::Option<
    unsafe extern "C" fn(
        root: *mut PlannerInfo,
        stage: UpperRelationKind,
        input_rel: *mut RelOptInfo,
        output_rel: *mut RelOptInfo,
    ),
>;
pub type GrantObjectType = ::std::os::raw::c_uint;
pub type HTSU_Result = ::std::os::raw::c_uint;
pub type HeapScanDesc = *mut HeapScanDescData;
pub type ParallelHeapScanDesc = *mut ParallelHeapScanDescData;
pub type ParamFetchHook = ::std::option::Option<
    unsafe extern "C" fn(params: ParamListInfo, paramid: ::std::os::raw::c_int),
>;
pub type PartitionDispatch = *mut PartitionDispatchData;
pub type Pattern_Prefix_Status = ::std::os::raw::c_uint;
pub type Pattern_Type = ::std::os::raw::c_uint;
pub type RefetchForeignRow_function = ::std::option::Option<
    unsafe extern "C" fn(
        estate: *mut EState,
        erm: *mut ExecRowMark,
        rowid: Datum,
        updated: *mut bool,
    ) -> HeapTuple,
>;
pub type SnapshotSatisfiesFunc = ::std::option::Option<
    unsafe extern "C" fn(htup: HeapTuple, snapshot: Snapshot, buffer: Buffer) -> bool,
>;
pub type TupleConstr = tupleConstr;
pub type TupleDesc = *mut tupleDesc;
pub type VacuumOption = ::std::os::raw::c_uint;
pub type bitmapword = uint32;
pub type create_upper_paths_hook_type = ::std::option::Option<
    unsafe extern "C" fn(
        root: *mut PlannerInfo,
        stage: UpperRelationKind,
        input_rel: *mut RelOptInfo,
        output_rel: *mut RelOptInfo,
    ),
>;
pub type signedbitmapword = int32;
pub type validate_string_relopt =
    ::std::option::Option<unsafe extern "C" fn(value: *mut ::std::os::raw::c_char)>;
