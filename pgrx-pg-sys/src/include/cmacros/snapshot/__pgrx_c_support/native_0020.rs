// Shared generated C macro support.

#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SlruSharedData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SlruSharedData> as c::expression::OrdinaryField<Field_latest_page_number>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).latest_page_number) };
#[doc(hidden)]
pub struct Field_slru_stats_idx;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SlruSharedData, slru_stats_idx) == 84
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_slru_stats_idx>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SlruSharedData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 84;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SlruSharedData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SlruSharedData> as c::expression::OrdinaryField<Field_slru_stats_idx>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).slru_stats_idx) };
#[doc(hidden)]
pub struct Field_snapshot_type;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SnapshotData>() == 120);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SnapshotData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SnapshotData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapshotData, snapshot_type) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_snapshot_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__SnapshotType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SnapshotData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData> as c::expression::OrdinaryField<Field_snapshot_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).snapshot_type) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapshotData, xmin) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xmin>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SnapshotData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData> as c::expression::OrdinaryField<Field_xmin>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xmin) };
#[doc(hidden)]
pub struct Field_xmax;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapshotData, xmax) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xmax>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SnapshotData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData> as c::expression::OrdinaryField<Field_xmax>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xmax) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapshotData, xcnt) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xcnt>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SnapshotData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData> as c::expression::OrdinaryField<Field_xcnt>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xcnt) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapshotData, subxcnt) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_subxcnt>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SnapshotData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData> as c::expression::OrdinaryField<Field_subxcnt>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).subxcnt) };
#[doc(hidden)]
pub struct Field_suboverflowed;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapshotData, suboverflowed) == 44);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_suboverflowed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SnapshotData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData> as c::expression::OrdinaryField<Field_suboverflowed>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).suboverflowed) };
#[doc(hidden)]
pub struct Field_takenDuringRecovery;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapshotData, takenDuringRecovery) == 45
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_takenDuringRecovery>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 45;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SnapshotData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData> as c::expression::OrdinaryField<Field_takenDuringRecovery>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).takenDuringRecovery) };
#[doc(hidden)]
pub struct Field_copied;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapshotData, copied) == 46);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_copied>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 46;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SnapshotData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData> as c::expression::OrdinaryField<Field_copied>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).copied) };
#[doc(hidden)]
pub struct Field_curcid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapshotData, curcid) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_curcid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SnapshotData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData> as c::expression::OrdinaryField<Field_curcid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).curcid) };
#[doc(hidden)]
pub struct Field_speculativeToken;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapshotData, speculativeToken) == 52
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_speculativeToken>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SnapshotData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData> as c::expression::OrdinaryField<Field_speculativeToken>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).speculativeToken) };
#[doc(hidden)]
pub struct Field_vistest;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapshotData, vistest) == 56);
const _: () = {
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::GlobalVisState>() == 8);
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::GlobalVisState>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vistest>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData>
{
    type Member = c::expression::CPointer<
        c::expression::COpaque<crate::__pgrx_c_bindings::GlobalVisState>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SnapshotData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData> as c::expression::OrdinaryField<Field_vistest>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).vistest) };
#[doc(hidden)]
pub struct Field_active_count;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapshotData, active_count) == 64);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_active_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SnapshotData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData> as c::expression::OrdinaryField<Field_active_count>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).active_count) };
#[doc(hidden)]
pub struct Field_regd_count;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapshotData, regd_count) == 68);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_regd_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 68;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SnapshotData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData> as c::expression::OrdinaryField<Field_regd_count>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).regd_count) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
#[doc(hidden)]
pub struct Field_whenTaken;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapshotData, whenTaken) == 96);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_whenTaken>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SnapshotData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData> as c::expression::OrdinaryField<Field_whenTaken>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).whenTaken) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapshotData, lsn) == 104);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SnapshotData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData> as c::expression::OrdinaryField<Field_lsn>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lsn) };
#[doc(hidden)]
pub struct Field_snapXactCompletionCount;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapshotData, snapXactCompletionCount) == 112
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_snapXactCompletionCount>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SnapshotData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData> as c::expression::OrdinaryField<Field_snapXactCompletionCount>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).snapXactCompletionCount) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Sort>() == 144);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Sort>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::Sort {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Sort, numCols) == 104);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numCols>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Sort>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Sort,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Sort> as c::expression::OrdinaryField<
        Field_numCols,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numCols) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SortBy>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SortBy>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SortBy {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SortBy, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SortBy>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SortBy,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SortBy> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_sortby_dir;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SortBy, sortby_dir) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sortby_dir>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SortBy>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__SortByDir,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SortBy,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SortBy> as c::expression::OrdinaryField<
        Field_sortby_dir,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sortby_dir) };
#[doc(hidden)]
pub struct Field_sortby_nulls;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SortBy, sortby_nulls) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sortby_nulls>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SortBy>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__SortByNulls,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SortBy,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SortBy> as c::expression::OrdinaryField<
        Field_sortby_nulls,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sortby_nulls) };
#[doc(hidden)]
pub struct Field_useOp;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SortBy, useOp) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_useOp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SortBy>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SortBy,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SortBy> as c::expression::OrdinaryField<
        Field_useOp,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).useOp) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SortBy, location) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SortBy>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SortBy,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SortBy> as c::expression::OrdinaryField<
        Field_location,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[doc(hidden)]
pub struct Field_isWorker;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SortCoordinateData>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SortCoordinateData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SortCoordinateData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SortCoordinateData, isWorker) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isWorker>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SortCoordinateData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SortCoordinateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SortCoordinateData> as c::expression::OrdinaryField<Field_isWorker>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isWorker) };
#[doc(hidden)]
pub struct Field_nParticipants;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SortCoordinateData, nParticipants) == 4
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nParticipants>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SortCoordinateData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SortCoordinateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SortCoordinateData> as c::expression::OrdinaryField<Field_nParticipants>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nParticipants) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SortGroupClause>() == 20);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SortGroupClause>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SortGroupClause {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SortGroupClause, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SortGroupClause>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SortGroupClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::SortGroupClause> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_tleSortGroupRef;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SortGroupClause, tleSortGroupRef) == 4
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tleSortGroupRef>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SortGroupClause>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SortGroupClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::SortGroupClause> as c::expression::OrdinaryField<Field_tleSortGroupRef>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tleSortGroupRef) };
#[doc(hidden)]
pub struct Field_eqop;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SortGroupClause, eqop) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_eqop>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SortGroupClause>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SortGroupClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::SortGroupClause> as c::expression::OrdinaryField<Field_eqop>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).eqop) };
#[doc(hidden)]
pub struct Field_sortop;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SortGroupClause, sortop) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sortop>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SortGroupClause>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SortGroupClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::SortGroupClause> as c::expression::OrdinaryField<Field_sortop>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sortop) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SortGroupClause, nulls_first) == 16);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nulls_first>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SortGroupClause>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SortGroupClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::SortGroupClause> as c::expression::OrdinaryField<Field_nulls_first>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nulls_first) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SortGroupClause, hashable) == 17);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hashable>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SortGroupClause>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 17;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SortGroupClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::SortGroupClause> as c::expression::OrdinaryField<Field_hashable>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hashable) };
#[doc(hidden)]
pub struct Field_randomAccess;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SortState>() == 280);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SortState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SortState {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SortState, randomAccess) == 224);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_randomAccess>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SortState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 224;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SortState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SortState> as c::expression::OrdinaryField<
        Field_randomAccess,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).randomAccess) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SortState, bounded) == 225);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bounded>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SortState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 225;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SortState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SortState> as c::expression::OrdinaryField<
        Field_bounded,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).bounded) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SortState, bound) == 232);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bound>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SortState>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 232;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SortState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SortState> as c::expression::OrdinaryField<
        Field_bound,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).bound) };
#[doc(hidden)]
pub struct Field_sort_Done;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SortState, sort_Done) == 240);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sort_Done>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SortState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 240;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SortState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SortState> as c::expression::OrdinaryField<
        Field_sort_Done,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sort_Done) };
#[doc(hidden)]
pub struct Field_bounded_Done;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SortState, bounded_Done) == 241);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bounded_Done>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SortState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 241;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SortState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SortState> as c::expression::OrdinaryField<
        Field_bounded_Done,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).bounded_Done) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SortState, bound_Done) == 248);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bound_Done>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SortState>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 248;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SortState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SortState> as c::expression::OrdinaryField<
        Field_bound_Done,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).bound_Done) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SortState, am_worker) == 264);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_am_worker>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SortState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 264;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SortState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SortState> as c::expression::OrdinaryField<
        Field_am_worker,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).am_worker) };
#[doc(hidden)]
pub struct Field_datumSort;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SortState, datumSort) == 265);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_datumSort>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SortState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 265;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SortState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SortState> as c::expression::OrdinaryField<
        Field_datumSort,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).datumSort) };
