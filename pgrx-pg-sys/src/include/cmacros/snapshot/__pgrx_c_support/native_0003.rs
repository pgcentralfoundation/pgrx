// Shared generated C macro support.

/// Explicit C typedef identities for the selected native callback capabilities.
pub mod __pgrx_c_callbacks {
    #[doc = "Explicit C callback typedef `AcquireSampleRowsFunc` with its exact native binding storage."]
    pub type AcquireSampleRowsFunc = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_int__0l__struct__RelationData__0p__0c__int__0c__struct__HeapTupleData__0p__0p__0c__int__0c__double__0p__0c__double__0p__0r>;
    #[doc = "Explicit C callback typedef `AddForeignUpdateTargets_function` with its exact native binding storage."]
    pub type AddForeignUpdateTargets_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__PlannerInfo__0p__0c__unsigned__int__0c__struct__RangeTblEntry__0p__0c__struct__RelationData__0p__0r>;
    #[doc = "Explicit C callback typedef `AnalyzeAttrComputeStatsFunc` with its exact native binding storage."]
    pub type AnalyzeAttrComputeStatsFunc = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__VacAttrStats__0p__0c__unsigned__long__0l__0p__0r__0l__struct__VacAttrStats__0p__0c__int__0c__0x5f426f6f6c__0p__0r__0c__int__0c__double__0r>;
    #[doc = "Explicit C callback typedef `AnalyzeAttrFetchFunc` with its exact native binding storage."]
    pub type AnalyzeAttrFetchFunc = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_unsigned__long__0l__struct__VacAttrStats__0p__0c__int__0c__0x5f426f6f6c__0p__0r>;
    #[doc = "Explicit C callback typedef `AnalyzeForeignTable_function` with its exact native binding storage."]
    pub type AnalyzeForeignTable_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__RelationData__0p__0c__int__0l__0p__0p__0r__0l__struct__RelationData__0p__0c__int__0c__struct__HeapTupleData__0p__0p__0c__int__0c__double__0p__0c__double__0p__0r__0c__unsigned__int__0p__0r>;
    #[cfg(feature = "pg15")]
    #[doc = "Explicit C callback typedef `ArchiveCheckConfiguredCB` with its exact native binding storage."]
    pub type ArchiveCheckConfiguredCB = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__void__0r,
    >;
    #[cfg(feature = "pg15")]
    #[doc = "Explicit C callback typedef `ArchiveFileCB` with its exact native binding storage."]
    pub type ArchiveFileCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__const__char__0p__0c__const__char__0p__0r>;
    #[cfg(feature = "pg15")]
    #[doc = "Explicit C callback typedef `ArchiveModuleInit` with its exact native binding storage."]
    pub type ArchiveModuleInit = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__ArchiveModuleCallbacks__0p__0r,
    >;
    #[cfg(feature = "pg15")]
    #[doc = "Explicit C callback typedef `ArchiveShutdownCB` with its exact native binding storage."]
    pub type ArchiveShutdownCB = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__void__0r,
    >;
    #[doc = "Explicit C callback typedef `BeginDirectModify_function` with its exact native binding storage."]
    pub type BeginDirectModify_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__ForeignScanState__0p__0c__int__0r,
    >;
    #[doc = "Explicit C callback typedef `BeginForeignInsert_function` with its exact native binding storage."]
    pub type BeginForeignInsert_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ModifyTableState__0p__0c__struct__ResultRelInfo__0p__0r>;
    #[doc = "Explicit C callback typedef `BeginForeignModify_function` with its exact native binding storage."]
    pub type BeginForeignModify_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ModifyTableState__0p__0c__struct__ResultRelInfo__0p__0c__struct__List__0p__0c__int__0c__int__0r>;
    #[doc = "Explicit C callback typedef `BeginForeignScan_function` with its exact native binding storage."]
    pub type BeginForeignScan_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__ForeignScanState__0p__0c__int__0r,
    >;
    #[doc = "Explicit C callback typedef `BeginSampleScan_function` with its exact native binding storage."]
    pub type BeginSampleScan_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__SampleScanState__0p__0c__unsigned__long__0p__0c__int__0c__unsigned__int__0r>;
    #[doc = "Explicit C callback typedef `CCFastEqualFN` with its exact native binding storage."]
    pub type CCFastEqualFN = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__unsigned__long__0c__unsigned__long__0r>;
    #[doc = "Explicit C callback typedef `CCHashFN` with its exact native binding storage."]
    pub type CCHashFN = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_unsigned__int__0l__unsigned__long__0r,
    >;
    #[doc = "Explicit C callback typedef `CoerceParamHook` with its exact native binding storage."]
    pub type CoerceParamHook = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__Node__0p__0l__struct__ParseState__0p__0c__struct__Param__0p__0c__unsigned__int__0c__int__0c__int__0r>;
    #[doc = "Explicit C callback typedef `EOM_flatten_into_method` with its exact native binding storage."]
    pub type EOM_flatten_into_method = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ExpandedObjectHeader__0p__0c__void__0p__0c__unsigned__long__0r>;
    #[doc = "Explicit C callback typedef `EOM_get_flat_size_method` with its exact native binding storage."]
    pub type EOM_get_flat_size_method = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_unsigned__long__0l__struct__ExpandedObjectHeader__0p__0r>;
    #[doc = "Explicit C callback typedef `EndDirectModify_function` with its exact native binding storage."]
    pub type EndDirectModify_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__ForeignScanState__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `EndForeignInsert_function` with its exact native binding storage."]
    pub type EndForeignInsert_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__EState__0p__0c__struct__ResultRelInfo__0p__0r>;
    #[doc = "Explicit C callback typedef `EndForeignModify_function` with its exact native binding storage."]
    pub type EndForeignModify_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__EState__0p__0c__struct__ResultRelInfo__0p__0r>;
    #[doc = "Explicit C callback typedef `EndForeignScan_function` with its exact native binding storage."]
    pub type EndForeignScan_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__ForeignScanState__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `EndSampleScan_function` with its exact native binding storage."]
    pub type EndSampleScan_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__SampleScanState__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `EstimateDSMForeignScan_function` with its exact native binding storage."]
    pub type EstimateDSMForeignScan_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_unsigned__long__0l__struct__ForeignScanState__0p__0c__struct__ParallelContext__0p__0r>;
    #[doc = "Explicit C callback typedef `ExecEvalBoolSubroutine` with its exact native binding storage."]
    pub type ExecEvalBoolSubroutine = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__ExprState__0p__0c__struct__ExprEvalStep__0p__0c__struct__ExprContext__0p__0r>;
    #[doc = "Explicit C callback typedef `ExecEvalSubroutine` with its exact native binding storage."]
    pub type ExecEvalSubroutine = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ExprState__0p__0c__struct__ExprEvalStep__0p__0c__struct__ExprContext__0p__0r>;
    #[doc = "Explicit C callback typedef `ExecForeignBatchInsert_function` with its exact native binding storage."]
    pub type ExecForeignBatchInsert_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__TupleTableSlot__0p__0p__0l__struct__EState__0p__0c__struct__ResultRelInfo__0p__0c__struct__TupleTableSlot__0p__0p__0c__struct__TupleTableSlot__0p__0p__0c__int__0p__0r>;
    #[doc = "Explicit C callback typedef `ExecForeignDelete_function` with its exact native binding storage."]
    pub type ExecForeignDelete_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__TupleTableSlot__0p__0l__struct__EState__0p__0c__struct__ResultRelInfo__0p__0c__struct__TupleTableSlot__0p__0c__struct__TupleTableSlot__0p__0r>;
    #[doc = "Explicit C callback typedef `ExecForeignInsert_function` with its exact native binding storage."]
    pub type ExecForeignInsert_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__TupleTableSlot__0p__0l__struct__EState__0p__0c__struct__ResultRelInfo__0p__0c__struct__TupleTableSlot__0p__0c__struct__TupleTableSlot__0p__0r>;
    #[doc = "Explicit C callback typedef `ExecForeignTruncate_function` with its exact native binding storage."]
    pub type ExecForeignTruncate_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__List__0p__0c__enum__DropBehavior__0c__0x5f426f6f6c__0r>;
    #[doc = "Explicit C callback typedef `ExecForeignUpdate_function` with its exact native binding storage."]
    pub type ExecForeignUpdate_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__TupleTableSlot__0p__0l__struct__EState__0p__0c__struct__ResultRelInfo__0p__0c__struct__TupleTableSlot__0p__0c__struct__TupleTableSlot__0p__0r>;
    #[doc = "Explicit C callback typedef `ExecProcNodeMtd` with its exact native binding storage."]
    pub type ExecProcNodeMtd = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__TupleTableSlot__0p__0l__struct__PlanState__0p__0r>;
    #[doc = "Explicit C callback typedef `ExecScanAccessMtd` with its exact native binding storage."]
    pub type ExecScanAccessMtd = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__TupleTableSlot__0p__0l__struct__ScanState__0p__0r>;
    #[doc = "Explicit C callback typedef `ExecScanRecheckMtd` with its exact native binding storage."]
    pub type ExecScanRecheckMtd = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__ScanState__0p__0c__struct__TupleTableSlot__0p__0r>;
    #[cfg(feature = "pg15")]
    #[doc = "Explicit C callback typedef `ExecutorCheckPerms_hook_type` with its exact native binding storage."]
    pub type ExecutorCheckPerms_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__List__0p__0c__0x5f426f6f6c__0r>;
    #[doc = "Explicit C callback typedef `ExecutorEnd_hook_type` with its exact native binding storage."]
    pub type ExecutorEnd_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__QueryDesc__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `ExecutorFinish_hook_type` with its exact native binding storage."]
    pub type ExecutorFinish_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__QueryDesc__0p__0r,
    >;
    #[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
    #[doc = "Explicit C callback typedef `ExecutorRun_hook_type` with its exact native binding storage."]
    pub type ExecutorRun_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__QueryDesc__0p__0c__enum__ScanDirection__0c__unsigned__long__0c__0x5f426f6f6c__0r>;
    #[doc = "Explicit C callback typedef `ExecutorStart_hook_type` with its exact native binding storage."]
    pub type ExecutorStart_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__QueryDesc__0p__0c__int__0r,
    >;
    #[doc = "Explicit C callback typedef `ExplainDirectModify_function` with its exact native binding storage."]
    pub type ExplainDirectModify_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ForeignScanState__0p__0c__struct__ExplainState__0p__0r>;
    #[doc = "Explicit C callback typedef `ExplainForeignModify_function` with its exact native binding storage."]
    pub type ExplainForeignModify_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ModifyTableState__0p__0c__struct__ResultRelInfo__0p__0c__struct__List__0p__0c__int__0c__struct__ExplainState__0p__0r>;
    #[doc = "Explicit C callback typedef `ExplainForeignScan_function` with its exact native binding storage."]
    pub type ExplainForeignScan_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ForeignScanState__0p__0c__struct__ExplainState__0p__0r>;
    #[doc = "Explicit C callback typedef `ExplainOneQuery_hook_type` with its exact native binding storage."]
    pub type ExplainOneQuery_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__Query__0p__0c__int__0c__struct__IntoClause__0p__0c__struct__ExplainState__0p__0c__const__char__0p__0c__struct__ParamListInfoData__0p__0c__struct__QueryEnvironment__0p__0r>;
    #[doc = "Explicit C callback typedef `ExprContextCallbackFunction` with its exact native binding storage."]
    pub type ExprContextCallbackFunction = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__unsigned__long__0r,
    >;
    #[doc = "Explicit C callback typedef `ExprStateEvalFunc` with its exact native binding storage."]
    pub type ExprStateEvalFunc = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_unsigned__long__0l__struct__ExprState__0p__0c__struct__ExprContext__0p__0c__0x5f426f6f6c__0p__0r>;
    #[doc = "Explicit C callback typedef `ForeignAsyncConfigureWait_function` with its exact native binding storage."]
    pub type ForeignAsyncConfigureWait_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__AsyncRequest__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `ForeignAsyncNotify_function` with its exact native binding storage."]
    pub type ForeignAsyncNotify_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__AsyncRequest__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `ForeignAsyncRequest_function` with its exact native binding storage."]
    pub type ForeignAsyncRequest_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__AsyncRequest__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `GetForeignJoinPaths_function` with its exact native binding storage."]
    pub type GetForeignJoinPaths_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__RelOptInfo__0p__0c__enum__JoinType__0c__struct__JoinPathExtraData__0p__0r>;
    #[doc = "Explicit C callback typedef `GetForeignModifyBatchSize_function` with its exact native binding storage."]
    pub type GetForeignModifyBatchSize_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_int__0l__struct__ResultRelInfo__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `GetForeignPaths_function` with its exact native binding storage."]
    pub type GetForeignPaths_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__unsigned__int__0r>;
    #[doc = "Explicit C callback typedef `GetForeignPlan_function` with its exact native binding storage."]
    pub type GetForeignPlan_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__ForeignScan__0p__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__unsigned__int__0c__struct__ForeignPath__0p__0c__struct__List__0p__0c__struct__List__0p__0c__struct__Plan__0p__0r>;
    #[doc = "Explicit C callback typedef `GetForeignRelSize_function` with its exact native binding storage."]
    pub type GetForeignRelSize_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__unsigned__int__0r>;
    #[doc = "Explicit C callback typedef `GetForeignRowMarkType_function` with its exact native binding storage."]
    pub type GetForeignRowMarkType_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_enum__RowMarkType__0l__struct__RangeTblEntry__0p__0c__enum__LockClauseStrength__0r>;
    #[doc = "Explicit C callback typedef `GetForeignUpperPaths_function` with its exact native binding storage."]
    pub type GetForeignUpperPaths_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__PlannerInfo__0p__0c__enum__UpperRelationKind__0c__struct__RelOptInfo__0p__0c__struct__RelOptInfo__0p__0c__void__0p__0r>;
    #[doc = "Explicit C callback typedef `GucBoolAssignHook` with its exact native binding storage."]
    pub type GucBoolAssignHook = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__0x5f426f6f6c__0c__void__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `GucBoolCheckHook` with its exact native binding storage."]
    pub type GucBoolCheckHook = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__0x5f426f6f6c__0p__0c__void__0p__0p__0c__GucSource__0r>;
    #[doc = "Explicit C callback typedef `GucEnumAssignHook` with its exact native binding storage."]
    pub type GucEnumAssignHook = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__int__0c__void__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `GucEnumCheckHook` with its exact native binding storage."]
    pub type GucEnumCheckHook = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__int__0p__0c__void__0p__0p__0c__GucSource__0r>;
    #[doc = "Explicit C callback typedef `GucIntAssignHook` with its exact native binding storage."]
    pub type GucIntAssignHook = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__int__0c__void__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `GucIntCheckHook` with its exact native binding storage."]
    pub type GucIntCheckHook = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__int__0p__0c__void__0p__0p__0c__GucSource__0r>;
    #[doc = "Explicit C callback typedef `GucRealAssignHook` with its exact native binding storage."]
    pub type GucRealAssignHook = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__double__0c__void__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `GucRealCheckHook` with its exact native binding storage."]
    pub type GucRealCheckHook = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__double__0p__0c__void__0p__0p__0c__GucSource__0r>;
    #[doc = "Explicit C callback typedef `GucShowHook` with its exact native binding storage."]
    pub type GucShowHook = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_const__char__0p__0l__void__0r,
    >;
    #[doc = "Explicit C callback typedef `GucStringAssignHook` with its exact native binding storage."]
    pub type GucStringAssignHook = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__const__char__0p__0c__void__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `GucStringCheckHook` with its exact native binding storage."]
    pub type GucStringCheckHook = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__char__0p__0p__0c__void__0p__0p__0c__GucSource__0r>;
    #[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
    #[doc = "Explicit C callback typedef `HashAllocFunc` with its exact native binding storage."]
    pub type HashAllocFunc = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0p__0l__unsigned__long__0r,
    >;
    #[doc = "Explicit C callback typedef `HashCompareFunc` with its exact native binding storage."]
    pub type HashCompareFunc = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_int__0l__const__void__0p__0c__const__void__0p__0c__unsigned__long__0r>;
    #[doc = "Explicit C callback typedef `HashCopyFunc` with its exact native binding storage."]
    pub type HashCopyFunc = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0p__0l__void__0p__0c__const__void__0p__0c__unsigned__long__0r>;
    #[doc = "Explicit C callback typedef `HashValueFunc` with its exact native binding storage."]
    pub type HashValueFunc = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_unsigned__int__0l__const__void__0p__0c__unsigned__long__0r>;
    #[doc = "Explicit C callback typedef `ImportForeignSchema_function` with its exact native binding storage."]
    pub type ImportForeignSchema_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__List__0p__0l__struct__ImportForeignSchemaStmt__0p__0c__unsigned__int__0r>;
    #[doc = "Explicit C callback typedef `IndexBuildCallback` with its exact native binding storage."]
    pub type IndexBuildCallback = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__RelationData__0p__0c__struct__ItemPointerData__0p__0c__unsigned__long__0p__0c__0x5f426f6f6c__0p__0c__0x5f426f6f6c__0c__void__0p__0r>;
    #[doc = "Explicit C callback typedef `IndexBulkDeleteCallback` with its exact native binding storage."]
    pub type IndexBulkDeleteCallback = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__ItemPointerData__0p__0c__void__0p__0r>;
    #[doc = "Explicit C callback typedef `InitSampleScan_function` with its exact native binding storage."]
    pub type InitSampleScan_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__SampleScanState__0p__0c__int__0r,
    >;
    #[doc = "Explicit C callback typedef `InitializeDSMForeignScan_function` with its exact native binding storage."]
    pub type InitializeDSMForeignScan_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ForeignScanState__0p__0c__struct__ParallelContext__0p__0c__void__0p__0r>;
    #[doc = "Explicit C callback typedef `InitializeWorkerForeignScan_function` with its exact native binding storage."]
    pub type InitializeWorkerForeignScan_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ForeignScanState__0p__0c__struct__shm_toc__0p__0c__void__0p__0r>;
    #[doc = "Explicit C callback typedef `IsForeignPathAsyncCapable_function` with its exact native binding storage."]
    pub type IsForeignPathAsyncCapable_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__ForeignPath__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `IsForeignRelUpdatable_function` with its exact native binding storage."]
    pub type IsForeignRelUpdatable_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_int__0l__struct__RelationData__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `IsForeignScanParallelSafe_function` with its exact native binding storage."]
    pub type IsForeignScanParallelSafe_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__RangeTblEntry__0p__0r>;
    #[doc = "Explicit C callback typedef `IterateDirectModify_function` with its exact native binding storage."]
    pub type IterateDirectModify_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__TupleTableSlot__0p__0l__struct__ForeignScanState__0p__0r>;
    #[doc = "Explicit C callback typedef `IterateForeignScan_function` with its exact native binding storage."]
    pub type IterateForeignScan_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__TupleTableSlot__0p__0l__struct__ForeignScanState__0p__0r>;
    #[doc = "Explicit C callback typedef `JitProviderCompileExprCB` with its exact native binding storage."]
    pub type JitProviderCompileExprCB = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__ExprState__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `JitProviderInit` with its exact native binding storage."]
    pub type JitProviderInit = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__JitProviderCallbacks__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `JitProviderReleaseContextCB` with its exact native binding storage."]
    pub type JitProviderReleaseContextCB = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__JitContext__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `JitProviderResetAfterErrorCB` with its exact native binding storage."]
    pub type JitProviderResetAfterErrorCB = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__void__0r,
    >;
    #[doc = "Explicit C callback typedef `LogicalDecodeBeginCB` with its exact native binding storage."]
    pub type LogicalDecodeBeginCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__ReorderBufferTXN__0p__0r>;
    #[doc = "Explicit C callback typedef `LogicalDecodeBeginPrepareCB` with its exact native binding storage."]
    pub type LogicalDecodeBeginPrepareCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__ReorderBufferTXN__0p__0r>;
    #[doc = "Explicit C callback typedef `LogicalDecodeChangeCB` with its exact native binding storage."]
    pub type LogicalDecodeChangeCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__ReorderBufferTXN__0p__0c__struct__RelationData__0p__0c__struct__ReorderBufferChange__0p__0r>;
    #[doc = "Explicit C callback typedef `LogicalDecodeCommitCB` with its exact native binding storage."]
    pub type LogicalDecodeCommitCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0r>;
    #[doc = "Explicit C callback typedef `LogicalDecodeCommitPreparedCB` with its exact native binding storage."]
    pub type LogicalDecodeCommitPreparedCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0r>;
    #[doc = "Explicit C callback typedef `LogicalDecodeFilterByOriginCB` with its exact native binding storage."]
    pub type LogicalDecodeFilterByOriginCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__LogicalDecodingContext__0p__0c__unsigned__short__0r>;
    #[doc = "Explicit C callback typedef `LogicalDecodeFilterPrepareCB` with its exact native binding storage."]
    pub type LogicalDecodeFilterPrepareCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__LogicalDecodingContext__0p__0c__unsigned__int__0c__const__char__0p__0r>;
    #[doc = "Explicit C callback typedef `LogicalDecodeMessageCB` with its exact native binding storage."]
    pub type LogicalDecodeMessageCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0c__0x5f426f6f6c__0c__const__char__0p__0c__unsigned__long__0c__const__char__0p__0r>;
    #[doc = "Explicit C callback typedef `LogicalDecodePrepareCB` with its exact native binding storage."]
    pub type LogicalDecodePrepareCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0r>;
    #[doc = "Explicit C callback typedef `LogicalDecodeRollbackPreparedCB` with its exact native binding storage."]
    pub type LogicalDecodeRollbackPreparedCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0c__long__0r>;
    #[doc = "Explicit C callback typedef `LogicalDecodeShutdownCB` with its exact native binding storage."]
    pub type LogicalDecodeShutdownCB = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__LogicalDecodingContext__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `LogicalDecodeStartupCB` with its exact native binding storage."]
    pub type LogicalDecodeStartupCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__OutputPluginOptions__0p__0c__0x5f426f6f6c__0r>;
    #[doc = "Explicit C callback typedef `LogicalDecodeStreamAbortCB` with its exact native binding storage."]
    pub type LogicalDecodeStreamAbortCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0r>;
    #[doc = "Explicit C callback typedef `LogicalDecodeStreamChangeCB` with its exact native binding storage."]
    pub type LogicalDecodeStreamChangeCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__ReorderBufferTXN__0p__0c__struct__RelationData__0p__0c__struct__ReorderBufferChange__0p__0r>;
    #[doc = "Explicit C callback typedef `LogicalDecodeStreamCommitCB` with its exact native binding storage."]
    pub type LogicalDecodeStreamCommitCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0r>;
    #[doc = "Explicit C callback typedef `LogicalDecodeStreamMessageCB` with its exact native binding storage."]
    pub type LogicalDecodeStreamMessageCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0c__0x5f426f6f6c__0c__const__char__0p__0c__unsigned__long__0c__const__char__0p__0r>;
    #[doc = "Explicit C callback typedef `LogicalDecodeStreamPrepareCB` with its exact native binding storage."]
    pub type LogicalDecodeStreamPrepareCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0r>;
    #[doc = "Explicit C callback typedef `LogicalDecodeStreamStartCB` with its exact native binding storage."]
    pub type LogicalDecodeStreamStartCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__ReorderBufferTXN__0p__0r>;
    #[doc = "Explicit C callback typedef `LogicalDecodeStreamStopCB` with its exact native binding storage."]
    pub type LogicalDecodeStreamStopCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__LogicalDecodingContext__0p__0c__struct__ReorderBufferTXN__0p__0r>;
    #[doc = "Explicit C callback typedef `LogicalOutputPluginInit` with its exact native binding storage."]
    pub type LogicalOutputPluginInit = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__OutputPluginCallbacks__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `LogicalOutputPluginWriterPrepareWrite` with its exact native binding storage."]
    pub type LogicalOutputPluginWriterPrepareWrite = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__LogicalDecodingContext__0p__0c__unsigned__long__0c__unsigned__int__0c__0x5f426f6f6c__0r>;
    #[doc = "Explicit C callback typedef `LogicalOutputPluginWriterUpdateProgress` with its exact native binding storage."]
    pub type LogicalOutputPluginWriterUpdateProgress = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__LogicalDecodingContext__0p__0c__unsigned__long__0c__unsigned__int__0c__0x5f426f6f6c__0r>;
    #[doc = "Explicit C callback typedef `LogicalOutputPluginWriterWrite` with its exact native binding storage."]
    pub type LogicalOutputPluginWriterWrite = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__LogicalDecodingContext__0p__0c__unsigned__long__0c__unsigned__int__0c__0x5f426f6f6c__0r>;
    #[doc = "Explicit C callback typedef `MemoryContextCallbackFunction` with its exact native binding storage."]
    pub type MemoryContextCallbackFunction = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__void__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `MemoryStatsPrintFunc` with its exact native binding storage."]
    pub type MemoryStatsPrintFunc = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__MemoryContextData__0p__0c__void__0p__0c__const__char__0p__0c__0x5f426f6f6c__0r>;
    #[doc = "Explicit C callback typedef `NextSampleBlock_function` with its exact native binding storage."]
    pub type NextSampleBlock_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_unsigned__int__0l__struct__SampleScanState__0p__0c__unsigned__int__0r>;
    #[doc = "Explicit C callback typedef `NextSampleTuple_function` with its exact native binding storage."]
    pub type NextSampleTuple_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_unsigned__short__0l__struct__SampleScanState__0p__0c__unsigned__int__0c__unsigned__short__0r>;
    #[doc = "Explicit C callback typedef `PGFInfoFunction` with its exact native binding storage."]
    pub type PGFInfoFunction = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_const__Pg_finfo_record__0p__0l__void__0r,
    >;
    #[doc = "Explicit C callback typedef `PGFunction` with its exact native binding storage."]
    pub type PGFunction = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_unsigned__long__0l__struct__FunctionCallInfoBaseData__0p__0r>;
    #[doc = "Explicit C callback typedef `PGModuleMagicFunction` with its exact native binding storage."]
    pub type PGModuleMagicFunction = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_const__Pg_magic_struct__0p__0l__void__0r,
    >;
    #[doc = "Explicit C callback typedef `ParamCompileHook` with its exact native binding storage."]
    pub type ParamCompileHook = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ParamListInfoData__0p__0c__struct__Param__0p__0c__struct__ExprState__0p__0c__unsigned__long__0p__0c__0x5f426f6f6c__0p__0r>;
    #[doc = "Explicit C callback typedef `ParamFetchHook` with its exact native binding storage."]
    pub type ParamFetchHook = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__ParamExternData__0p__0l__struct__ParamListInfoData__0p__0c__int__0c__0x5f426f6f6c__0c__struct__ParamExternData__0p__0r>;
    #[doc = "Explicit C callback typedef `ParseParamRefHook` with its exact native binding storage."]
    pub type ParseParamRefHook = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__Node__0p__0l__struct__ParseState__0p__0c__struct__ParamRef__0p__0r>;
    #[doc = "Explicit C callback typedef `ParserSetupHook` with its exact native binding storage."]
    pub type ParserSetupHook = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__ParseState__0p__0c__void__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `PlanDirectModify_function` with its exact native binding storage."]
    pub type PlanDirectModify_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__PlannerInfo__0p__0c__struct__ModifyTable__0p__0c__unsigned__int__0c__int__0r>;
    #[doc = "Explicit C callback typedef `PlanForeignModify_function` with its exact native binding storage."]
    pub type PlanForeignModify_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__List__0p__0l__struct__PlannerInfo__0p__0c__struct__ModifyTable__0p__0c__unsigned__int__0c__int__0r>;
    #[doc = "Explicit C callback typedef `PostParseColumnRefHook` with its exact native binding storage."]
    pub type PostParseColumnRefHook = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__Node__0p__0l__struct__ParseState__0p__0c__struct__ColumnRef__0p__0c__struct__Node__0p__0r>;
    #[doc = "Explicit C callback typedef `PreParseColumnRefHook` with its exact native binding storage."]
    pub type PreParseColumnRefHook = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__Node__0p__0l__struct__ParseState__0p__0c__struct__ColumnRef__0p__0r>;
    #[doc = "Explicit C callback typedef `ProcessUtility_hook_type` with its exact native binding storage."]
    pub type ProcessUtility_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__PlannedStmt__0p__0c__const__char__0p__0c__0x5f426f6f6c__0c__ProcessUtilityContext__0c__struct__ParamListInfoData__0p__0c__struct__QueryEnvironment__0p__0c__struct__0x5f446573745265636569766572__0p__0c__struct__QueryCompletion__0p__0r>;
    #[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
    #[doc = "Explicit C callback typedef `PushFunction` with its exact native binding storage."]
    pub type PushFunction = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__unsigned__long__0c__struct__TSQueryParserStateData__0p__0c__char__0p__0c__int__0c__short__0c__0x5f426f6f6c__0r>;
    #[doc = "Explicit C callback typedef `RangeVarGetRelidCallback` with its exact native binding storage."]
    pub type RangeVarGetRelidCallback = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__const__struct__RangeVar__0p__0c__unsigned__int__0c__unsigned__int__0c__void__0p__0r>;
    #[doc = "Explicit C callback typedef `ReInitializeDSMForeignScan_function` with its exact native binding storage."]
    pub type ReInitializeDSMForeignScan_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ForeignScanState__0p__0c__struct__ParallelContext__0p__0c__void__0p__0r>;
    #[doc = "Explicit C callback typedef `ReScanForeignScan_function` with its exact native binding storage."]
    pub type ReScanForeignScan_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__ForeignScanState__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `RecheckForeignScan_function` with its exact native binding storage."]
    pub type RecheckForeignScan_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__ForeignScanState__0p__0c__struct__TupleTableSlot__0p__0r>;
    #[doc = "Explicit C callback typedef `RefetchForeignRow_function` with its exact native binding storage."]
    pub type RefetchForeignRow_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__EState__0p__0c__struct__ExecRowMark__0p__0c__unsigned__long__0c__struct__TupleTableSlot__0p__0c__0x5f426f6f6c__0p__0r>;
    #[doc = "Explicit C callback typedef `RelcacheCallbackFunction` with its exact native binding storage."]
    pub type RelcacheCallbackFunction = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__unsigned__long__0c__unsigned__int__0r,
    >;
    #[doc = "Explicit C callback typedef `ReorderBufferApplyChangeCB` with its exact native binding storage."]
    pub type ReorderBufferApplyChangeCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0c__struct__RelationData__0p__0c__struct__ReorderBufferChange__0p__0r>;
    #[doc = "Explicit C callback typedef `ReorderBufferBeginCB` with its exact native binding storage."]
    pub type ReorderBufferBeginCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0r>;
    #[doc = "Explicit C callback typedef `ReorderBufferBeginPrepareCB` with its exact native binding storage."]
    pub type ReorderBufferBeginPrepareCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0r>;
    #[doc = "Explicit C callback typedef `ReorderBufferCommitCB` with its exact native binding storage."]
    pub type ReorderBufferCommitCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0r>;
    #[doc = "Explicit C callback typedef `ReorderBufferCommitPreparedCB` with its exact native binding storage."]
    pub type ReorderBufferCommitPreparedCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0r>;
    #[doc = "Explicit C callback typedef `ReorderBufferMessageCB` with its exact native binding storage."]
    pub type ReorderBufferMessageCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0c__0x5f426f6f6c__0c__const__char__0p__0c__unsigned__long__0c__const__char__0p__0r>;
    #[doc = "Explicit C callback typedef `ReorderBufferPrepareCB` with its exact native binding storage."]
    pub type ReorderBufferPrepareCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0r>;
    #[doc = "Explicit C callback typedef `ReorderBufferRollbackPreparedCB` with its exact native binding storage."]
    pub type ReorderBufferRollbackPreparedCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0c__long__0r>;
    #[doc = "Explicit C callback typedef `ReorderBufferStreamAbortCB` with its exact native binding storage."]
    pub type ReorderBufferStreamAbortCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0r>;
    #[doc = "Explicit C callback typedef `ReorderBufferStreamChangeCB` with its exact native binding storage."]
    pub type ReorderBufferStreamChangeCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0c__struct__RelationData__0p__0c__struct__ReorderBufferChange__0p__0r>;
    #[doc = "Explicit C callback typedef `ReorderBufferStreamCommitCB` with its exact native binding storage."]
    pub type ReorderBufferStreamCommitCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0r>;
    #[doc = "Explicit C callback typedef `ReorderBufferStreamMessageCB` with its exact native binding storage."]
    pub type ReorderBufferStreamMessageCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0c__0x5f426f6f6c__0c__const__char__0p__0c__unsigned__long__0c__const__char__0p__0r>;
    #[doc = "Explicit C callback typedef `ReorderBufferStreamPrepareCB` with its exact native binding storage."]
    pub type ReorderBufferStreamPrepareCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0r>;
    #[doc = "Explicit C callback typedef `ReorderBufferStreamStartCB` with its exact native binding storage."]
    pub type ReorderBufferStreamStartCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0r>;
    #[doc = "Explicit C callback typedef `ReorderBufferStreamStopCB` with its exact native binding storage."]
    pub type ReorderBufferStreamStopCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0r>;
    #[doc = "Explicit C callback typedef `ReparameterizeForeignPathByChild_function` with its exact native binding storage."]
    pub type ReparameterizeForeignPathByChild_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__List__0p__0l__struct__PlannerInfo__0p__0c__struct__List__0p__0c__struct__RelOptInfo__0p__0r>;
    #[doc = "Explicit C callback typedef `ResourceReleaseCallback` with its exact native binding storage."]
    pub type ResourceReleaseCallback = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__ResourceReleasePhase__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__void__0p__0r>;
    #[doc = "Explicit C callback typedef `SampleScanGetSampleSize_function` with its exact native binding storage."]
    pub type SampleScanGetSampleSize_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__List__0p__0c__unsigned__int__0p__0c__double__0p__0r>;
    #[doc = "Explicit C callback typedef `ScanKeywordHashFunc` with its exact native binding storage."]
    pub type ScanKeywordHashFunc = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_int__0l__const__void__0p__0c__unsigned__long__0r,
    >;
    #[doc = "Explicit C callback typedef `ShutdownForeignScan_function` with its exact native binding storage."]
    pub type ShutdownForeignScan_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__ForeignScanState__0p__0r,
    >;
    #[cfg(any(feature = "pg15", feature = "pg16"))]
    #[doc = "Explicit C callback typedef `SlruScanCallback` with its exact native binding storage."]
    pub type SlruScanCallback = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__SlruCtlData__0p__0c__char__0p__0c__int__0c__void__0p__0r>;
    #[doc = "Explicit C callback typedef `SubXactCallback` with its exact native binding storage."]
    pub type SubXactCallback = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__SubXactEvent__0c__unsigned__int__0c__unsigned__int__0c__void__0p__0r>;
    #[doc = "Explicit C callback typedef `SubscriptExecSetup` with its exact native binding storage."]
    pub type SubscriptExecSetup = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__const__struct__SubscriptingRef__0p__0c__struct__SubscriptingRefState__0p__0c__struct__SubscriptExecSteps__0p__0r>;
    #[doc = "Explicit C callback typedef `SubscriptTransform` with its exact native binding storage."]
    pub type SubscriptTransform = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__SubscriptingRef__0p__0c__struct__List__0p__0c__struct__ParseState__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0r>;
    #[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
    #[doc = "Explicit C callback typedef `SyscacheCallbackFunction` with its exact native binding storage."]
    pub type SyscacheCallbackFunction = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__unsigned__long__0c__int__0c__unsigned__int__0r>;
    #[doc = "Explicit C callback typedef `TSExecuteCallback` with its exact native binding storage."]
    pub type TSExecuteCallback = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_TSTernaryValue__0l__void__0p__0c__QueryOperand__0p__0c__struct__ExecPhraseData__0p__0r>;
    #[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
    #[doc = "Explicit C callback typedef `TwoPhaseCallback` with its exact native binding storage."]
    pub type TwoPhaseCallback = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__unsigned__int__0c__unsigned__short__0c__void__0p__0c__unsigned__int__0r>;
    #[doc = "Explicit C callback typedef `WALSegmentCloseCB` with its exact native binding storage."]
    pub type WALSegmentCloseCB = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__XLogReaderState__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `WALSegmentOpenCB` with its exact native binding storage."]
    pub type WALSegmentOpenCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__XLogReaderState__0p__0c__unsigned__long__0c__unsigned__int__0p__0r>;
    #[doc = "Explicit C callback typedef `XLogPageReadCB` with its exact native binding storage."]
    pub type XLogPageReadCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_int__0l__struct__XLogReaderState__0p__0c__unsigned__long__0c__int__0c__unsigned__long__0c__char__0p__0r>;
    #[doc = "Explicit C callback typedef `XactCallback` with its exact native binding storage."]
    pub type XactCallback = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__XactEvent__0c__void__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `amadjustmembers_function` with its exact native binding storage."]
    pub type amadjustmembers_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__unsigned__int__0c__unsigned__int__0c__struct__List__0p__0c__struct__List__0p__0r>;
    #[doc = "Explicit C callback typedef `ambeginscan_function` with its exact native binding storage."]
    pub type ambeginscan_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__IndexScanDescData__0p__0l__struct__RelationData__0p__0c__int__0c__int__0r>;
    #[doc = "Explicit C callback typedef `ambuild_function` with its exact native binding storage."]
    pub type ambuild_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__IndexBuildResult__0p__0l__struct__RelationData__0p__0c__struct__RelationData__0p__0c__struct__IndexInfo__0p__0r>;
    #[doc = "Explicit C callback typedef `ambuildempty_function` with its exact native binding storage."]
    pub type ambuildempty_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__RelationData__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `ambuildphasename_function` with its exact native binding storage."]
    pub type ambuildphasename_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_char__0p__0l__long__0r,
    >;
    #[doc = "Explicit C callback typedef `ambulkdelete_function` with its exact native binding storage."]
    pub type ambulkdelete_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__IndexBulkDeleteResult__0p__0l__struct__IndexVacuumInfo__0p__0c__struct__IndexBulkDeleteResult__0p__0c__0x5f426f6f6c__0l__0p__0r__0l__struct__ItemPointerData__0p__0c__void__0p__0r__0c__void__0p__0r>;
    #[doc = "Explicit C callback typedef `amcanreturn_function` with its exact native binding storage."]
    pub type amcanreturn_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__RelationData__0p__0c__int__0r>;
    #[doc = "Explicit C callback typedef `amcostestimate_function` with its exact native binding storage."]
    pub type amcostestimate_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__PlannerInfo__0p__0c__struct__IndexPath__0p__0c__double__0c__double__0p__0c__double__0p__0c__double__0p__0c__double__0p__0c__double__0p__0r>;
    #[doc = "Explicit C callback typedef `amendscan_function` with its exact native binding storage."]
    pub type amendscan_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__IndexScanDescData__0p__0r,
    >;
    #[cfg(any(feature = "pg15", feature = "pg16"))]
    #[doc = "Explicit C callback typedef `amestimateparallelscan_function` with its exact native binding storage."]
    pub type amestimateparallelscan_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_unsigned__long__0l__void__0r,
    >;
    #[doc = "Explicit C callback typedef `amgetbitmap_function` with its exact native binding storage."]
    pub type amgetbitmap_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_long__0l__struct__IndexScanDescData__0p__0c__struct__TIDBitmap__0p__0r>;
    #[doc = "Explicit C callback typedef `amgettuple_function` with its exact native binding storage."]
    pub type amgettuple_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__IndexScanDescData__0p__0c__enum__ScanDirection__0r>;
    #[doc = "Explicit C callback typedef `aminitparallelscan_function` with its exact native binding storage."]
    pub type aminitparallelscan_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__void__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `aminsert_function` with its exact native binding storage."]
    pub type aminsert_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__RelationData__0p__0c__unsigned__long__0p__0c__0x5f426f6f6c__0p__0c__struct__ItemPointerData__0p__0c__struct__RelationData__0p__0c__enum__IndexUniqueCheck__0c__0x5f426f6f6c__0c__struct__IndexInfo__0p__0r>;
    #[doc = "Explicit C callback typedef `ammarkpos_function` with its exact native binding storage."]
    pub type ammarkpos_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__IndexScanDescData__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `amoptions_function` with its exact native binding storage."]
    pub type amoptions_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__varlena__0p__0l__unsigned__long__0c__0x5f426f6f6c__0r>;
    #[doc = "Explicit C callback typedef `amparallelrescan_function` with its exact native binding storage."]
    pub type amparallelrescan_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__IndexScanDescData__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `amproperty_function` with its exact native binding storage."]
    pub type amproperty_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__unsigned__int__0c__int__0c__enum__IndexAMProperty__0c__const__char__0p__0c__0x5f426f6f6c__0p__0c__0x5f426f6f6c__0p__0r>;
    #[doc = "Explicit C callback typedef `amrescan_function` with its exact native binding storage."]
    pub type amrescan_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__IndexScanDescData__0p__0c__struct__ScanKeyData__0p__0c__int__0c__struct__ScanKeyData__0p__0c__int__0r>;
    #[doc = "Explicit C callback typedef `amrestrpos_function` with its exact native binding storage."]
    pub type amrestrpos_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__IndexScanDescData__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `amvacuumcleanup_function` with its exact native binding storage."]
    pub type amvacuumcleanup_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__IndexBulkDeleteResult__0p__0l__struct__IndexVacuumInfo__0p__0c__struct__IndexBulkDeleteResult__0p__0r>;
    #[doc = "Explicit C callback typedef `amvalidate_function` with its exact native binding storage."]
    pub type amvalidate_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__unsigned__int__0r,
    >;
    #[doc = "Explicit C callback typedef `bgworker_main_type` with its exact native binding storage."]
    pub type bgworker_main_type = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__unsigned__long__0r,
    >;
    #[doc = "Explicit C callback typedef `brin_serialize_callback_type` with its exact native binding storage."]
    pub type brin_serialize_callback_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__BrinDesc__0p__0c__unsigned__long__0c__unsigned__long__0p__0r>;
    #[doc = "Explicit C callback typedef `check_function_callback` with its exact native binding storage."]
    pub type check_function_callback = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__unsigned__int__0c__void__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `check_object_relabel_type` with its exact native binding storage."]
    pub type check_object_relabel_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__const__struct__ObjectAddress__0p__0c__const__char__0p__0r>;
    #[doc = "Explicit C callback typedef `check_password_hook_type` with its exact native binding storage."]
    pub type check_password_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__const__char__0p__0c__const__char__0p__0c__enum__PasswordType__0c__unsigned__long__0c__0x5f426f6f6c__0r>;
    #[doc = "Explicit C callback typedef `copy_data_source_cb` with its exact native binding storage."]
    pub type copy_data_source_cb = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_int__0l__void__0p__0c__int__0c__int__0r,
    >;
    #[doc = "Explicit C callback typedef `create_upper_paths_hook_type` with its exact native binding storage."]
    pub type create_upper_paths_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__PlannerInfo__0p__0c__enum__UpperRelationKind__0c__struct__RelOptInfo__0p__0c__struct__RelOptInfo__0p__0c__void__0p__0r>;
    #[doc = "Explicit C callback typedef `dshash_compare_function` with its exact native binding storage."]
    pub type dshash_compare_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_int__0l__const__void__0p__0c__const__void__0p__0c__unsigned__long__0c__void__0p__0r>;
    #[doc = "Explicit C callback typedef `dshash_hash_function` with its exact native binding storage."]
    pub type dshash_hash_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_unsigned__int__0l__const__void__0p__0c__unsigned__long__0c__void__0p__0r>;
    #[doc = "Explicit C callback typedef `ec_matches_callback_type` with its exact native binding storage."]
    pub type ec_matches_callback_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__EquivalenceClass__0p__0c__struct__EquivalenceMember__0p__0c__void__0p__0r>;
    #[doc = "Explicit C callback typedef `emit_log_hook_type` with its exact native binding storage."]
    pub type emit_log_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__ErrorData__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `explain_get_index_name_hook_type` with its exact native binding storage."]
    pub type explain_get_index_name_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_const__char__0p__0l__unsigned__int__0r,
    >;
    #[doc = "Explicit C callback typedef `fill_string_relopt` with its exact native binding storage."]
    pub type fill_string_relopt = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_unsigned__long__0l__const__char__0p__0c__void__0p__0r,
    >;
    #[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
    #[doc = "Explicit C callback typedef `fmExprContextCallbackFunction` with its exact native binding storage."]
    pub type fmExprContextCallbackFunction = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__unsigned__long__0r,
    >;
    #[doc = "Explicit C callback typedef `fmgr_hook_type` with its exact native binding storage."]
    pub type fmgr_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__enum__FmgrHookEventType__0c__struct__FmgrInfo__0p__0c__unsigned__long__0p__0r>;
    #[doc = "Explicit C callback typedef `get_attavgwidth_hook_type` with its exact native binding storage."]
    pub type get_attavgwidth_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_int__0l__unsigned__int__0c__short__0r,
    >;
    #[doc = "Explicit C callback typedef `get_index_stats_hook_type` with its exact native binding storage."]
    pub type get_index_stats_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__PlannerInfo__0p__0c__unsigned__int__0c__short__0c__struct__VariableStatData__0p__0r>;
    #[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
    #[doc = "Explicit C callback typedef `get_relation_info_hook_type` with its exact native binding storage."]
    pub type get_relation_info_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__PlannerInfo__0p__0c__unsigned__int__0c__0x5f426f6f6c__0c__struct__RelOptInfo__0p__0r>;
    #[doc = "Explicit C callback typedef `get_relation_stats_hook_type` with its exact native binding storage."]
    pub type get_relation_stats_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__PlannerInfo__0p__0c__struct__RangeTblEntry__0p__0c__short__0c__struct__VariableStatData__0p__0r>;
    #[doc = "Explicit C callback typedef `join_search_hook_type` with its exact native binding storage."]
    pub type join_search_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__RelOptInfo__0p__0l__struct__PlannerInfo__0p__0c__int__0c__struct__List__0p__0r>;
    #[doc = "Explicit C callback typedef `list_sort_comparator` with its exact native binding storage."]
    pub type list_sort_comparator = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_int__0l__const__union__ListCell__0p__0c__const__union__ListCell__0p__0r>;
    #[doc = "Explicit C callback typedef `mb2wchar_with_len_converter` with its exact native binding storage."]
    pub type mb2wchar_with_len_converter = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_int__0l__const__unsigned__char__0p__0c__unsigned__int__0p__0c__int__0r>;
    #[doc = "Explicit C callback typedef `mbchar_verifier` with its exact native binding storage."]
    pub type mbchar_verifier = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_int__0l__const__unsigned__char__0p__0c__int__0r,
    >;
    #[doc = "Explicit C callback typedef `mbcharacter_incrementer` with its exact native binding storage."]
    pub type mbcharacter_incrementer = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__unsigned__char__0p__0c__int__0r,
    >;
    #[doc = "Explicit C callback typedef `mbdisplaylen_converter` with its exact native binding storage."]
    pub type mbdisplaylen_converter = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_int__0l__const__unsigned__char__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `mblen_converter` with its exact native binding storage."]
    pub type mblen_converter = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_int__0l__const__unsigned__char__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `mbstr_verifier` with its exact native binding storage."]
    pub type mbstr_verifier = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_int__0l__const__unsigned__char__0p__0c__int__0r,
    >;
    #[doc = "Explicit C callback typedef `needs_fmgr_hook_type` with its exact native binding storage."]
    pub type needs_fmgr_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__unsigned__int__0r,
    >;
    #[doc = "Explicit C callback typedef `object_access_hook_type` with its exact native binding storage."]
    pub type object_access_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__enum__ObjectAccessType__0c__unsigned__int__0c__unsigned__int__0c__int__0c__void__0p__0r>;
    #[doc = "Explicit C callback typedef `object_access_hook_type_str` with its exact native binding storage."]
    pub type object_access_hook_type_str = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__enum__ObjectAccessType__0c__unsigned__int__0c__const__char__0p__0c__int__0c__void__0p__0r>;
    #[doc = "Explicit C callback typedef `on_dsm_detach_callback` with its exact native binding storage."]
    pub type on_dsm_detach_callback = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__dsm_segment__0p__0c__unsigned__long__0r>;
    #[doc = "Explicit C callback typedef `pairingheap_comparator` with its exact native binding storage."]
    pub type pairingheap_comparator = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_int__0l__const__struct__pairingheap_node__0p__0c__const__struct__pairingheap_node__0p__0c__void__0p__0r>;
    #[doc = "Explicit C callback typedef `parallel_worker_main_type` with its exact native binding storage."]
    pub type parallel_worker_main_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__dsm_segment__0p__0c__struct__shm_toc__0p__0r>;
    #[doc = "Explicit C callback typedef `pg_funcptr_t` with its exact native binding storage."]
    pub type pg_funcptr_t = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__void__0r,
    >;
    #[doc = "Explicit C callback typedef `pg_on_exit_callback` with its exact native binding storage."]
    pub type pg_on_exit_callback = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__int__0c__unsigned__long__0r,
    >;
    #[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
    #[doc = "Explicit C callback typedef `planner_hook_type` with its exact native binding storage."]
    pub type planner_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__PlannedStmt__0p__0l__struct__Query__0p__0c__const__char__0p__0c__int__0c__struct__ParamListInfoData__0p__0r>;
    #[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
    #[doc = "Explicit C callback typedef `post_parse_analyze_hook_type` with its exact native binding storage."]
    pub type post_parse_analyze_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ParseState__0p__0c__struct__Query__0p__0c__struct__JumbleState__0p__0r>;
    #[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
    #[doc = "Explicit C callback typedef `pqsigfunc` with its exact native binding storage."]
    pub type pqsigfunc = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__int__0r,
    >;
    #[doc = "Explicit C callback typedef `qsort_arg_comparator` with its exact native binding storage."]
    pub type qsort_arg_comparator = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_int__0l__const__void__0p__0c__const__void__0p__0c__void__0p__0r>;
    #[doc = "Explicit C callback typedef `query_pathkeys_callback` with its exact native binding storage."]
    pub type query_pathkeys_callback = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__PlannerInfo__0p__0c__void__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `rbt_allocfunc` with its exact native binding storage."]
    pub type rbt_allocfunc = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_struct__RBTNode__0p__0l__void__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `rbt_combiner` with its exact native binding storage."]
    pub type rbt_combiner = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__RBTNode__0p__0c__const__struct__RBTNode__0p__0c__void__0p__0r>;
    #[doc = "Explicit C callback typedef `rbt_comparator` with its exact native binding storage."]
    pub type rbt_comparator = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_int__0l__const__struct__RBTNode__0p__0c__const__struct__RBTNode__0p__0c__void__0p__0r>;
    #[doc = "Explicit C callback typedef `rbt_freefunc` with its exact native binding storage."]
    pub type rbt_freefunc = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__RBTNode__0p__0c__void__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `relopts_validator` with its exact native binding storage."]
    pub type relopts_validator = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__void__0p__0c__struct__relopt_value__0p__0c__int__0r>;
    #[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
    #[doc = "Explicit C callback typedef `replace_rte_variables_callback` with its exact native binding storage."]
    pub type replace_rte_variables_callback = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__Node__0p__0l__struct__Var__0p__0c__struct__replace_rte_variables_context__0p__0r>;
    #[doc = "Explicit C callback typedef `row_security_policy_hook_type` with its exact native binding storage."]
    pub type row_security_policy_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__List__0p__0l__enum__CmdType__0c__struct__RelationData__0p__0r>;
    #[doc = "Explicit C callback typedef `set_join_pathlist_hook_type` with its exact native binding storage."]
    pub type set_join_pathlist_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__RelOptInfo__0p__0c__enum__JoinType__0c__struct__JoinPathExtraData__0p__0r>;
    #[doc = "Explicit C callback typedef `set_rel_pathlist_hook_type` with its exact native binding storage."]
    pub type set_rel_pathlist_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__unsigned__int__0c__struct__RangeTblEntry__0p__0r>;
    #[doc = "Explicit C callback typedef `shmem_request_hook_type` with its exact native binding storage."]
    pub type shmem_request_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__void__0r,
    >;
    #[doc = "Explicit C callback typedef `shmem_startup_hook_type` with its exact native binding storage."]
    pub type shmem_startup_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__void__0r,
    >;
    #[doc = "Explicit C callback typedef `utf_local_conversion_func` with its exact native binding storage."]
    pub type utf_local_conversion_func = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_unsigned__int__0l__unsigned__int__0r,
    >;
    #[doc = "Explicit C callback typedef `validate_string_relopt` with its exact native binding storage."]
    pub type validate_string_relopt = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__const__char__0p__0r,
    >;
    #[cfg(feature = "pg15")]
    #[doc = "Explicit C callback typedef `walrcv_check_conninfo_fn` with its exact native binding storage."]
    pub type walrcv_check_conninfo_fn = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__const__char__0p__0r,
    >;
    #[cfg(feature = "pg15")]
    #[doc = "Explicit C callback typedef `walrcv_connect_fn` with its exact native binding storage."]
    pub type walrcv_connect_fn = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__WalReceiverConn__0p__0l__const__char__0p__0c__0x5f426f6f6c__0c__const__char__0p__0c__char__0p__0p__0r>;
    #[cfg(any(feature = "pg15", feature = "pg16"))]
    #[doc = "Explicit C callback typedef `walrcv_create_slot_fn` with its exact native binding storage."]
    pub type walrcv_create_slot_fn = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_char__0p__0l__struct__WalReceiverConn__0p__0c__const__char__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__CRSSnapshotAction__0c__unsigned__long__0p__0r>;
    #[doc = "Explicit C callback typedef `walrcv_disconnect_fn` with its exact native binding storage."]
    pub type walrcv_disconnect_fn = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__WalReceiverConn__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `walrcv_endstreaming_fn` with its exact native binding storage."]
    pub type walrcv_endstreaming_fn = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__WalReceiverConn__0p__0c__unsigned__int__0p__0r>;
    #[doc = "Explicit C callback typedef `walrcv_exec_fn` with its exact native binding storage."]
    pub type walrcv_exec_fn = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__WalRcvExecResult__0p__0l__struct__WalReceiverConn__0p__0c__const__char__0p__0c__int__0c__const__unsigned__int__0p__0r>;
    #[doc = "Explicit C callback typedef `walrcv_get_backend_pid_fn` with its exact native binding storage."]
    pub type walrcv_get_backend_pid_fn = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_int__0l__struct__WalReceiverConn__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `walrcv_get_conninfo_fn` with its exact native binding storage."]
    pub type walrcv_get_conninfo_fn = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_char__0p__0l__struct__WalReceiverConn__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `walrcv_get_senderinfo_fn` with its exact native binding storage."]
    pub type walrcv_get_senderinfo_fn = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__WalReceiverConn__0p__0c__char__0p__0p__0c__int__0p__0r>;
    #[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
    #[doc = "Explicit C callback typedef `walrcv_identify_system_fn` with its exact native binding storage."]
    pub type walrcv_identify_system_fn = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_char__0p__0l__struct__WalReceiverConn__0p__0c__unsigned__int__0p__0r>;
    #[doc = "Explicit C callback typedef `walrcv_readtimelinehistoryfile_fn` with its exact native binding storage."]
    pub type walrcv_readtimelinehistoryfile_fn = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__WalReceiverConn__0p__0c__unsigned__int__0c__char__0p__0p__0c__char__0p__0p__0c__int__0p__0r>;
    #[doc = "Explicit C callback typedef `walrcv_receive_fn` with its exact native binding storage."]
    pub type walrcv_receive_fn = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_int__0l__struct__WalReceiverConn__0p__0c__char__0p__0p__0c__int__0p__0r>;
    #[doc = "Explicit C callback typedef `walrcv_send_fn` with its exact native binding storage."]
    pub type walrcv_send_fn = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__WalReceiverConn__0p__0c__const__char__0p__0c__int__0r>;
    #[doc = "Explicit C callback typedef `walrcv_server_version_fn` with its exact native binding storage."]
    pub type walrcv_server_version_fn = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_int__0l__struct__WalReceiverConn__0p__0r,
    >;
    #[doc = "Explicit C callback typedef `walrcv_startstreaming_fn` with its exact native binding storage."]
    pub type walrcv_startstreaming_fn = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__WalReceiverConn__0p__0c__const__WalRcvStreamOptions__0p__0r>;
    #[doc = "Explicit C callback typedef `wchar2mb_with_len_converter` with its exact native binding storage."]
    pub type wchar2mb_with_len_converter = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_int__0l__const__unsigned__int__0p__0c__unsigned__char__0p__0c__int__0r>;
    #[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `ExecutorCheckPerms_hook_type` with its exact native binding storage."]
    pub type ExecutorCheckPerms_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__List__0p__0c__struct__List__0p__0c__0x5f426f6f6c__0r>;
    #[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `ReorderBufferUpdateProgressTxnCB` with its exact native binding storage."]
    pub type ReorderBufferUpdateProgressTxnCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ReorderBuffer__0p__0c__struct__ReorderBufferTXN__0p__0c__unsigned__long__0r>;
    #[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `SortTupleComparator` with its exact native binding storage."]
    pub type SortTupleComparator = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_int__0l__const__SortTuple__0p__0c__const__SortTuple__0p__0c__struct__Tuplesortstate__0p__0r>;
    #[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `copy_data_dest_cb` with its exact native binding storage."]
    pub type copy_data_dest_cb = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__void__0p__0c__int__0r,
    >;
    #[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `planstate_tree_walker_callback` with its exact native binding storage."]
    pub type planstate_tree_walker_callback = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__PlanState__0p__0c__void__0p__0r>;
    #[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `tree_mutator_callback` with its exact native binding storage."]
    pub type tree_mutator_callback = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__Node__0p__0l__struct__Node__0p__0c__void__0p__0r>;
    #[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `tree_walker_callback` with its exact native binding storage."]
    pub type tree_walker_callback = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__Node__0p__0c__void__0p__0r,
    >;
    #[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `walrcv_check_conninfo_fn` with its exact native binding storage."]
    pub type walrcv_check_conninfo_fn = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__const__char__0p__0c__0x5f426f6f6c__0r,
    >;
    #[cfg(feature = "pg16")]
    #[doc = "Explicit C callback typedef `walrcv_connect_fn` with its exact native binding storage."]
    pub type walrcv_connect_fn = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__WalReceiverConn__0p__0l__const__char__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__const__char__0p__0c__char__0p__0p__0r>;
    #[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `ReadStreamBlockNumberCB` with its exact native binding storage."]
    pub type ReadStreamBlockNumberCB = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_unsigned__int__0l__struct__ReadStream__0p__0c__void__0p__0c__void__0p__0r>;
    #[cfg(any(feature = "pg17", feature = "pg18"))]
    #[doc = "Explicit C callback typedef `SlruScanCallback` with its exact native binding storage."]
    pub type SlruScanCallback = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__SlruCtlData__0p__0c__char__0p__0c__long__0c__void__0p__0r>;
    #[cfg(feature = "pg17")]
    #[doc = "Explicit C callback typedef `amestimateparallelscan_function` with its exact native binding storage."]
    pub type amestimateparallelscan_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_unsigned__long__0l__int__0c__int__0r,
    >;
    #[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `aminsertcleanup_function` with its exact native binding storage."]
    pub type aminsertcleanup_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__RelationData__0p__0c__struct__IndexInfo__0p__0r>;
    #[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `dshash_copy_function` with its exact native binding storage."]
    pub type dshash_copy_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__void__0p__0c__const__void__0p__0c__unsigned__long__0c__void__0p__0r>;
    #[cfg(feature = "pg17")]
    #[doc = "Explicit C callback typedef `walrcv_alter_slot_fn` with its exact native binding storage."]
    pub type walrcv_alter_slot_fn = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__WalReceiverConn__0p__0c__const__char__0p__0c__0x5f426f6f6c__0r>;
    #[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `walrcv_connect_fn` with its exact native binding storage."]
    pub type walrcv_connect_fn = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__WalReceiverConn__0p__0l__const__char__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__const__char__0p__0c__char__0p__0p__0r>;
    #[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `walrcv_create_slot_fn` with its exact native binding storage."]
    pub type walrcv_create_slot_fn = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_char__0p__0l__struct__WalReceiverConn__0p__0c__const__char__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__CRSSnapshotAction__0c__unsigned__long__0p__0r>;
    #[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `walrcv_get_dbname_from_conninfo_fn` with its exact native binding storage."]
    pub type walrcv_get_dbname_from_conninfo_fn = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_char__0p__0l__const__char__0p__0r,
    >;
    #[cfg(any(feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `CachedFunctionCompileCallback` with its exact native binding storage."]
    pub type CachedFunctionCompileCallback = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__FunctionCallInfoBaseData__0p__0c__struct__HeapTupleData__0p__0c__const__struct__CachedFunctionHashKey__0p__0c__struct__CachedFunction__0p__0c__0x5f426f6f6c__0r>;
    #[cfg(any(feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `CachedFunctionDeleteCallback` with its exact native binding storage."]
    pub type CachedFunctionDeleteCallback = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__CachedFunction__0p__0r,
    >;
    #[cfg(feature = "pg18")]
    #[doc = "Explicit C callback typedef `ChangeVarNodes_callback` with its exact native binding storage."]
    pub type ChangeVarNodes_callback = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__Node__0p__0c__struct__ChangeVarNodes_context__0p__0r>;
    #[cfg(any(feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `ExecutorRun_hook_type` with its exact native binding storage."]
    pub type ExecutorRun_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__QueryDesc__0p__0c__enum__ScanDirection__0c__unsigned__long__0r>;
    #[cfg(any(feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `ExplainOptionHandler` with its exact native binding storage."]
    pub type ExplainOptionHandler = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ExplainState__0p__0c__struct__DefElem__0p__0c__struct__ParseState__0p__0r>;
    #[cfg(any(feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `PgAioHandleCallbackComplete` with its exact native binding storage."]
    pub type PgAioHandleCallbackComplete = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__PgAioResult__0l__struct__PgAioHandle__0p__0c__struct__PgAioResult__0c__unsigned__char__0r>;
    #[cfg(any(feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `PgAioHandleCallbackReport` with its exact native binding storage."]
    pub type PgAioHandleCallbackReport = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__PgAioResult__0c__const__union__PgAioTargetData__0p__0c__int__0r>;
    #[cfg(any(feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `PgAioHandleCallbackStage` with its exact native binding storage."]
    pub type PgAioHandleCallbackStage = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__PgAioHandle__0p__0c__unsigned__char__0r>;
    #[cfg(any(feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `PostRewriteHook` with its exact native binding storage."]
    pub type PostRewriteHook = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__struct__List__0p__0c__void__0p__0r,
    >;
    #[cfg(any(feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `RelSyncCallbackFunction` with its exact native binding storage."]
    pub type RelSyncCallbackFunction = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__unsigned__long__0c__unsigned__int__0r,
    >;
    #[cfg(any(feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `SkipSupportIncDec` with its exact native binding storage."]
    pub type SkipSupportIncDec = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_unsigned__long__0l__struct__RelationData__0p__0c__unsigned__long__0c__0x5f426f6f6c__0p__0r>;
    #[cfg(any(feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `amestimateparallelscan_function` with its exact native binding storage."]
    pub type amestimateparallelscan_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_unsigned__long__0l__struct__RelationData__0p__0c__int__0c__int__0r>;
    #[cfg(any(feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `amgettreeheight_function` with its exact native binding storage."]
    pub type amgettreeheight_function = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_int__0l__struct__RelationData__0p__0r,
    >;
    #[cfg(any(feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `amtranslate_cmptype_function` with its exact native binding storage."]
    pub type amtranslate_cmptype_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_unsigned__short__0l__enum__CompareType__0c__unsigned__int__0r>;
    #[cfg(any(feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `amtranslate_strategy_function` with its exact native binding storage."]
    pub type amtranslate_strategy_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_enum__CompareType__0l__unsigned__short__0c__unsigned__int__0r>;
    #[cfg(any(feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `explain_per_node_hook_type` with its exact native binding storage."]
    pub type explain_per_node_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__PlanState__0p__0c__struct__List__0p__0c__const__char__0p__0c__const__char__0p__0c__struct__ExplainState__0p__0r>;
    #[cfg(any(feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `explain_per_plan_hook_type` with its exact native binding storage."]
    pub type explain_per_plan_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__PlannedStmt__0p__0c__struct__IntoClause__0p__0c__struct__ExplainState__0p__0c__const__char__0p__0c__struct__ParamListInfoData__0p__0c__struct__QueryEnvironment__0p__0r>;
    #[cfg(any(feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `explain_validate_options_hook_type` with its exact native binding storage."]
    pub type explain_validate_options_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ExplainState__0p__0c__struct__List__0p__0c__struct__ParseState__0p__0r>;
    #[cfg(any(feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `grouping_eqop_callback` with its exact native binding storage."]
    pub type grouping_eqop_callback = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_unsigned__int__0l__struct__Var__0p__0c__void__0p__0r,
    >;
    #[cfg(any(feature = "pg18", feature = "pg19"))]
    #[doc = "Explicit C callback typedef `walrcv_alter_slot_fn` with its exact native binding storage."]
    pub type walrcv_alter_slot_fn = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__WalReceiverConn__0p__0c__const__char__0p__0c__const__0x5f426f6f6c__0p__0c__const__0x5f426f6f6c__0p__0r>;
    #[cfg(feature = "pg19")]
    #[doc = "Explicit C callback typedef `ConnectionWarningFilter` with its exact native binding storage."]
    pub type ConnectionWarningFilter = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__void__0r,
    >;
    #[cfg(feature = "pg19")]
    #[doc = "Explicit C callback typedef `ExplainOptionGUCCheckHandler` with its exact native binding storage."]
    pub type ExplainOptionGUCCheckHandler = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__const__char__0p__0c__const__char__0p__0c__enum__NodeTag__0r>;
    #[cfg(feature = "pg19")]
    #[doc = "Explicit C callback typedef `HashAllocFunc` with its exact native binding storage."]
    pub type HashAllocFunc = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0p__0l__unsigned__long__0c__void__0p__0r,
    >;
    #[cfg(feature = "pg19")]
    #[doc = "Explicit C callback typedef `ImportForeignStatistics_function` with its exact native binding storage."]
    pub type ImportForeignStatistics_function = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__RelationData__0p__0c__struct__List__0p__0c__int__0r>;
    #[cfg(feature = "pg19")]
    #[doc = "Explicit C callback typedef `PushFunction` with its exact native binding storage."]
    pub type PushFunction = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__void__0p__0c__struct__TSQueryParserStateData__0p__0c__char__0p__0c__int__0c__short__0c__0x5f426f6f6c__0r>;
    #[cfg(feature = "pg19")]
    #[doc = "Explicit C callback typedef `ShmemAttachCallback` with its exact native binding storage."]
    pub type ShmemAttachCallback = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__void__0p__0r,
    >;
    #[cfg(feature = "pg19")]
    #[doc = "Explicit C callback typedef `ShmemInitCallback` with its exact native binding storage."]
    pub type ShmemInitCallback = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__void__0p__0r,
    >;
    #[cfg(feature = "pg19")]
    #[doc = "Explicit C callback typedef `ShmemRequestCallback` with its exact native binding storage."]
    pub type ShmemRequestCallback = crate::__pgrx_c_macros::expression::FunctionValue<
        crate::__pgrx_c_generated::Signature_void__0l__void__0p__0r,
    >;
    #[cfg(feature = "pg19")]
    #[doc = "Explicit C callback typedef `SlruScanCallback` with its exact native binding storage."]
    pub type SlruScanCallback = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__SlruDesc__0p__0c__char__0p__0c__long__0c__void__0p__0r>;
    #[cfg(feature = "pg19")]
    #[doc = "Explicit C callback typedef `SyscacheCallbackFunction` with its exact native binding storage."]
    pub type SyscacheCallbackFunction = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__unsigned__long__0c__enum__SysCacheIdentifier__0c__unsigned__int__0r>;
    #[cfg(feature = "pg19")]
    #[doc = "Explicit C callback typedef `TwoPhaseCallback` with its exact native binding storage."]
    pub type TwoPhaseCallback = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__FullTransactionId__0c__unsigned__short__0c__void__0p__0c__unsigned__int__0r>;
    #[cfg(feature = "pg19")]
    #[doc = "Explicit C callback typedef `build_simple_rel_hook_type` with its exact native binding storage."]
    pub type build_simple_rel_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__RangeTblEntry__0p__0r>;
    #[cfg(feature = "pg19")]
    #[doc = "Explicit C callback typedef `join_path_setup_hook_type` with its exact native binding storage."]
    pub type join_path_setup_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__RelOptInfo__0p__0c__enum__JoinType__0c__struct__JoinPathExtraData__0p__0r>;
    #[cfg(feature = "pg19")]
    #[doc = "Explicit C callback typedef `joinrel_setup_hook_type` with its exact native binding storage."]
    pub type joinrel_setup_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__SpecialJoinInfo__0p__0c__struct__List__0p__0r>;
    #[cfg(feature = "pg19")]
    #[doc = "Explicit C callback typedef `planner_hook_type` with its exact native binding storage."]
    pub type planner_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__PlannedStmt__0p__0l__struct__Query__0p__0c__const__char__0p__0c__int__0c__struct__ParamListInfoData__0p__0c__struct__ExplainState__0p__0r>;
    #[cfg(feature = "pg19")]
    #[doc = "Explicit C callback typedef `planner_setup_hook_type` with its exact native binding storage."]
    pub type planner_setup_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__PlannerGlobal__0p__0c__struct__Query__0p__0c__const__char__0p__0c__int__0c__double__0p__0c__struct__ExplainState__0p__0r>;
    #[cfg(feature = "pg19")]
    #[doc = "Explicit C callback typedef `planner_shutdown_hook_type` with its exact native binding storage."]
    pub type planner_shutdown_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__PlannerGlobal__0p__0c__struct__Query__0p__0c__const__char__0p__0c__struct__PlannedStmt__0p__0r>;
    #[cfg(feature = "pg19")]
    #[doc = "Explicit C callback typedef `post_parse_analyze_hook_type` with its exact native binding storage."]
    pub type post_parse_analyze_hook_type = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__struct__ParseState__0p__0c__struct__Query__0p__0c__const__struct__JumbleState__0p__0r>;
    #[cfg(feature = "pg19")]
    #[doc = "Explicit C callback typedef `pqsigfunc` with its exact native binding storage."]
    pub type pqsigfunc = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_void__0l__int__0c__const__struct__pg_signal_info__0p__0r>;
    #[cfg(feature = "pg19")]
    #[doc = "Explicit C callback typedef `replace_rte_variables_callback` with its exact native binding storage."]
    pub type replace_rte_variables_callback = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_struct__Node__0p__0l__const__struct__Var__0p__0c__struct__replace_rte_variables_context__0p__0r>;
    #[cfg(feature = "pg19")]
    #[doc = "Explicit C callback typedef `walrcv_identify_system_fn` with its exact native binding storage."]
    pub type walrcv_identify_system_fn = crate::__pgrx_c_macros::expression::FunctionValue<crate::__pgrx_c_generated::Signature_char__0p__0l__struct__WalReceiverConn__0p__0c__unsigned__int__0p__0c__unsigned__long__0p__0r>;
}
#[doc(hidden)]
pub struct Field_N;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BlockSamplerData>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BlockSamplerData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BlockSamplerData {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BlockSamplerData, N) == 0);
const _: () = {
    assert!(::core::mem::size_of::<u32>() == 4);
    assert!(::core::mem::align_of::<u32>() == 4);
};
use c::expression::{ReadOnly as FieldReadOnly, ReadWrite as FieldReadWrite};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_N>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BlockSamplerData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
use c::expression::FieldProjection as Projection;
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BlockSamplerData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BlockSamplerData> as c::expression::OrdinaryField<Field_N>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).N) };
#[doc(hidden)]
pub struct Field_n;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BlockSamplerData, n) == 4);
const _: () = {
    assert!(::core::mem::size_of::<i32>() == 4);
    assert!(::core::mem::align_of::<i32>() == 4);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_n>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BlockSamplerData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BlockSamplerData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BlockSamplerData> as c::expression::OrdinaryField<Field_n>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).n) };
