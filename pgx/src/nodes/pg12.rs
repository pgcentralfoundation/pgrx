use crate::{pg_sys, PgBox, PgMemoryContexts};

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum PgNode {
    Invalid = pg_sys::NodeTag_T_Invalid as isize,

    /*
     * TAGS FOR EXECUTOR NODES (execnodes.h)
     */
    IndexInfo = pg_sys::NodeTag_T_IndexInfo as isize,
    ExprContext = pg_sys::NodeTag_T_ExprContext as isize,
    ProjectionInfo = pg_sys::NodeTag_T_ProjectionInfo as isize,
    JunkFilter = pg_sys::NodeTag_T_JunkFilter as isize,
    OnConflictSetState = pg_sys::pg12_specific::NodeTag_T_OnConflictSetState as isize,
    ResultRelInfo = pg_sys::pg12_specific::NodeTag_T_ResultRelInfo as isize,
    EState = pg_sys::pg12_specific::NodeTag_T_EState as isize,
    TupleTableSlot = pg_sys::pg12_specific::NodeTag_T_TupleTableSlot as isize,

    /*
     * TAGS FOR PLAN NODES (plannodes.h)
     */
    Plan = pg_sys::pg12_specific::NodeTag_T_Plan as isize,
    Result = pg_sys::pg12_specific::NodeTag_T_Result as isize,
    ProjectSet = pg_sys::pg12_specific::NodeTag_T_ProjectSet as isize,
    ModifyTable = pg_sys::pg12_specific::NodeTag_T_ModifyTable as isize,
    Append = pg_sys::pg12_specific::NodeTag_T_Append as isize,
    MergeAppend = pg_sys::pg12_specific::NodeTag_T_MergeAppend as isize,
    RecursiveUnion = pg_sys::pg12_specific::NodeTag_T_RecursiveUnion as isize,
    BitmapAnd = pg_sys::pg12_specific::NodeTag_T_BitmapAnd as isize,
    BitmapOr = pg_sys::pg12_specific::NodeTag_T_BitmapOr as isize,
    Scan = pg_sys::pg12_specific::NodeTag_T_Scan as isize,
    SeqScan = pg_sys::pg12_specific::NodeTag_T_SeqScan as isize,
    SampleScan = pg_sys::pg12_specific::NodeTag_T_SampleScan as isize,
    IndexScan = pg_sys::pg12_specific::NodeTag_T_IndexScan as isize,
    IndexOnlyScan = pg_sys::pg12_specific::NodeTag_T_IndexOnlyScan as isize,
    BitmapIndexScan = pg_sys::pg12_specific::NodeTag_T_BitmapIndexScan as isize,
    BitmapHeapScan = pg_sys::pg12_specific::NodeTag_T_BitmapHeapScan as isize,
    TidScan = pg_sys::pg12_specific::NodeTag_T_TidScan as isize,
    SubqueryScan = pg_sys::pg12_specific::NodeTag_T_SubqueryScan as isize,
    FunctionScan = pg_sys::pg12_specific::NodeTag_T_FunctionScan as isize,
    ValuesScan = pg_sys::pg12_specific::NodeTag_T_ValuesScan as isize,
    TableFuncScan = pg_sys::pg12_specific::NodeTag_T_TableFuncScan as isize,
    CteScan = pg_sys::pg12_specific::NodeTag_T_CteScan as isize,
    NamedTuplestoreScan = pg_sys::pg12_specific::NodeTag_T_NamedTuplestoreScan as isize,
    WorkTableScan = pg_sys::pg12_specific::NodeTag_T_WorkTableScan as isize,
    ForeignScan = pg_sys::pg12_specific::NodeTag_T_ForeignScan as isize,
    CustomScan = pg_sys::pg12_specific::NodeTag_T_CustomScan as isize,
    Join = pg_sys::pg12_specific::NodeTag_T_Join as isize,
    NestLoop = pg_sys::pg12_specific::NodeTag_T_NestLoop as isize,
    MergeJoin = pg_sys::pg12_specific::NodeTag_T_MergeJoin as isize,
    HashJoin = pg_sys::pg12_specific::NodeTag_T_HashJoin as isize,
    Material = pg_sys::pg12_specific::NodeTag_T_Material as isize,
    Sort = pg_sys::pg12_specific::NodeTag_T_Sort as isize,
    Group = pg_sys::pg12_specific::NodeTag_T_Group as isize,
    Agg = pg_sys::pg12_specific::NodeTag_T_Agg as isize,
    WindowAgg = pg_sys::pg12_specific::NodeTag_T_WindowAgg as isize,
    Unique = pg_sys::pg12_specific::NodeTag_T_Unique as isize,
    Gather = pg_sys::pg12_specific::NodeTag_T_Gather as isize,
    GatherMerge = pg_sys::pg12_specific::NodeTag_T_GatherMerge as isize,
    Hash = pg_sys::pg12_specific::NodeTag_T_Hash as isize,
    SetOp = pg_sys::pg12_specific::NodeTag_T_SetOp as isize,
    LockRows = pg_sys::pg12_specific::NodeTag_T_LockRows as isize,
    Limit = pg_sys::pg12_specific::NodeTag_T_Limit as isize,
    /* these aren't subclasses of Plan: */
    NestLoopParam = pg_sys::pg12_specific::NodeTag_T_NestLoopParam as isize,
    PlanRowMark = pg_sys::pg12_specific::NodeTag_T_PlanRowMark as isize,
    PartitionPruneInfo = pg_sys::pg12_specific::NodeTag_T_PartitionPruneInfo as isize,
    PartitionedRelPruneInfo = pg_sys::pg12_specific::NodeTag_T_PartitionedRelPruneInfo as isize,
    PartitionPruneStepOp = pg_sys::pg12_specific::NodeTag_T_PartitionPruneStepOp as isize,
    PartitionPruneStepCombine = pg_sys::pg12_specific::NodeTag_T_PartitionPruneStepCombine as isize,
    PlanInvalItem = pg_sys::pg12_specific::NodeTag_T_PlanInvalItem as isize,

    /*
     * TAGS FOR PLAN STATE NODES (execnodes.h)
     *
     * These should correspond one-to-one with Plan node types.
     */
    PlanState = pg_sys::pg12_specific::NodeTag_T_PlanState as isize,
    ResultState = pg_sys::pg12_specific::NodeTag_T_ResultState as isize,
    ProjectSetState = pg_sys::pg12_specific::NodeTag_T_ProjectSetState as isize,
    ModifyTableState = pg_sys::pg12_specific::NodeTag_T_ModifyTableState as isize,
    AppendState = pg_sys::pg12_specific::NodeTag_T_AppendState as isize,
    MergeAppendState = pg_sys::pg12_specific::NodeTag_T_MergeAppendState as isize,
    RecursiveUnionState = pg_sys::pg12_specific::NodeTag_T_RecursiveUnionState as isize,
    BitmapAndState = pg_sys::pg12_specific::NodeTag_T_BitmapAndState as isize,
    BitmapOrState = pg_sys::pg12_specific::NodeTag_T_BitmapOrState as isize,
    ScanState = pg_sys::pg12_specific::NodeTag_T_ScanState as isize,
    SeqScanState = pg_sys::pg12_specific::NodeTag_T_SeqScanState as isize,
    SampleScanState = pg_sys::pg12_specific::NodeTag_T_SampleScanState as isize,
    IndexScanState = pg_sys::pg12_specific::NodeTag_T_IndexScanState as isize,
    IndexOnlyScanState = pg_sys::pg12_specific::NodeTag_T_IndexOnlyScanState as isize,
    BitmapIndexScanState = pg_sys::pg12_specific::NodeTag_T_BitmapIndexScanState as isize,
    BitmapHeapScanState = pg_sys::pg12_specific::NodeTag_T_BitmapHeapScanState as isize,
    TidScanState = pg_sys::pg12_specific::NodeTag_T_TidScanState as isize,
    SubqueryScanState = pg_sys::pg12_specific::NodeTag_T_SubqueryScanState as isize,
    FunctionScanState = pg_sys::pg12_specific::NodeTag_T_FunctionScanState as isize,
    TableFuncScanState = pg_sys::pg12_specific::NodeTag_T_TableFuncScanState as isize,
    ValuesScanState = pg_sys::pg12_specific::NodeTag_T_ValuesScanState as isize,
    CteScanState = pg_sys::pg12_specific::NodeTag_T_CteScanState as isize,
    NamedTuplestoreScanState = pg_sys::pg12_specific::NodeTag_T_NamedTuplestoreScanState as isize,
    WorkTableScanState = pg_sys::pg12_specific::NodeTag_T_WorkTableScanState as isize,
    ForeignScanState = pg_sys::pg12_specific::NodeTag_T_ForeignScanState as isize,
    CustomScanState = pg_sys::pg12_specific::NodeTag_T_CustomScanState as isize,
    JoinState = pg_sys::pg12_specific::NodeTag_T_JoinState as isize,
    NestLoopState = pg_sys::pg12_specific::NodeTag_T_NestLoopState as isize,
    MergeJoinState = pg_sys::pg12_specific::NodeTag_T_MergeJoinState as isize,
    HashJoinState = pg_sys::pg12_specific::NodeTag_T_HashJoinState as isize,
    MaterialState = pg_sys::pg12_specific::NodeTag_T_MaterialState as isize,
    SortState = pg_sys::pg12_specific::NodeTag_T_SortState as isize,
    GroupState = pg_sys::pg12_specific::NodeTag_T_GroupState as isize,
    AggState = pg_sys::pg12_specific::NodeTag_T_AggState as isize,
    WindowAggState = pg_sys::pg12_specific::NodeTag_T_WindowAggState as isize,
    UniqueState = pg_sys::pg12_specific::NodeTag_T_UniqueState as isize,
    GatherState = pg_sys::pg12_specific::NodeTag_T_GatherState as isize,
    GatherMergeState = pg_sys::pg12_specific::NodeTag_T_GatherMergeState as isize,
    HashState = pg_sys::pg12_specific::NodeTag_T_HashState as isize,
    SetOpState = pg_sys::pg12_specific::NodeTag_T_SetOpState as isize,
    LockRowsState = pg_sys::pg12_specific::NodeTag_T_LockRowsState as isize,
    LimitState = pg_sys::pg12_specific::NodeTag_T_LimitState as isize,

    /*
     * TAGS FOR PRIMITIVE NODES (primnodes.h)
     */
    Alias = pg_sys::pg12_specific::NodeTag_T_Alias as isize,
    RangeVar = pg_sys::pg12_specific::NodeTag_T_RangeVar as isize,
    TableFunc = pg_sys::pg12_specific::NodeTag_T_TableFunc as isize,
    Expr = pg_sys::pg12_specific::NodeTag_T_Expr as isize,
    Var = pg_sys::pg12_specific::NodeTag_T_Var as isize,
    Const = pg_sys::pg12_specific::NodeTag_T_Const as isize,
    Param = pg_sys::pg12_specific::NodeTag_T_Param as isize,
    Aggref = pg_sys::pg12_specific::NodeTag_T_Aggref as isize,
    GroupingFunc = pg_sys::pg12_specific::NodeTag_T_GroupingFunc as isize,
    WindowFunc = pg_sys::pg12_specific::NodeTag_T_WindowFunc as isize,
    SubscriptingRef = pg_sys::pg12_specific::NodeTag_T_SubscriptingRef as isize,
    FuncExpr = pg_sys::pg12_specific::NodeTag_T_FuncExpr as isize,
    NamedArgExpr = pg_sys::pg12_specific::NodeTag_T_NamedArgExpr as isize,
    OpExpr = pg_sys::pg12_specific::NodeTag_T_OpExpr as isize,
    DistinctExpr = pg_sys::pg12_specific::NodeTag_T_DistinctExpr as isize,
    NullIfExpr = pg_sys::pg12_specific::NodeTag_T_NullIfExpr as isize,
    ScalarArrayOpExpr = pg_sys::pg12_specific::NodeTag_T_ScalarArrayOpExpr as isize,
    BoolExpr = pg_sys::pg12_specific::NodeTag_T_BoolExpr as isize,
    SubLink = pg_sys::pg12_specific::NodeTag_T_SubLink as isize,
    SubPlan = pg_sys::pg12_specific::NodeTag_T_SubPlan as isize,
    AlternativeSubPlan = pg_sys::pg12_specific::NodeTag_T_AlternativeSubPlan as isize,
    FieldSelect = pg_sys::pg12_specific::NodeTag_T_FieldSelect as isize,
    FieldStore = pg_sys::pg12_specific::NodeTag_T_FieldStore as isize,
    RelabelType = pg_sys::pg12_specific::NodeTag_T_RelabelType as isize,
    CoerceViaIO = pg_sys::pg12_specific::NodeTag_T_CoerceViaIO as isize,
    ArrayCoerceExpr = pg_sys::pg12_specific::NodeTag_T_ArrayCoerceExpr as isize,
    ConvertRowtypeExpr = pg_sys::pg12_specific::NodeTag_T_ConvertRowtypeExpr as isize,
    CollateExpr = pg_sys::pg12_specific::NodeTag_T_CollateExpr as isize,
    CaseExpr = pg_sys::pg12_specific::NodeTag_T_CaseExpr as isize,
    CaseWhen = pg_sys::pg12_specific::NodeTag_T_CaseWhen as isize,
    CaseTestExpr = pg_sys::pg12_specific::NodeTag_T_CaseTestExpr as isize,
    ArrayExpr = pg_sys::pg12_specific::NodeTag_T_ArrayExpr as isize,
    RowExpr = pg_sys::pg12_specific::NodeTag_T_RowExpr as isize,
    RowCompareExpr = pg_sys::pg12_specific::NodeTag_T_RowCompareExpr as isize,
    CoalesceExpr = pg_sys::pg12_specific::NodeTag_T_CoalesceExpr as isize,
    MinMaxExpr = pg_sys::pg12_specific::NodeTag_T_MinMaxExpr as isize,
    SQLValueFunction = pg_sys::pg12_specific::NodeTag_T_SQLValueFunction as isize,
    XmlExpr = pg_sys::pg12_specific::NodeTag_T_XmlExpr as isize,
    NullTest = pg_sys::pg12_specific::NodeTag_T_NullTest as isize,
    BooleanTest = pg_sys::pg12_specific::NodeTag_T_BooleanTest as isize,
    CoerceToDomain = pg_sys::pg12_specific::NodeTag_T_CoerceToDomain as isize,
    CoerceToDomainValue = pg_sys::pg12_specific::NodeTag_T_CoerceToDomainValue as isize,
    SetToDefault = pg_sys::pg12_specific::NodeTag_T_SetToDefault as isize,
    CurrentOfExpr = pg_sys::pg12_specific::NodeTag_T_CurrentOfExpr as isize,
    NextValueExpr = pg_sys::pg12_specific::NodeTag_T_NextValueExpr as isize,
    InferenceElem = pg_sys::pg12_specific::NodeTag_T_InferenceElem as isize,
    TargetEntry = pg_sys::pg12_specific::NodeTag_T_TargetEntry as isize,
    RangeTblRef = pg_sys::pg12_specific::NodeTag_T_RangeTblRef as isize,
    JoinExpr = pg_sys::pg12_specific::NodeTag_T_JoinExpr as isize,
    FromExpr = pg_sys::pg12_specific::NodeTag_T_FromExpr as isize,
    OnConflictExpr = pg_sys::pg12_specific::NodeTag_T_OnConflictExpr as isize,
    IntoClause = pg_sys::pg12_specific::NodeTag_T_IntoClause as isize,

    /*
     * TAGS FOR EXPRESSION STATE NODES (execnodes.h)
     *
     * ExprState represents the evaluation state for a whole expression tree.
     * Most Expr-based plan nodes do not have a corresponding expression state
     * node, they're fully handled within execExpr* - but sometimes the state
     * needs to be shared with other parts of the executor, as for example
     * with AggrefExprState, which nodeAgg.c has to modify.
     */
    ExprState = pg_sys::pg12_specific::NodeTag_T_ExprState as isize,
    AggrefExprState = pg_sys::pg12_specific::NodeTag_T_AggrefExprState as isize,
    WindowFuncExprState = pg_sys::pg12_specific::NodeTag_T_WindowFuncExprState as isize,
    SetExprState = pg_sys::pg12_specific::NodeTag_T_SetExprState as isize,
    SubPlanState = pg_sys::pg12_specific::NodeTag_T_SubPlanState as isize,
    AlternativeSubPlanState = pg_sys::pg12_specific::NodeTag_T_AlternativeSubPlanState as isize,
    DomainConstraintState = pg_sys::pg12_specific::NodeTag_T_DomainConstraintState as isize,

    /*
     * TAGS FOR PLANNER NODES (pathnodes.h)
     */
    PlannerInfo = pg_sys::pg12_specific::NodeTag_T_PlannerInfo as isize,
    PlannerGlobal = pg_sys::pg12_specific::NodeTag_T_PlannerGlobal as isize,
    RelOptInfo = pg_sys::pg12_specific::NodeTag_T_RelOptInfo as isize,
    IndexOptInfo = pg_sys::pg12_specific::NodeTag_T_IndexOptInfo as isize,
    ForeignKeyOptInfo = pg_sys::pg12_specific::NodeTag_T_ForeignKeyOptInfo as isize,
    ParamPathInfo = pg_sys::pg12_specific::NodeTag_T_ParamPathInfo as isize,
    Path = pg_sys::pg12_specific::NodeTag_T_Path as isize,
    IndexPath = pg_sys::pg12_specific::NodeTag_T_IndexPath as isize,
    BitmapHeapPath = pg_sys::pg12_specific::NodeTag_T_BitmapHeapPath as isize,
    BitmapAndPath = pg_sys::pg12_specific::NodeTag_T_BitmapAndPath as isize,
    BitmapOrPath = pg_sys::pg12_specific::NodeTag_T_BitmapOrPath as isize,
    TidPath = pg_sys::pg12_specific::NodeTag_T_TidPath as isize,
    SubqueryScanPath = pg_sys::pg12_specific::NodeTag_T_SubqueryScanPath as isize,
    ForeignPath = pg_sys::pg12_specific::NodeTag_T_ForeignPath as isize,
    CustomPath = pg_sys::pg12_specific::NodeTag_T_CustomPath as isize,
    NestPath = pg_sys::pg12_specific::NodeTag_T_NestPath as isize,
    MergePath = pg_sys::pg12_specific::NodeTag_T_MergePath as isize,
    HashPath = pg_sys::pg12_specific::NodeTag_T_HashPath as isize,
    AppendPath = pg_sys::pg12_specific::NodeTag_T_AppendPath as isize,
    MergeAppendPath = pg_sys::pg12_specific::NodeTag_T_MergeAppendPath as isize,
    GroupResultPath = pg_sys::pg12_specific::NodeTag_T_GroupResultPath as isize,
    MaterialPath = pg_sys::pg12_specific::NodeTag_T_MaterialPath as isize,
    UniquePath = pg_sys::pg12_specific::NodeTag_T_UniquePath as isize,
    GatherPath = pg_sys::pg12_specific::NodeTag_T_GatherPath as isize,
    GatherMergePath = pg_sys::pg12_specific::NodeTag_T_GatherMergePath as isize,
    ProjectionPath = pg_sys::pg12_specific::NodeTag_T_ProjectionPath as isize,
    ProjectSetPath = pg_sys::pg12_specific::NodeTag_T_ProjectSetPath as isize,
    SortPath = pg_sys::pg12_specific::NodeTag_T_SortPath as isize,
    GroupPath = pg_sys::pg12_specific::NodeTag_T_GroupPath as isize,
    UpperUniquePath = pg_sys::pg12_specific::NodeTag_T_UpperUniquePath as isize,
    AggPath = pg_sys::pg12_specific::NodeTag_T_AggPath as isize,
    GroupingSetsPath = pg_sys::pg12_specific::NodeTag_T_GroupingSetsPath as isize,
    MinMaxAggPath = pg_sys::pg12_specific::NodeTag_T_MinMaxAggPath as isize,
    WindowAggPath = pg_sys::pg12_specific::NodeTag_T_WindowAggPath as isize,
    SetOpPath = pg_sys::pg12_specific::NodeTag_T_SetOpPath as isize,
    RecursiveUnionPath = pg_sys::pg12_specific::NodeTag_T_RecursiveUnionPath as isize,
    LockRowsPath = pg_sys::pg12_specific::NodeTag_T_LockRowsPath as isize,
    ModifyTablePath = pg_sys::pg12_specific::NodeTag_T_ModifyTablePath as isize,
    LimitPath = pg_sys::pg12_specific::NodeTag_T_LimitPath as isize,
    /* these aren't subclasses of Path: */
    EquivalenceClass = pg_sys::pg12_specific::NodeTag_T_EquivalenceClass as isize,
    EquivalenceMember = pg_sys::pg12_specific::NodeTag_T_EquivalenceMember as isize,
    PathKey = pg_sys::pg12_specific::NodeTag_T_PathKey as isize,
    PathTarget = pg_sys::pg12_specific::NodeTag_T_PathTarget as isize,
    RestrictInfo = pg_sys::pg12_specific::NodeTag_T_RestrictInfo as isize,
    IndexClause = pg_sys::pg12_specific::NodeTag_T_IndexClause as isize,
    PlaceHolderVar = pg_sys::pg12_specific::NodeTag_T_PlaceHolderVar as isize,
    SpecialJoinInfo = pg_sys::pg12_specific::NodeTag_T_SpecialJoinInfo as isize,
    AppendRelInfo = pg_sys::pg12_specific::NodeTag_T_AppendRelInfo as isize,
    PlaceHolderInfo = pg_sys::pg12_specific::NodeTag_T_PlaceHolderInfo as isize,
    MinMaxAggInfo = pg_sys::pg12_specific::NodeTag_T_MinMaxAggInfo as isize,
    PlannerParamItem = pg_sys::pg12_specific::NodeTag_T_PlannerParamItem as isize,
    RollupData = pg_sys::pg12_specific::NodeTag_T_RollupData as isize,
    GroupingSetData = pg_sys::pg12_specific::NodeTag_T_GroupingSetData as isize,
    StatisticExtInfo = pg_sys::pg12_specific::NodeTag_T_StatisticExtInfo as isize,

    /*
     * TAGS FOR MEMORY NODES (memnodes.h)
     */
    MemoryContext = pg_sys::pg12_specific::NodeTag_T_MemoryContext as isize,
    AllocSetContext = pg_sys::pg12_specific::NodeTag_T_AllocSetContext as isize,
    SlabContext = pg_sys::pg12_specific::NodeTag_T_SlabContext as isize,
    GenerationContext = pg_sys::pg12_specific::NodeTag_T_GenerationContext as isize,

    /*
     * TAGS FOR VALUE NODES (value.h)
     */
    Value = pg_sys::pg12_specific::NodeTag_T_Value as isize,
    Integer = pg_sys::pg12_specific::NodeTag_T_Integer as isize,
    Float = pg_sys::pg12_specific::NodeTag_T_Float as isize,
    String = pg_sys::pg12_specific::NodeTag_T_String as isize,
    BitString = pg_sys::pg12_specific::NodeTag_T_BitString as isize,
    Null = pg_sys::pg12_specific::NodeTag_T_Null as isize,

    /*
     * TAGS FOR LIST NODES (pg_list.h)
     */
    List = pg_sys::pg12_specific::NodeTag_T_List as isize,
    IntList = pg_sys::pg12_specific::NodeTag_T_IntList as isize,
    OidList = pg_sys::pg12_specific::NodeTag_T_OidList as isize,

    /*
     * TAGS FOR EXTENSIBLE NODES (extensible.h)
     */
    ExtensibleNode = pg_sys::pg12_specific::NodeTag_T_ExtensibleNode as isize,

    /*
     * TAGS FOR STATEMENT NODES (mostly in parsenodes.h)
     */
    RawStmt = pg_sys::pg12_specific::NodeTag_T_RawStmt as isize,
    Query = pg_sys::pg12_specific::NodeTag_T_Query as isize,
    PlannedStmt = pg_sys::pg12_specific::NodeTag_T_PlannedStmt as isize,
    InsertStmt = pg_sys::pg12_specific::NodeTag_T_InsertStmt as isize,
    DeleteStmt = pg_sys::pg12_specific::NodeTag_T_DeleteStmt as isize,
    UpdateStmt = pg_sys::pg12_specific::NodeTag_T_UpdateStmt as isize,
    SelectStmt = pg_sys::pg12_specific::NodeTag_T_SelectStmt as isize,
    AlterTableStmt = pg_sys::pg12_specific::NodeTag_T_AlterTableStmt as isize,
    AlterTableCmd = pg_sys::pg12_specific::NodeTag_T_AlterTableCmd as isize,
    AlterDomainStmt = pg_sys::pg12_specific::NodeTag_T_AlterDomainStmt as isize,
    SetOperationStmt = pg_sys::pg12_specific::NodeTag_T_SetOperationStmt as isize,
    GrantStmt = pg_sys::pg12_specific::NodeTag_T_GrantStmt as isize,
    GrantRoleStmt = pg_sys::pg12_specific::NodeTag_T_GrantRoleStmt as isize,
    AlterDefaultPrivilegesStmt =
        pg_sys::pg12_specific::NodeTag_T_AlterDefaultPrivilegesStmt as isize,
    ClosePortalStmt = pg_sys::pg12_specific::NodeTag_T_ClosePortalStmt as isize,
    ClusterStmt = pg_sys::pg12_specific::NodeTag_T_ClusterStmt as isize,
    CopyStmt = pg_sys::pg12_specific::NodeTag_T_CopyStmt as isize,
    CreateStmt = pg_sys::pg12_specific::NodeTag_T_CreateStmt as isize,
    DefineStmt = pg_sys::pg12_specific::NodeTag_T_DefineStmt as isize,
    DropStmt = pg_sys::pg12_specific::NodeTag_T_DropStmt as isize,
    TruncateStmt = pg_sys::pg12_specific::NodeTag_T_TruncateStmt as isize,
    CommentStmt = pg_sys::pg12_specific::NodeTag_T_CommentStmt as isize,
    FetchStmt = pg_sys::pg12_specific::NodeTag_T_FetchStmt as isize,
    IndexStmt = pg_sys::pg12_specific::NodeTag_T_IndexStmt as isize,
    CreateFunctionStmt = pg_sys::pg12_specific::NodeTag_T_CreateFunctionStmt as isize,
    AlterFunctionStmt = pg_sys::pg12_specific::NodeTag_T_AlterFunctionStmt as isize,
    DoStmt = pg_sys::pg12_specific::NodeTag_T_DoStmt as isize,
    RenameStmt = pg_sys::pg12_specific::NodeTag_T_RenameStmt as isize,
    RuleStmt = pg_sys::pg12_specific::NodeTag_T_RuleStmt as isize,
    NotifyStmt = pg_sys::pg12_specific::NodeTag_T_NotifyStmt as isize,
    ListenStmt = pg_sys::pg12_specific::NodeTag_T_ListenStmt as isize,
    UnlistenStmt = pg_sys::pg12_specific::NodeTag_T_UnlistenStmt as isize,
    TransactionStmt = pg_sys::pg12_specific::NodeTag_T_TransactionStmt as isize,
    ViewStmt = pg_sys::pg12_specific::NodeTag_T_ViewStmt as isize,
    LoadStmt = pg_sys::pg12_specific::NodeTag_T_LoadStmt as isize,
    CreateDomainStmt = pg_sys::pg12_specific::NodeTag_T_CreateDomainStmt as isize,
    CreatedbStmt = pg_sys::pg12_specific::NodeTag_T_CreatedbStmt as isize,
    DropdbStmt = pg_sys::pg12_specific::NodeTag_T_DropdbStmt as isize,
    VacuumStmt = pg_sys::pg12_specific::NodeTag_T_VacuumStmt as isize,
    ExplainStmt = pg_sys::pg12_specific::NodeTag_T_ExplainStmt as isize,
    CreateTableAsStmt = pg_sys::pg12_specific::NodeTag_T_CreateTableAsStmt as isize,
    CreateSeqStmt = pg_sys::pg12_specific::NodeTag_T_CreateSeqStmt as isize,
    AlterSeqStmt = pg_sys::pg12_specific::NodeTag_T_AlterSeqStmt as isize,
    VariableSetStmt = pg_sys::pg12_specific::NodeTag_T_VariableSetStmt as isize,
    VariableShowStmt = pg_sys::pg12_specific::NodeTag_T_VariableShowStmt as isize,
    DiscardStmt = pg_sys::pg12_specific::NodeTag_T_DiscardStmt as isize,
    CreateTrigStmt = pg_sys::pg12_specific::NodeTag_T_CreateTrigStmt as isize,
    CreatePLangStmt = pg_sys::pg12_specific::NodeTag_T_CreatePLangStmt as isize,
    CreateRoleStmt = pg_sys::pg12_specific::NodeTag_T_CreateRoleStmt as isize,
    AlterRoleStmt = pg_sys::pg12_specific::NodeTag_T_AlterRoleStmt as isize,
    DropRoleStmt = pg_sys::pg12_specific::NodeTag_T_DropRoleStmt as isize,
    LockStmt = pg_sys::pg12_specific::NodeTag_T_LockStmt as isize,
    ConstraintsSetStmt = pg_sys::pg12_specific::NodeTag_T_ConstraintsSetStmt as isize,
    ReindexStmt = pg_sys::pg12_specific::NodeTag_T_ReindexStmt as isize,
    CheckPointStmt = pg_sys::pg12_specific::NodeTag_T_CheckPointStmt as isize,
    CreateSchemaStmt = pg_sys::pg12_specific::NodeTag_T_CreateSchemaStmt as isize,
    AlterDatabaseStmt = pg_sys::pg12_specific::NodeTag_T_AlterDatabaseStmt as isize,
    AlterDatabaseSetStmt = pg_sys::pg12_specific::NodeTag_T_AlterDatabaseSetStmt as isize,
    AlterRoleSetStmt = pg_sys::pg12_specific::NodeTag_T_AlterRoleSetStmt as isize,
    CreateConversionStmt = pg_sys::pg12_specific::NodeTag_T_CreateConversionStmt as isize,
    CreateCastStmt = pg_sys::pg12_specific::NodeTag_T_CreateCastStmt as isize,
    CreateOpClassStmt = pg_sys::pg12_specific::NodeTag_T_CreateOpClassStmt as isize,
    CreateOpFamilyStmt = pg_sys::pg12_specific::NodeTag_T_CreateOpFamilyStmt as isize,
    AlterOpFamilyStmt = pg_sys::pg12_specific::NodeTag_T_AlterOpFamilyStmt as isize,
    PrepareStmt = pg_sys::pg12_specific::NodeTag_T_PrepareStmt as isize,
    ExecuteStmt = pg_sys::pg12_specific::NodeTag_T_ExecuteStmt as isize,
    DeallocateStmt = pg_sys::pg12_specific::NodeTag_T_DeallocateStmt as isize,
    DeclareCursorStmt = pg_sys::pg12_specific::NodeTag_T_DeclareCursorStmt as isize,
    CreateTableSpaceStmt = pg_sys::pg12_specific::NodeTag_T_CreateTableSpaceStmt as isize,
    DropTableSpaceStmt = pg_sys::pg12_specific::NodeTag_T_DropTableSpaceStmt as isize,
    AlterObjectDependsStmt = pg_sys::pg12_specific::NodeTag_T_AlterObjectDependsStmt as isize,
    AlterObjectSchemaStmt = pg_sys::pg12_specific::NodeTag_T_AlterObjectSchemaStmt as isize,
    AlterOwnerStmt = pg_sys::pg12_specific::NodeTag_T_AlterOwnerStmt as isize,
    AlterOperatorStmt = pg_sys::pg12_specific::NodeTag_T_AlterOperatorStmt as isize,
    DropOwnedStmt = pg_sys::pg12_specific::NodeTag_T_DropOwnedStmt as isize,
    ReassignOwnedStmt = pg_sys::pg12_specific::NodeTag_T_ReassignOwnedStmt as isize,
    CompositeTypeStmt = pg_sys::pg12_specific::NodeTag_T_CompositeTypeStmt as isize,
    CreateEnumStmt = pg_sys::pg12_specific::NodeTag_T_CreateEnumStmt as isize,
    CreateRangeStmt = pg_sys::pg12_specific::NodeTag_T_CreateRangeStmt as isize,
    AlterEnumStmt = pg_sys::pg12_specific::NodeTag_T_AlterEnumStmt as isize,
    AlterTSDictionaryStmt = pg_sys::pg12_specific::NodeTag_T_AlterTSDictionaryStmt as isize,
    AlterTSConfigurationStmt = pg_sys::pg12_specific::NodeTag_T_AlterTSConfigurationStmt as isize,
    CreateFdwStmt = pg_sys::pg12_specific::NodeTag_T_CreateFdwStmt as isize,
    AlterFdwStmt = pg_sys::pg12_specific::NodeTag_T_AlterFdwStmt as isize,
    CreateForeignServerStmt = pg_sys::pg12_specific::NodeTag_T_CreateForeignServerStmt as isize,
    AlterForeignServerStmt = pg_sys::pg12_specific::NodeTag_T_AlterForeignServerStmt as isize,
    CreateUserMappingStmt = pg_sys::pg12_specific::NodeTag_T_CreateUserMappingStmt as isize,
    AlterUserMappingStmt = pg_sys::pg12_specific::NodeTag_T_AlterUserMappingStmt as isize,
    DropUserMappingStmt = pg_sys::pg12_specific::NodeTag_T_DropUserMappingStmt as isize,
    AlterTableSpaceOptionsStmt =
        pg_sys::pg12_specific::NodeTag_T_AlterTableSpaceOptionsStmt as isize,
    AlterTableMoveAllStmt = pg_sys::pg12_specific::NodeTag_T_AlterTableMoveAllStmt as isize,
    SecLabelStmt = pg_sys::pg12_specific::NodeTag_T_SecLabelStmt as isize,
    CreateForeignTableStmt = pg_sys::pg12_specific::NodeTag_T_CreateForeignTableStmt as isize,
    ImportForeignSchemaStmt = pg_sys::pg12_specific::NodeTag_T_ImportForeignSchemaStmt as isize,
    CreateExtensionStmt = pg_sys::pg12_specific::NodeTag_T_CreateExtensionStmt as isize,
    AlterExtensionStmt = pg_sys::pg12_specific::NodeTag_T_AlterExtensionStmt as isize,
    AlterExtensionContentsStmt =
        pg_sys::pg12_specific::NodeTag_T_AlterExtensionContentsStmt as isize,
    CreateEventTrigStmt = pg_sys::pg12_specific::NodeTag_T_CreateEventTrigStmt as isize,
    AlterEventTrigStmt = pg_sys::pg12_specific::NodeTag_T_AlterEventTrigStmt as isize,
    RefreshMatViewStmt = pg_sys::pg12_specific::NodeTag_T_RefreshMatViewStmt as isize,
    ReplicaIdentityStmt = pg_sys::pg12_specific::NodeTag_T_ReplicaIdentityStmt as isize,
    AlterSystemStmt = pg_sys::pg12_specific::NodeTag_T_AlterSystemStmt as isize,
    CreatePolicyStmt = pg_sys::pg12_specific::NodeTag_T_CreatePolicyStmt as isize,
    AlterPolicyStmt = pg_sys::pg12_specific::NodeTag_T_AlterPolicyStmt as isize,
    CreateTransformStmt = pg_sys::pg12_specific::NodeTag_T_CreateTransformStmt as isize,
    CreateAmStmt = pg_sys::pg12_specific::NodeTag_T_CreateAmStmt as isize,
    CreatePublicationStmt = pg_sys::pg12_specific::NodeTag_T_CreatePublicationStmt as isize,
    AlterPublicationStmt = pg_sys::pg12_specific::NodeTag_T_AlterPublicationStmt as isize,
    CreateSubscriptionStmt = pg_sys::pg12_specific::NodeTag_T_CreateSubscriptionStmt as isize,
    AlterSubscriptionStmt = pg_sys::pg12_specific::NodeTag_T_AlterSubscriptionStmt as isize,
    DropSubscriptionStmt = pg_sys::pg12_specific::NodeTag_T_DropSubscriptionStmt as isize,
    CreateStatsStmt = pg_sys::pg12_specific::NodeTag_T_CreateStatsStmt as isize,
    AlterCollationStmt = pg_sys::pg12_specific::NodeTag_T_AlterCollationStmt as isize,
    CallStmt = pg_sys::pg12_specific::NodeTag_T_CallStmt as isize,

    /*
     * TAGS FOR PARSE TREE NODES (parsenodes.h)
     */
    A_Expr = pg_sys::pg12_specific::NodeTag_T_A_Expr as isize,
    ColumnRef = pg_sys::pg12_specific::NodeTag_T_ColumnRef as isize,
    ParamRef = pg_sys::pg12_specific::NodeTag_T_ParamRef as isize,
    A_Const = pg_sys::pg12_specific::NodeTag_T_A_Const as isize,
    FuncCall = pg_sys::pg12_specific::NodeTag_T_FuncCall as isize,
    A_Star = pg_sys::pg12_specific::NodeTag_T_A_Star as isize,
    A_Indices = pg_sys::pg12_specific::NodeTag_T_A_Indices as isize,
    A_Indirection = pg_sys::pg12_specific::NodeTag_T_A_Indirection as isize,
    A_ArrayExpr = pg_sys::pg12_specific::NodeTag_T_A_ArrayExpr as isize,
    ResTarget = pg_sys::pg12_specific::NodeTag_T_ResTarget as isize,
    MultiAssignRef = pg_sys::pg12_specific::NodeTag_T_MultiAssignRef as isize,
    TypeCast = pg_sys::pg12_specific::NodeTag_T_TypeCast as isize,
    CollateClause = pg_sys::pg12_specific::NodeTag_T_CollateClause as isize,
    SortBy = pg_sys::pg12_specific::NodeTag_T_SortBy as isize,
    WindowDef = pg_sys::pg12_specific::NodeTag_T_WindowDef as isize,
    RangeSubselect = pg_sys::pg12_specific::NodeTag_T_RangeSubselect as isize,
    RangeFunction = pg_sys::pg12_specific::NodeTag_T_RangeFunction as isize,
    RangeTableSample = pg_sys::pg12_specific::NodeTag_T_RangeTableSample as isize,
    RangeTableFunc = pg_sys::pg12_specific::NodeTag_T_RangeTableFunc as isize,
    RangeTableFuncCol = pg_sys::pg12_specific::NodeTag_T_RangeTableFuncCol as isize,
    TypeName = pg_sys::pg12_specific::NodeTag_T_TypeName as isize,
    ColumnDef = pg_sys::pg12_specific::NodeTag_T_ColumnDef as isize,
    IndexElem = pg_sys::pg12_specific::NodeTag_T_IndexElem as isize,
    Constraint = pg_sys::pg12_specific::NodeTag_T_Constraint as isize,
    DefElem = pg_sys::pg12_specific::NodeTag_T_DefElem as isize,
    RangeTblEntry = pg_sys::pg12_specific::NodeTag_T_RangeTblEntry as isize,
    RangeTblFunction = pg_sys::pg12_specific::NodeTag_T_RangeTblFunction as isize,
    TableSampleClause = pg_sys::pg12_specific::NodeTag_T_TableSampleClause as isize,
    WithCheckOption = pg_sys::pg12_specific::NodeTag_T_WithCheckOption as isize,
    SortGroupClause = pg_sys::pg12_specific::NodeTag_T_SortGroupClause as isize,
    GroupingSet = pg_sys::pg12_specific::NodeTag_T_GroupingSet as isize,
    WindowClause = pg_sys::pg12_specific::NodeTag_T_WindowClause as isize,
    ObjectWithArgs = pg_sys::pg12_specific::NodeTag_T_ObjectWithArgs as isize,
    AccessPriv = pg_sys::pg12_specific::NodeTag_T_AccessPriv as isize,
    CreateOpClassItem = pg_sys::pg12_specific::NodeTag_T_CreateOpClassItem as isize,
    TableLikeClause = pg_sys::pg12_specific::NodeTag_T_TableLikeClause as isize,
    FunctionParameter = pg_sys::pg12_specific::NodeTag_T_FunctionParameter as isize,
    LockingClause = pg_sys::pg12_specific::NodeTag_T_LockingClause as isize,
    RowMarkClause = pg_sys::pg12_specific::NodeTag_T_RowMarkClause as isize,
    XmlSerialize = pg_sys::pg12_specific::NodeTag_T_XmlSerialize as isize,
    WithClause = pg_sys::pg12_specific::NodeTag_T_WithClause as isize,
    InferClause = pg_sys::pg12_specific::NodeTag_T_InferClause as isize,
    OnConflictClause = pg_sys::pg12_specific::NodeTag_T_OnConflictClause as isize,
    CommonTableExpr = pg_sys::pg12_specific::NodeTag_T_CommonTableExpr as isize,
    RoleSpec = pg_sys::pg12_specific::NodeTag_T_RoleSpec as isize,
    TriggerTransition = pg_sys::pg12_specific::NodeTag_T_TriggerTransition as isize,
    PartitionElem = pg_sys::pg12_specific::NodeTag_T_PartitionElem as isize,
    PartitionSpec = pg_sys::pg12_specific::NodeTag_T_PartitionSpec as isize,
    PartitionBoundSpec = pg_sys::pg12_specific::NodeTag_T_PartitionBoundSpec as isize,
    PartitionRangeDatum = pg_sys::pg12_specific::NodeTag_T_PartitionRangeDatum as isize,
    PartitionCmd = pg_sys::pg12_specific::NodeTag_T_PartitionCmd as isize,
    VacuumRelation = pg_sys::pg12_specific::NodeTag_T_VacuumRelation as isize,

    /*
     * TAGS FOR REPLICATION GRAMMAR PARSE NODES (replnodes.h)
     */
    IdentifySystemCmd = pg_sys::pg12_specific::NodeTag_T_IdentifySystemCmd as isize,
    BaseBackupCmd = pg_sys::pg12_specific::NodeTag_T_BaseBackupCmd as isize,
    CreateReplicationSlotCmd = pg_sys::pg12_specific::NodeTag_T_CreateReplicationSlotCmd as isize,
    DropReplicationSlotCmd = pg_sys::pg12_specific::NodeTag_T_DropReplicationSlotCmd as isize,
    StartReplicationCmd = pg_sys::pg12_specific::NodeTag_T_StartReplicationCmd as isize,
    TimeLineHistoryCmd = pg_sys::pg12_specific::NodeTag_T_TimeLineHistoryCmd as isize,
    SQLCmd = pg_sys::pg12_specific::NodeTag_T_SQLCmd as isize,

    /*
     * TAGS FOR RANDOM OTHER STUFF
     *
     * These are objects that aren't part of parse/plan/execute node tree
     * structures, but we give them NodeTags anyway for identification
     * purposes (usually because they are involved in APIs where we want to
     * pass multiple object types through the same pointer).
     */
    TriggerData = pg_sys::pg12_specific::NodeTag_T_TriggerData as isize,
    /* in commands/trigger.h */
    EventTriggerData = pg_sys::pg12_specific::NodeTag_T_EventTriggerData as isize,
    /* in commands/event_trigger.h */
    ReturnSetInfo = pg_sys::pg12_specific::NodeTag_T_ReturnSetInfo as isize,
    /* in nodes/execnodes.h */
    WindowObjectData = pg_sys::pg12_specific::NodeTag_T_WindowObjectData as isize,
    /* private in nodeWindowAgg.c */
    TIDBitmap = pg_sys::pg12_specific::NodeTag_T_TIDBitmap as isize,
    /* in nodes/tidbitmap.h */
    InlineCodeBlock = pg_sys::pg12_specific::NodeTag_T_InlineCodeBlock as isize,
    /* in nodes/parsenodes.h */
    FdwRoutine = pg_sys::pg12_specific::NodeTag_T_FdwRoutine as isize,
    /* in foreign/fdwapi.h */
    IndexAmRoutine = pg_sys::pg12_specific::NodeTag_T_IndexAmRoutine as isize,
    /* in access/amapi.h */
    TableAmRoutine = pg_sys::pg12_specific::NodeTag_T_TableAmRoutine as isize,
    /* in access/tableam.h */
    TsmRoutine = pg_sys::pg12_specific::NodeTag_T_TsmRoutine as isize,
    /* in access/tsmapi.h */
    ForeignKeyCacheInfo = pg_sys::pg12_specific::NodeTag_T_ForeignKeyCacheInfo as isize,
    /* in utils/rel.h */
    CallContext = pg_sys::pg12_specific::NodeTag_T_CallContext as isize,
    /* in nodes/parsenodes.h */
    SupportRequestSimplify = pg_sys::pg12_specific::NodeTag_T_SupportRequestSimplify as isize,
    /* in nodes/supportnodes.h */
    SupportRequestSelectivity = pg_sys::pg12_specific::NodeTag_T_SupportRequestSelectivity as isize,
    /* in nodes/supportnodes.h */
    SupportRequestCost = pg_sys::pg12_specific::NodeTag_T_SupportRequestCost as isize,
    /* in nodes/supportnodes.h */
    SupportRequestRows = pg_sys::pg12_specific::NodeTag_T_SupportRequestRows as isize,
    /* in nodes/supportnodes.h */
    SupportRequestIndexCondition =
        pg_sys::pg12_specific::NodeTag_T_SupportRequestIndexCondition as isize, /* in nodes/supportnodes.h */
}