#[doc(hidden)]
pub struct Field_ssup_collation;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SortSupportData>() == 64);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SortSupportData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SortSupportData {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SortSupportData, ssup_collation) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ssup_collation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SortSupportData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SortSupportData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SortSupportData> as c::expression::OrdinaryField<Field_ssup_collation>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ssup_collation) };
#[doc(hidden)]
pub struct Field_ssup_reverse;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SortSupportData, ssup_reverse) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ssup_reverse>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SortSupportData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SortSupportData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SortSupportData> as c::expression::OrdinaryField<Field_ssup_reverse>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ssup_reverse) };
#[doc(hidden)]
pub struct Field_ssup_nulls_first;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SortSupportData, ssup_nulls_first) == 13
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ssup_nulls_first>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SortSupportData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 13;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SortSupportData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SortSupportData> as c::expression::OrdinaryField<Field_ssup_nulls_first>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ssup_nulls_first) };
#[doc(hidden)]
pub struct Field_ssup_attno;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SortSupportData, ssup_attno) == 14);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ssup_attno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SortSupportData>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 14;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SortSupportData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SortSupportData> as c::expression::OrdinaryField<Field_ssup_attno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ssup_attno) };
#[doc(hidden)]
pub struct Field_abbreviate;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SortSupportData, abbreviate) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_abbreviate>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SortSupportData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SortSupportData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SortSupportData> as c::expression::OrdinaryField<Field_abbreviate>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).abbreviate) };
#[doc(hidden)]
pub struct Field_attPrefixType;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SpGistCache>() == 128);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SpGistCache>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SpGistCache {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistCache, attPrefixType) == 40);
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SpGistTypeDesc>() == 12);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SpGistTypeDesc>() == 4);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attPrefixType>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistCache>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::SpGistTypeDesc>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpGistCache, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpGistCache> as c::expression::OrdinaryField<Field_attPrefixType>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attPrefixType) };
#[doc(hidden)]
pub struct Field_attLabelType;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistCache, attLabelType) == 52);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attLabelType>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistCache>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::SpGistTypeDesc>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpGistCache, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpGistCache> as c::expression::OrdinaryField<Field_attLabelType>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attLabelType) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SpGistDeadTupleData>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SpGistDeadTupleData>() == 4);
};
#[doc(hidden)]
pub struct Field_tupstate;
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SpGistDeadTupleData {}
const _: () = {
    assert!(
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistDeadTupleData, _bitfield_1) * 8 + 0
            == 0
    );
};
unsafe extern "C" {
    fn __pgrx_bitfield__get__SpGistDeadTupleData__tupstate(
        p: *const crate::__pgrx_c_bindings::SpGistDeadTupleData,
    ) -> u32;
    fn __pgrx_bitfield__set__SpGistDeadTupleData__tupstate(
        p: *mut crate::__pgrx_c_bindings::SpGistDeadTupleData,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_volatile__SpGistDeadTupleData__tupstate(
        p: *const crate::__pgrx_c_bindings::SpGistDeadTupleData,
    ) -> u32;
    fn __pgrx_bitfield__set_volatile__SpGistDeadTupleData__tupstate(
        p: *mut crate::__pgrx_c_bindings::SpGistDeadTupleData,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned__SpGistDeadTupleData__tupstate(
        p: *const crate::__pgrx_c_bindings::SpGistDeadTupleData,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned__SpGistDeadTupleData__tupstate(
        p: *mut crate::__pgrx_c_bindings::SpGistDeadTupleData,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned_volatile__SpGistDeadTupleData__tupstate(
        p: *const crate::__pgrx_c_bindings::SpGistDeadTupleData,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned_volatile__SpGistDeadTupleData__tupstate(
        p: *mut crate::__pgrx_c_bindings::SpGistDeadTupleData,
        value: u32,
    ) -> u32;
}
#[doc(hidden)]
pub struct Bitfield_SpGistDeadTupleData__tupstate<Q: c::expression::Qualifier> {
    address: Q::Raw<crate::__pgrx_c_bindings::SpGistDeadTupleData>,
    access: c::expression::Access,
}
impl<Q: c::expression::Qualifier> Copy for Bitfield_SpGistDeadTupleData__tupstate<Q> {}
impl<Q: c::expression::Qualifier> Clone for Bitfield_SpGistDeadTupleData__tupstate<Q> {
    fn clone(&self) -> Self {
        *self
    }
}
// SAFETY: Clang and bindgen layout witnesses agree. This projects the record address without a reference or record read.
unsafe impl<Q: c::expression::Qualifier> c::expression::Field<Field_tupstate, Q>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistDeadTupleData>
{
    type Output = Bitfield_SpGistDeadTupleData__tupstate<Q>;
    unsafe fn project(base: c::expression::Place<Self, Q>) -> Self::Output {
        Bitfield_SpGistDeadTupleData__tupstate::<Q> {
            address: <Q as c::expression::Qualifier>::from_mut(base.pointer().as_mut_address()),
            access: c::expression::Access {
                volatile: base.access().volatile || false,
                unaligned: base.access().unaligned,
            },
        }
    }
}
impl<Q: c::expression::Qualifier> c::expression::VolatilePlace
    for Bitfield_SpGistDeadTupleData__tupstate<Q>
{
    type Qualified = Self;
    fn qualify_volatile(mut self) -> Self::Qualified {
        self.access.volatile = true;
        self
    }
}
// SAFETY: The same-profile C compiler owns bitfield storage reads, including partially initialized neighbor/padding bits. The marker retains declared size and compiler-proved arithmetic promotion.
unsafe impl<Q: c::expression::Qualifier> c::expression::ReadPlace
    for Bitfield_SpGistDeadTupleData__tupstate<Q>
{
    type Object = c::CUnsignedInt;
    type Type = c::expression::CBitfield<c::CUnsignedInt, c::CInt>;
    unsafe fn load(self) -> <Self::Type as c::expression::CType>::Value {
        let pointer = Q::into_mut(self.address).cast_const(); // SAFETY: The caller establishes the original C field load contract; helpers perform only that field access.
        let storage = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__get_unaligned_volatile__SpGistDeadTupleData__tupstate(pointer)
                } else {
                    __pgrx_bitfield__get_unaligned__SpGistDeadTupleData__tupstate(pointer)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__get_volatile__SpGistDeadTupleData__tupstate(pointer)
            } else {
                __pgrx_bitfield__get__SpGistDeadTupleData__tupstate(pointer)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(storage)
    }
}
// SAFETY: Write capability retains ReadWrite qualification. Original C performs declared-base conversion and width narrowing without Rust byte reads.
unsafe impl c::expression::WritePlace
    for Bitfield_SpGistDeadTupleData__tupstate<c::expression::ReadWrite>
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
                    __pgrx_bitfield__set_unaligned_volatile__SpGistDeadTupleData__tupstate(
                        self.address,
                        value,
                    )
                } else {
                    __pgrx_bitfield__set_unaligned__SpGistDeadTupleData__tupstate(
                        self.address,
                        value,
                    )
                }
            } else if self.access.volatile {
                __pgrx_bitfield__set_volatile__SpGistDeadTupleData__tupstate(self.address, value)
            } else {
                __pgrx_bitfield__set__SpGistDeadTupleData__tupstate(self.address, value)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(stored)
    }
}
const _: () = {
    assert!(
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistDeadTupleData, _bitfield_1) * 8 + 2
            == 2
    );
};
unsafe extern "C" {
    fn __pgrx_bitfield__get__SpGistDeadTupleData__size(
        p: *const crate::__pgrx_c_bindings::SpGistDeadTupleData,
    ) -> u32;
    fn __pgrx_bitfield__set__SpGistDeadTupleData__size(
        p: *mut crate::__pgrx_c_bindings::SpGistDeadTupleData,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_volatile__SpGistDeadTupleData__size(
        p: *const crate::__pgrx_c_bindings::SpGistDeadTupleData,
    ) -> u32;
    fn __pgrx_bitfield__set_volatile__SpGistDeadTupleData__size(
        p: *mut crate::__pgrx_c_bindings::SpGistDeadTupleData,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned__SpGistDeadTupleData__size(
        p: *const crate::__pgrx_c_bindings::SpGistDeadTupleData,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned__SpGistDeadTupleData__size(
        p: *mut crate::__pgrx_c_bindings::SpGistDeadTupleData,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned_volatile__SpGistDeadTupleData__size(
        p: *const crate::__pgrx_c_bindings::SpGistDeadTupleData,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned_volatile__SpGistDeadTupleData__size(
        p: *mut crate::__pgrx_c_bindings::SpGistDeadTupleData,
        value: u32,
    ) -> u32;
}
#[doc(hidden)]
pub struct Bitfield_SpGistDeadTupleData__size<Q: c::expression::Qualifier> {
    address: Q::Raw<crate::__pgrx_c_bindings::SpGistDeadTupleData>,
    access: c::expression::Access,
}
impl<Q: c::expression::Qualifier> Copy for Bitfield_SpGistDeadTupleData__size<Q> {}
impl<Q: c::expression::Qualifier> Clone for Bitfield_SpGistDeadTupleData__size<Q> {
    fn clone(&self) -> Self {
        *self
    }
}
// SAFETY: Clang and bindgen layout witnesses agree. This projects the record address without a reference or record read.
unsafe impl<Q: c::expression::Qualifier> c::expression::Field<Field_size, Q>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistDeadTupleData>
{
    type Output = Bitfield_SpGistDeadTupleData__size<Q>;
    unsafe fn project(base: c::expression::Place<Self, Q>) -> Self::Output {
        Bitfield_SpGistDeadTupleData__size::<Q> {
            address: <Q as c::expression::Qualifier>::from_mut(base.pointer().as_mut_address()),
            access: c::expression::Access {
                volatile: base.access().volatile || false,
                unaligned: base.access().unaligned,
            },
        }
    }
}
impl<Q: c::expression::Qualifier> c::expression::VolatilePlace
    for Bitfield_SpGistDeadTupleData__size<Q>
{
    type Qualified = Self;
    fn qualify_volatile(mut self) -> Self::Qualified {
        self.access.volatile = true;
        self
    }
}
// SAFETY: The same-profile C compiler owns bitfield storage reads, including partially initialized neighbor/padding bits. The marker retains declared size and compiler-proved arithmetic promotion.
unsafe impl<Q: c::expression::Qualifier> c::expression::ReadPlace
    for Bitfield_SpGistDeadTupleData__size<Q>
{
    type Object = c::CUnsignedInt;
    type Type = c::expression::CBitfield<c::CUnsignedInt, c::CInt>;
    unsafe fn load(self) -> <Self::Type as c::expression::CType>::Value {
        let pointer = Q::into_mut(self.address).cast_const(); // SAFETY: The caller establishes the original C field load contract; helpers perform only that field access.
        let storage = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__get_unaligned_volatile__SpGistDeadTupleData__size(pointer)
                } else {
                    __pgrx_bitfield__get_unaligned__SpGistDeadTupleData__size(pointer)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__get_volatile__SpGistDeadTupleData__size(pointer)
            } else {
                __pgrx_bitfield__get__SpGistDeadTupleData__size(pointer)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(storage)
    }
}
// SAFETY: Write capability retains ReadWrite qualification. Original C performs declared-base conversion and width narrowing without Rust byte reads.
unsafe impl c::expression::WritePlace
    for Bitfield_SpGistDeadTupleData__size<c::expression::ReadWrite>
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
                    __pgrx_bitfield__set_unaligned_volatile__SpGistDeadTupleData__size(
                        self.address,
                        value,
                    )
                } else {
                    __pgrx_bitfield__set_unaligned__SpGistDeadTupleData__size(self.address, value)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__set_volatile__SpGistDeadTupleData__size(self.address, value)
            } else {
                __pgrx_bitfield__set__SpGistDeadTupleData__size(self.address, value)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(stored)
    }
}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistDeadTupleData, t_info) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_t_info>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistDeadTupleData>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpGistDeadTupleData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpGistDeadTupleData> as c::expression::OrdinaryField<Field_t_info>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).t_info) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistDeadTupleData, xid) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistDeadTupleData>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpGistDeadTupleData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpGistDeadTupleData> as c::expression::OrdinaryField<Field_xid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xid) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SpGistInnerTupleData>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SpGistInnerTupleData>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SpGistInnerTupleData {}
const _: () = {
    assert!(
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistInnerTupleData, _bitfield_1) * 8
            + 0
            == 0
    );
};
unsafe extern "C" {
    fn __pgrx_bitfield__get__SpGistInnerTupleData__tupstate(
        p: *const crate::__pgrx_c_bindings::SpGistInnerTupleData,
    ) -> u32;
    fn __pgrx_bitfield__set__SpGistInnerTupleData__tupstate(
        p: *mut crate::__pgrx_c_bindings::SpGistInnerTupleData,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_volatile__SpGistInnerTupleData__tupstate(
        p: *const crate::__pgrx_c_bindings::SpGistInnerTupleData,
    ) -> u32;
    fn __pgrx_bitfield__set_volatile__SpGistInnerTupleData__tupstate(
        p: *mut crate::__pgrx_c_bindings::SpGistInnerTupleData,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned__SpGistInnerTupleData__tupstate(
        p: *const crate::__pgrx_c_bindings::SpGistInnerTupleData,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned__SpGistInnerTupleData__tupstate(
        p: *mut crate::__pgrx_c_bindings::SpGistInnerTupleData,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned_volatile__SpGistInnerTupleData__tupstate(
        p: *const crate::__pgrx_c_bindings::SpGistInnerTupleData,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned_volatile__SpGistInnerTupleData__tupstate(
        p: *mut crate::__pgrx_c_bindings::SpGistInnerTupleData,
        value: u32,
    ) -> u32;
}
#[doc(hidden)]
pub struct Bitfield_SpGistInnerTupleData__tupstate<Q: c::expression::Qualifier> {
    address: Q::Raw<crate::__pgrx_c_bindings::SpGistInnerTupleData>,
    access: c::expression::Access,
}
impl<Q: c::expression::Qualifier> Copy for Bitfield_SpGistInnerTupleData__tupstate<Q> {}
impl<Q: c::expression::Qualifier> Clone for Bitfield_SpGistInnerTupleData__tupstate<Q> {
    fn clone(&self) -> Self {
        *self
    }
}
// SAFETY: Clang and bindgen layout witnesses agree. This projects the record address without a reference or record read.
unsafe impl<Q: c::expression::Qualifier> c::expression::Field<Field_tupstate, Q>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistInnerTupleData>
{
    type Output = Bitfield_SpGistInnerTupleData__tupstate<Q>;
    unsafe fn project(base: c::expression::Place<Self, Q>) -> Self::Output {
        Bitfield_SpGistInnerTupleData__tupstate::<Q> {
            address: <Q as c::expression::Qualifier>::from_mut(base.pointer().as_mut_address()),
            access: c::expression::Access {
                volatile: base.access().volatile || false,
                unaligned: base.access().unaligned,
            },
        }
    }
}
impl<Q: c::expression::Qualifier> c::expression::VolatilePlace
    for Bitfield_SpGistInnerTupleData__tupstate<Q>
{
    type Qualified = Self;
    fn qualify_volatile(mut self) -> Self::Qualified {
        self.access.volatile = true;
        self
    }
}
// SAFETY: The same-profile C compiler owns bitfield storage reads, including partially initialized neighbor/padding bits. The marker retains declared size and compiler-proved arithmetic promotion.
unsafe impl<Q: c::expression::Qualifier> c::expression::ReadPlace
    for Bitfield_SpGistInnerTupleData__tupstate<Q>
{
    type Object = c::CUnsignedInt;
    type Type = c::expression::CBitfield<c::CUnsignedInt, c::CInt>;
    unsafe fn load(self) -> <Self::Type as c::expression::CType>::Value {
        let pointer = Q::into_mut(self.address).cast_const(); // SAFETY: The caller establishes the original C field load contract; helpers perform only that field access.
        let storage = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__get_unaligned_volatile__SpGistInnerTupleData__tupstate(pointer)
                } else {
                    __pgrx_bitfield__get_unaligned__SpGistInnerTupleData__tupstate(pointer)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__get_volatile__SpGistInnerTupleData__tupstate(pointer)
            } else {
                __pgrx_bitfield__get__SpGistInnerTupleData__tupstate(pointer)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(storage)
    }
}
// SAFETY: Write capability retains ReadWrite qualification. Original C performs declared-base conversion and width narrowing without Rust byte reads.
unsafe impl c::expression::WritePlace
    for Bitfield_SpGistInnerTupleData__tupstate<c::expression::ReadWrite>
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
                    __pgrx_bitfield__set_unaligned_volatile__SpGistInnerTupleData__tupstate(
                        self.address,
                        value,
                    )
                } else {
                    __pgrx_bitfield__set_unaligned__SpGistInnerTupleData__tupstate(
                        self.address,
                        value,
                    )
                }
            } else if self.access.volatile {
                __pgrx_bitfield__set_volatile__SpGistInnerTupleData__tupstate(self.address, value)
            } else {
                __pgrx_bitfield__set__SpGistInnerTupleData__tupstate(self.address, value)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(stored)
    }
}
#[doc(hidden)]
pub struct Field_allTheSame;
const _: () = {
    assert!(
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistInnerTupleData, _bitfield_1) * 8
            + 2
            == 2
    );
};
unsafe extern "C" {
    fn __pgrx_bitfield__get__SpGistInnerTupleData__allTheSame(
        p: *const crate::__pgrx_c_bindings::SpGistInnerTupleData,
    ) -> u32;
    fn __pgrx_bitfield__set__SpGistInnerTupleData__allTheSame(
        p: *mut crate::__pgrx_c_bindings::SpGistInnerTupleData,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_volatile__SpGistInnerTupleData__allTheSame(
        p: *const crate::__pgrx_c_bindings::SpGistInnerTupleData,
    ) -> u32;
    fn __pgrx_bitfield__set_volatile__SpGistInnerTupleData__allTheSame(
        p: *mut crate::__pgrx_c_bindings::SpGistInnerTupleData,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned__SpGistInnerTupleData__allTheSame(
        p: *const crate::__pgrx_c_bindings::SpGistInnerTupleData,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned__SpGistInnerTupleData__allTheSame(
        p: *mut crate::__pgrx_c_bindings::SpGistInnerTupleData,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned_volatile__SpGistInnerTupleData__allTheSame(
        p: *const crate::__pgrx_c_bindings::SpGistInnerTupleData,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned_volatile__SpGistInnerTupleData__allTheSame(
        p: *mut crate::__pgrx_c_bindings::SpGistInnerTupleData,
        value: u32,
    ) -> u32;
}
#[doc(hidden)]
pub struct Bitfield_SpGistInnerTupleData__allTheSame<Q: c::expression::Qualifier> {
    address: Q::Raw<crate::__pgrx_c_bindings::SpGistInnerTupleData>,
    access: c::expression::Access,
}
impl<Q: c::expression::Qualifier> Copy for Bitfield_SpGistInnerTupleData__allTheSame<Q> {}
impl<Q: c::expression::Qualifier> Clone for Bitfield_SpGistInnerTupleData__allTheSame<Q> {
    fn clone(&self) -> Self {
        *self
    }
}
// SAFETY: Clang and bindgen layout witnesses agree. This projects the record address without a reference or record read.
unsafe impl<Q: c::expression::Qualifier> c::expression::Field<Field_allTheSame, Q>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistInnerTupleData>
{
    type Output = Bitfield_SpGistInnerTupleData__allTheSame<Q>;
    unsafe fn project(base: c::expression::Place<Self, Q>) -> Self::Output {
        Bitfield_SpGistInnerTupleData__allTheSame::<Q> {
            address: <Q as c::expression::Qualifier>::from_mut(base.pointer().as_mut_address()),
            access: c::expression::Access {
                volatile: base.access().volatile || false,
                unaligned: base.access().unaligned,
            },
        }
    }
}
impl<Q: c::expression::Qualifier> c::expression::VolatilePlace
    for Bitfield_SpGistInnerTupleData__allTheSame<Q>
{
    type Qualified = Self;
    fn qualify_volatile(mut self) -> Self::Qualified {
        self.access.volatile = true;
        self
    }
}
// SAFETY: The same-profile C compiler owns bitfield storage reads, including partially initialized neighbor/padding bits. The marker retains declared size and compiler-proved arithmetic promotion.
unsafe impl<Q: c::expression::Qualifier> c::expression::ReadPlace
    for Bitfield_SpGistInnerTupleData__allTheSame<Q>
{
    type Object = c::CUnsignedInt;
    type Type = c::expression::CBitfield<c::CUnsignedInt, c::CInt>;
    unsafe fn load(self) -> <Self::Type as c::expression::CType>::Value {
        let pointer = Q::into_mut(self.address).cast_const(); // SAFETY: The caller establishes the original C field load contract; helpers perform only that field access.
        let storage = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__get_unaligned_volatile__SpGistInnerTupleData__allTheSame(
                        pointer,
                    )
                } else {
                    __pgrx_bitfield__get_unaligned__SpGistInnerTupleData__allTheSame(pointer)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__get_volatile__SpGistInnerTupleData__allTheSame(pointer)
            } else {
                __pgrx_bitfield__get__SpGistInnerTupleData__allTheSame(pointer)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(storage)
    }
}
// SAFETY: Write capability retains ReadWrite qualification. Original C performs declared-base conversion and width narrowing without Rust byte reads.
unsafe impl c::expression::WritePlace
    for Bitfield_SpGistInnerTupleData__allTheSame<c::expression::ReadWrite>
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
                    __pgrx_bitfield__set_unaligned_volatile__SpGistInnerTupleData__allTheSame(
                        self.address,
                        value,
                    )
                } else {
                    __pgrx_bitfield__set_unaligned__SpGistInnerTupleData__allTheSame(
                        self.address,
                        value,
                    )
                }
            } else if self.access.volatile {
                __pgrx_bitfield__set_volatile__SpGistInnerTupleData__allTheSame(self.address, value)
            } else {
                __pgrx_bitfield__set__SpGistInnerTupleData__allTheSame(self.address, value)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(stored)
    }
}
#[doc(hidden)]
pub struct Field_nNodes;
const _: () = {
    assert!(
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistInnerTupleData, _bitfield_1) * 8
            + 3
            == 3
    );
};
unsafe extern "C" {
    fn __pgrx_bitfield__get__SpGistInnerTupleData__nNodes(
        p: *const crate::__pgrx_c_bindings::SpGistInnerTupleData,
    ) -> u32;
    fn __pgrx_bitfield__set__SpGistInnerTupleData__nNodes(
        p: *mut crate::__pgrx_c_bindings::SpGistInnerTupleData,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_volatile__SpGistInnerTupleData__nNodes(
        p: *const crate::__pgrx_c_bindings::SpGistInnerTupleData,
    ) -> u32;
    fn __pgrx_bitfield__set_volatile__SpGistInnerTupleData__nNodes(
        p: *mut crate::__pgrx_c_bindings::SpGistInnerTupleData,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned__SpGistInnerTupleData__nNodes(
        p: *const crate::__pgrx_c_bindings::SpGistInnerTupleData,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned__SpGistInnerTupleData__nNodes(
        p: *mut crate::__pgrx_c_bindings::SpGistInnerTupleData,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned_volatile__SpGistInnerTupleData__nNodes(
        p: *const crate::__pgrx_c_bindings::SpGistInnerTupleData,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned_volatile__SpGistInnerTupleData__nNodes(
        p: *mut crate::__pgrx_c_bindings::SpGistInnerTupleData,
        value: u32,
    ) -> u32;
}
#[doc(hidden)]
pub struct Bitfield_SpGistInnerTupleData__nNodes<Q: c::expression::Qualifier> {
    address: Q::Raw<crate::__pgrx_c_bindings::SpGistInnerTupleData>,
    access: c::expression::Access,
}
impl<Q: c::expression::Qualifier> Copy for Bitfield_SpGistInnerTupleData__nNodes<Q> {}
impl<Q: c::expression::Qualifier> Clone for Bitfield_SpGistInnerTupleData__nNodes<Q> {
    fn clone(&self) -> Self {
        *self
    }
}
// SAFETY: Clang and bindgen layout witnesses agree. This projects the record address without a reference or record read.
unsafe impl<Q: c::expression::Qualifier> c::expression::Field<Field_nNodes, Q>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistInnerTupleData>
{
    type Output = Bitfield_SpGistInnerTupleData__nNodes<Q>;
    unsafe fn project(base: c::expression::Place<Self, Q>) -> Self::Output {
        Bitfield_SpGistInnerTupleData__nNodes::<Q> {
            address: <Q as c::expression::Qualifier>::from_mut(base.pointer().as_mut_address()),
            access: c::expression::Access {
                volatile: base.access().volatile || false,
                unaligned: base.access().unaligned,
            },
        }
    }
}
impl<Q: c::expression::Qualifier> c::expression::VolatilePlace
    for Bitfield_SpGistInnerTupleData__nNodes<Q>
{
    type Qualified = Self;
    fn qualify_volatile(mut self) -> Self::Qualified {
        self.access.volatile = true;
        self
    }
}
// SAFETY: The same-profile C compiler owns bitfield storage reads, including partially initialized neighbor/padding bits. The marker retains declared size and compiler-proved arithmetic promotion.
unsafe impl<Q: c::expression::Qualifier> c::expression::ReadPlace
    for Bitfield_SpGistInnerTupleData__nNodes<Q>
{
    type Object = c::CUnsignedInt;
    type Type = c::expression::CBitfield<c::CUnsignedInt, c::CInt>;
    unsafe fn load(self) -> <Self::Type as c::expression::CType>::Value {
        let pointer = Q::into_mut(self.address).cast_const(); // SAFETY: The caller establishes the original C field load contract; helpers perform only that field access.
        let storage = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__get_unaligned_volatile__SpGistInnerTupleData__nNodes(pointer)
                } else {
                    __pgrx_bitfield__get_unaligned__SpGistInnerTupleData__nNodes(pointer)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__get_volatile__SpGistInnerTupleData__nNodes(pointer)
            } else {
                __pgrx_bitfield__get__SpGistInnerTupleData__nNodes(pointer)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(storage)
    }
}
// SAFETY: Write capability retains ReadWrite qualification. Original C performs declared-base conversion and width narrowing without Rust byte reads.
unsafe impl c::expression::WritePlace
    for Bitfield_SpGistInnerTupleData__nNodes<c::expression::ReadWrite>
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
                    __pgrx_bitfield__set_unaligned_volatile__SpGistInnerTupleData__nNodes(
                        self.address,
                        value,
                    )
                } else {
                    __pgrx_bitfield__set_unaligned__SpGistInnerTupleData__nNodes(
                        self.address,
                        value,
                    )
                }
            } else if self.access.volatile {
                __pgrx_bitfield__set_volatile__SpGistInnerTupleData__nNodes(self.address, value)
            } else {
                __pgrx_bitfield__set__SpGistInnerTupleData__nNodes(self.address, value)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(stored)
    }
}
#[doc(hidden)]
pub struct Field_prefixSize;
const _: () = {
    assert!(
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistInnerTupleData, _bitfield_1) * 8
            + 16
            == 16
    );
};
unsafe extern "C" {
    fn __pgrx_bitfield__get__SpGistInnerTupleData__prefixSize(
        p: *const crate::__pgrx_c_bindings::SpGistInnerTupleData,
    ) -> u32;
    fn __pgrx_bitfield__set__SpGistInnerTupleData__prefixSize(
        p: *mut crate::__pgrx_c_bindings::SpGistInnerTupleData,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_volatile__SpGistInnerTupleData__prefixSize(
        p: *const crate::__pgrx_c_bindings::SpGistInnerTupleData,
    ) -> u32;
    fn __pgrx_bitfield__set_volatile__SpGistInnerTupleData__prefixSize(
        p: *mut crate::__pgrx_c_bindings::SpGistInnerTupleData,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned__SpGistInnerTupleData__prefixSize(
        p: *const crate::__pgrx_c_bindings::SpGistInnerTupleData,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned__SpGistInnerTupleData__prefixSize(
        p: *mut crate::__pgrx_c_bindings::SpGistInnerTupleData,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned_volatile__SpGistInnerTupleData__prefixSize(
        p: *const crate::__pgrx_c_bindings::SpGistInnerTupleData,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned_volatile__SpGistInnerTupleData__prefixSize(
        p: *mut crate::__pgrx_c_bindings::SpGistInnerTupleData,
        value: u32,
    ) -> u32;
}
#[doc(hidden)]
pub struct Bitfield_SpGistInnerTupleData__prefixSize<Q: c::expression::Qualifier> {
    address: Q::Raw<crate::__pgrx_c_bindings::SpGistInnerTupleData>,
    access: c::expression::Access,
}
impl<Q: c::expression::Qualifier> Copy for Bitfield_SpGistInnerTupleData__prefixSize<Q> {}
impl<Q: c::expression::Qualifier> Clone for Bitfield_SpGistInnerTupleData__prefixSize<Q> {
    fn clone(&self) -> Self {
        *self
    }
}
// SAFETY: Clang and bindgen layout witnesses agree. This projects the record address without a reference or record read.
unsafe impl<Q: c::expression::Qualifier> c::expression::Field<Field_prefixSize, Q>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistInnerTupleData>
{
    type Output = Bitfield_SpGistInnerTupleData__prefixSize<Q>;
    unsafe fn project(base: c::expression::Place<Self, Q>) -> Self::Output {
        Bitfield_SpGistInnerTupleData__prefixSize::<Q> {
            address: <Q as c::expression::Qualifier>::from_mut(base.pointer().as_mut_address()),
            access: c::expression::Access {
                volatile: base.access().volatile || false,
                unaligned: base.access().unaligned,
            },
        }
    }
}
impl<Q: c::expression::Qualifier> c::expression::VolatilePlace
    for Bitfield_SpGistInnerTupleData__prefixSize<Q>
{
    type Qualified = Self;
    fn qualify_volatile(mut self) -> Self::Qualified {
        self.access.volatile = true;
        self
    }
}
// SAFETY: The same-profile C compiler owns bitfield storage reads, including partially initialized neighbor/padding bits. The marker retains declared size and compiler-proved arithmetic promotion.
unsafe impl<Q: c::expression::Qualifier> c::expression::ReadPlace
    for Bitfield_SpGistInnerTupleData__prefixSize<Q>
{
    type Object = c::CUnsignedInt;
    type Type = c::expression::CBitfield<c::CUnsignedInt, c::CInt>;
    unsafe fn load(self) -> <Self::Type as c::expression::CType>::Value {
        let pointer = Q::into_mut(self.address).cast_const(); // SAFETY: The caller establishes the original C field load contract; helpers perform only that field access.
        let storage = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__get_unaligned_volatile__SpGistInnerTupleData__prefixSize(
                        pointer,
                    )
                } else {
                    __pgrx_bitfield__get_unaligned__SpGistInnerTupleData__prefixSize(pointer)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__get_volatile__SpGistInnerTupleData__prefixSize(pointer)
            } else {
                __pgrx_bitfield__get__SpGistInnerTupleData__prefixSize(pointer)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(storage)
    }
}
// SAFETY: Write capability retains ReadWrite qualification. Original C performs declared-base conversion and width narrowing without Rust byte reads.
unsafe impl c::expression::WritePlace
    for Bitfield_SpGistInnerTupleData__prefixSize<c::expression::ReadWrite>
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
                    __pgrx_bitfield__set_unaligned_volatile__SpGistInnerTupleData__prefixSize(
                        self.address,
                        value,
                    )
                } else {
                    __pgrx_bitfield__set_unaligned__SpGistInnerTupleData__prefixSize(
                        self.address,
                        value,
                    )
                }
            } else if self.access.volatile {
                __pgrx_bitfield__set_volatile__SpGistInnerTupleData__prefixSize(self.address, value)
            } else {
                __pgrx_bitfield__set__SpGistInnerTupleData__prefixSize(self.address, value)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(stored)
    }
}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistInnerTupleData, size) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistInnerTupleData>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpGistInnerTupleData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpGistInnerTupleData> as c::expression::OrdinaryField<Field_size>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).size) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SpGistLastUsedPage>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SpGistLastUsedPage>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SpGistLastUsedPage {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistLastUsedPage, blkno) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_blkno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistLastUsedPage>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpGistLastUsedPage, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpGistLastUsedPage> as c::expression::OrdinaryField<Field_blkno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).blkno) };
#[doc(hidden)]
pub struct Field_freeSpace;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistLastUsedPage, freeSpace) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_freeSpace>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistLastUsedPage>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpGistLastUsedPage, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpGistLastUsedPage> as c::expression::OrdinaryField<Field_freeSpace>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).freeSpace) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SpGistLeafTupleData>() == 12);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SpGistLeafTupleData>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SpGistLeafTupleData {}
const _: () = {
    assert!(
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistLeafTupleData, _bitfield_1) * 8 + 0
            == 0
    );
};
unsafe extern "C" {
    fn __pgrx_bitfield__get__SpGistLeafTupleData__tupstate(
        p: *const crate::__pgrx_c_bindings::SpGistLeafTupleData,
    ) -> u32;
    fn __pgrx_bitfield__set__SpGistLeafTupleData__tupstate(
        p: *mut crate::__pgrx_c_bindings::SpGistLeafTupleData,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_volatile__SpGistLeafTupleData__tupstate(
        p: *const crate::__pgrx_c_bindings::SpGistLeafTupleData,
    ) -> u32;
    fn __pgrx_bitfield__set_volatile__SpGistLeafTupleData__tupstate(
        p: *mut crate::__pgrx_c_bindings::SpGistLeafTupleData,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned__SpGistLeafTupleData__tupstate(
        p: *const crate::__pgrx_c_bindings::SpGistLeafTupleData,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned__SpGistLeafTupleData__tupstate(
        p: *mut crate::__pgrx_c_bindings::SpGistLeafTupleData,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned_volatile__SpGistLeafTupleData__tupstate(
        p: *const crate::__pgrx_c_bindings::SpGistLeafTupleData,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned_volatile__SpGistLeafTupleData__tupstate(
        p: *mut crate::__pgrx_c_bindings::SpGistLeafTupleData,
        value: u32,
    ) -> u32;
}
#[doc(hidden)]
pub struct Bitfield_SpGistLeafTupleData__tupstate<Q: c::expression::Qualifier> {
    address: Q::Raw<crate::__pgrx_c_bindings::SpGistLeafTupleData>,
    access: c::expression::Access,
}
impl<Q: c::expression::Qualifier> Copy for Bitfield_SpGistLeafTupleData__tupstate<Q> {}
impl<Q: c::expression::Qualifier> Clone for Bitfield_SpGistLeafTupleData__tupstate<Q> {
    fn clone(&self) -> Self {
        *self
    }
}
// SAFETY: Clang and bindgen layout witnesses agree. This projects the record address without a reference or record read.
unsafe impl<Q: c::expression::Qualifier> c::expression::Field<Field_tupstate, Q>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistLeafTupleData>
{
    type Output = Bitfield_SpGistLeafTupleData__tupstate<Q>;
    unsafe fn project(base: c::expression::Place<Self, Q>) -> Self::Output {
        Bitfield_SpGistLeafTupleData__tupstate::<Q> {
            address: <Q as c::expression::Qualifier>::from_mut(base.pointer().as_mut_address()),
            access: c::expression::Access {
                volatile: base.access().volatile || false,
                unaligned: base.access().unaligned,
            },
        }
    }
}
impl<Q: c::expression::Qualifier> c::expression::VolatilePlace
    for Bitfield_SpGistLeafTupleData__tupstate<Q>
{
    type Qualified = Self;
    fn qualify_volatile(mut self) -> Self::Qualified {
        self.access.volatile = true;
        self
    }
}
// SAFETY: The same-profile C compiler owns bitfield storage reads, including partially initialized neighbor/padding bits. The marker retains declared size and compiler-proved arithmetic promotion.
unsafe impl<Q: c::expression::Qualifier> c::expression::ReadPlace
    for Bitfield_SpGistLeafTupleData__tupstate<Q>
{
    type Object = c::CUnsignedInt;
    type Type = c::expression::CBitfield<c::CUnsignedInt, c::CInt>;
    unsafe fn load(self) -> <Self::Type as c::expression::CType>::Value {
        let pointer = Q::into_mut(self.address).cast_const(); // SAFETY: The caller establishes the original C field load contract; helpers perform only that field access.
        let storage = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__get_unaligned_volatile__SpGistLeafTupleData__tupstate(pointer)
                } else {
                    __pgrx_bitfield__get_unaligned__SpGistLeafTupleData__tupstate(pointer)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__get_volatile__SpGistLeafTupleData__tupstate(pointer)
            } else {
                __pgrx_bitfield__get__SpGistLeafTupleData__tupstate(pointer)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(storage)
    }
}
// SAFETY: Write capability retains ReadWrite qualification. Original C performs declared-base conversion and width narrowing without Rust byte reads.
unsafe impl c::expression::WritePlace
    for Bitfield_SpGistLeafTupleData__tupstate<c::expression::ReadWrite>
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
                    __pgrx_bitfield__set_unaligned_volatile__SpGistLeafTupleData__tupstate(
                        self.address,
                        value,
                    )
                } else {
                    __pgrx_bitfield__set_unaligned__SpGistLeafTupleData__tupstate(
                        self.address,
                        value,
                    )
                }
            } else if self.access.volatile {
                __pgrx_bitfield__set_volatile__SpGistLeafTupleData__tupstate(self.address, value)
            } else {
                __pgrx_bitfield__set__SpGistLeafTupleData__tupstate(self.address, value)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(stored)
    }
}
const _: () = {
    assert!(
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistLeafTupleData, _bitfield_1) * 8 + 2
            == 2
    );
};
unsafe extern "C" {
    fn __pgrx_bitfield__get__SpGistLeafTupleData__size(
        p: *const crate::__pgrx_c_bindings::SpGistLeafTupleData,
    ) -> u32;
    fn __pgrx_bitfield__set__SpGistLeafTupleData__size(
        p: *mut crate::__pgrx_c_bindings::SpGistLeafTupleData,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_volatile__SpGistLeafTupleData__size(
        p: *const crate::__pgrx_c_bindings::SpGistLeafTupleData,
    ) -> u32;
    fn __pgrx_bitfield__set_volatile__SpGistLeafTupleData__size(
        p: *mut crate::__pgrx_c_bindings::SpGistLeafTupleData,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned__SpGistLeafTupleData__size(
        p: *const crate::__pgrx_c_bindings::SpGistLeafTupleData,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned__SpGistLeafTupleData__size(
        p: *mut crate::__pgrx_c_bindings::SpGistLeafTupleData,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned_volatile__SpGistLeafTupleData__size(
        p: *const crate::__pgrx_c_bindings::SpGistLeafTupleData,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned_volatile__SpGistLeafTupleData__size(
        p: *mut crate::__pgrx_c_bindings::SpGistLeafTupleData,
        value: u32,
    ) -> u32;
}
#[doc(hidden)]
pub struct Bitfield_SpGistLeafTupleData__size<Q: c::expression::Qualifier> {
    address: Q::Raw<crate::__pgrx_c_bindings::SpGistLeafTupleData>,
    access: c::expression::Access,
}
impl<Q: c::expression::Qualifier> Copy for Bitfield_SpGistLeafTupleData__size<Q> {}
impl<Q: c::expression::Qualifier> Clone for Bitfield_SpGistLeafTupleData__size<Q> {
    fn clone(&self) -> Self {
        *self
    }
}
// SAFETY: Clang and bindgen layout witnesses agree. This projects the record address without a reference or record read.
unsafe impl<Q: c::expression::Qualifier> c::expression::Field<Field_size, Q>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistLeafTupleData>
{
    type Output = Bitfield_SpGistLeafTupleData__size<Q>;
    unsafe fn project(base: c::expression::Place<Self, Q>) -> Self::Output {
        Bitfield_SpGistLeafTupleData__size::<Q> {
            address: <Q as c::expression::Qualifier>::from_mut(base.pointer().as_mut_address()),
            access: c::expression::Access {
                volatile: base.access().volatile || false,
                unaligned: base.access().unaligned,
            },
        }
    }
}
impl<Q: c::expression::Qualifier> c::expression::VolatilePlace
    for Bitfield_SpGistLeafTupleData__size<Q>
{
    type Qualified = Self;
    fn qualify_volatile(mut self) -> Self::Qualified {
        self.access.volatile = true;
        self
    }
}
// SAFETY: The same-profile C compiler owns bitfield storage reads, including partially initialized neighbor/padding bits. The marker retains declared size and compiler-proved arithmetic promotion.
unsafe impl<Q: c::expression::Qualifier> c::expression::ReadPlace
    for Bitfield_SpGistLeafTupleData__size<Q>
{
    type Object = c::CUnsignedInt;
    type Type = c::expression::CBitfield<c::CUnsignedInt, c::CInt>;
    unsafe fn load(self) -> <Self::Type as c::expression::CType>::Value {
        let pointer = Q::into_mut(self.address).cast_const(); // SAFETY: The caller establishes the original C field load contract; helpers perform only that field access.
        let storage = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__get_unaligned_volatile__SpGistLeafTupleData__size(pointer)
                } else {
                    __pgrx_bitfield__get_unaligned__SpGistLeafTupleData__size(pointer)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__get_volatile__SpGistLeafTupleData__size(pointer)
            } else {
                __pgrx_bitfield__get__SpGistLeafTupleData__size(pointer)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(storage)
    }
}
// SAFETY: Write capability retains ReadWrite qualification. Original C performs declared-base conversion and width narrowing without Rust byte reads.
unsafe impl c::expression::WritePlace
    for Bitfield_SpGistLeafTupleData__size<c::expression::ReadWrite>
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
                    __pgrx_bitfield__set_unaligned_volatile__SpGistLeafTupleData__size(
                        self.address,
                        value,
                    )
                } else {
                    __pgrx_bitfield__set_unaligned__SpGistLeafTupleData__size(self.address, value)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__set_volatile__SpGistLeafTupleData__size(self.address, value)
            } else {
                __pgrx_bitfield__set__SpGistLeafTupleData__size(self.address, value)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(stored)
    }
}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistLeafTupleData, t_info) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_t_info>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistLeafTupleData>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpGistLeafTupleData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpGistLeafTupleData> as c::expression::OrdinaryField<Field_t_info>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).t_info) };
#[doc(hidden)]
pub struct Field_magicNumber;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SpGistMetaPageData>() == 68);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SpGistMetaPageData>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SpGistMetaPageData {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistMetaPageData, magicNumber) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_magicNumber>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistMetaPageData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpGistMetaPageData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpGistMetaPageData> as c::expression::OrdinaryField<Field_magicNumber>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).magicNumber) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SpGistOptions>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SpGistOptions>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SpGistOptions {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistOptions, varlena_header_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_varlena_header_>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistOptions>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpGistOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpGistOptions> as c::expression::OrdinaryField<Field_varlena_header_>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).varlena_header_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistOptions, fillfactor) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fillfactor>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistOptions>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpGistOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpGistOptions> as c::expression::OrdinaryField<Field_fillfactor>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fillfactor) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SpGistPageOpaqueData>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SpGistPageOpaqueData>() == 2);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SpGistPageOpaqueData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistPageOpaqueData, flags) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistPageOpaqueData>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpGistPageOpaqueData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpGistPageOpaqueData> as c::expression::OrdinaryField<Field_flags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).flags) };
#[doc(hidden)]
pub struct Field_nRedirection;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistPageOpaqueData, nRedirection) == 2
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nRedirection>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistPageOpaqueData>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 2;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpGistPageOpaqueData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpGistPageOpaqueData> as c::expression::OrdinaryField<Field_nRedirection>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nRedirection) };
#[doc(hidden)]
pub struct Field_nPlaceholder;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistPageOpaqueData, nPlaceholder) == 4
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nPlaceholder>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistPageOpaqueData>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpGistPageOpaqueData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpGistPageOpaqueData> as c::expression::OrdinaryField<Field_nPlaceholder>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nPlaceholder) };
#[doc(hidden)]
pub struct Field_spgist_page_id;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistPageOpaqueData, spgist_page_id) == 6
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_spgist_page_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistPageOpaqueData>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 6;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpGistPageOpaqueData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpGistPageOpaqueData> as c::expression::OrdinaryField<Field_spgist_page_id>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).spgist_page_id) };
#[doc(hidden)]
pub struct Field_searchNulls;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SpGistScanOpaqueData>() == 10120);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SpGistScanOpaqueData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SpGistScanOpaqueData {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistScanOpaqueData, searchNulls) == 120
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_searchNulls>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistScanOpaqueData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpGistScanOpaqueData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpGistScanOpaqueData> as c::expression::OrdinaryField<Field_searchNulls>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).searchNulls) };
#[doc(hidden)]
pub struct Field_searchNonNulls;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistScanOpaqueData, searchNonNulls) == 121
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_searchNonNulls>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistScanOpaqueData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 121;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpGistScanOpaqueData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpGistScanOpaqueData> as c::expression::OrdinaryField<Field_searchNonNulls>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).searchNonNulls) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistScanOpaqueData, numberOfKeys) == 124
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numberOfKeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistScanOpaqueData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 124;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpGistScanOpaqueData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpGistScanOpaqueData> as c::expression::OrdinaryField<Field_numberOfKeys>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numberOfKeys) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistScanOpaqueData, numberOfOrderBys)
        == 136
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numberOfOrderBys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistScanOpaqueData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpGistScanOpaqueData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpGistScanOpaqueData> as c::expression::OrdinaryField<Field_numberOfOrderBys>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numberOfOrderBys) };
#[doc(hidden)]
pub struct Field_numberOfNonNullOrderBys;
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::SpGistScanOpaqueData,
        numberOfNonNullOrderBys
    ) == 140
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numberOfNonNullOrderBys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistScanOpaqueData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 140;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpGistScanOpaqueData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpGistScanOpaqueData> as c::expression::OrdinaryField<Field_numberOfNonNullOrderBys>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numberOfNonNullOrderBys) };
#[doc(hidden)]
pub struct Field_indexCollation;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistScanOpaqueData, indexCollation) == 168
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indexCollation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistScanOpaqueData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 168;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpGistScanOpaqueData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpGistScanOpaqueData> as c::expression::OrdinaryField<Field_indexCollation>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indexCollation) };
#[doc(hidden)]
pub struct Field_ntids;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistScanOpaqueData, ntids) == 296);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ntids>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistScanOpaqueData>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 296;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpGistScanOpaqueData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpGistScanOpaqueData> as c::expression::OrdinaryField<Field_ntids>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ntids) };
#[doc(hidden)]
pub struct Field_want_itup;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistScanOpaqueData, want_itup) == 304
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_want_itup>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistScanOpaqueData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 304;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpGistScanOpaqueData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpGistScanOpaqueData> as c::expression::OrdinaryField<Field_want_itup>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).want_itup) };
#[doc(hidden)]
pub struct Field_nPtrs;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistScanOpaqueData, nPtrs) == 320);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nPtrs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistScanOpaqueData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 320;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpGistScanOpaqueData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpGistScanOpaqueData> as c::expression::OrdinaryField<Field_nPtrs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nPtrs) };
#[doc(hidden)]
pub struct Field_iPtr;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistScanOpaqueData, iPtr) == 324);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_iPtr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistScanOpaqueData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 324;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpGistScanOpaqueData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpGistScanOpaqueData> as c::expression::OrdinaryField<Field_iPtr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).iPtr) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SpGistSearchItem>() == 64);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SpGistSearchItem>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SpGistSearchItem {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistSearchItem, value) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_value>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistSearchItem>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, crate::__pgrx_c_bindings::Datum>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpGistSearchItem, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpGistSearchItem> as c::expression::OrdinaryField<Field_value>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).value) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistSearchItem, level) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_level>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistSearchItem>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpGistSearchItem, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpGistSearchItem> as c::expression::OrdinaryField<Field_level>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).level) };
#[doc(hidden)]
pub struct Field_isNull;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistSearchItem, isNull) == 58);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isNull>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistSearchItem>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 58;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpGistSearchItem, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpGistSearchItem> as c::expression::OrdinaryField<Field_isNull>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isNull) };
#[doc(hidden)]
pub struct Field_isLeaf;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistSearchItem, isLeaf) == 59);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isLeaf>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistSearchItem>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 59;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpGistSearchItem, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpGistSearchItem> as c::expression::OrdinaryField<Field_isLeaf>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isLeaf) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistSearchItem, recheck) == 60);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_recheck>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistSearchItem>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 60;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpGistSearchItem, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpGistSearchItem> as c::expression::OrdinaryField<Field_recheck>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).recheck) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistSearchItem, recheckDistances) == 61
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_recheckDistances>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistSearchItem>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 61;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpGistSearchItem, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpGistSearchItem> as c::expression::OrdinaryField<Field_recheckDistances>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).recheckDistances) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistSearchItem, distances) == 64);
impl c::expression::OffsetField<Field_distances>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistSearchItem>
{
    type Member = ();
    const OFFSET: usize =
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistSearchItem, distances);
}
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SpGistState>() == 96);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SpGistState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SpGistState {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistState, attPrefixType) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attPrefixType>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistState>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::SpGistTypeDesc>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpGistState, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpGistState> as c::expression::OrdinaryField<Field_attPrefixType>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attPrefixType) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistState, attLabelType) == 60);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attLabelType>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistState>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::SpGistTypeDesc>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 60;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpGistState, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpGistState> as c::expression::OrdinaryField<Field_attLabelType>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attLabelType) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
#[doc(hidden)]
pub struct Field_myXid;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistState, myXid) == 88);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_myXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistState>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpGistState, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpGistState> as c::expression::OrdinaryField<Field_myXid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).myXid) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistState, isBuild) == 92);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isBuild>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 92;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpGistState, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpGistState> as c::expression::OrdinaryField<Field_isBuild>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isBuild) };
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SpGistTypeDesc {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistTypeDesc, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistTypeDesc>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpGistTypeDesc, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpGistTypeDesc> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistTypeDesc, attlen) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attlen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistTypeDesc>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpGistTypeDesc, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpGistTypeDesc> as c::expression::OrdinaryField<Field_attlen>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attlen) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistTypeDesc, attbyval) == 6);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attbyval>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistTypeDesc>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 6;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpGistTypeDesc, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpGistTypeDesc> as c::expression::OrdinaryField<Field_attbyval>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attbyval) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistTypeDesc, attalign) == 7);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attalign>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistTypeDesc>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 7;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpGistTypeDesc, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpGistTypeDesc> as c::expression::OrdinaryField<Field_attalign>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attalign) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistTypeDesc, attstorage) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attstorage>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistTypeDesc>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpGistTypeDesc, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpGistTypeDesc> as c::expression::OrdinaryField<Field_attstorage>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attstorage) };
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SpecialJoinInfo>() == 64);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SpecialJoinInfo>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SpecialJoinInfo {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpecialJoinInfo, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpecialJoinInfo>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpecialJoinInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpecialJoinInfo> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpecialJoinInfo, jointype) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_jointype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpecialJoinInfo>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__JoinType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpecialJoinInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpecialJoinInfo> as c::expression::OrdinaryField<Field_jointype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).jointype) };
#[doc(hidden)]
pub struct Field_lhs_strict;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpecialJoinInfo, lhs_strict) == 44);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lhs_strict>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpecialJoinInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpecialJoinInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpecialJoinInfo> as c::expression::OrdinaryField<Field_lhs_strict>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lhs_strict) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_delay_upper_joins;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SpecialJoinInfo, delay_upper_joins) == 45
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_delay_upper_joins>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpecialJoinInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 45;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpecialJoinInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpecialJoinInfo> as c::expression::OrdinaryField<Field_delay_upper_joins>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).delay_upper_joins) };
#[doc(hidden)]
pub struct Field_semi_can_btree;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SpecialJoinInfo, semi_can_btree) == 46
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_semi_can_btree>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpecialJoinInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 46;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpecialJoinInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpecialJoinInfo> as c::expression::OrdinaryField<Field_semi_can_btree>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).semi_can_btree) };
#[doc(hidden)]
pub struct Field_semi_can_hash;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SpecialJoinInfo, semi_can_hash) == 47
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_semi_can_hash>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpecialJoinInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 47;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpecialJoinInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpecialJoinInfo> as c::expression::OrdinaryField<Field_semi_can_hash>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).semi_can_hash) };
#[doc(hidden)]
pub struct Field_semi_operators;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SpecialJoinInfo, semi_operators) == 48
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_semi_operators>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpecialJoinInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpecialJoinInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpecialJoinInfo> as c::expression::OrdinaryField<Field_semi_operators>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).semi_operators) };
#[doc(hidden)]
pub struct Field_semi_rhs_exprs;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SpecialJoinInfo, semi_rhs_exprs) == 56
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_semi_rhs_exprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpecialJoinInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpecialJoinInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpecialJoinInfo> as c::expression::OrdinaryField<Field_semi_rhs_exprs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).semi_rhs_exprs) };
#[doc(hidden)]
pub struct Field_lenlist;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SplitedPageLayout>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SplitedPageLayout>() == 8);
};
#[cfg(any(feature = "pg15", feature = "pg16"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SplitedPageLayout {}
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SplitedPageLayout, lenlist) == 16);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lenlist>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SplitedPageLayout>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SplitedPageLayout, <<c::expression::CRecord<crate::__pgrx_c_bindings::SplitedPageLayout> as c::expression::OrdinaryField<Field_lenlist>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lenlist) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SplitedPageLayout, page) == 32);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_page>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SplitedPageLayout>
{
    type Member = c::expression::CPointer<
        c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SplitedPageLayout, <<c::expression::CRecord<crate::__pgrx_c_bindings::SplitedPageLayout> as c::expression::OrdinaryField<Field_page>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).page) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SplitedPageLayout, buffer) == 40);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_buffer>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SplitedPageLayout>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SplitedPageLayout, <<c::expression::CRecord<crate::__pgrx_c_bindings::SplitedPageLayout> as c::expression::OrdinaryField<Field_buffer>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).buffer) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::StartReplicationCmd>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::StartReplicationCmd>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::StartReplicationCmd {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::StartReplicationCmd, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::StartReplicationCmd>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::StartReplicationCmd, <<c::expression::CRecord<crate::__pgrx_c_bindings::StartReplicationCmd> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::StartReplicationCmd, kind) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_kind>
    for c::expression::CRecord<crate::__pgrx_c_bindings::StartReplicationCmd>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ReplicationKind,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::StartReplicationCmd, <<c::expression::CRecord<crate::__pgrx_c_bindings::StartReplicationCmd> as c::expression::OrdinaryField<Field_kind>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).kind) };