#[doc(hidden)]
pub struct Field_t;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BlockSamplerData, t) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_t>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BlockSamplerData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BlockSamplerData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BlockSamplerData> as c::expression::OrdinaryField<Field_t>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).t) };
#[doc(hidden)]
pub struct Field_m;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BlockSamplerData, m) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_m>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BlockSamplerData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BlockSamplerData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BlockSamplerData> as c::expression::OrdinaryField<Field_m>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).m) };
#[doc(hidden)]
pub struct Field_allocatedMemory;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BuildAccumulator>() == 72);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BuildAccumulator>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BuildAccumulator {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BuildAccumulator, allocatedMemory) == 8
);
const _: () = {
    assert!(::core::mem::size_of::<usize>() == 8);
    assert!(::core::mem::align_of::<usize>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_allocatedMemory>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BuildAccumulator>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BuildAccumulator, <<c::expression::CRecord<crate::__pgrx_c_bindings::BuildAccumulator> as c::expression::OrdinaryField<Field_allocatedMemory>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).allocatedMemory) };
#[doc(hidden)]
pub struct Field_eas_used;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BuildAccumulator, eas_used) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_eas_used>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BuildAccumulator>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BuildAccumulator, <<c::expression::CRecord<crate::__pgrx_c_bindings::BuildAccumulator> as c::expression::OrdinaryField<Field_eas_used>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).eas_used) };
#[doc(hidden)]
pub struct Field_mutex;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ConditionVariable>() == 12);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ConditionVariable>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ConditionVariable {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ConditionVariable, mutex) == 0);
const _: () = {
    assert!(::core::mem::size_of::<u8>() == 1);
    assert!(::core::mem::align_of::<u8>() == 1);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mutex>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ConditionVariable>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ConditionVariable, <<c::expression::CRecord<crate::__pgrx_c_bindings::ConditionVariable> as c::expression::OrdinaryField<Field_mutex>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mutex) };