pub struct PgNodeFactory();

#[allow(non_snake_case)]
impl PgNodeFactory {
    pub fn makeIndexInfo() -> PgBox<pg_sys::pg12_specific::IndexInfo> {
        PgBox::<pg_sys::pg12_specific::IndexInfo>::alloc_node(PgNode::IndexInfo)
    }
    pub fn makeExprContext() -> PgBox<pg_sys::ExprContext> {
        PgBox::<pg_sys::ExprContext>::alloc_node(PgNode::ExprContext)
    }
    pub fn makeProjectionInfo() -> PgBox<pg_sys::ProjectionInfo> {
        PgBox::<pg_sys::ProjectionInfo>::alloc_node(PgNode::ProjectionInfo)
    }
    pub fn makeJunkFilter() -> PgBox<pg_sys::JunkFilter> {
        PgBox::<pg_sys::JunkFilter>::alloc_node(PgNode::JunkFilter)
    }
    pub fn makeOnConflictSetState() -> PgBox<pg_sys::pg12_specific::OnConflictSetState> {
        PgBox::<pg_sys::pg12_specific::OnConflictSetState>::alloc_node(PgNode::OnConflictSetState)
    }
    pub fn makeResultRelInfo() -> PgBox<pg_sys::pg12_specific::ResultRelInfo> {
        PgBox::<pg_sys::pg12_specific::ResultRelInfo>::alloc_node(PgNode::ResultRelInfo)
    }
    pub fn makeEState() -> PgBox<pg_sys::pg12_specific::EState> {
        PgBox::<pg_sys::pg12_specific::EState>::alloc_node(PgNode::EState)
    }
    pub fn makeTupleTableSlot() -> PgBox<pg_sys::pg12_specific::TupleTableSlot> {
        PgBox::<pg_sys::pg12_specific::TupleTableSlot>::alloc_node(PgNode::TupleTableSlot)
    }