#[doc(hidden)]
pub struct Field_timeline;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::StartReplicationCmd, timeline) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_timeline>
    for c::expression::CRecord<crate::__pgrx_c_bindings::StartReplicationCmd>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::StartReplicationCmd, <<c::expression::CRecord<crate::__pgrx_c_bindings::StartReplicationCmd> as c::expression::OrdinaryField<Field_timeline>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).timeline) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::StartReplicationCmd, startpoint) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_startpoint>
    for c::expression::CRecord<crate::__pgrx_c_bindings::StartReplicationCmd>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::StartReplicationCmd, <<c::expression::CRecord<crate::__pgrx_c_bindings::StartReplicationCmd> as c::expression::OrdinaryField<Field_startpoint>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).startpoint) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::StartReplicationCmd, options) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::StartReplicationCmd>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::StartReplicationCmd, <<c::expression::CRecord<crate::__pgrx_c_bindings::StartReplicationCmd> as c::expression::OrdinaryField<Field_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::StatisticExtInfo>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::StatisticExtInfo>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::StatisticExtInfo {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::StatisticExtInfo, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::StatisticExtInfo>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::StatisticExtInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::StatisticExtInfo> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_statOid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::StatisticExtInfo, statOid) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_statOid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::StatisticExtInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::StatisticExtInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::StatisticExtInfo> as c::expression::OrdinaryField<Field_statOid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).statOid) };