#[doc(hidden)]
pub struct Field_in_use;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::DecodedBkpBlock>() == 64);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::DecodedBkpBlock>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::DecodedBkpBlock {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodedBkpBlock, in_use) == 0);
const _: () = {
    assert!(::core::mem::size_of::<bool>() == 1);
    assert!(::core::mem::align_of::<bool>() == 1);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_in_use>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodedBkpBlock, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock> as c::expression::OrdinaryField<Field_in_use>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).in_use) };
#[doc(hidden)]
pub struct Field_forknum;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodedBkpBlock, forknum) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_forknum>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ForkNumber,
        c::CInt,
        i32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodedBkpBlock, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock> as c::expression::OrdinaryField<Field_forknum>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).forknum) };
#[doc(hidden)]
pub struct Field_blkno;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodedBkpBlock, blkno) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_blkno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodedBkpBlock, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock> as c::expression::OrdinaryField<Field_blkno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).blkno) };
#[doc(hidden)]
pub struct Field_prefetch_buffer;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodedBkpBlock, prefetch_buffer) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_prefetch_buffer>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodedBkpBlock, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock> as c::expression::OrdinaryField<Field_prefetch_buffer>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).prefetch_buffer) };
#[doc(hidden)]
pub struct Field_flags;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodedBkpBlock, flags) == 28);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodedBkpBlock, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock> as c::expression::OrdinaryField<Field_flags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).flags) };
#[doc(hidden)]
pub struct Field_has_image;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodedBkpBlock, has_image) == 29);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_has_image>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 29;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodedBkpBlock, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock> as c::expression::OrdinaryField<Field_has_image>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).has_image) };
#[doc(hidden)]
pub struct Field_apply_image;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodedBkpBlock, apply_image) == 30);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_apply_image>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 30;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodedBkpBlock, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock> as c::expression::OrdinaryField<Field_apply_image>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).apply_image) };
#[doc(hidden)]
pub struct Field_hole_offset;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodedBkpBlock, hole_offset) == 40);
const _: () = {
    assert!(::core::mem::size_of::<u16>() == 2);
    assert!(::core::mem::align_of::<u16>() == 2);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hole_offset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodedBkpBlock, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock> as c::expression::OrdinaryField<Field_hole_offset>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hole_offset) };