    pub fn makePlan() -> PgBox<pg_sys::Plan> {
        PgBox::<pg_sys::Plan>::alloc_node(PgNode::Plan)
    }
    pub fn makeResult() -> PgBox<pg_sys::Result> {
        PgBox::<pg_sys::Result>::alloc_node(PgNode::Result)
    }
    pub fn makeProjectSet() -> PgBox<pg_sys::ProjectSet> {
        PgBox::<pg_sys::ProjectSet>::alloc_node(PgNode::ProjectSet)
    }
    pub fn makeModifyTable() -> PgBox<pg_sys::pg12_specific::ModifyTable> {
        PgBox::<pg_sys::pg12_specific::ModifyTable>::alloc_node(PgNode::ModifyTable)
    }
    pub fn makeAppend() -> PgBox<pg_sys::pg12_specific::Append> {
        PgBox::<pg_sys::pg12_specific::Append>::alloc_node(PgNode::Append)
    }
    pub fn makeMergeAppend() -> PgBox<pg_sys::pg12_specific::MergeAppend> {
        PgBox::<pg_sys::pg12_specific::MergeAppend>::alloc_node(PgNode::MergeAppend)
    }
    pub fn makeRecursiveUnion() -> PgBox<pg_sys::pg12_specific::RecursiveUnion> {
        PgBox::<pg_sys::pg12_specific::RecursiveUnion>::alloc_node(PgNode::RecursiveUnion)
    }
    pub fn makeBitmapAnd() -> PgBox<pg_sys::BitmapAnd> {
        PgBox::<pg_sys::BitmapAnd>::alloc_node(PgNode::BitmapAnd)
    }
    pub fn makeBitmapOr() -> PgBox<pg_sys::BitmapOr> {
        PgBox::<pg_sys::BitmapOr>::alloc_node(PgNode::BitmapOr)
    }
    pub fn makeScan() -> PgBox<pg_sys::Scan> {
        PgBox::<pg_sys::Scan>::alloc_node(PgNode::Scan)
    }
    pub fn makeSeqScan() -> PgBox<pg_sys::SeqScan> {
        PgBox::<pg_sys::SeqScan>::alloc_node(PgNode::SeqScan)
    }
    pub fn makeSampleScan() -> PgBox<pg_sys::SampleScan> {
        PgBox::<pg_sys::SampleScan>::alloc_node(PgNode::SampleScan)
    }
    pub fn makeIndexScan() -> PgBox<pg_sys::IndexScan> {
        PgBox::<pg_sys::IndexScan>::alloc_node(PgNode::IndexScan)
    }
    pub fn makeIndexOnlyScan() -> PgBox<pg_sys::IndexOnlyScan> {
        PgBox::<pg_sys::IndexOnlyScan>::alloc_node(PgNode::IndexOnlyScan)
    }
    pub fn makeBitmapIndexScan() -> PgBox<pg_sys::BitmapIndexScan> {
        PgBox::<pg_sys::BitmapIndexScan>::alloc_node(PgNode::BitmapIndexScan)
    }
    pub fn makeBitmapHeapScan() -> PgBox<pg_sys::BitmapHeapScan> {
        PgBox::<pg_sys::BitmapHeapScan>::alloc_node(PgNode::BitmapHeapScan)
    }
    pub fn makeTidScan() -> PgBox<pg_sys::TidScan> {
        PgBox::<pg_sys::TidScan>::alloc_node(PgNode::TidScan)
    }
    pub fn makeSubqueryScan() -> PgBox<pg_sys::SubqueryScan> {
        PgBox::<pg_sys::SubqueryScan>::alloc_node(PgNode::SubqueryScan)
    }
    pub fn makeFunctionScan() -> PgBox<pg_sys::FunctionScan> {
        PgBox::<pg_sys::FunctionScan>::alloc_node(PgNode::FunctionScan)
    }
    pub fn makeValuesScan() -> PgBox<pg_sys::ValuesScan> {
        PgBox::<pg_sys::ValuesScan>::alloc_node(PgNode::ValuesScan)
    }
    pub fn makeTableFuncScan() -> PgBox<pg_sys::TableFuncScan> {
        PgBox::<pg_sys::TableFuncScan>::alloc_node(PgNode::TableFuncScan)
    }
    pub fn makeCteScan() -> PgBox<pg_sys::CteScan> {
        PgBox::<pg_sys::CteScan>::alloc_node(PgNode::CteScan)
    }
    pub fn makeNamedTuplestoreScan() -> PgBox<pg_sys::NamedTuplestoreScan> {
        PgBox::<pg_sys::NamedTuplestoreScan>::alloc_node(PgNode::NamedTuplestoreScan)
    }
    pub fn makeWorkTableScan() -> PgBox<pg_sys::WorkTableScan> {
        PgBox::<pg_sys::WorkTableScan>::alloc_node(PgNode::WorkTableScan)
    }
    pub fn makeForeignScan() -> PgBox<pg_sys::ForeignScan> {
        PgBox::<pg_sys::ForeignScan>::alloc_node(PgNode::ForeignScan)
    }
    pub fn makeCustomScan() -> PgBox<pg_sys::CustomScan> {
        PgBox::<pg_sys::CustomScan>::alloc_node(PgNode::CustomScan)
    }
    pub fn makeJoin() -> PgBox<pg_sys::Join> {
        PgBox::<pg_sys::Join>::alloc_node(PgNode::Join)
    }
    pub fn makeNestLoop() -> PgBox<pg_sys::NestLoop> {
        PgBox::<pg_sys::NestLoop>::alloc_node(PgNode::NestLoop)
    }
    pub fn makeMergeJoin() -> PgBox<pg_sys::MergeJoin> {
        PgBox::<pg_sys::MergeJoin>::alloc_node(PgNode::MergeJoin)
    }
    pub fn makeHashJoin() -> PgBox<pg_sys::pg12_specific::HashJoin> {
        PgBox::<pg_sys::pg12_specific::HashJoin>::alloc_node(PgNode::HashJoin)
    }
    pub fn makeMaterial() -> PgBox<pg_sys::Material> {
        PgBox::<pg_sys::Material>::alloc_node(PgNode::Material)
    }
    pub fn makeSort() -> PgBox<pg_sys::Sort> {
        PgBox::<pg_sys::Sort>::alloc_node(PgNode::Sort)
    }
    pub fn makeGroup() -> PgBox<pg_sys::pg12_specific::Group> {
        PgBox::<pg_sys::pg12_specific::Group>::alloc_node(PgNode::Group)
    }
    pub fn makeAgg() -> PgBox<pg_sys::pg12_specific::Agg> {
        PgBox::<pg_sys::pg12_specific::Agg>::alloc_node(PgNode::Agg)
    }
    pub fn makeWindowAgg() -> PgBox<pg_sys::pg12_specific::WindowAgg> {
        PgBox::<pg_sys::pg12_specific::WindowAgg>::alloc_node(PgNode::WindowAgg)
    }
    pub fn makeUnique() -> PgBox<pg_sys::pg12_specific::Unique> {
        PgBox::<pg_sys::pg12_specific::Unique>::alloc_node(PgNode::Unique)
    }
    pub fn makeGather() -> PgBox<pg_sys::pg12_specific::Gather> {
        PgBox::<pg_sys::pg12_specific::Gather>::alloc_node(PgNode::Gather)
    }
    pub fn makeGatherMerge() -> PgBox<pg_sys::pg12_specific::GatherMerge> {
        PgBox::<pg_sys::pg12_specific::GatherMerge>::alloc_node(PgNode::GatherMerge)
    }
    pub fn makeHash() -> PgBox<pg_sys::pg12_specific::Hash> {
        PgBox::<pg_sys::pg12_specific::Hash>::alloc_node(PgNode::Hash)
    }
    pub fn makeSetOp() -> PgBox<pg_sys::pg12_specific::SetOp> {
        PgBox::<pg_sys::pg12_specific::SetOp>::alloc_node(PgNode::SetOp)
    }
    pub fn makeLockRows() -> PgBox<pg_sys::LockRows> {
        PgBox::<pg_sys::LockRows>::alloc_node(PgNode::LockRows)
    }
    pub fn makeLimit() -> PgBox<pg_sys::Limit> {
        PgBox::<pg_sys::Limit>::alloc_node(PgNode::Limit)
    }