#[doc(hidden)]
pub struct Field_inherit;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::StatisticExtInfo, inherit) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inherit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::StatisticExtInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::StatisticExtInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::StatisticExtInfo> as c::expression::OrdinaryField<Field_inherit>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).inherit) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::StatisticExtInfo, kind) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_kind>
    for c::expression::CRecord<crate::__pgrx_c_bindings::StatisticExtInfo>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::StatisticExtInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::StatisticExtInfo> as c::expression::OrdinaryField<Field_kind>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).kind) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::StatisticExtInfo, exprs) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_exprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::StatisticExtInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::StatisticExtInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::StatisticExtInfo> as c::expression::OrdinaryField<Field_exprs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).exprs) };
#[doc(hidden)]
pub struct Field_numrows;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::StatsBuildData>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::StatsBuildData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::StatsBuildData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::StatsBuildData, numrows) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numrows>
    for c::expression::CRecord<crate::__pgrx_c_bindings::StatsBuildData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::StatsBuildData, <<c::expression::CRecord<crate::__pgrx_c_bindings::StatsBuildData> as c::expression::OrdinaryField<Field_numrows>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numrows) };
#[doc(hidden)]
pub struct Field_nattnums;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::StatsBuildData, nattnums) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nattnums>
    for c::expression::CRecord<crate::__pgrx_c_bindings::StatsBuildData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::StatsBuildData, <<c::expression::CRecord<crate::__pgrx_c_bindings::StatsBuildData> as c::expression::OrdinaryField<Field_nattnums>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nattnums) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::StatsElem>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::StatsElem>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::StatsElem {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::StatsElem, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::StatsElem>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::StatsElem,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::StatsElem> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::StdRdOptions>() == 112);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::StdRdOptions>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::StdRdOptions {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::StdRdOptions, vl_len_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vl_len_>
    for c::expression::CRecord<crate::__pgrx_c_bindings::StdRdOptions>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::StdRdOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::StdRdOptions> as c::expression::OrdinaryField<Field_vl_len_>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).vl_len_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::StdRdOptions, fillfactor) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fillfactor>
    for c::expression::CRecord<crate::__pgrx_c_bindings::StdRdOptions>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::StdRdOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::StdRdOptions> as c::expression::OrdinaryField<Field_fillfactor>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fillfactor) };
