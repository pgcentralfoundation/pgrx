// Shared generated C macro support.

#[cfg(any(feature = "pg18", feature = "pg19"))]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__GetRelationPath"]
    fn raw_inline_GetRelationPath(
        arg0: crate::__pgrx_c_bindings::Oid,
        arg1: crate::__pgrx_c_bindings::Oid,
        arg2: crate::__pgrx_c_bindings::Oid,
        arg3: i32,
        arg4: i32,
    ) -> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::RelPathStr>;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc = "Call the original C function `GetRelationPath`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_GetRelationPath(
    arg0: crate::__pgrx_c_bindings::Oid,
    arg1: crate::__pgrx_c_bindings::Oid,
    arg2: crate::__pgrx_c_bindings::Oid,
    arg3: i32,
    arg4: i32,
) -> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::RelPathStr> {
    const _: () = assert!(!::core::mem::needs_drop::<crate::__pgrx_c_bindings::Oid>());
    const _: () = assert!(!::core::mem::needs_drop::<crate::__pgrx_c_bindings::Oid>());
    const _: () = assert!(!::core::mem::needs_drop::<crate::__pgrx_c_bindings::Oid>());
    const _: () = assert!(!::core::mem::needs_drop::<i32>());
    const _: () = assert!(!::core::mem::needs_drop::<i32>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe {
        crate::ffi::pg_guard_ffi_boundary(move || {
            raw_inline_GetRelationPath(arg0, arg1, arg2, arg3, arg4)
        })
    }
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_SnapBuildState;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for EnumIdentity_SnapBuildState {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::EnumIdentity for EnumIdentity_SnapBuildState {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::enumeration::NumericEnum for EnumIdentity_SnapBuildState {
    type Compatible = c::CInt;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__CompareType;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for EnumIdentity_enum__CompareType {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::EnumIdentity for EnumIdentity_enum__CompareType {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__CompareType {
    type Compatible = c::CUnsignedInt;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__CopyLogVerbosityChoice {
    type Compatible = c::CInt;
}
#[cfg(feature = "pg18")]
// SAFETY: The explicit Rust integer repr matches Clang's compatible enum type and the assertions prove layout. Decoding receives a valid unit Rust variant; encoding checks every discriminant.
unsafe impl c::expression::EnumStorage<EnumIdentity_enum__NodeTag, c::CUnsignedInt>
    for crate::__pgrx_c_bindings::NodeTag
{
    fn decode(value: Self) -> u32 {
        value as u32
    }
    fn encode(value: u32) -> Self {
        match value {
            148 => crate::__pgrx_c_bindings::NodeTag::T_ATAlterConstraint,
            80 => crate::__pgrx_c_bindings::NodeTag::T_A_ArrayExpr,
            72 => crate::__pgrx_c_bindings::NodeTag::T_A_Const,
            71 => crate::__pgrx_c_bindings::NodeTag::T_A_Expr,
            78 => crate::__pgrx_c_bindings::NodeTag::T_A_Indices,
            79 => crate::__pgrx_c_bindings::NodeTag::T_A_Indirection,
            77 => crate::__pgrx_c_bindings::NodeTag::T_A_Star,
            154 => crate::__pgrx_c_bindings::NodeTag::T_AccessPriv,
            365 => crate::__pgrx_c_bindings::NodeTag::T_Agg,
            327 => crate::__pgrx_c_bindings::NodeTag::T_AggInfo,
            307 => crate::__pgrx_c_bindings::NodeTag::T_AggPath,
            429 => crate::__pgrx_c_bindings::NodeTag::T_AggState,
            328 => crate::__pgrx_c_bindings::NodeTag::T_AggTransInfo,
            9 => crate::__pgrx_c_bindings::NodeTag::T_Aggref,
            2 => crate::__pgrx_c_bindings::NodeTag::T_Alias,
            474 => crate::__pgrx_c_bindings::NodeTag::T_AllocSetContext,
            150 => crate::__pgrx_c_bindings::NodeTag::T_AlterCollationStmt,
            234 => crate::__pgrx_c_bindings::NodeTag::T_AlterDatabaseRefreshCollStmt,
            235 => crate::__pgrx_c_bindings::NodeTag::T_AlterDatabaseSetStmt,
            233 => crate::__pgrx_c_bindings::NodeTag::T_AlterDatabaseStmt,
            156 => crate::__pgrx_c_bindings::NodeTag::T_AlterDefaultPrivilegesStmt,
            151 => crate::__pgrx_c_bindings::NodeTag::T_AlterDomainStmt,
            229 => crate::__pgrx_c_bindings::NodeTag::T_AlterEnumStmt,
            183 => crate::__pgrx_c_bindings::NodeTag::T_AlterEventTrigStmt,
            168 => crate::__pgrx_c_bindings::NodeTag::T_AlterExtensionContentsStmt,
            167 => crate::__pgrx_c_bindings::NodeTag::T_AlterExtensionStmt,
            170 => crate::__pgrx_c_bindings::NodeTag::T_AlterFdwStmt,
            172 => crate::__pgrx_c_bindings::NodeTag::T_AlterForeignServerStmt,
            210 => crate::__pgrx_c_bindings::NodeTag::T_AlterFunctionStmt,
            216 => crate::__pgrx_c_bindings::NodeTag::T_AlterObjectDependsStmt,
            217 => crate::__pgrx_c_bindings::NodeTag::T_AlterObjectSchemaStmt,
            196 => crate::__pgrx_c_bindings::NodeTag::T_AlterOpFamilyStmt,
            219 => crate::__pgrx_c_bindings::NodeTag::T_AlterOperatorStmt,
            218 => crate::__pgrx_c_bindings::NodeTag::T_AlterOwnerStmt,
            179 => crate::__pgrx_c_bindings::NodeTag::T_AlterPolicyStmt,
            262 => crate::__pgrx_c_bindings::NodeTag::T_AlterPublicationStmt,
            452 => crate::__pgrx_c_bindings::NodeTag::T_AlterReplicationSlotCmd,
            187 => crate::__pgrx_c_bindings::NodeTag::T_AlterRoleSetStmt,
            186 => crate::__pgrx_c_bindings::NodeTag::T_AlterRoleStmt,
            190 => crate::__pgrx_c_bindings::NodeTag::T_AlterSeqStmt,
            207 => crate::__pgrx_c_bindings::NodeTag::T_AlterStatsStmt,
            264 => crate::__pgrx_c_bindings::NodeTag::T_AlterSubscriptionStmt,
            237 => crate::__pgrx_c_bindings::NodeTag::T_AlterSystemStmt,
            258 => crate::__pgrx_c_bindings::NodeTag::T_AlterTSConfigurationStmt,
            257 => crate::__pgrx_c_bindings::NodeTag::T_AlterTSDictionaryStmt,
            147 => crate::__pgrx_c_bindings::NodeTag::T_AlterTableCmd,
            165 => crate::__pgrx_c_bindings::NodeTag::T_AlterTableMoveAllStmt,
            164 => crate::__pgrx_c_bindings::NodeTag::T_AlterTableSpaceOptionsStmt,
            146 => crate::__pgrx_c_bindings::NodeTag::T_AlterTableStmt,
            220 => crate::__pgrx_c_bindings::NodeTag::T_AlterTypeStmt,
            175 => crate::__pgrx_c_bindings::NodeTag::T_AlterUserMappingStmt,
            24 => crate::__pgrx_c_bindings::NodeTag::T_AlternativeSubPlan,
            334 => crate::__pgrx_c_bindings::NodeTag::T_Append,
            290 => crate::__pgrx_c_bindings::NodeTag::T_AppendPath,
            322 => crate::__pgrx_c_bindings::NodeTag::T_AppendRelInfo,
            397 => crate::__pgrx_c_bindings::NodeTag::T_AppendState,
            29 => crate::__pgrx_c_bindings::NodeTag::T_ArrayCoerceExpr,
            35 => crate::__pgrx_c_bindings::NodeTag::T_ArrayExpr,
            449 => crate::__pgrx_c_bindings::NodeTag::T_BaseBackupCmd,
            469 => crate::__pgrx_c_bindings::NodeTag::T_BitString,
            337 => crate::__pgrx_c_bindings::NodeTag::T_BitmapAnd,
            283 => crate::__pgrx_c_bindings::NodeTag::T_BitmapAndPath,
            400 => crate::__pgrx_c_bindings::NodeTag::T_BitmapAndState,
            282 => crate::__pgrx_c_bindings::NodeTag::T_BitmapHeapPath,
            344 => crate::__pgrx_c_bindings::NodeTag::T_BitmapHeapScan,
            408 => crate::__pgrx_c_bindings::NodeTag::T_BitmapHeapScanState,
            343 => crate::__pgrx_c_bindings::NodeTag::T_BitmapIndexScan,
            407 => crate::__pgrx_c_bindings::NodeTag::T_BitmapIndexScanState,
            338 => crate::__pgrx_c_bindings::NodeTag::T_BitmapOr,
            284 => crate::__pgrx_c_bindings::NodeTag::T_BitmapOrPath,
            401 => crate::__pgrx_c_bindings::NodeTag::T_BitmapOrState,
            445 => crate::__pgrx_c_bindings::NodeTag::T_Bitmapset,
            21 => crate::__pgrx_c_bindings::NodeTag::T_BoolExpr,
            467 => crate::__pgrx_c_bindings::NodeTag::T_Boolean,
            53 => crate::__pgrx_c_bindings::NodeTag::T_BooleanTest,
            477 => crate::__pgrx_c_bindings::NodeTag::T_BumpContext,
            114 => crate::__pgrx_c_bindings::NodeTag::T_CTECycleClause,
            113 => crate::__pgrx_c_bindings::NodeTag::T_CTESearchClause,
            214 => crate::__pgrx_c_bindings::NodeTag::T_CallContext,
            213 => crate::__pgrx_c_bindings::NodeTag::T_CallStmt,
            32 => crate::__pgrx_c_bindings::NodeTag::T_CaseExpr,
            34 => crate::__pgrx_c_bindings::NodeTag::T_CaseTestExpr,
            33 => crate::__pgrx_c_bindings::NodeTag::T_CaseWhen,
            244 => crate::__pgrx_c_bindings::NodeTag::T_CheckPointStmt,
            202 => crate::__pgrx_c_bindings::NodeTag::T_ClosePortalStmt,
            238 => crate::__pgrx_c_bindings::NodeTag::T_ClusterStmt,
            38 => crate::__pgrx_c_bindings::NodeTag::T_CoalesceExpr,
            55 => crate::__pgrx_c_bindings::NodeTag::T_CoerceToDomain,
            56 => crate::__pgrx_c_bindings::NodeTag::T_CoerceToDomainValue,
            28 => crate::__pgrx_c_bindings::NodeTag::T_CoerceViaIO,
            74 => crate::__pgrx_c_bindings::NodeTag::T_CollateClause,
            31 => crate::__pgrx_c_bindings::NodeTag::T_CollateExpr,
            90 => crate::__pgrx_c_bindings::NodeTag::T_ColumnDef,
            69 => crate::__pgrx_c_bindings::NodeTag::T_ColumnRef,
            199 => crate::__pgrx_c_bindings::NodeTag::T_CommentStmt,
            115 => crate::__pgrx_c_bindings::NodeTag::T_CommonTableExpr,
            226 => crate::__pgrx_c_bindings::NodeTag::T_CompositeTypeStmt,
            7 => crate::__pgrx_c_bindings::NodeTag::T_Const,
            161 => crate::__pgrx_c_bindings::NodeTag::T_Constraint,
            247 => crate::__pgrx_c_bindings::NodeTag::T_ConstraintsSetStmt,
            30 => crate::__pgrx_c_bindings::NodeTag::T_ConvertRowtypeExpr,
            157 => crate::__pgrx_c_bindings::NodeTag::T_CopyStmt,
            180 => crate::__pgrx_c_bindings::NodeTag::T_CreateAmStmt,
            250 => crate::__pgrx_c_bindings::NodeTag::T_CreateCastStmt,
            249 => crate::__pgrx_c_bindings::NodeTag::T_CreateConversionStmt,
            192 => crate::__pgrx_c_bindings::NodeTag::T_CreateDomainStmt,
            227 => crate::__pgrx_c_bindings::NodeTag::T_CreateEnumStmt,
            182 => crate::__pgrx_c_bindings::NodeTag::T_CreateEventTrigStmt,
            166 => crate::__pgrx_c_bindings::NodeTag::T_CreateExtensionStmt,
            169 => crate::__pgrx_c_bindings::NodeTag::T_CreateFdwStmt,
            171 => crate::__pgrx_c_bindings::NodeTag::T_CreateForeignServerStmt,
            173 => crate::__pgrx_c_bindings::NodeTag::T_CreateForeignTableStmt,
            208 => crate::__pgrx_c_bindings::NodeTag::T_CreateFunctionStmt,
            194 => crate::__pgrx_c_bindings::NodeTag::T_CreateOpClassItem,
            193 => crate::__pgrx_c_bindings::NodeTag::T_CreateOpClassStmt,
            195 => crate::__pgrx_c_bindings::NodeTag::T_CreateOpFamilyStmt,
            184 => crate::__pgrx_c_bindings::NodeTag::T_CreatePLangStmt,
            178 => crate::__pgrx_c_bindings::NodeTag::T_CreatePolicyStmt,
            261 => crate::__pgrx_c_bindings::NodeTag::T_CreatePublicationStmt,
            228 => crate::__pgrx_c_bindings::NodeTag::T_CreateRangeStmt,
            450 => crate::__pgrx_c_bindings::NodeTag::T_CreateReplicationSlotCmd,
            185 => crate::__pgrx_c_bindings::NodeTag::T_CreateRoleStmt,
            145 => crate::__pgrx_c_bindings::NodeTag::T_CreateSchemaStmt,
            189 => crate::__pgrx_c_bindings::NodeTag::T_CreateSeqStmt,
            205 => crate::__pgrx_c_bindings::NodeTag::T_CreateStatsStmt,
            160 => crate::__pgrx_c_bindings::NodeTag::T_CreateStmt,
            263 => crate::__pgrx_c_bindings::NodeTag::T_CreateSubscriptionStmt,
            242 => crate::__pgrx_c_bindings::NodeTag::T_CreateTableAsStmt,
            162 => crate::__pgrx_c_bindings::NodeTag::T_CreateTableSpaceStmt,
            251 => crate::__pgrx_c_bindings::NodeTag::T_CreateTransformStmt,
            181 => crate::__pgrx_c_bindings::NodeTag::T_CreateTrigStmt,
            174 => crate::__pgrx_c_bindings::NodeTag::T_CreateUserMappingStmt,
            232 => crate::__pgrx_c_bindings::NodeTag::T_CreatedbStmt,
            351 => crate::__pgrx_c_bindings::NodeTag::T_CteScan,
            415 => crate::__pgrx_c_bindings::NodeTag::T_CteScanState,
            58 => crate::__pgrx_c_bindings::NodeTag::T_CurrentOfExpr,
            289 => crate::__pgrx_c_bindings::NodeTag::T_CustomPath,
            355 => crate::__pgrx_c_bindings::NodeTag::T_CustomScan,
            419 => crate::__pgrx_c_bindings::NodeTag::T_CustomScanState,
            254 => crate::__pgrx_c_bindings::NodeTag::T_DeallocateStmt,
            201 => crate::__pgrx_c_bindings::NodeTag::T_DeclareCursorStmt,
            93 => crate::__pgrx_c_bindings::NodeTag::T_DefElem,
            191 => crate::__pgrx_c_bindings::NodeTag::T_DefineStmt,
            138 => crate::__pgrx_c_bindings::NodeTag::T_DeleteStmt,
            245 => crate::__pgrx_c_bindings::NodeTag::T_DiscardStmt,
            18 => crate::__pgrx_c_bindings::NodeTag::T_DistinctExpr,
            211 => crate::__pgrx_c_bindings::NodeTag::T_DoStmt,
            393 => crate::__pgrx_c_bindings::NodeTag::T_DomainConstraintState,
            255 => crate::__pgrx_c_bindings::NodeTag::T_DropOwnedStmt,
            451 => crate::__pgrx_c_bindings::NodeTag::T_DropReplicationSlotCmd,
            188 => crate::__pgrx_c_bindings::NodeTag::T_DropRoleStmt,
            197 => crate::__pgrx_c_bindings::NodeTag::T_DropStmt,
            265 => crate::__pgrx_c_bindings::NodeTag::T_DropSubscriptionStmt,
            163 => crate::__pgrx_c_bindings::NodeTag::T_DropTableSpaceStmt,
            176 => crate::__pgrx_c_bindings::NodeTag::T_DropUserMappingStmt,
            236 => crate::__pgrx_c_bindings::NodeTag::T_DropdbStmt,
            389 => crate::__pgrx_c_bindings::NodeTag::T_EState,
            273 => crate::__pgrx_c_bindings::NodeTag::T_EquivalenceClass,
            274 => crate::__pgrx_c_bindings::NodeTag::T_EquivalenceMember,
            447 => crate::__pgrx_c_bindings::NodeTag::T_ErrorSaveContext,
            441 => crate::__pgrx_c_bindings::NodeTag::T_EventTriggerData,
            253 => crate::__pgrx_c_bindings::NodeTag::T_ExecuteStmt,
            241 => crate::__pgrx_c_bindings::NodeTag::T_ExplainStmt,
            382 => crate::__pgrx_c_bindings::NodeTag::T_ExprContext,
            380 => crate::__pgrx_c_bindings::NodeTag::T_ExprState,
            446 => crate::__pgrx_c_bindings::NodeTag::T_ExtensibleNode,
            444 => crate::__pgrx_c_bindings::NodeTag::T_FdwRoutine,
            203 => crate::__pgrx_c_bindings::NodeTag::T_FetchStmt,
            25 => crate::__pgrx_c_bindings::NodeTag::T_FieldSelect,
            26 => crate::__pgrx_c_bindings::NodeTag::T_FieldStore,
            466 => crate::__pgrx_c_bindings::NodeTag::T_Float,
            470 => crate::__pgrx_c_bindings::NodeTag::T_ForeignKeyCacheInfo,
            270 => crate::__pgrx_c_bindings::NodeTag::T_ForeignKeyOptInfo,
            288 => crate::__pgrx_c_bindings::NodeTag::T_ForeignPath,
            354 => crate::__pgrx_c_bindings::NodeTag::T_ForeignScan,
            418 => crate::__pgrx_c_bindings::NodeTag::T_ForeignScanState,
            65 => crate::__pgrx_c_bindings::NodeTag::T_FromExpr,
            76 => crate::__pgrx_c_bindings::NodeTag::T_FuncCall,
            15 => crate::__pgrx_c_bindings::NodeTag::T_FuncExpr,
            209 => crate::__pgrx_c_bindings::NodeTag::T_FunctionParameter,
            348 => crate::__pgrx_c_bindings::NodeTag::T_FunctionScan,
            412 => crate::__pgrx_c_bindings::NodeTag::T_FunctionScanState,
            368 => crate::__pgrx_c_bindings::NodeTag::T_Gather,
            369 => crate::__pgrx_c_bindings::NodeTag::T_GatherMerge,
            297 => crate::__pgrx_c_bindings::NodeTag::T_GatherMergePath,
            433 => crate::__pgrx_c_bindings::NodeTag::T_GatherMergeState,
            296 => crate::__pgrx_c_bindings::NodeTag::T_GatherPath,
            432 => crate::__pgrx_c_bindings::NodeTag::T_GatherState,
            475 => crate::__pgrx_c_bindings::NodeTag::T_GenerationContext,
            155 => crate::__pgrx_c_bindings::NodeTag::T_GrantRoleStmt,
            152 => crate::__pgrx_c_bindings::NodeTag::T_GrantStmt,
            364 => crate::__pgrx_c_bindings::NodeTag::T_Group,
            276 => crate::__pgrx_c_bindings::NodeTag::T_GroupByOrdering,
            305 => crate::__pgrx_c_bindings::NodeTag::T_GroupPath,
            292 => crate::__pgrx_c_bindings::NodeTag::T_GroupResultPath,
            428 => crate::__pgrx_c_bindings::NodeTag::T_GroupState,
            10 => crate::__pgrx_c_bindings::NodeTag::T_GroupingFunc,
            107 => crate::__pgrx_c_bindings::NodeTag::T_GroupingSet,
            308 => crate::__pgrx_c_bindings::NodeTag::T_GroupingSetData,
            310 => crate::__pgrx_c_bindings::NodeTag::T_GroupingSetsPath,
            370 => crate::__pgrx_c_bindings::NodeTag::T_Hash,
            359 => crate::__pgrx_c_bindings::NodeTag::T_HashJoin,
            423 => crate::__pgrx_c_bindings::NodeTag::T_HashJoinState,
            300 => crate::__pgrx_c_bindings::NodeTag::T_HashPath,
            434 => crate::__pgrx_c_bindings::NodeTag::T_HashState,
            448 => crate::__pgrx_c_bindings::NodeTag::T_IdentifySystemCmd,
            177 => crate::__pgrx_c_bindings::NodeTag::T_ImportForeignSchemaStmt,
            363 => crate::__pgrx_c_bindings::NodeTag::T_IncrementalSort,
            304 => crate::__pgrx_c_bindings::NodeTag::T_IncrementalSortPath,
            427 => crate::__pgrx_c_bindings::NodeTag::T_IncrementalSortState,
            438 => crate::__pgrx_c_bindings::NodeTag::T_IndexAmRoutine,
            281 => crate::__pgrx_c_bindings::NodeTag::T_IndexClause,
            92 => crate::__pgrx_c_bindings::NodeTag::T_IndexElem,
            381 => crate::__pgrx_c_bindings::NodeTag::T_IndexInfo,
            342 => crate::__pgrx_c_bindings::NodeTag::T_IndexOnlyScan,
            406 => crate::__pgrx_c_bindings::NodeTag::T_IndexOnlyScanState,
            269 => crate::__pgrx_c_bindings::NodeTag::T_IndexOptInfo,
            280 => crate::__pgrx_c_bindings::NodeTag::T_IndexPath,
            341 => crate::__pgrx_c_bindings::NodeTag::T_IndexScan,
            405 => crate::__pgrx_c_bindings::NodeTag::T_IndexScanState,
            204 => crate::__pgrx_c_bindings::NodeTag::T_IndexStmt,
            111 => crate::__pgrx_c_bindings::NodeTag::T_InferClause,
            60 => crate::__pgrx_c_bindings::NodeTag::T_InferenceElem,
            212 => crate::__pgrx_c_bindings::NodeTag::T_InlineCodeBlock,
            137 => crate::__pgrx_c_bindings::NodeTag::T_InsertStmt,
            471 => crate::__pgrx_c_bindings::NodeTag::T_IntList,
            465 => crate::__pgrx_c_bindings::NodeTag::T_Integer,
            5 => crate::__pgrx_c_bindings::NodeTag::T_IntoClause,
            0 => crate::__pgrx_c_bindings::NodeTag::T_Invalid,
            272 => crate::__pgrx_c_bindings::NodeTag::T_JoinDomain,
            64 => crate::__pgrx_c_bindings::NodeTag::T_JoinExpr,
            420 => crate::__pgrx_c_bindings::NodeTag::T_JoinState,
            133 => crate::__pgrx_c_bindings::NodeTag::T_JsonAggConstructor,
            121 => crate::__pgrx_c_bindings::NodeTag::T_JsonArgument,
            135 => crate::__pgrx_c_bindings::NodeTag::T_JsonArrayAgg,
            131 => crate::__pgrx_c_bindings::NodeTag::T_JsonArrayConstructor,
            132 => crate::__pgrx_c_bindings::NodeTag::T_JsonArrayQueryConstructor,
            47 => crate::__pgrx_c_bindings::NodeTag::T_JsonBehavior,
            45 => crate::__pgrx_c_bindings::NodeTag::T_JsonConstructorExpr,
            48 => crate::__pgrx_c_bindings::NodeTag::T_JsonExpr,
            42 => crate::__pgrx_c_bindings::NodeTag::T_JsonFormat,
            122 => crate::__pgrx_c_bindings::NodeTag::T_JsonFuncExpr,
            46 => crate::__pgrx_c_bindings::NodeTag::T_JsonIsPredicate,
            126 => crate::__pgrx_c_bindings::NodeTag::T_JsonKeyValue,
            134 => crate::__pgrx_c_bindings::NodeTag::T_JsonObjectAgg,
            130 => crate::__pgrx_c_bindings::NodeTag::T_JsonObjectConstructor,
            120 => crate::__pgrx_c_bindings::NodeTag::T_JsonOutput,
            127 => crate::__pgrx_c_bindings::NodeTag::T_JsonParseExpr,
            43 => crate::__pgrx_c_bindings::NodeTag::T_JsonReturning,
            128 => crate::__pgrx_c_bindings::NodeTag::T_JsonScalarExpr,
            129 => crate::__pgrx_c_bindings::NodeTag::T_JsonSerializeExpr,
            124 => crate::__pgrx_c_bindings::NodeTag::T_JsonTable,
            125 => crate::__pgrx_c_bindings::NodeTag::T_JsonTableColumn,
            49 => crate::__pgrx_c_bindings::NodeTag::T_JsonTablePath,
            50 => crate::__pgrx_c_bindings::NodeTag::T_JsonTablePathScan,
            123 => crate::__pgrx_c_bindings::NodeTag::T_JsonTablePathSpec,
            51 => crate::__pgrx_c_bindings::NodeTag::T_JsonTableSiblingJoin,
            44 => crate::__pgrx_c_bindings::NodeTag::T_JsonValueExpr,
            385 => crate::__pgrx_c_bindings::NodeTag::T_JunkFilter,
            373 => crate::__pgrx_c_bindings::NodeTag::T_Limit,
            317 => crate::__pgrx_c_bindings::NodeTag::T_LimitPath,
            437 => crate::__pgrx_c_bindings::NodeTag::T_LimitState,
            1 => crate::__pgrx_c_bindings::NodeTag::T_List,
            223 => crate::__pgrx_c_bindings::NodeTag::T_ListenStmt,
            231 => crate::__pgrx_c_bindings::NodeTag::T_LoadStmt,
            372 => crate::__pgrx_c_bindings::NodeTag::T_LockRows,
            315 => crate::__pgrx_c_bindings::NodeTag::T_LockRowsPath,
            436 => crate::__pgrx_c_bindings::NodeTag::T_LockRowsState,
            246 => crate::__pgrx_c_bindings::NodeTag::T_LockStmt,
            94 => crate::__pgrx_c_bindings::NodeTag::T_LockingClause,
            360 => crate::__pgrx_c_bindings::NodeTag::T_Material,
            293 => crate::__pgrx_c_bindings::NodeTag::T_MaterialPath,
            424 => crate::__pgrx_c_bindings::NodeTag::T_MaterialState,
            361 => crate::__pgrx_c_bindings::NodeTag::T_Memoize,
            294 => crate::__pgrx_c_bindings::NodeTag::T_MemoizePath,
            425 => crate::__pgrx_c_bindings::NodeTag::T_MemoizeState,
            54 => crate::__pgrx_c_bindings::NodeTag::T_MergeAction,
            387 => crate::__pgrx_c_bindings::NodeTag::T_MergeActionState,
            335 => crate::__pgrx_c_bindings::NodeTag::T_MergeAppend,
            291 => crate::__pgrx_c_bindings::NodeTag::T_MergeAppendPath,
            398 => crate::__pgrx_c_bindings::NodeTag::T_MergeAppendState,
            358 => crate::__pgrx_c_bindings::NodeTag::T_MergeJoin,
            422 => crate::__pgrx_c_bindings::NodeTag::T_MergeJoinState,
            299 => crate::__pgrx_c_bindings::NodeTag::T_MergePath,
            140 => crate::__pgrx_c_bindings::NodeTag::T_MergeStmt,
            13 => crate::__pgrx_c_bindings::NodeTag::T_MergeSupportFunc,
            116 => crate::__pgrx_c_bindings::NodeTag::T_MergeWhenClause,
            325 => crate::__pgrx_c_bindings::NodeTag::T_MinMaxAggInfo,
            311 => crate::__pgrx_c_bindings::NodeTag::T_MinMaxAggPath,
            39 => crate::__pgrx_c_bindings::NodeTag::T_MinMaxExpr,
            333 => crate::__pgrx_c_bindings::NodeTag::T_ModifyTable,
            316 => crate::__pgrx_c_bindings::NodeTag::T_ModifyTablePath,
            396 => crate::__pgrx_c_bindings::NodeTag::T_ModifyTableState,
            82 => crate::__pgrx_c_bindings::NodeTag::T_MultiAssignRef,
            16 => crate::__pgrx_c_bindings::NodeTag::T_NamedArgExpr,
            352 => crate::__pgrx_c_bindings::NodeTag::T_NamedTuplestoreScan,
            416 => crate::__pgrx_c_bindings::NodeTag::T_NamedTuplestoreScanState,
            356 => crate::__pgrx_c_bindings::NodeTag::T_NestLoop,
            357 => crate::__pgrx_c_bindings::NodeTag::T_NestLoopParam,
            421 => crate::__pgrx_c_bindings::NodeTag::T_NestLoopState,
            298 => crate::__pgrx_c_bindings::NodeTag::T_NestPath,
            59 => crate::__pgrx_c_bindings::NodeTag::T_NextValueExpr,
            222 => crate::__pgrx_c_bindings::NodeTag::T_NotifyStmt,
            19 => crate::__pgrx_c_bindings::NodeTag::T_NullIfExpr,
            52 => crate::__pgrx_c_bindings::NodeTag::T_NullTest,
            153 => crate::__pgrx_c_bindings::NodeTag::T_ObjectWithArgs,
            472 => crate::__pgrx_c_bindings::NodeTag::T_OidList,
            112 => crate::__pgrx_c_bindings::NodeTag::T_OnConflictClause,
            66 => crate::__pgrx_c_bindings::NodeTag::T_OnConflictExpr,
            386 => crate::__pgrx_c_bindings::NodeTag::T_OnConflictSetState,
            17 => crate::__pgrx_c_bindings::NodeTag::T_OpExpr,
            321 => crate::__pgrx_c_bindings::NodeTag::T_OuterJoinClauseInfo,
            144 => crate::__pgrx_c_bindings::NodeTag::T_PLAssignStmt,
            8 => crate::__pgrx_c_bindings::NodeTag::T_Param,
            278 => crate::__pgrx_c_bindings::NodeTag::T_ParamPathInfo,
            70 => crate::__pgrx_c_bindings::NodeTag::T_ParamRef,
            98 => crate::__pgrx_c_bindings::NodeTag::T_PartitionBoundSpec,
            100 => crate::__pgrx_c_bindings::NodeTag::T_PartitionCmd,
            96 => crate::__pgrx_c_bindings::NodeTag::T_PartitionElem,
            375 => crate::__pgrx_c_bindings::NodeTag::T_PartitionPruneInfo,
            378 => crate::__pgrx_c_bindings::NodeTag::T_PartitionPruneStepCombine,
            377 => crate::__pgrx_c_bindings::NodeTag::T_PartitionPruneStepOp,
            99 => crate::__pgrx_c_bindings::NodeTag::T_PartitionRangeDatum,
            97 => crate::__pgrx_c_bindings::NodeTag::T_PartitionSpec,
            376 => crate::__pgrx_c_bindings::NodeTag::T_PartitionedRelPruneInfo,
            279 => crate::__pgrx_c_bindings::NodeTag::T_Path,
            275 => crate::__pgrx_c_bindings::NodeTag::T_PathKey,
            277 => crate::__pgrx_c_bindings::NodeTag::T_PathTarget,
            324 => crate::__pgrx_c_bindings::NodeTag::T_PlaceHolderInfo,
            319 => crate::__pgrx_c_bindings::NodeTag::T_PlaceHolderVar,
            379 => crate::__pgrx_c_bindings::NodeTag::T_PlanInvalItem,
            374 => crate::__pgrx_c_bindings::NodeTag::T_PlanRowMark,
            330 => crate::__pgrx_c_bindings::NodeTag::T_PlannedStmt,
            266 => crate::__pgrx_c_bindings::NodeTag::T_PlannerGlobal,
            267 => crate::__pgrx_c_bindings::NodeTag::T_PlannerInfo,
            326 => crate::__pgrx_c_bindings::NodeTag::T_PlannerParamItem,
            252 => crate::__pgrx_c_bindings::NodeTag::T_PrepareStmt,
            332 => crate::__pgrx_c_bindings::NodeTag::T_ProjectSet,
            302 => crate::__pgrx_c_bindings::NodeTag::T_ProjectSetPath,
            395 => crate::__pgrx_c_bindings::NodeTag::T_ProjectSetState,
            384 => crate::__pgrx_c_bindings::NodeTag::T_ProjectionInfo,
            301 => crate::__pgrx_c_bindings::NodeTag::T_ProjectionPath,
            260 => crate::__pgrx_c_bindings::NodeTag::T_PublicationObjSpec,
            259 => crate::__pgrx_c_bindings::NodeTag::T_PublicationTable,
            67 => crate::__pgrx_c_bindings::NodeTag::T_Query,
            102 => crate::__pgrx_c_bindings::NodeTag::T_RTEPermissionInfo,
            86 => crate::__pgrx_c_bindings::NodeTag::T_RangeFunction,
            85 => crate::__pgrx_c_bindings::NodeTag::T_RangeSubselect,
            87 => crate::__pgrx_c_bindings::NodeTag::T_RangeTableFunc,
            88 => crate::__pgrx_c_bindings::NodeTag::T_RangeTableFuncCol,
            89 => crate::__pgrx_c_bindings::NodeTag::T_RangeTableSample,
            101 => crate::__pgrx_c_bindings::NodeTag::T_RangeTblEntry,
            103 => crate::__pgrx_c_bindings::NodeTag::T_RangeTblFunction,
            63 => crate::__pgrx_c_bindings::NodeTag::T_RangeTblRef,
            3 => crate::__pgrx_c_bindings::NodeTag::T_RangeVar,
            136 => crate::__pgrx_c_bindings::NodeTag::T_RawStmt,
            454 => crate::__pgrx_c_bindings::NodeTag::T_ReadReplicationSlotCmd,
            256 => crate::__pgrx_c_bindings::NodeTag::T_ReassignOwnedStmt,
            336 => crate::__pgrx_c_bindings::NodeTag::T_RecursiveUnion,
            314 => crate::__pgrx_c_bindings::NodeTag::T_RecursiveUnionPath,
            399 => crate::__pgrx_c_bindings::NodeTag::T_RecursiveUnionState,
            243 => crate::__pgrx_c_bindings::NodeTag::T_RefreshMatViewStmt,
            248 => crate::__pgrx_c_bindings::NodeTag::T_ReindexStmt,
            268 => crate::__pgrx_c_bindings::NodeTag::T_RelOptInfo,
            27 => crate::__pgrx_c_bindings::NodeTag::T_RelabelType,
            215 => crate::__pgrx_c_bindings::NodeTag::T_RenameStmt,
            149 => crate::__pgrx_c_bindings::NodeTag::T_ReplicaIdentityStmt,
            81 => crate::__pgrx_c_bindings::NodeTag::T_ResTarget,
            318 => crate::__pgrx_c_bindings::NodeTag::T_RestrictInfo,
            331 => crate::__pgrx_c_bindings::NodeTag::T_Result,
            388 => crate::__pgrx_c_bindings::NodeTag::T_ResultRelInfo,
            394 => crate::__pgrx_c_bindings::NodeTag::T_ResultState,
            383 => crate::__pgrx_c_bindings::NodeTag::T_ReturnSetInfo,
            143 => crate::__pgrx_c_bindings::NodeTag::T_ReturnStmt,
            118 => crate::__pgrx_c_bindings::NodeTag::T_ReturningClause,
            61 => crate::__pgrx_c_bindings::NodeTag::T_ReturningExpr,
            117 => crate::__pgrx_c_bindings::NodeTag::T_ReturningOption,
            75 => crate::__pgrx_c_bindings::NodeTag::T_RoleSpec,
            309 => crate::__pgrx_c_bindings::NodeTag::T_RollupData,
            37 => crate::__pgrx_c_bindings::NodeTag::T_RowCompareExpr,
            36 => crate::__pgrx_c_bindings::NodeTag::T_RowExpr,
            323 => crate::__pgrx_c_bindings::NodeTag::T_RowIdentityVarInfo,
            109 => crate::__pgrx_c_bindings::NodeTag::T_RowMarkClause,
            221 => crate::__pgrx_c_bindings::NodeTag::T_RuleStmt,
            40 => crate::__pgrx_c_bindings::NodeTag::T_SQLValueFunction,
            340 => crate::__pgrx_c_bindings::NodeTag::T_SampleScan,
            404 => crate::__pgrx_c_bindings::NodeTag::T_SampleScanState,
            20 => crate::__pgrx_c_bindings::NodeTag::T_ScalarArrayOpExpr,
            402 => crate::__pgrx_c_bindings::NodeTag::T_ScanState,
            200 => crate::__pgrx_c_bindings::NodeTag::T_SecLabelStmt,
            141 => crate::__pgrx_c_bindings::NodeTag::T_SelectStmt,
            339 => crate::__pgrx_c_bindings::NodeTag::T_SeqScan,
            403 => crate::__pgrx_c_bindings::NodeTag::T_SeqScanState,
            391 => crate::__pgrx_c_bindings::NodeTag::T_SetExprState,
            371 => crate::__pgrx_c_bindings::NodeTag::T_SetOp,
            313 => crate::__pgrx_c_bindings::NodeTag::T_SetOpPath,
            435 => crate::__pgrx_c_bindings::NodeTag::T_SetOpState,
            142 => crate::__pgrx_c_bindings::NodeTag::T_SetOperationStmt,
            57 => crate::__pgrx_c_bindings::NodeTag::T_SetToDefault,
            476 => crate::__pgrx_c_bindings::NodeTag::T_SlabContext,
            362 => crate::__pgrx_c_bindings::NodeTag::T_Sort,
            83 => crate::__pgrx_c_bindings::NodeTag::T_SortBy,
            106 => crate::__pgrx_c_bindings::NodeTag::T_SortGroupClause,
            303 => crate::__pgrx_c_bindings::NodeTag::T_SortPath,
            426 => crate::__pgrx_c_bindings::NodeTag::T_SortState,
            320 => crate::__pgrx_c_bindings::NodeTag::T_SpecialJoinInfo,
            453 => crate::__pgrx_c_bindings::NodeTag::T_StartReplicationCmd,
            271 => crate::__pgrx_c_bindings::NodeTag::T_StatisticExtInfo,
            206 => crate::__pgrx_c_bindings::NodeTag::T_StatsElem,
            468 => crate::__pgrx_c_bindings::NodeTag::T_String,
            22 => crate::__pgrx_c_bindings::NodeTag::T_SubLink,
            23 => crate::__pgrx_c_bindings::NodeTag::T_SubPlan,
            392 => crate::__pgrx_c_bindings::NodeTag::T_SubPlanState,
            347 => crate::__pgrx_c_bindings::NodeTag::T_SubqueryScan,
            287 => crate::__pgrx_c_bindings::NodeTag::T_SubqueryScanPath,
            411 => crate::__pgrx_c_bindings::NodeTag::T_SubqueryScanState,
            14 => crate::__pgrx_c_bindings::NodeTag::T_SubscriptingRef,
            459 => crate::__pgrx_c_bindings::NodeTag::T_SupportRequestCost,
            461 => crate::__pgrx_c_bindings::NodeTag::T_SupportRequestIndexCondition,
            464 => crate::__pgrx_c_bindings::NodeTag::T_SupportRequestModifyInPlace,
            463 => crate::__pgrx_c_bindings::NodeTag::T_SupportRequestOptimizeWindowClause,
            460 => crate::__pgrx_c_bindings::NodeTag::T_SupportRequestRows,
            458 => crate::__pgrx_c_bindings::NodeTag::T_SupportRequestSelectivity,
            457 => crate::__pgrx_c_bindings::NodeTag::T_SupportRequestSimplify,
            462 => crate::__pgrx_c_bindings::NodeTag::T_SupportRequestWFuncMonotonic,
            478 => crate::__pgrx_c_bindings::NodeTag::T_TIDBitmap,
            439 => crate::__pgrx_c_bindings::NodeTag::T_TableAmRoutine,
            4 => crate::__pgrx_c_bindings::NodeTag::T_TableFunc,
            350 => crate::__pgrx_c_bindings::NodeTag::T_TableFuncScan,
            414 => crate::__pgrx_c_bindings::NodeTag::T_TableFuncScanState,
            91 => crate::__pgrx_c_bindings::NodeTag::T_TableLikeClause,
            104 => crate::__pgrx_c_bindings::NodeTag::T_TableSampleClause,
            62 => crate::__pgrx_c_bindings::NodeTag::T_TargetEntry,
            285 => crate::__pgrx_c_bindings::NodeTag::T_TidPath,
            286 => crate::__pgrx_c_bindings::NodeTag::T_TidRangePath,
            346 => crate::__pgrx_c_bindings::NodeTag::T_TidRangeScan,
            410 => crate::__pgrx_c_bindings::NodeTag::T_TidRangeScanState,
            345 => crate::__pgrx_c_bindings::NodeTag::T_TidScan,
            409 => crate::__pgrx_c_bindings::NodeTag::T_TidScanState,
            455 => crate::__pgrx_c_bindings::NodeTag::T_TimeLineHistoryCmd,
            225 => crate::__pgrx_c_bindings::NodeTag::T_TransactionStmt,
            442 => crate::__pgrx_c_bindings::NodeTag::T_TriggerData,
            119 => crate::__pgrx_c_bindings::NodeTag::T_TriggerTransition,
            198 => crate::__pgrx_c_bindings::NodeTag::T_TruncateStmt,
            440 => crate::__pgrx_c_bindings::NodeTag::T_TsmRoutine,
            443 => crate::__pgrx_c_bindings::NodeTag::T_TupleTableSlot,
            73 => crate::__pgrx_c_bindings::NodeTag::T_TypeCast,
            68 => crate::__pgrx_c_bindings::NodeTag::T_TypeName,
            367 => crate::__pgrx_c_bindings::NodeTag::T_Unique,
            295 => crate::__pgrx_c_bindings::NodeTag::T_UniquePath,
            329 => crate::__pgrx_c_bindings::NodeTag::T_UniqueRelInfo,
            431 => crate::__pgrx_c_bindings::NodeTag::T_UniqueState,
            224 => crate::__pgrx_c_bindings::NodeTag::T_UnlistenStmt,
            139 => crate::__pgrx_c_bindings::NodeTag::T_UpdateStmt,
            456 => crate::__pgrx_c_bindings::NodeTag::T_UploadManifestCmd,
            306 => crate::__pgrx_c_bindings::NodeTag::T_UpperUniquePath,
            240 => crate::__pgrx_c_bindings::NodeTag::T_VacuumRelation,
            239 => crate::__pgrx_c_bindings::NodeTag::T_VacuumStmt,
            349 => crate::__pgrx_c_bindings::NodeTag::T_ValuesScan,
            413 => crate::__pgrx_c_bindings::NodeTag::T_ValuesScanState,
            6 => crate::__pgrx_c_bindings::NodeTag::T_Var,
            158 => crate::__pgrx_c_bindings::NodeTag::T_VariableSetStmt,
            159 => crate::__pgrx_c_bindings::NodeTag::T_VariableShowStmt,
            230 => crate::__pgrx_c_bindings::NodeTag::T_ViewStmt,
            366 => crate::__pgrx_c_bindings::NodeTag::T_WindowAgg,
            312 => crate::__pgrx_c_bindings::NodeTag::T_WindowAggPath,
            430 => crate::__pgrx_c_bindings::NodeTag::T_WindowAggState,
            108 => crate::__pgrx_c_bindings::NodeTag::T_WindowClause,
            84 => crate::__pgrx_c_bindings::NodeTag::T_WindowDef,
            11 => crate::__pgrx_c_bindings::NodeTag::T_WindowFunc,
            390 => crate::__pgrx_c_bindings::NodeTag::T_WindowFuncExprState,
            12 => crate::__pgrx_c_bindings::NodeTag::T_WindowFuncRunCondition,
            479 => crate::__pgrx_c_bindings::NodeTag::T_WindowObjectData,
            105 => crate::__pgrx_c_bindings::NodeTag::T_WithCheckOption,
            110 => crate::__pgrx_c_bindings::NodeTag::T_WithClause,
            353 => crate::__pgrx_c_bindings::NodeTag::T_WorkTableScan,
            417 => crate::__pgrx_c_bindings::NodeTag::T_WorkTableScanState,
            473 => crate::__pgrx_c_bindings::NodeTag::T_XidList,
            41 => crate::__pgrx_c_bindings::NodeTag::T_XmlExpr,
            95 => crate::__pgrx_c_bindings::NodeTag::T_XmlSerialize,
            _ => panic!("C enum value has no corresponding Rust variant"),
        }
    }
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__PLpgSQL_rwopt;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for EnumIdentity_enum__PLpgSQL_rwopt {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::EnumIdentity for EnumIdentity_enum__PLpgSQL_rwopt {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__PLpgSQL_rwopt {
    type Compatible = c::CUnsignedInt;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__PublishGencolsType;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for EnumIdentity_enum__PublishGencolsType {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::EnumIdentity for EnumIdentity_enum__PublishGencolsType {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__PublishGencolsType {
    type Compatible = c::CUnsignedInt;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__ReturningOptionKind;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for EnumIdentity_enum__ReturningOptionKind {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::EnumIdentity for EnumIdentity_enum__ReturningOptionKind {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__ReturningOptionKind {
    type Compatible = c::CUnsignedInt;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__VarReturningType;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for EnumIdentity_enum__VarReturningType {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::EnumIdentity for EnumIdentity_enum__VarReturningType {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__VarReturningType {
    type Compatible = c::CUnsignedInt;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerInfo,
            )
                -> *mut crate::__pgrx_c_bindings::GeqoPrivateData,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerInfo,
            )
                -> *mut crate::__pgrx_c_bindings::GeqoPrivateData,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_GeqoPrivateData__0p__0l__struct__PlannerInfo__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_GeqoPrivateData__0p__0l__struct__PlannerInfo__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_GeqoPrivateData__0p__0l__struct__PlannerInfo__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerInfo,
            )
                -> *mut crate::__pgrx_c_bindings::GeqoPrivateData,
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn() -> u32>>() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn() -> u32>>() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_TimingClockSourceType__0l__void__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_TimingClockSourceType__0l__void__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_TimingClockSourceType__0l__void__0r {
    type Physical =
        PhysicalFunction_C_unwind_0<::core::option::Option<unsafe extern "C-unwind" fn() -> u32>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                crate::__pgrx_c_bindings::NodeTag,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                crate::__pgrx_c_bindings::NodeTag,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__const__char__0p__0c__const__char__0p__0c__enum__NodeTag__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed
    for Signature_0x5f426f6f6c__0l__const__char__0p__0c__const__char__0p__0c__enum__NodeTag__0r
{
}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__const__char__0p__0c__const__char__0p__0c__enum__NodeTag__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                crate::__pgrx_c_bindings::NodeTag,
            ) -> bool,
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::List,
                i32,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::List,
                i32,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__RelationData__0p__0c__struct__List__0p__0c__int__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed
    for Signature_0x5f426f6f6c__0l__struct__RelationData__0p__0c__struct__List__0p__0c__int__0r
{
}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__struct__RelationData__0p__0c__struct__List__0p__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::List,
                i32,
            ) -> bool,
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::SlruDesc,
                *mut ::core::ffi::c_char,
                i64,
                *mut ::core::ffi::c_void,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::SlruDesc,
                *mut ::core::ffi::c_char,
                i64,
                *mut ::core::ffi::c_void,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__SlruDesc__0p__0c__char__0p__0c__long__0c__void__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed
    for Signature_0x5f426f6f6c__0l__struct__SlruDesc__0p__0c__char__0p__0c__long__0c__void__0p__0r
{
}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__struct__SlruDesc__0p__0c__char__0p__0c__long__0c__void__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_4<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::SlruDesc,
                *mut ::core::ffi::c_char,
                i64,
                *mut ::core::ffi::c_void,
            ) -> bool,
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableScanDescData,
                *mut f64,
                *mut f64,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableScanDescData,
                *mut f64,
                *mut f64,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__double__0p__0c__double__0p__0c__struct__TupleTableSlot__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__double__0p__0c__double__0p__0c__struct__TupleTableSlot__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__double__0p__0c__double__0p__0c__struct__TupleTableSlot__0p__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TableScanDescData, *mut f64, *mut f64, *mut crate::__pgrx_c_bindings::TupleTableSlot) -> bool>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                *mut crate::__pgrx_c_bindings::pg_locale_struct,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                *mut crate::__pgrx_c_bindings::pg_locale_struct,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__unsigned__int__0c__struct__pg_locale_struct__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed
    for Signature_0x5f426f6f6c__0l__unsigned__int__0c__struct__pg_locale_struct__0p__0r
{
}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__unsigned__int__0c__struct__pg_locale_struct__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                *mut crate::__pgrx_c_bindings::pg_locale_struct,
            ) -> bool,
        >,
    >;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__enum__SysCacheIdentifier__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__enum__SysCacheIdentifier__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__enum__SysCacheIdentifier__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(i32, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum) -> bool>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn(u32) -> bool>>() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn(u32) -> bool>>()
        == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__enum__vartag_external__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__enum__vartag_external__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__enum__vartag_external__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(u32) -> bool>,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::Fitness>,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::Fitness>,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__Fitness__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__Fitness__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__struct__Fitness__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::Fitness>,
            ) -> bool,
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::varatt_external>,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::varatt_external>,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__varatt_external__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__varatt_external__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__struct__varatt_external__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::varatt_external>,
            ) -> bool,
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::WalReceiverConn,
                *mut u32,
                *mut u64,
            ) -> *mut ::core::ffi::c_char,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::WalReceiverConn,
                *mut u32,
                *mut u64,
            ) -> *mut ::core::ffi::c_char,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_char__0p__0l__struct__WalReceiverConn__0p__0c__unsigned__int__0p__0c__unsigned__long__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_char__0p__0l__struct__WalReceiverConn__0p__0c__unsigned__int__0p__0c__unsigned__long__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_char__0p__0l__struct__WalReceiverConn__0p__0c__unsigned__int__0p__0c__unsigned__long__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::WalReceiverConn, *mut u32, *mut u64) -> *mut ::core::ffi::c_char>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<*mut u64>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<*mut u64>() == 8);
#[cfg(feature = "pg19")]
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<A0: c::expression::ImplicitTo<c::expression::CPointer<c::expression::COpaque<crate::__pgrx_c_bindings::WalReceiverConn>, c::expression::ReadWrite>>, A1: c::expression::ImplicitTo<c::expression::CPointer<c::CUnsignedInt, c::expression::ReadWrite>>, A2: c::expression::ImplicitTo<c::expression::CPointer<c::CUnsignedLong, c::expression::ReadWrite>>> c::expression::Call<(A0,A1,A2,)> for Signature_char__0p__0l__struct__WalReceiverConn__0p__0c__unsigned__int__0p__0c__unsigned__long__0p__0r {
 type Output = <c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadWrite> as c::expression::CType>::Value;
 unsafe fn call(pointer: Self::Pointer, args: (A0,A1,A2,)) -> Self::Output {
let function = pointer.expect("C indirect call requires a non-null function pointer");
const { assert!(!::core::mem::needs_drop::<A0>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::WalReceiverConn>());
let native0 = <c::expression::CPointer<c::expression::COpaque<crate::__pgrx_c_bindings::WalReceiverConn>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::COpaque<crate::__pgrx_c_bindings::WalReceiverConn>, c::expression::ReadWrite>, _>(args.0));
const { assert!(!::core::mem::needs_drop::<A1>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut u32>());
let native1 = <c::expression::CPointer<c::CUnsignedInt, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::CUnsignedInt, c::expression::ReadWrite>, _>(args.1));
const { assert!(!::core::mem::needs_drop::<A2>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut u64>());
let native2 = <c::expression::CPointer<c::CUnsignedLong, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::CUnsignedLong, c::expression::ReadWrite>, _>(args.2));
// SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
let result = unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0, native1, native2)) };
<c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadWrite> as c::expression::CType>::from_storage(result)
}
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const ::core::ffi::c_void) -> *mut ::core::ffi::c_char,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const ::core::ffi::c_void) -> *mut ::core::ffi::c_char,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_char__0p__0l__const__void__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_char__0p__0l__const__void__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_char__0p__0l__const__void__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const ::core::ffi::c_void) -> *mut ::core::ffi::c_char,
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(f64, f64, *mut crate::__pgrx_c_bindings::Node) -> f64,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(f64, f64, *mut crate::__pgrx_c_bindings::Node) -> f64,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_double__0l__double__0c__double__0c__struct__Node__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_double__0l__double__0c__double__0c__struct__Node__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_double__0l__double__0c__double__0c__struct__Node__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(f64, f64, *mut crate::__pgrx_c_bindings::Node) -> f64,
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn() -> u32>>() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn() -> u32>>() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_enum__LogicalRepWorkerType__0l__void__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_enum__LogicalRepWorkerType__0l__void__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_enum__LogicalRepWorkerType__0l__void__0r {
    type Physical =
        PhysicalFunction_C_unwind_0<::core::option::Option<unsafe extern "C-unwind" fn() -> u32>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::ItemPointerData,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
                u32,
                u32,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                bool,
                *mut crate::__pgrx_c_bindings::TM_FailureData,
                *mut u32,
                *mut u32,
            ) -> u32,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::ItemPointerData,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
                u32,
                u32,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                bool,
                *mut crate::__pgrx_c_bindings::TM_FailureData,
                *mut u32,
                *mut u32,
            ) -> u32,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_enum__TM_Result__0l__struct__RelationData__0p__0c__struct__ItemPointerData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__unsigned__int__0c__struct__SnapshotData__0p__0c__struct__SnapshotData__0p__0c__0x5f426f6f6c__0c__struct__TM_FailureData__0p__0c__enum__LockTupleMode__0p__0c__enum__TU_UpdateIndexes__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_enum__TM_Result__0l__struct__RelationData__0p__0c__struct__ItemPointerData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__unsigned__int__0c__struct__SnapshotData__0p__0c__struct__SnapshotData__0p__0c__0x5f426f6f6c__0c__struct__TM_FailureData__0p__0c__enum__LockTupleMode__0p__0c__enum__TU_UpdateIndexes__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_enum__TM_Result__0l__struct__RelationData__0p__0c__struct__ItemPointerData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__unsigned__int__0c__struct__SnapshotData__0p__0c__struct__SnapshotData__0p__0c__0x5f426f6f6c__0c__struct__TM_FailureData__0p__0c__enum__LockTupleMode__0p__0c__enum__TU_UpdateIndexes__0p__0r {
 type Physical = PhysicalFunction_C_unwind_11<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::ItemPointerData, *mut crate::__pgrx_c_bindings::TupleTableSlot, u32, u32, *mut crate::__pgrx_c_bindings::SnapshotData, *mut crate::__pgrx_c_bindings::SnapshotData, bool, *mut crate::__pgrx_c_bindings::TM_FailureData, *mut u32, *mut u32) -> u32>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::ItemPointerData,
                u32,
                u32,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                bool,
                *mut crate::__pgrx_c_bindings::TM_FailureData,
            ) -> u32,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::ItemPointerData,
                u32,
                u32,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                bool,
                *mut crate::__pgrx_c_bindings::TM_FailureData,
            ) -> u32,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_enum__TM_Result__0l__struct__RelationData__0p__0c__struct__ItemPointerData__0p__0c__unsigned__int__0c__unsigned__int__0c__struct__SnapshotData__0p__0c__struct__SnapshotData__0p__0c__0x5f426f6f6c__0c__struct__TM_FailureData__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_enum__TM_Result__0l__struct__RelationData__0p__0c__struct__ItemPointerData__0p__0c__unsigned__int__0c__unsigned__int__0c__struct__SnapshotData__0p__0c__struct__SnapshotData__0p__0c__0x5f426f6f6c__0c__struct__TM_FailureData__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_enum__TM_Result__0l__struct__RelationData__0p__0c__struct__ItemPointerData__0p__0c__unsigned__int__0c__unsigned__int__0c__struct__SnapshotData__0p__0c__struct__SnapshotData__0p__0c__0x5f426f6f6c__0c__struct__TM_FailureData__0p__0r {
 type Physical = PhysicalFunction_C_unwind_8<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::ItemPointerData, u32, u32, *mut crate::__pgrx_c_bindings::SnapshotData, *mut crate::__pgrx_c_bindings::SnapshotData, bool, *mut crate::__pgrx_c_bindings::TM_FailureData) -> u32>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_void) -> u32>,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_void) -> u32>,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_enum__vartag_external__0l__const__void__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_enum__vartag_external__0l__const__void__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_enum__vartag_external__0l__const__void__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_void) -> u32>,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *mut crate::__pgrx_c_bindings::pg_locale_struct,
            ) -> i32,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *mut crate::__pgrx_c_bindings::pg_locale_struct,
            ) -> i32,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__const__char__0p__0c__const__char__0p__0c__struct__pg_locale_struct__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_int__0l__const__char__0p__0c__const__char__0p__0c__struct__pg_locale_struct__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_int__0l__const__char__0p__0c__const__char__0p__0c__struct__pg_locale_struct__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_char, *const ::core::ffi::c_char, *mut crate::__pgrx_c_bindings::pg_locale_struct) -> i32>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_char,
                usize,
                *const ::core::ffi::c_char,
                usize,
                *mut crate::__pgrx_c_bindings::pg_locale_struct,
            ) -> i32,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_char,
                usize,
                *const ::core::ffi::c_char,
                usize,
                *mut crate::__pgrx_c_bindings::pg_locale_struct,
            ) -> i32,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__const__char__0p__0c__unsigned__long__0c__const__char__0p__0c__unsigned__long__0c__struct__pg_locale_struct__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_int__0l__const__char__0p__0c__unsigned__long__0c__const__char__0p__0c__unsigned__long__0c__struct__pg_locale_struct__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_int__0l__const__char__0p__0c__unsigned__long__0c__const__char__0p__0c__unsigned__long__0c__struct__pg_locale_struct__0p__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_char, usize, *const ::core::ffi::c_char, usize, *mut crate::__pgrx_c_bindings::pg_locale_struct) -> i32>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_void) -> i32>,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_void) -> i32>,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__const__void__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_int__0l__const__void__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_int__0l__const__void__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_void) -> i32>,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::ItemPointerData,
                *const crate::__pgrx_c_bindings::ItemPointerData,
            ) -> i32,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::ItemPointerData,
                *const crate::__pgrx_c_bindings::ItemPointerData,
            ) -> i32,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__const__struct__ItemPointerData__0p__0c__const__struct__ItemPointerData__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_int__0l__const__struct__ItemPointerData__0p__0c__const__struct__ItemPointerData__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_int__0l__const__struct__ItemPointerData__0p__0c__const__struct__ItemPointerData__0p__0r {
 type Physical = PhysicalFunction_C_unwind_2<::core::option::Option<unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::ItemPointerData, *const crate::__pgrx_c_bindings::ItemPointerData) -> i32>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::Fitness>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::Fitness>,
            ) -> i32,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::Fitness>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::Fitness>,
            ) -> i32,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__struct__Fitness__0c__struct__Fitness__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_int__0l__struct__Fitness__0c__struct__Fitness__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_int__0l__struct__Fitness__0c__struct__Fitness__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::Fitness>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::Fitness>,
            ) -> i32,
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::GinState,
                u16,
                crate::__pgrx_c_bindings::Datum,
                i8,
                crate::__pgrx_c_bindings::Datum,
                i8,
            ) -> i32,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::GinState,
                u16,
                crate::__pgrx_c_bindings::Datum,
                i8,
                crate::__pgrx_c_bindings::Datum,
                i8,
            ) -> i32,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__struct__GinState__0p__0c__unsigned__short__0c__unsigned__long__0c__signed__char__0c__unsigned__long__0c__signed__char__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_int__0l__struct__GinState__0p__0c__unsigned__short__0c__unsigned__long__0c__signed__char__0c__unsigned__long__0c__signed__char__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_int__0l__struct__GinState__0p__0c__unsigned__short__0c__unsigned__long__0c__signed__char__0c__unsigned__long__0c__signed__char__0r {
 type Physical = PhysicalFunction_C_unwind_6<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::GinState, u16, crate::__pgrx_c_bindings::Datum, i8, crate::__pgrx_c_bindings::Datum, i8) -> i32>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::GinState,
                u16,
                crate::__pgrx_c_bindings::Datum,
                i8,
                u16,
                crate::__pgrx_c_bindings::Datum,
                i8,
            ) -> i32,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::GinState,
                u16,
                crate::__pgrx_c_bindings::Datum,
                i8,
                u16,
                crate::__pgrx_c_bindings::Datum,
                i8,
            ) -> i32,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__struct__GinState__0p__0c__unsigned__short__0c__unsigned__long__0c__signed__char__0c__unsigned__short__0c__unsigned__long__0c__signed__char__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_int__0l__struct__GinState__0p__0c__unsigned__short__0c__unsigned__long__0c__signed__char__0c__unsigned__short__0c__unsigned__long__0c__signed__char__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_int__0l__struct__GinState__0p__0c__unsigned__short__0c__unsigned__long__0c__signed__char__0c__unsigned__short__0c__unsigned__long__0c__signed__char__0r {
 type Physical = PhysicalFunction_C_unwind_7<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::GinState, u16, crate::__pgrx_c_bindings::Datum, i8, u16, crate::__pgrx_c_bindings::Datum, i8) -> i32>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::ItemPointerData) -> i64,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::ItemPointerData) -> i64,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_long__0l__const__struct__ItemPointerData__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_long__0l__const__struct__ItemPointerData__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_long__0l__const__struct__ItemPointerData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::ItemPointerData) -> i64,
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::MultiXactId) -> i64,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::MultiXactId) -> i64,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_long__0l__unsigned__int__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_long__0l__unsigned__int__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_long__0l__unsigned__int__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::MultiXactId) -> i64,
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn() -> i64>>() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn() -> i64>>() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_long__0l__void__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_long__0l__void__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_long__0l__void__0r {
    type Physical =
        PhysicalFunction_C_unwind_0<::core::option::Option<unsafe extern "C-unwind" fn() -> i64>>;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__HeapTupleData__0p__0l__enum__SysCacheIdentifier__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_struct__HeapTupleData__0p__0l__enum__SysCacheIdentifier__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_struct__HeapTupleData__0p__0l__enum__SysCacheIdentifier__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(i32, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum) -> *mut crate::__pgrx_c_bindings::HeapTupleData>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                u32,
            )
                -> *mut crate::__pgrx_c_bindings::IndexFetchTableData,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                u32,
            )
                -> *mut crate::__pgrx_c_bindings::IndexFetchTableData,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__IndexFetchTableData__0p__0l__struct__RelationData__0p__0c__unsigned__int__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_struct__IndexFetchTableData__0p__0l__struct__RelationData__0p__0c__unsigned__int__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_struct__IndexFetchTableData__0p__0l__struct__RelationData__0p__0c__unsigned__int__0r {
 type Physical = PhysicalFunction_C_unwind_2<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, u32) -> *mut crate::__pgrx_c_bindings::IndexFetchTableData>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::SlruDesc,
                i64,
            ) -> *mut crate::__pgrx_c_bindings::LWLock,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::SlruDesc,
                i64,
            ) -> *mut crate::__pgrx_c_bindings::LWLock,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__LWLock__0p__0l__struct__SlruDesc__0p__0c__long__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_struct__LWLock__0p__0l__struct__SlruDesc__0p__0c__long__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_struct__LWLock__0p__0l__struct__SlruDesc__0p__0c__long__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::SlruDesc,
                i64,
            ) -> *mut crate::__pgrx_c_bindings::LWLock,
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
            ) -> *mut crate::__pgrx_c_bindings::List,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
            ) -> *mut crate::__pgrx_c_bindings::List,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__List__0p__0l__enum__NodeTag__0c__union__ListCell__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_struct__List__0p__0l__enum__NodeTag__0c__union__ListCell__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_struct__List__0p__0l__enum__NodeTag__0c__union__ListCell__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
            ) -> *mut crate::__pgrx_c_bindings::List,
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
            ) -> *mut crate::__pgrx_c_bindings::List,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
            ) -> *mut crate::__pgrx_c_bindings::List,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__List__0p__0l__enum__NodeTag__0c__union__ListCell__0c__union__ListCell__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed
    for Signature_struct__List__0p__0l__enum__NodeTag__0c__union__ListCell__0c__union__ListCell__0r
{
}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_struct__List__0p__0l__enum__NodeTag__0c__union__ListCell__0c__union__ListCell__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
            ) -> *mut crate::__pgrx_c_bindings::List,
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
            ) -> *mut crate::__pgrx_c_bindings::List,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
            ) -> *mut crate::__pgrx_c_bindings::List,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__List__0p__0l__enum__NodeTag__0c__union__ListCell__0c__union__ListCell__0c__union__ListCell__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_struct__List__0p__0l__enum__NodeTag__0c__union__ListCell__0c__union__ListCell__0c__union__ListCell__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_struct__List__0p__0l__enum__NodeTag__0c__union__ListCell__0c__union__ListCell__0c__union__ListCell__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(u32, ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>, ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>, ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>) -> *mut crate::__pgrx_c_bindings::List>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
            ) -> *mut crate::__pgrx_c_bindings::List,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
            ) -> *mut crate::__pgrx_c_bindings::List,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__List__0p__0l__enum__NodeTag__0c__union__ListCell__0c__union__ListCell__0c__union__ListCell__0c__union__ListCell__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_struct__List__0p__0l__enum__NodeTag__0c__union__ListCell__0c__union__ListCell__0c__union__ListCell__0c__union__ListCell__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_struct__List__0p__0l__enum__NodeTag__0c__union__ListCell__0c__union__ListCell__0c__union__ListCell__0c__union__ListCell__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(u32, ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>, ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>, ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>, ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>) -> *mut crate::__pgrx_c_bindings::List>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
            ) -> *mut crate::__pgrx_c_bindings::List,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
            ) -> *mut crate::__pgrx_c_bindings::List,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__List__0p__0l__enum__NodeTag__0c__union__ListCell__0c__union__ListCell__0c__union__ListCell__0c__union__ListCell__0c__union__ListCell__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_struct__List__0p__0l__enum__NodeTag__0c__union__ListCell__0c__union__ListCell__0c__union__ListCell__0c__union__ListCell__0c__union__ListCell__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_struct__List__0p__0l__enum__NodeTag__0c__union__ListCell__0c__union__ListCell__0c__union__ListCell__0c__union__ListCell__0c__union__ListCell__0r {
 type Physical = PhysicalFunction_C_unwind_6<::core::option::Option<unsafe extern "C-unwind" fn(u32, ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>, ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>, ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>, ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>, ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>) -> *mut crate::__pgrx_c_bindings::List>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::Var,
                *mut crate::__pgrx_c_bindings::replace_rte_variables_context,
            ) -> *mut crate::__pgrx_c_bindings::Node,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::Var,
                *mut crate::__pgrx_c_bindings::replace_rte_variables_context,
            ) -> *mut crate::__pgrx_c_bindings::Node,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__Node__0p__0l__const__struct__Var__0p__0c__struct__replace_rte_variables_context__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_struct__Node__0p__0l__const__struct__Var__0p__0c__struct__replace_rte_variables_context__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_struct__Node__0p__0l__const__struct__Var__0p__0c__struct__replace_rte_variables_context__0p__0r {
 type Physical = PhysicalFunction_C_unwind_2<::core::option::Option<unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::Var, *mut crate::__pgrx_c_bindings::replace_rte_variables_context) -> *mut crate::__pgrx_c_bindings::Node>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::Query,
                *const ::core::ffi::c_char,
                i32,
                *mut crate::__pgrx_c_bindings::ParamListInfoData,
                *mut crate::__pgrx_c_bindings::ExplainState,
            ) -> *mut crate::__pgrx_c_bindings::PlannedStmt,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::Query,
                *const ::core::ffi::c_char,
                i32,
                *mut crate::__pgrx_c_bindings::ParamListInfoData,
                *mut crate::__pgrx_c_bindings::ExplainState,
            ) -> *mut crate::__pgrx_c_bindings::PlannedStmt,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__PlannedStmt__0p__0l__struct__Query__0p__0c__const__char__0p__0c__int__0c__struct__ParamListInfoData__0p__0c__struct__ExplainState__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_struct__PlannedStmt__0p__0l__struct__Query__0p__0c__const__char__0p__0c__int__0c__struct__ParamListInfoData__0p__0c__struct__ExplainState__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_struct__PlannedStmt__0p__0l__struct__Query__0p__0c__const__char__0p__0c__int__0c__struct__ParamListInfoData__0p__0c__struct__ExplainState__0p__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::Query, *const ::core::ffi::c_char, i32, *mut crate::__pgrx_c_bindings::ParamListInfoData, *mut crate::__pgrx_c_bindings::ExplainState) -> *mut crate::__pgrx_c_bindings::PlannedStmt>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                i32,
                *mut crate::__pgrx_c_bindings::ScanKeyData,
                bool,
                bool,
                bool,
                u32,
            )
                -> *mut crate::__pgrx_c_bindings::TableScanDescData,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                i32,
                *mut crate::__pgrx_c_bindings::ScanKeyData,
                bool,
                bool,
                bool,
                u32,
            )
                -> *mut crate::__pgrx_c_bindings::TableScanDescData,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0c__struct__SnapshotData__0p__0c__int__0c__struct__ScanKeyData__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__unsigned__int__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0c__struct__SnapshotData__0p__0c__int__0c__struct__ScanKeyData__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__unsigned__int__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0c__struct__SnapshotData__0p__0c__int__0c__struct__ScanKeyData__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__unsigned__int__0r {
 type Physical = PhysicalFunction_C_unwind_8<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::SnapshotData, i32, *mut crate::__pgrx_c_bindings::ScanKeyData, bool, bool, bool, u32) -> *mut crate::__pgrx_c_bindings::TableScanDescData>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                i32,
                *mut crate::__pgrx_c_bindings::ScanKeyData,
                u32,
            )
                -> *mut crate::__pgrx_c_bindings::TableScanDescData,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                i32,
                *mut crate::__pgrx_c_bindings::ScanKeyData,
                u32,
            )
                -> *mut crate::__pgrx_c_bindings::TableScanDescData,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0c__struct__SnapshotData__0p__0c__int__0c__struct__ScanKeyData__0p__0c__unsigned__int__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0c__struct__SnapshotData__0p__0c__int__0c__struct__ScanKeyData__0p__0c__unsigned__int__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0c__struct__SnapshotData__0p__0c__int__0c__struct__ScanKeyData__0p__0c__unsigned__int__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::SnapshotData, i32, *mut crate::__pgrx_c_bindings::ScanKeyData, u32) -> *mut crate::__pgrx_c_bindings::TableScanDescData>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                *mut crate::__pgrx_c_bindings::ItemPointerData,
                *mut crate::__pgrx_c_bindings::ItemPointerData,
                u32,
            )
                -> *mut crate::__pgrx_c_bindings::TableScanDescData,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                *mut crate::__pgrx_c_bindings::ItemPointerData,
                *mut crate::__pgrx_c_bindings::ItemPointerData,
                u32,
            )
                -> *mut crate::__pgrx_c_bindings::TableScanDescData,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0c__struct__SnapshotData__0p__0c__struct__ItemPointerData__0p__0c__struct__ItemPointerData__0p__0c__unsigned__int__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0c__struct__SnapshotData__0p__0c__struct__ItemPointerData__0p__0c__struct__ItemPointerData__0p__0c__unsigned__int__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0c__struct__SnapshotData__0p__0c__struct__ItemPointerData__0p__0c__struct__ItemPointerData__0p__0c__unsigned__int__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::SnapshotData, *mut crate::__pgrx_c_bindings::ItemPointerData, *mut crate::__pgrx_c_bindings::ItemPointerData, u32) -> *mut crate::__pgrx_c_bindings::TableScanDescData>>;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__catclist__0p__0l__enum__SysCacheIdentifier__0c__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_struct__catclist__0p__0l__enum__SysCacheIdentifier__0c__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_struct__catclist__0p__0l__enum__SysCacheIdentifier__0c__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(i32, i32, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum) -> *mut crate::__pgrx_c_bindings::catclist>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                i32,
            ) -> ::core::mem::MaybeUninit<
                crate::__pgrx_c_bindings::ListCell,
            >,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                i32,
            ) -> ::core::mem::MaybeUninit<
                crate::__pgrx_c_bindings::ListCell,
            >,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_union__ListCell__0l__int__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_union__ListCell__0l__int__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_union__ListCell__0l__int__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                i32,
            ) -> ::core::mem::MaybeUninit<
                crate::__pgrx_c_bindings::ListCell,
            >,
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
            ) -> ::core::mem::MaybeUninit<
                crate::__pgrx_c_bindings::ListCell,
            >,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
            ) -> ::core::mem::MaybeUninit<
                crate::__pgrx_c_bindings::ListCell,
            >,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_union__ListCell__0l__void__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_union__ListCell__0l__void__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_union__ListCell__0l__void__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
            ) -> ::core::mem::MaybeUninit<
                crate::__pgrx_c_bindings::ListCell,
            >,
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn(i8) -> u8>>() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn(i8) -> u8>>() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__char__0l__char__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_unsigned__char__0l__char__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_unsigned__char__0l__char__0r {
    type Physical =
        PhysicalFunction_C_unwind_1<::core::option::Option<unsafe extern "C-unwind" fn(i8) -> u8>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn(u8) -> u8>>() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn(u8) -> u8>>() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__char__0l__unsigned__char__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_unsigned__char__0l__unsigned__char__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_unsigned__char__0l__unsigned__char__0r {
    type Physical =
        PhysicalFunction_C_unwind_1<::core::option::Option<unsafe extern "C-unwind" fn(u8) -> u8>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(u32, *mut u8) -> *mut u8>,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(u32, *mut u8) -> *mut u8>,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__char__0p__0l__unsigned__int__0c__unsigned__char__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed
    for Signature_unsigned__char__0p__0l__unsigned__int__0c__unsigned__char__0p__0r
{
}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__char__0p__0l__unsigned__int__0c__unsigned__char__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<unsafe extern "C-unwind" fn(u32, *mut u8) -> *mut u8>,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                *mut crate::__pgrx_c_bindings::pg_locale_struct,
            ) -> u32,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                *mut crate::__pgrx_c_bindings::pg_locale_struct,
            ) -> u32,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__int__0l__unsigned__int__0c__struct__pg_locale_struct__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed
    for Signature_unsigned__int__0l__unsigned__int__0c__struct__pg_locale_struct__0p__0r
{
}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__int__0l__unsigned__int__0c__struct__pg_locale_struct__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                *mut crate::__pgrx_c_bindings::pg_locale_struct,
            ) -> u32,
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_char,
            ) -> crate::__pgrx_c_bindings::TransactionId,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_char,
            ) -> crate::__pgrx_c_bindings::TransactionId,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__int__0l__const__char__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_unsigned__int__0l__const__char__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_unsigned__int__0l__const__char__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_char,
            ) -> crate::__pgrx_c_bindings::TransactionId,
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn(*const u8) -> u32>>()
        == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn(*const u8) -> u32>>(
    ) == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__int__0l__const__unsigned__char__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_unsigned__int__0l__const__unsigned__char__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__int__0l__const__unsigned__char__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const u8) -> u32>,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_void) -> u32>,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_void) -> u32>,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__int__0l__const__void__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_unsigned__int__0l__const__void__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_unsigned__int__0l__const__void__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_void) -> u32>,
    >;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__int__0l__enum__SysCacheIdentifier__0c__short__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_unsigned__int__0l__enum__SysCacheIdentifier__0c__short__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_unsigned__int__0l__enum__SysCacheIdentifier__0c__short__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_6<::core::option::Option<unsafe extern "C-unwind" fn(i32, i16, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum) -> crate::__pgrx_c_bindings::Oid>>;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__int__0l__enum__SysCacheIdentifier__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_unsigned__int__0l__enum__SysCacheIdentifier__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_unsigned__int__0l__enum__SysCacheIdentifier__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(i32, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum) -> u32>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::varatt_external>,
            ) -> u32,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::varatt_external>,
            ) -> u32,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__int__0l__struct__varatt_external__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_unsigned__int__0l__struct__varatt_external__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__int__0l__struct__varatt_external__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::varatt_external>,
            ) -> u32,
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn(u16, u16) -> u32>>()
        == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn(u16, u16) -> u32>>()
        == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__int__0l__unsigned__short__0c__unsigned__short__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_unsigned__int__0l__unsigned__short__0c__unsigned__short__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__int__0l__unsigned__short__0c__unsigned__short__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<unsafe extern "C-unwind" fn(u16, u16) -> u32>,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_char,
                usize,
                *const ::core::ffi::c_char,
                *mut crate::__pgrx_c_bindings::pg_locale_struct,
            ) -> usize,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_char,
                usize,
                *const ::core::ffi::c_char,
                *mut crate::__pgrx_c_bindings::pg_locale_struct,
            ) -> usize,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__char__0p__0c__unsigned__long__0c__const__char__0p__0c__struct__pg_locale_struct__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_unsigned__long__0l__char__0p__0c__unsigned__long__0c__const__char__0p__0c__struct__pg_locale_struct__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__char__0p__0c__unsigned__long__0c__const__char__0p__0c__struct__pg_locale_struct__0p__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_char, usize, *const ::core::ffi::c_char, *mut crate::__pgrx_c_bindings::pg_locale_struct) -> usize>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_char,
                usize,
                *const ::core::ffi::c_char,
                usize,
                *mut crate::__pgrx_c_bindings::pg_locale_struct,
            ) -> usize,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_char,
                usize,
                *const ::core::ffi::c_char,
                usize,
                *mut crate::__pgrx_c_bindings::pg_locale_struct,
            ) -> usize,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__char__0p__0c__unsigned__long__0c__const__char__0p__0c__unsigned__long__0c__struct__pg_locale_struct__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_unsigned__long__0l__char__0p__0c__unsigned__long__0c__const__char__0p__0c__unsigned__long__0c__struct__pg_locale_struct__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__char__0p__0c__unsigned__long__0c__const__char__0p__0c__unsigned__long__0c__struct__pg_locale_struct__0p__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_char, usize, *const ::core::ffi::c_char, usize, *mut crate::__pgrx_c_bindings::pg_locale_struct) -> usize>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_char,
                *mut u32,
                bool,
                i32,
                u8,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_char,
                *mut u32,
                bool,
                i32,
                u8,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__const__char__0p__0c__unsigned__int__0p__0c__0x5f426f6f6c__0c__int__0c__unsigned__char__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_unsigned__long__0l__const__char__0p__0c__unsigned__int__0p__0c__0x5f426f6f6c__0c__int__0c__unsigned__char__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__const__char__0p__0c__unsigned__int__0p__0c__0x5f426f6f6c__0c__int__0c__unsigned__char__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_char, *mut u32, bool, i32, u8) -> crate::__pgrx_c_bindings::Datum>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_void) -> usize>,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_void) -> usize>,
    >() == 8
);
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__const__void__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_void) -> usize>,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::PageXLogRecPtr) -> u64,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::PageXLogRecPtr) -> u64,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__const__volatile__PageXLogRecPtr__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_unsigned__long__0l__const__volatile__PageXLogRecPtr__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__long__0l__const__volatile__PageXLogRecPtr__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::PageXLogRecPtr) -> u64,
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn(u32) -> usize>>()
        == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn(u32) -> usize>>()
        == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__enum__vartag_external__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_unsigned__long__0l__enum__vartag_external__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__long__0l__enum__vartag_external__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(u32) -> usize>,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::BufferDesc,
                u64,
                u64,
                u64,
                i32,
            ) -> u64,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::BufferDesc,
                u64,
                u64,
                u64,
                i32,
            ) -> u64,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__struct__BufferDesc__0p__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__int__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_unsigned__long__0l__struct__BufferDesc__0p__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__int__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__struct__BufferDesc__0p__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__int__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::BufferDesc, u64, u64, u64, i32) -> u64>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::varatt_external>,
            ) -> usize,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::varatt_external>,
            ) -> usize,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__struct__varatt_external__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_unsigned__long__0l__struct__varatt_external__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__long__0l__struct__varatt_external__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::varatt_external>,
            ) -> usize,
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_char,
                *const ::core::ffi::c_void,
                usize,
                u16,
                i32,
            ) -> u16,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_char,
                *const ::core::ffi::c_void,
                usize,
                u16,
                i32,
            ) -> u16,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__short__0l__char__0p__0c__const__void__0p__0c__unsigned__long__0c__unsigned__short__0c__int__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_unsigned__short__0l__char__0p__0c__const__void__0p__0c__unsigned__long__0c__unsigned__short__0c__int__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_unsigned__short__0l__char__0p__0c__const__void__0p__0c__unsigned__long__0c__unsigned__short__0c__int__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_char, *const ::core::ffi::c_void, usize, u16, i32) -> u16>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(i32, *const crate::__pgrx_c_bindings::pg_signal_info) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(i32, *const crate::__pgrx_c_bindings::pg_signal_info) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__int__0c__const__struct__pg_signal_info__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0l__int__0c__const__struct__pg_signal_info__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__int__0c__const__struct__pg_signal_info__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(i32, *const crate::__pgrx_c_bindings::pg_signal_info) -> (),
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::FullTransactionId,
                u16,
                *mut ::core::ffi::c_void,
                u32,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::FullTransactionId,
                u16,
                *mut ::core::ffi::c_void,
                u32,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__FullTransactionId__0c__unsigned__short__0c__void__0p__0c__unsigned__int__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0l__struct__FullTransactionId__0c__unsigned__short__0c__void__0p__0c__unsigned__int__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__FullTransactionId__0c__unsigned__short__0c__void__0p__0c__unsigned__int__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::FullTransactionId, u16, *mut ::core::ffi::c_void, u32) -> ()>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ParseState,
                *mut crate::__pgrx_c_bindings::Query,
                *const crate::__pgrx_c_bindings::JumbleState,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ParseState,
                *mut crate::__pgrx_c_bindings::Query,
                *const crate::__pgrx_c_bindings::JumbleState,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__ParseState__0p__0c__struct__Query__0p__0c__const__struct__JumbleState__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0l__struct__ParseState__0p__0c__struct__Query__0p__0c__const__struct__JumbleState__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__ParseState__0p__0c__struct__Query__0p__0c__const__struct__JumbleState__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ParseState, *mut crate::__pgrx_c_bindings::Query, *const crate::__pgrx_c_bindings::JumbleState) -> ()>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerGlobal,
                *mut crate::__pgrx_c_bindings::Query,
                *const ::core::ffi::c_char,
                i32,
                *mut f64,
                *mut crate::__pgrx_c_bindings::ExplainState,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerGlobal,
                *mut crate::__pgrx_c_bindings::Query,
                *const ::core::ffi::c_char,
                i32,
                *mut f64,
                *mut crate::__pgrx_c_bindings::ExplainState,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__PlannerGlobal__0p__0c__struct__Query__0p__0c__const__char__0p__0c__int__0c__double__0p__0c__struct__ExplainState__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0l__struct__PlannerGlobal__0p__0c__struct__Query__0p__0c__const__char__0p__0c__int__0c__double__0p__0c__struct__ExplainState__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__PlannerGlobal__0p__0c__struct__Query__0p__0c__const__char__0p__0c__int__0c__double__0p__0c__struct__ExplainState__0p__0r {
 type Physical = PhysicalFunction_C_unwind_6<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PlannerGlobal, *mut crate::__pgrx_c_bindings::Query, *const ::core::ffi::c_char, i32, *mut f64, *mut crate::__pgrx_c_bindings::ExplainState) -> ()>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerGlobal,
                *mut crate::__pgrx_c_bindings::Query,
                *const ::core::ffi::c_char,
                *mut crate::__pgrx_c_bindings::PlannedStmt,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerGlobal,
                *mut crate::__pgrx_c_bindings::Query,
                *const ::core::ffi::c_char,
                *mut crate::__pgrx_c_bindings::PlannedStmt,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__PlannerGlobal__0p__0c__struct__Query__0p__0c__const__char__0p__0c__struct__PlannedStmt__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0l__struct__PlannerGlobal__0p__0c__struct__Query__0p__0c__const__char__0p__0c__struct__PlannedStmt__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__PlannerGlobal__0p__0c__struct__Query__0p__0c__const__char__0p__0c__struct__PlannedStmt__0p__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PlannerGlobal, *mut crate::__pgrx_c_bindings::Query, *const ::core::ffi::c_char, *mut crate::__pgrx_c_bindings::PlannedStmt) -> ()>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerInfo,
                *mut crate::__pgrx_c_bindings::RelOptInfo,
                *mut crate::__pgrx_c_bindings::RangeTblEntry,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerInfo,
                *mut crate::__pgrx_c_bindings::RelOptInfo,
                *mut crate::__pgrx_c_bindings::RangeTblEntry,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__RangeTblEntry__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__RangeTblEntry__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__RangeTblEntry__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PlannerInfo, *mut crate::__pgrx_c_bindings::RelOptInfo, *mut crate::__pgrx_c_bindings::RangeTblEntry) -> ()>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerInfo,
                *mut crate::__pgrx_c_bindings::RelOptInfo,
                *mut crate::__pgrx_c_bindings::RelOptInfo,
                *mut crate::__pgrx_c_bindings::RelOptInfo,
                *mut crate::__pgrx_c_bindings::SpecialJoinInfo,
                *mut crate::__pgrx_c_bindings::List,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerInfo,
                *mut crate::__pgrx_c_bindings::RelOptInfo,
                *mut crate::__pgrx_c_bindings::RelOptInfo,
                *mut crate::__pgrx_c_bindings::RelOptInfo,
                *mut crate::__pgrx_c_bindings::SpecialJoinInfo,
                *mut crate::__pgrx_c_bindings::List,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__SpecialJoinInfo__0p__0c__struct__List__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__SpecialJoinInfo__0p__0c__struct__List__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__SpecialJoinInfo__0p__0c__struct__List__0p__0r {
 type Physical = PhysicalFunction_C_unwind_6<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PlannerInfo, *mut crate::__pgrx_c_bindings::RelOptInfo, *mut crate::__pgrx_c_bindings::RelOptInfo, *mut crate::__pgrx_c_bindings::RelOptInfo, *mut crate::__pgrx_c_bindings::SpecialJoinInfo, *mut crate::__pgrx_c_bindings::List) -> ()>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *const crate::__pgrx_c_bindings::VacuumParams,
                *mut crate::__pgrx_c_bindings::BufferAccessStrategyData,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *const crate::__pgrx_c_bindings::VacuumParams,
                *mut crate::__pgrx_c_bindings::BufferAccessStrategyData,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__RelationData__0p__0c__const__struct__VacuumParams__0p__0c__struct__BufferAccessStrategyData__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0l__struct__RelationData__0p__0c__const__struct__VacuumParams__0p__0c__struct__BufferAccessStrategyData__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__RelationData__0p__0c__const__struct__VacuumParams__0p__0c__struct__BufferAccessStrategyData__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *const crate::__pgrx_c_bindings::VacuumParams, *mut crate::__pgrx_c_bindings::BufferAccessStrategyData) -> ()>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::RelationData,
                bool,
                crate::__pgrx_c_bindings::TransactionId,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                *mut crate::__pgrx_c_bindings::TransactionId,
                *mut crate::__pgrx_c_bindings::MultiXactId,
                *mut f64,
                *mut f64,
                *mut f64,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::RelationData,
                bool,
                crate::__pgrx_c_bindings::TransactionId,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                *mut crate::__pgrx_c_bindings::TransactionId,
                *mut crate::__pgrx_c_bindings::MultiXactId,
                *mut f64,
                *mut f64,
                *mut f64,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__RelationData__0p__0c__struct__RelationData__0p__0c__struct__RelationData__0p__0c__0x5f426f6f6c__0c__unsigned__int__0c__struct__SnapshotData__0p__0c__unsigned__int__0p__0c__unsigned__int__0p__0c__double__0p__0c__double__0p__0c__double__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0l__struct__RelationData__0p__0c__struct__RelationData__0p__0c__struct__RelationData__0p__0c__0x5f426f6f6c__0c__unsigned__int__0c__struct__SnapshotData__0p__0c__unsigned__int__0p__0c__unsigned__int__0p__0c__double__0p__0c__double__0p__0c__double__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__RelationData__0p__0c__struct__RelationData__0p__0c__struct__RelationData__0p__0c__0x5f426f6f6c__0c__unsigned__int__0c__struct__SnapshotData__0p__0c__unsigned__int__0p__0c__unsigned__int__0p__0c__double__0p__0c__double__0p__0c__double__0p__0r {
 type Physical = PhysicalFunction_C_unwind_11<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::RelationData, bool, crate::__pgrx_c_bindings::TransactionId, *mut crate::__pgrx_c_bindings::SnapshotData, *mut crate::__pgrx_c_bindings::TransactionId, *mut crate::__pgrx_c_bindings::MultiXactId, *mut f64, *mut f64, *mut f64) -> ()>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut *mut crate::__pgrx_c_bindings::TupleTableSlot,
                i32,
                u32,
                u32,
                *mut crate::__pgrx_c_bindings::BulkInsertStateData,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut *mut crate::__pgrx_c_bindings::TupleTableSlot,
                i32,
                u32,
                u32,
                *mut crate::__pgrx_c_bindings::BulkInsertStateData,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__RelationData__0p__0c__struct__TupleTableSlot__0p__0p__0c__int__0c__unsigned__int__0c__unsigned__int__0c__struct__BulkInsertStateData__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0l__struct__RelationData__0p__0c__struct__TupleTableSlot__0p__0p__0c__int__0c__unsigned__int__0c__unsigned__int__0c__struct__BulkInsertStateData__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__RelationData__0p__0c__struct__TupleTableSlot__0p__0p__0c__int__0c__unsigned__int__0c__unsigned__int__0c__struct__BulkInsertStateData__0p__0r {
 type Physical = PhysicalFunction_C_unwind_6<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *mut *mut crate::__pgrx_c_bindings::TupleTableSlot, i32, u32, u32, *mut crate::__pgrx_c_bindings::BulkInsertStateData) -> ()>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
                u32,
                u32,
                *mut crate::__pgrx_c_bindings::BulkInsertStateData,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
                u32,
                u32,
                *mut crate::__pgrx_c_bindings::BulkInsertStateData,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__RelationData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__unsigned__int__0c__struct__BulkInsertStateData__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0l__struct__RelationData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__unsigned__int__0c__struct__BulkInsertStateData__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__RelationData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__unsigned__int__0c__struct__BulkInsertStateData__0p__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::TupleTableSlot, u32, u32, *mut crate::__pgrx_c_bindings::BulkInsertStateData) -> ()>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
                u32,
                u32,
                *mut crate::__pgrx_c_bindings::BulkInsertStateData,
                u32,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
                u32,
                u32,
                *mut crate::__pgrx_c_bindings::BulkInsertStateData,
                u32,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__RelationData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__unsigned__int__0c__struct__BulkInsertStateData__0p__0c__unsigned__int__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0l__struct__RelationData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__unsigned__int__0c__struct__BulkInsertStateData__0p__0c__unsigned__int__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__RelationData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__unsigned__int__0c__struct__BulkInsertStateData__0p__0c__unsigned__int__0r {
 type Physical = PhysicalFunction_C_unwind_6<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::TupleTableSlot, u32, u32, *mut crate::__pgrx_c_bindings::BulkInsertStateData, u32) -> ()>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, u32) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, u32) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__RelationData__0p__0c__unsigned__int__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0l__struct__RelationData__0p__0c__unsigned__int__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__RelationData__0p__0c__unsigned__int__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, u32) -> (),
        >,
    >;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__unsigned__long__0c__enum__SysCacheIdentifier__0c__unsigned__int__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed
    for Signature_void__0l__unsigned__long__0c__enum__SysCacheIdentifier__0c__unsigned__int__0r
{
}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__unsigned__long__0c__enum__SysCacheIdentifier__0c__unsigned__int__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Datum, i32, u32) -> (),
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
                *mut crate::__pgrx_c_bindings::TSQueryParserStateData,
                *mut ::core::ffi::c_char,
                i32,
                i16,
                bool,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
                *mut crate::__pgrx_c_bindings::TSQueryParserStateData,
                *mut ::core::ffi::c_char,
                i32,
                i16,
                bool,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__void__0p__0c__struct__TSQueryParserStateData__0p__0c__char__0p__0c__int__0c__short__0c__0x5f426f6f6c__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0l__void__0p__0c__struct__TSQueryParserStateData__0p__0c__char__0p__0c__int__0c__short__0c__0x5f426f6f6c__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_void__0l__void__0p__0c__struct__TSQueryParserStateData__0p__0c__char__0p__0c__int__0c__short__0c__0x5f426f6f6c__0r {
 type Physical = PhysicalFunction_C_unwind_6<::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void, *mut crate::__pgrx_c_bindings::TSQueryParserStateData, *mut ::core::ffi::c_char, i32, i16, bool) -> ()>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__void__0p__0c__void__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0l__void__0p__0c__void__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_void__0l__void__0p__0c__void__0p__0r {
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> (),
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const u8, i32, *mut bool) -> ()>,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const u8, i32, *mut bool) -> ()>,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__const__unsigned__char__0p__0c__int__0c__0x5f426f6f6c__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed
    for Signature_void__0l__const__unsigned__char__0p__0c__int__0c__0x5f426f6f6c__0p__0r
{
}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__const__unsigned__char__0p__0c__int__0c__0x5f426f6f6c__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const u8, i32, *mut bool) -> ()>,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn(i32, u32) -> ()>>()
        == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn(i32, u32) -> ()>>()
        == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__int__0c__enum__BufferLockMode__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0l__int__0c__enum__BufferLockMode__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__int__0c__enum__BufferLockMode__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<unsafe extern "C-unwind" fn(i32, u32) -> ()>,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::BufferDesc) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::BufferDesc) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__BufferDesc__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0l__struct__BufferDesc__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__BufferDesc__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::BufferDesc) -> (),
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::IOStats,
                *mut crate::__pgrx_c_bindings::IOStats,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::IOStats,
                *mut crate::__pgrx_c_bindings::IOStats,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__IOStats__0p__0c__struct__IOStats__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0l__struct__IOStats__0p__0c__struct__IOStats__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__IOStats__0p__0c__struct__IOStats__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::IOStats,
                *mut crate::__pgrx_c_bindings::IOStats,
            ) -> (),
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void, u32) -> ()>,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void, u32) -> ()>,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__void__0p__0c__enum__vartag_external__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0l__void__0p__0c__enum__vartag_external__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__void__0p__0c__enum__vartag_external__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void, u32) -> ()>,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void, usize) -> ()>,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void, usize) -> ()>,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__void__0p__0c__unsigned__long__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0l__void__0p__0c__unsigned__long__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__void__0p__0c__unsigned__long__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void, usize) -> ()>,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PageXLogRecPtr, u64) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PageXLogRecPtr, u64) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__volatile__PageXLogRecPtr__0p__0c__unsigned__long__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed
    for Signature_void__0l__volatile__PageXLogRecPtr__0p__0c__unsigned__long__0r
{
}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__volatile__PageXLogRecPtr__0p__0c__unsigned__long__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PageXLogRecPtr, u64) -> (),
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn(*mut u8) -> ()>>()
        == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn(*mut u8) -> ()>>()
        == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__volatile__unsigned__char__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0l__volatile__unsigned__char__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__volatile__unsigned__char__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(*mut u8) -> ()>,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                usize,
                *mut ::core::ffi::c_void,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                usize,
                *mut ::core::ffi::c_void,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0p__0l__unsigned__long__0c__void__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0p__0l__unsigned__long__0c__void__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_void__0p__0l__unsigned__long__0c__void__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                usize,
                *mut ::core::ffi::c_void,
            ) -> *mut ::core::ffi::c_void,
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_char,
                *const crate::__pgrx_c_bindings::ItemIdData,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_char,
                *const crate::__pgrx_c_bindings::ItemIdData,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0p__0l__char__0p__0c__const__struct__ItemIdData__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0p__0l__char__0p__0c__const__struct__ItemIdData__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_void__0p__0l__char__0p__0c__const__struct__ItemIdData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_char,
                *const crate::__pgrx_c_bindings::ItemIdData,
            ) -> *mut ::core::ffi::c_void,
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0p__0l__const__void__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0p__0l__const__void__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_void__0p__0l__const__void__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerGlobal,
                i32,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerGlobal,
                i32,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0p__0l__struct__PlannerGlobal__0p__0c__int__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0p__0l__struct__PlannerGlobal__0p__0c__int__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_void__0p__0l__struct__PlannerGlobal__0p__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerGlobal,
                i32,
            ) -> *mut ::core::ffi::c_void,
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerInfo,
                i32,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerInfo,
                i32,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0p__0l__struct__PlannerInfo__0p__0c__int__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0p__0l__struct__PlannerInfo__0p__0c__int__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_void__0p__0l__struct__PlannerInfo__0p__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerInfo,
                i32,
            ) -> *mut ::core::ffi::c_void,
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelOptInfo,
                i32,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelOptInfo,
                i32,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0p__0l__struct__RelOptInfo__0p__0c__int__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0p__0l__struct__RelOptInfo__0p__0c__int__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_void__0p__0l__struct__RelOptInfo__0p__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelOptInfo,
                i32,
            ) -> *mut ::core::ffi::c_void,
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::dshash_table,
                *const ::core::ffi::c_void,
                *mut bool,
                i32,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::dshash_table,
                *const ::core::ffi::c_void,
                *mut bool,
                i32,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0p__0l__struct__dshash_table__0p__0c__const__void__0p__0c__0x5f426f6f6c__0p__0c__int__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0p__0l__struct__dshash_table__0p__0c__const__void__0p__0c__0x5f426f6f6c__0p__0c__int__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_void__0p__0l__struct__dshash_table__0p__0c__const__void__0p__0c__0x5f426f6f6c__0p__0c__int__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::dshash_table, *const ::core::ffi::c_void, *mut bool, i32) -> *mut ::core::ffi::c_void>>;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_numNonLeafPages;
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::GenericCosts>() == 72);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::GenericCosts>() == 8);
};
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GenericCosts, numNonLeafPages) == 64);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numNonLeafPages>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GenericCosts>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::GenericCosts, <<c::expression::CRecord<crate::__pgrx_c_bindings::GenericCosts> as c::expression::OrdinaryField<Field_numNonLeafPages>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numNonLeafPages) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::InternalGrant>() == 72);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::InternalGrant>() == 8);
};
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::InternalGrant, behavior) == 64);
#[cfg(feature = "pg19")]
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
    const OFFSET: usize = 64;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PGChecksummablePage>() == 8192);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PGChecksummablePage>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PageXLogRecPtr>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PageXLogRecPtr>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PageXLogRecPtr, lsn) == 0);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PageXLogRecPtr>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PageXLogRecPtr, <<c::expression::CRecord<crate::__pgrx_c_bindings::PageXLogRecPtr> as c::expression::OrdinaryField<Field_lsn>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lsn) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_curbyte;
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SortTuple, curbyte) == 17);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_curbyte>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SortTuple>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 17;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SortTuple,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SortTuple> as c::expression::OrdinaryField<
        Field_curbyte,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).curbyte) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_apply_reply_requested;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WalRcvData, apply_reply_requested) == 2240
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_apply_reply_requested>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 2240;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WalRcvData, <<c::expression::CRecord<crate::__pgrx_c_bindings::WalRcvData> as c::expression::OrdinaryField<Field_apply_reply_requested>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).apply_reply_requested) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BufferDesc>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BufferDesc>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BufferDesc, wait_backend_pgprocno) == 32
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wait_backend_pgprocno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BufferDesc>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::HASHCTL>() == 80);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::HASHCTL>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HASHCTL, keysize) == 8);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_keysize>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HASHCTL>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HASHCTL, entrysize) == 16);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_entrysize>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HASHCTL>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::IoMethodOps>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::IoMethodOps>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry>() == 96);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry>() == 8);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_mem_exceeded_count;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry, mem_exceeded_count)
        == 48
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mem_exceeded_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry> as c::expression::OrdinaryField<Field_mem_exceeded_count>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mem_exceeded_count) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry, total_txns) == 56
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_total_txns>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry, total_bytes) == 64
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_total_bytes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_slotsync_skip_count;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry,
        slotsync_skip_count
    ) == 72
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_slotsync_skip_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry> as c::expression::OrdinaryField<Field_slotsync_skip_count>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).slotsync_skip_count) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_slotsync_last_skip;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry, slotsync_last_skip)
        == 80
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_slotsync_last_skip>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry> as c::expression::OrdinaryField<Field_slotsync_last_skip>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).slotsync_last_skip) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry,
        stat_reset_timestamp
    ) == 88
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stat_reset_timestamp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ShmemCallbacks>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ShmemCallbacks>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ShmemCallbacks {}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ShmemCallbacks, flags) == 0);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ShmemCallbacks>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ShmemCallbacks, <<c::expression::CRecord<crate::__pgrx_c_bindings::ShmemCallbacks> as c::expression::OrdinaryField<Field_flags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).flags) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ShmemHashOpts>() == 144);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ShmemHashOpts>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ShmemHashOpts {}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ShmemHashOpts, nelems) == 40);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nelems>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ShmemHashOpts>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ShmemHashOpts, <<c::expression::CRecord<crate::__pgrx_c_bindings::ShmemHashOpts> as c::expression::OrdinaryField<Field_nelems>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nelems) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_hash_flags;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ShmemHashOpts, hash_flags) == 128);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hash_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ShmemHashOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ShmemHashOpts, <<c::expression::CRecord<crate::__pgrx_c_bindings::ShmemHashOpts> as c::expression::OrdinaryField<Field_hash_flags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hash_flags) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ShmemStructOpts>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ShmemStructOpts>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ShmemStructOpts {}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ShmemStructOpts, size) == 8);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ShmemStructOpts>
{
    type Member = c::expression::CIntegerStorage<c::CLong, isize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ShmemStructOpts, <<c::expression::CRecord<crate::__pgrx_c_bindings::ShmemStructOpts> as c::expression::OrdinaryField<Field_size>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).size) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_alignment;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ShmemStructOpts, alignment) == 16);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_alignment>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ShmemStructOpts>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ShmemStructOpts, <<c::expression::CRecord<crate::__pgrx_c_bindings::ShmemStructOpts> as c::expression::OrdinaryField<Field_alignment>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).alignment) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_nslots;
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SlruOpts>() == 104);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SlruOpts>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SlruOpts {}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SlruOpts, nslots) == 48);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nslots>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SlruOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SlruOpts,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SlruOpts> as c::expression::OrdinaryField<
        Field_nslots,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nslots) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_nlsns;
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SlruOpts, nlsns) == 52);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nlsns>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SlruOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SlruOpts,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SlruOpts> as c::expression::OrdinaryField<
        Field_nlsns,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nlsns) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SlruOpts, sync_handler) == 56);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sync_handler>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SlruOpts>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__SyncRequestHandler,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SlruOpts,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SlruOpts> as c::expression::OrdinaryField<
        Field_sync_handler,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sync_handler) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SlruOpts, long_segment_names) == 72);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_long_segment_names>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SlruOpts>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SlruOpts,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SlruOpts> as c::expression::OrdinaryField<
        Field_long_segment_names,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).long_segment_names) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_buffer_tranche_id;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SlruOpts, buffer_tranche_id) == 96);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_buffer_tranche_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SlruOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SlruOpts,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SlruOpts> as c::expression::OrdinaryField<
        Field_buffer_tranche_id,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).buffer_tranche_id) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_bank_tranche_id;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SlruOpts, bank_tranche_id) == 100);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bank_tranche_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SlruOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 100;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SlruOpts,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SlruOpts> as c::expression::OrdinaryField<
        Field_bank_tranche_id,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).bank_tranche_id) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_frequency_khz;
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TscClockSourceInfo>() == 136);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TscClockSourceInfo>() == 4);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TscClockSourceInfo {}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TscClockSourceInfo, frequency_khz) == 0
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_frequency_khz>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TscClockSourceInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TscClockSourceInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::TscClockSourceInfo> as c::expression::OrdinaryField<Field_frequency_khz>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).frequency_khz) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_calibrated_frequency_khz;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TscClockSourceInfo, calibrated_frequency_khz)
        == 4
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_calibrated_frequency_khz>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TscClockSourceInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TscClockSourceInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::TscClockSourceInfo> as c::expression::OrdinaryField<Field_calibrated_frequency_khz>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).calibrated_frequency_khz) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::VacuumParams>() == 64);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::VacuumParams>() == 8);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_log_vacuum_min_duration;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::VacuumParams, log_vacuum_min_duration) == 24
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_log_vacuum_min_duration>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacuumParams>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VacuumParams, <<c::expression::CRecord<crate::__pgrx_c_bindings::VacuumParams> as c::expression::OrdinaryField<Field_log_vacuum_min_duration>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).log_vacuum_min_duration) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_log_analyze_min_duration;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::VacuumParams, log_analyze_min_duration) == 28
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_log_analyze_min_duration>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacuumParams>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VacuumParams, <<c::expression::CRecord<crate::__pgrx_c_bindings::VacuumParams> as c::expression::OrdinaryField<Field_log_analyze_min_duration>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).log_analyze_min_duration) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VacuumParams, index_cleanup) == 32);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_index_cleanup>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacuumParams>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__VacOptValue,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VacuumParams, truncate) == 36);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_truncate>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacuumParams>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__VacOptValue,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VacuumParams, toast_parent) == 40);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_toast_parent>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacuumParams>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VacuumParams, nworkers) == 56);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nworkers>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacuumParams>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::WalUsage>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::WalUsage>() == 8);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_wal_fpi_bytes;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WalUsage, wal_fpi_bytes) == 24);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wal_fpi_bytes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalUsage>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::WalUsage,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::WalUsage> as c::expression::OrdinaryField<
        Field_wal_fpi_bytes,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wal_fpi_bytes) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WalUsage, wal_buffers_full) == 32);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wal_buffers_full>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalUsage>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::collate_methods>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::collate_methods>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::collate_methods, strxfrm_is_safe) == 48
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_strxfrm_is_safe>
    for c::expression::CRecord<crate::__pgrx_c_bindings::collate_methods>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::config_enum>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::config_enum>() == 8);
};
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_enum, boot_val) == 8);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_boot_val>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_enum>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_enum, reset_val) == 48);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_reset_val>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_enum>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::config_generic>() == 216);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::config_generic>() == 8);
};
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_generic, sourceline) == 144);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sourceline>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_generic>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::pg_signal_info>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::pg_signal_info>() == 4);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::pg_signal_info {}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_signal_info, pid) == 0);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_signal_info>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::pg_signal_info, <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_signal_info> as c::expression::OrdinaryField<Field_pid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pid) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_uid;
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_signal_info, uid) == 4);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_uid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_signal_info>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::pg_signal_info, <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_signal_info> as c::expression::OrdinaryField<Field_uid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).uid) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AggClauseInfo>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AggClauseInfo>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AggClauseInfo {}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggClauseInfo, type_) == 0);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggClauseInfo>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggClauseInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggClauseInfo> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_subtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterDomainStmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__AlterDomainType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterPublicationStmt, action) == 32);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_action>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterPublicationStmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__AlterPublicationAction,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterPublicationStmt, for_all_tables) == 36
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_for_all_tables>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterPublicationStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_for_all_sequences;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterPublicationStmt, for_all_sequences)
        == 37
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_for_all_sequences>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterPublicationStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 37;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterPublicationStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterPublicationStmt> as c::expression::OrdinaryField<Field_for_all_sequences>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).for_all_sequences) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AlterSubscriptionStmt>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AlterSubscriptionStmt>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterSubscriptionStmt, publication) == 32
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_publication>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterSubscriptionStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterSubscriptionStmt, options) == 40
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterSubscriptionStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_child_append_relid_sets;
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Append>() == 144);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Append>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::Append, child_append_relid_sets) == 112
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_child_append_relid_sets>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Append>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Append,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Append> as c::expression::OrdinaryField<
        Field_child_append_relid_sets,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).child_append_relid_sets) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Append, appendplans) == 120);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_appendplans>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Append>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Append, nasyncplans) == 128);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nasyncplans>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Append>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Append, first_partial_plan) == 132);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_first_partial_plan>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Append>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 132;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Append, part_prune_index) == 136);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_part_prune_index>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Append>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AppendPath>() == 112);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AppendPath>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AppendPath, child_append_relid_sets) == 104
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_child_append_relid_sets>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AppendPath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AppendPath, <<c::expression::CRecord<crate::__pgrx_c_bindings::AppendPath> as c::expression::OrdinaryField<Field_child_append_relid_sets>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).child_append_relid_sets) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AutoVacOpts>() == 96);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AutoVacOpts>() == 8);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_autovacuum_parallel_workers;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, autovacuum_parallel_workers)
        == 4
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_autovacuum_parallel_workers>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AutoVacOpts, <<c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts> as c::expression::OrdinaryField<Field_autovacuum_parallel_workers>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).autovacuum_parallel_workers) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, vacuum_threshold) == 8);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vacuum_threshold>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, vacuum_max_threshold) == 12
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vacuum_max_threshold>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, vacuum_ins_threshold) == 16
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vacuum_ins_threshold>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, analyze_threshold) == 20
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_analyze_threshold>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, vacuum_cost_limit) == 24
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vacuum_cost_limit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, freeze_min_age) == 28);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_freeze_min_age>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, freeze_max_age) == 32);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_freeze_max_age>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, freeze_table_age) == 36);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_freeze_table_age>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, multixact_freeze_min_age) == 40
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_multixact_freeze_min_age>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, multixact_freeze_max_age) == 44
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_multixact_freeze_max_age>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, multixact_freeze_table_age)
        == 48
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_multixact_freeze_table_age>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, log_vacuum_min_duration) == 52
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_log_vacuum_min_duration>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AutoVacOpts, <<c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts> as c::expression::OrdinaryField<Field_log_vacuum_min_duration>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).log_vacuum_min_duration) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AutoVacOpts, log_analyze_min_duration) == 56
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_log_analyze_min_duration>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AutoVacOpts, <<c::expression::CRecord<crate::__pgrx_c_bindings::AutoVacOpts> as c::expression::OrdinaryField<Field_log_analyze_min_duration>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).log_analyze_min_duration) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::BitmapHeapScanInstrumentation>() == 72
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::BitmapHeapScanInstrumentation>() == 8
    );
};
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BitmapHeapScanState>() == 344);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BitmapHeapScanState>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BitmapHeapScanState, initialized) == 312
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_initialized>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BitmapHeapScanState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 312;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BitmapHeapScanState, recheck) == 336);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_recheck>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BitmapHeapScanState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 336;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BufferHeapTupleTableSlot>() == 120);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BufferHeapTupleTableSlot>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BufferHeapTupleTableSlot, buffer) == 112
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_buffer>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BufferHeapTupleTableSlot>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BufferManagerRelation, rel) == 0);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rel>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BufferManagerRelation>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BufferManagerRelation, <<c::expression::CRecord<crate::__pgrx_c_bindings::BufferManagerRelation> as c::expression::OrdinaryField<Field_rel>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rel) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_smgr;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BufferManagerRelation, smgr) == 8);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_smgr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BufferManagerRelation>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::SMgrRelationData>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BufferManagerRelation, <<c::expression::CRecord<crate::__pgrx_c_bindings::BufferManagerRelation> as c::expression::OrdinaryField<Field_smgr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).smgr) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CheckPoint>() == 96);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CheckPoint>() == 8);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_logicalDecodingEnabled;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, logicalDecodingEnabled) == 24
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_logicalDecodingEnabled>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckPoint, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint> as c::expression::OrdinaryField<Field_logicalDecodingEnabled>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).logicalDecodingEnabled) };
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, nextOid) == 40);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nextOid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, nextMulti) == 44);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nextMulti>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::MultiXactId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, nextMultiOffset) == 48);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nextMultiOffset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, oldestXid) == 56);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldestXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, oldestXidDB) == 60);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldestXidDB>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 60;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, oldestMulti) == 64);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldestMulti>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::MultiXactId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, oldestMultiDB) == 68);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldestMultiDB>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 68;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, time) == 72);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, oldestCommitTsXid) == 80);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldestCommitTsXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, newestCommitTsXid) == 84);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_newestCommitTsXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 84;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, oldestActiveXid) == 88);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldestActiveXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CheckPointStmt>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CheckPointStmt>() == 8);
};
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPointStmt, options) == 8);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPointStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckPointStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckPointStmt> as c::expression::OrdinaryField<Field_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CompactAttribute>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CompactAttribute>() == 2);
};
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attcacheoff>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CompactAttribute>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CompactAttribute, attlen) == 2);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attlen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CompactAttribute>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 2;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CompactAttribute, attbyval) == 4);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attbyval>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CompactAttribute>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CompactAttribute, attalignby) == 5);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attalignby>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CompactAttribute>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 5;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::CompactAttribute, _bitfield_1) * 8 + 0
            == 48
    );
};
#[cfg(feature = "pg19")]
unsafe extern "C" {
    fn __pgrx_bitfield__get__CompactAttribute__attispackable(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set__CompactAttribute__attispackable(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
    fn __pgrx_bitfield__get_volatile__CompactAttribute__attispackable(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set_volatile__CompactAttribute__attispackable(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
    fn __pgrx_bitfield__get_unaligned__CompactAttribute__attispackable(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set_unaligned__CompactAttribute__attispackable(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
    fn __pgrx_bitfield__get_unaligned_volatile__CompactAttribute__attispackable(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set_unaligned_volatile__CompactAttribute__attispackable(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Bitfield_CompactAttribute__attispackable<Q: c::expression::Qualifier> {
    address: Q::Raw<crate::__pgrx_c_bindings::CompactAttribute>,
    access: c::expression::Access,
}
#[cfg(feature = "pg19")]
impl<Q: c::expression::Qualifier> Copy for Bitfield_CompactAttribute__attispackable<Q> {}
#[cfg(feature = "pg19")]
impl<Q: c::expression::Qualifier> Clone for Bitfield_CompactAttribute__attispackable<Q> {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "pg19")]
// SAFETY: Clang and bindgen layout witnesses agree. This projects the record address without a reference or record read.
unsafe impl<Q: c::expression::Qualifier> c::expression::Field<Field_attispackable, Q>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CompactAttribute>
{
    type Output = Bitfield_CompactAttribute__attispackable<Q>;
    unsafe fn project(base: c::expression::Place<Self, Q>) -> Self::Output {
        Bitfield_CompactAttribute__attispackable::<Q> {
            address: <Q as c::expression::Qualifier>::from_mut(base.pointer().as_mut_address()),
            access: c::expression::Access {
                volatile: base.access().volatile || false,
                unaligned: base.access().unaligned,
            },
        }
    }
}
#[cfg(feature = "pg19")]
impl<Q: c::expression::Qualifier> c::expression::VolatilePlace
    for Bitfield_CompactAttribute__attispackable<Q>
{
    type Qualified = Self;
    fn qualify_volatile(mut self) -> Self::Qualified {
        self.access.volatile = true;
        self
    }
}
#[cfg(feature = "pg19")]
// SAFETY: The same-profile C compiler owns bitfield storage reads, including partially initialized neighbor/padding bits. The marker retains declared size and compiler-proved arithmetic promotion.
unsafe impl<Q: c::expression::Qualifier> c::expression::ReadPlace
    for Bitfield_CompactAttribute__attispackable<Q>
{
    type Object = c::CBool;
    type Type = c::expression::CBitfield<c::CBool, c::CInt>;
    unsafe fn load(self) -> <Self::Type as c::expression::CType>::Value {
        let pointer = Q::into_mut(self.address).cast_const(); // SAFETY: The caller establishes the original C field load contract; helpers perform only that field access.
        let storage = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__get_unaligned_volatile__CompactAttribute__attispackable(
                        pointer,
                    )
                } else {
                    __pgrx_bitfield__get_unaligned__CompactAttribute__attispackable(pointer)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__get_volatile__CompactAttribute__attispackable(pointer)
            } else {
                __pgrx_bitfield__get__CompactAttribute__attispackable(pointer)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(storage)
    }
}
#[cfg(feature = "pg19")]
// SAFETY: Write capability retains ReadWrite qualification. Original C performs declared-base conversion and width narrowing without Rust byte reads.
unsafe impl c::expression::WritePlace
    for Bitfield_CompactAttribute__attispackable<c::expression::ReadWrite>
{
    type Assignment = c::CBool;
    unsafe fn store(
        self,
        value: <Self::Assignment as c::expression::CType>::Value,
    ) -> <Self::Type as c::expression::CType>::Value {
        let value = <Self::Assignment as c::expression::CType>::into_storage(value); // SAFETY: The caller establishes writable field storage; this helper evaluates one original-C assignment and returns its narrowed result.
        let stored = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__set_unaligned_volatile__CompactAttribute__attispackable(
                        self.address,
                        value,
                    )
                } else {
                    __pgrx_bitfield__set_unaligned__CompactAttribute__attispackable(
                        self.address,
                        value,
                    )
                }
            } else if self.access.volatile {
                __pgrx_bitfield__set_volatile__CompactAttribute__attispackable(self.address, value)
            } else {
                __pgrx_bitfield__set__CompactAttribute__attispackable(self.address, value)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(stored)
    }
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::CompactAttribute, _bitfield_1) * 8 + 1
            == 49
    );
};
#[cfg(feature = "pg19")]
unsafe extern "C" {
    fn __pgrx_bitfield__get__CompactAttribute__atthasmissing(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set__CompactAttribute__atthasmissing(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
    fn __pgrx_bitfield__get_volatile__CompactAttribute__atthasmissing(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set_volatile__CompactAttribute__atthasmissing(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
    fn __pgrx_bitfield__get_unaligned__CompactAttribute__atthasmissing(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set_unaligned__CompactAttribute__atthasmissing(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
    fn __pgrx_bitfield__get_unaligned_volatile__CompactAttribute__atthasmissing(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set_unaligned_volatile__CompactAttribute__atthasmissing(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Bitfield_CompactAttribute__atthasmissing<Q: c::expression::Qualifier> {
    address: Q::Raw<crate::__pgrx_c_bindings::CompactAttribute>,
    access: c::expression::Access,
}
#[cfg(feature = "pg19")]
impl<Q: c::expression::Qualifier> Copy for Bitfield_CompactAttribute__atthasmissing<Q> {}
#[cfg(feature = "pg19")]
impl<Q: c::expression::Qualifier> Clone for Bitfield_CompactAttribute__atthasmissing<Q> {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "pg19")]
// SAFETY: Clang and bindgen layout witnesses agree. This projects the record address without a reference or record read.
unsafe impl<Q: c::expression::Qualifier> c::expression::Field<Field_atthasmissing, Q>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CompactAttribute>
{
    type Output = Bitfield_CompactAttribute__atthasmissing<Q>;
    unsafe fn project(base: c::expression::Place<Self, Q>) -> Self::Output {
        Bitfield_CompactAttribute__atthasmissing::<Q> {
            address: <Q as c::expression::Qualifier>::from_mut(base.pointer().as_mut_address()),
            access: c::expression::Access {
                volatile: base.access().volatile || false,
                unaligned: base.access().unaligned,
            },
        }
    }
}
#[cfg(feature = "pg19")]
impl<Q: c::expression::Qualifier> c::expression::VolatilePlace
    for Bitfield_CompactAttribute__atthasmissing<Q>
{
    type Qualified = Self;
    fn qualify_volatile(mut self) -> Self::Qualified {
        self.access.volatile = true;
        self
    }
}
#[cfg(feature = "pg19")]
// SAFETY: The same-profile C compiler owns bitfield storage reads, including partially initialized neighbor/padding bits. The marker retains declared size and compiler-proved arithmetic promotion.
unsafe impl<Q: c::expression::Qualifier> c::expression::ReadPlace
    for Bitfield_CompactAttribute__atthasmissing<Q>
{
    type Object = c::CBool;
    type Type = c::expression::CBitfield<c::CBool, c::CInt>;
    unsafe fn load(self) -> <Self::Type as c::expression::CType>::Value {
        let pointer = Q::into_mut(self.address).cast_const(); // SAFETY: The caller establishes the original C field load contract; helpers perform only that field access.
        let storage = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__get_unaligned_volatile__CompactAttribute__atthasmissing(
                        pointer,
                    )
                } else {
                    __pgrx_bitfield__get_unaligned__CompactAttribute__atthasmissing(pointer)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__get_volatile__CompactAttribute__atthasmissing(pointer)
            } else {
                __pgrx_bitfield__get__CompactAttribute__atthasmissing(pointer)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(storage)
    }
}
#[cfg(feature = "pg19")]
// SAFETY: Write capability retains ReadWrite qualification. Original C performs declared-base conversion and width narrowing without Rust byte reads.
unsafe impl c::expression::WritePlace
    for Bitfield_CompactAttribute__atthasmissing<c::expression::ReadWrite>
{
    type Assignment = c::CBool;
    unsafe fn store(
        self,
        value: <Self::Assignment as c::expression::CType>::Value,
    ) -> <Self::Type as c::expression::CType>::Value {
        let value = <Self::Assignment as c::expression::CType>::into_storage(value); // SAFETY: The caller establishes writable field storage; this helper evaluates one original-C assignment and returns its narrowed result.
        let stored = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__set_unaligned_volatile__CompactAttribute__atthasmissing(
                        self.address,
                        value,
                    )
                } else {
                    __pgrx_bitfield__set_unaligned__CompactAttribute__atthasmissing(
                        self.address,
                        value,
                    )
                }
            } else if self.access.volatile {
                __pgrx_bitfield__set_volatile__CompactAttribute__atthasmissing(self.address, value)
            } else {
                __pgrx_bitfield__set__CompactAttribute__atthasmissing(self.address, value)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(stored)
    }
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::CompactAttribute, _bitfield_1) * 8 + 2
            == 50
    );
};
#[cfg(feature = "pg19")]
unsafe extern "C" {
    fn __pgrx_bitfield__get__CompactAttribute__attisdropped(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set__CompactAttribute__attisdropped(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
    fn __pgrx_bitfield__get_volatile__CompactAttribute__attisdropped(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set_volatile__CompactAttribute__attisdropped(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
    fn __pgrx_bitfield__get_unaligned__CompactAttribute__attisdropped(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set_unaligned__CompactAttribute__attisdropped(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
    fn __pgrx_bitfield__get_unaligned_volatile__CompactAttribute__attisdropped(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set_unaligned_volatile__CompactAttribute__attisdropped(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Bitfield_CompactAttribute__attisdropped<Q: c::expression::Qualifier> {
    address: Q::Raw<crate::__pgrx_c_bindings::CompactAttribute>,
    access: c::expression::Access,
}
#[cfg(feature = "pg19")]
impl<Q: c::expression::Qualifier> Copy for Bitfield_CompactAttribute__attisdropped<Q> {}
#[cfg(feature = "pg19")]
impl<Q: c::expression::Qualifier> Clone for Bitfield_CompactAttribute__attisdropped<Q> {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "pg19")]
// SAFETY: Clang and bindgen layout witnesses agree. This projects the record address without a reference or record read.
unsafe impl<Q: c::expression::Qualifier> c::expression::Field<Field_attisdropped, Q>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CompactAttribute>
{
    type Output = Bitfield_CompactAttribute__attisdropped<Q>;
    unsafe fn project(base: c::expression::Place<Self, Q>) -> Self::Output {
        Bitfield_CompactAttribute__attisdropped::<Q> {
            address: <Q as c::expression::Qualifier>::from_mut(base.pointer().as_mut_address()),
            access: c::expression::Access {
                volatile: base.access().volatile || false,
                unaligned: base.access().unaligned,
            },
        }
    }
}
#[cfg(feature = "pg19")]
impl<Q: c::expression::Qualifier> c::expression::VolatilePlace
    for Bitfield_CompactAttribute__attisdropped<Q>
{
    type Qualified = Self;
    fn qualify_volatile(mut self) -> Self::Qualified {
        self.access.volatile = true;
        self
    }
}
#[cfg(feature = "pg19")]
// SAFETY: The same-profile C compiler owns bitfield storage reads, including partially initialized neighbor/padding bits. The marker retains declared size and compiler-proved arithmetic promotion.
unsafe impl<Q: c::expression::Qualifier> c::expression::ReadPlace
    for Bitfield_CompactAttribute__attisdropped<Q>
{
    type Object = c::CBool;
    type Type = c::expression::CBitfield<c::CBool, c::CInt>;
    unsafe fn load(self) -> <Self::Type as c::expression::CType>::Value {
        let pointer = Q::into_mut(self.address).cast_const(); // SAFETY: The caller establishes the original C field load contract; helpers perform only that field access.
        let storage = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__get_unaligned_volatile__CompactAttribute__attisdropped(pointer)
                } else {
                    __pgrx_bitfield__get_unaligned__CompactAttribute__attisdropped(pointer)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__get_volatile__CompactAttribute__attisdropped(pointer)
            } else {
                __pgrx_bitfield__get__CompactAttribute__attisdropped(pointer)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(storage)
    }
}
#[cfg(feature = "pg19")]
// SAFETY: Write capability retains ReadWrite qualification. Original C performs declared-base conversion and width narrowing without Rust byte reads.
unsafe impl c::expression::WritePlace
    for Bitfield_CompactAttribute__attisdropped<c::expression::ReadWrite>
{
    type Assignment = c::CBool;
    unsafe fn store(
        self,
        value: <Self::Assignment as c::expression::CType>::Value,
    ) -> <Self::Type as c::expression::CType>::Value {
        let value = <Self::Assignment as c::expression::CType>::into_storage(value); // SAFETY: The caller establishes writable field storage; this helper evaluates one original-C assignment and returns its narrowed result.
        let stored = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__set_unaligned_volatile__CompactAttribute__attisdropped(
                        self.address,
                        value,
                    )
                } else {
                    __pgrx_bitfield__set_unaligned__CompactAttribute__attisdropped(
                        self.address,
                        value,
                    )
                }
            } else if self.access.volatile {
                __pgrx_bitfield__set_volatile__CompactAttribute__attisdropped(self.address, value)
            } else {
                __pgrx_bitfield__set__CompactAttribute__attisdropped(self.address, value)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(stored)
    }
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::CompactAttribute, _bitfield_1) * 8 + 3
            == 51
    );
};
#[cfg(feature = "pg19")]
unsafe extern "C" {
    fn __pgrx_bitfield__get__CompactAttribute__attgenerated(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set__CompactAttribute__attgenerated(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
    fn __pgrx_bitfield__get_volatile__CompactAttribute__attgenerated(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set_volatile__CompactAttribute__attgenerated(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
    fn __pgrx_bitfield__get_unaligned__CompactAttribute__attgenerated(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set_unaligned__CompactAttribute__attgenerated(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
    fn __pgrx_bitfield__get_unaligned_volatile__CompactAttribute__attgenerated(
        p: *const crate::__pgrx_c_bindings::CompactAttribute,
    ) -> bool;
    fn __pgrx_bitfield__set_unaligned_volatile__CompactAttribute__attgenerated(
        p: *mut crate::__pgrx_c_bindings::CompactAttribute,
        value: bool,
    ) -> bool;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Bitfield_CompactAttribute__attgenerated<Q: c::expression::Qualifier> {
    address: Q::Raw<crate::__pgrx_c_bindings::CompactAttribute>,
    access: c::expression::Access,
}
#[cfg(feature = "pg19")]
impl<Q: c::expression::Qualifier> Copy for Bitfield_CompactAttribute__attgenerated<Q> {}
#[cfg(feature = "pg19")]
impl<Q: c::expression::Qualifier> Clone for Bitfield_CompactAttribute__attgenerated<Q> {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "pg19")]
// SAFETY: Clang and bindgen layout witnesses agree. This projects the record address without a reference or record read.
unsafe impl<Q: c::expression::Qualifier> c::expression::Field<Field_attgenerated, Q>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CompactAttribute>
{
    type Output = Bitfield_CompactAttribute__attgenerated<Q>;
    unsafe fn project(base: c::expression::Place<Self, Q>) -> Self::Output {
        Bitfield_CompactAttribute__attgenerated::<Q> {
            address: <Q as c::expression::Qualifier>::from_mut(base.pointer().as_mut_address()),
            access: c::expression::Access {
                volatile: base.access().volatile || false,
                unaligned: base.access().unaligned,
            },
        }
    }
}
#[cfg(feature = "pg19")]
impl<Q: c::expression::Qualifier> c::expression::VolatilePlace
    for Bitfield_CompactAttribute__attgenerated<Q>
{
    type Qualified = Self;
    fn qualify_volatile(mut self) -> Self::Qualified {
        self.access.volatile = true;
        self
    }
}
#[cfg(feature = "pg19")]
// SAFETY: The same-profile C compiler owns bitfield storage reads, including partially initialized neighbor/padding bits. The marker retains declared size and compiler-proved arithmetic promotion.
unsafe impl<Q: c::expression::Qualifier> c::expression::ReadPlace
    for Bitfield_CompactAttribute__attgenerated<Q>
{
    type Object = c::CBool;
    type Type = c::expression::CBitfield<c::CBool, c::CInt>;
    unsafe fn load(self) -> <Self::Type as c::expression::CType>::Value {
        let pointer = Q::into_mut(self.address).cast_const(); // SAFETY: The caller establishes the original C field load contract; helpers perform only that field access.
        let storage = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__get_unaligned_volatile__CompactAttribute__attgenerated(pointer)
                } else {
                    __pgrx_bitfield__get_unaligned__CompactAttribute__attgenerated(pointer)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__get_volatile__CompactAttribute__attgenerated(pointer)
            } else {
                __pgrx_bitfield__get__CompactAttribute__attgenerated(pointer)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(storage)
    }
}
#[cfg(feature = "pg19")]
// SAFETY: Write capability retains ReadWrite qualification. Original C performs declared-base conversion and width narrowing without Rust byte reads.
unsafe impl c::expression::WritePlace
    for Bitfield_CompactAttribute__attgenerated<c::expression::ReadWrite>
{
    type Assignment = c::CBool;
    unsafe fn store(
        self,
        value: <Self::Assignment as c::expression::CType>::Value,
    ) -> <Self::Type as c::expression::CType>::Value {
        let value = <Self::Assignment as c::expression::CType>::into_storage(value); // SAFETY: The caller establishes writable field storage; this helper evaluates one original-C assignment and returns its narrowed result.
        let stored = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__set_unaligned_volatile__CompactAttribute__attgenerated(
                        self.address,
                        value,
                    )
                } else {
                    __pgrx_bitfield__set_unaligned__CompactAttribute__attgenerated(
                        self.address,
                        value,
                    )
                }
            } else if self.access.volatile {
                __pgrx_bitfield__set_volatile__CompactAttribute__attgenerated(self.address, value)
            } else {
                __pgrx_bitfield__set__CompactAttribute__attgenerated(self.address, value)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(stored)
    }
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CompactAttribute, attnullability) == 7
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attnullability>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CompactAttribute>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 7;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ControlFileData>() == 312);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ControlFileData>() == 8);
};
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, unloggedLSN) == 136);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_unloggedLSN>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, minRecoveryPoint) == 144
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_minRecoveryPoint>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, minRecoveryPointTLI) == 152
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_minRecoveryPointTLI>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, backupStartPoint) == 160
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_backupStartPoint>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, backupEndPoint) == 168
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_backupEndPoint>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 168;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, backupEndRequired) == 176
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_backupEndRequired>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 176;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, wal_level) == 180);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wal_level>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 180;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, wal_log_hints) == 184
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wal_log_hints>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 184;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, MaxConnections) == 188
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_MaxConnections>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 188;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, max_worker_processes) == 192
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_max_worker_processes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 192;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, max_wal_senders) == 196
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_max_wal_senders>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 196;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, max_prepared_xacts) == 200
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_max_prepared_xacts>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 200;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, max_locks_per_xact) == 204
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_max_locks_per_xact>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 204;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, track_commit_timestamp)
        == 208
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_track_commit_timestamp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 208;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, maxAlign) == 212);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_maxAlign>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 212;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, blcksz) == 224);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_blcksz>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 224;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, relseg_size) == 228);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relseg_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 228;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_slru_pages_per_segment;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, slru_pages_per_segment)
        == 232
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_slru_pages_per_segment>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 232;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ControlFileData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData> as c::expression::OrdinaryField<Field_slru_pages_per_segment>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).slru_pages_per_segment) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, xlog_blcksz) == 236);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xlog_blcksz>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 236;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, xlog_seg_size) == 240
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xlog_seg_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 240;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, nameDataLen) == 244);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nameDataLen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 244;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, indexMaxKeys) == 248
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indexMaxKeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 248;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, toast_max_chunk_size) == 252
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_toast_max_chunk_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 252;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, loblksize) == 256);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_loblksize>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 256;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, float8ByVal) == 260);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_float8ByVal>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 260;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_data_checksum_version_init;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, data_checksum_version_init)
        == 264
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_data_checksum_version_init>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 264;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ControlFileData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData> as c::expression::OrdinaryField<Field_data_checksum_version_init>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).data_checksum_version_init) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, data_checksum_version)
        == 268
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_data_checksum_version>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 268;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, default_char_signedness)
        == 272
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_default_char_signedness>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 272;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, crc) == 308);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_crc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 308;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CopyFormatOptions>() == 192);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CopyFormatOptions>() == 8);
};
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, format) == 4);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_format>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__CopyFormat,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFormatOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions> as c::expression::OrdinaryField<Field_format>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).format) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, freeze) == 8);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_freeze>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, header_line) == 12
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_header_line>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_force_array;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, force_array) == 128
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_force_array>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFormatOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions> as c::expression::OrdinaryField<Field_force_array>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).force_array) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, force_null) == 136
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_force_null>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, force_null_all) == 144
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_force_null_all>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, convert_selectively)
        == 160
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_convert_selectively>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, on_error) == 164);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_on_error>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__CopyOnErrorChoice,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 164;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, log_verbosity) == 168
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_log_verbosity>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__CopyLogVerbosityChoice,
        c::CInt,
        i32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 168;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, reject_limit) == 176
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_reject_limit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 176;
}