    pub fn makeNestLoopParam() -> PgBox<pg_sys::NestLoopParam> {
        PgBox::<pg_sys::NestLoopParam>::alloc_node(PgNode::NestLoopParam)
    }
    pub fn makePlanRowMark() -> PgBox<pg_sys::PlanRowMark> {
        PgBox::<pg_sys::PlanRowMark>::alloc_node(PgNode::PlanRowMark)
    }
    pub fn makePartitionPruneInfo() -> PgBox<pg_sys::pg12_specific::PartitionPruneInfo> {
        PgBox::<pg_sys::pg12_specific::PartitionPruneInfo>::alloc_node(PgNode::PartitionPruneInfo)
    }
    pub fn makePartitionedRelPruneInfo() -> PgBox<pg_sys::pg12_specific::PartitionedRelPruneInfo> {
        PgBox::<pg_sys::pg12_specific::PartitionedRelPruneInfo>::alloc_node(
            PgNode::PartitionedRelPruneInfo,
        )
    }
    pub fn makePartitionPruneStepOp() -> PgBox<pg_sys::pg12_specific::PartitionPruneStepOp> {
        PgBox::<pg_sys::pg12_specific::PartitionPruneStepOp>::alloc_node(
            PgNode::PartitionPruneStepOp,
        )
    }
    pub fn makePartitionPruneStepCombine() -> PgBox<pg_sys::pg12_specific::PartitionPruneStepCombine>
    {
        PgBox::<pg_sys::pg12_specific::PartitionPruneStepCombine>::alloc_node(
            PgNode::PartitionPruneStepCombine,
        )
    }
    pub fn makePlanInvalItem() -> PgBox<pg_sys::PlanInvalItem> {
        PgBox::<pg_sys::PlanInvalItem>::alloc_node(PgNode::PlanInvalItem)
    }

    pub fn makePlanState() -> PgBox<pg_sys::pg12_specific::PlanState> {
        PgBox::<pg_sys::pg12_specific::PlanState>::alloc_node(PgNode::PlanState)
    }
    pub fn makeResultState() -> PgBox<pg_sys::ResultState> {
        PgBox::<pg_sys::ResultState>::alloc_node(PgNode::ResultState)
    }
    pub fn makeProjectSetState() -> PgBox<pg_sys::pg12_specific::ProjectSetState> {
        PgBox::<pg_sys::pg12_specific::ProjectSetState>::alloc_node(PgNode::ProjectSetState)
    }
    pub fn makeModifyTableState() -> PgBox<pg_sys::pg12_specific::ModifyTableState> {
        PgBox::<pg_sys::pg12_specific::ModifyTableState>::alloc_node(PgNode::ModifyTableState)
    }
    pub fn makeAppendState() -> PgBox<pg_sys::pg12_specific::AppendState> {
        PgBox::<pg_sys::pg12_specific::AppendState>::alloc_node(PgNode::AppendState)
    }
    pub fn makeMergeAppendState() -> PgBox<pg_sys::pg12_specific::MergeAppendState> {
        PgBox::<pg_sys::pg12_specific::MergeAppendState>::alloc_node(PgNode::MergeAppendState)
    }
    pub fn makeRecursiveUnionState() -> PgBox<pg_sys::pg12_specific::RecursiveUnionState> {
        PgBox::<pg_sys::pg12_specific::RecursiveUnionState>::alloc_node(PgNode::RecursiveUnionState)
    }
    pub fn makeBitmapAndState() -> PgBox<pg_sys::BitmapAndState> {
        PgBox::<pg_sys::BitmapAndState>::alloc_node(PgNode::BitmapAndState)
    }
    pub fn makeBitmapOrState() -> PgBox<pg_sys::BitmapOrState> {
        PgBox::<pg_sys::BitmapOrState>::alloc_node(PgNode::BitmapOrState)
    }
    pub fn makeScanState() -> PgBox<pg_sys::pg12_specific::ScanState> {
        PgBox::<pg_sys::pg12_specific::ScanState>::alloc_node(PgNode::ScanState)
    }
    pub fn makeSeqScanState() -> PgBox<pg_sys::SeqScanState> {
        PgBox::<pg_sys::SeqScanState>::alloc_node(PgNode::SeqScanState)
    }
    pub fn makeSampleScanState() -> PgBox<pg_sys::pg12_specific::SampleScanState> {
        PgBox::<pg_sys::pg12_specific::SampleScanState>::alloc_node(PgNode::SampleScanState)
    }
    pub fn makeIndexScanState() -> PgBox<pg_sys::pg12_specific::IndexScanState> {
        PgBox::<pg_sys::pg12_specific::IndexScanState>::alloc_node(PgNode::IndexScanState)
    }
    pub fn makeIndexOnlyScanState() -> PgBox<pg_sys::pg12_specific::IndexOnlyScanState> {
        PgBox::<pg_sys::pg12_specific::IndexOnlyScanState>::alloc_node(PgNode::IndexOnlyScanState)
    }
    pub fn makeBitmapIndexScanState() -> PgBox<pg_sys::pg12_specific::BitmapIndexScanState> {
        PgBox::<pg_sys::pg12_specific::BitmapIndexScanState>::alloc_node(
            PgNode::BitmapIndexScanState,
        )
    }
    pub fn makeBitmapHeapScanState() -> PgBox<pg_sys::pg12_specific::BitmapHeapScanState> {
        PgBox::<pg_sys::pg12_specific::BitmapHeapScanState>::alloc_node(PgNode::BitmapHeapScanState)
    }
    pub fn makeTidScanState() -> PgBox<pg_sys::TidScanState> {
        PgBox::<pg_sys::TidScanState>::alloc_node(PgNode::TidScanState)
    }
    pub fn makeSubqueryScanState() -> PgBox<pg_sys::SubqueryScanState> {
        PgBox::<pg_sys::SubqueryScanState>::alloc_node(PgNode::SubqueryScanState)
    }
    pub fn makeFunctionScanState() -> PgBox<pg_sys::FunctionScanState> {
        PgBox::<pg_sys::FunctionScanState>::alloc_node(PgNode::FunctionScanState)
    }
    pub fn makeTableFuncScanState() -> PgBox<pg_sys::TableFuncScanState> {
        PgBox::<pg_sys::TableFuncScanState>::alloc_node(PgNode::TableFuncScanState)
    }
    pub fn makeValuesScanState() -> PgBox<pg_sys::ValuesScanState> {
        PgBox::<pg_sys::ValuesScanState>::alloc_node(PgNode::ValuesScanState)
    }
    pub fn makeCteScanState() -> PgBox<pg_sys::CteScanState> {
        PgBox::<pg_sys::CteScanState>::alloc_node(PgNode::CteScanState)
    }
    pub fn makeNamedTuplestoreScanState() -> PgBox<pg_sys::NamedTuplestoreScanState> {
        PgBox::<pg_sys::NamedTuplestoreScanState>::alloc_node(PgNode::NamedTuplestoreScanState)
    }
    pub fn makeWorkTableScanState() -> PgBox<pg_sys::WorkTableScanState> {
        PgBox::<pg_sys::WorkTableScanState>::alloc_node(PgNode::WorkTableScanState)
    }
    pub fn makeForeignScanState() -> PgBox<pg_sys::ForeignScanState> {
        PgBox::<pg_sys::ForeignScanState>::alloc_node(PgNode::ForeignScanState)
    }
    pub fn makeCustomScanState() -> PgBox<pg_sys::CustomScanState> {
        PgBox::<pg_sys::CustomScanState>::alloc_node(PgNode::CustomScanState)
    }
    pub fn makeJoinState() -> PgBox<pg_sys::JoinState> {
        PgBox::<pg_sys::JoinState>::alloc_node(PgNode::JoinState)
    }
    pub fn makeNestLoopState() -> PgBox<pg_sys::NestLoopState> {
        PgBox::<pg_sys::NestLoopState>::alloc_node(PgNode::NestLoopState)
    }
    pub fn makeMergeJoinState() -> PgBox<pg_sys::MergeJoinState> {
        PgBox::<pg_sys::MergeJoinState>::alloc_node(PgNode::MergeJoinState)
    }
    pub fn makeHashJoinState() -> PgBox<pg_sys::pg12_specific::HashJoinState> {
        PgBox::<pg_sys::pg12_specific::HashJoinState>::alloc_node(PgNode::HashJoinState)
    }
    pub fn makeMaterialState() -> PgBox<pg_sys::MaterialState> {
        PgBox::<pg_sys::MaterialState>::alloc_node(PgNode::MaterialState)
    }
    pub fn makeSortState() -> PgBox<pg_sys::pg12_specific::SortState> {
        PgBox::<pg_sys::pg12_specific::SortState>::alloc_node(PgNode::SortState)
    }
    pub fn makeGroupState() -> PgBox<pg_sys::pg12_specific::GroupState> {
        PgBox::<pg_sys::pg12_specific::GroupState>::alloc_node(PgNode::GroupState)
    }
    pub fn makeAggState() -> PgBox<pg_sys::pg12_specific::AggState> {
        PgBox::<pg_sys::pg12_specific::AggState>::alloc_node(PgNode::AggState)
    }
    pub fn makeWindowAggState() -> PgBox<pg_sys::pg12_specific::WindowAggState> {
        PgBox::<pg_sys::pg12_specific::WindowAggState>::alloc_node(PgNode::WindowAggState)
    }
    pub fn makeUniqueState() -> PgBox<pg_sys::pg12_specific::UniqueState> {
        PgBox::<pg_sys::pg12_specific::UniqueState>::alloc_node(PgNode::UniqueState)
    }
    pub fn makeGatherState() -> PgBox<pg_sys::pg12_specific::GatherState> {
        PgBox::<pg_sys::pg12_specific::GatherState>::alloc_node(PgNode::GatherState)
    }
    pub fn makeGatherMergeState() -> PgBox<pg_sys::pg12_specific::GatherMergeState> {
        PgBox::<pg_sys::pg12_specific::GatherMergeState>::alloc_node(PgNode::GatherMergeState)
    }
    pub fn makeHashState() -> PgBox<pg_sys::pg12_specific::HashState> {
        PgBox::<pg_sys::pg12_specific::HashState>::alloc_node(PgNode::HashState)
    }
    pub fn makeSetOpState() -> PgBox<pg_sys::pg12_specific::SetOpState> {
        PgBox::<pg_sys::pg12_specific::SetOpState>::alloc_node(PgNode::SetOpState)
    }
    pub fn makeLockRowsState() -> PgBox<pg_sys::pg12_specific::LockRowsState> {
        PgBox::<pg_sys::pg12_specific::LockRowsState>::alloc_node(PgNode::LockRowsState)
    }
    pub fn makeLimitState() -> PgBox<pg_sys::LimitState> {
        PgBox::<pg_sys::LimitState>::alloc_node(PgNode::LimitState)
    }