#[doc(hidden)]
pub struct Field_toast_tuple_target;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::StdRdOptions, toast_tuple_target) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_toast_tuple_target>
    for c::expression::CRecord<crate::__pgrx_c_bindings::StdRdOptions>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::StdRdOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::StdRdOptions> as c::expression::OrdinaryField<Field_toast_tuple_target>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).toast_tuple_target) };
#[doc(hidden)]
pub struct Field_user_catalog_table;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::StdRdOptions, user_catalog_table) == 96
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_user_catalog_table>
    for c::expression::CRecord<crate::__pgrx_c_bindings::StdRdOptions>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::StdRdOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::StdRdOptions> as c::expression::OrdinaryField<Field_user_catalog_table>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).user_catalog_table) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::StdRdOptions, parallel_workers) == 100
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parallel_workers>
    for c::expression::CRecord<crate::__pgrx_c_bindings::StdRdOptions>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 100;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::StdRdOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::StdRdOptions> as c::expression::OrdinaryField<Field_parallel_workers>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).parallel_workers) };
#[doc(hidden)]
pub struct Field_vacuum_index_cleanup;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::StdRdOptions, vacuum_index_cleanup) == 104
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vacuum_index_cleanup>
    for c::expression::CRecord<crate::__pgrx_c_bindings::StdRdOptions>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__StdRdOptIndexCleanup,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::StdRdOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::StdRdOptions> as c::expression::OrdinaryField<Field_vacuum_index_cleanup>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).vacuum_index_cleanup) };