#[doc(hidden)]
pub struct Field_hole_length;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodedBkpBlock, hole_length) == 42);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hole_length>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 42;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodedBkpBlock, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock> as c::expression::OrdinaryField<Field_hole_length>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hole_length) };
#[doc(hidden)]
pub struct Field_bimg_len;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodedBkpBlock, bimg_len) == 44);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bimg_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodedBkpBlock, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock> as c::expression::OrdinaryField<Field_bimg_len>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).bimg_len) };
#[doc(hidden)]
pub struct Field_bimg_info;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodedBkpBlock, bimg_info) == 46);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bimg_info>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 46;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodedBkpBlock, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock> as c::expression::OrdinaryField<Field_bimg_info>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).bimg_info) };
#[doc(hidden)]
pub struct Field_has_data;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodedBkpBlock, has_data) == 47);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_has_data>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 47;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodedBkpBlock, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock> as c::expression::OrdinaryField<Field_has_data>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).has_data) };
#[doc(hidden)]
pub struct Field_data;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodedBkpBlock, data) == 48);
const _: () = {
    assert!(::core::mem::size_of::<*mut ::core::ffi::c_char>() == 8);
    assert!(::core::mem::align_of::<*mut ::core::ffi::c_char>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_data>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock>
{
    type Member = c::expression::CPointer<
        c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodedBkpBlock, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock> as c::expression::OrdinaryField<Field_data>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).data) };