    pub fn makeAlias() -> PgBox<pg_sys::Alias> {
        PgBox::<pg_sys::Alias>::alloc_node(PgNode::Alias)
    }
    pub fn makeRangeVar() -> PgBox<pg_sys::RangeVar> {
        PgBox::<pg_sys::RangeVar>::alloc_node(PgNode::RangeVar)
    }
    pub fn makeTableFunc() -> PgBox<pg_sys::TableFunc> {
        PgBox::<pg_sys::TableFunc>::alloc_node(PgNode::TableFunc)
    }
    pub fn makeExpr() -> PgBox<pg_sys::Expr> {
        PgBox::<pg_sys::Expr>::alloc_node(PgNode::Expr)
    }
    pub fn makeVar() -> PgBox<pg_sys::Var> {
        PgBox::<pg_sys::Var>::alloc_node(PgNode::Var)
    }
    pub fn makeConst() -> PgBox<pg_sys::Const> {
        PgBox::<pg_sys::Const>::alloc_node(PgNode::Const)
    }
    pub fn makeParam() -> PgBox<pg_sys::Param> {
        PgBox::<pg_sys::Param>::alloc_node(PgNode::Param)
    }
    pub fn makeAggref() -> PgBox<pg_sys::Aggref> {
        PgBox::<pg_sys::Aggref>::alloc_node(PgNode::Aggref)
    }
    pub fn makeGroupingFunc() -> PgBox<pg_sys::GroupingFunc> {
        PgBox::<pg_sys::GroupingFunc>::alloc_node(PgNode::GroupingFunc)
    }
    pub fn makeWindowFunc() -> PgBox<pg_sys::WindowFunc> {
        PgBox::<pg_sys::WindowFunc>::alloc_node(PgNode::WindowFunc)
    }
    pub fn makeSubscriptingRef() -> PgBox<pg_sys::pg12_specific::SubscriptingRef> {
        PgBox::<pg_sys::pg12_specific::SubscriptingRef>::alloc_node(PgNode::SubscriptingRef)
    }
    pub fn makeFuncExpr() -> PgBox<pg_sys::FuncExpr> {
        PgBox::<pg_sys::FuncExpr>::alloc_node(PgNode::FuncExpr)
    }
    pub fn makeNamedArgExpr() -> PgBox<pg_sys::NamedArgExpr> {
        PgBox::<pg_sys::NamedArgExpr>::alloc_node(PgNode::NamedArgExpr)
    }
    pub fn makeOpExpr() -> PgBox<pg_sys::OpExpr> {
        PgBox::<pg_sys::OpExpr>::alloc_node(PgNode::OpExpr)
    }
    pub fn makeDistinctExpr() -> PgBox<pg_sys::DistinctExpr> {
        PgBox::<pg_sys::DistinctExpr>::alloc_node(PgNode::DistinctExpr)
    }
    pub fn makeNullIfExpr() -> PgBox<pg_sys::NullIfExpr> {
        PgBox::<pg_sys::NullIfExpr>::alloc_node(PgNode::NullIfExpr)
    }
    pub fn makeScalarArrayOpExpr() -> PgBox<pg_sys::ScalarArrayOpExpr> {
        PgBox::<pg_sys::ScalarArrayOpExpr>::alloc_node(PgNode::ScalarArrayOpExpr)
    }
    pub fn makeBoolExpr() -> PgBox<pg_sys::BoolExpr> {
        PgBox::<pg_sys::BoolExpr>::alloc_node(PgNode::BoolExpr)
    }
    pub fn makeSubLink() -> PgBox<pg_sys::SubLink> {
        PgBox::<pg_sys::SubLink>::alloc_node(PgNode::SubLink)
    }
    pub fn makeSubPlan() -> PgBox<pg_sys::SubPlan> {
        PgBox::<pg_sys::SubPlan>::alloc_node(PgNode::SubPlan)
    }
    pub fn makeAlternativeSubPlan() -> PgBox<pg_sys::AlternativeSubPlan> {
        PgBox::<pg_sys::AlternativeSubPlan>::alloc_node(PgNode::AlternativeSubPlan)
    }
    pub fn makeFieldSelect() -> PgBox<pg_sys::FieldSelect> {
        PgBox::<pg_sys::FieldSelect>::alloc_node(PgNode::FieldSelect)
    }
    pub fn makeFieldStore() -> PgBox<pg_sys::FieldStore> {
        PgBox::<pg_sys::FieldStore>::alloc_node(PgNode::FieldStore)
    }
    pub fn makeRelabelType() -> PgBox<pg_sys::RelabelType> {
        PgBox::<pg_sys::RelabelType>::alloc_node(PgNode::RelabelType)
    }
    pub fn makeCoerceViaIO() -> PgBox<pg_sys::CoerceViaIO> {
        PgBox::<pg_sys::CoerceViaIO>::alloc_node(PgNode::CoerceViaIO)
    }
    pub fn makeArrayCoerceExpr() -> PgBox<pg_sys::pg12_specific::ArrayCoerceExpr> {
        PgBox::<pg_sys::pg12_specific::ArrayCoerceExpr>::alloc_node(PgNode::ArrayCoerceExpr)
    }
    pub fn makeConvertRowtypeExpr() -> PgBox<pg_sys::ConvertRowtypeExpr> {
        PgBox::<pg_sys::ConvertRowtypeExpr>::alloc_node(PgNode::ConvertRowtypeExpr)
    }
    pub fn makeCollateExpr() -> PgBox<pg_sys::CollateExpr> {
        PgBox::<pg_sys::CollateExpr>::alloc_node(PgNode::CollateExpr)
    }
    pub fn makeCaseExpr() -> PgBox<pg_sys::CaseExpr> {
        PgBox::<pg_sys::CaseExpr>::alloc_node(PgNode::CaseExpr)
    }
    pub fn makeCaseWhen() -> PgBox<pg_sys::CaseWhen> {
        PgBox::<pg_sys::CaseWhen>::alloc_node(PgNode::CaseWhen)
    }
    pub fn makeCaseTestExpr() -> PgBox<pg_sys::CaseTestExpr> {
        PgBox::<pg_sys::CaseTestExpr>::alloc_node(PgNode::CaseTestExpr)
    }
    pub fn makeArrayExpr() -> PgBox<pg_sys::ArrayExpr> {
        PgBox::<pg_sys::ArrayExpr>::alloc_node(PgNode::ArrayExpr)
    }
    pub fn makeRowExpr() -> PgBox<pg_sys::RowExpr> {
        PgBox::<pg_sys::RowExpr>::alloc_node(PgNode::RowExpr)
    }
    pub fn makeRowCompareExpr() -> PgBox<pg_sys::RowCompareExpr> {
        PgBox::<pg_sys::RowCompareExpr>::alloc_node(PgNode::RowCompareExpr)
    }
    pub fn makeCoalesceExpr() -> PgBox<pg_sys::CoalesceExpr> {
        PgBox::<pg_sys::CoalesceExpr>::alloc_node(PgNode::CoalesceExpr)
    }
    pub fn makeMinMaxExpr() -> PgBox<pg_sys::MinMaxExpr> {
        PgBox::<pg_sys::MinMaxExpr>::alloc_node(PgNode::MinMaxExpr)
    }
    pub fn makeSQLValueFunction() -> PgBox<pg_sys::SQLValueFunction> {
        PgBox::<pg_sys::SQLValueFunction>::alloc_node(PgNode::SQLValueFunction)
    }
    pub fn makeXmlExpr() -> PgBox<pg_sys::XmlExpr> {
        PgBox::<pg_sys::XmlExpr>::alloc_node(PgNode::XmlExpr)
    }
    pub fn makeNullTest() -> PgBox<pg_sys::NullTest> {
        PgBox::<pg_sys::NullTest>::alloc_node(PgNode::NullTest)
    }
    pub fn makeBooleanTest() -> PgBox<pg_sys::BooleanTest> {
        PgBox::<pg_sys::BooleanTest>::alloc_node(PgNode::BooleanTest)
    }
    pub fn makeCoerceToDomain() -> PgBox<pg_sys::CoerceToDomain> {
        PgBox::<pg_sys::CoerceToDomain>::alloc_node(PgNode::CoerceToDomain)
    }
    pub fn makeCoerceToDomainValue() -> PgBox<pg_sys::CoerceToDomainValue> {
        PgBox::<pg_sys::CoerceToDomainValue>::alloc_node(PgNode::CoerceToDomainValue)
    }
    pub fn makeSetToDefault() -> PgBox<pg_sys::SetToDefault> {
        PgBox::<pg_sys::SetToDefault>::alloc_node(PgNode::SetToDefault)
    }
    pub fn makeCurrentOfExpr() -> PgBox<pg_sys::CurrentOfExpr> {
        PgBox::<pg_sys::CurrentOfExpr>::alloc_node(PgNode::CurrentOfExpr)
    }
    pub fn makeNextValueExpr() -> PgBox<pg_sys::NextValueExpr> {
        PgBox::<pg_sys::NextValueExpr>::alloc_node(PgNode::NextValueExpr)
    }
    pub fn makeInferenceElem() -> PgBox<pg_sys::InferenceElem> {
        PgBox::<pg_sys::InferenceElem>::alloc_node(PgNode::InferenceElem)
    }
    pub fn makeTargetEntry() -> PgBox<pg_sys::TargetEntry> {
        PgBox::<pg_sys::TargetEntry>::alloc_node(PgNode::TargetEntry)
    }
    pub fn makeRangeTblRef() -> PgBox<pg_sys::RangeTblRef> {
        PgBox::<pg_sys::RangeTblRef>::alloc_node(PgNode::RangeTblRef)
    }
    pub fn makeJoinExpr() -> PgBox<pg_sys::JoinExpr> {
        PgBox::<pg_sys::JoinExpr>::alloc_node(PgNode::JoinExpr)
    }
    pub fn makeFromExpr() -> PgBox<pg_sys::FromExpr> {
        PgBox::<pg_sys::FromExpr>::alloc_node(PgNode::FromExpr)
    }
    pub fn makeOnConflictExpr() -> PgBox<pg_sys::OnConflictExpr> {
        PgBox::<pg_sys::OnConflictExpr>::alloc_node(PgNode::OnConflictExpr)
    }
    pub fn makeIntoClause() -> PgBox<pg_sys::pg12_specific::IntoClause> {
        PgBox::<pg_sys::pg12_specific::IntoClause>::alloc_node(PgNode::IntoClause)
    }

    pub fn makeExprState() -> PgBox<pg_sys::pg12_specific::ExprState> {
        PgBox::<pg_sys::pg12_specific::ExprState>::alloc_node(PgNode::ExprState)
    }
    pub fn makeAggrefExprState() -> PgBox<pg_sys::AggrefExprState> {
        PgBox::<pg_sys::AggrefExprState>::alloc_node(PgNode::AggrefExprState)
    }
    pub fn makeWindowFuncExprState() -> PgBox<pg_sys::WindowFuncExprState> {
        PgBox::<pg_sys::WindowFuncExprState>::alloc_node(PgNode::WindowFuncExprState)
    }
    pub fn makeSetExprState() -> PgBox<pg_sys::pg12_specific::SetExprState> {
        PgBox::<pg_sys::pg12_specific::SetExprState>::alloc_node(PgNode::SetExprState)
    }
    pub fn makeSubPlanState() -> PgBox<pg_sys::pg12_specific::SubPlanState> {
        PgBox::<pg_sys::pg12_specific::SubPlanState>::alloc_node(PgNode::SubPlanState)
    }
    pub fn makeAlternativeSubPlanState() -> PgBox<pg_sys::AlternativeSubPlanState> {
        PgBox::<pg_sys::AlternativeSubPlanState>::alloc_node(PgNode::AlternativeSubPlanState)
    }
    pub fn makeDomainConstraintState() -> PgBox<pg_sys::DomainConstraintState> {
        PgBox::<pg_sys::DomainConstraintState>::alloc_node(PgNode::DomainConstraintState)
    }

    pub fn makePlannerInfo() -> PgBox<pg_sys::pg12_specific::PlannerInfo> {
        PgBox::<pg_sys::pg12_specific::PlannerInfo>::alloc_node(PgNode::PlannerInfo)
    }
    pub fn makePlannerGlobal() -> PgBox<pg_sys::pg12_specific::PlannerGlobal> {
        PgBox::<pg_sys::pg12_specific::PlannerGlobal>::alloc_node(PgNode::PlannerGlobal)
    }
    pub fn makeRelOptInfo() -> PgBox<pg_sys::pg12_specific::RelOptInfo> {
        PgBox::<pg_sys::pg12_specific::RelOptInfo>::alloc_node(PgNode::RelOptInfo)
    }
    pub fn makeIndexOptInfo() -> PgBox<pg_sys::pg12_specific::IndexOptInfo> {
        PgBox::<pg_sys::pg12_specific::IndexOptInfo>::alloc_node(PgNode::IndexOptInfo)
    }
    pub fn makeForeignKeyOptInfo() -> PgBox<pg_sys::ForeignKeyOptInfo> {
        PgBox::<pg_sys::ForeignKeyOptInfo>::alloc_node(PgNode::ForeignKeyOptInfo)
    }
    pub fn makeParamPathInfo() -> PgBox<pg_sys::ParamPathInfo> {
        PgBox::<pg_sys::ParamPathInfo>::alloc_node(PgNode::ParamPathInfo)
    }
    pub fn makePath() -> PgBox<pg_sys::Path> {
        PgBox::<pg_sys::Path>::alloc_node(PgNode::Path)
    }
    pub fn makeIndexPath() -> PgBox<pg_sys::pg12_specific::IndexPath> {
        PgBox::<pg_sys::pg12_specific::IndexPath>::alloc_node(PgNode::IndexPath)
    }
    pub fn makeBitmapHeapPath() -> PgBox<pg_sys::BitmapHeapPath> {
        PgBox::<pg_sys::BitmapHeapPath>::alloc_node(PgNode::BitmapHeapPath)
    }
    pub fn makeBitmapAndPath() -> PgBox<pg_sys::BitmapAndPath> {
        PgBox::<pg_sys::BitmapAndPath>::alloc_node(PgNode::BitmapAndPath)
    }
    pub fn makeBitmapOrPath() -> PgBox<pg_sys::BitmapOrPath> {
        PgBox::<pg_sys::BitmapOrPath>::alloc_node(PgNode::BitmapOrPath)
    }
    pub fn makeTidPath() -> PgBox<pg_sys::TidPath> {
        PgBox::<pg_sys::TidPath>::alloc_node(PgNode::TidPath)
    }
    pub fn makeSubqueryScanPath() -> PgBox<pg_sys::SubqueryScanPath> {
        PgBox::<pg_sys::SubqueryScanPath>::alloc_node(PgNode::SubqueryScanPath)
    }
    pub fn makeForeignPath() -> PgBox<pg_sys::ForeignPath> {
        PgBox::<pg_sys::ForeignPath>::alloc_node(PgNode::ForeignPath)
    }
    pub fn makeCustomPath() -> PgBox<pg_sys::CustomPath> {
        PgBox::<pg_sys::CustomPath>::alloc_node(PgNode::CustomPath)
    }
    pub fn makeNestPath() -> PgBox<pg_sys::NestPath> {
        PgBox::<pg_sys::NestPath>::alloc_node(PgNode::NestPath)
    }
    pub fn makeMergePath() -> PgBox<pg_sys::MergePath> {
        PgBox::<pg_sys::MergePath>::alloc_node(PgNode::MergePath)
    }
    pub fn makeHashPath() -> PgBox<pg_sys::pg12_specific::HashPath> {
        PgBox::<pg_sys::pg12_specific::HashPath>::alloc_node(PgNode::HashPath)
    }
    pub fn makeAppendPath() -> PgBox<pg_sys::pg12_specific::AppendPath> {
        PgBox::<pg_sys::pg12_specific::AppendPath>::alloc_node(PgNode::AppendPath)
    }
    pub fn makeMergeAppendPath() -> PgBox<pg_sys::MergeAppendPath> {
        PgBox::<pg_sys::MergeAppendPath>::alloc_node(PgNode::MergeAppendPath)
    }
    pub fn makeGroupResultPath() -> PgBox<pg_sys::pg12_specific::GroupResultPath> {
        PgBox::<pg_sys::pg12_specific::GroupResultPath>::alloc_node(PgNode::GroupResultPath)
    }
    pub fn makeMaterialPath() -> PgBox<pg_sys::MaterialPath> {
        PgBox::<pg_sys::MaterialPath>::alloc_node(PgNode::MaterialPath)
    }
    pub fn makeUniquePath() -> PgBox<pg_sys::UniquePath> {
        PgBox::<pg_sys::UniquePath>::alloc_node(PgNode::UniquePath)
    }
    pub fn makeGatherPath() -> PgBox<pg_sys::GatherPath> {
        PgBox::<pg_sys::GatherPath>::alloc_node(PgNode::GatherPath)
    }
    pub fn makeGatherMergePath() -> PgBox<pg_sys::GatherMergePath> {
        PgBox::<pg_sys::GatherMergePath>::alloc_node(PgNode::GatherMergePath)
    }
    pub fn makeProjectionPath() -> PgBox<pg_sys::ProjectionPath> {
        PgBox::<pg_sys::ProjectionPath>::alloc_node(PgNode::ProjectionPath)
    }
    pub fn makeProjectSetPath() -> PgBox<pg_sys::ProjectSetPath> {
        PgBox::<pg_sys::ProjectSetPath>::alloc_node(PgNode::ProjectSetPath)
    }
    pub fn makeSortPath() -> PgBox<pg_sys::SortPath> {
        PgBox::<pg_sys::SortPath>::alloc_node(PgNode::SortPath)
    }
    pub fn makeGroupPath() -> PgBox<pg_sys::GroupPath> {
        PgBox::<pg_sys::GroupPath>::alloc_node(PgNode::GroupPath)
    }
    pub fn makeUpperUniquePath() -> PgBox<pg_sys::UpperUniquePath> {
        PgBox::<pg_sys::UpperUniquePath>::alloc_node(PgNode::UpperUniquePath)
    }
    pub fn makeAggPath() -> PgBox<pg_sys::AggPath> {
        PgBox::<pg_sys::AggPath>::alloc_node(PgNode::AggPath)
    }
    pub fn makeGroupingSetsPath() -> PgBox<pg_sys::GroupingSetsPath> {
        PgBox::<pg_sys::GroupingSetsPath>::alloc_node(PgNode::GroupingSetsPath)
    }
    pub fn makeMinMaxAggPath() -> PgBox<pg_sys::MinMaxAggPath> {
        PgBox::<pg_sys::MinMaxAggPath>::alloc_node(PgNode::MinMaxAggPath)
    }
    pub fn makeWindowAggPath() -> PgBox<pg_sys::pg12_specific::WindowAggPath> {
        PgBox::<pg_sys::pg12_specific::WindowAggPath>::alloc_node(PgNode::WindowAggPath)
    }
    pub fn makeSetOpPath() -> PgBox<pg_sys::SetOpPath> {
        PgBox::<pg_sys::SetOpPath>::alloc_node(PgNode::SetOpPath)
    }
    pub fn makeRecursiveUnionPath() -> PgBox<pg_sys::RecursiveUnionPath> {
        PgBox::<pg_sys::RecursiveUnionPath>::alloc_node(PgNode::RecursiveUnionPath)
    }
    pub fn makeLockRowsPath() -> PgBox<pg_sys::LockRowsPath> {
        PgBox::<pg_sys::LockRowsPath>::alloc_node(PgNode::LockRowsPath)
    }
    pub fn makeModifyTablePath() -> PgBox<pg_sys::pg12_specific::ModifyTablePath> {
        PgBox::<pg_sys::pg12_specific::ModifyTablePath>::alloc_node(PgNode::ModifyTablePath)
    }
    pub fn makeLimitPath() -> PgBox<pg_sys::LimitPath> {
        PgBox::<pg_sys::LimitPath>::alloc_node(PgNode::LimitPath)
    }