#[doc(hidden)]
pub struct Field_vacuum_truncate;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::StdRdOptions, vacuum_truncate) == 108
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vacuum_truncate>
    for c::expression::CRecord<crate::__pgrx_c_bindings::StdRdOptions>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 108;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::StdRdOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::StdRdOptions> as c::expression::OrdinaryField<Field_vacuum_truncate>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).vacuum_truncate) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::String>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::String>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::String {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::String, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::String>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::String,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::String> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_sval;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::String, sval) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sval>
    for c::expression::CRecord<crate::__pgrx_c_bindings::String>
{
    type Member = c::expression::CPointer<
        c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::String,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::String> as c::expression::OrdinaryField<
        Field_sval,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sval) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::StringInfoData>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::StringInfoData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::StringInfoData {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::StringInfoData, data) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_data>
    for c::expression::CRecord<crate::__pgrx_c_bindings::StringInfoData>
{
    type Member = c::expression::CPointer<
        c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::StringInfoData, <<c::expression::CRecord<crate::__pgrx_c_bindings::StringInfoData> as c::expression::OrdinaryField<Field_data>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).data) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::StringInfoData, len) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::StringInfoData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::StringInfoData, <<c::expression::CRecord<crate::__pgrx_c_bindings::StringInfoData> as c::expression::OrdinaryField<Field_len>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).len) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::StringInfoData, maxlen) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_maxlen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::StringInfoData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::StringInfoData, <<c::expression::CRecord<crate::__pgrx_c_bindings::StringInfoData> as c::expression::OrdinaryField<Field_maxlen>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).maxlen) };
#[doc(hidden)]
pub struct Field_cursor;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::StringInfoData, cursor) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cursor>
    for c::expression::CRecord<crate::__pgrx_c_bindings::StringInfoData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::StringInfoData, <<c::expression::CRecord<crate::__pgrx_c_bindings::StringInfoData> as c::expression::OrdinaryField<Field_cursor>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cursor) };
#[doc(hidden)]
pub struct Field_subLinkType;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SubLink>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SubLink>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SubLink {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubLink, subLinkType) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_subLinkType>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubLink>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__SubLinkType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SubLink,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SubLink> as c::expression::OrdinaryField<
        Field_subLinkType,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).subLinkType) };
#[doc(hidden)]
pub struct Field_subLinkId;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubLink, subLinkId) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_subLinkId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubLink>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SubLink,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SubLink> as c::expression::OrdinaryField<
        Field_subLinkId,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).subLinkId) };
#[doc(hidden)]
pub struct Field_operName;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubLink, operName) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_operName>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubLink>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SubLink,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SubLink> as c::expression::OrdinaryField<
        Field_operName,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).operName) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubLink, location) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubLink>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SubLink,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SubLink> as c::expression::OrdinaryField<
        Field_location,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SubPlan>() == 96);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SubPlan>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SubPlan {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubPlan, subLinkType) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_subLinkType>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubPlan>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__SubLinkType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SubPlan,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SubPlan> as c::expression::OrdinaryField<
        Field_subLinkType,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).subLinkType) };
#[doc(hidden)]
pub struct Field_paramIds;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubPlan, paramIds) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_paramIds>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubPlan>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SubPlan,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SubPlan> as c::expression::OrdinaryField<
        Field_paramIds,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).paramIds) };
#[doc(hidden)]
pub struct Field_plan_id;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubPlan, plan_id) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_plan_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubPlan>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SubPlan,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SubPlan> as c::expression::OrdinaryField<
        Field_plan_id,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).plan_id) };
#[doc(hidden)]
pub struct Field_firstColType;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubPlan, firstColType) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_firstColType>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubPlan>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SubPlan,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SubPlan> as c::expression::OrdinaryField<
        Field_firstColType,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).firstColType) };
#[doc(hidden)]
pub struct Field_firstColTypmod;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubPlan, firstColTypmod) == 44);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_firstColTypmod>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubPlan>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SubPlan,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SubPlan> as c::expression::OrdinaryField<
        Field_firstColTypmod,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).firstColTypmod) };
#[doc(hidden)]
pub struct Field_firstColCollation;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubPlan, firstColCollation) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_firstColCollation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubPlan>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SubPlan,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SubPlan> as c::expression::OrdinaryField<
        Field_firstColCollation,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).firstColCollation) };