#[doc(hidden)]
pub struct Field_data_len;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodedBkpBlock, data_len) == 56);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_data_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodedBkpBlock, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock> as c::expression::OrdinaryField<Field_data_len>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).data_len) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
pub struct Field_data_bufsz;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DecodedBkpBlock, data_bufsz) == 58);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_data_bufsz>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 58;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DecodedBkpBlock, <<c::expression::CRecord<crate::__pgrx_c_bindings::DecodedBkpBlock> as c::expression::OrdinaryField<Field_data_bufsz>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).data_bufsz) };
#[doc(hidden)]
pub struct Field_isend;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::DictSubState>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::DictSubState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::DictSubState {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DictSubState, isend) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isend>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DictSubState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DictSubState, <<c::expression::CRecord<crate::__pgrx_c_bindings::DictSubState> as c::expression::OrdinaryField<Field_isend>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isend) };
#[doc(hidden)]
pub struct Field_getnext;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DictSubState, getnext) == 1);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_getnext>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DictSubState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DictSubState, <<c::expression::CRecord<crate::__pgrx_c_bindings::DictSubState> as c::expression::OrdinaryField<Field_getnext>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).getnext) };
#[doc(hidden)]
pub struct Field_lastRec;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo>() == 80);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::EndOfWalRecoveryInfo {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::EndOfWalRecoveryInfo, lastRec) == 0);
const _: () = {
    assert!(::core::mem::size_of::<u64>() == 8);
    assert!(::core::mem::align_of::<u64>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lastRec>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo> as c::expression::OrdinaryField<Field_lastRec>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lastRec) };
#[doc(hidden)]
pub struct Field_lastRecTLI;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EndOfWalRecoveryInfo, lastRecTLI) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lastRecTLI>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo> as c::expression::OrdinaryField<Field_lastRecTLI>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lastRecTLI) };
#[doc(hidden)]
pub struct Field_endOfLog;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EndOfWalRecoveryInfo, endOfLog) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_endOfLog>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo> as c::expression::OrdinaryField<Field_endOfLog>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).endOfLog) };
#[doc(hidden)]
pub struct Field_endOfLogTLI;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EndOfWalRecoveryInfo, endOfLogTLI) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_endOfLogTLI>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo> as c::expression::OrdinaryField<Field_endOfLogTLI>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).endOfLogTLI) };
#[doc(hidden)]
pub struct Field_lastPageBeginPtr;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EndOfWalRecoveryInfo, lastPageBeginPtr) == 32
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lastPageBeginPtr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo> as c::expression::OrdinaryField<Field_lastPageBeginPtr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lastPageBeginPtr) };
#[doc(hidden)]
pub struct Field_abortedRecPtr;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EndOfWalRecoveryInfo, abortedRecPtr) == 48
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_abortedRecPtr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo> as c::expression::OrdinaryField<Field_abortedRecPtr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).abortedRecPtr) };
#[doc(hidden)]
pub struct Field_missingContrecPtr;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::EndOfWalRecoveryInfo, missingContrecPtr)
        == 56
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_missingContrecPtr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo> as c::expression::OrdinaryField<Field_missingContrecPtr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).missingContrecPtr) };
#[doc(hidden)]
pub struct Field_standby_signal_file_found;
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::EndOfWalRecoveryInfo,
        standby_signal_file_found
    ) == 72
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_standby_signal_file_found>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo> as c::expression::OrdinaryField<Field_standby_signal_file_found>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).standby_signal_file_found) };
#[doc(hidden)]
pub struct Field_recovery_signal_file_found;
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::EndOfWalRecoveryInfo,
        recovery_signal_file_found
    ) == 73
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_recovery_signal_file_found>
    for c::expression::CRecord<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 73;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::EndOfWalRecoveryInfo> as c::expression::OrdinaryField<Field_recovery_signal_file_found>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).recovery_signal_file_found) };
#[doc(hidden)]
pub struct Field_fp_next_slot;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::FSMPageData>() == 4);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::FSMPageData>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::FSMPageData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::FSMPageData, fp_next_slot) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fp_next_slot>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FSMPageData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FSMPageData, <<c::expression::CRecord<crate::__pgrx_c_bindings::FSMPageData> as c::expression::OrdinaryField<Field_fp_next_slot>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fp_next_slot) };
#[doc(hidden)]
pub struct Field_limit_needed;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::FinalPathExtraData>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::FinalPathExtraData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::FinalPathExtraData {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FinalPathExtraData, limit_needed) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_limit_needed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FinalPathExtraData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FinalPathExtraData, <<c::expression::CRecord<crate::__pgrx_c_bindings::FinalPathExtraData> as c::expression::OrdinaryField<Field_limit_needed>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).limit_needed) };
#[doc(hidden)]
pub struct Field_count_est;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::FinalPathExtraData, count_est) == 16);
const _: () = {
    assert!(::core::mem::size_of::<i64>() == 8);
    assert!(::core::mem::align_of::<i64>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_count_est>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FinalPathExtraData>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FinalPathExtraData, <<c::expression::CRecord<crate::__pgrx_c_bindings::FinalPathExtraData> as c::expression::OrdinaryField<Field_count_est>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).count_est) };
#[doc(hidden)]
pub struct Field_offset_est;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FinalPathExtraData, offset_est) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_offset_est>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FinalPathExtraData>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FinalPathExtraData, <<c::expression::CRecord<crate::__pgrx_c_bindings::FinalPathExtraData> as c::expression::OrdinaryField<Field_offset_est>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).offset_est) };
#[doc(hidden)]
pub struct Field_freespace;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::GISTInsertState>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::GISTInsertState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::GISTInsertState {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GISTInsertState, freespace) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_freespace>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GISTInsertState>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::GISTInsertState, <<c::expression::CRecord<crate::__pgrx_c_bindings::GISTInsertState> as c::expression::OrdinaryField<Field_freespace>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).freespace) };
#[doc(hidden)]
pub struct Field_is_build;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GISTInsertState, is_build) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_build>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GISTInsertState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::GISTInsertState, <<c::expression::CRecord<crate::__pgrx_c_bindings::GISTInsertState> as c::expression::OrdinaryField<Field_is_build>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_build) };
#[doc(hidden)]
pub struct Field_nodeBlocknum;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::GISTNodeBuffer>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::GISTNodeBuffer>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::GISTNodeBuffer {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GISTNodeBuffer, nodeBlocknum) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nodeBlocknum>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GISTNodeBuffer>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::GISTNodeBuffer, <<c::expression::CRecord<crate::__pgrx_c_bindings::GISTNodeBuffer> as c::expression::OrdinaryField<Field_nodeBlocknum>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nodeBlocknum) };
#[doc(hidden)]
pub struct Field_blocksCount;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GISTNodeBuffer, blocksCount) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_blocksCount>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GISTNodeBuffer>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::GISTNodeBuffer, <<c::expression::CRecord<crate::__pgrx_c_bindings::GISTNodeBuffer> as c::expression::OrdinaryField<Field_blocksCount>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).blocksCount) };
#[doc(hidden)]
pub struct Field_pageBlocknum;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GISTNodeBuffer, pageBlocknum) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pageBlocknum>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GISTNodeBuffer>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::GISTNodeBuffer, <<c::expression::CRecord<crate::__pgrx_c_bindings::GISTNodeBuffer> as c::expression::OrdinaryField<Field_pageBlocknum>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pageBlocknum) };
#[doc(hidden)]
pub struct Field_queuedForEmptying;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::GISTNodeBuffer, queuedForEmptying) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_queuedForEmptying>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GISTNodeBuffer>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::GISTNodeBuffer, <<c::expression::CRecord<crate::__pgrx_c_bindings::GISTNodeBuffer> as c::expression::OrdinaryField<Field_queuedForEmptying>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).queuedForEmptying) };
#[doc(hidden)]
pub struct Field_isTemp;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GISTNodeBuffer, isTemp) == 25);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isTemp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GISTNodeBuffer>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 25;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::GISTNodeBuffer, <<c::expression::CRecord<crate::__pgrx_c_bindings::GISTNodeBuffer> as c::expression::OrdinaryField<Field_isTemp>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isTemp) };
#[doc(hidden)]
pub struct Field_level;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GISTNodeBuffer, level) == 28);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_level>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GISTNodeBuffer>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::GISTNodeBuffer, <<c::expression::CRecord<crate::__pgrx_c_bindings::GISTNodeBuffer> as c::expression::OrdinaryField<Field_level>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).level) };
#[doc(hidden)]
pub struct Field_prev;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::GISTNodeBufferPage>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::GISTNodeBufferPage>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::GISTNodeBufferPage {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GISTNodeBufferPage, prev) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_prev>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GISTNodeBufferPage>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::GISTNodeBufferPage, <<c::expression::CRecord<crate::__pgrx_c_bindings::GISTNodeBufferPage> as c::expression::OrdinaryField<Field_prev>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).prev) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GISTNodeBufferPage, freespace) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_freespace>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GISTNodeBufferPage>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::GISTNodeBufferPage, <<c::expression::CRecord<crate::__pgrx_c_bindings::GISTNodeBufferPage> as c::expression::OrdinaryField<Field_freespace>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).freespace) };
#[doc(hidden)]
pub struct Field_tupledata;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GISTNodeBufferPage, tupledata) == 8);
impl c::expression::OffsetField<Field_tupledata>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GISTNodeBufferPage>
{
    type Member = ();
    const OFFSET: usize =
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::GISTNodeBufferPage, tupledata);
}
#[doc(hidden)]
pub struct Field_buf;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::GISTPageSplitInfo>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::GISTPageSplitInfo>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::GISTPageSplitInfo {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GISTPageSplitInfo, buf) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_buf>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GISTPageSplitInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::GISTPageSplitInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::GISTPageSplitInfo> as c::expression::OrdinaryField<Field_buf>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).buf) };
#[doc(hidden)]
pub struct Field_nitem;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::GinBtreeDataLeafInsertData>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::GinBtreeDataLeafInsertData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::GinBtreeDataLeafInsertData {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::GinBtreeDataLeafInsertData, nitem) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nitem>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GinBtreeDataLeafInsertData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::GinBtreeDataLeafInsertData, <<c::expression::CRecord<crate::__pgrx_c_bindings::GinBtreeDataLeafInsertData> as c::expression::OrdinaryField<Field_nitem>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nitem) };
#[doc(hidden)]
pub struct Field_curitem;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::GinBtreeDataLeafInsertData, curitem) == 12
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_curitem>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GinBtreeDataLeafInsertData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::GinBtreeDataLeafInsertData, <<c::expression::CRecord<crate::__pgrx_c_bindings::GinBtreeDataLeafInsertData> as c::expression::OrdinaryField<Field_curitem>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).curitem) };
#[doc(hidden)]
pub struct Field_isDelete;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::GinBtreeEntryInsertData>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::GinBtreeEntryInsertData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::GinBtreeEntryInsertData {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::GinBtreeEntryInsertData, isDelete) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isDelete>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GinBtreeEntryInsertData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::GinBtreeEntryInsertData, <<c::expression::CRecord<crate::__pgrx_c_bindings::GinBtreeEntryInsertData> as c::expression::OrdinaryField<Field_isDelete>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isDelete) };
#[doc(hidden)]
pub struct Field_nbytes;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::GinPostingList>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::GinPostingList>() == 2);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::GinPostingList {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GinPostingList, nbytes) == 6);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nbytes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GinPostingList>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 6;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::GinPostingList, <<c::expression::CRecord<crate::__pgrx_c_bindings::GinPostingList> as c::expression::OrdinaryField<Field_nbytes>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nbytes) };
#[doc(hidden)]
pub struct Field_bytes;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GinPostingList, bytes) == 8);
impl c::expression::OffsetField<Field_bytes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GinPostingList>
{
    type Member = ();
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::GinPostingList, bytes);
}
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::GistEntryVector>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::GistEntryVector>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::GistEntryVector {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GistEntryVector, n) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_n>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GistEntryVector>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::GistEntryVector, <<c::expression::CRecord<crate::__pgrx_c_bindings::GistEntryVector> as c::expression::OrdinaryField<Field_n>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).n) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::GroupPathExtraData>() == 120);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::GroupPathExtraData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::GroupPathExtraData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GroupPathExtraData, flags) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GroupPathExtraData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::GroupPathExtraData, <<c::expression::CRecord<crate::__pgrx_c_bindings::GroupPathExtraData> as c::expression::OrdinaryField<Field_flags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).flags) };
#[doc(hidden)]
pub struct Field_partial_costs_set;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::GroupPathExtraData, partial_costs_set) == 4
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_partial_costs_set>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GroupPathExtraData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::GroupPathExtraData, <<c::expression::CRecord<crate::__pgrx_c_bindings::GroupPathExtraData> as c::expression::OrdinaryField<Field_partial_costs_set>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).partial_costs_set) };
#[doc(hidden)]
pub struct Field_target_parallel_safe;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::GroupPathExtraData, target_parallel_safe)
        == 88
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_target_parallel_safe>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GroupPathExtraData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::GroupPathExtraData, <<c::expression::CRecord<crate::__pgrx_c_bindings::GroupPathExtraData> as c::expression::OrdinaryField<Field_target_parallel_safe>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).target_parallel_safe) };
#[doc(hidden)]
pub struct Field_targetList;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::GroupPathExtraData, targetList) == 104
);
const _: () = {
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::List>() == 8);
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::List>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_targetList>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GroupPathExtraData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::GroupPathExtraData, <<c::expression::CRecord<crate::__pgrx_c_bindings::GroupPathExtraData> as c::expression::OrdinaryField<Field_targetList>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).targetList) };
#[doc(hidden)]
pub struct Field_patype;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GroupPathExtraData, patype) == 112);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_patype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GroupPathExtraData>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_PartitionwiseAggregateType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::GroupPathExtraData, <<c::expression::CRecord<crate::__pgrx_c_bindings::GroupPathExtraData> as c::expression::OrdinaryField<Field_patype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).patype) };
#[doc(hidden)]
pub struct Field_curBucket;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::HASH_SEQ_STATUS>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::HASH_SEQ_STATUS>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::HASH_SEQ_STATUS {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HASH_SEQ_STATUS, curBucket) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_curBucket>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HASH_SEQ_STATUS>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HASH_SEQ_STATUS, <<c::expression::CRecord<crate::__pgrx_c_bindings::HASH_SEQ_STATUS> as c::expression::OrdinaryField<Field_curBucket>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).curBucket) };
#[doc(hidden)]
pub struct Field_lenwords;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::HeadlineParsedText>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::HeadlineParsedText>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::HeadlineParsedText {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeadlineParsedText, lenwords) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lenwords>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeadlineParsedText>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HeadlineParsedText, <<c::expression::CRecord<crate::__pgrx_c_bindings::HeadlineParsedText> as c::expression::OrdinaryField<Field_lenwords>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lenwords) };
#[doc(hidden)]
pub struct Field_curwords;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeadlineParsedText, curwords) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_curwords>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeadlineParsedText>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HeadlineParsedText, <<c::expression::CRecord<crate::__pgrx_c_bindings::HeadlineParsedText> as c::expression::OrdinaryField<Field_curwords>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).curwords) };
#[doc(hidden)]
pub struct Field_vectorpos;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeadlineParsedText, vectorpos) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vectorpos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeadlineParsedText>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HeadlineParsedText, <<c::expression::CRecord<crate::__pgrx_c_bindings::HeadlineParsedText> as c::expression::OrdinaryField<Field_vectorpos>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).vectorpos) };
#[doc(hidden)]
pub struct Field_startsellen;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HeadlineParsedText, startsellen) == 48
);
const _: () = {
    assert!(::core::mem::size_of::<i16>() == 2);
    assert!(::core::mem::align_of::<i16>() == 2);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_startsellen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeadlineParsedText>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HeadlineParsedText, <<c::expression::CRecord<crate::__pgrx_c_bindings::HeadlineParsedText> as c::expression::OrdinaryField<Field_startsellen>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).startsellen) };