    pub fn makeEquivalenceClass() -> PgBox<pg_sys::EquivalenceClass> {
        PgBox::<pg_sys::EquivalenceClass>::alloc_node(PgNode::EquivalenceClass)
    }
    pub fn makeEquivalenceMember() -> PgBox<pg_sys::EquivalenceMember> {
        PgBox::<pg_sys::EquivalenceMember>::alloc_node(PgNode::EquivalenceMember)
    }
    pub fn makePathKey() -> PgBox<pg_sys::PathKey> {
        PgBox::<pg_sys::PathKey>::alloc_node(PgNode::PathKey)
    }
    pub fn makePathTarget() -> PgBox<pg_sys::PathTarget> {
        PgBox::<pg_sys::PathTarget>::alloc_node(PgNode::PathTarget)
    }
    pub fn makeRestrictInfo() -> PgBox<pg_sys::pg12_specific::RestrictInfo> {
        PgBox::<pg_sys::pg12_specific::RestrictInfo>::alloc_node(PgNode::RestrictInfo)
    }
    pub fn makeIndexClause() -> PgBox<pg_sys::pg12_specific::IndexClause> {
        PgBox::<pg_sys::pg12_specific::IndexClause>::alloc_node(PgNode::IndexClause)
    }
    pub fn makePlaceHolderVar() -> PgBox<pg_sys::PlaceHolderVar> {
        PgBox::<pg_sys::PlaceHolderVar>::alloc_node(PgNode::PlaceHolderVar)
    }
    pub fn makeSpecialJoinInfo() -> PgBox<pg_sys::SpecialJoinInfo> {
        PgBox::<pg_sys::SpecialJoinInfo>::alloc_node(PgNode::SpecialJoinInfo)
    }
    pub fn makeAppendRelInfo() -> PgBox<pg_sys::AppendRelInfo> {
        PgBox::<pg_sys::AppendRelInfo>::alloc_node(PgNode::AppendRelInfo)
    }
    pub fn makePlaceHolderInfo() -> PgBox<pg_sys::PlaceHolderInfo> {
        PgBox::<pg_sys::PlaceHolderInfo>::alloc_node(PgNode::PlaceHolderInfo)
    }
    pub fn makeMinMaxAggInfo() -> PgBox<pg_sys::MinMaxAggInfo> {
        PgBox::<pg_sys::MinMaxAggInfo>::alloc_node(PgNode::MinMaxAggInfo)
    }
    pub fn makePlannerParamItem() -> PgBox<pg_sys::PlannerParamItem> {
        PgBox::<pg_sys::PlannerParamItem>::alloc_node(PgNode::PlannerParamItem)
    }
    pub fn makeRollupData() -> PgBox<pg_sys::RollupData> {
        PgBox::<pg_sys::RollupData>::alloc_node(PgNode::RollupData)
    }
    pub fn makeGroupingSetData() -> PgBox<pg_sys::GroupingSetData> {
        PgBox::<pg_sys::GroupingSetData>::alloc_node(PgNode::GroupingSetData)
    }
    pub fn makeStatisticExtInfo() -> PgBox<pg_sys::StatisticExtInfo> {
        PgBox::<pg_sys::StatisticExtInfo>::alloc_node(PgNode::StatisticExtInfo)
    }

    pub fn makeValue() -> PgBox<pg_sys::Value> {
        PgBox::<pg_sys::Value>::alloc_node(PgNode::Value)
    }
    pub fn makeInteger(i: i32) -> PgBox<pg_sys::Value> {
        let mut value = PgNodeFactory::makeValue();

        value.type_ = PgNode::Integer as u32;
        value.val.ival = i;
        value
    }
    pub fn makeFloat(memory_context: PgMemoryContexts, f: f64) -> PgBox<pg_sys::Value> {
        let mut value = PgNodeFactory::makeValue();

        value.type_ = PgNode::Float as u32;
        value.val.str = memory_context.pstrdup(f.to_string().as_str());
        value
    }
    pub fn makeString(memory_context: PgMemoryContexts, s: &str) -> PgBox<pg_sys::Value> {
        let mut value = PgNodeFactory::makeValue();

        value.type_ = PgNode::String as u32;
        value.val.str = memory_context.pstrdup(s);
        value
    }
    pub fn makeBitString(memory_context: PgMemoryContexts, bs: &str) -> PgBox<pg_sys::Value> {
        let mut value = PgNodeFactory::makeValue();

        value.type_ = PgNode::BitString as u32;
        value.val.str = memory_context.pstrdup(bs);
        value
    }
    pub fn makeNull() -> PgBox<pg_sys::Const> {
        let con = unsafe {
            pg_sys::makeConst(
                pg_sys::UNKNOWNOID,
                -1,
                pg_sys::InvalidOid,
                -2,
                0 as pg_sys::Datum,
                true,
                false,
            )
        };
        PgBox::<pg_sys::Const>::from_pg(con)
    }

    pub fn makeList() -> PgBox<pg_sys::List> {
        // an empty list is NIL, so we represent that as a NULL PgDatum
        PgBox::<pg_sys::List>::null()
    }
    pub fn makeIntList() -> PgBox<pg_sys::List> {
        // an empty list is NIL, so we represent that as a NULL PgDatum
        PgBox::<pg_sys::List>::null()
    }
    pub fn makeOidList() -> PgBox<pg_sys::List> {
        // an empty list is NIL, so we represent that as a NULL PgDatum
        PgBox::<pg_sys::List>::null()
    }

    pub fn makeExtensibleNode() -> PgBox<pg_sys::ExtensibleNode> {
        PgBox::<pg_sys::ExtensibleNode>::alloc_node(PgNode::ExtensibleNode)
    }

