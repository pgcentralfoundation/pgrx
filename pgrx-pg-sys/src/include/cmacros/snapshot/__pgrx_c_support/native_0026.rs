// Shared generated C macro support.

#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MaterialPath>() == 80);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MaterialPath>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::MaterialPath {}
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Material>() == 104);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Material>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::Material {}
impl c::sealed::Sealed for crate::__pgrx_c_bindings::LogicalTapeSet {}
impl c::expression::NativeType for crate::__pgrx_c_bindings::LogicalTapeSet {
    type Marker = c::expression::COpaque<Self>;
}
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LogicalRewriteMappingData>() == 36);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LogicalRewriteMappingData>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::LogicalRewriteMappingData {}
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LockRowsState>() == 296);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LockRowsState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::LockRowsState {}
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LockInfoData>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LockInfoData>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::LockInfoData {}
#[cfg(feature = "pg15")]
impl c::sealed::Sealed for crate::__pgrx_c_bindings::KeyActions {}
#[cfg(feature = "pg15")]
impl c::expression::NativeType for crate::__pgrx_c_bindings::KeyActions {
    type Marker = c::expression::COpaque<Self>;
}
#[cfg(feature = "pg15")]
impl c::sealed::Sealed for crate::__pgrx_c_bindings::KeyAction {}
#[cfg(feature = "pg15")]
impl c::expression::NativeType for crate::__pgrx_c_bindings::KeyAction {
    type Marker = c::expression::COpaque<Self>;
}
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JitProviderCallbacks>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JitProviderCallbacks>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JitProviderCallbacks {}
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::IndexBuildResult>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::IndexBuildResult>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::IndexBuildResult {}
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::IndexAttributeBitMapData>() == 4);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::IndexAttributeBitMapData>() == 1);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::IndexAttributeBitMapData {}
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::IncrementalSortInfo>() == 96);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::IncrementalSortInfo>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::IncrementalSortInfo {}
#[cfg(feature = "pg15")]
impl c::sealed::Sealed for crate::__pgrx_c_bindings::ImportQual {}
#[cfg(feature = "pg15")]
impl c::expression::NativeType for crate::__pgrx_c_bindings::ImportQual {
    type Marker = c::expression::COpaque<Self>;
}
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::HashJoin>() == 152);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::HashJoin>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::HashJoin {}
impl c::sealed::Sealed for crate::__pgrx_c_bindings::HashAggSpill {}
impl c::expression::NativeType for crate::__pgrx_c_bindings::HashAggSpill {
    type Marker = c::expression::COpaque<Self>;
}
impl c::sealed::Sealed for crate::__pgrx_c_bindings::HTAB {}
impl c::expression::NativeType for crate::__pgrx_c_bindings::HTAB {
    type Marker = c::expression::COpaque<Self>;
}
impl c::sealed::Sealed for crate::__pgrx_c_bindings::HSpool {}
impl c::expression::NativeType for crate::__pgrx_c_bindings::HSpool {
    type Marker = c::expression::COpaque<Self>;
}
impl c::sealed::Sealed for crate::__pgrx_c_bindings::HASHHDR {}
impl c::expression::NativeType for crate::__pgrx_c_bindings::HASHHDR {
    type Marker = c::expression::COpaque<Self>;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::GroupResultPath>() == 80);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::GroupResultPath>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::GroupResultPath {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::GroupPath>() == 96);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::GroupPath>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::GroupPath {}
#[cfg(feature = "pg15")]
impl c::sealed::Sealed for crate::__pgrx_c_bindings::GroupClause {}
#[cfg(feature = "pg15")]
impl c::expression::NativeType for crate::__pgrx_c_bindings::GroupClause {
    type Marker = c::expression::COpaque<Self>;
}
impl c::sealed::Sealed for crate::__pgrx_c_bindings::GlobalTransactionData {}
impl c::expression::NativeType for crate::__pgrx_c_bindings::GlobalTransactionData {
    type Marker = c::expression::COpaque<Self>;
}
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::GistSplitVector>() == 648);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::GistSplitVector>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::GistSplitVector {}
impl c::sealed::Sealed for crate::__pgrx_c_bindings::GinVacuumState {}
impl c::expression::NativeType for crate::__pgrx_c_bindings::GinVacuumState {
    type Marker = c::expression::COpaque<Self>;
}
impl c::sealed::Sealed for crate::__pgrx_c_bindings::GenericXLogState {}
impl c::expression::NativeType for crate::__pgrx_c_bindings::GenericXLogState {
    type Marker = c::expression::COpaque<Self>;
}
impl c::sealed::Sealed for crate::__pgrx_c_bindings::GMReaderTupleBuffer {}
impl c::expression::NativeType for crate::__pgrx_c_bindings::GMReaderTupleBuffer {
    type Marker = c::expression::COpaque<Self>;
}
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::GISTSTATE>() == 13992);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::GISTSTATE>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::GISTSTATE {}
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::GISTDeletedPageContents>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::GISTDeletedPageContents>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::GISTDeletedPageContents {}
impl c::sealed::Sealed for crate::__pgrx_c_bindings::FunctionScanPerFuncState {}
impl c::expression::NativeType for crate::__pgrx_c_bindings::FunctionScanPerFuncState {
    type Marker = c::expression::COpaque<Self>;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ForeignPath>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ForeignPath>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ForeignPath {}
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_37>(
        ) == 8
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_37>(
        ) == 8
    );
};
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_37
{
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_36>(
        ) == 8
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_36>(
        ) == 8
    );
};
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_36
{
}
#[cfg(feature = "pg15")]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_35>(
        ) == 8
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_35>(
        ) == 8
    );
};
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_35
{
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_33>(
        ) == 40
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_33>(
        ) == 8
    );
};
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_33
{
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_28>(
        ) == 16
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_28>(
        ) == 8
    );
};
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_28
{
}
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_21>(
        ) == 24
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_21>(
        ) == 8
    );
};
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_21
{
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_17>(
        ) == 8
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_17>(
        ) == 8
    );
};
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_17
{
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_16>(
        ) == 32
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_16>(
        ) == 8
    );
};
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_16
{
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_11>(
        ) == 16
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_11>(
        ) == 8
    );
};
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_11
{
}
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ExpandedObjectMethods>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ExpandedObjectMethods>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ExpandedObjectMethods {}
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ErrorContextCallback>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ErrorContextCallback>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ErrorContextCallback {}
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::EphemeralNamedRelationData>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::EphemeralNamedRelationData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::EphemeralNamedRelationData {}
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::EPQStateExtra>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::EPQStateExtra>() == 8);
};
#[cfg(feature = "pg15")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::EPQStateExtra {}
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::DynamicZoneAbbrev>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::DynamicZoneAbbrev>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::DynamicZoneAbbrev {}
impl c::sealed::Sealed for crate::__pgrx_c_bindings::DomainConstraintCache {}
impl c::expression::NativeType for crate::__pgrx_c_bindings::DomainConstraintCache {
    type Marker = c::expression::COpaque<Self>;
}
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CustomScanMethods>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CustomScanMethods>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CustomScanMethods {}
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CustomPathMethods>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CustomPathMethods>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CustomPathMethods {}
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CustomExecMethods>() == 104);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CustomExecMethods>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CustomExecMethods {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CreateForeignTableStmt>() == 120);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CreateForeignTableStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CreateForeignTableStmt {}
impl c::sealed::Sealed for crate::__pgrx_c_bindings::CopyToStateData {}
impl c::expression::NativeType for crate::__pgrx_c_bindings::CopyToStateData {
    type Marker = c::expression::COpaque<Self>;
}
impl c::sealed::Sealed for crate::__pgrx_c_bindings::CopyMultiInsertBuffer {}
impl c::expression::NativeType for crate::__pgrx_c_bindings::CopyMultiInsertBuffer {
    type Marker = c::expression::COpaque<Self>;
}
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ConfigData>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ConfigData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ConfigData {}
const _: () = {
    assert!(
        ::core::mem::size_of::<
            crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_5,
        >() == 32
    );
    assert!(
        ::core::mem::align_of::<
            crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_5,
        >() == 8
    );
};
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_5
{
}
const _: () = {
    assert!(
        ::core::mem::size_of::<
            crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_4,
        >() == 32
    );
    assert!(
        ::core::mem::align_of::<
            crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_4,
        >() == 8
    );
};
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_4
{
}
const _: () = {
    assert!(
        ::core::mem::size_of::<
            crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_3,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_3,
        >() == 8
    );
};
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_3
{
}
const _: () = {
    assert!(
        ::core::mem::size_of::<
            crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_1,
        >() == 24
    );
    assert!(
        ::core::mem::align_of::<
            crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_1,
        >() == 4
    );
};
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_1
{
}
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CollectedATSubcmd>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CollectedATSubcmd>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CollectedATSubcmd {}
impl c::sealed::Sealed for crate::__pgrx_c_bindings::BufFile {}
impl c::expression::NativeType for crate::__pgrx_c_bindings::BufFile {
    type Marker = c::expression::COpaque<Self>;
}
impl c::sealed::Sealed for crate::__pgrx_c_bindings::BrinRevmap {}
impl c::expression::NativeType for crate::__pgrx_c_bindings::BrinRevmap {
    type Marker = c::expression::COpaque<Self>;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BitmapOrPath>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BitmapOrPath>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BitmapOrPath {}
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BitmapHeapScan>() == 120);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BitmapHeapScan>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BitmapHeapScan {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BitmapHeapPath>() == 80);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BitmapHeapPath>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BitmapHeapPath {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BitmapAndPath>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BitmapAndPath>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BitmapAndPath {}
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BitmapAnd>() == 112);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BitmapAnd>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BitmapAnd {}
impl c::sealed::Sealed for crate::__pgrx_c_bindings::BackgroundWorkerHandle {}
impl c::expression::NativeType for crate::__pgrx_c_bindings::BackgroundWorkerHandle {
    type Marker = c::expression::COpaque<Self>;
}
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BTDeletedPageData>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BTDeletedPageData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BTDeletedPageData {}
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AttInMetadata>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AttInMetadata>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AttInMetadata {}
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ArrayMapState>() == 144);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ArrayMapState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ArrayMapState {}
impl c::sealed::Sealed for crate::__pgrx_c_bindings::ArrayIteratorData {}
impl c::expression::NativeType for crate::__pgrx_c_bindings::ArrayIteratorData {
    type Marker = c::expression::COpaque<Self>;
}
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ArrayBuildStateAny>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ArrayBuildStateAny>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ArrayBuildStateAny {}
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ArchiveModuleCallbacks>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ArchiveModuleCallbacks>() == 8);
};
#[cfg(feature = "pg15")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ArchiveModuleCallbacks {}
impl c::sealed::Sealed for crate::__pgrx_c_bindings::AfterTriggersTableData {}
impl c::expression::NativeType for crate::__pgrx_c_bindings::AfterTriggersTableData {
    type Marker = c::expression::COpaque<Self>;
}
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::__sigset_t>() == 128);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::__sigset_t>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::__sigset_t {}
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ReservoirStateData>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ReservoirStateData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ReservoirStateData {}
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Point>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Point>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::Point {}
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PipeProtoChunk>() == 4096);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PipeProtoChunk>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PipeProtoChunk {}
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LSEG>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LSEG>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::LSEG {}
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LINE>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LINE>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::LINE {}
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::GeqoPrivateData>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::GeqoPrivateData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::GeqoPrivateData {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::GenericCosts>() == 64);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::GenericCosts>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::GenericCosts {}
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CIRCLE>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CIRCLE>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CIRCLE {}
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BOX>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BOX>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BOX {}
const _: () = assert!(::core::mem::size_of::<*mut ::core::ffi::c_char>() == 8);
const _: () = assert!(::core::mem::align_of::<*mut ::core::ffi::c_char>() == 8);
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>>()
        == 8
);
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>>(
    ) == 8
);
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__BTPageGetDeleteXid"]
    fn raw_inline_BTPageGetDeleteXid(
        arg0: *mut ::core::ffi::c_char,
    ) -> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>;
}
#[doc = "Call the original C function `BTPageGetDeleteXid`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_BTPageGetDeleteXid(
    arg0: *mut ::core::ffi::c_char,
) -> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId> {
    const _: () = assert!(!::core::mem::needs_drop::<*mut ::core::ffi::c_char>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_BTPageGetDeleteXid(arg0)) }
}
const _: () = assert!(::core::mem::size_of::<*mut ::core::ffi::c_char>() == 8);
const _: () = assert!(::core::mem::align_of::<*mut ::core::ffi::c_char>() == 8);
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>>()
        == 8
);
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>>(
    ) == 8
);
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__BTPageSetDeleted"]
    fn raw_inline_BTPageSetDeleted(
        arg0: *mut ::core::ffi::c_char,
        arg1: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>,
    ) -> ();
}
#[doc = "Call the original C function `BTPageSetDeleted`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_BTPageSetDeleted(
    arg0: *mut ::core::ffi::c_char,
    arg1: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>,
) -> () {
    const _: () = assert!(!::core::mem::needs_drop::<*mut ::core::ffi::c_char>());
    const _: () = assert!(!::core::mem::needs_drop::<
        ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>,
    >());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_BTPageSetDeleted(arg0, arg1)) }
}
const _: () = assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::Node>() == 8);
const _: () = assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::Node>() == 8);
const _: () = assert!(::core::mem::size_of::<u32>() == 4);
const _: () = assert!(::core::mem::align_of::<u32>() == 4);
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__CreateCommandTag"]
    fn raw_inline_CreateCommandTag(arg0: *mut crate::__pgrx_c_bindings::Node) -> u32;
}
#[doc = "Call the original C function `CreateCommandTag`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_CreateCommandTag(arg0: *mut crate::__pgrx_c_bindings::Node) -> u32 {
    const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::Node>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_CreateCommandTag(arg0)) }
}
const _: () = assert!(::core::mem::size_of::<u32>() == 4);
const _: () = assert!(::core::mem::align_of::<u32>() == 4);
const _: () = assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TransactionId>() == 4);
const _: () = assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TransactionId>() == 4);
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>>()
        == 8
);
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>>(
    ) == 8
);
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__FullTransactionIdFromEpochAndXid"]
    fn raw_inline_FullTransactionIdFromEpochAndXid(
        arg0: u32,
        arg1: crate::__pgrx_c_bindings::TransactionId,
    ) -> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>;
}
#[doc = "Call the original C function `FullTransactionIdFromEpochAndXid`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_FullTransactionIdFromEpochAndXid(
    arg0: u32,
    arg1: crate::__pgrx_c_bindings::TransactionId,
) -> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId> {
    const _: () = assert!(!::core::mem::needs_drop::<u32>());
    const _: () = assert!(!::core::mem::needs_drop::<crate::__pgrx_c_bindings::TransactionId>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe {
        crate::ffi::pg_guard_ffi_boundary(move || {
            raw_inline_FullTransactionIdFromEpochAndXid(arg0, arg1)
        })
    }
}
const _: () = assert!(::core::mem::size_of::<u64>() == 8);
const _: () = assert!(::core::mem::align_of::<u64>() == 8);
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>>()
        == 8
);
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>>(
    ) == 8
);
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__FullTransactionIdFromU64"]
    fn raw_inline_FullTransactionIdFromU64(
        arg0: u64,
    ) -> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>;
}
#[doc = "Call the original C function `FullTransactionIdFromU64`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_FullTransactionIdFromU64(
    arg0: u64,
) -> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId> {
    const _: () = assert!(!::core::mem::needs_drop::<u64>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_FullTransactionIdFromU64(arg0)) }
}
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>>()
        == 8
);
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>>(
    ) == 8
);
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>>()
        == 8
);
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>>(
    ) == 8
);
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>>()
        == 8
);
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>>(
    ) == 8
);
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__FullTransactionIdNewer"]
    fn raw_inline_FullTransactionIdNewer(
        arg0: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>,
        arg1: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>,
    ) -> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>;
}
#[doc = "Call the original C function `FullTransactionIdNewer`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_FullTransactionIdNewer(
    arg0: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>,
    arg1: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>,
) -> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId> {
    const _: () = assert!(!::core::mem::needs_drop::<
        ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>,
    >());
    const _: () = assert!(!::core::mem::needs_drop::<
        ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>,
    >());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe {
        crate::ffi::pg_guard_ffi_boundary(move || raw_inline_FullTransactionIdNewer(arg0, arg1))
    }
}
const _: () = assert!(::core::mem::size_of::<u32>() == 4);
const _: () = assert!(::core::mem::align_of::<u32>() == 4);
const _: () = assert!(::core::mem::size_of::<*const ::core::ffi::c_char>() == 8);
const _: () = assert!(::core::mem::align_of::<*const ::core::ffi::c_char>() == 8);
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__GetCommandTagName"]
    fn raw_inline_GetCommandTagName(arg0: u32) -> *const ::core::ffi::c_char;
}
#[doc = "Call the original C function `GetCommandTagName`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_GetCommandTagName(arg0: u32) -> *const ::core::ffi::c_char {
    const _: () = assert!(!::core::mem::needs_drop::<u32>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_GetCommandTagName(arg0)) }
}
const _: () = assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Oid>() == 4);
const _: () = assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Oid>() == 4);
const _: () = assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Oid>() == 4);
const _: () = assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Oid>() == 4);
const _: () = assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Oid>() == 4);
const _: () = assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Oid>() == 4);
const _: () = assert!(::core::mem::size_of::<i32>() == 4);
const _: () = assert!(::core::mem::align_of::<i32>() == 4);
const _: () = assert!(::core::mem::size_of::<i32>() == 4);
const _: () = assert!(::core::mem::align_of::<i32>() == 4);
const _: () = assert!(::core::mem::size_of::<*mut ::core::ffi::c_char>() == 8);
const _: () = assert!(::core::mem::align_of::<*mut ::core::ffi::c_char>() == 8);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__GetRelationPath"]
    fn raw_inline_GetRelationPath(
        arg0: crate::__pgrx_c_bindings::Oid,
        arg1: crate::__pgrx_c_bindings::Oid,
        arg2: crate::__pgrx_c_bindings::Oid,
        arg3: i32,
        arg4: i32,
    ) -> *mut ::core::ffi::c_char;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