#[doc(hidden)]
pub struct Field_stopsellen;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HeadlineParsedText, stopsellen) == 50
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stopsellen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeadlineParsedText>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 50;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HeadlineParsedText, <<c::expression::CRecord<crate::__pgrx_c_bindings::HeadlineParsedText> as c::expression::OrdinaryField<Field_stopsellen>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stopsellen) };
#[doc(hidden)]
pub struct Field_fragdelimlen;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HeadlineParsedText, fragdelimlen) == 52
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fragdelimlen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeadlineParsedText>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HeadlineParsedText, <<c::expression::CRecord<crate::__pgrx_c_bindings::HeadlineParsedText> as c::expression::OrdinaryField<Field_fragdelimlen>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fragdelimlen) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::HeadlineWordEntry>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::HeadlineWordEntry>() == 8);
};
#[doc(hidden)]
pub struct Field_selected;
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::HeadlineWordEntry {}
const _: () = {
    assert!(
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::HeadlineWordEntry, _bitfield_1) * 8 + 0
            == 0
    );
};
unsafe extern "C" {
    fn __pgrx_bitfield__get__HeadlineWordEntry__selected(
        p: *const crate::__pgrx_c_bindings::HeadlineWordEntry,
    ) -> u32;
    fn __pgrx_bitfield__set__HeadlineWordEntry__selected(
        p: *mut crate::__pgrx_c_bindings::HeadlineWordEntry,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_volatile__HeadlineWordEntry__selected(
        p: *const crate::__pgrx_c_bindings::HeadlineWordEntry,
    ) -> u32;
    fn __pgrx_bitfield__set_volatile__HeadlineWordEntry__selected(
        p: *mut crate::__pgrx_c_bindings::HeadlineWordEntry,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned__HeadlineWordEntry__selected(
        p: *const crate::__pgrx_c_bindings::HeadlineWordEntry,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned__HeadlineWordEntry__selected(
        p: *mut crate::__pgrx_c_bindings::HeadlineWordEntry,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned_volatile__HeadlineWordEntry__selected(
        p: *const crate::__pgrx_c_bindings::HeadlineWordEntry,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned_volatile__HeadlineWordEntry__selected(
        p: *mut crate::__pgrx_c_bindings::HeadlineWordEntry,
        value: u32,
    ) -> u32;
}
#[doc(hidden)]
pub struct Bitfield_HeadlineWordEntry__selected<Q: c::expression::Qualifier> {
    address: Q::Raw<crate::__pgrx_c_bindings::HeadlineWordEntry>,
    access: c::expression::Access,
}
impl<Q: c::expression::Qualifier> Copy for Bitfield_HeadlineWordEntry__selected<Q> {}
impl<Q: c::expression::Qualifier> Clone for Bitfield_HeadlineWordEntry__selected<Q> {
    fn clone(&self) -> Self {
        *self
    }
}
// SAFETY: Clang and bindgen layout witnesses agree. This projects the record address without a reference or record read.
unsafe impl<Q: c::expression::Qualifier> c::expression::Field<Field_selected, Q>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeadlineWordEntry>
{
    type Output = Bitfield_HeadlineWordEntry__selected<Q>;
    unsafe fn project(base: c::expression::Place<Self, Q>) -> Self::Output {
        Bitfield_HeadlineWordEntry__selected::<Q> {
            address: <Q as c::expression::Qualifier>::from_mut(base.pointer().as_mut_address()),
            access: c::expression::Access {
                volatile: base.access().volatile || false,
                unaligned: base.access().unaligned,
            },
        }
    }
}
impl<Q: c::expression::Qualifier> c::expression::VolatilePlace
    for Bitfield_HeadlineWordEntry__selected<Q>
{
    type Qualified = Self;
    fn qualify_volatile(mut self) -> Self::Qualified {
        self.access.volatile = true;
        self
    }
}
// SAFETY: The same-profile C compiler owns bitfield storage reads, including partially initialized neighbor/padding bits. The marker retains declared size and compiler-proved arithmetic promotion.
unsafe impl<Q: c::expression::Qualifier> c::expression::ReadPlace
    for Bitfield_HeadlineWordEntry__selected<Q>
{
    type Object = c::CUnsignedInt;
    type Type = c::expression::CBitfield<c::CUnsignedInt, c::CInt>;
    unsafe fn load(self) -> <Self::Type as c::expression::CType>::Value {
        let pointer = Q::into_mut(self.address).cast_const(); // SAFETY: The caller establishes the original C field load contract; helpers perform only that field access.
        let storage = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__get_unaligned_volatile__HeadlineWordEntry__selected(pointer)
                } else {
                    __pgrx_bitfield__get_unaligned__HeadlineWordEntry__selected(pointer)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__get_volatile__HeadlineWordEntry__selected(pointer)
            } else {
                __pgrx_bitfield__get__HeadlineWordEntry__selected(pointer)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(storage)
    }
}
// SAFETY: Write capability retains ReadWrite qualification. Original C performs declared-base conversion and width narrowing without Rust byte reads.
unsafe impl c::expression::WritePlace
    for Bitfield_HeadlineWordEntry__selected<c::expression::ReadWrite>
{
    type Assignment = c::CUnsignedInt;
    unsafe fn store(
        self,
        value: <Self::Assignment as c::expression::CType>::Value,
    ) -> <Self::Type as c::expression::CType>::Value {
        let value = <Self::Assignment as c::expression::CType>::into_storage(value); // SAFETY: The caller establishes writable field storage; this helper evaluates one original-C assignment and returns its narrowed result.
        let stored = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__set_unaligned_volatile__HeadlineWordEntry__selected(
                        self.address,
                        value,
                    )
                } else {
                    __pgrx_bitfield__set_unaligned__HeadlineWordEntry__selected(self.address, value)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__set_volatile__HeadlineWordEntry__selected(self.address, value)
            } else {
                __pgrx_bitfield__set__HeadlineWordEntry__selected(self.address, value)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(stored)
    }
}
#[doc(hidden)]
pub struct Field_replace;
const _: () = {
    assert!(
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::HeadlineWordEntry, _bitfield_1) * 8 + 2
            == 2
    );
};
unsafe extern "C" {
    fn __pgrx_bitfield__get__HeadlineWordEntry__replace(
        p: *const crate::__pgrx_c_bindings::HeadlineWordEntry,
    ) -> u32;
    fn __pgrx_bitfield__set__HeadlineWordEntry__replace(
        p: *mut crate::__pgrx_c_bindings::HeadlineWordEntry,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_volatile__HeadlineWordEntry__replace(
        p: *const crate::__pgrx_c_bindings::HeadlineWordEntry,
    ) -> u32;
    fn __pgrx_bitfield__set_volatile__HeadlineWordEntry__replace(
        p: *mut crate::__pgrx_c_bindings::HeadlineWordEntry,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned__HeadlineWordEntry__replace(
        p: *const crate::__pgrx_c_bindings::HeadlineWordEntry,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned__HeadlineWordEntry__replace(
        p: *mut crate::__pgrx_c_bindings::HeadlineWordEntry,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned_volatile__HeadlineWordEntry__replace(
        p: *const crate::__pgrx_c_bindings::HeadlineWordEntry,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned_volatile__HeadlineWordEntry__replace(
        p: *mut crate::__pgrx_c_bindings::HeadlineWordEntry,
        value: u32,
    ) -> u32;
}
#[doc(hidden)]
pub struct Bitfield_HeadlineWordEntry__replace<Q: c::expression::Qualifier> {
    address: Q::Raw<crate::__pgrx_c_bindings::HeadlineWordEntry>,
    access: c::expression::Access,
}
impl<Q: c::expression::Qualifier> Copy for Bitfield_HeadlineWordEntry__replace<Q> {}
impl<Q: c::expression::Qualifier> Clone for Bitfield_HeadlineWordEntry__replace<Q> {
    fn clone(&self) -> Self {
        *self
    }
}
// SAFETY: Clang and bindgen layout witnesses agree. This projects the record address without a reference or record read.
unsafe impl<Q: c::expression::Qualifier> c::expression::Field<Field_replace, Q>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeadlineWordEntry>
{
    type Output = Bitfield_HeadlineWordEntry__replace<Q>;
    unsafe fn project(base: c::expression::Place<Self, Q>) -> Self::Output {
        Bitfield_HeadlineWordEntry__replace::<Q> {
            address: <Q as c::expression::Qualifier>::from_mut(base.pointer().as_mut_address()),
            access: c::expression::Access {
                volatile: base.access().volatile || false,
                unaligned: base.access().unaligned,
            },
        }
    }
}
impl<Q: c::expression::Qualifier> c::expression::VolatilePlace
    for Bitfield_HeadlineWordEntry__replace<Q>
{
    type Qualified = Self;
    fn qualify_volatile(mut self) -> Self::Qualified {
        self.access.volatile = true;
        self
    }
}
// SAFETY: The same-profile C compiler owns bitfield storage reads, including partially initialized neighbor/padding bits. The marker retains declared size and compiler-proved arithmetic promotion.
unsafe impl<Q: c::expression::Qualifier> c::expression::ReadPlace
    for Bitfield_HeadlineWordEntry__replace<Q>
{
    type Object = c::CUnsignedInt;
    type Type = c::expression::CBitfield<c::CUnsignedInt, c::CInt>;
    unsafe fn load(self) -> <Self::Type as c::expression::CType>::Value {
        let pointer = Q::into_mut(self.address).cast_const(); // SAFETY: The caller establishes the original C field load contract; helpers perform only that field access.
        let storage = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__get_unaligned_volatile__HeadlineWordEntry__replace(pointer)
                } else {
                    __pgrx_bitfield__get_unaligned__HeadlineWordEntry__replace(pointer)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__get_volatile__HeadlineWordEntry__replace(pointer)
            } else {
                __pgrx_bitfield__get__HeadlineWordEntry__replace(pointer)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(storage)
    }
}
// SAFETY: Write capability retains ReadWrite qualification. Original C performs declared-base conversion and width narrowing without Rust byte reads.
unsafe impl c::expression::WritePlace
    for Bitfield_HeadlineWordEntry__replace<c::expression::ReadWrite>
{
    type Assignment = c::CUnsignedInt;
    unsafe fn store(
        self,
        value: <Self::Assignment as c::expression::CType>::Value,
    ) -> <Self::Type as c::expression::CType>::Value {
        let value = <Self::Assignment as c::expression::CType>::into_storage(value); // SAFETY: The caller establishes writable field storage; this helper evaluates one original-C assignment and returns its narrowed result.
        let stored = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__set_unaligned_volatile__HeadlineWordEntry__replace(
                        self.address,
                        value,
                    )
                } else {
                    __pgrx_bitfield__set_unaligned__HeadlineWordEntry__replace(self.address, value)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__set_volatile__HeadlineWordEntry__replace(self.address, value)
            } else {
                __pgrx_bitfield__set__HeadlineWordEntry__replace(self.address, value)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(stored)
    }
}
#[doc(hidden)]
pub struct Field_repeated;
const _: () = {
    assert!(
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::HeadlineWordEntry, _bitfield_1) * 8 + 3
            == 3
    );
};
unsafe extern "C" {
    fn __pgrx_bitfield__get__HeadlineWordEntry__repeated(
        p: *const crate::__pgrx_c_bindings::HeadlineWordEntry,
    ) -> u32;
    fn __pgrx_bitfield__set__HeadlineWordEntry__repeated(
        p: *mut crate::__pgrx_c_bindings::HeadlineWordEntry,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_volatile__HeadlineWordEntry__repeated(
        p: *const crate::__pgrx_c_bindings::HeadlineWordEntry,
    ) -> u32;
    fn __pgrx_bitfield__set_volatile__HeadlineWordEntry__repeated(
        p: *mut crate::__pgrx_c_bindings::HeadlineWordEntry,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned__HeadlineWordEntry__repeated(
        p: *const crate::__pgrx_c_bindings::HeadlineWordEntry,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned__HeadlineWordEntry__repeated(
        p: *mut crate::__pgrx_c_bindings::HeadlineWordEntry,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned_volatile__HeadlineWordEntry__repeated(
        p: *const crate::__pgrx_c_bindings::HeadlineWordEntry,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned_volatile__HeadlineWordEntry__repeated(
        p: *mut crate::__pgrx_c_bindings::HeadlineWordEntry,
        value: u32,
    ) -> u32;
}
#[doc(hidden)]
pub struct Bitfield_HeadlineWordEntry__repeated<Q: c::expression::Qualifier> {
    address: Q::Raw<crate::__pgrx_c_bindings::HeadlineWordEntry>,
    access: c::expression::Access,
}
impl<Q: c::expression::Qualifier> Copy for Bitfield_HeadlineWordEntry__repeated<Q> {}
impl<Q: c::expression::Qualifier> Clone for Bitfield_HeadlineWordEntry__repeated<Q> {
    fn clone(&self) -> Self {
        *self
    }
}
// SAFETY: Clang and bindgen layout witnesses agree. This projects the record address without a reference or record read.
unsafe impl<Q: c::expression::Qualifier> c::expression::Field<Field_repeated, Q>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeadlineWordEntry>
{
    type Output = Bitfield_HeadlineWordEntry__repeated<Q>;
    unsafe fn project(base: c::expression::Place<Self, Q>) -> Self::Output {
        Bitfield_HeadlineWordEntry__repeated::<Q> {
            address: <Q as c::expression::Qualifier>::from_mut(base.pointer().as_mut_address()),
            access: c::expression::Access {
                volatile: base.access().volatile || false,
                unaligned: base.access().unaligned,
            },
        }
    }
}
impl<Q: c::expression::Qualifier> c::expression::VolatilePlace
    for Bitfield_HeadlineWordEntry__repeated<Q>
{
    type Qualified = Self;
    fn qualify_volatile(mut self) -> Self::Qualified {
        self.access.volatile = true;
        self
    }
}
// SAFETY: The same-profile C compiler owns bitfield storage reads, including partially initialized neighbor/padding bits. The marker retains declared size and compiler-proved arithmetic promotion.
unsafe impl<Q: c::expression::Qualifier> c::expression::ReadPlace
    for Bitfield_HeadlineWordEntry__repeated<Q>
{
    type Object = c::CUnsignedInt;
    type Type = c::expression::CBitfield<c::CUnsignedInt, c::CInt>;
    unsafe fn load(self) -> <Self::Type as c::expression::CType>::Value {
        let pointer = Q::into_mut(self.address).cast_const(); // SAFETY: The caller establishes the original C field load contract; helpers perform only that field access.
        let storage = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__get_unaligned_volatile__HeadlineWordEntry__repeated(pointer)
                } else {
                    __pgrx_bitfield__get_unaligned__HeadlineWordEntry__repeated(pointer)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__get_volatile__HeadlineWordEntry__repeated(pointer)
            } else {
                __pgrx_bitfield__get__HeadlineWordEntry__repeated(pointer)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(storage)
    }
}
// SAFETY: Write capability retains ReadWrite qualification. Original C performs declared-base conversion and width narrowing without Rust byte reads.
unsafe impl c::expression::WritePlace
    for Bitfield_HeadlineWordEntry__repeated<c::expression::ReadWrite>
{
    type Assignment = c::CUnsignedInt;
    unsafe fn store(
        self,
        value: <Self::Assignment as c::expression::CType>::Value,
    ) -> <Self::Type as c::expression::CType>::Value {
        let value = <Self::Assignment as c::expression::CType>::into_storage(value); // SAFETY: The caller establishes writable field storage; this helper evaluates one original-C assignment and returns its narrowed result.
        let stored = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__set_unaligned_volatile__HeadlineWordEntry__repeated(
                        self.address,
                        value,
                    )
                } else {
                    __pgrx_bitfield__set_unaligned__HeadlineWordEntry__repeated(self.address, value)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__set_volatile__HeadlineWordEntry__repeated(self.address, value)
            } else {
                __pgrx_bitfield__set__HeadlineWordEntry__repeated(self.address, value)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(stored)
    }
}
#[doc(hidden)]
pub struct Field_skip;
const _: () = {
    assert!(
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::HeadlineWordEntry, _bitfield_1) * 8 + 4
            == 4
    );
};
unsafe extern "C" {
    fn __pgrx_bitfield__get__HeadlineWordEntry__skip(
        p: *const crate::__pgrx_c_bindings::HeadlineWordEntry,
    ) -> u32;
    fn __pgrx_bitfield__set__HeadlineWordEntry__skip(
        p: *mut crate::__pgrx_c_bindings::HeadlineWordEntry,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_volatile__HeadlineWordEntry__skip(
        p: *const crate::__pgrx_c_bindings::HeadlineWordEntry,
    ) -> u32;
    fn __pgrx_bitfield__set_volatile__HeadlineWordEntry__skip(
        p: *mut crate::__pgrx_c_bindings::HeadlineWordEntry,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned__HeadlineWordEntry__skip(
        p: *const crate::__pgrx_c_bindings::HeadlineWordEntry,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned__HeadlineWordEntry__skip(
        p: *mut crate::__pgrx_c_bindings::HeadlineWordEntry,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned_volatile__HeadlineWordEntry__skip(
        p: *const crate::__pgrx_c_bindings::HeadlineWordEntry,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned_volatile__HeadlineWordEntry__skip(
        p: *mut crate::__pgrx_c_bindings::HeadlineWordEntry,
        value: u32,
    ) -> u32;
}
#[doc(hidden)]
pub struct Bitfield_HeadlineWordEntry__skip<Q: c::expression::Qualifier> {
    address: Q::Raw<crate::__pgrx_c_bindings::HeadlineWordEntry>,
    access: c::expression::Access,
}
impl<Q: c::expression::Qualifier> Copy for Bitfield_HeadlineWordEntry__skip<Q> {}
impl<Q: c::expression::Qualifier> Clone for Bitfield_HeadlineWordEntry__skip<Q> {
    fn clone(&self) -> Self {
        *self
    }
}
// SAFETY: Clang and bindgen layout witnesses agree. This projects the record address without a reference or record read.
unsafe impl<Q: c::expression::Qualifier> c::expression::Field<Field_skip, Q>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeadlineWordEntry>
{
    type Output = Bitfield_HeadlineWordEntry__skip<Q>;
    unsafe fn project(base: c::expression::Place<Self, Q>) -> Self::Output {
        Bitfield_HeadlineWordEntry__skip::<Q> {
            address: <Q as c::expression::Qualifier>::from_mut(base.pointer().as_mut_address()),
            access: c::expression::Access {
                volatile: base.access().volatile || false,
                unaligned: base.access().unaligned,
            },
        }
    }
}
impl<Q: c::expression::Qualifier> c::expression::VolatilePlace
    for Bitfield_HeadlineWordEntry__skip<Q>
{
    type Qualified = Self;
    fn qualify_volatile(mut self) -> Self::Qualified {
        self.access.volatile = true;
        self
    }
}
// SAFETY: The same-profile C compiler owns bitfield storage reads, including partially initialized neighbor/padding bits. The marker retains declared size and compiler-proved arithmetic promotion.
unsafe impl<Q: c::expression::Qualifier> c::expression::ReadPlace
    for Bitfield_HeadlineWordEntry__skip<Q>
{
    type Object = c::CUnsignedInt;
    type Type = c::expression::CBitfield<c::CUnsignedInt, c::CInt>;
    unsafe fn load(self) -> <Self::Type as c::expression::CType>::Value {
        let pointer = Q::into_mut(self.address).cast_const(); // SAFETY: The caller establishes the original C field load contract; helpers perform only that field access.
        let storage = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__get_unaligned_volatile__HeadlineWordEntry__skip(pointer)
                } else {
                    __pgrx_bitfield__get_unaligned__HeadlineWordEntry__skip(pointer)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__get_volatile__HeadlineWordEntry__skip(pointer)
            } else {
                __pgrx_bitfield__get__HeadlineWordEntry__skip(pointer)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(storage)
    }
}
// SAFETY: Write capability retains ReadWrite qualification. Original C performs declared-base conversion and width narrowing without Rust byte reads.
unsafe impl c::expression::WritePlace
    for Bitfield_HeadlineWordEntry__skip<c::expression::ReadWrite>
{
    type Assignment = c::CUnsignedInt;
    unsafe fn store(
        self,
        value: <Self::Assignment as c::expression::CType>::Value,
    ) -> <Self::Type as c::expression::CType>::Value {
        let value = <Self::Assignment as c::expression::CType>::into_storage(value); // SAFETY: The caller establishes writable field storage; this helper evaluates one original-C assignment and returns its narrowed result.
        let stored = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__set_unaligned_volatile__HeadlineWordEntry__skip(
                        self.address,
                        value,
                    )
                } else {
                    __pgrx_bitfield__set_unaligned__HeadlineWordEntry__skip(self.address, value)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__set_volatile__HeadlineWordEntry__skip(self.address, value)
            } else {
                __pgrx_bitfield__set__HeadlineWordEntry__skip(self.address, value)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(stored)
    }
}
#[doc(hidden)]
pub struct Field_unused;
const _: () = {
    assert!(
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::HeadlineWordEntry, _bitfield_1) * 8 + 5
            == 5
    );
};
unsafe extern "C" {
    fn __pgrx_bitfield__get__HeadlineWordEntry__unused(
        p: *const crate::__pgrx_c_bindings::HeadlineWordEntry,
    ) -> u32;
    fn __pgrx_bitfield__set__HeadlineWordEntry__unused(
        p: *mut crate::__pgrx_c_bindings::HeadlineWordEntry,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_volatile__HeadlineWordEntry__unused(
        p: *const crate::__pgrx_c_bindings::HeadlineWordEntry,
    ) -> u32;
    fn __pgrx_bitfield__set_volatile__HeadlineWordEntry__unused(
        p: *mut crate::__pgrx_c_bindings::HeadlineWordEntry,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned__HeadlineWordEntry__unused(
        p: *const crate::__pgrx_c_bindings::HeadlineWordEntry,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned__HeadlineWordEntry__unused(
        p: *mut crate::__pgrx_c_bindings::HeadlineWordEntry,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned_volatile__HeadlineWordEntry__unused(
        p: *const crate::__pgrx_c_bindings::HeadlineWordEntry,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned_volatile__HeadlineWordEntry__unused(
        p: *mut crate::__pgrx_c_bindings::HeadlineWordEntry,
        value: u32,
    ) -> u32;
}
#[doc(hidden)]
pub struct Bitfield_HeadlineWordEntry__unused<Q: c::expression::Qualifier> {
    address: Q::Raw<crate::__pgrx_c_bindings::HeadlineWordEntry>,
    access: c::expression::Access,
}
impl<Q: c::expression::Qualifier> Copy for Bitfield_HeadlineWordEntry__unused<Q> {}
impl<Q: c::expression::Qualifier> Clone for Bitfield_HeadlineWordEntry__unused<Q> {
    fn clone(&self) -> Self {
        *self
    }
}
// SAFETY: Clang and bindgen layout witnesses agree. This projects the record address without a reference or record read.
unsafe impl<Q: c::expression::Qualifier> c::expression::Field<Field_unused, Q>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeadlineWordEntry>
{
    type Output = Bitfield_HeadlineWordEntry__unused<Q>;
    unsafe fn project(base: c::expression::Place<Self, Q>) -> Self::Output {
        Bitfield_HeadlineWordEntry__unused::<Q> {
            address: <Q as c::expression::Qualifier>::from_mut(base.pointer().as_mut_address()),
            access: c::expression::Access {
                volatile: base.access().volatile || false,
                unaligned: base.access().unaligned,
            },
        }
    }
}
impl<Q: c::expression::Qualifier> c::expression::VolatilePlace
    for Bitfield_HeadlineWordEntry__unused<Q>
{
    type Qualified = Self;
    fn qualify_volatile(mut self) -> Self::Qualified {
        self.access.volatile = true;
        self
    }
}
// SAFETY: The same-profile C compiler owns bitfield storage reads, including partially initialized neighbor/padding bits. The marker retains declared size and compiler-proved arithmetic promotion.
unsafe impl<Q: c::expression::Qualifier> c::expression::ReadPlace
    for Bitfield_HeadlineWordEntry__unused<Q>
{
    type Object = c::CUnsignedInt;
    type Type = c::expression::CBitfield<c::CUnsignedInt, c::CInt>;
    unsafe fn load(self) -> <Self::Type as c::expression::CType>::Value {
        let pointer = Q::into_mut(self.address).cast_const(); // SAFETY: The caller establishes the original C field load contract; helpers perform only that field access.
        let storage = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__get_unaligned_volatile__HeadlineWordEntry__unused(pointer)
                } else {
                    __pgrx_bitfield__get_unaligned__HeadlineWordEntry__unused(pointer)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__get_volatile__HeadlineWordEntry__unused(pointer)
            } else {
                __pgrx_bitfield__get__HeadlineWordEntry__unused(pointer)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(storage)
    }
}
// SAFETY: Write capability retains ReadWrite qualification. Original C performs declared-base conversion and width narrowing without Rust byte reads.
unsafe impl c::expression::WritePlace
    for Bitfield_HeadlineWordEntry__unused<c::expression::ReadWrite>
{
    type Assignment = c::CUnsignedInt;
    unsafe fn store(
        self,
        value: <Self::Assignment as c::expression::CType>::Value,
    ) -> <Self::Type as c::expression::CType>::Value {
        let value = <Self::Assignment as c::expression::CType>::into_storage(value); // SAFETY: The caller establishes writable field storage; this helper evaluates one original-C assignment and returns its narrowed result.
        let stored = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__set_unaligned_volatile__HeadlineWordEntry__unused(
                        self.address,
                        value,
                    )
                } else {
                    __pgrx_bitfield__set_unaligned__HeadlineWordEntry__unused(self.address, value)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__set_volatile__HeadlineWordEntry__unused(self.address, value)
            } else {
                __pgrx_bitfield__set__HeadlineWordEntry__unused(self.address, value)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(stored)
    }
}
#[doc(hidden)]
pub struct Field_len;
const _: () = {
    assert!(
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::HeadlineWordEntry, _bitfield_1) * 8 + 16
            == 16
    );
};
unsafe extern "C" {
    fn __pgrx_bitfield__get__HeadlineWordEntry__len(
        p: *const crate::__pgrx_c_bindings::HeadlineWordEntry,
    ) -> u32;
    fn __pgrx_bitfield__set__HeadlineWordEntry__len(
        p: *mut crate::__pgrx_c_bindings::HeadlineWordEntry,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_volatile__HeadlineWordEntry__len(
        p: *const crate::__pgrx_c_bindings::HeadlineWordEntry,
    ) -> u32;
    fn __pgrx_bitfield__set_volatile__HeadlineWordEntry__len(
        p: *mut crate::__pgrx_c_bindings::HeadlineWordEntry,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned__HeadlineWordEntry__len(
        p: *const crate::__pgrx_c_bindings::HeadlineWordEntry,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned__HeadlineWordEntry__len(
        p: *mut crate::__pgrx_c_bindings::HeadlineWordEntry,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned_volatile__HeadlineWordEntry__len(
        p: *const crate::__pgrx_c_bindings::HeadlineWordEntry,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned_volatile__HeadlineWordEntry__len(
        p: *mut crate::__pgrx_c_bindings::HeadlineWordEntry,
        value: u32,
    ) -> u32;
}
#[doc(hidden)]
pub struct Bitfield_HeadlineWordEntry__len<Q: c::expression::Qualifier> {
    address: Q::Raw<crate::__pgrx_c_bindings::HeadlineWordEntry>,
    access: c::expression::Access,
}
impl<Q: c::expression::Qualifier> Copy for Bitfield_HeadlineWordEntry__len<Q> {}
impl<Q: c::expression::Qualifier> Clone for Bitfield_HeadlineWordEntry__len<Q> {
    fn clone(&self) -> Self {
        *self
    }
}
// SAFETY: Clang and bindgen layout witnesses agree. This projects the record address without a reference or record read.
unsafe impl<Q: c::expression::Qualifier> c::expression::Field<Field_len, Q>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeadlineWordEntry>
{
    type Output = Bitfield_HeadlineWordEntry__len<Q>;
    unsafe fn project(base: c::expression::Place<Self, Q>) -> Self::Output {
        Bitfield_HeadlineWordEntry__len::<Q> {
            address: <Q as c::expression::Qualifier>::from_mut(base.pointer().as_mut_address()),
            access: c::expression::Access {
                volatile: base.access().volatile || false,
                unaligned: base.access().unaligned,
            },
        }
    }
}
impl<Q: c::expression::Qualifier> c::expression::VolatilePlace
    for Bitfield_HeadlineWordEntry__len<Q>
{
    type Qualified = Self;
    fn qualify_volatile(mut self) -> Self::Qualified {
        self.access.volatile = true;
        self
    }
}
// SAFETY: The same-profile C compiler owns bitfield storage reads, including partially initialized neighbor/padding bits. The marker retains declared size and compiler-proved arithmetic promotion.
unsafe impl<Q: c::expression::Qualifier> c::expression::ReadPlace
    for Bitfield_HeadlineWordEntry__len<Q>
{
    type Object = c::CUnsignedInt;
    type Type = c::expression::CBitfield<c::CUnsignedInt, c::CInt>;
    unsafe fn load(self) -> <Self::Type as c::expression::CType>::Value {
        let pointer = Q::into_mut(self.address).cast_const(); // SAFETY: The caller establishes the original C field load contract; helpers perform only that field access.
        let storage = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__get_unaligned_volatile__HeadlineWordEntry__len(pointer)
                } else {
                    __pgrx_bitfield__get_unaligned__HeadlineWordEntry__len(pointer)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__get_volatile__HeadlineWordEntry__len(pointer)
            } else {
                __pgrx_bitfield__get__HeadlineWordEntry__len(pointer)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(storage)
    }
}
// SAFETY: Write capability retains ReadWrite qualification. Original C performs declared-base conversion and width narrowing without Rust byte reads.
unsafe impl c::expression::WritePlace
    for Bitfield_HeadlineWordEntry__len<c::expression::ReadWrite>
{
    type Assignment = c::CUnsignedInt;
    unsafe fn store(
        self,
        value: <Self::Assignment as c::expression::CType>::Value,
    ) -> <Self::Type as c::expression::CType>::Value {
        let value = <Self::Assignment as c::expression::CType>::into_storage(value); // SAFETY: The caller establishes writable field storage; this helper evaluates one original-C assignment and returns its narrowed result.
        let stored = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__set_unaligned_volatile__HeadlineWordEntry__len(
                        self.address,
                        value,
                    )
                } else {
                    __pgrx_bitfield__set_unaligned__HeadlineWordEntry__len(self.address, value)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__set_volatile__HeadlineWordEntry__len(self.address, value)
            } else {
                __pgrx_bitfield__set__HeadlineWordEntry__len(self.address, value)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(stored)
    }
}
#[doc(hidden)]
pub struct Field_pos;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeadlineWordEntry, pos) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeadlineWordEntry>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HeadlineWordEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::HeadlineWordEntry> as c::expression::OrdinaryField<Field_pos>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pos) };
#[doc(hidden)]
pub struct Field_next_elem;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::IndexArrayKeyInfo>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::IndexArrayKeyInfo>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::IndexArrayKeyInfo {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexArrayKeyInfo, next_elem) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_next_elem>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexArrayKeyInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexArrayKeyInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexArrayKeyInfo> as c::expression::OrdinaryField<Field_next_elem>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).next_elem) };
#[doc(hidden)]
pub struct Field_num_elems;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexArrayKeyInfo, num_elems) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_elems>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexArrayKeyInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexArrayKeyInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexArrayKeyInfo> as c::expression::OrdinaryField<Field_num_elems>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_elems) };
#[doc(hidden)]
pub struct Field_key_toastable;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::IndexRuntimeKeyInfo>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::IndexRuntimeKeyInfo>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::IndexRuntimeKeyInfo {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexRuntimeKeyInfo, key_toastable) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_key_toastable>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexRuntimeKeyInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexRuntimeKeyInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexRuntimeKeyInfo> as c::expression::OrdinaryField<Field_key_toastable>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).key_toastable) };
#[doc(hidden)]
pub struct Field_is_grant;
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::InternalGrant>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::InternalGrant>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::InternalGrant {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::InternalGrant, is_grant) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_grant>
    for c::expression::CRecord<crate::__pgrx_c_bindings::InternalGrant>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::InternalGrant, <<c::expression::CRecord<crate::__pgrx_c_bindings::InternalGrant> as c::expression::OrdinaryField<Field_is_grant>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_grant) };
#[doc(hidden)]
pub struct Field_objtype;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::InternalGrant, objtype) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_objtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::InternalGrant>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ObjectType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::InternalGrant, <<c::expression::CRecord<crate::__pgrx_c_bindings::InternalGrant> as c::expression::OrdinaryField<Field_objtype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).objtype) };
#[doc(hidden)]
pub struct Field_objects;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::InternalGrant, objects) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_objects>
    for c::expression::CRecord<crate::__pgrx_c_bindings::InternalGrant>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::InternalGrant, <<c::expression::CRecord<crate::__pgrx_c_bindings::InternalGrant> as c::expression::OrdinaryField<Field_objects>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).objects) };
#[doc(hidden)]
pub struct Field_all_privs;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::InternalGrant, all_privs) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_all_privs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::InternalGrant>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::InternalGrant, <<c::expression::CRecord<crate::__pgrx_c_bindings::InternalGrant> as c::expression::OrdinaryField<Field_all_privs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).all_privs) };
#[doc(hidden)]
pub struct Field_privileges;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::InternalGrant, privileges) == 20);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_privileges>
    for c::expression::CRecord<crate::__pgrx_c_bindings::InternalGrant>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::InternalGrant, <<c::expression::CRecord<crate::__pgrx_c_bindings::InternalGrant> as c::expression::OrdinaryField<Field_privileges>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).privileges) };