    pub fn makeRawStmt() -> PgBox<pg_sys::RawStmt> {
        PgBox::<pg_sys::RawStmt>::alloc_node(PgNode::RawStmt)
    }
    pub fn makeQuery() -> PgBox<pg_sys::pg12_specific::Query> {
        PgBox::<pg_sys::pg12_specific::Query>::alloc_node(PgNode::Query)
    }
    pub fn makePlannedStmt() -> PgBox<pg_sys::pg12_specific::PlannedStmt> {
        PgBox::<pg_sys::pg12_specific::PlannedStmt>::alloc_node(PgNode::PlannedStmt)
    }
    pub fn makeInsertStmt() -> PgBox<pg_sys::InsertStmt> {
        PgBox::<pg_sys::InsertStmt>::alloc_node(PgNode::InsertStmt)
    }
    pub fn makeDeleteStmt() -> PgBox<pg_sys::DeleteStmt> {
        PgBox::<pg_sys::DeleteStmt>::alloc_node(PgNode::DeleteStmt)
    }
    pub fn makeUpdateStmt() -> PgBox<pg_sys::UpdateStmt> {
        PgBox::<pg_sys::UpdateStmt>::alloc_node(PgNode::UpdateStmt)
    }
    pub fn makeSelectStmt() -> PgBox<pg_sys::SelectStmt> {
        PgBox::<pg_sys::SelectStmt>::alloc_node(PgNode::SelectStmt)
    }
    pub fn makeAlterTableStmt() -> PgBox<pg_sys::AlterTableStmt> {
        PgBox::<pg_sys::AlterTableStmt>::alloc_node(PgNode::AlterTableStmt)
    }
    pub fn makeAlterTableCmd() -> PgBox<pg_sys::pg12_specific::AlterTableCmd> {
        PgBox::<pg_sys::pg12_specific::AlterTableCmd>::alloc_node(PgNode::AlterTableCmd)
    }
    pub fn makeAlterDomainStmt() -> PgBox<pg_sys::AlterDomainStmt> {
        PgBox::<pg_sys::AlterDomainStmt>::alloc_node(PgNode::AlterDomainStmt)
    }
    pub fn makeSetOperationStmt() -> PgBox<pg_sys::SetOperationStmt> {
        PgBox::<pg_sys::SetOperationStmt>::alloc_node(PgNode::SetOperationStmt)
    }
    pub fn makeGrantStmt() -> PgBox<pg_sys::pg12_specific::GrantStmt> {
        PgBox::<pg_sys::pg12_specific::GrantStmt>::alloc_node(PgNode::GrantStmt)
    }
    pub fn makeGrantRoleStmt() -> PgBox<pg_sys::GrantRoleStmt> {
        PgBox::<pg_sys::GrantRoleStmt>::alloc_node(PgNode::GrantRoleStmt)
    }
    pub fn makeAlterDefaultPrivilegesStmt() -> PgBox<pg_sys::AlterDefaultPrivilegesStmt> {
        PgBox::<pg_sys::AlterDefaultPrivilegesStmt>::alloc_node(PgNode::AlterDefaultPrivilegesStmt)
    }
    pub fn makeClosePortalStmt() -> PgBox<pg_sys::ClosePortalStmt> {
        PgBox::<pg_sys::ClosePortalStmt>::alloc_node(PgNode::ClosePortalStmt)
    }
    pub fn makeClusterStmt() -> PgBox<pg_sys::pg12_specific::ClusterStmt> {
        PgBox::<pg_sys::pg12_specific::ClusterStmt>::alloc_node(PgNode::ClusterStmt)
    }
    pub fn makeCopyStmt() -> PgBox<pg_sys::pg12_specific::CopyStmt> {
        PgBox::<pg_sys::pg12_specific::CopyStmt>::alloc_node(PgNode::CopyStmt)
    }
    pub fn makeCreateStmt() -> PgBox<pg_sys::pg12_specific::CreateStmt> {
        PgBox::<pg_sys::pg12_specific::CreateStmt>::alloc_node(PgNode::CreateStmt)
    }
    pub fn makeDefineStmt() -> PgBox<pg_sys::pg12_specific::DefineStmt> {
        PgBox::<pg_sys::pg12_specific::DefineStmt>::alloc_node(PgNode::DefineStmt)
    }
    pub fn makeDropStmt() -> PgBox<pg_sys::DropStmt> {
        PgBox::<pg_sys::DropStmt>::alloc_node(PgNode::DropStmt)
    }
    pub fn makeTruncateStmt() -> PgBox<pg_sys::TruncateStmt> {
        PgBox::<pg_sys::TruncateStmt>::alloc_node(PgNode::TruncateStmt)
    }
    pub fn makeCommentStmt() -> PgBox<pg_sys::CommentStmt> {
        PgBox::<pg_sys::CommentStmt>::alloc_node(PgNode::CommentStmt)
    }
    pub fn makeFetchStmt() -> PgBox<pg_sys::FetchStmt> {
        PgBox::<pg_sys::FetchStmt>::alloc_node(PgNode::FetchStmt)
    }
    pub fn makeIndexStmt() -> PgBox<pg_sys::pg12_specific::IndexStmt> {
        PgBox::<pg_sys::pg12_specific::IndexStmt>::alloc_node(PgNode::IndexStmt)
    }
    pub fn makeCreateFunctionStmt() -> PgBox<pg_sys::pg12_specific::CreateFunctionStmt> {
        PgBox::<pg_sys::pg12_specific::CreateFunctionStmt>::alloc_node(PgNode::CreateFunctionStmt)
    }
    pub fn makeAlterFunctionStmt() -> PgBox<pg_sys::pg12_specific::AlterFunctionStmt> {
        PgBox::<pg_sys::pg12_specific::AlterFunctionStmt>::alloc_node(PgNode::AlterFunctionStmt)
    }
    pub fn makeDoStmt() -> PgBox<pg_sys::DoStmt> {
        PgBox::<pg_sys::DoStmt>::alloc_node(PgNode::DoStmt)
    }
    pub fn makeRenameStmt() -> PgBox<pg_sys::RenameStmt> {
        PgBox::<pg_sys::RenameStmt>::alloc_node(PgNode::RenameStmt)
    }
    pub fn makeRuleStmt() -> PgBox<pg_sys::RuleStmt> {
        PgBox::<pg_sys::RuleStmt>::alloc_node(PgNode::RuleStmt)
    }
    pub fn makeNotifyStmt() -> PgBox<pg_sys::NotifyStmt> {
        PgBox::<pg_sys::NotifyStmt>::alloc_node(PgNode::NotifyStmt)
    }
    pub fn makeListenStmt() -> PgBox<pg_sys::ListenStmt> {
        PgBox::<pg_sys::ListenStmt>::alloc_node(PgNode::ListenStmt)
    }
    pub fn makeUnlistenStmt() -> PgBox<pg_sys::UnlistenStmt> {
        PgBox::<pg_sys::UnlistenStmt>::alloc_node(PgNode::UnlistenStmt)
    }
    pub fn makeTransactionStmt() -> PgBox<pg_sys::pg12_specific::TransactionStmt> {
        PgBox::<pg_sys::pg12_specific::TransactionStmt>::alloc_node(PgNode::TransactionStmt)
    }
    pub fn makeViewStmt() -> PgBox<pg_sys::ViewStmt> {
        PgBox::<pg_sys::ViewStmt>::alloc_node(PgNode::ViewStmt)
    }
    pub fn makeLoadStmt() -> PgBox<pg_sys::LoadStmt> {
        PgBox::<pg_sys::LoadStmt>::alloc_node(PgNode::LoadStmt)
    }
    pub fn makeCreateDomainStmt() -> PgBox<pg_sys::CreateDomainStmt> {
        PgBox::<pg_sys::CreateDomainStmt>::alloc_node(PgNode::CreateDomainStmt)
    }
    pub fn makeCreatedbStmt() -> PgBox<pg_sys::CreatedbStmt> {
        PgBox::<pg_sys::CreatedbStmt>::alloc_node(PgNode::CreatedbStmt)
    }
    pub fn makeDropdbStmt() -> PgBox<pg_sys::DropdbStmt> {
        PgBox::<pg_sys::DropdbStmt>::alloc_node(PgNode::DropdbStmt)
    }
    pub fn makeVacuumStmt() -> PgBox<pg_sys::pg12_specific::VacuumStmt> {
        PgBox::<pg_sys::pg12_specific::VacuumStmt>::alloc_node(PgNode::VacuumStmt)
    }
    pub fn makeExplainStmt() -> PgBox<pg_sys::ExplainStmt> {
        PgBox::<pg_sys::ExplainStmt>::alloc_node(PgNode::ExplainStmt)
    }
    pub fn makeCreateTableAsStmt() -> PgBox<pg_sys::CreateTableAsStmt> {
        PgBox::<pg_sys::CreateTableAsStmt>::alloc_node(PgNode::CreateTableAsStmt)
    }
    pub fn makeCreateSeqStmt() -> PgBox<pg_sys::CreateSeqStmt> {
        PgBox::<pg_sys::CreateSeqStmt>::alloc_node(PgNode::CreateSeqStmt)
    }
    pub fn makeAlterSeqStmt() -> PgBox<pg_sys::AlterSeqStmt> {
        PgBox::<pg_sys::AlterSeqStmt>::alloc_node(PgNode::AlterSeqStmt)
    }
    pub fn makeVariableSetStmt() -> PgBox<pg_sys::VariableSetStmt> {
        PgBox::<pg_sys::VariableSetStmt>::alloc_node(PgNode::VariableSetStmt)
    }
    pub fn makeVariableShowStmt() -> PgBox<pg_sys::VariableShowStmt> {
        PgBox::<pg_sys::VariableShowStmt>::alloc_node(PgNode::VariableShowStmt)
    }
    pub fn makeDiscardStmt() -> PgBox<pg_sys::DiscardStmt> {
        PgBox::<pg_sys::DiscardStmt>::alloc_node(PgNode::DiscardStmt)
    }
    pub fn makeCreateTrigStmt() -> PgBox<pg_sys::CreateTrigStmt> {
        PgBox::<pg_sys::CreateTrigStmt>::alloc_node(PgNode::CreateTrigStmt)
    }
    pub fn makeCreatePLangStmt() -> PgBox<pg_sys::CreatePLangStmt> {
        PgBox::<pg_sys::CreatePLangStmt>::alloc_node(PgNode::CreatePLangStmt)
    }
    pub fn makeCreateRoleStmt() -> PgBox<pg_sys::CreateRoleStmt> {
        PgBox::<pg_sys::CreateRoleStmt>::alloc_node(PgNode::CreateRoleStmt)
    }
    pub fn makeAlterRoleStmt() -> PgBox<pg_sys::AlterRoleStmt> {
        PgBox::<pg_sys::AlterRoleStmt>::alloc_node(PgNode::AlterRoleStmt)
    }
    pub fn makeDropRoleStmt() -> PgBox<pg_sys::DropRoleStmt> {
        PgBox::<pg_sys::DropRoleStmt>::alloc_node(PgNode::DropRoleStmt)
    }
    pub fn makeLockStmt() -> PgBox<pg_sys::LockStmt> {
        PgBox::<pg_sys::LockStmt>::alloc_node(PgNode::LockStmt)
    }
    pub fn makeConstraintsSetStmt() -> PgBox<pg_sys::ConstraintsSetStmt> {
        PgBox::<pg_sys::ConstraintsSetStmt>::alloc_node(PgNode::ConstraintsSetStmt)
    }
    pub fn makeReindexStmt() -> PgBox<pg_sys::pg12_specific::ReindexStmt> {
        PgBox::<pg_sys::pg12_specific::ReindexStmt>::alloc_node(PgNode::ReindexStmt)
    }
    pub fn makeCheckPointStmt() -> PgBox<pg_sys::CheckPointStmt> {
        PgBox::<pg_sys::CheckPointStmt>::alloc_node(PgNode::CheckPointStmt)
    }
    pub fn makeCreateSchemaStmt() -> PgBox<pg_sys::CreateSchemaStmt> {
        PgBox::<pg_sys::CreateSchemaStmt>::alloc_node(PgNode::CreateSchemaStmt)
    }
    pub fn makeAlterDatabaseStmt() -> PgBox<pg_sys::AlterDatabaseStmt> {
        PgBox::<pg_sys::AlterDatabaseStmt>::alloc_node(PgNode::AlterDatabaseStmt)
    }
    pub fn makeAlterDatabaseSetStmt() -> PgBox<pg_sys::AlterDatabaseSetStmt> {
        PgBox::<pg_sys::AlterDatabaseSetStmt>::alloc_node(PgNode::AlterDatabaseSetStmt)
    }
    pub fn makeAlterRoleSetStmt() -> PgBox<pg_sys::AlterRoleSetStmt> {
        PgBox::<pg_sys::AlterRoleSetStmt>::alloc_node(PgNode::AlterRoleSetStmt)
    }
    pub fn makeCreateConversionStmt() -> PgBox<pg_sys::CreateConversionStmt> {
        PgBox::<pg_sys::CreateConversionStmt>::alloc_node(PgNode::CreateConversionStmt)
    }
    pub fn makeCreateCastStmt() -> PgBox<pg_sys::CreateCastStmt> {
        PgBox::<pg_sys::CreateCastStmt>::alloc_node(PgNode::CreateCastStmt)
    }
    pub fn makeCreateOpClassStmt() -> PgBox<pg_sys::CreateOpClassStmt> {
        PgBox::<pg_sys::CreateOpClassStmt>::alloc_node(PgNode::CreateOpClassStmt)
    }
    pub fn makeCreateOpFamilyStmt() -> PgBox<pg_sys::CreateOpFamilyStmt> {
        PgBox::<pg_sys::CreateOpFamilyStmt>::alloc_node(PgNode::CreateOpFamilyStmt)
    }
    pub fn makeAlterOpFamilyStmt() -> PgBox<pg_sys::AlterOpFamilyStmt> {
        PgBox::<pg_sys::AlterOpFamilyStmt>::alloc_node(PgNode::AlterOpFamilyStmt)
    }
    pub fn makePrepareStmt() -> PgBox<pg_sys::PrepareStmt> {
        PgBox::<pg_sys::PrepareStmt>::alloc_node(PgNode::PrepareStmt)
    }
    pub fn makeExecuteStmt() -> PgBox<pg_sys::ExecuteStmt> {
        PgBox::<pg_sys::ExecuteStmt>::alloc_node(PgNode::ExecuteStmt)
    }
    pub fn makeDeallocateStmt() -> PgBox<pg_sys::DeallocateStmt> {
        PgBox::<pg_sys::DeallocateStmt>::alloc_node(PgNode::DeallocateStmt)
    }
    pub fn makeDeclareCursorStmt() -> PgBox<pg_sys::DeclareCursorStmt> {
        PgBox::<pg_sys::DeclareCursorStmt>::alloc_node(PgNode::DeclareCursorStmt)
    }
    pub fn makeCreateTableSpaceStmt() -> PgBox<pg_sys::CreateTableSpaceStmt> {
        PgBox::<pg_sys::CreateTableSpaceStmt>::alloc_node(PgNode::CreateTableSpaceStmt)
    }
    pub fn makeDropTableSpaceStmt() -> PgBox<pg_sys::DropTableSpaceStmt> {
        PgBox::<pg_sys::DropTableSpaceStmt>::alloc_node(PgNode::DropTableSpaceStmt)
    }
    pub fn makeAlterObjectDependsStmt() -> PgBox<pg_sys::AlterObjectDependsStmt> {
        PgBox::<pg_sys::AlterObjectDependsStmt>::alloc_node(PgNode::AlterObjectDependsStmt)
    }
    pub fn makeAlterObjectSchemaStmt() -> PgBox<pg_sys::AlterObjectSchemaStmt> {
        PgBox::<pg_sys::AlterObjectSchemaStmt>::alloc_node(PgNode::AlterObjectSchemaStmt)
    }
    pub fn makeAlterOwnerStmt() -> PgBox<pg_sys::AlterOwnerStmt> {
        PgBox::<pg_sys::AlterOwnerStmt>::alloc_node(PgNode::AlterOwnerStmt)
    }
    pub fn makeAlterOperatorStmt() -> PgBox<pg_sys::AlterOperatorStmt> {
        PgBox::<pg_sys::AlterOperatorStmt>::alloc_node(PgNode::AlterOperatorStmt)
    }
    pub fn makeDropOwnedStmt() -> PgBox<pg_sys::DropOwnedStmt> {
        PgBox::<pg_sys::DropOwnedStmt>::alloc_node(PgNode::DropOwnedStmt)
    }
    pub fn makeReassignOwnedStmt() -> PgBox<pg_sys::ReassignOwnedStmt> {
        PgBox::<pg_sys::ReassignOwnedStmt>::alloc_node(PgNode::ReassignOwnedStmt)
    }
    pub fn makeCompositeTypeStmt() -> PgBox<pg_sys::CompositeTypeStmt> {
        PgBox::<pg_sys::CompositeTypeStmt>::alloc_node(PgNode::CompositeTypeStmt)
    }
    pub fn makeCreateEnumStmt() -> PgBox<pg_sys::CreateEnumStmt> {
        PgBox::<pg_sys::CreateEnumStmt>::alloc_node(PgNode::CreateEnumStmt)
    }
    pub fn makeCreateRangeStmt() -> PgBox<pg_sys::CreateRangeStmt> {
        PgBox::<pg_sys::CreateRangeStmt>::alloc_node(PgNode::CreateRangeStmt)
    }
    pub fn makeAlterEnumStmt() -> PgBox<pg_sys::AlterEnumStmt> {
        PgBox::<pg_sys::AlterEnumStmt>::alloc_node(PgNode::AlterEnumStmt)
    }
    pub fn makeAlterTSDictionaryStmt() -> PgBox<pg_sys::AlterTSDictionaryStmt> {
        PgBox::<pg_sys::AlterTSDictionaryStmt>::alloc_node(PgNode::AlterTSDictionaryStmt)
    }
    pub fn makeAlterTSConfigurationStmt() -> PgBox<pg_sys::AlterTSConfigurationStmt> {
        PgBox::<pg_sys::AlterTSConfigurationStmt>::alloc_node(PgNode::AlterTSConfigurationStmt)
    }
    pub fn makeCreateFdwStmt() -> PgBox<pg_sys::CreateFdwStmt> {
        PgBox::<pg_sys::CreateFdwStmt>::alloc_node(PgNode::CreateFdwStmt)
    }
    pub fn makeAlterFdwStmt() -> PgBox<pg_sys::AlterFdwStmt> {
        PgBox::<pg_sys::AlterFdwStmt>::alloc_node(PgNode::AlterFdwStmt)
    }
    pub fn makeCreateForeignServerStmt() -> PgBox<pg_sys::CreateForeignServerStmt> {
        PgBox::<pg_sys::CreateForeignServerStmt>::alloc_node(PgNode::CreateForeignServerStmt)
    }
    pub fn makeAlterForeignServerStmt() -> PgBox<pg_sys::AlterForeignServerStmt> {
        PgBox::<pg_sys::AlterForeignServerStmt>::alloc_node(PgNode::AlterForeignServerStmt)
    }
    pub fn makeCreateUserMappingStmt() -> PgBox<pg_sys::CreateUserMappingStmt> {
        PgBox::<pg_sys::CreateUserMappingStmt>::alloc_node(PgNode::CreateUserMappingStmt)
    }
    pub fn makeAlterUserMappingStmt() -> PgBox<pg_sys::AlterUserMappingStmt> {
        PgBox::<pg_sys::AlterUserMappingStmt>::alloc_node(PgNode::AlterUserMappingStmt)
    }
    pub fn makeDropUserMappingStmt() -> PgBox<pg_sys::DropUserMappingStmt> {
        PgBox::<pg_sys::DropUserMappingStmt>::alloc_node(PgNode::DropUserMappingStmt)
    }
    pub fn makeAlterTableSpaceOptionsStmt() -> PgBox<pg_sys::AlterTableSpaceOptionsStmt> {
        PgBox::<pg_sys::AlterTableSpaceOptionsStmt>::alloc_node(PgNode::AlterTableSpaceOptionsStmt)
    }
    pub fn makeAlterTableMoveAllStmt() -> PgBox<pg_sys::AlterTableMoveAllStmt> {
        PgBox::<pg_sys::AlterTableMoveAllStmt>::alloc_node(PgNode::AlterTableMoveAllStmt)
    }
    pub fn makeSecLabelStmt() -> PgBox<pg_sys::SecLabelStmt> {
        PgBox::<pg_sys::SecLabelStmt>::alloc_node(PgNode::SecLabelStmt)
    }
    pub fn makeCreateForeignTableStmt() -> PgBox<pg_sys::CreateForeignTableStmt> {
        PgBox::<pg_sys::CreateForeignTableStmt>::alloc_node(PgNode::CreateForeignTableStmt)
    }
    pub fn makeImportForeignSchemaStmt() -> PgBox<pg_sys::ImportForeignSchemaStmt> {
        PgBox::<pg_sys::ImportForeignSchemaStmt>::alloc_node(PgNode::ImportForeignSchemaStmt)
    }
    pub fn makeCreateExtensionStmt() -> PgBox<pg_sys::CreateExtensionStmt> {
        PgBox::<pg_sys::CreateExtensionStmt>::alloc_node(PgNode::CreateExtensionStmt)
    }
    pub fn makeAlterExtensionStmt() -> PgBox<pg_sys::AlterExtensionStmt> {
        PgBox::<pg_sys::AlterExtensionStmt>::alloc_node(PgNode::AlterExtensionStmt)
    }
    pub fn makeAlterExtensionContentsStmt() -> PgBox<pg_sys::AlterExtensionContentsStmt> {
        PgBox::<pg_sys::AlterExtensionContentsStmt>::alloc_node(PgNode::AlterExtensionContentsStmt)
    }
    pub fn makeCreateEventTrigStmt() -> PgBox<pg_sys::CreateEventTrigStmt> {
        PgBox::<pg_sys::CreateEventTrigStmt>::alloc_node(PgNode::CreateEventTrigStmt)
    }
    pub fn makeAlterEventTrigStmt() -> PgBox<pg_sys::AlterEventTrigStmt> {
        PgBox::<pg_sys::AlterEventTrigStmt>::alloc_node(PgNode::AlterEventTrigStmt)
    }
    pub fn makeRefreshMatViewStmt() -> PgBox<pg_sys::RefreshMatViewStmt> {
        PgBox::<pg_sys::RefreshMatViewStmt>::alloc_node(PgNode::RefreshMatViewStmt)
    }
    pub fn makeReplicaIdentityStmt() -> PgBox<pg_sys::ReplicaIdentityStmt> {
        PgBox::<pg_sys::ReplicaIdentityStmt>::alloc_node(PgNode::ReplicaIdentityStmt)
    }
    pub fn makeAlterSystemStmt() -> PgBox<pg_sys::AlterSystemStmt> {
        PgBox::<pg_sys::AlterSystemStmt>::alloc_node(PgNode::AlterSystemStmt)
    }
    pub fn makeCreatePolicyStmt() -> PgBox<pg_sys::CreatePolicyStmt> {
        PgBox::<pg_sys::CreatePolicyStmt>::alloc_node(PgNode::CreatePolicyStmt)
    }
    pub fn makeAlterPolicyStmt() -> PgBox<pg_sys::AlterPolicyStmt> {
        PgBox::<pg_sys::AlterPolicyStmt>::alloc_node(PgNode::AlterPolicyStmt)
    }
    pub fn makeCreateTransformStmt() -> PgBox<pg_sys::CreateTransformStmt> {
        PgBox::<pg_sys::CreateTransformStmt>::alloc_node(PgNode::CreateTransformStmt)
    }
    pub fn makeCreateAmStmt() -> PgBox<pg_sys::CreateAmStmt> {
        PgBox::<pg_sys::CreateAmStmt>::alloc_node(PgNode::CreateAmStmt)
    }
    pub fn makeCreatePublicationStmt() -> PgBox<pg_sys::CreatePublicationStmt> {
        PgBox::<pg_sys::CreatePublicationStmt>::alloc_node(PgNode::CreatePublicationStmt)
    }
    pub fn makeAlterPublicationStmt() -> PgBox<pg_sys::AlterPublicationStmt> {
        PgBox::<pg_sys::AlterPublicationStmt>::alloc_node(PgNode::AlterPublicationStmt)
    }
    pub fn makeCreateSubscriptionStmt() -> PgBox<pg_sys::CreateSubscriptionStmt> {
        PgBox::<pg_sys::CreateSubscriptionStmt>::alloc_node(PgNode::CreateSubscriptionStmt)
    }
    pub fn makeAlterSubscriptionStmt() -> PgBox<pg_sys::AlterSubscriptionStmt> {
        PgBox::<pg_sys::AlterSubscriptionStmt>::alloc_node(PgNode::AlterSubscriptionStmt)
    }
    pub fn makeDropSubscriptionStmt() -> PgBox<pg_sys::DropSubscriptionStmt> {
        PgBox::<pg_sys::DropSubscriptionStmt>::alloc_node(PgNode::DropSubscriptionStmt)
    }
    pub fn makeCreateStatsStmt() -> PgBox<pg_sys::pg12_specific::CreateStatsStmt> {
        PgBox::<pg_sys::pg12_specific::CreateStatsStmt>::alloc_node(PgNode::CreateStatsStmt)
    }
    pub fn makeAlterCollationStmt() -> PgBox<pg_sys::AlterCollationStmt> {
        PgBox::<pg_sys::AlterCollationStmt>::alloc_node(PgNode::AlterCollationStmt)
    }
    pub fn makeCallStmt() -> PgBox<pg_sys::pg12_specific::CallStmt> {
        PgBox::<pg_sys::pg12_specific::CallStmt>::alloc_node(PgNode::CallStmt)
    }