#[doc(hidden)]
pub struct Field_useHashTable;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubPlan, useHashTable) == 52);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_useHashTable>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubPlan>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SubPlan,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SubPlan> as c::expression::OrdinaryField<
        Field_useHashTable,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).useHashTable) };
#[doc(hidden)]
pub struct Field_unknownEqFalse;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubPlan, unknownEqFalse) == 53);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_unknownEqFalse>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubPlan>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 53;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SubPlan,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SubPlan> as c::expression::OrdinaryField<
        Field_unknownEqFalse,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).unknownEqFalse) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubPlan, parallel_safe) == 54);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parallel_safe>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubPlan>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 54;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SubPlan,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SubPlan> as c::expression::OrdinaryField<
        Field_parallel_safe,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).parallel_safe) };
#[doc(hidden)]
pub struct Field_setParam;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubPlan, setParam) == 56);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_setParam>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubPlan>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SubPlan,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SubPlan> as c::expression::OrdinaryField<
        Field_setParam,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).setParam) };
#[doc(hidden)]
pub struct Field_parParam;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubPlan, parParam) == 64);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parParam>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubPlan>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SubPlan,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SubPlan> as c::expression::OrdinaryField<
        Field_parParam,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).parParam) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubPlan, args) == 72);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_args>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubPlan>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SubPlan,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SubPlan> as c::expression::OrdinaryField<
        Field_args,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).args) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SubPlanState>() == 208);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SubPlanState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SubPlanState {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubPlanState, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubPlanState>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SubPlanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::SubPlanState> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubPlanState, args) == 40);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_args>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubPlanState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SubPlanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::SubPlanState> as c::expression::OrdinaryField<Field_args>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).args) };
#[doc(hidden)]
pub struct Field_curArray;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubPlanState, curArray) == 56);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_curArray>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubPlanState>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, crate::__pgrx_c_bindings::Datum>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SubPlanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::SubPlanState> as c::expression::OrdinaryField<Field_curArray>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).curArray) };
#[doc(hidden)]
pub struct Field_havehashrows;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubPlanState, havehashrows) == 104);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_havehashrows>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubPlanState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SubPlanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::SubPlanState> as c::expression::OrdinaryField<Field_havehashrows>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).havehashrows) };
#[doc(hidden)]
pub struct Field_havenullrows;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubPlanState, havenullrows) == 105);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_havenullrows>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubPlanState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 105;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SubPlanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::SubPlanState> as c::expression::OrdinaryField<Field_havenullrows>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).havenullrows) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubPlanState, numCols) == 136);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numCols>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubPlanState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SubPlanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::SubPlanState> as c::expression::OrdinaryField<Field_numCols>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numCols) };
#[doc(hidden)]
pub struct Field_scanstatus;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SubqueryScan>() == 128);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SubqueryScan>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SubqueryScan {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubqueryScan, scanstatus) == 120);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_scanstatus>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubqueryScan>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__SubqueryScanStatus,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SubqueryScan, <<c::expression::CRecord<crate::__pgrx_c_bindings::SubqueryScan> as c::expression::OrdinaryField<Field_scanstatus>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).scanstatus) };
#[doc(hidden)]
pub struct Field_isassignment;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SubscriptingRefState>() == 112);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SubscriptingRefState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SubscriptingRefState {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SubscriptingRefState, isassignment) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isassignment>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubscriptingRefState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SubscriptingRefState, <<c::expression::CRecord<crate::__pgrx_c_bindings::SubscriptingRefState> as c::expression::OrdinaryField<Field_isassignment>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isassignment) };
#[doc(hidden)]
pub struct Field_numupper;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SubscriptingRefState, numupper) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numupper>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubscriptingRefState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SubscriptingRefState, <<c::expression::CRecord<crate::__pgrx_c_bindings::SubscriptingRefState> as c::expression::OrdinaryField<Field_numupper>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numupper) };
#[doc(hidden)]
pub struct Field_numlower;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SubscriptingRefState, numlower) == 48
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numlower>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubscriptingRefState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SubscriptingRefState, <<c::expression::CRecord<crate::__pgrx_c_bindings::SubscriptingRefState> as c::expression::OrdinaryField<Field_numlower>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numlower) };
#[doc(hidden)]
pub struct Field_replacevalue;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SubscriptingRefState, replacevalue) == 80
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_replacevalue>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubscriptingRefState>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, crate::__pgrx_c_bindings::Datum>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SubscriptingRefState, <<c::expression::CRecord<crate::__pgrx_c_bindings::SubscriptingRefState> as c::expression::OrdinaryField<Field_replacevalue>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).replacevalue) };
#[doc(hidden)]
pub struct Field_replacenull;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SubscriptingRefState, replacenull) == 88
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_replacenull>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubscriptingRefState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SubscriptingRefState, <<c::expression::CRecord<crate::__pgrx_c_bindings::SubscriptingRefState> as c::expression::OrdinaryField<Field_replacenull>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).replacenull) };
#[doc(hidden)]
pub struct Field_prevvalue;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SubscriptingRefState, prevvalue) == 96
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_prevvalue>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubscriptingRefState>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, crate::__pgrx_c_bindings::Datum>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SubscriptingRefState, <<c::expression::CRecord<crate::__pgrx_c_bindings::SubscriptingRefState> as c::expression::OrdinaryField<Field_prevvalue>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).prevvalue) };
#[doc(hidden)]
pub struct Field_prevnull;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SubscriptingRefState, prevnull) == 104
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_prevnull>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubscriptingRefState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SubscriptingRefState, <<c::expression::CRecord<crate::__pgrx_c_bindings::SubscriptingRefState> as c::expression::OrdinaryField<Field_prevnull>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).prevnull) };
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Subscription>() == 72);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Subscription>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::Subscription {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Subscription, oid) == 0);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Subscription>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Subscription, <<c::expression::CRecord<crate::__pgrx_c_bindings::Subscription> as c::expression::OrdinaryField<Field_oid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oid) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Subscription, dbid) == 4);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dbid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Subscription>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Subscription, <<c::expression::CRecord<crate::__pgrx_c_bindings::Subscription> as c::expression::OrdinaryField<Field_dbid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dbid) };
#[doc(hidden)]
pub struct Field_skiplsn;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Subscription, skiplsn) == 8);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_skiplsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Subscription>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Subscription, <<c::expression::CRecord<crate::__pgrx_c_bindings::Subscription> as c::expression::OrdinaryField<Field_skiplsn>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).skiplsn) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Subscription, owner) == 24);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_owner>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Subscription>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Subscription, <<c::expression::CRecord<crate::__pgrx_c_bindings::Subscription> as c::expression::OrdinaryField<Field_owner>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).owner) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Subscription, enabled) == 28);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_enabled>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Subscription>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Subscription, <<c::expression::CRecord<crate::__pgrx_c_bindings::Subscription> as c::expression::OrdinaryField<Field_enabled>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).enabled) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Subscription, binary) == 29);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_binary>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Subscription>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 29;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Subscription, <<c::expression::CRecord<crate::__pgrx_c_bindings::Subscription> as c::expression::OrdinaryField<Field_binary>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).binary) };
#[doc(hidden)]
pub struct Field_stream;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Subscription, stream) == 30);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stream>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Subscription>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 30;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Subscription, <<c::expression::CRecord<crate::__pgrx_c_bindings::Subscription> as c::expression::OrdinaryField<Field_stream>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stream) };
#[doc(hidden)]
pub struct Field_twophasestate;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Subscription, twophasestate) == 31);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_twophasestate>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Subscription>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 31;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Subscription, <<c::expression::CRecord<crate::__pgrx_c_bindings::Subscription> as c::expression::OrdinaryField<Field_twophasestate>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).twophasestate) };
#[doc(hidden)]
pub struct Field_disableonerr;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Subscription, disableonerr) == 32);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_disableonerr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Subscription>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Subscription, <<c::expression::CRecord<crate::__pgrx_c_bindings::Subscription> as c::expression::OrdinaryField<Field_disableonerr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).disableonerr) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Subscription, publications) == 64);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_publications>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Subscription>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Subscription, <<c::expression::CRecord<crate::__pgrx_c_bindings::Subscription> as c::expression::OrdinaryField<Field_publications>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).publications) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SubscriptionRelState>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SubscriptionRelState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SubscriptionRelState {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubscriptionRelState, relid) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubscriptionRelState>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SubscriptionRelState, <<c::expression::CRecord<crate::__pgrx_c_bindings::SubscriptionRelState> as c::expression::OrdinaryField<Field_relid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relid) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubscriptionRelState, lsn) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubscriptionRelState>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SubscriptionRelState, <<c::expression::CRecord<crate::__pgrx_c_bindings::SubscriptionRelState> as c::expression::OrdinaryField<Field_lsn>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lsn) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubscriptionRelState, state) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_state>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubscriptionRelState>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SubscriptionRelState, <<c::expression::CRecord<crate::__pgrx_c_bindings::SubscriptionRelState> as c::expression::OrdinaryField<Field_state>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).state) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SupportRequestCost>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SupportRequestCost>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SupportRequestCost {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SupportRequestCost, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestCost>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SupportRequestCost, <<c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestCost> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SupportRequestCost, funcid) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_funcid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestCost>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SupportRequestCost, <<c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestCost> as c::expression::OrdinaryField<Field_funcid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).funcid) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SupportRequestIndexCondition>() == 64);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SupportRequestIndexCondition>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SupportRequestIndexCondition {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SupportRequestIndexCondition, type_) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestIndexCondition>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SupportRequestIndexCondition, <<c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestIndexCondition> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SupportRequestIndexCondition, funcid) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_funcid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestIndexCondition>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SupportRequestIndexCondition, <<c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestIndexCondition> as c::expression::OrdinaryField<Field_funcid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).funcid) };
#[doc(hidden)]
pub struct Field_indexarg;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SupportRequestIndexCondition, indexarg) == 32
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indexarg>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestIndexCondition>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SupportRequestIndexCondition, <<c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestIndexCondition> as c::expression::OrdinaryField<Field_indexarg>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indexarg) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SupportRequestIndexCondition, indexcol) == 48
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indexcol>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestIndexCondition>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SupportRequestIndexCondition, <<c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestIndexCondition> as c::expression::OrdinaryField<Field_indexcol>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indexcol) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SupportRequestIndexCondition, opfamily) == 52
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_opfamily>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestIndexCondition>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SupportRequestIndexCondition, <<c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestIndexCondition> as c::expression::OrdinaryField<Field_opfamily>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).opfamily) };
#[doc(hidden)]
pub struct Field_indexcollation;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SupportRequestIndexCondition, indexcollation)
        == 56
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indexcollation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestIndexCondition>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SupportRequestIndexCondition, <<c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestIndexCondition> as c::expression::OrdinaryField<Field_indexcollation>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indexcollation) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SupportRequestIndexCondition, lossy) == 60
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lossy>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestIndexCondition>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 60;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SupportRequestIndexCondition, <<c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestIndexCondition> as c::expression::OrdinaryField<Field_lossy>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lossy) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SupportRequestRows>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SupportRequestRows>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SupportRequestRows {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SupportRequestRows, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestRows>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SupportRequestRows, <<c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestRows> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SupportRequestRows, funcid) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_funcid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestRows>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SupportRequestRows, <<c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestRows> as c::expression::OrdinaryField<Field_funcid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).funcid) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SupportRequestSelectivity>() == 64);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SupportRequestSelectivity>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SupportRequestSelectivity {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SupportRequestSelectivity, type_) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestSelectivity>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SupportRequestSelectivity, <<c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestSelectivity> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SupportRequestSelectivity, funcid) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_funcid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestSelectivity>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SupportRequestSelectivity, <<c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestSelectivity> as c::expression::OrdinaryField<Field_funcid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).funcid) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SupportRequestSelectivity, args) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_args>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestSelectivity>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SupportRequestSelectivity, <<c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestSelectivity> as c::expression::OrdinaryField<Field_args>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).args) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SupportRequestSelectivity, inputcollid) == 32
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inputcollid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestSelectivity>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SupportRequestSelectivity, <<c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestSelectivity> as c::expression::OrdinaryField<Field_inputcollid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).inputcollid) };
#[doc(hidden)]
pub struct Field_is_join;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SupportRequestSelectivity, is_join) == 36
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_join>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestSelectivity>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SupportRequestSelectivity, <<c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestSelectivity> as c::expression::OrdinaryField<Field_is_join>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_join) };
#[doc(hidden)]
pub struct Field_varRelid;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SupportRequestSelectivity, varRelid) == 40
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_varRelid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestSelectivity>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SupportRequestSelectivity, <<c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestSelectivity> as c::expression::OrdinaryField<Field_varRelid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).varRelid) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SupportRequestSelectivity, jointype) == 44
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_jointype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestSelectivity>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__JoinType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SupportRequestSelectivity, <<c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestSelectivity> as c::expression::OrdinaryField<Field_jointype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).jointype) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SupportRequestSimplify>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SupportRequestSimplify>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SupportRequestSimplify {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SupportRequestSimplify, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestSimplify>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SupportRequestSimplify, <<c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestSimplify> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SupportRequestWFuncMonotonic>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SupportRequestWFuncMonotonic>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SupportRequestWFuncMonotonic {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SupportRequestWFuncMonotonic, type_) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestWFuncMonotonic>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SupportRequestWFuncMonotonic, <<c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestWFuncMonotonic> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_monotonic;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SupportRequestWFuncMonotonic, monotonic)
        == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_monotonic>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestWFuncMonotonic>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__MonotonicFunction,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SupportRequestWFuncMonotonic, <<c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestWFuncMonotonic> as c::expression::OrdinaryField<Field_monotonic>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).monotonic) };