#[doc(hidden)]
pub struct Field_col_privs;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::InternalGrant, col_privs) == 24);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_col_privs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::InternalGrant>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::InternalGrant, <<c::expression::CRecord<crate::__pgrx_c_bindings::InternalGrant> as c::expression::OrdinaryField<Field_col_privs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).col_privs) };
#[doc(hidden)]
pub struct Field_grantees;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::InternalGrant, grantees) == 32);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_grantees>
    for c::expression::CRecord<crate::__pgrx_c_bindings::InternalGrant>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::InternalGrant, <<c::expression::CRecord<crate::__pgrx_c_bindings::InternalGrant> as c::expression::OrdinaryField<Field_grantees>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).grantees) };
#[doc(hidden)]
pub struct Field_grant_option;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::InternalGrant, grant_option) == 40);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_grant_option>
    for c::expression::CRecord<crate::__pgrx_c_bindings::InternalGrant>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::InternalGrant, <<c::expression::CRecord<crate::__pgrx_c_bindings::InternalGrant> as c::expression::OrdinaryField<Field_grant_option>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).grant_option) };
#[doc(hidden)]
pub struct Field_behavior;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::InternalGrant, behavior) == 44);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_behavior>
    for c::expression::CRecord<crate::__pgrx_c_bindings::InternalGrant>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__DropBehavior,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::InternalGrant, <<c::expression::CRecord<crate::__pgrx_c_bindings::InternalGrant> as c::expression::OrdinaryField<Field_behavior>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).behavior) };