#[doc = "Call the original C function `GetRelationPath`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_GetRelationPath(
    arg0: crate::__pgrx_c_bindings::Oid,
    arg1: crate::__pgrx_c_bindings::Oid,
    arg2: crate::__pgrx_c_bindings::Oid,
    arg3: i32,
    arg4: i32,
) -> *mut ::core::ffi::c_char {
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
const _: () = assert!(::core::mem::size_of::<u8>() == 1);
const _: () = assert!(::core::mem::align_of::<u8>() == 1);
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::RmgrData>>() == 64
);
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::RmgrData>>() == 8
);
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__GetRmgr"]
    fn raw_inline_GetRmgr(arg0: u8)
    -> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::RmgrData>;
}
#[doc = "Call the original C function `GetRmgr`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_GetRmgr(
    arg0: u8,
) -> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::RmgrData> {
    const _: () = assert!(!::core::mem::needs_drop::<u8>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_GetRmgr(arg0)) }
}
const _: () = assert!(::core::mem::size_of::<*mut ::core::ffi::c_char>() == 8);
const _: () = assert!(::core::mem::align_of::<*mut ::core::ffi::c_char>() == 8);
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>>()
        == 8
);
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>>(
    ) == 8
);
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__GistPageGetDeleteXid"]
    fn raw_inline_GistPageGetDeleteXid(
        arg0: *mut ::core::ffi::c_char,
    ) -> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>;
}
#[doc = "Call the original C function `GistPageGetDeleteXid`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_GistPageGetDeleteXid(
    arg0: *mut ::core::ffi::c_char,
) -> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId> {
    const _: () = assert!(!::core::mem::needs_drop::<*mut ::core::ffi::c_char>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_GistPageGetDeleteXid(arg0)) }
}
const _: () = assert!(::core::mem::size_of::<*mut ::core::ffi::c_char>() == 8);
const _: () = assert!(::core::mem::align_of::<*mut ::core::ffi::c_char>() == 8);
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>>()
        == 8
);
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>>(
    ) == 8
);
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__GistPageSetDeleted"]
    fn raw_inline_GistPageSetDeleted(
        arg0: *mut ::core::ffi::c_char,
        arg1: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>,
    ) -> ();
}
#[doc = "Call the original C function `GistPageSetDeleted`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_GistPageSetDeleted(
    arg0: *mut ::core::ffi::c_char,
    arg1: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>,
) -> () {
    const _: () = assert!(!::core::mem::needs_drop::<*mut ::core::ffi::c_char>());
    const _: () = assert!(!::core::mem::needs_drop::<
        ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>,
    >());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_GistPageSetDeleted(arg0, arg1)) }
}
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>>()
        == 8
);
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>>(
    ) == 8
);
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__ReadNextFullTransactionId"]
    fn raw_inline_ReadNextFullTransactionId()
    -> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>;
}
#[doc = "Call the original C function `ReadNextFullTransactionId`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_ReadNextFullTransactionId()
-> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId> {
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_ReadNextFullTransactionId()) }
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::RelationData>() == 8);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::RelationData>() == 8);
const _: () = assert!(::core::mem::size_of::<i32>() == 4);
const _: () = assert!(::core::mem::align_of::<i32>() == 4);
const _: () = assert!(::core::mem::size_of::<u32>() == 4);
const _: () = assert!(::core::mem::align_of::<u32>() == 4);
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__RelationGetNumberOfBlocksInFork"]
    fn raw_inline_RelationGetNumberOfBlocksInFork(
        arg0: *mut crate::__pgrx_c_bindings::RelationData,
        arg1: i32,
    ) -> u32;
}
#[doc = "Call the original C function `RelationGetNumberOfBlocksInFork`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_RelationGetNumberOfBlocksInFork(
    arg0: *mut crate::__pgrx_c_bindings::RelationData,
    arg1: i32,
) -> u32 {
    const _: () =
        assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::RelationData>());
    const _: () = assert!(!::core::mem::needs_drop::<i32>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe {
        crate::ffi::pg_guard_ffi_boundary(move || {
            raw_inline_RelationGetNumberOfBlocksInFork(arg0, arg1)
        })
    }
}
const _: () = assert!(::core::mem::size_of::<*mut *const u16>() == 8);
const _: () = assert!(::core::mem::align_of::<*mut *const u16>() == 8);
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__05f5f63747970655f625f6c6f63"]
    fn raw_inline___ctype_b_loc() -> *mut *const u16;
}
#[doc = "Call the original C function `__ctype_b_loc`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline___ctype_b_loc() -> *mut *const u16 {
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline___ctype_b_loc()) }
}
const _: () = assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::StringInfoData>() == 8);
const _: () =
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::StringInfoData>() == 8);
const _: () = assert!(::core::mem::size_of::<i8>() == 1);
const _: () = assert!(::core::mem::align_of::<i8>() == 1);
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__appendStringInfoChar"]
    fn raw_inline_appendStringInfoChar(
        arg0: *mut crate::__pgrx_c_bindings::StringInfoData,
        arg1: i8,
    ) -> ();
}
#[doc = "Call the original C function `appendStringInfoChar`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_appendStringInfoChar(
    arg0: *mut crate::__pgrx_c_bindings::StringInfoData,
    arg1: i8,
) -> () {
    const _: () =
        assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::StringInfoData>());
    const _: () = assert!(!::core::mem::needs_drop::<i8>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe {
        crate::ffi::pg_guard_ffi_boundary(move || raw_inline_appendStringInfoChar(arg0, arg1))
    }
}
const _: () = assert!(::core::mem::size_of::<*const i8>() == 8);
const _: () = assert!(::core::mem::align_of::<*const i8>() == 8);
const _: () = assert!(::core::mem::size_of::<f64>() == 8);
const _: () = assert!(::core::mem::align_of::<f64>() == 8);
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__atof"]
    fn raw_inline_atof(arg0: *const i8) -> f64;
}
#[doc = "Call the original C function `atof`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_atof(arg0: *const i8) -> f64 {
    const _: () = assert!(!::core::mem::needs_drop::<*const i8>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_atof(arg0)) }
}
const _: () = assert!(::core::mem::size_of::<f64>() == 8);
const _: () = assert!(::core::mem::align_of::<f64>() == 8);
const _: () = assert!(::core::mem::size_of::<f64>() == 8);
const _: () = assert!(::core::mem::align_of::<f64>() == 8);
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__ceil"]
    fn raw_inline_ceil(arg0: f64) -> f64;
}
#[doc = "Call the original C function `ceil`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_ceil(arg0: f64) -> f64 {
    const _: () = assert!(!::core::mem::needs_drop::<f64>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_ceil(arg0)) }
}
const _: () = assert!(::core::mem::size_of::<i32>() == 4);
const _: () = assert!(::core::mem::align_of::<i32>() == 4);
#[cfg(feature = "pg15")]
const _: () = assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::timespec>() == 8);
#[cfg(feature = "pg15")]
const _: () = assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::timespec>() == 8);
const _: () = assert!(::core::mem::size_of::<i32>() == 4);
const _: () = assert!(::core::mem::align_of::<i32>() == 4);
#[cfg(feature = "pg15")]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__clock_gettime"]
    fn raw_inline_clock_gettime(arg0: i32, arg1: *mut crate::__pgrx_c_bindings::timespec) -> i32;
}
#[cfg(feature = "pg15")]
#[doc = "Call the original C function `clock_gettime`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_clock_gettime(
    arg0: i32,
    arg1: *mut crate::__pgrx_c_bindings::timespec,
) -> i32 {
    const _: () = assert!(!::core::mem::needs_drop::<i32>());
    const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::timespec>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_clock_gettime(arg0, arg1)) }
}
const _: () = assert!(::core::mem::size_of::<i8>() == 1);
const _: () = assert!(::core::mem::align_of::<i8>() == 1);
const _: () = assert!(::core::mem::size_of::<*const ::core::ffi::c_char>() == 8);
const _: () = assert!(::core::mem::align_of::<*const ::core::ffi::c_char>() == 8);
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__collprovider_name"]
    fn raw_inline_collprovider_name(arg0: i8) -> *const ::core::ffi::c_char;
}
#[doc = "Call the original C function `collprovider_name`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_collprovider_name(arg0: i8) -> *const ::core::ffi::c_char {
    const _: () = assert!(!::core::mem::needs_drop::<i8>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_collprovider_name(arg0)) }
}
const _: () = assert!(::core::mem::size_of::<*const i8>() == 8);
const _: () = assert!(::core::mem::align_of::<*const i8>() == 8);
const _: () = assert!(::core::mem::size_of::<*const i8>() == 8);
const _: () = assert!(::core::mem::align_of::<*const i8>() == 8);
const _: () = assert!(::core::mem::size_of::<i32>() == 4);
const _: () = assert!(::core::mem::align_of::<i32>() == 4);
const _: () = assert!(::core::mem::size_of::<*mut i8>() == 8);
const _: () = assert!(::core::mem::align_of::<*mut i8>() == 8);
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__dcgettext"]
    fn raw_inline_dcgettext(arg0: *const i8, arg1: *const i8, arg2: i32) -> *mut i8;
}
#[doc = "Call the original C function `dcgettext`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_dcgettext(arg0: *const i8, arg1: *const i8, arg2: i32) -> *mut i8 {
    const _: () = assert!(!::core::mem::needs_drop::<*const i8>());
    const _: () = assert!(!::core::mem::needs_drop::<*const i8>());
    const _: () = assert!(!::core::mem::needs_drop::<i32>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_dcgettext(arg0, arg1, arg2)) }
}
const _: () = assert!(::core::mem::size_of::<f64>() == 8);
const _: () = assert!(::core::mem::align_of::<f64>() == 8);
const _: () = assert!(::core::mem::size_of::<f64>() == 8);
const _: () = assert!(::core::mem::align_of::<f64>() == 8);
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__floor"]
    fn raw_inline_floor(arg0: f64) -> f64;
}
#[doc = "Call the original C function `floor`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_floor(arg0: f64) -> f64 {
    const _: () = assert!(!::core::mem::needs_drop::<f64>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_floor(arg0)) }
}
const _: () = assert!(::core::mem::size_of::<*const crate::__pgrx_c_bindings::List>() == 8);
const _: () = assert!(::core::mem::align_of::<*const crate::__pgrx_c_bindings::List>() == 8);
const _: () = assert!(::core::mem::size_of::<*const crate::__pgrx_c_bindings::ListCell>() == 8);
const _: () = assert!(::core::mem::align_of::<*const crate::__pgrx_c_bindings::ListCell>() == 8);
const _: () = assert!(::core::mem::size_of::<*const crate::__pgrx_c_bindings::List>() == 8);
const _: () = assert!(::core::mem::align_of::<*const crate::__pgrx_c_bindings::List>() == 8);
const _: () = assert!(::core::mem::size_of::<*const crate::__pgrx_c_bindings::ListCell>() == 8);
const _: () = assert!(::core::mem::align_of::<*const crate::__pgrx_c_bindings::ListCell>() == 8);
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ForBothCellState>>()
        == 24
);
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ForBothCellState>>()
        == 8
);
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__for_both_cell_setup"]
    fn raw_inline_for_both_cell_setup(
        arg0: *const crate::__pgrx_c_bindings::List,
        arg1: *const crate::__pgrx_c_bindings::ListCell,
        arg2: *const crate::__pgrx_c_bindings::List,
        arg3: *const crate::__pgrx_c_bindings::ListCell,
    ) -> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ForBothCellState>;
}
#[doc = "Call the original C function `for_both_cell_setup`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_for_both_cell_setup(
    arg0: *const crate::__pgrx_c_bindings::List,
    arg1: *const crate::__pgrx_c_bindings::ListCell,
    arg2: *const crate::__pgrx_c_bindings::List,
    arg3: *const crate::__pgrx_c_bindings::ListCell,
) -> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ForBothCellState> {
    const _: () = assert!(!::core::mem::needs_drop::<*const crate::__pgrx_c_bindings::List>());
    const _: () = assert!(!::core::mem::needs_drop::<*const crate::__pgrx_c_bindings::ListCell>());
    const _: () = assert!(!::core::mem::needs_drop::<*const crate::__pgrx_c_bindings::List>());
    const _: () = assert!(!::core::mem::needs_drop::<*const crate::__pgrx_c_bindings::ListCell>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe {
        crate::ffi::pg_guard_ffi_boundary(move || {
            raw_inline_for_both_cell_setup(arg0, arg1, arg2, arg3)
        })
    }
}
const _: () = assert!(::core::mem::size_of::<*const crate::__pgrx_c_bindings::List>() == 8);
const _: () = assert!(::core::mem::align_of::<*const crate::__pgrx_c_bindings::List>() == 8);
const _: () = assert!(::core::mem::size_of::<*const crate::__pgrx_c_bindings::ListCell>() == 8);
const _: () = assert!(::core::mem::align_of::<*const crate::__pgrx_c_bindings::ListCell>() == 8);
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ForEachState>>()
        == 16
);
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ForEachState>>()
        == 8
);
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__for_each_cell_setup"]
    fn raw_inline_for_each_cell_setup(
        arg0: *const crate::__pgrx_c_bindings::List,
        arg1: *const crate::__pgrx_c_bindings::ListCell,
    ) -> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ForEachState>;
}
#[doc = "Call the original C function `for_each_cell_setup`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_for_each_cell_setup(
    arg0: *const crate::__pgrx_c_bindings::List,
    arg1: *const crate::__pgrx_c_bindings::ListCell,
) -> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ForEachState> {
    const _: () = assert!(!::core::mem::needs_drop::<*const crate::__pgrx_c_bindings::List>());
    const _: () = assert!(!::core::mem::needs_drop::<*const crate::__pgrx_c_bindings::ListCell>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_for_each_cell_setup(arg0, arg1)) }
}
const _: () = assert!(::core::mem::size_of::<*const crate::__pgrx_c_bindings::List>() == 8);
const _: () = assert!(::core::mem::align_of::<*const crate::__pgrx_c_bindings::List>() == 8);
const _: () = assert!(::core::mem::size_of::<i32>() == 4);
const _: () = assert!(::core::mem::align_of::<i32>() == 4);
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ForEachState>>()
        == 16
);
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ForEachState>>()
        == 8
);
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__for_each_from_setup"]
    fn raw_inline_for_each_from_setup(
        arg0: *const crate::__pgrx_c_bindings::List,
        arg1: i32,
    ) -> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ForEachState>;
}
#[doc = "Call the original C function `for_each_from_setup`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_for_each_from_setup(
    arg0: *const crate::__pgrx_c_bindings::List,
    arg1: i32,
) -> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ForEachState> {
    const _: () = assert!(!::core::mem::needs_drop::<*const crate::__pgrx_c_bindings::List>());
    const _: () = assert!(!::core::mem::needs_drop::<i32>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_for_each_from_setup(arg0, arg1)) }
}
const _: () = assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Oid>() == 4);
const _: () = assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Oid>() == 4);
const _: () = assert!(::core::mem::size_of::<i8>() == 1);
const _: () = assert!(::core::mem::align_of::<i8>() == 1);
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__get_typstorage"]
    fn raw_inline_get_typstorage(arg0: crate::__pgrx_c_bindings::Oid) -> i8;
}
#[doc = "Call the original C function `get_typstorage`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_get_typstorage(arg0: crate::__pgrx_c_bindings::Oid) -> i8 {
    const _: () = assert!(!::core::mem::needs_drop::<crate::__pgrx_c_bindings::Oid>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_get_typstorage(arg0)) }
}
const _: () = assert!(::core::mem::size_of::<*mut ::core::ffi::c_void>() == 8);
const _: () = assert!(::core::mem::align_of::<*mut ::core::ffi::c_void>() == 8);
const _: () = assert!(::core::mem::size_of::<*const ::core::ffi::c_void>() == 8);
const _: () = assert!(::core::mem::align_of::<*const ::core::ffi::c_void>() == 8);
const _: () = assert!(::core::mem::size_of::<u64>() == 8);
const _: () = assert!(::core::mem::align_of::<u64>() == 8);
const _: () = assert!(::core::mem::size_of::<*mut ::core::ffi::c_void>() == 8);
const _: () = assert!(::core::mem::align_of::<*mut ::core::ffi::c_void>() == 8);
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__memcpy"]
    fn raw_inline_memcpy(
        arg0: *mut ::core::ffi::c_void,
        arg1: *const ::core::ffi::c_void,
        arg2: u64,
    ) -> *mut ::core::ffi::c_void;
}
#[doc = "Call the original C function `memcpy`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_memcpy(
    arg0: *mut ::core::ffi::c_void,
    arg1: *const ::core::ffi::c_void,
    arg2: u64,
) -> *mut ::core::ffi::c_void {
    const _: () = assert!(!::core::mem::needs_drop::<*mut ::core::ffi::c_void>());
    const _: () = assert!(!::core::mem::needs_drop::<*const ::core::ffi::c_void>());
    const _: () = assert!(!::core::mem::needs_drop::<u64>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_memcpy(arg0, arg1, arg2)) }
}
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__pg_re_throw"]
    fn raw_inline_pg_re_throw() -> ();
}
#[doc = "Call the original C function `pg_re_throw`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_pg_re_throw() -> () {
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_pg_re_throw()) }
}
const _: () = assert!(::core::mem::size_of::<*const crate::__pgrx_c_bindings::RangeType>() == 8);
const _: () = assert!(::core::mem::align_of::<*const crate::__pgrx_c_bindings::RangeType>() == 8);
const _: () = assert!(::core::mem::size_of::<i8>() == 1);
const _: () = assert!(::core::mem::align_of::<i8>() == 1);
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__range_get_flags"]
    fn raw_inline_range_get_flags(arg0: *const crate::__pgrx_c_bindings::RangeType) -> i8;
}
#[doc = "Call the original C function `range_get_flags`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_range_get_flags(arg0: *const crate::__pgrx_c_bindings::RangeType) -> i8 {
    const _: () = assert!(!::core::mem::needs_drop::<*const crate::__pgrx_c_bindings::RangeType>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_range_get_flags(arg0)) }
}
const _: () = assert!(::core::mem::size_of::<f64>() == 8);
const _: () = assert!(::core::mem::align_of::<f64>() == 8);
const _: () = assert!(::core::mem::size_of::<f64>() == 8);
const _: () = assert!(::core::mem::align_of::<f64>() == 8);
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__rint"]
    fn raw_inline_rint(arg0: f64) -> f64;
}
#[doc = "Call the original C function `rint`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_rint(arg0: f64) -> f64 {
    const _: () = assert!(!::core::mem::needs_drop::<f64>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_rint(arg0)) }
}
const _: () = assert!(::core::mem::size_of::<*const i8>() == 8);
const _: () = assert!(::core::mem::align_of::<*const i8>() == 8);
const _: () = assert!(::core::mem::size_of::<*const i8>() == 8);
const _: () = assert!(::core::mem::align_of::<*const i8>() == 8);
const _: () = assert!(::core::mem::size_of::<i32>() == 4);
const _: () = assert!(::core::mem::align_of::<i32>() == 4);
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__strcmp"]
    fn raw_inline_strcmp(arg0: *const i8, arg1: *const i8) -> i32;
}
#[doc = "Call the original C function `strcmp`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_strcmp(arg0: *const i8, arg1: *const i8) -> i32 {
    const _: () = assert!(!::core::mem::needs_drop::<*const i8>());
    const _: () = assert!(!::core::mem::needs_drop::<*const i8>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_strcmp(arg0, arg1)) }
}
const _: () = assert!(::core::mem::size_of::<*const i8>() == 8);
const _: () = assert!(::core::mem::align_of::<*const i8>() == 8);
const _: () = assert!(::core::mem::size_of::<u64>() == 8);
const _: () = assert!(::core::mem::align_of::<u64>() == 8);
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__strlen"]
    fn raw_inline_strlen(arg0: *const i8) -> u64;
}
#[doc = "Call the original C function `strlen`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_strlen(arg0: *const i8) -> u64 {
    const _: () = assert!(!::core::mem::needs_drop::<*const i8>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_strlen(arg0)) }
}
const _: () = assert!(::core::mem::size_of::<*const i8>() == 8);
const _: () = assert!(::core::mem::align_of::<*const i8>() == 8);
const _: () = assert!(::core::mem::size_of::<*const i8>() == 8);
const _: () = assert!(::core::mem::align_of::<*const i8>() == 8);
const _: () = assert!(::core::mem::size_of::<u64>() == 8);
const _: () = assert!(::core::mem::align_of::<u64>() == 8);
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__strspn"]
    fn raw_inline_strspn(arg0: *const i8, arg1: *const i8) -> u64;
}
#[doc = "Call the original C function `strspn`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_strspn(arg0: *const i8, arg1: *const i8) -> u64 {
    const _: () = assert!(!::core::mem::needs_drop::<*const i8>());
    const _: () = assert!(!::core::mem::needs_drop::<*const i8>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_strspn(arg0, arg1)) }
}
const _: () = assert!(::core::mem::size_of::<*const i8>() == 8);
const _: () = assert!(::core::mem::align_of::<*const i8>() == 8);
const _: () = assert!(::core::mem::size_of::<*mut *mut i8>() == 8);
const _: () = assert!(::core::mem::align_of::<*mut *mut i8>() == 8);
const _: () = assert!(::core::mem::size_of::<i32>() == 4);
const _: () = assert!(::core::mem::align_of::<i32>() == 4);
const _: () = assert!(::core::mem::size_of::<i64>() == 8);
const _: () = assert!(::core::mem::align_of::<i64>() == 8);
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__strtol"]
    fn raw_inline_strtol(arg0: *const i8, arg1: *mut *mut i8, arg2: i32) -> i64;
}
#[doc = "Call the original C function `strtol`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_strtol(arg0: *const i8, arg1: *mut *mut i8, arg2: i32) -> i64 {
    const _: () = assert!(!::core::mem::needs_drop::<*const i8>());
    const _: () = assert!(!::core::mem::needs_drop::<*mut *mut i8>());
    const _: () = assert!(!::core::mem::needs_drop::<i32>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_strtol(arg0, arg1, arg2)) }
}
const _: () = assert!(::core::mem::size_of::<*const i8>() == 8);
const _: () = assert!(::core::mem::align_of::<*const i8>() == 8);
const _: () = assert!(::core::mem::size_of::<*mut *mut i8>() == 8);
const _: () = assert!(::core::mem::align_of::<*mut *mut i8>() == 8);
const _: () = assert!(::core::mem::size_of::<i32>() == 4);
const _: () = assert!(::core::mem::align_of::<i32>() == 4);
const _: () = assert!(::core::mem::size_of::<u64>() == 8);
const _: () = assert!(::core::mem::align_of::<u64>() == 8);
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__strtoul"]
    fn raw_inline_strtoul(arg0: *const i8, arg1: *mut *mut i8, arg2: i32) -> u64;
}
#[doc = "Call the original C function `strtoul`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_strtoul(arg0: *const i8, arg1: *mut *mut i8, arg2: i32) -> u64 {
    const _: () = assert!(!::core::mem::needs_drop::<*const i8>());
    const _: () = assert!(!::core::mem::needs_drop::<*mut *mut i8>());
    const _: () = assert!(!::core::mem::needs_drop::<i32>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_strtoul(arg0, arg1, arg2)) }
}
const _: () =
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::TableScanDescData>() == 8);
const _: () =
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::TableScanDescData>() == 8);
const _: () = assert!(::core::mem::size_of::<i32>() == 4);
const _: () = assert!(::core::mem::align_of::<i32>() == 4);
const _: () = assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::TupleTableSlot>() == 8);
const _: () =
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::TupleTableSlot>() == 8);
const _: () = assert!(::core::mem::size_of::<bool>() == 1);
const _: () = assert!(::core::mem::align_of::<bool>() == 1);
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__table_scan_getnextslot"]
    fn raw_inline_table_scan_getnextslot(
        arg0: *mut crate::__pgrx_c_bindings::TableScanDescData,
        arg1: i32,
        arg2: *mut crate::__pgrx_c_bindings::TupleTableSlot,
    ) -> bool;
}
#[doc = "Call the original C function `table_scan_getnextslot`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_table_scan_getnextslot(
    arg0: *mut crate::__pgrx_c_bindings::TableScanDescData,
    arg1: i32,
    arg2: *mut crate::__pgrx_c_bindings::TupleTableSlot,
) -> bool {
    const _: () =
        assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::TableScanDescData>());
    const _: () = assert!(!::core::mem::needs_drop::<i32>());
    const _: () =
        assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::TupleTableSlot>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe {
        crate::ffi::pg_guard_ffi_boundary(move || {
            raw_inline_table_scan_getnextslot(arg0, arg1, arg2)
        })
    }
}
const _: () =
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::TableScanDescData>() == 8);
const _: () =
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::TableScanDescData>() == 8);
const _: () = assert!(::core::mem::size_of::<i32>() == 4);
const _: () = assert!(::core::mem::align_of::<i32>() == 4);
const _: () = assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::TupleTableSlot>() == 8);
const _: () =
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::TupleTableSlot>() == 8);
const _: () = assert!(::core::mem::size_of::<bool>() == 1);
const _: () = assert!(::core::mem::align_of::<bool>() == 1);
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__table_scan_getnextslot_tidrange"]
    fn raw_inline_table_scan_getnextslot_tidrange(
        arg0: *mut crate::__pgrx_c_bindings::TableScanDescData,
        arg1: i32,
        arg2: *mut crate::__pgrx_c_bindings::TupleTableSlot,
    ) -> bool;
}
#[doc = "Call the original C function `table_scan_getnextslot_tidrange`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_table_scan_getnextslot_tidrange(
    arg0: *mut crate::__pgrx_c_bindings::TableScanDescData,
    arg1: i32,
    arg2: *mut crate::__pgrx_c_bindings::TupleTableSlot,
) -> bool {
    const _: () =
        assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::TableScanDescData>());
    const _: () = assert!(!::core::mem::needs_drop::<i32>());
    const _: () =
        assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::TupleTableSlot>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe {
        crate::ffi::pg_guard_ffi_boundary(move || {
            raw_inline_table_scan_getnextslot_tidrange(arg0, arg1, arg2)
        })
    }
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_AuxProcType;
#[cfg(any(feature = "pg15", feature = "pg16"))]
impl c::sealed::Sealed for EnumIdentity_AuxProcType {}
#[cfg(any(feature = "pg15", feature = "pg16"))]
impl c::expression::EnumIdentity for EnumIdentity_AuxProcType {}
const _: () = {
    assert!(::core::mem::size_of::<i32>() == 4);
    assert!(::core::mem::align_of::<i32>() == 4);
};
#[cfg(any(feature = "pg15", feature = "pg16"))]
impl c::expression::enumeration::NumericEnum for EnumIdentity_AuxProcType {
    type Compatible = c::CInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_BgWorkerStartTime;
impl c::sealed::Sealed for EnumIdentity_BgWorkerStartTime {}
impl c::expression::EnumIdentity for EnumIdentity_BgWorkerStartTime {}
const _: () = {
    assert!(::core::mem::size_of::<u32>() == 4);
    assert!(::core::mem::align_of::<u32>() == 4);
};
impl c::expression::enumeration::NumericEnum for EnumIdentity_BgWorkerStartTime {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_CRSSnapshotAction;
impl c::sealed::Sealed for EnumIdentity_CRSSnapshotAction {}
impl c::expression::EnumIdentity for EnumIdentity_CRSSnapshotAction {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_CRSSnapshotAction {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_CommandDest;
impl c::sealed::Sealed for EnumIdentity_CommandDest {}
impl c::expression::EnumIdentity for EnumIdentity_CommandDest {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_CommandDest {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_ExprDoneCond;
impl c::sealed::Sealed for EnumIdentity_ExprDoneCond {}
impl c::expression::EnumIdentity for EnumIdentity_ExprDoneCond {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_ExprDoneCond {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_GucContext;
impl c::sealed::Sealed for EnumIdentity_GucContext {}
impl c::expression::EnumIdentity for EnumIdentity_GucContext {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_GucContext {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_GucSource;
impl c::sealed::Sealed for EnumIdentity_GucSource {}
impl c::expression::EnumIdentity for EnumIdentity_GucSource {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_GucSource {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_GucStackState;
impl c::sealed::Sealed for EnumIdentity_GucStackState {}
impl c::expression::EnumIdentity for EnumIdentity_GucStackState {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_GucStackState {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_HotStandbyState;
impl c::sealed::Sealed for EnumIdentity_HotStandbyState {}
impl c::expression::EnumIdentity for EnumIdentity_HotStandbyState {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_HotStandbyState {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_IncrementalSortExecutionStatus;
impl c::sealed::Sealed for EnumIdentity_IncrementalSortExecutionStatus {}
impl c::expression::EnumIdentity for EnumIdentity_IncrementalSortExecutionStatus {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_IncrementalSortExecutionStatus {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_JsonbIterState;
impl c::sealed::Sealed for EnumIdentity_JsonbIterState {}
impl c::expression::EnumIdentity for EnumIdentity_JsonbIterState {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_JsonbIterState {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_LimitStateCond;
impl c::sealed::Sealed for EnumIdentity_LimitStateCond {}
impl c::expression::EnumIdentity for EnumIdentity_LimitStateCond {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_LimitStateCond {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_MultiXactStatus;
impl c::sealed::Sealed for EnumIdentity_MultiXactStatus {}
impl c::expression::EnumIdentity for EnumIdentity_MultiXactStatus {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_MultiXactStatus {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_PartitionwiseAggregateType;
impl c::sealed::Sealed for EnumIdentity_PartitionwiseAggregateType {}
impl c::expression::EnumIdentity for EnumIdentity_PartitionwiseAggregateType {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_PartitionwiseAggregateType {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_ProcWaitStatus;
impl c::sealed::Sealed for EnumIdentity_ProcWaitStatus {}
impl c::expression::EnumIdentity for EnumIdentity_ProcWaitStatus {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_ProcWaitStatus {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_RawParseMode;
impl c::sealed::Sealed for EnumIdentity_RawParseMode {}
impl c::expression::EnumIdentity for EnumIdentity_RawParseMode {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_RawParseMode {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_SetFunctionReturnMode;
impl c::sealed::Sealed for EnumIdentity_SetFunctionReturnMode {}
impl c::expression::EnumIdentity for EnumIdentity_SetFunctionReturnMode {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_SetFunctionReturnMode {
    type Compatible = c::CUnsignedInt;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_SharedBitmapState;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::sealed::Sealed for EnumIdentity_SharedBitmapState {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::EnumIdentity for EnumIdentity_SharedBitmapState {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::enumeration::NumericEnum for EnumIdentity_SharedBitmapState {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_TuplesortMethod;
impl c::sealed::Sealed for EnumIdentity_TuplesortMethod {}
impl c::expression::EnumIdentity for EnumIdentity_TuplesortMethod {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_TuplesortMethod {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_TuplesortSpaceType;
impl c::sealed::Sealed for EnumIdentity_TuplesortSpaceType {}
impl c::expression::EnumIdentity for EnumIdentity_TuplesortSpaceType {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_TuplesortSpaceType {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_WalRcvExecStatus;
impl c::sealed::Sealed for EnumIdentity_WalRcvExecStatus {}
impl c::expression::EnumIdentity for EnumIdentity_WalRcvExecStatus {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_WalRcvExecStatus {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_WalRcvState;
impl c::sealed::Sealed for EnumIdentity_WalRcvState {}
impl c::expression::EnumIdentity for EnumIdentity_WalRcvState {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_WalRcvState {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__A_Expr_Kind;
impl c::sealed::Sealed for EnumIdentity_enum__A_Expr_Kind {}
impl c::expression::EnumIdentity for EnumIdentity_enum__A_Expr_Kind {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__A_Expr_Kind {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__AggSplit;
impl c::sealed::Sealed for EnumIdentity_enum__AggSplit {}
impl c::expression::EnumIdentity for EnumIdentity_enum__AggSplit {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__AggSplit {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__AggStrategy;
impl c::sealed::Sealed for EnumIdentity_enum__AggStrategy {}
impl c::expression::EnumIdentity for EnumIdentity_enum__AggStrategy {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__AggStrategy {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__AlterPublicationAction;
impl c::sealed::Sealed for EnumIdentity_enum__AlterPublicationAction {}
impl c::expression::EnumIdentity for EnumIdentity_enum__AlterPublicationAction {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__AlterPublicationAction {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__AlterSubscriptionType;
impl c::sealed::Sealed for EnumIdentity_enum__AlterSubscriptionType {}
impl c::expression::EnumIdentity for EnumIdentity_enum__AlterSubscriptionType {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__AlterSubscriptionType {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__AlterTSConfigType;
impl c::sealed::Sealed for EnumIdentity_enum__AlterTSConfigType {}
impl c::expression::EnumIdentity for EnumIdentity_enum__AlterTSConfigType {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__AlterTSConfigType {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__AlterTableType;
impl c::sealed::Sealed for EnumIdentity_enum__AlterTableType {}
impl c::expression::EnumIdentity for EnumIdentity_enum__AlterTableType {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__AlterTableType {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__BackendState;
impl c::sealed::Sealed for EnumIdentity_enum__BackendState {}
impl c::expression::EnumIdentity for EnumIdentity_enum__BackendState {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__BackendState {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__BackendType;
impl c::sealed::Sealed for EnumIdentity_enum__BackendType {}
impl c::expression::EnumIdentity for EnumIdentity_enum__BackendType {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__BackendType {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__BoolExprType;
impl c::sealed::Sealed for EnumIdentity_enum__BoolExprType {}
impl c::expression::EnumIdentity for EnumIdentity_enum__BoolExprType {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__BoolExprType {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__BoolTestType;
impl c::sealed::Sealed for EnumIdentity_enum__BoolTestType {}
impl c::expression::EnumIdentity for EnumIdentity_enum__BoolTestType {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__BoolTestType {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__CTEMaterialize;
impl c::sealed::Sealed for EnumIdentity_enum__CTEMaterialize {}
impl c::expression::EnumIdentity for EnumIdentity_enum__CTEMaterialize {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__CTEMaterialize {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__CmdType;
impl c::sealed::Sealed for EnumIdentity_enum__CmdType {}
impl c::expression::EnumIdentity for EnumIdentity_enum__CmdType {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__CmdType {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__CoercionContext;
impl c::sealed::Sealed for EnumIdentity_enum__CoercionContext {}
impl c::expression::EnumIdentity for EnumIdentity_enum__CoercionContext {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__CoercionContext {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__CoercionForm;
impl c::sealed::Sealed for EnumIdentity_enum__CoercionForm {}
impl c::expression::EnumIdentity for EnumIdentity_enum__CoercionForm {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__CoercionForm {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__CollectedCommandType;
impl c::sealed::Sealed for EnumIdentity_enum__CollectedCommandType {}
impl c::expression::EnumIdentity for EnumIdentity_enum__CollectedCommandType {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__CollectedCommandType {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__CommandTag;
impl c::sealed::Sealed for EnumIdentity_enum__CommandTag {}
impl c::expression::EnumIdentity for EnumIdentity_enum__CommandTag {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__CommandTag {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__ConstrType;
impl c::sealed::Sealed for EnumIdentity_enum__ConstrType {}
impl c::expression::EnumIdentity for EnumIdentity_enum__ConstrType {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__ConstrType {
    type Compatible = c::CUnsignedInt;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__CopyHeaderChoice;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::sealed::Sealed for EnumIdentity_enum__CopyHeaderChoice {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::EnumIdentity for EnumIdentity_enum__CopyHeaderChoice {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__CopyHeaderChoice {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__CopySource;
impl c::sealed::Sealed for EnumIdentity_enum__CopySource {}
impl c::expression::EnumIdentity for EnumIdentity_enum__CopySource {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__CopySource {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__DBState;
impl c::sealed::Sealed for EnumIdentity_enum__DBState {}
impl c::expression::EnumIdentity for EnumIdentity_enum__DBState {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__DBState {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__DefElemAction;
impl c::sealed::Sealed for EnumIdentity_enum__DefElemAction {}
impl c::expression::EnumIdentity for EnumIdentity_enum__DefElemAction {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__DefElemAction {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__DiscardMode;
impl c::sealed::Sealed for EnumIdentity_enum__DiscardMode {}
impl c::expression::EnumIdentity for EnumIdentity_enum__DiscardMode {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__DiscardMode {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__DomainConstraintType;
impl c::sealed::Sealed for EnumIdentity_enum__DomainConstraintType {}
impl c::expression::EnumIdentity for EnumIdentity_enum__DomainConstraintType {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__DomainConstraintType {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__DropBehavior;
impl c::sealed::Sealed for EnumIdentity_enum__DropBehavior {}
impl c::expression::EnumIdentity for EnumIdentity_enum__DropBehavior {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__DropBehavior {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__EolType;
impl c::sealed::Sealed for EnumIdentity_enum__EolType {}
impl c::expression::EnumIdentity for EnumIdentity_enum__EolType {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__EolType {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__EphemeralNameRelationType;
impl c::sealed::Sealed for EnumIdentity_enum__EphemeralNameRelationType {}
impl c::expression::EnumIdentity for EnumIdentity_enum__EphemeralNameRelationType {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__EphemeralNameRelationType {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__ExplainFormat;
impl c::sealed::Sealed for EnumIdentity_enum__ExplainFormat {}
impl c::expression::EnumIdentity for EnumIdentity_enum__ExplainFormat {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__ExplainFormat {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__FetchDirection;
impl c::sealed::Sealed for EnumIdentity_enum__FetchDirection {}
impl c::expression::EnumIdentity for EnumIdentity_enum__FetchDirection {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__FetchDirection {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__ForkNumber;
impl c::sealed::Sealed for EnumIdentity_enum__ForkNumber {}
impl c::expression::EnumIdentity for EnumIdentity_enum__ForkNumber {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__ForkNumber {
    type Compatible = c::CInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__FunctionParameterMode;
impl c::sealed::Sealed for EnumIdentity_enum__FunctionParameterMode {}
impl c::expression::EnumIdentity for EnumIdentity_enum__FunctionParameterMode {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__FunctionParameterMode {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__GistOptBufferingMode;
impl c::sealed::Sealed for EnumIdentity_enum__GistOptBufferingMode {}
impl c::expression::EnumIdentity for EnumIdentity_enum__GistOptBufferingMode {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__GistOptBufferingMode {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__GrantTargetType;
impl c::sealed::Sealed for EnumIdentity_enum__GrantTargetType {}
impl c::expression::EnumIdentity for EnumIdentity_enum__GrantTargetType {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__GrantTargetType {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__GroupingSetKind;
impl c::sealed::Sealed for EnumIdentity_enum__GroupingSetKind {}
impl c::expression::EnumIdentity for EnumIdentity_enum__GroupingSetKind {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__GroupingSetKind {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__ImportForeignSchemaType;
impl c::sealed::Sealed for EnumIdentity_enum__ImportForeignSchemaType {}
impl c::expression::EnumIdentity for EnumIdentity_enum__ImportForeignSchemaType {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__ImportForeignSchemaType {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__JoinType;
impl c::sealed::Sealed for EnumIdentity_enum__JoinType {}
impl c::expression::EnumIdentity for EnumIdentity_enum__JoinType {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__JoinType {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__LimitOption;
impl c::sealed::Sealed for EnumIdentity_enum__LimitOption {}
impl c::expression::EnumIdentity for EnumIdentity_enum__LimitOption {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__LimitOption {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__LockClauseStrength;
impl c::sealed::Sealed for EnumIdentity_enum__LockClauseStrength {}
impl c::expression::EnumIdentity for EnumIdentity_enum__LockClauseStrength {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__LockClauseStrength {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__LockTagType;
impl c::sealed::Sealed for EnumIdentity_enum__LockTagType {}
impl c::expression::EnumIdentity for EnumIdentity_enum__LockTagType {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__LockTagType {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__LockTupleMode;
impl c::sealed::Sealed for EnumIdentity_enum__LockTupleMode {}
impl c::expression::EnumIdentity for EnumIdentity_enum__LockTupleMode {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__LockTupleMode {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__LockWaitPolicy;
impl c::sealed::Sealed for EnumIdentity_enum__LockWaitPolicy {}
impl c::expression::EnumIdentity for EnumIdentity_enum__LockWaitPolicy {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__LockWaitPolicy {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__MinMaxOp;
impl c::sealed::Sealed for EnumIdentity_enum__MinMaxOp {}
impl c::expression::EnumIdentity for EnumIdentity_enum__MinMaxOp {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__MinMaxOp {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__MonotonicFunction;
impl c::sealed::Sealed for EnumIdentity_enum__MonotonicFunction {}
impl c::expression::EnumIdentity for EnumIdentity_enum__MonotonicFunction {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__MonotonicFunction {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__NodeTag;
impl c::sealed::Sealed for EnumIdentity_enum__NodeTag {}
impl c::expression::EnumIdentity for EnumIdentity_enum__NodeTag {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__NodeTag {
    type Compatible = c::CUnsignedInt;
}
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::NodeTag>() == 4);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::NodeTag>() == 4);
};
#[cfg(feature = "pg15")]
// SAFETY: The explicit Rust integer repr matches Clang's compatible enum type and the assertions prove layout. Decoding receives a valid unit Rust variant; encoding checks every discriminant.
unsafe impl c::expression::EnumStorage<EnumIdentity_enum__NodeTag, c::CUnsignedInt>
    for crate::__pgrx_c_bindings::NodeTag
{
    fn decode(value: Self) -> u32 {
        value as u32
    }
    fn encode(value: u32) -> Self {
        match value {
            363 => crate::__pgrx_c_bindings::NodeTag::T_A_ArrayExpr,
            358 => crate::__pgrx_c_bindings::NodeTag::T_A_Const,
            355 => crate::__pgrx_c_bindings::NodeTag::T_A_Expr,
            361 => crate::__pgrx_c_bindings::NodeTag::T_A_Indices,
            362 => crate::__pgrx_c_bindings::NodeTag::T_A_Indirection,
            360 => crate::__pgrx_c_bindings::NodeTag::T_A_Star,
            389 => crate::__pgrx_c_bindings::NodeTag::T_AccessPriv,
            46 => crate::__pgrx_c_bindings::NodeTag::T_Agg,
            196 => crate::__pgrx_c_bindings::NodeTag::T_AggPath,
            98 => crate::__pgrx_c_bindings::NodeTag::T_AggState,
            113 => crate::__pgrx_c_bindings::NodeTag::T_Aggref,
            107 => crate::__pgrx_c_bindings::NodeTag::T_Alias,
            223 => crate::__pgrx_c_bindings::NodeTag::T_AllocSetContext,
            352 => crate::__pgrx_c_bindings::NodeTag::T_AlterCollationStmt,
            295 => crate::__pgrx_c_bindings::NodeTag::T_AlterDatabaseRefreshCollStmt,
            296 => crate::__pgrx_c_bindings::NodeTag::T_AlterDatabaseSetStmt,
            294 => crate::__pgrx_c_bindings::NodeTag::T_AlterDatabaseStmt,
            251 => crate::__pgrx_c_bindings::NodeTag::T_AlterDefaultPrivilegesStmt,
            247 => crate::__pgrx_c_bindings::NodeTag::T_AlterDomainStmt,
            319 => crate::__pgrx_c_bindings::NodeTag::T_AlterEnumStmt,
            338 => crate::__pgrx_c_bindings::NodeTag::T_AlterEventTrigStmt,
            336 => crate::__pgrx_c_bindings::NodeTag::T_AlterExtensionContentsStmt,
            335 => crate::__pgrx_c_bindings::NodeTag::T_AlterExtensionStmt,
            323 => crate::__pgrx_c_bindings::NodeTag::T_AlterFdwStmt,
            325 => crate::__pgrx_c_bindings::NodeTag::T_AlterForeignServerStmt,
            263 => crate::__pgrx_c_bindings::NodeTag::T_AlterFunctionStmt,
            309 => crate::__pgrx_c_bindings::NodeTag::T_AlterObjectDependsStmt,
            310 => crate::__pgrx_c_bindings::NodeTag::T_AlterObjectSchemaStmt,
            302 => crate::__pgrx_c_bindings::NodeTag::T_AlterOpFamilyStmt,
            312 => crate::__pgrx_c_bindings::NodeTag::T_AlterOperatorStmt,
            311 => crate::__pgrx_c_bindings::NodeTag::T_AlterOwnerStmt,
            343 => crate::__pgrx_c_bindings::NodeTag::T_AlterPolicyStmt,
            347 => crate::__pgrx_c_bindings::NodeTag::T_AlterPublicationStmt,
            297 => crate::__pgrx_c_bindings::NodeTag::T_AlterRoleSetStmt,
            287 => crate::__pgrx_c_bindings::NodeTag::T_AlterRoleStmt,
            280 => crate::__pgrx_c_bindings::NodeTag::T_AlterSeqStmt,
            354 => crate::__pgrx_c_bindings::NodeTag::T_AlterStatsStmt,
            349 => crate::__pgrx_c_bindings::NodeTag::T_AlterSubscriptionStmt,
            341 => crate::__pgrx_c_bindings::NodeTag::T_AlterSystemStmt,
            321 => crate::__pgrx_c_bindings::NodeTag::T_AlterTSConfigurationStmt,
            320 => crate::__pgrx_c_bindings::NodeTag::T_AlterTSDictionaryStmt,
            246 => crate::__pgrx_c_bindings::NodeTag::T_AlterTableCmd,
            330 => crate::__pgrx_c_bindings::NodeTag::T_AlterTableMoveAllStmt,
            329 => crate::__pgrx_c_bindings::NodeTag::T_AlterTableSpaceOptionsStmt,
            245 => crate::__pgrx_c_bindings::NodeTag::T_AlterTableStmt,
            313 => crate::__pgrx_c_bindings::NodeTag::T_AlterTypeStmt,
            327 => crate::__pgrx_c_bindings::NodeTag::T_AlterUserMappingStmt,
            126 => crate::__pgrx_c_bindings::NodeTag::T_AlternativeSubPlan,
            14 => crate::__pgrx_c_bindings::NodeTag::T_Append,
            182 => crate::__pgrx_c_bindings::NodeTag::T_AppendPath,
            214 => crate::__pgrx_c_bindings::NodeTag::T_AppendRelInfo,
            66 => crate::__pgrx_c_bindings::NodeTag::T_AppendState,
            131 => crate::__pgrx_c_bindings::NodeTag::T_ArrayCoerceExpr,
            137 => crate::__pgrx_c_bindings::NodeTag::T_ArrayExpr,
            414 => crate::__pgrx_c_bindings::NodeTag::T_BaseBackupCmd,
            230 => crate::__pgrx_c_bindings::NodeTag::T_BitString,
            17 => crate::__pgrx_c_bindings::NodeTag::T_BitmapAnd,
            172 => crate::__pgrx_c_bindings::NodeTag::T_BitmapAndPath,
            69 => crate::__pgrx_c_bindings::NodeTag::T_BitmapAndState,
            171 => crate::__pgrx_c_bindings::NodeTag::T_BitmapHeapPath,
            25 => crate::__pgrx_c_bindings::NodeTag::T_BitmapHeapScan,
            77 => crate::__pgrx_c_bindings::NodeTag::T_BitmapHeapScanState,
            24 => crate::__pgrx_c_bindings::NodeTag::T_BitmapIndexScan,
            76 => crate::__pgrx_c_bindings::NodeTag::T_BitmapIndexScanState,
            18 => crate::__pgrx_c_bindings::NodeTag::T_BitmapOr,
            173 => crate::__pgrx_c_bindings::NodeTag::T_BitmapOrPath,
            70 => crate::__pgrx_c_bindings::NodeTag::T_BitmapOrState,
            123 => crate::__pgrx_c_bindings::NodeTag::T_BoolExpr,
            228 => crate::__pgrx_c_bindings::NodeTag::T_Boolean,
            145 => crate::__pgrx_c_bindings::NodeTag::T_BooleanTest,
            400 => crate::__pgrx_c_bindings::NodeTag::T_CTECycleClause,
            399 => crate::__pgrx_c_bindings::NodeTag::T_CTESearchClause,
            431 => crate::__pgrx_c_bindings::NodeTag::T_CallContext,
            353 => crate::__pgrx_c_bindings::NodeTag::T_CallStmt,
            134 => crate::__pgrx_c_bindings::NodeTag::T_CaseExpr,
            136 => crate::__pgrx_c_bindings::NodeTag::T_CaseTestExpr,
            135 => crate::__pgrx_c_bindings::NodeTag::T_CaseWhen,
            292 => crate::__pgrx_c_bindings::NodeTag::T_CheckPointStmt,
            252 => crate::__pgrx_c_bindings::NodeTag::T_ClosePortalStmt,
            253 => crate::__pgrx_c_bindings::NodeTag::T_ClusterStmt,
            140 => crate::__pgrx_c_bindings::NodeTag::T_CoalesceExpr,
            146 => crate::__pgrx_c_bindings::NodeTag::T_CoerceToDomain,
            147 => crate::__pgrx_c_bindings::NodeTag::T_CoerceToDomainValue,
            130 => crate::__pgrx_c_bindings::NodeTag::T_CoerceViaIO,
            367 => crate::__pgrx_c_bindings::NodeTag::T_CollateClause,
            133 => crate::__pgrx_c_bindings::NodeTag::T_CollateExpr,
            376 => crate::__pgrx_c_bindings::NodeTag::T_ColumnDef,
            356 => crate::__pgrx_c_bindings::NodeTag::T_ColumnRef,
            259 => crate::__pgrx_c_bindings::NodeTag::T_CommentStmt,
            401 => crate::__pgrx_c_bindings::NodeTag::T_CommonTableExpr,
            316 => crate::__pgrx_c_bindings::NodeTag::T_CompositeTypeStmt,
            111 => crate::__pgrx_c_bindings::NodeTag::T_Const,
            379 => crate::__pgrx_c_bindings::NodeTag::T_Constraint,
            290 => crate::__pgrx_c_bindings::NodeTag::T_ConstraintsSetStmt,
            132 => crate::__pgrx_c_bindings::NodeTag::T_ConvertRowtypeExpr,
            254 => crate::__pgrx_c_bindings::NodeTag::T_CopyStmt,
            345 => crate::__pgrx_c_bindings::NodeTag::T_CreateAmStmt,
            299 => crate::__pgrx_c_bindings::NodeTag::T_CreateCastStmt,
            298 => crate::__pgrx_c_bindings::NodeTag::T_CreateConversionStmt,
            273 => crate::__pgrx_c_bindings::NodeTag::T_CreateDomainStmt,
            317 => crate::__pgrx_c_bindings::NodeTag::T_CreateEnumStmt,
            337 => crate::__pgrx_c_bindings::NodeTag::T_CreateEventTrigStmt,
            334 => crate::__pgrx_c_bindings::NodeTag::T_CreateExtensionStmt,
            322 => crate::__pgrx_c_bindings::NodeTag::T_CreateFdwStmt,
            324 => crate::__pgrx_c_bindings::NodeTag::T_CreateForeignServerStmt,
            332 => crate::__pgrx_c_bindings::NodeTag::T_CreateForeignTableStmt,
            262 => crate::__pgrx_c_bindings::NodeTag::T_CreateFunctionStmt,
            390 => crate::__pgrx_c_bindings::NodeTag::T_CreateOpClassItem,
            300 => crate::__pgrx_c_bindings::NodeTag::T_CreateOpClassStmt,
            301 => crate::__pgrx_c_bindings::NodeTag::T_CreateOpFamilyStmt,
            285 => crate::__pgrx_c_bindings::NodeTag::T_CreatePLangStmt,
            342 => crate::__pgrx_c_bindings::NodeTag::T_CreatePolicyStmt,
            346 => crate::__pgrx_c_bindings::NodeTag::T_CreatePublicationStmt,
            318 => crate::__pgrx_c_bindings::NodeTag::T_CreateRangeStmt,
            415 => crate::__pgrx_c_bindings::NodeTag::T_CreateReplicationSlotCmd,
            286 => crate::__pgrx_c_bindings::NodeTag::T_CreateRoleStmt,
            293 => crate::__pgrx_c_bindings::NodeTag::T_CreateSchemaStmt,
            279 => crate::__pgrx_c_bindings::NodeTag::T_CreateSeqStmt,
            351 => crate::__pgrx_c_bindings::NodeTag::T_CreateStatsStmt,
            255 => crate::__pgrx_c_bindings::NodeTag::T_CreateStmt,
            348 => crate::__pgrx_c_bindings::NodeTag::T_CreateSubscriptionStmt,
            278 => crate::__pgrx_c_bindings::NodeTag::T_CreateTableAsStmt,
            307 => crate::__pgrx_c_bindings::NodeTag::T_CreateTableSpaceStmt,
            344 => crate::__pgrx_c_bindings::NodeTag::T_CreateTransformStmt,
            284 => crate::__pgrx_c_bindings::NodeTag::T_CreateTrigStmt,
            326 => crate::__pgrx_c_bindings::NodeTag::T_CreateUserMappingStmt,
            274 => crate::__pgrx_c_bindings::NodeTag::T_CreatedbStmt,
            32 => crate::__pgrx_c_bindings::NodeTag::T_CteScan,
            84 => crate::__pgrx_c_bindings::NodeTag::T_CteScanState,
            149 => crate::__pgrx_c_bindings::NodeTag::T_CurrentOfExpr,
            178 => crate::__pgrx_c_bindings::NodeTag::T_CustomPath,
            36 => crate::__pgrx_c_bindings::NodeTag::T_CustomScan,
            88 => crate::__pgrx_c_bindings::NodeTag::T_CustomScanState,
            305 => crate::__pgrx_c_bindings::NodeTag::T_DeallocateStmt,
            306 => crate::__pgrx_c_bindings::NodeTag::T_DeclareCursorStmt,
            380 => crate::__pgrx_c_bindings::NodeTag::T_DefElem,
            256 => crate::__pgrx_c_bindings::NodeTag::T_DefineStmt,
            239 => crate::__pgrx_c_bindings::NodeTag::T_DeleteStmt,
            283 => crate::__pgrx_c_bindings::NodeTag::T_DiscardStmt,
            120 => crate::__pgrx_c_bindings::NodeTag::T_DistinctExpr,
            264 => crate::__pgrx_c_bindings::NodeTag::T_DoStmt,
            162 => crate::__pgrx_c_bindings::NodeTag::T_DomainConstraintState,
            314 => crate::__pgrx_c_bindings::NodeTag::T_DropOwnedStmt,
            416 => crate::__pgrx_c_bindings::NodeTag::T_DropReplicationSlotCmd,
            288 => crate::__pgrx_c_bindings::NodeTag::T_DropRoleStmt,
            257 => crate::__pgrx_c_bindings::NodeTag::T_DropStmt,
            350 => crate::__pgrx_c_bindings::NodeTag::T_DropSubscriptionStmt,
            308 => crate::__pgrx_c_bindings::NodeTag::T_DropTableSpaceStmt,
            328 => crate::__pgrx_c_bindings::NodeTag::T_DropUserMappingStmt,
            275 => crate::__pgrx_c_bindings::NodeTag::T_DropdbStmt,
            8 => crate::__pgrx_c_bindings::NodeTag::T_EState,
            205 => crate::__pgrx_c_bindings::NodeTag::T_EquivalenceClass,
            206 => crate::__pgrx_c_bindings::NodeTag::T_EquivalenceMember,
            421 => crate::__pgrx_c_bindings::NodeTag::T_EventTriggerData,
            304 => crate::__pgrx_c_bindings::NodeTag::T_ExecuteStmt,
            277 => crate::__pgrx_c_bindings::NodeTag::T_ExplainStmt,
            2 => crate::__pgrx_c_bindings::NodeTag::T_ExprContext,
            158 => crate::__pgrx_c_bindings::NodeTag::T_ExprState,
            234 => crate::__pgrx_c_bindings::NodeTag::T_ExtensibleNode,
            426 => crate::__pgrx_c_bindings::NodeTag::T_FdwRoutine,
            260 => crate::__pgrx_c_bindings::NodeTag::T_FetchStmt,
            127 => crate::__pgrx_c_bindings::NodeTag::T_FieldSelect,
            128 => crate::__pgrx_c_bindings::NodeTag::T_FieldStore,
            227 => crate::__pgrx_c_bindings::NodeTag::T_Float,
            430 => crate::__pgrx_c_bindings::NodeTag::T_ForeignKeyCacheInfo,
            167 => crate::__pgrx_c_bindings::NodeTag::T_ForeignKeyOptInfo,
            177 => crate::__pgrx_c_bindings::NodeTag::T_ForeignPath,
            35 => crate::__pgrx_c_bindings::NodeTag::T_ForeignScan,
            87 => crate::__pgrx_c_bindings::NodeTag::T_ForeignScanState,
            155 => crate::__pgrx_c_bindings::NodeTag::T_FromExpr,
            359 => crate::__pgrx_c_bindings::NodeTag::T_FuncCall,
            117 => crate::__pgrx_c_bindings::NodeTag::T_FuncExpr,
            392 => crate::__pgrx_c_bindings::NodeTag::T_FunctionParameter,
            29 => crate::__pgrx_c_bindings::NodeTag::T_FunctionScan,
            81 => crate::__pgrx_c_bindings::NodeTag::T_FunctionScanState,
            49 => crate::__pgrx_c_bindings::NodeTag::T_Gather,
            50 => crate::__pgrx_c_bindings::NodeTag::T_GatherMerge,
            189 => crate::__pgrx_c_bindings::NodeTag::T_GatherMergePath,
            102 => crate::__pgrx_c_bindings::NodeTag::T_GatherMergeState,
            188 => crate::__pgrx_c_bindings::NodeTag::T_GatherPath,
            101 => crate::__pgrx_c_bindings::NodeTag::T_GatherState,
            225 => crate::__pgrx_c_bindings::NodeTag::T_GenerationContext,
            250 => crate::__pgrx_c_bindings::NodeTag::T_GrantRoleStmt,
            249 => crate::__pgrx_c_bindings::NodeTag::T_GrantStmt,
            45 => crate::__pgrx_c_bindings::NodeTag::T_Group,
            194 => crate::__pgrx_c_bindings::NodeTag::T_GroupPath,
            184 => crate::__pgrx_c_bindings::NodeTag::T_GroupResultPath,
            97 => crate::__pgrx_c_bindings::NodeTag::T_GroupState,
            114 => crate::__pgrx_c_bindings::NodeTag::T_GroupingFunc,
            386 => crate::__pgrx_c_bindings::NodeTag::T_GroupingSet,
            220 => crate::__pgrx_c_bindings::NodeTag::T_GroupingSetData,
            197 => crate::__pgrx_c_bindings::NodeTag::T_GroupingSetsPath,
            51 => crate::__pgrx_c_bindings::NodeTag::T_Hash,
            40 => crate::__pgrx_c_bindings::NodeTag::T_HashJoin,
            92 => crate::__pgrx_c_bindings::NodeTag::T_HashJoinState,
            181 => crate::__pgrx_c_bindings::NodeTag::T_HashPath,
            103 => crate::__pgrx_c_bindings::NodeTag::T_HashState,
            413 => crate::__pgrx_c_bindings::NodeTag::T_IdentifySystemCmd,
            333 => crate::__pgrx_c_bindings::NodeTag::T_ImportForeignSchemaStmt,
            44 => crate::__pgrx_c_bindings::NodeTag::T_IncrementalSort,
            193 => crate::__pgrx_c_bindings::NodeTag::T_IncrementalSortPath,
            96 => crate::__pgrx_c_bindings::NodeTag::T_IncrementalSortState,
            427 => crate::__pgrx_c_bindings::NodeTag::T_IndexAmRoutine,
            211 => crate::__pgrx_c_bindings::NodeTag::T_IndexClause,
            377 => crate::__pgrx_c_bindings::NodeTag::T_IndexElem,
            1 => crate::__pgrx_c_bindings::NodeTag::T_IndexInfo,
            23 => crate::__pgrx_c_bindings::NodeTag::T_IndexOnlyScan,
            75 => crate::__pgrx_c_bindings::NodeTag::T_IndexOnlyScanState,
            166 => crate::__pgrx_c_bindings::NodeTag::T_IndexOptInfo,
            170 => crate::__pgrx_c_bindings::NodeTag::T_IndexPath,
            22 => crate::__pgrx_c_bindings::NodeTag::T_IndexScan,
            74 => crate::__pgrx_c_bindings::NodeTag::T_IndexScanState,
            261 => crate::__pgrx_c_bindings::NodeTag::T_IndexStmt,
            397 => crate::__pgrx_c_bindings::NodeTag::T_InferClause,
            151 => crate::__pgrx_c_bindings::NodeTag::T_InferenceElem,
            425 => crate::__pgrx_c_bindings::NodeTag::T_InlineCodeBlock,
            238 => crate::__pgrx_c_bindings::NodeTag::T_InsertStmt,
            232 => crate::__pgrx_c_bindings::NodeTag::T_IntList,
            226 => crate::__pgrx_c_bindings::NodeTag::T_Integer,
            157 => crate::__pgrx_c_bindings::NodeTag::T_IntoClause,
            0 => crate::__pgrx_c_bindings::NodeTag::T_Invalid,
            37 => crate::__pgrx_c_bindings::NodeTag::T_Join,
            154 => crate::__pgrx_c_bindings::NodeTag::T_JoinExpr,
            89 => crate::__pgrx_c_bindings::NodeTag::T_JoinState,
            4 => crate::__pgrx_c_bindings::NodeTag::T_JunkFilter,
            54 => crate::__pgrx_c_bindings::NodeTag::T_Limit,
            204 => crate::__pgrx_c_bindings::NodeTag::T_LimitPath,
            106 => crate::__pgrx_c_bindings::NodeTag::T_LimitState,
            231 => crate::__pgrx_c_bindings::NodeTag::T_List,
            268 => crate::__pgrx_c_bindings::NodeTag::T_ListenStmt,
            272 => crate::__pgrx_c_bindings::NodeTag::T_LoadStmt,
            53 => crate::__pgrx_c_bindings::NodeTag::T_LockRows,
            202 => crate::__pgrx_c_bindings::NodeTag::T_LockRowsPath,
            105 => crate::__pgrx_c_bindings::NodeTag::T_LockRowsState,
            289 => crate::__pgrx_c_bindings::NodeTag::T_LockStmt,
            393 => crate::__pgrx_c_bindings::NodeTag::T_LockingClause,
            41 => crate::__pgrx_c_bindings::NodeTag::T_Material,
            185 => crate::__pgrx_c_bindings::NodeTag::T_MaterialPath,
            93 => crate::__pgrx_c_bindings::NodeTag::T_MaterialState,
            42 => crate::__pgrx_c_bindings::NodeTag::T_Memoize,
            186 => crate::__pgrx_c_bindings::NodeTag::T_MemoizePath,
            94 => crate::__pgrx_c_bindings::NodeTag::T_MemoizeState,
            222 => crate::__pgrx_c_bindings::NodeTag::T_MergeAction,
            6 => crate::__pgrx_c_bindings::NodeTag::T_MergeActionState,
            15 => crate::__pgrx_c_bindings::NodeTag::T_MergeAppend,
            183 => crate::__pgrx_c_bindings::NodeTag::T_MergeAppendPath,
            67 => crate::__pgrx_c_bindings::NodeTag::T_MergeAppendState,
            39 => crate::__pgrx_c_bindings::NodeTag::T_MergeJoin,
            91 => crate::__pgrx_c_bindings::NodeTag::T_MergeJoinState,
            180 => crate::__pgrx_c_bindings::NodeTag::T_MergePath,
            241 => crate::__pgrx_c_bindings::NodeTag::T_MergeStmt,
            402 => crate::__pgrx_c_bindings::NodeTag::T_MergeWhenClause,
            217 => crate::__pgrx_c_bindings::NodeTag::T_MinMaxAggInfo,
            198 => crate::__pgrx_c_bindings::NodeTag::T_MinMaxAggPath,
            141 => crate::__pgrx_c_bindings::NodeTag::T_MinMaxExpr,
            13 => crate::__pgrx_c_bindings::NodeTag::T_ModifyTable,
            203 => crate::__pgrx_c_bindings::NodeTag::T_ModifyTablePath,
            65 => crate::__pgrx_c_bindings::NodeTag::T_ModifyTableState,
            365 => crate::__pgrx_c_bindings::NodeTag::T_MultiAssignRef,
            118 => crate::__pgrx_c_bindings::NodeTag::T_NamedArgExpr,
            33 => crate::__pgrx_c_bindings::NodeTag::T_NamedTuplestoreScan,
            85 => crate::__pgrx_c_bindings::NodeTag::T_NamedTuplestoreScanState,
            38 => crate::__pgrx_c_bindings::NodeTag::T_NestLoop,
            55 => crate::__pgrx_c_bindings::NodeTag::T_NestLoopParam,
            90 => crate::__pgrx_c_bindings::NodeTag::T_NestLoopState,
            179 => crate::__pgrx_c_bindings::NodeTag::T_NestPath,
            150 => crate::__pgrx_c_bindings::NodeTag::T_NextValueExpr,
            267 => crate::__pgrx_c_bindings::NodeTag::T_NotifyStmt,
            121 => crate::__pgrx_c_bindings::NodeTag::T_NullIfExpr,
            144 => crate::__pgrx_c_bindings::NodeTag::T_NullTest,
            388 => crate::__pgrx_c_bindings::NodeTag::T_ObjectWithArgs,
            233 => crate::__pgrx_c_bindings::NodeTag::T_OidList,
            398 => crate::__pgrx_c_bindings::NodeTag::T_OnConflictClause,
            156 => crate::__pgrx_c_bindings::NodeTag::T_OnConflictExpr,
            5 => crate::__pgrx_c_bindings::NodeTag::T_OnConflictSetState,
            119 => crate::__pgrx_c_bindings::NodeTag::T_OpExpr,
            244 => crate::__pgrx_c_bindings::NodeTag::T_PLAssignStmt,
            112 => crate::__pgrx_c_bindings::NodeTag::T_Param,
            168 => crate::__pgrx_c_bindings::NodeTag::T_ParamPathInfo,
            357 => crate::__pgrx_c_bindings::NodeTag::T_ParamRef,
            407 => crate::__pgrx_c_bindings::NodeTag::T_PartitionBoundSpec,
            409 => crate::__pgrx_c_bindings::NodeTag::T_PartitionCmd,
            405 => crate::__pgrx_c_bindings::NodeTag::T_PartitionElem,
            57 => crate::__pgrx_c_bindings::NodeTag::T_PartitionPruneInfo,
            60 => crate::__pgrx_c_bindings::NodeTag::T_PartitionPruneStepCombine,
            59 => crate::__pgrx_c_bindings::NodeTag::T_PartitionPruneStepOp,
            408 => crate::__pgrx_c_bindings::NodeTag::T_PartitionRangeDatum,
            406 => crate::__pgrx_c_bindings::NodeTag::T_PartitionSpec,
            58 => crate::__pgrx_c_bindings::NodeTag::T_PartitionedRelPruneInfo,
            169 => crate::__pgrx_c_bindings::NodeTag::T_Path,
            207 => crate::__pgrx_c_bindings::NodeTag::T_PathKey,
            208 => crate::__pgrx_c_bindings::NodeTag::T_PathKeyInfo,
            209 => crate::__pgrx_c_bindings::NodeTag::T_PathTarget,
            216 => crate::__pgrx_c_bindings::NodeTag::T_PlaceHolderInfo,
            212 => crate::__pgrx_c_bindings::NodeTag::T_PlaceHolderVar,
            10 => crate::__pgrx_c_bindings::NodeTag::T_Plan,
            61 => crate::__pgrx_c_bindings::NodeTag::T_PlanInvalItem,
            56 => crate::__pgrx_c_bindings::NodeTag::T_PlanRowMark,
            62 => crate::__pgrx_c_bindings::NodeTag::T_PlanState,
            237 => crate::__pgrx_c_bindings::NodeTag::T_PlannedStmt,
            164 => crate::__pgrx_c_bindings::NodeTag::T_PlannerGlobal,
            163 => crate::__pgrx_c_bindings::NodeTag::T_PlannerInfo,
            218 => crate::__pgrx_c_bindings::NodeTag::T_PlannerParamItem,
            303 => crate::__pgrx_c_bindings::NodeTag::T_PrepareStmt,
            12 => crate::__pgrx_c_bindings::NodeTag::T_ProjectSet,
            191 => crate::__pgrx_c_bindings::NodeTag::T_ProjectSetPath,
            64 => crate::__pgrx_c_bindings::NodeTag::T_ProjectSetState,
            3 => crate::__pgrx_c_bindings::NodeTag::T_ProjectionInfo,
            190 => crate::__pgrx_c_bindings::NodeTag::T_ProjectionPath,
            411 => crate::__pgrx_c_bindings::NodeTag::T_PublicationObjSpec,
            412 => crate::__pgrx_c_bindings::NodeTag::T_PublicationTable,
            236 => crate::__pgrx_c_bindings::NodeTag::T_Query,
            371 => crate::__pgrx_c_bindings::NodeTag::T_RangeFunction,
            370 => crate::__pgrx_c_bindings::NodeTag::T_RangeSubselect,
            373 => crate::__pgrx_c_bindings::NodeTag::T_RangeTableFunc,
            374 => crate::__pgrx_c_bindings::NodeTag::T_RangeTableFuncCol,
            372 => crate::__pgrx_c_bindings::NodeTag::T_RangeTableSample,
            381 => crate::__pgrx_c_bindings::NodeTag::T_RangeTblEntry,
            382 => crate::__pgrx_c_bindings::NodeTag::T_RangeTblFunction,
            153 => crate::__pgrx_c_bindings::NodeTag::T_RangeTblRef,
            108 => crate::__pgrx_c_bindings::NodeTag::T_RangeVar,
            235 => crate::__pgrx_c_bindings::NodeTag::T_RawStmt,
            417 => crate::__pgrx_c_bindings::NodeTag::T_ReadReplicationSlotCmd,
            315 => crate::__pgrx_c_bindings::NodeTag::T_ReassignOwnedStmt,
            16 => crate::__pgrx_c_bindings::NodeTag::T_RecursiveUnion,
            201 => crate::__pgrx_c_bindings::NodeTag::T_RecursiveUnionPath,
            68 => crate::__pgrx_c_bindings::NodeTag::T_RecursiveUnionState,
            339 => crate::__pgrx_c_bindings::NodeTag::T_RefreshMatViewStmt,
            291 => crate::__pgrx_c_bindings::NodeTag::T_ReindexStmt,
            165 => crate::__pgrx_c_bindings::NodeTag::T_RelOptInfo,
            129 => crate::__pgrx_c_bindings::NodeTag::T_RelabelType,
            265 => crate::__pgrx_c_bindings::NodeTag::T_RenameStmt,
            340 => crate::__pgrx_c_bindings::NodeTag::T_ReplicaIdentityStmt,
            364 => crate::__pgrx_c_bindings::NodeTag::T_ResTarget,
            210 => crate::__pgrx_c_bindings::NodeTag::T_RestrictInfo,
            11 => crate::__pgrx_c_bindings::NodeTag::T_Result,
            7 => crate::__pgrx_c_bindings::NodeTag::T_ResultRelInfo,
            63 => crate::__pgrx_c_bindings::NodeTag::T_ResultState,
            422 => crate::__pgrx_c_bindings::NodeTag::T_ReturnSetInfo,
            243 => crate::__pgrx_c_bindings::NodeTag::T_ReturnStmt,
            403 => crate::__pgrx_c_bindings::NodeTag::T_RoleSpec,
            219 => crate::__pgrx_c_bindings::NodeTag::T_RollupData,
            139 => crate::__pgrx_c_bindings::NodeTag::T_RowCompareExpr,
            138 => crate::__pgrx_c_bindings::NodeTag::T_RowExpr,
            215 => crate::__pgrx_c_bindings::NodeTag::T_RowIdentityVarInfo,
            394 => crate::__pgrx_c_bindings::NodeTag::T_RowMarkClause,
            266 => crate::__pgrx_c_bindings::NodeTag::T_RuleStmt,
            142 => crate::__pgrx_c_bindings::NodeTag::T_SQLValueFunction,
            21 => crate::__pgrx_c_bindings::NodeTag::T_SampleScan,
            73 => crate::__pgrx_c_bindings::NodeTag::T_SampleScanState,
            122 => crate::__pgrx_c_bindings::NodeTag::T_ScalarArrayOpExpr,
            19 => crate::__pgrx_c_bindings::NodeTag::T_Scan,
            71 => crate::__pgrx_c_bindings::NodeTag::T_ScanState,
            331 => crate::__pgrx_c_bindings::NodeTag::T_SecLabelStmt,
            242 => crate::__pgrx_c_bindings::NodeTag::T_SelectStmt,
            20 => crate::__pgrx_c_bindings::NodeTag::T_SeqScan,
            72 => crate::__pgrx_c_bindings::NodeTag::T_SeqScanState,
            160 => crate::__pgrx_c_bindings::NodeTag::T_SetExprState,
            52 => crate::__pgrx_c_bindings::NodeTag::T_SetOp,
            200 => crate::__pgrx_c_bindings::NodeTag::T_SetOpPath,
            104 => crate::__pgrx_c_bindings::NodeTag::T_SetOpState,
            248 => crate::__pgrx_c_bindings::NodeTag::T_SetOperationStmt,
            148 => crate::__pgrx_c_bindings::NodeTag::T_SetToDefault,
            224 => crate::__pgrx_c_bindings::NodeTag::T_SlabContext,
            43 => crate::__pgrx_c_bindings::NodeTag::T_Sort,
            368 => crate::__pgrx_c_bindings::NodeTag::T_SortBy,
            385 => crate::__pgrx_c_bindings::NodeTag::T_SortGroupClause,
            192 => crate::__pgrx_c_bindings::NodeTag::T_SortPath,
            95 => crate::__pgrx_c_bindings::NodeTag::T_SortState,
            213 => crate::__pgrx_c_bindings::NodeTag::T_SpecialJoinInfo,
            418 => crate::__pgrx_c_bindings::NodeTag::T_StartReplicationCmd,
            221 => crate::__pgrx_c_bindings::NodeTag::T_StatisticExtInfo,
            378 => crate::__pgrx_c_bindings::NodeTag::T_StatsElem,
            229 => crate::__pgrx_c_bindings::NodeTag::T_String,
            124 => crate::__pgrx_c_bindings::NodeTag::T_SubLink,
            125 => crate::__pgrx_c_bindings::NodeTag::T_SubPlan,
            161 => crate::__pgrx_c_bindings::NodeTag::T_SubPlanState,
            28 => crate::__pgrx_c_bindings::NodeTag::T_SubqueryScan,
            176 => crate::__pgrx_c_bindings::NodeTag::T_SubqueryScanPath,
            80 => crate::__pgrx_c_bindings::NodeTag::T_SubqueryScanState,
            116 => crate::__pgrx_c_bindings::NodeTag::T_SubscriptingRef,
            434 => crate::__pgrx_c_bindings::NodeTag::T_SupportRequestCost,
            436 => crate::__pgrx_c_bindings::NodeTag::T_SupportRequestIndexCondition,
            435 => crate::__pgrx_c_bindings::NodeTag::T_SupportRequestRows,
            433 => crate::__pgrx_c_bindings::NodeTag::T_SupportRequestSelectivity,
            432 => crate::__pgrx_c_bindings::NodeTag::T_SupportRequestSimplify,
            437 => crate::__pgrx_c_bindings::NodeTag::T_SupportRequestWFuncMonotonic,
            424 => crate::__pgrx_c_bindings::NodeTag::T_TIDBitmap,
            428 => crate::__pgrx_c_bindings::NodeTag::T_TableAmRoutine,
            109 => crate::__pgrx_c_bindings::NodeTag::T_TableFunc,
            31 => crate::__pgrx_c_bindings::NodeTag::T_TableFuncScan,
            82 => crate::__pgrx_c_bindings::NodeTag::T_TableFuncScanState,
            391 => crate::__pgrx_c_bindings::NodeTag::T_TableLikeClause,
            383 => crate::__pgrx_c_bindings::NodeTag::T_TableSampleClause,
            152 => crate::__pgrx_c_bindings::NodeTag::T_TargetEntry,
            174 => crate::__pgrx_c_bindings::NodeTag::T_TidPath,
            175 => crate::__pgrx_c_bindings::NodeTag::T_TidRangePath,
            27 => crate::__pgrx_c_bindings::NodeTag::T_TidRangeScan,
            79 => crate::__pgrx_c_bindings::NodeTag::T_TidRangeScanState,
            26 => crate::__pgrx_c_bindings::NodeTag::T_TidScan,
            78 => crate::__pgrx_c_bindings::NodeTag::T_TidScanState,
            419 => crate::__pgrx_c_bindings::NodeTag::T_TimeLineHistoryCmd,
            270 => crate::__pgrx_c_bindings::NodeTag::T_TransactionStmt,
            420 => crate::__pgrx_c_bindings::NodeTag::T_TriggerData,
            404 => crate::__pgrx_c_bindings::NodeTag::T_TriggerTransition,
            258 => crate::__pgrx_c_bindings::NodeTag::T_TruncateStmt,
            429 => crate::__pgrx_c_bindings::NodeTag::T_TsmRoutine,
            9 => crate::__pgrx_c_bindings::NodeTag::T_TupleTableSlot,
            366 => crate::__pgrx_c_bindings::NodeTag::T_TypeCast,
            375 => crate::__pgrx_c_bindings::NodeTag::T_TypeName,
            48 => crate::__pgrx_c_bindings::NodeTag::T_Unique,
            187 => crate::__pgrx_c_bindings::NodeTag::T_UniquePath,
            100 => crate::__pgrx_c_bindings::NodeTag::T_UniqueState,
            269 => crate::__pgrx_c_bindings::NodeTag::T_UnlistenStmt,
            240 => crate::__pgrx_c_bindings::NodeTag::T_UpdateStmt,
            195 => crate::__pgrx_c_bindings::NodeTag::T_UpperUniquePath,
            410 => crate::__pgrx_c_bindings::NodeTag::T_VacuumRelation,
            276 => crate::__pgrx_c_bindings::NodeTag::T_VacuumStmt,
            30 => crate::__pgrx_c_bindings::NodeTag::T_ValuesScan,
            83 => crate::__pgrx_c_bindings::NodeTag::T_ValuesScanState,
            110 => crate::__pgrx_c_bindings::NodeTag::T_Var,
            281 => crate::__pgrx_c_bindings::NodeTag::T_VariableSetStmt,
            282 => crate::__pgrx_c_bindings::NodeTag::T_VariableShowStmt,
            271 => crate::__pgrx_c_bindings::NodeTag::T_ViewStmt,
            47 => crate::__pgrx_c_bindings::NodeTag::T_WindowAgg,
            199 => crate::__pgrx_c_bindings::NodeTag::T_WindowAggPath,
            99 => crate::__pgrx_c_bindings::NodeTag::T_WindowAggState,
            387 => crate::__pgrx_c_bindings::NodeTag::T_WindowClause,
            369 => crate::__pgrx_c_bindings::NodeTag::T_WindowDef,
            115 => crate::__pgrx_c_bindings::NodeTag::T_WindowFunc,
            159 => crate::__pgrx_c_bindings::NodeTag::T_WindowFuncExprState,
            423 => crate::__pgrx_c_bindings::NodeTag::T_WindowObjectData,
            384 => crate::__pgrx_c_bindings::NodeTag::T_WithCheckOption,
            396 => crate::__pgrx_c_bindings::NodeTag::T_WithClause,
            34 => crate::__pgrx_c_bindings::NodeTag::T_WorkTableScan,
            86 => crate::__pgrx_c_bindings::NodeTag::T_WorkTableScanState,
            143 => crate::__pgrx_c_bindings::NodeTag::T_XmlExpr,
            395 => crate::__pgrx_c_bindings::NodeTag::T_XmlSerialize,
            _ => panic!("C enum value has no corresponding Rust variant"),
        }
    }
}
impl c::sealed::Sealed for crate::__pgrx_c_bindings::NodeTag {}
impl c::expression::NativeType for crate::__pgrx_c_bindings::NodeTag {
    type Marker = c::expression::CEnumObject<
        EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
}
impl c::expression::IntoExpression for crate::__pgrx_c_bindings::NodeTag {
    type Value = c::CValue<c::expression::CEnum<EnumIdentity_enum__NodeTag, c::CUnsignedInt>>;
    fn into_expression(self) -> Self::Value {
        <c::expression::CEnumObject<
            EnumIdentity_enum__NodeTag,
            c::CUnsignedInt,
            crate::__pgrx_c_bindings::NodeTag,
        > as c::expression::CType>::from_storage(self)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__NullTestType;
impl c::sealed::Sealed for EnumIdentity_enum__NullTestType {}
impl c::expression::EnumIdentity for EnumIdentity_enum__NullTestType {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__NullTestType {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__ObjectType;
impl c::sealed::Sealed for EnumIdentity_enum__ObjectType {}
impl c::expression::EnumIdentity for EnumIdentity_enum__ObjectType {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__ObjectType {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__OnCommitAction;
impl c::sealed::Sealed for EnumIdentity_enum__OnCommitAction {}
impl c::expression::EnumIdentity for EnumIdentity_enum__OnCommitAction {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__OnCommitAction {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__OnConflictAction;
impl c::sealed::Sealed for EnumIdentity_enum__OnConflictAction {}
impl c::expression::EnumIdentity for EnumIdentity_enum__OnConflictAction {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__OnConflictAction {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__OutputPluginOutputType;
impl c::sealed::Sealed for EnumIdentity_enum__OutputPluginOutputType {}
impl c::expression::EnumIdentity for EnumIdentity_enum__OutputPluginOutputType {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__OutputPluginOutputType {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__OverridingKind;
impl c::sealed::Sealed for EnumIdentity_enum__OverridingKind {}
impl c::expression::EnumIdentity for EnumIdentity_enum__OverridingKind {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__OverridingKind {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__PLpgSQL_datum_type;
impl c::sealed::Sealed for EnumIdentity_enum__PLpgSQL_datum_type {}
impl c::expression::EnumIdentity for EnumIdentity_enum__PLpgSQL_datum_type {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__PLpgSQL_datum_type {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__PLpgSQL_getdiag_kind;
impl c::sealed::Sealed for EnumIdentity_enum__PLpgSQL_getdiag_kind {}
impl c::expression::EnumIdentity for EnumIdentity_enum__PLpgSQL_getdiag_kind {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__PLpgSQL_getdiag_kind {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__PLpgSQL_nsitem_type;
impl c::sealed::Sealed for EnumIdentity_enum__PLpgSQL_nsitem_type {}
impl c::expression::EnumIdentity for EnumIdentity_enum__PLpgSQL_nsitem_type {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__PLpgSQL_nsitem_type {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__PLpgSQL_promise_type;
impl c::sealed::Sealed for EnumIdentity_enum__PLpgSQL_promise_type {}
impl c::expression::EnumIdentity for EnumIdentity_enum__PLpgSQL_promise_type {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__PLpgSQL_promise_type {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__PLpgSQL_raise_option_type;
impl c::sealed::Sealed for EnumIdentity_enum__PLpgSQL_raise_option_type {}
impl c::expression::EnumIdentity for EnumIdentity_enum__PLpgSQL_raise_option_type {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__PLpgSQL_raise_option_type {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__PLpgSQL_resolve_option;
impl c::sealed::Sealed for EnumIdentity_enum__PLpgSQL_resolve_option {}
impl c::expression::EnumIdentity for EnumIdentity_enum__PLpgSQL_resolve_option {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__PLpgSQL_resolve_option {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__PLpgSQL_stmt_type;
impl c::sealed::Sealed for EnumIdentity_enum__PLpgSQL_stmt_type {}
impl c::expression::EnumIdentity for EnumIdentity_enum__PLpgSQL_stmt_type {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__PLpgSQL_stmt_type {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__PLpgSQL_trigtype;
impl c::sealed::Sealed for EnumIdentity_enum__PLpgSQL_trigtype {}
impl c::expression::EnumIdentity for EnumIdentity_enum__PLpgSQL_trigtype {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__PLpgSQL_trigtype {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__PLpgSQL_type_type;
impl c::sealed::Sealed for EnumIdentity_enum__PLpgSQL_type_type {}
impl c::expression::EnumIdentity for EnumIdentity_enum__PLpgSQL_type_type {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__PLpgSQL_type_type {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__ParallelHashGrowth;
impl c::sealed::Sealed for EnumIdentity_enum__ParallelHashGrowth {}
impl c::expression::EnumIdentity for EnumIdentity_enum__ParallelHashGrowth {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__ParallelHashGrowth {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__ParamKind;
impl c::sealed::Sealed for EnumIdentity_enum__ParamKind {}
impl c::expression::EnumIdentity for EnumIdentity_enum__ParamKind {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__ParamKind {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__ParseExprKind;
impl c::sealed::Sealed for EnumIdentity_enum__ParseExprKind {}
impl c::expression::EnumIdentity for EnumIdentity_enum__ParseExprKind {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__ParseExprKind {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__PartitionPruneCombineOp;
impl c::sealed::Sealed for EnumIdentity_enum__PartitionPruneCombineOp {}
impl c::expression::EnumIdentity for EnumIdentity_enum__PartitionPruneCombineOp {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__PartitionPruneCombineOp {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__PartitionRangeDatumKind;
impl c::sealed::Sealed for EnumIdentity_enum__PartitionRangeDatumKind {}
impl c::expression::EnumIdentity for EnumIdentity_enum__PartitionRangeDatumKind {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__PartitionRangeDatumKind {
    type Compatible = c::CInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__PortalStatus;
impl c::sealed::Sealed for EnumIdentity_enum__PortalStatus {}
impl c::expression::EnumIdentity for EnumIdentity_enum__PortalStatus {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__PortalStatus {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__PortalStrategy;
impl c::sealed::Sealed for EnumIdentity_enum__PortalStrategy {}
impl c::expression::EnumIdentity for EnumIdentity_enum__PortalStrategy {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__PortalStrategy {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__ProcessingMode;
impl c::sealed::Sealed for EnumIdentity_enum__ProcessingMode {}
impl c::expression::EnumIdentity for EnumIdentity_enum__ProcessingMode {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__ProcessingMode {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__ProgressCommandType;
impl c::sealed::Sealed for EnumIdentity_enum__ProgressCommandType {}
impl c::expression::EnumIdentity for EnumIdentity_enum__ProgressCommandType {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__ProgressCommandType {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__PublicationObjSpecType;
impl c::sealed::Sealed for EnumIdentity_enum__PublicationObjSpecType {}
impl c::expression::EnumIdentity for EnumIdentity_enum__PublicationObjSpecType {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__PublicationObjSpecType {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__QuerySource;
impl c::sealed::Sealed for EnumIdentity_enum__QuerySource {}
impl c::expression::EnumIdentity for EnumIdentity_enum__QuerySource {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__QuerySource {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__RTEKind;
impl c::sealed::Sealed for EnumIdentity_enum__RTEKind {}
impl c::expression::EnumIdentity for EnumIdentity_enum__RTEKind {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__RTEKind {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__ReindexObjectType;
impl c::sealed::Sealed for EnumIdentity_enum__ReindexObjectType {}
impl c::expression::EnumIdentity for EnumIdentity_enum__ReindexObjectType {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__ReindexObjectType {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__RelOptKind;
impl c::sealed::Sealed for EnumIdentity_enum__RelOptKind {}
impl c::expression::EnumIdentity for EnumIdentity_enum__RelOptKind {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__RelOptKind {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__ReorderBufferChangeType;
impl c::sealed::Sealed for EnumIdentity_enum__ReorderBufferChangeType {}
impl c::expression::EnumIdentity for EnumIdentity_enum__ReorderBufferChangeType {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__ReorderBufferChangeType {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__ReplicationKind;
impl c::sealed::Sealed for EnumIdentity_enum__ReplicationKind {}
impl c::expression::EnumIdentity for EnumIdentity_enum__ReplicationKind {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__ReplicationKind {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__ReplicationSlotPersistency;
impl c::sealed::Sealed for EnumIdentity_enum__ReplicationSlotPersistency {}
impl c::expression::EnumIdentity for EnumIdentity_enum__ReplicationSlotPersistency {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__ReplicationSlotPersistency {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__RoleSpecType;
impl c::sealed::Sealed for EnumIdentity_enum__RoleSpecType {}
impl c::expression::EnumIdentity for EnumIdentity_enum__RoleSpecType {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__RoleSpecType {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__RoleStmtType;
impl c::sealed::Sealed for EnumIdentity_enum__RoleStmtType {}
impl c::expression::EnumIdentity for EnumIdentity_enum__RoleStmtType {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__RoleStmtType {
    type Compatible = c::CUnsignedInt;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__RowCompareType;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
impl c::sealed::Sealed for EnumIdentity_enum__RowCompareType {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
impl c::expression::EnumIdentity for EnumIdentity_enum__RowCompareType {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__RowCompareType {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__RowMarkType;
impl c::sealed::Sealed for EnumIdentity_enum__RowMarkType {}
impl c::expression::EnumIdentity for EnumIdentity_enum__RowMarkType {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__RowMarkType {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__SQLValueFunctionOp;
impl c::sealed::Sealed for EnumIdentity_enum__SQLValueFunctionOp {}
impl c::expression::EnumIdentity for EnumIdentity_enum__SQLValueFunctionOp {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__SQLValueFunctionOp {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__ScanDirection;
impl c::sealed::Sealed for EnumIdentity_enum__ScanDirection {}
impl c::expression::EnumIdentity for EnumIdentity_enum__ScanDirection {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__ScanDirection {
    type Compatible = c::CInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__SetOpCmd;
impl c::sealed::Sealed for EnumIdentity_enum__SetOpCmd {}
impl c::expression::EnumIdentity for EnumIdentity_enum__SetOpCmd {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__SetOpCmd {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__SetOpStrategy;
impl c::sealed::Sealed for EnumIdentity_enum__SetOpStrategy {}
impl c::expression::EnumIdentity for EnumIdentity_enum__SetOpStrategy {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__SetOpStrategy {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__SetOperation;
impl c::sealed::Sealed for EnumIdentity_enum__SetOperation {}
impl c::expression::EnumIdentity for EnumIdentity_enum__SetOperation {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__SetOperation {
    type Compatible = c::CUnsignedInt;
}
#[cfg(feature = "pg15")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__SetQuantifier;
#[cfg(feature = "pg15")]
impl c::sealed::Sealed for EnumIdentity_enum__SetQuantifier {}
#[cfg(feature = "pg15")]
impl c::expression::EnumIdentity for EnumIdentity_enum__SetQuantifier {}
#[cfg(feature = "pg15")]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__SetQuantifier {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__SnapshotType;
impl c::sealed::Sealed for EnumIdentity_enum__SnapshotType {}
impl c::expression::EnumIdentity for EnumIdentity_enum__SnapshotType {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__SnapshotType {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__SortByDir;
impl c::sealed::Sealed for EnumIdentity_enum__SortByDir {}
impl c::expression::EnumIdentity for EnumIdentity_enum__SortByDir {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__SortByDir {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__SortByNulls;
impl c::sealed::Sealed for EnumIdentity_enum__SortByNulls {}
impl c::expression::EnumIdentity for EnumIdentity_enum__SortByNulls {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__SortByNulls {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__StdRdOptIndexCleanup;
impl c::sealed::Sealed for EnumIdentity_enum__StdRdOptIndexCleanup {}
impl c::expression::EnumIdentity for EnumIdentity_enum__StdRdOptIndexCleanup {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__StdRdOptIndexCleanup {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__SubLinkType;
impl c::sealed::Sealed for EnumIdentity_enum__SubLinkType {}
impl c::expression::EnumIdentity for EnumIdentity_enum__SubLinkType {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__SubLinkType {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__SubqueryScanStatus;
impl c::sealed::Sealed for EnumIdentity_enum__SubqueryScanStatus {}
impl c::expression::EnumIdentity for EnumIdentity_enum__SubqueryScanStatus {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__SubqueryScanStatus {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__SyncRequestHandler;
impl c::sealed::Sealed for EnumIdentity_enum__SyncRequestHandler {}
impl c::expression::EnumIdentity for EnumIdentity_enum__SyncRequestHandler {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__SyncRequestHandler {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__TM_Result;
impl c::sealed::Sealed for EnumIdentity_enum__TM_Result {}
impl c::expression::EnumIdentity for EnumIdentity_enum__TM_Result {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__TM_Result {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__TransactionStmtKind;
impl c::sealed::Sealed for EnumIdentity_enum__TransactionStmtKind {}
impl c::expression::EnumIdentity for EnumIdentity_enum__TransactionStmtKind {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__TransactionStmtKind {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__TwoPhasePredicateRecordType;
impl c::sealed::Sealed for EnumIdentity_enum__TwoPhasePredicateRecordType {}
impl c::expression::EnumIdentity for EnumIdentity_enum__TwoPhasePredicateRecordType {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__TwoPhasePredicateRecordType {
    type Compatible = c::CUnsignedInt;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__UniquePathMethod;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::sealed::Sealed for EnumIdentity_enum__UniquePathMethod {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::EnumIdentity for EnumIdentity_enum__UniquePathMethod {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__UniquePathMethod {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__VacOptValue;
impl c::sealed::Sealed for EnumIdentity_enum__VacOptValue {}
impl c::expression::EnumIdentity for EnumIdentity_enum__VacOptValue {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__VacOptValue {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__VariableSetKind;
impl c::sealed::Sealed for EnumIdentity_enum__VariableSetKind {}
impl c::expression::EnumIdentity for EnumIdentity_enum__VariableSetKind {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__VariableSetKind {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__ViewCheckOption;
impl c::sealed::Sealed for EnumIdentity_enum__ViewCheckOption {}
impl c::expression::EnumIdentity for EnumIdentity_enum__ViewCheckOption {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__ViewCheckOption {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__ViewOptCheckOption;
impl c::sealed::Sealed for EnumIdentity_enum__ViewOptCheckOption {}
impl c::expression::EnumIdentity for EnumIdentity_enum__ViewOptCheckOption {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__ViewOptCheckOption {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__VolatileFunctionStatus;
impl c::sealed::Sealed for EnumIdentity_enum__VolatileFunctionStatus {}
impl c::expression::EnumIdentity for EnumIdentity_enum__VolatileFunctionStatus {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__VolatileFunctionStatus {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__WCOKind;
impl c::sealed::Sealed for EnumIdentity_enum__WCOKind {}
impl c::expression::EnumIdentity for EnumIdentity_enum__WCOKind {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__WCOKind {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__WalSndState;
impl c::sealed::Sealed for EnumIdentity_enum__WalSndState {}
impl c::expression::EnumIdentity for EnumIdentity_enum__WalSndState {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__WalSndState {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__WindowAggStatus;
impl c::sealed::Sealed for EnumIdentity_enum__WindowAggStatus {}
impl c::expression::EnumIdentity for EnumIdentity_enum__WindowAggStatus {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__WindowAggStatus {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__XmlExprOp;
impl c::sealed::Sealed for EnumIdentity_enum__XmlExprOp {}
impl c::expression::EnumIdentity for EnumIdentity_enum__XmlExprOp {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__XmlExprOp {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__XmlOptionType;
impl c::sealed::Sealed for EnumIdentity_enum__XmlOptionType {}
impl c::expression::EnumIdentity for EnumIdentity_enum__XmlOptionType {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__XmlOptionType {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__0x5f5f7069645f74797065;
impl c::sealed::Sealed for EnumIdentity_enum__0x5f5f7069645f74797065 {}
impl c::expression::EnumIdentity for EnumIdentity_enum__0x5f5f7069645f74797065 {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__0x5f5f7069645f74797065 {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__config_group;
impl c::sealed::Sealed for EnumIdentity_enum__config_group {}
impl c::expression::EnumIdentity for EnumIdentity_enum__config_group {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__config_group {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__config_type;
impl c::sealed::Sealed for EnumIdentity_enum__config_type {}
impl c::expression::EnumIdentity for EnumIdentity_enum__config_type {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__config_type {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__jbvType;
impl c::sealed::Sealed for EnumIdentity_enum__jbvType {}
impl c::expression::EnumIdentity for EnumIdentity_enum__jbvType {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__jbvType {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__pg_enc;
impl c::sealed::Sealed for EnumIdentity_enum__pg_enc {}
impl c::expression::EnumIdentity for EnumIdentity_enum__pg_enc {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__pg_enc {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__relopt_type;
impl c::sealed::Sealed for EnumIdentity_enum__relopt_type {}
impl c::expression::EnumIdentity for EnumIdentity_enum__relopt_type {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__relopt_type {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__spgChooseResultType;
impl c::sealed::Sealed for EnumIdentity_enum__spgChooseResultType {}
impl c::expression::EnumIdentity for EnumIdentity_enum__spgChooseResultType {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__spgChooseResultType {
    type Compatible = c::CUnsignedInt;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_subxids_array_status;
impl c::sealed::Sealed for EnumIdentity_subxids_array_status {}
impl c::expression::EnumIdentity for EnumIdentity_subxids_array_status {}
impl c::expression::enumeration::NumericEnum for EnumIdentity_subxids_array_status {
    type Compatible = c::CUnsignedInt;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::BOX,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::BOX,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_BOX__0p__0l__unsigned__long__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_BOX__0p__0l__unsigned__long__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_BOX__0p__0l__unsigned__long__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::BOX,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::CIRCLE,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::CIRCLE,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_CIRCLE__0p__0l__unsigned__long__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_CIRCLE__0p__0l__unsigned__long__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_CIRCLE__0p__0l__unsigned__long__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::CIRCLE,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::Interval,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::Interval,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_Interval__0p__0l__unsigned__long__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_Interval__0p__0l__unsigned__long__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_Interval__0p__0l__unsigned__long__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::Interval,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::Jsonb,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::Jsonb,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_Jsonb__0p__0l__unsigned__long__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_Jsonb__0p__0l__unsigned__long__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_Jsonb__0p__0l__unsigned__long__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::Jsonb,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::LINE,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::LINE,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_LINE__0p__0l__unsigned__long__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_LINE__0p__0l__unsigned__long__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_LINE__0p__0l__unsigned__long__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::LINE,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::LSEG,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::LSEG,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_LSEG__0p__0l__unsigned__long__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_LSEG__0p__0l__unsigned__long__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_LSEG__0p__0l__unsigned__long__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::LSEG,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::PATH,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::PATH,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_PATH__0p__0l__unsigned__long__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_PATH__0p__0l__unsigned__long__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_PATH__0p__0l__unsigned__long__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::PATH,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::POLYGON,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::POLYGON,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_POLYGON__0p__0l__unsigned__long__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_POLYGON__0p__0l__unsigned__long__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_POLYGON__0p__0l__unsigned__long__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::POLYGON,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::Point,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::Point,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_Point__0p__0l__unsigned__long__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_Point__0p__0l__unsigned__long__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_Point__0p__0l__unsigned__long__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::Point,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::RangeType,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::RangeType,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_RangeType__0p__0l__unsigned__long__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_RangeType__0p__0l__unsigned__long__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_RangeType__0p__0l__unsigned__long__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::RangeType,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::TSQueryData,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::TSQueryData,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_TSQueryData__0p__0l__unsigned__long__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_TSQueryData__0p__0l__unsigned__long__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_TSQueryData__0p__0l__unsigned__long__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::TSQueryData,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::TSVectorData,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::TSVectorData,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_TSVectorData__0p__0l__unsigned__long__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_TSVectorData__0p__0l__unsigned__long__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_TSVectorData__0p__0l__unsigned__long__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::TSVectorData,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::TimeTzADT,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::TimeTzADT,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_TimeTzADT__0p__0l__unsigned__long__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_TimeTzADT__0p__0l__unsigned__long__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_TimeTzADT__0p__0l__unsigned__long__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::TimeTzADT,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::List,
                *mut crate::__pgrx_c_bindings::List,
                bool,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::List,
                *mut crate::__pgrx_c_bindings::List,
                bool,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__List__0p__0c__struct__List__0p__0c__0x5f426f6f6c__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed
    for Signature_0x5f426f6f6c__0l__struct__List__0p__0c__struct__List__0p__0c__0x5f426f6f6c__0r
{
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__struct__List__0p__0c__struct__List__0p__0c__0x5f426f6f6c__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::List,
                *mut crate::__pgrx_c_bindings::List,
                bool,
            ) -> bool,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::Node,
                *mut ::core::ffi::c_void,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::Node,
                *mut ::core::ffi::c_void,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__Node__0p__0c__void__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__Node__0p__0c__void__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__struct__Node__0p__0c__void__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::Node,
                *mut ::core::ffi::c_void,
            ) -> bool,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlanState,
                *mut ::core::ffi::c_void,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlanState,
                *mut ::core::ffi::c_void,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__PlanState__0p__0c__void__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__PlanState__0p__0c__void__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__struct__PlanState__0p__0c__void__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlanState,
                *mut ::core::ffi::c_void,
            ) -> bool,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_char,
                *mut crate::__pgrx_c_bindings::RelationData,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_char,
                *mut crate::__pgrx_c_bindings::RelationData,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__char__0p__0c__struct__RelationData__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__char__0p__0c__struct__RelationData__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__char__0p__0c__struct__RelationData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_char,
                *mut crate::__pgrx_c_bindings::RelationData,
            ) -> bool,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::ItemPointerData) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::ItemPointerData) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__const__struct__ItemPointerData__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__const__struct__ItemPointerData__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__const__struct__ItemPointerData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::ItemPointerData) -> bool,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::dclist_head) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::dclist_head) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__const__struct__dclist_head__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__const__struct__dclist_head__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__const__struct__dclist_head__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::dclist_head) -> bool,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::dclist_head,
                *const crate::__pgrx_c_bindings::dlist_node,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::dclist_head,
                *const crate::__pgrx_c_bindings::dlist_node,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__const__struct__dclist_head__0p__0c__const__struct__dlist_node__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__const__struct__dclist_head__0p__0c__const__struct__dlist_node__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__const__struct__dclist_head__0p__0c__const__struct__dlist_node__0p__0r {
 type Physical = PhysicalFunction_C_unwind_2<::core::option::Option<unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::dclist_head, *const crate::__pgrx_c_bindings::dlist_node) -> bool>>;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::dlist_head) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::dlist_head) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__const__struct__dlist_head__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__const__struct__dlist_head__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__const__struct__dlist_head__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::dlist_head) -> bool,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::dlist_node) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::dlist_node) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__const__struct__dlist_node__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__const__struct__dlist_node__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__const__struct__dlist_node__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::dlist_node) -> bool,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::proclist_head,
                i32,
                usize,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::proclist_head,
                i32,
                usize,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__const__struct__proclist_head__0p__0c__int__0c__unsigned__long__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__const__struct__proclist_head__0p__0c__int__0c__unsigned__long__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__const__struct__proclist_head__0p__0c__int__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::proclist_head, i32, usize) -> bool>>;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::slist_head) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::slist_head) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__const__struct__slist_head__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__const__struct__slist_head__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__const__struct__slist_head__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::slist_head) -> bool,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::slist_head,
                *const crate::__pgrx_c_bindings::slist_node,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::slist_head,
                *const crate::__pgrx_c_bindings::slist_node,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__const__struct__slist_head__0p__0c__const__struct__slist_node__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__const__struct__slist_head__0p__0c__const__struct__slist_node__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__const__struct__slist_head__0p__0c__const__struct__slist_node__0p__0r {
 type Physical = PhysicalFunction_C_unwind_2<::core::option::Option<unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::slist_head, *const crate::__pgrx_c_bindings::slist_node) -> bool>>;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::HeapTupleData,
                *mut crate::__pgrx_c_bindings::TupleDescData,
                i32,
                *mut crate::__pgrx_c_bindings::ScanKeyData,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::HeapTupleData,
                *mut crate::__pgrx_c_bindings::TupleDescData,
                i32,
                *mut crate::__pgrx_c_bindings::ScanKeyData,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__HeapTupleData__0p__0c__struct__TupleDescData__0p__0c__int__0c__struct__ScanKeyData__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__HeapTupleData__0p__0c__struct__TupleDescData__0p__0c__int__0c__struct__ScanKeyData__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__struct__HeapTupleData__0p__0c__struct__TupleDescData__0p__0c__int__0c__struct__ScanKeyData__0p__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::HeapTupleData, *mut crate::__pgrx_c_bindings::TupleDescData, i32, *mut crate::__pgrx_c_bindings::ScanKeyData) -> bool>>;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::List,
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::Node,
                        *mut ::core::ffi::c_void,
                    ) -> bool,
                >,
                *mut ::core::ffi::c_void,
                i32,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::List,
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::Node,
                        *mut ::core::ffi::c_void,
                    ) -> bool,
                >,
                *mut ::core::ffi::c_void,
                i32,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__List__0p__0c__0x5f426f6f6c__0l__0p__0r__0l__struct__Node__0p__0c__void__0p__0r__0c__void__0p__0c__int__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__List__0p__0c__0x5f426f6f6c__0l__0p__0r__0l__struct__Node__0p__0c__void__0p__0r__0c__void__0p__0c__int__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__struct__List__0p__0c__0x5f426f6f6c__0l__0p__0r__0l__struct__Node__0p__0c__void__0p__0r__0c__void__0p__0c__int__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::List, ::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::Node, *mut ::core::ffi::c_void) -> bool>, *mut ::core::ffi::c_void, i32) -> bool>>;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::Node,
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::Node,
                        *mut ::core::ffi::c_void,
                    ) -> bool,
                >,
                *mut ::core::ffi::c_void,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::Node,
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::Node,
                        *mut ::core::ffi::c_void,
                    ) -> bool,
                >,
                *mut ::core::ffi::c_void,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__Node__0p__0c__0x5f426f6f6c__0l__0p__0r__0l__struct__Node__0p__0c__void__0p__0r__0c__void__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__Node__0p__0c__0x5f426f6f6c__0l__0p__0r__0l__struct__Node__0p__0c__void__0p__0r__0c__void__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__struct__Node__0p__0c__0x5f426f6f6c__0l__0p__0r__0l__struct__Node__0p__0c__void__0p__0r__0c__void__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::Node, ::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::Node, *mut ::core::ffi::c_void) -> bool>, *mut ::core::ffi::c_void) -> bool>>;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::Node,
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::Node,
                        *mut ::core::ffi::c_void,
                    ) -> bool,
                >,
                *mut ::core::ffi::c_void,
                i32,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::Node,
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::Node,
                        *mut ::core::ffi::c_void,
                    ) -> bool,
                >,
                *mut ::core::ffi::c_void,
                i32,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__Node__0p__0c__0x5f426f6f6c__0l__0p__0r__0l__struct__Node__0p__0c__void__0p__0r__0c__void__0p__0c__int__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__Node__0p__0c__0x5f426f6f6c__0l__0p__0r__0l__struct__Node__0p__0c__void__0p__0r__0c__void__0p__0c__int__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__struct__Node__0p__0c__0x5f426f6f6c__0l__0p__0r__0l__struct__Node__0p__0c__void__0p__0r__0c__void__0p__0c__int__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::Node, ::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::Node, *mut ::core::ffi::c_void) -> bool>, *mut ::core::ffi::c_void, i32) -> bool>>;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlanState,
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::PlanState,
                        *mut ::core::ffi::c_void,
                    ) -> bool,
                >,
                *mut ::core::ffi::c_void,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlanState,
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::PlanState,
                        *mut ::core::ffi::c_void,
                    ) -> bool,
                >,
                *mut ::core::ffi::c_void,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__PlanState__0p__0c__0x5f426f6f6c__0l__0p__0r__0l__struct__PlanState__0p__0c__void__0p__0r__0c__void__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__PlanState__0p__0c__0x5f426f6f6c__0l__0p__0r__0l__struct__PlanState__0p__0c__void__0p__0r__0c__void__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__struct__PlanState__0p__0c__0x5f426f6f6c__0l__0p__0r__0l__struct__PlanState__0p__0c__void__0p__0r__0c__void__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PlanState, ::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PlanState, *mut ::core::ffi::c_void) -> bool>, *mut ::core::ffi::c_void) -> bool>>;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::Query,
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::Node,
                        *mut ::core::ffi::c_void,
                    ) -> bool,
                >,
                *mut ::core::ffi::c_void,
                i32,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::Query,
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::Node,
                        *mut ::core::ffi::c_void,
                    ) -> bool,
                >,
                *mut ::core::ffi::c_void,
                i32,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__Query__0p__0c__0x5f426f6f6c__0l__0p__0r__0l__struct__Node__0p__0c__void__0p__0r__0c__void__0p__0c__int__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__Query__0p__0c__0x5f426f6f6c__0l__0p__0r__0l__struct__Node__0p__0c__void__0p__0r__0c__void__0p__0c__int__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__struct__Query__0p__0c__0x5f426f6f6c__0l__0p__0r__0l__struct__Node__0p__0c__void__0p__0r__0c__void__0p__0c__int__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::Query, ::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::Node, *mut ::core::ffi::c_void) -> bool>, *mut ::core::ffi::c_void, i32) -> bool>>;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RangeTblEntry,
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::Node,
                        *mut ::core::ffi::c_void,
                    ) -> bool,
                >,
                *mut ::core::ffi::c_void,
                i32,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RangeTblEntry,
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::Node,
                        *mut ::core::ffi::c_void,
                    ) -> bool,
                >,
                *mut ::core::ffi::c_void,
                i32,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__RangeTblEntry__0p__0c__0x5f426f6f6c__0l__0p__0r__0l__struct__Node__0p__0c__void__0p__0r__0c__void__0p__0c__int__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__RangeTblEntry__0p__0c__0x5f426f6f6c__0l__0p__0r__0l__struct__Node__0p__0c__void__0p__0r__0c__void__0p__0c__int__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__struct__RangeTblEntry__0p__0c__0x5f426f6f6c__0l__0p__0r__0l__struct__Node__0p__0c__void__0p__0r__0c__void__0p__0c__int__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RangeTblEntry, ::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::Node, *mut ::core::ffi::c_void) -> bool>, *mut ::core::ffi::c_void, i32) -> bool>>;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Datum) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Datum) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__unsigned__long__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__unsigned__long__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__unsigned__long__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Datum) -> bool,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Datum) -> i8>,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Datum) -> i8>,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_char__0l__unsigned__long__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_char__0l__unsigned__long__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_char__0l__unsigned__long__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Datum) -> i8>,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_char,
                *mut crate::__pgrx_c_bindings::ItemIdData,
            ) -> *mut ::core::ffi::c_char,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_char,
                *mut crate::__pgrx_c_bindings::ItemIdData,
            ) -> *mut ::core::ffi::c_char,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_char__0p__0l__char__0p__0c__struct__ItemIdData__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17"))]
impl c::sealed::Sealed for Signature_char__0p__0l__char__0p__0c__struct__ItemIdData__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17"))]
impl c::expression::NativeFunctionSignature
    for Signature_char__0p__0l__char__0p__0c__struct__ItemIdData__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_char,
                *mut crate::__pgrx_c_bindings::ItemIdData,
            ) -> *mut ::core::ffi::c_char,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut ::core::ffi::c_char,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut ::core::ffi::c_char,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_char__0p__0l__unsigned__long__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_char__0p__0l__unsigned__long__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_char__0p__0l__unsigned__long__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut ::core::ffi::c_char,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::ItemPointerData,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
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
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::ItemPointerData,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
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
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_enum__TM_Result__0l__struct__RelationData__0p__0c__struct__ItemPointerData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__struct__SnapshotData__0p__0c__struct__SnapshotData__0p__0c__0x5f426f6f6c__0c__struct__TM_FailureData__0p__0c__enum__LockTupleMode__0p__0c__enum__TU_UpdateIndexes__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::sealed::Sealed for Signature_enum__TM_Result__0l__struct__RelationData__0p__0c__struct__ItemPointerData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__struct__SnapshotData__0p__0c__struct__SnapshotData__0p__0c__0x5f426f6f6c__0c__struct__TM_FailureData__0p__0c__enum__LockTupleMode__0p__0c__enum__TU_UpdateIndexes__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::NativeFunctionSignature for Signature_enum__TM_Result__0l__struct__RelationData__0p__0c__struct__ItemPointerData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__struct__SnapshotData__0p__0c__struct__SnapshotData__0p__0c__0x5f426f6f6c__0c__struct__TM_FailureData__0p__0c__enum__LockTupleMode__0p__0c__enum__TU_UpdateIndexes__0p__0r {
 type Physical = PhysicalFunction_C_unwind_10<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::ItemPointerData, *mut crate::__pgrx_c_bindings::TupleTableSlot, u32, *mut crate::__pgrx_c_bindings::SnapshotData, *mut crate::__pgrx_c_bindings::SnapshotData, bool, *mut crate::__pgrx_c_bindings::TM_FailureData, *mut u32, *mut u32) -> u32>>;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<A0: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>, c::expression::ReadWrite>>, A1: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::ItemPointerData>, c::expression::ReadWrite>>, A2: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot>, c::expression::ReadWrite>>, A3: c::expression::ImplicitTo<c::CUnsignedInt>, A4: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData>, c::expression::ReadWrite>>, A5: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData>, c::expression::ReadWrite>>, A6: c::expression::ImplicitTo<c::CBool>, A7: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TM_FailureData>, c::expression::ReadWrite>>, A8: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CEnumObject<crate::__pgrx_c_generated::EnumIdentity_enum__LockTupleMode, c::CUnsignedInt, u32>, c::expression::ReadWrite>>, A9: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CEnumObject<crate::__pgrx_c_generated::EnumIdentity_enum__TU_UpdateIndexes, c::CUnsignedInt, u32>, c::expression::ReadWrite>>> c::expression::Call<(A0,A1,A2,A3,A4,A5,A6,A7,A8,A9,)> for Signature_enum__TM_Result__0l__struct__RelationData__0p__0c__struct__ItemPointerData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__struct__SnapshotData__0p__0c__struct__SnapshotData__0p__0c__0x5f426f6f6c__0c__struct__TM_FailureData__0p__0c__enum__LockTupleMode__0p__0c__enum__TU_UpdateIndexes__0p__0r {
 type Output = <c::expression::CEnumObject<crate::__pgrx_c_generated::EnumIdentity_enum__TM_Result, c::CUnsignedInt, u32> as c::expression::CType>::Value;
 unsafe fn call(pointer: Self::Pointer, args: (A0,A1,A2,A3,A4,A5,A6,A7,A8,A9,)) -> Self::Output {
let function = pointer.expect("C indirect call requires a non-null function pointer");
const { assert!(!::core::mem::needs_drop::<A0>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::RelationData>());
let native0 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>, c::expression::ReadWrite>, _>(args.0));
const { assert!(!::core::mem::needs_drop::<A1>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::ItemPointerData>());
let native1 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::ItemPointerData>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::ItemPointerData>, c::expression::ReadWrite>, _>(args.1));
const { assert!(!::core::mem::needs_drop::<A2>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::TupleTableSlot>());
let native2 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot>, c::expression::ReadWrite>, _>(args.2));
const { assert!(!::core::mem::needs_drop::<A3>()); }
const _: () = assert!(!::core::mem::needs_drop::<u32>());
let native3 = <c::CUnsignedInt as c::expression::CType>::into_storage(c::expression::implicit::<c::CUnsignedInt, _>(args.3));
const { assert!(!::core::mem::needs_drop::<A4>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::SnapshotData>());
let native4 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData>, c::expression::ReadWrite>, _>(args.4));
const { assert!(!::core::mem::needs_drop::<A5>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::SnapshotData>());
let native5 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData>, c::expression::ReadWrite>, _>(args.5));
const { assert!(!::core::mem::needs_drop::<A6>()); }
const _: () = assert!(!::core::mem::needs_drop::<bool>());
let native6 = <c::CBool as c::expression::CType>::into_storage(c::expression::implicit::<c::CBool, _>(args.6));
const { assert!(!::core::mem::needs_drop::<A7>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::TM_FailureData>());
let native7 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TM_FailureData>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TM_FailureData>, c::expression::ReadWrite>, _>(args.7));
const { assert!(!::core::mem::needs_drop::<A8>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut u32>());
let native8 = <c::expression::CPointer<c::expression::CEnumObject<crate::__pgrx_c_generated::EnumIdentity_enum__LockTupleMode, c::CUnsignedInt, u32>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CEnumObject<crate::__pgrx_c_generated::EnumIdentity_enum__LockTupleMode, c::CUnsignedInt, u32>, c::expression::ReadWrite>, _>(args.8));
const { assert!(!::core::mem::needs_drop::<A9>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut u32>());
let native9 = <c::expression::CPointer<c::expression::CEnumObject<crate::__pgrx_c_generated::EnumIdentity_enum__TU_UpdateIndexes, c::CUnsignedInt, u32>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CEnumObject<crate::__pgrx_c_generated::EnumIdentity_enum__TU_UpdateIndexes, c::CUnsignedInt, u32>, c::expression::ReadWrite>, _>(args.9));
// SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
let result = unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0, native1, native2, native3, native4, native5, native6, native7, native8, native9)) };
<c::expression::CEnumObject<crate::__pgrx_c_generated::EnumIdentity_enum__TM_Result, c::CUnsignedInt, u32> as c::expression::CType>::from_storage(result)
}
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::SortTuple,
                *const crate::__pgrx_c_bindings::SortTuple,
                *mut crate::__pgrx_c_bindings::Tuplesortstate,
            ) -> i32,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::SortTuple,
                *const crate::__pgrx_c_bindings::SortTuple,
                *mut crate::__pgrx_c_bindings::Tuplesortstate,
            ) -> i32,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__const__SortTuple__0p__0c__const__SortTuple__0p__0c__struct__Tuplesortstate__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_int__0l__const__SortTuple__0p__0c__const__SortTuple__0p__0c__struct__Tuplesortstate__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_int__0l__const__SortTuple__0p__0c__const__SortTuple__0p__0c__struct__Tuplesortstate__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::SortTuple, *const crate::__pgrx_c_bindings::SortTuple, *mut crate::__pgrx_c_bindings::Tuplesortstate) -> i32>>;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::List) -> i32,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::List) -> i32,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__const__struct__List__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_int__0l__const__struct__List__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_int__0l__const__struct__List__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::List) -> i32,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn(i64) -> i64>>() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn(i64) -> i64>>() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_long__0l__long__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_long__0l__long__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_long__0l__long__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(i64) -> i64>,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Datum) -> i64>,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Datum) -> i64>,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_long__0l__unsigned__long__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::sealed::Sealed for Signature_long__0l__unsigned__long__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::NativeFunctionSignature for Signature_long__0l__unsigned__long__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Datum) -> i64>,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Datum) -> i16>,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Datum) -> i16>,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_short__0l__unsigned__long__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_short__0l__unsigned__long__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_short__0l__unsigned__long__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::Datum) -> i16>,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            )
                -> *mut crate::__pgrx_c_bindings::ItemPointerData,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            )
                -> *mut crate::__pgrx_c_bindings::ItemPointerData,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__ItemPointerData__0p__0l__unsigned__long__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_struct__ItemPointerData__0p__0l__unsigned__long__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_struct__ItemPointerData__0p__0l__unsigned__long__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            )
                -> *mut crate::__pgrx_c_bindings::ItemPointerData,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(u32) -> *mut crate::__pgrx_c_bindings::LWLock,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(u32) -> *mut crate::__pgrx_c_bindings::LWLock,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__LWLock__0p__0l__unsigned__int__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_struct__LWLock__0p__0l__unsigned__int__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_struct__LWLock__0p__0l__unsigned__int__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(u32) -> *mut crate::__pgrx_c_bindings::LWLock,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::List,
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::Node,
                        *mut ::core::ffi::c_void,
                    )
                        -> *mut crate::__pgrx_c_bindings::Node,
                >,
                *mut ::core::ffi::c_void,
                i32,
            ) -> *mut crate::__pgrx_c_bindings::List,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::List,
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::Node,
                        *mut ::core::ffi::c_void,
                    )
                        -> *mut crate::__pgrx_c_bindings::Node,
                >,
                *mut ::core::ffi::c_void,
                i32,
            ) -> *mut crate::__pgrx_c_bindings::List,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__List__0p__0l__struct__List__0p__0c__struct__Node__0p__0l__0p__0r__0l__struct__Node__0p__0c__void__0p__0r__0c__void__0p__0c__int__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_struct__List__0p__0l__struct__List__0p__0c__struct__Node__0p__0l__0p__0r__0l__struct__Node__0p__0c__void__0p__0r__0c__void__0p__0c__int__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_struct__List__0p__0l__struct__List__0p__0c__struct__Node__0p__0l__0p__0r__0l__struct__Node__0p__0c__void__0p__0r__0c__void__0p__0c__int__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::List, ::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::Node, *mut ::core::ffi::c_void) -> *mut crate::__pgrx_c_bindings::Node>, *mut ::core::ffi::c_void, i32) -> *mut crate::__pgrx_c_bindings::List>>;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::Node,
                *mut ::core::ffi::c_void,
            ) -> *mut crate::__pgrx_c_bindings::Node,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::Node,
                *mut ::core::ffi::c_void,
            ) -> *mut crate::__pgrx_c_bindings::Node,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__Node__0p__0l__struct__Node__0p__0c__void__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_struct__Node__0p__0l__struct__Node__0p__0c__void__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_struct__Node__0p__0l__struct__Node__0p__0c__void__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::Node,
                *mut ::core::ffi::c_void,
            ) -> *mut crate::__pgrx_c_bindings::Node,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::Node,
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::Node,
                        *mut ::core::ffi::c_void,
                    )
                        -> *mut crate::__pgrx_c_bindings::Node,
                >,
                *mut ::core::ffi::c_void,
            ) -> *mut crate::__pgrx_c_bindings::Node,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::Node,
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::Node,
                        *mut ::core::ffi::c_void,
                    )
                        -> *mut crate::__pgrx_c_bindings::Node,
                >,
                *mut ::core::ffi::c_void,
            ) -> *mut crate::__pgrx_c_bindings::Node,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__Node__0p__0l__struct__Node__0p__0c__struct__Node__0p__0l__0p__0r__0l__struct__Node__0p__0c__void__0p__0r__0c__void__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_struct__Node__0p__0l__struct__Node__0p__0c__struct__Node__0p__0l__0p__0r__0l__struct__Node__0p__0c__void__0p__0r__0c__void__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_struct__Node__0p__0l__struct__Node__0p__0c__struct__Node__0p__0l__0p__0r__0l__struct__Node__0p__0c__void__0p__0r__0c__void__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::Node, ::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::Node, *mut ::core::ffi::c_void) -> *mut crate::__pgrx_c_bindings::Node>, *mut ::core::ffi::c_void) -> *mut crate::__pgrx_c_bindings::Node>>;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::Node,
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::Node,
                        *mut ::core::ffi::c_void,
                    )
                        -> *mut crate::__pgrx_c_bindings::Node,
                >,
                *mut ::core::ffi::c_void,
                i32,
            ) -> *mut crate::__pgrx_c_bindings::Node,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::Node,
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::Node,
                        *mut ::core::ffi::c_void,
                    )
                        -> *mut crate::__pgrx_c_bindings::Node,
                >,
                *mut ::core::ffi::c_void,
                i32,
            ) -> *mut crate::__pgrx_c_bindings::Node,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__Node__0p__0l__struct__Node__0p__0c__struct__Node__0p__0l__0p__0r__0l__struct__Node__0p__0c__void__0p__0r__0c__void__0p__0c__int__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_struct__Node__0p__0l__struct__Node__0p__0c__struct__Node__0p__0l__0p__0r__0l__struct__Node__0p__0c__void__0p__0r__0c__void__0p__0c__int__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_struct__Node__0p__0l__struct__Node__0p__0c__struct__Node__0p__0l__0p__0r__0l__struct__Node__0p__0c__void__0p__0r__0c__void__0p__0c__int__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::Node, ::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::Node, *mut ::core::ffi::c_void) -> *mut crate::__pgrx_c_bindings::Node>, *mut ::core::ffi::c_void, i32) -> *mut crate::__pgrx_c_bindings::Node>>;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::NumericData,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::NumericData,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__NumericData__0p__0l__unsigned__long__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_struct__NumericData__0p__0l__unsigned__long__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_struct__NumericData__0p__0l__unsigned__long__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::Datum,
            ) -> *mut crate::__pgrx_c_bindings::NumericData,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::Query,
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::Node,
                        *mut ::core::ffi::c_void,
                    )
                        -> *mut crate::__pgrx_c_bindings::Node,
                >,
                *mut ::core::ffi::c_void,
                i32,
            ) -> *mut crate::__pgrx_c_bindings::Query,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::Query,
                ::core::option::Option<
                    unsafe extern "C-unwind" fn(
                        *mut crate::__pgrx_c_bindings::Node,
                        *mut ::core::ffi::c_void,
                    )
                        -> *mut crate::__pgrx_c_bindings::Node,
                >,
                *mut ::core::ffi::c_void,
                i32,
            ) -> *mut crate::__pgrx_c_bindings::Query,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__Query__0p__0l__struct__Query__0p__0c__struct__Node__0p__0l__0p__0r__0l__struct__Node__0p__0c__void__0p__0r__0c__void__0p__0c__int__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_struct__Query__0p__0l__struct__Query__0p__0c__struct__Node__0p__0l__0p__0r__0l__struct__Node__0p__0c__void__0p__0r__0c__void__0p__0c__int__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_struct__Query__0p__0l__struct__Query__0p__0c__struct__Node__0p__0l__0p__0r__0l__struct__Node__0p__0c__void__0p__0r__0c__void__0p__0c__int__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::Query, ::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::Node, *mut ::core::ffi::c_void) -> *mut crate::__pgrx_c_bindings::Node>, *mut ::core::ffi::c_void, i32) -> *mut crate::__pgrx_c_bindings::Query>>;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::buftag,
            ) -> ::core::mem::MaybeUninit<
                crate::__pgrx_c_bindings::RelFileLocator,
            >,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::buftag,
            ) -> ::core::mem::MaybeUninit<
                crate::__pgrx_c_bindings::RelFileLocator,
            >,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__RelFileLocator__0l__const__struct__buftag__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_struct__RelFileLocator__0l__const__struct__buftag__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_struct__RelFileLocator__0l__const__struct__buftag__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::buftag,
            ) -> ::core::mem::MaybeUninit<
                crate::__pgrx_c_bindings::RelFileLocator,
            >,
        >,
    >;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerInfo,
                *mut crate::__pgrx_c_bindings::Expr,
                bool,
                bool,
                bool,
                bool,
                u32,
                *mut crate::__pgrx_c_bindings::Bitmapset,
                *mut crate::__pgrx_c_bindings::Bitmapset,
                *mut crate::__pgrx_c_bindings::Bitmapset,
            ) -> *mut crate::__pgrx_c_bindings::RestrictInfo,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerInfo,
                *mut crate::__pgrx_c_bindings::Expr,
                bool,
                bool,
                bool,
                bool,
                u32,
                *mut crate::__pgrx_c_bindings::Bitmapset,
                *mut crate::__pgrx_c_bindings::Bitmapset,
                *mut crate::__pgrx_c_bindings::Bitmapset,
            ) -> *mut crate::__pgrx_c_bindings::RestrictInfo,
        >,
    >() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__RestrictInfo__0p__0l__struct__PlannerInfo__0p__0c__struct__Expr__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__unsigned__int__0c__struct__Bitmapset__0p__0c__struct__Bitmapset__0p__0c__struct__Bitmapset__0p__0r;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_struct__RestrictInfo__0p__0l__struct__PlannerInfo__0p__0c__struct__Expr__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__unsigned__int__0c__struct__Bitmapset__0p__0c__struct__Bitmapset__0p__0c__struct__Bitmapset__0p__0r {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_struct__RestrictInfo__0p__0l__struct__PlannerInfo__0p__0c__struct__Expr__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__unsigned__int__0c__struct__Bitmapset__0p__0c__struct__Bitmapset__0p__0c__struct__Bitmapset__0p__0r {
 type Physical = PhysicalFunction_C_unwind_10<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PlannerInfo, *mut crate::__pgrx_c_bindings::Expr, bool, bool, bool, bool, u32, *mut crate::__pgrx_c_bindings::Bitmapset, *mut crate::__pgrx_c_bindings::Bitmapset, *mut crate::__pgrx_c_bindings::Bitmapset) -> *mut crate::__pgrx_c_bindings::RestrictInfo>>;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<A0: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>, c::expression::ReadWrite>>, A1: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData>, c::expression::ReadWrite>>, A2: c::expression::ImplicitTo<c::CInt>, A3: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::ScanKeyData>, c::expression::ReadWrite>>, A4: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelTableScanDescData>, c::expression::ReadWrite>>, A5: c::expression::ImplicitTo<c::CUnsignedInt>> c::expression::Call<(A0,A1,A2,A3,A4,A5,)> for Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0c__struct__SnapshotData__0p__0c__int__0c__struct__ScanKeyData__0p__0c__struct__ParallelTableScanDescData__0p__0c__unsigned__int__0r {
 type Output = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TableScanDescData>, c::expression::ReadWrite> as c::expression::CType>::Value;
 unsafe fn call(pointer: Self::Pointer, args: (A0,A1,A2,A3,A4,A5,)) -> Self::Output {
let function = pointer.expect("C indirect call requires a non-null function pointer");
const { assert!(!::core::mem::needs_drop::<A0>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::RelationData>());
let native0 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>, c::expression::ReadWrite>, _>(args.0));
const { assert!(!::core::mem::needs_drop::<A1>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::SnapshotData>());
let native1 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData>, c::expression::ReadWrite>, _>(args.1));
const { assert!(!::core::mem::needs_drop::<A2>()); }
const _: () = assert!(!::core::mem::needs_drop::<i32>());
let native2 = <c::CInt as c::expression::CType>::into_storage(c::expression::implicit::<c::CInt, _>(args.2));
const { assert!(!::core::mem::needs_drop::<A3>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::ScanKeyData>());
let native3 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::ScanKeyData>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::ScanKeyData>, c::expression::ReadWrite>, _>(args.3));
const { assert!(!::core::mem::needs_drop::<A4>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::ParallelTableScanDescData>());
let native4 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelTableScanDescData>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelTableScanDescData>, c::expression::ReadWrite>, _>(args.4));
const { assert!(!::core::mem::needs_drop::<A5>()); }
const _: () = assert!(!::core::mem::needs_drop::<u32>());
let native5 = <c::CUnsignedInt as c::expression::CType>::into_storage(c::expression::implicit::<c::CUnsignedInt, _>(args.5));
// SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
let result = unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0, native1, native2, native3, native4, native5)) };
<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TableScanDescData>, c::expression::ReadWrite> as c::expression::CType>::from_storage(result)
}
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::PlanState>() == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::PlanState>() == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<
    A0: c::expression::ImplicitTo<
            c::expression::CPointer<
                c::expression::CRecord<crate::__pgrx_c_bindings::PlanState>,
                c::expression::ReadWrite,
            >,
        >,
> c::expression::Call<(A0,)>
    for Signature_struct__TupleTableSlot__0p__0l__struct__PlanState__0p__0r
{
    type Output = <c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot>,
        c::expression::ReadWrite,
    > as c::expression::CType>::Value;
    unsafe fn call(pointer: Self::Pointer, args: (A0,)) -> Self::Output {
        let function = pointer.expect("C indirect call requires a non-null function pointer");
        const {
            assert!(!::core::mem::needs_drop::<A0>());
        }
        const _: () =
            assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::PlanState>());
        let native0 = <c::expression::CPointer<
            c::expression::CRecord<crate::__pgrx_c_bindings::PlanState>,
            c::expression::ReadWrite,
        > as c::expression::CType>::into_storage(c::expression::implicit::<
            c::expression::CPointer<
                c::expression::CRecord<crate::__pgrx_c_bindings::PlanState>,
                c::expression::ReadWrite,
            >,
            _,
        >(args.0));
        // SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
        let result = unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0)) };
        <c::expression::CPointer<
            c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot>,
            c::expression::ReadWrite,
        > as c::expression::CType>::from_storage(result)
    }
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_char,
                bool,
                bool,
                *const ::core::ffi::c_char,
                *mut *mut ::core::ffi::c_char,
            )
                -> *mut crate::__pgrx_c_bindings::WalReceiverConn,
        >,
    >() == 8
);
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_char,
                bool,
                bool,
                *const ::core::ffi::c_char,
                *mut *mut ::core::ffi::c_char,
            )
                -> *mut crate::__pgrx_c_bindings::WalReceiverConn,
        >,
    >() == 8
);
#[cfg(feature = "pg16")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__WalReceiverConn__0p__0l__const__char__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__const__char__0p__0c__char__0p__0p__0r;
#[cfg(feature = "pg16")]
impl c::sealed::Sealed for Signature_struct__WalReceiverConn__0p__0l__const__char__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__const__char__0p__0c__char__0p__0p__0r {}
#[cfg(feature = "pg16")]
impl c::expression::NativeFunctionSignature for Signature_struct__WalReceiverConn__0p__0l__const__char__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__const__char__0p__0c__char__0p__0p__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_char, bool, bool, *const ::core::ffi::c_char, *mut *mut ::core::ffi::c_char) -> *mut crate::__pgrx_c_bindings::WalReceiverConn>>;
}