#[doc(hidden)]
pub struct Field_config_size;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SyncRepConfigData>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SyncRepConfigData>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SyncRepConfigData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SyncRepConfigData, config_size) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_config_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SyncRepConfigData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SyncRepConfigData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SyncRepConfigData> as c::expression::OrdinaryField<Field_config_size>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).config_size) };
#[doc(hidden)]
pub struct Field_num_sync;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SyncRepConfigData, num_sync) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_sync>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SyncRepConfigData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SyncRepConfigData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SyncRepConfigData> as c::expression::OrdinaryField<Field_num_sync>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_sync) };
#[doc(hidden)]
pub struct Field_syncrep_method;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SyncRepConfigData, syncrep_method) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_syncrep_method>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SyncRepConfigData>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SyncRepConfigData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SyncRepConfigData> as c::expression::OrdinaryField<Field_syncrep_method>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).syncrep_method) };
#[doc(hidden)]
pub struct Field_nmembers;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SyncRepConfigData, nmembers) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nmembers>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SyncRepConfigData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SyncRepConfigData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SyncRepConfigData> as c::expression::OrdinaryField<Field_nmembers>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nmembers) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SyncRepStandbyData>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SyncRepStandbyData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SyncRepStandbyData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SyncRepStandbyData, pid) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SyncRepStandbyData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SyncRepStandbyData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SyncRepStandbyData> as c::expression::OrdinaryField<Field_pid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pid) };
#[doc(hidden)]
pub struct Field_write;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SyncRepStandbyData, write) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_write>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SyncRepStandbyData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SyncRepStandbyData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SyncRepStandbyData> as c::expression::OrdinaryField<Field_write>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).write) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SyncRepStandbyData, flush) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_flush>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SyncRepStandbyData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SyncRepStandbyData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SyncRepStandbyData> as c::expression::OrdinaryField<Field_flush>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).flush) };
#[doc(hidden)]
pub struct Field_apply;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SyncRepStandbyData, apply) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_apply>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SyncRepStandbyData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SyncRepStandbyData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SyncRepStandbyData> as c::expression::OrdinaryField<Field_apply>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).apply) };
#[doc(hidden)]
pub struct Field_sync_standby_priority;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SyncRepStandbyData, sync_standby_priority)
        == 32
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sync_standby_priority>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SyncRepStandbyData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SyncRepStandbyData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SyncRepStandbyData> as c::expression::OrdinaryField<Field_sync_standby_priority>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sync_standby_priority) };
#[doc(hidden)]
pub struct Field_walsnd_index;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SyncRepStandbyData, walsnd_index) == 36
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_walsnd_index>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SyncRepStandbyData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SyncRepStandbyData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SyncRepStandbyData> as c::expression::OrdinaryField<Field_walsnd_index>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).walsnd_index) };
#[doc(hidden)]
pub struct Field_is_me;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SyncRepStandbyData, is_me) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_me>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SyncRepStandbyData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SyncRepStandbyData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SyncRepStandbyData> as c::expression::OrdinaryField<Field_is_me>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_me) };
#[doc(hidden)]
pub struct Field_blockno;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TBMIterateResult>() == 12);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TBMIterateResult>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TBMIterateResult {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TBMIterateResult, blockno) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_blockno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TBMIterateResult>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TBMIterateResult, <<c::expression::CRecord<crate::__pgrx_c_bindings::TBMIterateResult> as c::expression::OrdinaryField<Field_blockno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).blockno) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TBMIterateResult, ntuples) == 4);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ntuples>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TBMIterateResult>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TBMIterateResult, <<c::expression::CRecord<crate::__pgrx_c_bindings::TBMIterateResult> as c::expression::OrdinaryField<Field_ntuples>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ntuples) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TBMIterateResult, recheck) == 8);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_recheck>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TBMIterateResult>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TBMIterateResult, <<c::expression::CRecord<crate::__pgrx_c_bindings::TBMIterateResult> as c::expression::OrdinaryField<Field_recheck>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).recheck) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TM_FailureData>() == 20);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TM_FailureData>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TM_FailureData {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TM_FailureData, xmax) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xmax>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TM_FailureData>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TM_FailureData, <<c::expression::CRecord<crate::__pgrx_c_bindings::TM_FailureData> as c::expression::OrdinaryField<Field_xmax>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xmax) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TM_FailureData, cmax) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cmax>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TM_FailureData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TM_FailureData, <<c::expression::CRecord<crate::__pgrx_c_bindings::TM_FailureData> as c::expression::OrdinaryField<Field_cmax>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cmax) };
#[doc(hidden)]
pub struct Field_traversed;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TM_FailureData, traversed) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_traversed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TM_FailureData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TM_FailureData, <<c::expression::CRecord<crate::__pgrx_c_bindings::TM_FailureData> as c::expression::OrdinaryField<Field_traversed>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).traversed) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TM_IndexDelete>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TM_IndexDelete>() == 2);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TM_IndexDelete {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TM_IndexDelete, id) == 6);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TM_IndexDelete>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 6;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TM_IndexDelete, <<c::expression::CRecord<crate::__pgrx_c_bindings::TM_IndexDelete> as c::expression::OrdinaryField<Field_id>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).id) };
#[doc(hidden)]
pub struct Field_iblknum;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TM_IndexDeleteOp>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TM_IndexDeleteOp>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TM_IndexDeleteOp {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TM_IndexDeleteOp, iblknum) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_iblknum>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TM_IndexDeleteOp>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TM_IndexDeleteOp, <<c::expression::CRecord<crate::__pgrx_c_bindings::TM_IndexDeleteOp> as c::expression::OrdinaryField<Field_iblknum>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).iblknum) };
#[doc(hidden)]
pub struct Field_bottomup;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TM_IndexDeleteOp, bottomup) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bottomup>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TM_IndexDeleteOp>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TM_IndexDeleteOp, <<c::expression::CRecord<crate::__pgrx_c_bindings::TM_IndexDeleteOp> as c::expression::OrdinaryField<Field_bottomup>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).bottomup) };
#[doc(hidden)]
pub struct Field_bottomupfreespace;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TM_IndexDeleteOp, bottomupfreespace) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bottomupfreespace>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TM_IndexDeleteOp>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TM_IndexDeleteOp, <<c::expression::CRecord<crate::__pgrx_c_bindings::TM_IndexDeleteOp> as c::expression::OrdinaryField<Field_bottomupfreespace>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).bottomupfreespace) };
#[doc(hidden)]
pub struct Field_ndeltids;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TM_IndexDeleteOp, ndeltids) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ndeltids>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TM_IndexDeleteOp>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TM_IndexDeleteOp, <<c::expression::CRecord<crate::__pgrx_c_bindings::TM_IndexDeleteOp> as c::expression::OrdinaryField<Field_ndeltids>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ndeltids) };
#[doc(hidden)]
pub struct Field_idxoffnum;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TM_IndexStatus>() == 6);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TM_IndexStatus>() == 2);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TM_IndexStatus {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TM_IndexStatus, idxoffnum) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_idxoffnum>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TM_IndexStatus>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TM_IndexStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::TM_IndexStatus> as c::expression::OrdinaryField<Field_idxoffnum>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).idxoffnum) };
#[doc(hidden)]
pub struct Field_knowndeletable;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TM_IndexStatus, knowndeletable) == 2);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_knowndeletable>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TM_IndexStatus>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 2;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TM_IndexStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::TM_IndexStatus> as c::expression::OrdinaryField<Field_knowndeletable>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).knowndeletable) };
#[doc(hidden)]
pub struct Field_promising;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TM_IndexStatus, promising) == 3);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_promising>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TM_IndexStatus>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 3;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TM_IndexStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::TM_IndexStatus> as c::expression::OrdinaryField<Field_promising>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).promising) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TM_IndexStatus, freespace) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_freespace>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TM_IndexStatus>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TM_IndexStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::TM_IndexStatus> as c::expression::OrdinaryField<Field_freespace>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).freespace) };
#[doc(hidden)]
pub struct Field_objId;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TSAnyCacheEntry>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TSAnyCacheEntry>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TSAnyCacheEntry {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TSAnyCacheEntry, objId) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_objId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TSAnyCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TSAnyCacheEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TSAnyCacheEntry> as c::expression::OrdinaryField<Field_objId>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).objId) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TSAnyCacheEntry, isvalid) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isvalid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TSAnyCacheEntry>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TSAnyCacheEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TSAnyCacheEntry> as c::expression::OrdinaryField<Field_isvalid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isvalid) };
#[doc(hidden)]
pub struct Field_dictId;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TSDictionaryCacheEntry>() == 80);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TSDictionaryCacheEntry>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TSDictionaryCacheEntry {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TSDictionaryCacheEntry, dictId) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dictId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TSDictionaryCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TSDictionaryCacheEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TSDictionaryCacheEntry> as c::expression::OrdinaryField<Field_dictId>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dictId) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TSDictionaryCacheEntry, isvalid) == 4
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isvalid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TSDictionaryCacheEntry>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TSDictionaryCacheEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TSDictionaryCacheEntry> as c::expression::OrdinaryField<Field_isvalid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isvalid) };
#[doc(hidden)]
pub struct Field_lexizeOid;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TSDictionaryCacheEntry, lexizeOid) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lexizeOid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TSDictionaryCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TSDictionaryCacheEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TSDictionaryCacheEntry> as c::expression::OrdinaryField<Field_lexizeOid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lexizeOid) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TSParserCacheEntry>() == 224);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TSParserCacheEntry>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TSParserCacheEntry {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TSParserCacheEntry, prsId) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_prsId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TSParserCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TSParserCacheEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TSParserCacheEntry> as c::expression::OrdinaryField<Field_prsId>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).prsId) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TSParserCacheEntry, isvalid) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isvalid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TSParserCacheEntry>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TSParserCacheEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TSParserCacheEntry> as c::expression::OrdinaryField<Field_isvalid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isvalid) };
#[doc(hidden)]
pub struct Field_startOid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TSParserCacheEntry, startOid) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_startOid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TSParserCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TSParserCacheEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TSParserCacheEntry> as c::expression::OrdinaryField<Field_startOid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).startOid) };
#[doc(hidden)]
pub struct Field_tokenOid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TSParserCacheEntry, tokenOid) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tokenOid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TSParserCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TSParserCacheEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TSParserCacheEntry> as c::expression::OrdinaryField<Field_tokenOid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tokenOid) };
#[doc(hidden)]
pub struct Field_endOid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TSParserCacheEntry, endOid) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_endOid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TSParserCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TSParserCacheEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TSParserCacheEntry> as c::expression::OrdinaryField<Field_endOid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).endOid) };
#[doc(hidden)]
pub struct Field_headlineOid;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TSParserCacheEntry, headlineOid) == 20
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_headlineOid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TSParserCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