#[doc(hidden)]
pub struct Field_time;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Interval>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Interval>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::Interval {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Interval, time) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Interval>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Interval,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Interval> as c::expression::OrdinaryField<
        Field_time,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).time) };
#[doc(hidden)]
pub struct Field_day;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Interval, day) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_day>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Interval>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Interval,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Interval> as c::expression::OrdinaryField<
        Field_day,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).day) };
#[doc(hidden)]
pub struct Field_month;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Interval, month) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_month>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Interval>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Interval,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Interval> as c::expression::OrdinaryField<
        Field_month,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).month) };
#[doc(hidden)]
pub struct Field_vl_len_;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Jsonb>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Jsonb>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::Jsonb {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Jsonb, vl_len_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vl_len_>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Jsonb>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Jsonb,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Jsonb> as c::expression::OrdinaryField<
        Field_vl_len_,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).vl_len_) };
#[doc(hidden)]
pub struct Field_lexid;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LexDescr>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LexDescr>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::LexDescr {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LexDescr, lexid) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lexid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LexDescr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::LexDescr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::LexDescr> as c::expression::OrdinaryField<
        Field_lexid,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lexid) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ListDictionary>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ListDictionary>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ListDictionary {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ListDictionary, len) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ListDictionary>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ListDictionary, <<c::expression::CRecord<crate::__pgrx_c_bindings::ListDictionary> as c::expression::OrdinaryField<Field_len>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).len) };
#[doc(hidden)]
pub struct Field_dropflags;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ObjectAccessDrop>() == 4);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ObjectAccessDrop>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ObjectAccessDrop {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ObjectAccessDrop, dropflags) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dropflags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ObjectAccessDrop>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ObjectAccessDrop, <<c::expression::CRecord<crate::__pgrx_c_bindings::ObjectAccessDrop> as c::expression::OrdinaryField<Field_dropflags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dropflags) };
#[doc(hidden)]
pub struct Field_ereport_on_violation;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ObjectAccessNamespaceSearch>() == 2);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ObjectAccessNamespaceSearch>() == 1);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ObjectAccessNamespaceSearch {}
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ObjectAccessNamespaceSearch,
        ereport_on_violation
    ) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ereport_on_violation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ObjectAccessNamespaceSearch>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ObjectAccessNamespaceSearch, <<c::expression::CRecord<crate::__pgrx_c_bindings::ObjectAccessNamespaceSearch> as c::expression::OrdinaryField<Field_ereport_on_violation>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ereport_on_violation) };
#[doc(hidden)]
pub struct Field_result;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ObjectAccessNamespaceSearch, result) == 1
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_result>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ObjectAccessNamespaceSearch>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ObjectAccessNamespaceSearch, <<c::expression::CRecord<crate::__pgrx_c_bindings::ObjectAccessNamespaceSearch> as c::expression::OrdinaryField<Field_result>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).result) };
#[doc(hidden)]
pub struct Field_auxiliary_id;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ObjectAccessPostAlter>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ObjectAccessPostAlter>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ObjectAccessPostAlter {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ObjectAccessPostAlter, auxiliary_id) == 0
);
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Oid>() == 4);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Oid>() == 4);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_auxiliary_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ObjectAccessPostAlter>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ObjectAccessPostAlter, <<c::expression::CRecord<crate::__pgrx_c_bindings::ObjectAccessPostAlter> as c::expression::OrdinaryField<Field_auxiliary_id>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).auxiliary_id) };
#[doc(hidden)]
pub struct Field_is_internal;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ObjectAccessPostAlter, is_internal) == 4
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_internal>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ObjectAccessPostAlter>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ObjectAccessPostAlter, <<c::expression::CRecord<crate::__pgrx_c_bindings::ObjectAccessPostAlter> as c::expression::OrdinaryField<Field_is_internal>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_internal) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ObjectAccessPostCreate>() == 1);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ObjectAccessPostCreate>() == 1);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ObjectAccessPostCreate {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ObjectAccessPostCreate, is_internal) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_internal>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ObjectAccessPostCreate>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ObjectAccessPostCreate, <<c::expression::CRecord<crate::__pgrx_c_bindings::ObjectAccessPostCreate> as c::expression::OrdinaryField<Field_is_internal>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_internal) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PATH>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PATH>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PATH {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PATH, vl_len_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vl_len_>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PATH>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PATH,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PATH> as c::expression::OrdinaryField<
        Field_vl_len_,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).vl_len_) };
#[doc(hidden)]
pub struct Field_npts;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PATH, npts) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_npts>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PATH>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PATH,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PATH> as c::expression::OrdinaryField<
        Field_npts,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).npts) };
#[doc(hidden)]
pub struct Field_closed;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PATH, closed) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_closed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PATH>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PATH,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PATH> as c::expression::OrdinaryField<
        Field_closed,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).closed) };
#[doc(hidden)]
pub struct Field_dummy;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PATH, dummy) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dummy>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PATH>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PATH,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PATH> as c::expression::OrdinaryField<
        Field_dummy,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dummy) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PGChecksummablePage>() == 8192);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PGChecksummablePage>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PGChecksummablePage {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGChecksummablePage, data) == 0);
const _: () = {
    assert!(::core::mem::size_of::<::core::mem::ManuallyDrop<[[u32; 32]; 64]>>() == 8192);
    assert!(::core::mem::align_of::<::core::mem::ManuallyDrop<[[u32; 32]; 64]>>() == 4);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_data>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGChecksummablePage>
{
    type Member = c::expression::CArray<c::expression::CArray<c::CUnsignedInt, 32>, 64>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PGChecksummablePage, ::core::mem::ManuallyDrop<<<c::expression::CRecord<crate::__pgrx_c_bindings::PGChecksummablePage> as c::expression::OrdinaryField<Field_data>>::Member as c::expression::CType>::Storage>> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).data) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::POLYGON>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::POLYGON>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::POLYGON {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::POLYGON, vl_len_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vl_len_>
    for c::expression::CRecord<crate::__pgrx_c_bindings::POLYGON>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::POLYGON,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::POLYGON> as c::expression::OrdinaryField<
        Field_vl_len_,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).vl_len_) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::POLYGON, npts) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_npts>
    for c::expression::CRecord<crate::__pgrx_c_bindings::POLYGON>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::POLYGON,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::POLYGON> as c::expression::OrdinaryField<
        Field_npts,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).npts) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
pub struct Field_xlogid;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PageXLogRecPtr>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PageXLogRecPtr>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PageXLogRecPtr {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PageXLogRecPtr, xlogid) == 0);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xlogid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PageXLogRecPtr>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PageXLogRecPtr, <<c::expression::CRecord<crate::__pgrx_c_bindings::PageXLogRecPtr> as c::expression::OrdinaryField<Field_xlogid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xlogid) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
pub struct Field_xrecoff;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PageXLogRecPtr, xrecoff) == 4);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xrecoff>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PageXLogRecPtr>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PageXLogRecPtr, <<c::expression::CRecord<crate::__pgrx_c_bindings::PageXLogRecPtr> as c::expression::OrdinaryField<Field_xrecoff>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xrecoff) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ParsedText>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ParsedText>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ParsedText {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParsedText, lenwords) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lenwords>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParsedText>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParsedText, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParsedText> as c::expression::OrdinaryField<Field_lenwords>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lenwords) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParsedText, curwords) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_curwords>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParsedText>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParsedText, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParsedText> as c::expression::OrdinaryField<Field_curwords>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).curwords) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParsedText, pos) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParsedText>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParsedText, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParsedText> as c::expression::OrdinaryField<Field_pos>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pos) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ParsedWord>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ParsedWord>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ParsedWord {}
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParsedWord, len) == 0);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParsedWord>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParsedWord, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParsedWord> as c::expression::OrdinaryField<Field_len>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).len) };
#[doc(hidden)]
pub struct Field_nvariant;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParsedWord, nvariant) == 2);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nvariant>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParsedWord>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 2;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParsedWord, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParsedWord> as c::expression::OrdinaryField<Field_nvariant>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nvariant) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParsedWord, flags) == 16);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParsedWord>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParsedWord, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParsedWord> as c::expression::OrdinaryField<Field_flags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).flags) };
#[doc(hidden)]
pub struct Field_alen;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParsedWord, alen) == 32);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_alen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParsedWord>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParsedWord, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParsedWord> as c::expression::OrdinaryField<Field_alen>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).alen) };
#[doc(hidden)]
pub struct Field_api_version;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Pg_finfo_record>() == 4);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Pg_finfo_record>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::Pg_finfo_record {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Pg_finfo_record, api_version) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_api_version>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Pg_finfo_record>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Pg_finfo_record, <<c::expression::CRecord<crate::__pgrx_c_bindings::Pg_finfo_record> as c::expression::OrdinaryField<Field_api_version>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).api_version) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Pg_magic_struct>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Pg_magic_struct>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::Pg_magic_struct {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Pg_magic_struct, len) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Pg_magic_struct>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Pg_magic_struct, <<c::expression::CRecord<crate::__pgrx_c_bindings::Pg_magic_struct> as c::expression::OrdinaryField<Field_len>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).len) };
#[doc(hidden)]
pub struct Field_version;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Pg_magic_struct, version) == 4);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_version>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Pg_magic_struct>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Pg_magic_struct, <<c::expression::CRecord<crate::__pgrx_c_bindings::Pg_magic_struct> as c::expression::OrdinaryField<Field_version>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).version) };
#[doc(hidden)]
pub struct Field_funcmaxargs;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Pg_magic_struct, funcmaxargs) == 8);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_funcmaxargs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Pg_magic_struct>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Pg_magic_struct, <<c::expression::CRecord<crate::__pgrx_c_bindings::Pg_magic_struct> as c::expression::OrdinaryField<Field_funcmaxargs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).funcmaxargs) };
#[doc(hidden)]
pub struct Field_indexmaxkeys;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Pg_magic_struct, indexmaxkeys) == 12);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indexmaxkeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Pg_magic_struct>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Pg_magic_struct, <<c::expression::CRecord<crate::__pgrx_c_bindings::Pg_magic_struct> as c::expression::OrdinaryField<Field_indexmaxkeys>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indexmaxkeys) };
#[doc(hidden)]
pub struct Field_namedatalen;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Pg_magic_struct, namedatalen) == 16);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_namedatalen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Pg_magic_struct>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Pg_magic_struct, <<c::expression::CRecord<crate::__pgrx_c_bindings::Pg_magic_struct> as c::expression::OrdinaryField<Field_namedatalen>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).namedatalen) };
#[doc(hidden)]
pub struct Field_float8byval;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Pg_magic_struct, float8byval) == 20);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_float8byval>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Pg_magic_struct>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Pg_magic_struct, <<c::expression::CRecord<crate::__pgrx_c_bindings::Pg_magic_struct> as c::expression::OrdinaryField<Field_float8byval>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).float8byval) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PipeProtoHeader>() == 12);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PipeProtoHeader>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PipeProtoHeader {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PipeProtoHeader, len) == 2);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PipeProtoHeader>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 2;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PipeProtoHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::PipeProtoHeader> as c::expression::OrdinaryField<Field_len>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).len) };
#[doc(hidden)]
pub struct Field_pid;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PipeProtoHeader, pid) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PipeProtoHeader>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PipeProtoHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::PipeProtoHeader> as c::expression::OrdinaryField<Field_pid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pid) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PipeProtoHeader, flags) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PipeProtoHeader>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PipeProtoHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::PipeProtoHeader> as c::expression::OrdinaryField<Field_flags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).flags) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PipeProtoHeader, data) == 9);
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::__IncompleteArrayField<::core::ffi::c_char>>(
        ) == 0
    );
    assert!(
        ::core::mem::align_of::<
            crate::__pgrx_c_bindings::__IncompleteArrayField<::core::ffi::c_char>,
        >() == 1
    );
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_data>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PipeProtoHeader>
{
    type Member = c::expression::CFlexibleArray<
        c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
        crate::__pgrx_c_bindings::__IncompleteArrayField<::core::ffi::c_char>,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 9;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PipeProtoHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::PipeProtoHeader> as c::expression::OrdinaryField<Field_data>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).data) };
#[doc(hidden)]
pub struct Field_child_blkno;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PostingItem>() == 10);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PostingItem>() == 2);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PostingItem {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PostingItem, child_blkno) == 0);
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BlockIdData>() == 4);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BlockIdData>() == 2);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_child_blkno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PostingItem>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::BlockIdData>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PostingItem, <<c::expression::CRecord<crate::__pgrx_c_bindings::PostingItem> as c::expression::OrdinaryField<Field_child_blkno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).child_blkno) };
#[doc(hidden)]
pub struct Field_from_sql;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PreparedStatement>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PreparedStatement>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PreparedStatement {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PreparedStatement, from_sql) == 72);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_from_sql>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PreparedStatement>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PreparedStatement, <<c::expression::CRecord<crate::__pgrx_c_bindings::PreparedStatement> as c::expression::OrdinaryField<Field_from_sql>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).from_sql) };
#[doc(hidden)]
pub struct Field_prepare_time;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PreparedStatement, prepare_time) == 80
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_prepare_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PreparedStatement>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PreparedStatement, <<c::expression::CRecord<crate::__pgrx_c_bindings::PreparedStatement> as c::expression::OrdinaryField<Field_prepare_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).prepare_time) };
#[doc(hidden)]
pub struct Field_type;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::QueryItem>() == 12);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::QueryItem>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::QueryItem {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::QueryItem, type_) == 0);
const _: () = {
    assert!(::core::mem::size_of::<i8>() == 1);
    assert!(::core::mem::align_of::<i8>() == 1);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::QueryItem>
{
    type Member = c::CSignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::QueryItem,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::QueryItem> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::QueryOperand>() == 12);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::QueryOperand>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::QueryOperand {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::QueryOperand, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::QueryOperand>
{
    type Member = c::CSignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::QueryOperand, <<c::expression::CRecord<crate::__pgrx_c_bindings::QueryOperand> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_weight;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::QueryOperand, weight) == 1);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_weight>
    for c::expression::CRecord<crate::__pgrx_c_bindings::QueryOperand>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::QueryOperand, <<c::expression::CRecord<crate::__pgrx_c_bindings::QueryOperand> as c::expression::OrdinaryField<Field_weight>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).weight) };
#[doc(hidden)]
pub struct Field_prefix;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::QueryOperand, prefix) == 2);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_prefix>
    for c::expression::CRecord<crate::__pgrx_c_bindings::QueryOperand>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 2;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::QueryOperand, <<c::expression::CRecord<crate::__pgrx_c_bindings::QueryOperand> as c::expression::OrdinaryField<Field_prefix>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).prefix) };
#[doc(hidden)]
pub struct Field_valcrc;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::QueryOperand, valcrc) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_valcrc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::QueryOperand>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::QueryOperand, <<c::expression::CRecord<crate::__pgrx_c_bindings::QueryOperand> as c::expression::OrdinaryField<Field_valcrc>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).valcrc) };
#[doc(hidden)]
pub struct Field_length;
const _: () = {
    assert!(
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::QueryOperand, _bitfield_1) * 8 + 0 == 64
    );
};
unsafe extern "C" {
    fn __pgrx_bitfield__get__QueryOperand__length(
        p: *const crate::__pgrx_c_bindings::QueryOperand,
    ) -> u32;
    fn __pgrx_bitfield__set__QueryOperand__length(
        p: *mut crate::__pgrx_c_bindings::QueryOperand,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_volatile__QueryOperand__length(
        p: *const crate::__pgrx_c_bindings::QueryOperand,
    ) -> u32;
    fn __pgrx_bitfield__set_volatile__QueryOperand__length(
        p: *mut crate::__pgrx_c_bindings::QueryOperand,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned__QueryOperand__length(
        p: *const crate::__pgrx_c_bindings::QueryOperand,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned__QueryOperand__length(
        p: *mut crate::__pgrx_c_bindings::QueryOperand,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned_volatile__QueryOperand__length(
        p: *const crate::__pgrx_c_bindings::QueryOperand,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned_volatile__QueryOperand__length(
        p: *mut crate::__pgrx_c_bindings::QueryOperand,
        value: u32,
    ) -> u32;
}
#[doc(hidden)]
pub struct Bitfield_QueryOperand__length<Q: c::expression::Qualifier> {
    address: Q::Raw<crate::__pgrx_c_bindings::QueryOperand>,
    access: c::expression::Access,
}
impl<Q: c::expression::Qualifier> Copy for Bitfield_QueryOperand__length<Q> {}
impl<Q: c::expression::Qualifier> Clone for Bitfield_QueryOperand__length<Q> {
    fn clone(&self) -> Self {
        *self
    }
}
// SAFETY: Clang and bindgen layout witnesses agree. This projects the record address without a reference or record read.
unsafe impl<Q: c::expression::Qualifier> c::expression::Field<Field_length, Q>
    for c::expression::CRecord<crate::__pgrx_c_bindings::QueryOperand>
{
    type Output = Bitfield_QueryOperand__length<Q>;
    unsafe fn project(base: c::expression::Place<Self, Q>) -> Self::Output {
        Bitfield_QueryOperand__length::<Q> {
            address: <Q as c::expression::Qualifier>::from_mut(base.pointer().as_mut_address()),
            access: c::expression::Access {
                volatile: base.access().volatile || false,
                unaligned: base.access().unaligned,
            },
        }
    }
}