    pub fn makeA_Expr() -> PgBox<pg_sys::A_Expr> {
        PgBox::<pg_sys::A_Expr>::alloc_node(PgNode::A_Expr)
    }
    pub fn makeColumnRef() -> PgBox<pg_sys::ColumnRef> {
        PgBox::<pg_sys::ColumnRef>::alloc_node(PgNode::ColumnRef)
    }
    pub fn makeParamRef() -> PgBox<pg_sys::ParamRef> {
        PgBox::<pg_sys::ParamRef>::alloc_node(PgNode::ParamRef)
    }
    pub fn makeA_Const() -> PgBox<pg_sys::A_Const> {
        PgBox::<pg_sys::A_Const>::alloc_node(PgNode::A_Const)
    }
    pub fn makeFuncCall() -> PgBox<pg_sys::FuncCall> {
        PgBox::<pg_sys::FuncCall>::alloc_node(PgNode::FuncCall)
    }
    pub fn makeA_Star() -> PgBox<pg_sys::A_Star> {
        PgBox::<pg_sys::A_Star>::alloc_node(PgNode::A_Star)
    }
    pub fn makeA_Indices() -> PgBox<pg_sys::A_Indices> {
        PgBox::<pg_sys::A_Indices>::alloc_node(PgNode::A_Indices)
    }
    pub fn makeA_Indirection() -> PgBox<pg_sys::A_Indirection> {
        PgBox::<pg_sys::A_Indirection>::alloc_node(PgNode::A_Indirection)
    }
    pub fn makeA_ArrayExpr() -> PgBox<pg_sys::A_ArrayExpr> {
        PgBox::<pg_sys::A_ArrayExpr>::alloc_node(PgNode::A_ArrayExpr)
    }
    pub fn makeResTarget() -> PgBox<pg_sys::ResTarget> {
        PgBox::<pg_sys::ResTarget>::alloc_node(PgNode::ResTarget)
    }
    pub fn makeMultiAssignRef() -> PgBox<pg_sys::MultiAssignRef> {
        PgBox::<pg_sys::MultiAssignRef>::alloc_node(PgNode::MultiAssignRef)
    }
    pub fn makeTypeCast() -> PgBox<pg_sys::TypeCast> {
        PgBox::<pg_sys::TypeCast>::alloc_node(PgNode::TypeCast)
    }
    pub fn makeCollateClause() -> PgBox<pg_sys::CollateClause> {
        PgBox::<pg_sys::CollateClause>::alloc_node(PgNode::CollateClause)
    }
    pub fn makeSortBy() -> PgBox<pg_sys::SortBy> {
        PgBox::<pg_sys::SortBy>::alloc_node(PgNode::SortBy)
    }
    pub fn makeWindowDef() -> PgBox<pg_sys::WindowDef> {
        PgBox::<pg_sys::WindowDef>::alloc_node(PgNode::WindowDef)
    }
    pub fn makeRangeSubselect() -> PgBox<pg_sys::RangeSubselect> {
        PgBox::<pg_sys::RangeSubselect>::alloc_node(PgNode::RangeSubselect)
    }
    pub fn makeRangeFunction() -> PgBox<pg_sys::RangeFunction> {
        PgBox::<pg_sys::RangeFunction>::alloc_node(PgNode::RangeFunction)
    }
    pub fn makeRangeTableSample() -> PgBox<pg_sys::RangeTableSample> {
        PgBox::<pg_sys::RangeTableSample>::alloc_node(PgNode::RangeTableSample)
    }
    pub fn makeRangeTableFunc() -> PgBox<pg_sys::RangeTableFunc> {
        PgBox::<pg_sys::RangeTableFunc>::alloc_node(PgNode::RangeTableFunc)
    }
    pub fn makeRangeTableFuncCol() -> PgBox<pg_sys::RangeTableFuncCol> {
        PgBox::<pg_sys::RangeTableFuncCol>::alloc_node(PgNode::RangeTableFuncCol)
    }
    pub fn makeTypeName() -> PgBox<pg_sys::TypeName> {
        PgBox::<pg_sys::TypeName>::alloc_node(PgNode::TypeName)
    }
    pub fn makeColumnDef() -> PgBox<pg_sys::pg12_specific::ColumnDef> {
        PgBox::<pg_sys::pg12_specific::ColumnDef>::alloc_node(PgNode::ColumnDef)
    }
    pub fn makeIndexElem() -> PgBox<pg_sys::IndexElem> {
        PgBox::<pg_sys::IndexElem>::alloc_node(PgNode::IndexElem)
    }
    pub fn makeConstraint() -> PgBox<pg_sys::pg12_specific::Constraint> {
        PgBox::<pg_sys::pg12_specific::Constraint>::alloc_node(PgNode::Constraint)
    }
    pub fn makeDefElem() -> PgBox<pg_sys::DefElem> {
        PgBox::<pg_sys::DefElem>::alloc_node(PgNode::DefElem)
    }
    pub fn makeRangeTblEntry() -> PgBox<pg_sys::pg12_specific::RangeTblEntry> {
        PgBox::<pg_sys::pg12_specific::RangeTblEntry>::alloc_node(PgNode::RangeTblEntry)
    }
    pub fn makeRangeTblFunction() -> PgBox<pg_sys::RangeTblFunction> {
        PgBox::<pg_sys::RangeTblFunction>::alloc_node(PgNode::RangeTblFunction)
    }
    pub fn makeTableSampleClause() -> PgBox<pg_sys::TableSampleClause> {
        PgBox::<pg_sys::TableSampleClause>::alloc_node(PgNode::TableSampleClause)
    }
    pub fn makeWithCheckOption() -> PgBox<pg_sys::WithCheckOption> {
        PgBox::<pg_sys::WithCheckOption>::alloc_node(PgNode::WithCheckOption)
    }
    pub fn makeSortGroupClause() -> PgBox<pg_sys::SortGroupClause> {
        PgBox::<pg_sys::SortGroupClause>::alloc_node(PgNode::SortGroupClause)
    }
    pub fn makeGroupingSet() -> PgBox<pg_sys::GroupingSet> {
        PgBox::<pg_sys::GroupingSet>::alloc_node(PgNode::GroupingSet)
    }
    pub fn makeWindowClause() -> PgBox<pg_sys::pg12_specific::WindowClause> {
        PgBox::<pg_sys::pg12_specific::WindowClause>::alloc_node(PgNode::WindowClause)
    }
    pub fn makeObjectWithArgs() -> PgBox<pg_sys::ObjectWithArgs> {
        PgBox::<pg_sys::ObjectWithArgs>::alloc_node(PgNode::ObjectWithArgs)
    }
    pub fn makeAccessPriv() -> PgBox<pg_sys::AccessPriv> {
        PgBox::<pg_sys::AccessPriv>::alloc_node(PgNode::AccessPriv)
    }
    pub fn makeCreateOpClassItem() -> PgBox<pg_sys::CreateOpClassItem> {
        PgBox::<pg_sys::CreateOpClassItem>::alloc_node(PgNode::CreateOpClassItem)
    }
    pub fn makeTableLikeClause() -> PgBox<pg_sys::TableLikeClause> {
        PgBox::<pg_sys::TableLikeClause>::alloc_node(PgNode::TableLikeClause)
    }
    pub fn makeFunctionParameter() -> PgBox<pg_sys::FunctionParameter> {
        PgBox::<pg_sys::FunctionParameter>::alloc_node(PgNode::FunctionParameter)
    }
    pub fn makeLockingClause() -> PgBox<pg_sys::LockingClause> {
        PgBox::<pg_sys::LockingClause>::alloc_node(PgNode::LockingClause)
    }
    pub fn makeRowMarkClause() -> PgBox<pg_sys::RowMarkClause> {
        PgBox::<pg_sys::RowMarkClause>::alloc_node(PgNode::RowMarkClause)
    }
    pub fn makeXmlSerialize() -> PgBox<pg_sys::XmlSerialize> {
        PgBox::<pg_sys::XmlSerialize>::alloc_node(PgNode::XmlSerialize)
    }
    pub fn makeWithClause() -> PgBox<pg_sys::WithClause> {
        PgBox::<pg_sys::WithClause>::alloc_node(PgNode::WithClause)
    }
    pub fn makeInferClause() -> PgBox<pg_sys::InferClause> {
        PgBox::<pg_sys::InferClause>::alloc_node(PgNode::InferClause)
    }
    pub fn makeOnConflictClause() -> PgBox<pg_sys::OnConflictClause> {
        PgBox::<pg_sys::OnConflictClause>::alloc_node(PgNode::OnConflictClause)
    }
    pub fn makeCommonTableExpr() -> PgBox<pg_sys::pg12_specific::CommonTableExpr> {
        PgBox::<pg_sys::pg12_specific::CommonTableExpr>::alloc_node(PgNode::CommonTableExpr)
    }
    pub fn makeRoleSpec() -> PgBox<pg_sys::RoleSpec> {
        PgBox::<pg_sys::RoleSpec>::alloc_node(PgNode::RoleSpec)
    }
    pub fn makeTriggerTransition() -> PgBox<pg_sys::TriggerTransition> {
        PgBox::<pg_sys::TriggerTransition>::alloc_node(PgNode::TriggerTransition)
    }
    pub fn makePartitionElem() -> PgBox<pg_sys::PartitionElem> {
        PgBox::<pg_sys::PartitionElem>::alloc_node(PgNode::PartitionElem)
    }
    pub fn makePartitionSpec() -> PgBox<pg_sys::PartitionSpec> {
        PgBox::<pg_sys::PartitionSpec>::alloc_node(PgNode::PartitionSpec)
    }
    pub fn makePartitionBoundSpec() -> PgBox<pg_sys::pg12_specific::PartitionBoundSpec> {
        PgBox::<pg_sys::pg12_specific::PartitionBoundSpec>::alloc_node(PgNode::PartitionBoundSpec)
    }
    pub fn makePartitionRangeDatum() -> PgBox<pg_sys::PartitionRangeDatum> {
        PgBox::<pg_sys::PartitionRangeDatum>::alloc_node(PgNode::PartitionRangeDatum)
    }
    pub fn makePartitionCmd() -> PgBox<pg_sys::PartitionCmd> {
        PgBox::<pg_sys::PartitionCmd>::alloc_node(PgNode::PartitionCmd)
    }
    pub fn makeVacuumRelation() -> PgBox<pg_sys::pg12_specific::VacuumRelation> {
        PgBox::<pg_sys::pg12_specific::VacuumRelation>::alloc_node(PgNode::VacuumRelation)
    }

    pub fn makeIdentifySystemCmd() -> PgBox<pg_sys::IdentifySystemCmd> {
        PgBox::<pg_sys::IdentifySystemCmd>::alloc_node(PgNode::IdentifySystemCmd)
    }
    pub fn makeBaseBackupCmd() -> PgBox<pg_sys::BaseBackupCmd> {
        PgBox::<pg_sys::BaseBackupCmd>::alloc_node(PgNode::BaseBackupCmd)
    }
    pub fn makeCreateReplicationSlotCmd() -> PgBox<pg_sys::CreateReplicationSlotCmd> {
        PgBox::<pg_sys::CreateReplicationSlotCmd>::alloc_node(PgNode::CreateReplicationSlotCmd)
    }
    pub fn makeDropReplicationSlotCmd() -> PgBox<pg_sys::DropReplicationSlotCmd> {
        PgBox::<pg_sys::DropReplicationSlotCmd>::alloc_node(PgNode::DropReplicationSlotCmd)
    }
    pub fn makeStartReplicationCmd() -> PgBox<pg_sys::StartReplicationCmd> {
        PgBox::<pg_sys::StartReplicationCmd>::alloc_node(PgNode::StartReplicationCmd)
    }
    pub fn makeTimeLineHistoryCmd() -> PgBox<pg_sys::TimeLineHistoryCmd> {
        PgBox::<pg_sys::TimeLineHistoryCmd>::alloc_node(PgNode::TimeLineHistoryCmd)
    }
    pub fn makeSQLCmd() -> PgBox<pg_sys::SQLCmd> {
        PgBox::<pg_sys::SQLCmd>::alloc_node(PgNode::SQLCmd)
    }

    pub fn makeTriggerData() -> PgBox<pg_sys::pg12_specific::TriggerData> {
        PgBox::<pg_sys::pg12_specific::TriggerData>::alloc_node(PgNode::TriggerData)
    }

    pub fn makeEventTriggerData() -> PgBox<pg_sys::EventTriggerData> {
        PgBox::<pg_sys::EventTriggerData>::alloc_node(PgNode::EventTriggerData)
    }

    pub fn makeReturnSetInfo() -> PgBox<pg_sys::ReturnSetInfo> {
        PgBox::<pg_sys::ReturnSetInfo>::alloc_node(PgNode::ReturnSetInfo)
    }

    pub fn makeWindowObjectData() -> PgBox<pg_sys::WindowObjectData> {
        PgBox::<pg_sys::WindowObjectData>::alloc_node(PgNode::WindowObjectData)
    }

    pub fn makeTIDBitmap() -> PgBox<pg_sys::TIDBitmap> {
        PgBox::<pg_sys::TIDBitmap>::alloc_node(PgNode::TIDBitmap)
    }

    pub fn makeInlineCodeBlock() -> PgBox<pg_sys::pg12_specific::InlineCodeBlock> {
        PgBox::<pg_sys::pg12_specific::InlineCodeBlock>::alloc_node(PgNode::InlineCodeBlock)
    }

    pub fn makeFdwRoutine() -> PgBox<pg_sys::FdwRoutine> {
        PgBox::<pg_sys::FdwRoutine>::alloc_node(PgNode::FdwRoutine)
    }

    pub fn makeIndexAmRoutine() -> PgBox<pg_sys::IndexAmRoutine> {
        PgBox::<pg_sys::IndexAmRoutine>::alloc_node(PgNode::IndexAmRoutine)
    }

    pub fn makeTableAmRoutine() -> PgBox<pg_sys::pg12_specific::TableAmRoutine> {
        PgBox::<pg_sys::pg12_specific::TableAmRoutine>::alloc_node(PgNode::TableAmRoutine)
    }

    pub fn makeTsmRoutine() -> PgBox<pg_sys::TsmRoutine> {
        PgBox::<pg_sys::TsmRoutine>::alloc_node(PgNode::TsmRoutine)
    }

    pub fn makeForeignKeyCacheInfo() -> PgBox<pg_sys::pg12_specific::ForeignKeyCacheInfo> {
        PgBox::<pg_sys::pg12_specific::ForeignKeyCacheInfo>::alloc_node(PgNode::ForeignKeyCacheInfo)
    }

    pub fn makeCallContext() -> PgBox<pg_sys::pg12_specific::CallContext> {
        PgBox::<pg_sys::pg12_specific::CallContext>::alloc_node(PgNode::CallContext)
    }

    pub fn makeSupportRequestSimplify() -> PgBox<pg_sys::pg12_specific::SupportRequestSimplify> {
        PgBox::<pg_sys::pg12_specific::SupportRequestSimplify>::alloc_node(
            PgNode::SupportRequestSimplify,
        )
    }

    pub fn makeSupportRequestSelectivity() -> PgBox<pg_sys::pg12_specific::SupportRequestSelectivity>
    {
        PgBox::<pg_sys::pg12_specific::SupportRequestSelectivity>::alloc_node(
            PgNode::SupportRequestSelectivity,
        )
    }

    pub fn makeSupportRequestCost() -> PgBox<pg_sys::pg12_specific::SupportRequestCost> {
        PgBox::<pg_sys::pg12_specific::SupportRequestCost>::alloc_node(PgNode::SupportRequestCost)
    }

    pub fn makeSupportRequestRows() -> PgBox<pg_sys::pg12_specific::SupportRequestRows> {
        PgBox::<pg_sys::pg12_specific::SupportRequestRows>::alloc_node(PgNode::SupportRequestRows)
    }

    pub fn makeSupportRequestIndexCondition(
    ) -> PgBox<pg_sys::pg12_specific::SupportRequestIndexCondition> {
        PgBox::<pg_sys::pg12_specific::SupportRequestIndexCondition>::alloc_node(
            PgNode::SupportRequestIndexCondition,
        )
    }
}
