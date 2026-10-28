// Shared generated C macro support.

// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hashkeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Hash>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Hash,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Hash> as c::expression::OrdinaryField<
        Field_hashkeys,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hashkeys) };
#[doc(hidden)]
pub struct Field_skewTable;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Hash, skewTable) == 112);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_skewTable>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Hash>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Hash,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Hash> as c::expression::OrdinaryField<
        Field_skewTable,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).skewTable) };
#[doc(hidden)]
pub struct Field_skewColumn;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Hash, skewColumn) == 116);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_skewColumn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Hash>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 116;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Hash,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Hash> as c::expression::OrdinaryField<
        Field_skewColumn,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).skewColumn) };
#[doc(hidden)]
pub struct Field_skewInherit;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Hash, skewInherit) == 118);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_skewInherit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Hash>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 118;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Hash,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Hash> as c::expression::OrdinaryField<
        Field_skewInherit,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).skewInherit) };
#[doc(hidden)]
pub struct Field_nbuckets;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::HashInstrumentation>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::HashInstrumentation>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::HashInstrumentation {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HashInstrumentation, nbuckets) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nbuckets>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashInstrumentation>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashInstrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashInstrumentation> as c::expression::OrdinaryField<Field_nbuckets>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nbuckets) };
#[doc(hidden)]
pub struct Field_nbuckets_original;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashInstrumentation, nbuckets_original) == 4
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nbuckets_original>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashInstrumentation>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashInstrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashInstrumentation> as c::expression::OrdinaryField<Field_nbuckets_original>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nbuckets_original) };
#[doc(hidden)]
pub struct Field_nbatch;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HashInstrumentation, nbatch) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nbatch>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashInstrumentation>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashInstrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashInstrumentation> as c::expression::OrdinaryField<Field_nbatch>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nbatch) };
#[doc(hidden)]
pub struct Field_nbatch_original;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashInstrumentation, nbatch_original) == 12
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nbatch_original>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashInstrumentation>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashInstrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashInstrumentation> as c::expression::OrdinaryField<Field_nbatch_original>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nbatch_original) };
#[doc(hidden)]
pub struct Field_space_peak;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashInstrumentation, space_peak) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_space_peak>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashInstrumentation>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashInstrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashInstrumentation> as c::expression::OrdinaryField<Field_space_peak>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).space_peak) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
#[doc(hidden)]
pub struct Field_hj_OuterHashKeys;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::HashJoinState>() == 328);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::HashJoinState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::HashJoinState {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashJoinState, hj_OuterHashKeys) == 224
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hj_OuterHashKeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 224;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashJoinState, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinState> as c::expression::OrdinaryField<Field_hj_OuterHashKeys>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hj_OuterHashKeys) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
#[doc(hidden)]
pub struct Field_hj_HashOperators;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashJoinState, hj_HashOperators) == 232
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hj_HashOperators>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 232;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashJoinState, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinState> as c::expression::OrdinaryField<Field_hj_HashOperators>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hj_HashOperators) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
#[doc(hidden)]
pub struct Field_hj_Collations;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HashJoinState, hj_Collations) == 240);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hj_Collations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 240;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashJoinState, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinState> as c::expression::OrdinaryField<Field_hj_Collations>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hj_Collations) };
#[doc(hidden)]
pub struct Field_hj_CurHashValue;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashJoinState, hj_CurHashValue) == 256
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hj_CurHashValue>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinState>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 256;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashJoinState, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinState> as c::expression::OrdinaryField<Field_hj_CurHashValue>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hj_CurHashValue) };
#[doc(hidden)]
pub struct Field_hj_CurBucketNo;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashJoinState, hj_CurBucketNo) == 260
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hj_CurBucketNo>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 260;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashJoinState, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinState> as c::expression::OrdinaryField<Field_hj_CurBucketNo>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hj_CurBucketNo) };
#[doc(hidden)]
pub struct Field_hj_CurSkewBucketNo;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashJoinState, hj_CurSkewBucketNo) == 264
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hj_CurSkewBucketNo>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 264;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashJoinState, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinState> as c::expression::OrdinaryField<Field_hj_CurSkewBucketNo>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hj_CurSkewBucketNo) };
#[doc(hidden)]
pub struct Field_hj_JoinState;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HashJoinState, hj_JoinState) == 320);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hj_JoinState>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 320;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashJoinState, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinState> as c::expression::OrdinaryField<Field_hj_JoinState>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hj_JoinState) };
#[doc(hidden)]
pub struct Field_hj_MatchedOuter;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashJoinState, hj_MatchedOuter) == 324
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hj_MatchedOuter>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 324;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashJoinState, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinState> as c::expression::OrdinaryField<Field_hj_MatchedOuter>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hj_MatchedOuter) };
#[doc(hidden)]
pub struct Field_hj_OuterNotEmpty;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashJoinState, hj_OuterNotEmpty) == 325
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hj_OuterNotEmpty>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 325;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashJoinState, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinState> as c::expression::OrdinaryField<Field_hj_OuterNotEmpty>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hj_OuterNotEmpty) };
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::HashJoinTableData>() == 264);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::HashJoinTableData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::HashJoinTableData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HashJoinTableData, nbuckets) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nbuckets>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinTableData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashJoinTableData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinTableData> as c::expression::OrdinaryField<Field_nbuckets>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nbuckets) };
#[doc(hidden)]
pub struct Field_log2_nbuckets;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashJoinTableData, log2_nbuckets) == 4
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_log2_nbuckets>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinTableData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashJoinTableData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinTableData> as c::expression::OrdinaryField<Field_log2_nbuckets>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).log2_nbuckets) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashJoinTableData, nbuckets_original) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nbuckets_original>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinTableData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashJoinTableData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinTableData> as c::expression::OrdinaryField<Field_nbuckets_original>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nbuckets_original) };
#[doc(hidden)]
pub struct Field_nbuckets_optimal;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashJoinTableData, nbuckets_optimal) == 12
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nbuckets_optimal>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinTableData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashJoinTableData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinTableData> as c::expression::OrdinaryField<Field_nbuckets_optimal>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nbuckets_optimal) };
#[doc(hidden)]
pub struct Field_log2_nbuckets_optimal;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashJoinTableData, log2_nbuckets_optimal)
        == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_log2_nbuckets_optimal>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinTableData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashJoinTableData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinTableData> as c::expression::OrdinaryField<Field_log2_nbuckets_optimal>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).log2_nbuckets_optimal) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
#[doc(hidden)]
pub struct Field_keepNulls;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HashJoinTableData, keepNulls) == 32);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_keepNulls>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinTableData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashJoinTableData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinTableData> as c::expression::OrdinaryField<Field_keepNulls>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).keepNulls) };
#[doc(hidden)]
pub struct Field_skewEnabled;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashJoinTableData, skewEnabled) == 33
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_skewEnabled>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinTableData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 33;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashJoinTableData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinTableData> as c::expression::OrdinaryField<Field_skewEnabled>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).skewEnabled) };
#[doc(hidden)]
pub struct Field_skewBucketLen;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashJoinTableData, skewBucketLen) == 48
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_skewBucketLen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinTableData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashJoinTableData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinTableData> as c::expression::OrdinaryField<Field_skewBucketLen>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).skewBucketLen) };
#[doc(hidden)]
pub struct Field_nSkewBuckets;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashJoinTableData, nSkewBuckets) == 52
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nSkewBuckets>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinTableData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashJoinTableData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinTableData> as c::expression::OrdinaryField<Field_nSkewBuckets>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nSkewBuckets) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HashJoinTableData, nbatch) == 64);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nbatch>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinTableData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashJoinTableData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinTableData> as c::expression::OrdinaryField<Field_nbatch>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nbatch) };
#[doc(hidden)]
pub struct Field_curbatch;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HashJoinTableData, curbatch) == 68);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_curbatch>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinTableData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 68;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashJoinTableData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinTableData> as c::expression::OrdinaryField<Field_curbatch>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).curbatch) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashJoinTableData, nbatch_original) == 72
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nbatch_original>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinTableData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashJoinTableData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinTableData> as c::expression::OrdinaryField<Field_nbatch_original>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nbatch_original) };
#[doc(hidden)]
pub struct Field_nbatch_outstart;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashJoinTableData, nbatch_outstart) == 76
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nbatch_outstart>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinTableData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 76;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashJoinTableData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinTableData> as c::expression::OrdinaryField<Field_nbatch_outstart>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nbatch_outstart) };
#[doc(hidden)]
pub struct Field_growEnabled;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashJoinTableData, growEnabled) == 80
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_growEnabled>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinTableData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashJoinTableData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinTableData> as c::expression::OrdinaryField<Field_growEnabled>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).growEnabled) };
#[doc(hidden)]
pub struct Field_spaceUsed;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HashJoinTableData, spaceUsed) == 160);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_spaceUsed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinTableData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashJoinTableData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinTableData> as c::expression::OrdinaryField<Field_spaceUsed>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).spaceUsed) };
#[doc(hidden)]
pub struct Field_spaceAllowed;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashJoinTableData, spaceAllowed) == 168
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_spaceAllowed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinTableData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 168;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashJoinTableData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinTableData> as c::expression::OrdinaryField<Field_spaceAllowed>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).spaceAllowed) };
#[doc(hidden)]
pub struct Field_spacePeak;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HashJoinTableData, spacePeak) == 176);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_spacePeak>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinTableData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 176;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashJoinTableData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinTableData> as c::expression::OrdinaryField<Field_spacePeak>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).spacePeak) };
#[doc(hidden)]
pub struct Field_spaceUsedSkew;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashJoinTableData, spaceUsedSkew) == 184
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_spaceUsedSkew>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinTableData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 184;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashJoinTableData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinTableData> as c::expression::OrdinaryField<Field_spaceUsedSkew>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).spaceUsedSkew) };
#[doc(hidden)]
pub struct Field_spaceAllowedSkew;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashJoinTableData, spaceAllowedSkew) == 192
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_spaceAllowedSkew>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinTableData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 192;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashJoinTableData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinTableData> as c::expression::OrdinaryField<Field_spaceAllowedSkew>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).spaceAllowedSkew) };
#[doc(hidden)]
pub struct Field_parallel_state;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashJoinTableData, parallel_state) == 240
);
const _: () = {
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::ParallelHashJoinState>() == 8);
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::ParallelHashJoinState>() == 8);
};
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parallel_state>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinTableData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinState>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 240;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashJoinTableData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinTableData> as c::expression::OrdinaryField<Field_parallel_state>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).parallel_state) };
#[doc(hidden)]
pub struct Field_current_chunk_shared;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashJoinTableData, current_chunk_shared)
        == 256
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_current_chunk_shared>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinTableData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 256;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashJoinTableData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinTableData> as c::expression::OrdinaryField<Field_current_chunk_shared>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).current_chunk_shared) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::HashJoinTupleData>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::HashJoinTupleData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::HashJoinTupleData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HashJoinTupleData, hashvalue) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hashvalue>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinTupleData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashJoinTupleData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashJoinTupleData> as c::expression::OrdinaryField<Field_hashvalue>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hashvalue) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::HashMemoryChunkData>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::HashMemoryChunkData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::HashMemoryChunkData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HashMemoryChunkData, ntuples) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ntuples>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashMemoryChunkData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashMemoryChunkData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashMemoryChunkData> as c::expression::OrdinaryField<Field_ntuples>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ntuples) };
#[doc(hidden)]
pub struct Field_maxlen;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HashMemoryChunkData, maxlen) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_maxlen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashMemoryChunkData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashMemoryChunkData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashMemoryChunkData> as c::expression::OrdinaryField<Field_maxlen>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).maxlen) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HashMemoryChunkData, used) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_used>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashMemoryChunkData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashMemoryChunkData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashMemoryChunkData> as c::expression::OrdinaryField<Field_used>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).used) };
#[doc(hidden)]
pub struct Field_hashm_magic;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::HashMetaPageData>() == 4544);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::HashMetaPageData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::HashMetaPageData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HashMetaPageData, hashm_magic) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hashm_magic>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashMetaPageData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashMetaPageData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashMetaPageData> as c::expression::OrdinaryField<Field_hashm_magic>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hashm_magic) };
#[doc(hidden)]
pub struct Field_hashm_version;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashMetaPageData, hashm_version) == 4
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hashm_version>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashMetaPageData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashMetaPageData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashMetaPageData> as c::expression::OrdinaryField<Field_hashm_version>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hashm_version) };
#[doc(hidden)]
pub struct Field_hashm_ffactor;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashMetaPageData, hashm_ffactor) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hashm_ffactor>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashMetaPageData>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashMetaPageData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashMetaPageData> as c::expression::OrdinaryField<Field_hashm_ffactor>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hashm_ffactor) };
#[doc(hidden)]
pub struct Field_hashm_bsize;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HashMetaPageData, hashm_bsize) == 18);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hashm_bsize>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashMetaPageData>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 18;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashMetaPageData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashMetaPageData> as c::expression::OrdinaryField<Field_hashm_bsize>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hashm_bsize) };
#[doc(hidden)]
pub struct Field_hashm_bmsize;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashMetaPageData, hashm_bmsize) == 20
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hashm_bmsize>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashMetaPageData>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashMetaPageData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashMetaPageData> as c::expression::OrdinaryField<Field_hashm_bmsize>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hashm_bmsize) };
#[doc(hidden)]
pub struct Field_hashm_bmshift;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashMetaPageData, hashm_bmshift) == 22
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hashm_bmshift>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashMetaPageData>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 22;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashMetaPageData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashMetaPageData> as c::expression::OrdinaryField<Field_hashm_bmshift>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hashm_bmshift) };
#[doc(hidden)]
pub struct Field_hashm_maxbucket;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashMetaPageData, hashm_maxbucket) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hashm_maxbucket>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashMetaPageData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashMetaPageData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashMetaPageData> as c::expression::OrdinaryField<Field_hashm_maxbucket>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hashm_maxbucket) };
#[doc(hidden)]
pub struct Field_hashm_highmask;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashMetaPageData, hashm_highmask) == 28
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hashm_highmask>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashMetaPageData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashMetaPageData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashMetaPageData> as c::expression::OrdinaryField<Field_hashm_highmask>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hashm_highmask) };
#[doc(hidden)]
pub struct Field_hashm_lowmask;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashMetaPageData, hashm_lowmask) == 32
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hashm_lowmask>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashMetaPageData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashMetaPageData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashMetaPageData> as c::expression::OrdinaryField<Field_hashm_lowmask>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hashm_lowmask) };
#[doc(hidden)]
pub struct Field_hashm_ovflpoint;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashMetaPageData, hashm_ovflpoint) == 36
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hashm_ovflpoint>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashMetaPageData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashMetaPageData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashMetaPageData> as c::expression::OrdinaryField<Field_hashm_ovflpoint>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hashm_ovflpoint) };
#[doc(hidden)]
pub struct Field_hashm_firstfree;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashMetaPageData, hashm_firstfree) == 40
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hashm_firstfree>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashMetaPageData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashMetaPageData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashMetaPageData> as c::expression::OrdinaryField<Field_hashm_firstfree>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hashm_firstfree) };
#[doc(hidden)]
pub struct Field_hashm_nmaps;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HashMetaPageData, hashm_nmaps) == 44);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hashm_nmaps>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashMetaPageData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashMetaPageData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashMetaPageData> as c::expression::OrdinaryField<Field_hashm_nmaps>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hashm_nmaps) };
#[doc(hidden)]
pub struct Field_hashm_procid;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashMetaPageData, hashm_procid) == 48
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hashm_procid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashMetaPageData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashMetaPageData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashMetaPageData> as c::expression::OrdinaryField<Field_hashm_procid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hashm_procid) };
#[doc(hidden)]
pub struct Field_hashm_spares;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashMetaPageData, hashm_spares) == 52
);
const _: () = {
    assert!(::core::mem::size_of::<[u32; 98]>() == 392);
    assert!(::core::mem::align_of::<[u32; 98]>() == 4);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hashm_spares>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashMetaPageData>
{
    type Member = c::expression::CArray<c::CUnsignedInt, 98>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashMetaPageData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashMetaPageData> as c::expression::OrdinaryField<Field_hashm_spares>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hashm_spares) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::HashOptions>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::HashOptions>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::HashOptions {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HashOptions, varlena_header_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_varlena_header_>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashOptions>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashOptions> as c::expression::OrdinaryField<Field_varlena_header_>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).varlena_header_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HashOptions, fillfactor) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fillfactor>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashOptions>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashOptions> as c::expression::OrdinaryField<Field_fillfactor>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fillfactor) };
#[doc(hidden)]
pub struct Field_hasho_prevblkno;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::HashPageOpaqueData>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::HashPageOpaqueData>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::HashPageOpaqueData {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashPageOpaqueData, hasho_prevblkno) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasho_prevblkno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashPageOpaqueData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashPageOpaqueData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashPageOpaqueData> as c::expression::OrdinaryField<Field_hasho_prevblkno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hasho_prevblkno) };
#[doc(hidden)]
pub struct Field_hasho_nextblkno;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashPageOpaqueData, hasho_nextblkno) == 4
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasho_nextblkno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashPageOpaqueData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashPageOpaqueData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashPageOpaqueData> as c::expression::OrdinaryField<Field_hasho_nextblkno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hasho_nextblkno) };
#[doc(hidden)]
pub struct Field_hasho_bucket;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashPageOpaqueData, hasho_bucket) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasho_bucket>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashPageOpaqueData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashPageOpaqueData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashPageOpaqueData> as c::expression::OrdinaryField<Field_hasho_bucket>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hasho_bucket) };
#[doc(hidden)]
pub struct Field_hasho_flag;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashPageOpaqueData, hasho_flag) == 12
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasho_flag>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashPageOpaqueData>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashPageOpaqueData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashPageOpaqueData> as c::expression::OrdinaryField<Field_hasho_flag>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hasho_flag) };
#[doc(hidden)]
pub struct Field_hasho_page_id;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashPageOpaqueData, hasho_page_id) == 14
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasho_page_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashPageOpaqueData>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 14;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashPageOpaqueData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashPageOpaqueData> as c::expression::OrdinaryField<Field_hasho_page_id>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hasho_page_id) };
#[doc(hidden)]
pub struct Field_path_hashclauses;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::HashPath>() == 128);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::HashPath>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::HashPath {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HashPath, path_hashclauses) == 104);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_path_hashclauses>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashPath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::HashPath,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::HashPath> as c::expression::OrdinaryField<
        Field_path_hashclauses,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).path_hashclauses) };
#[doc(hidden)]
pub struct Field_num_batches;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HashPath, num_batches) == 112);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_batches>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashPath>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::HashPath,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::HashPath> as c::expression::OrdinaryField<
        Field_num_batches,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_batches) };
#[doc(hidden)]
pub struct Field_hashso_sk_hash;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::HashScanOpaqueData>() == 3320);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::HashScanOpaqueData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::HashScanOpaqueData {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashScanOpaqueData, hashso_sk_hash) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hashso_sk_hash>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashScanOpaqueData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashScanOpaqueData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashScanOpaqueData> as c::expression::OrdinaryField<Field_hashso_sk_hash>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hashso_sk_hash) };
#[doc(hidden)]
pub struct Field_hashso_bucket_buf;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashScanOpaqueData, hashso_bucket_buf) == 4
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hashso_bucket_buf>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashScanOpaqueData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashScanOpaqueData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashScanOpaqueData> as c::expression::OrdinaryField<Field_hashso_bucket_buf>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hashso_bucket_buf) };
#[doc(hidden)]
pub struct Field_hashso_split_bucket_buf;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashScanOpaqueData, hashso_split_bucket_buf)
        == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hashso_split_bucket_buf>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashScanOpaqueData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashScanOpaqueData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashScanOpaqueData> as c::expression::OrdinaryField<Field_hashso_split_bucket_buf>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hashso_split_bucket_buf) };
#[doc(hidden)]
pub struct Field_hashso_buc_populated;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashScanOpaqueData, hashso_buc_populated)
        == 12
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hashso_buc_populated>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashScanOpaqueData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashScanOpaqueData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashScanOpaqueData> as c::expression::OrdinaryField<Field_hashso_buc_populated>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hashso_buc_populated) };
#[doc(hidden)]
pub struct Field_hashso_buc_split;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HashScanOpaqueData, hashso_buc_split) == 13
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hashso_buc_split>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashScanOpaqueData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 13;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashScanOpaqueData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashScanOpaqueData> as c::expression::OrdinaryField<Field_hashso_buc_split>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hashso_buc_split) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HashScanOpaqueData, numKilled) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numKilled>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashScanOpaqueData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashScanOpaqueData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashScanOpaqueData> as c::expression::OrdinaryField<Field_numKilled>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numKilled) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::HashScanPosData>() == 3292);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::HashScanPosData>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::HashScanPosData {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HashScanPosData, buf) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_buf>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashScanPosData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashScanPosData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashScanPosData> as c::expression::OrdinaryField<Field_buf>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).buf) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HashScanPosData, currPage) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_currPage>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashScanPosData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashScanPosData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashScanPosData> as c::expression::OrdinaryField<Field_currPage>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).currPage) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HashScanPosData, nextPage) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nextPage>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashScanPosData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashScanPosData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashScanPosData> as c::expression::OrdinaryField<Field_nextPage>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nextPage) };
#[doc(hidden)]
pub struct Field_prevPage;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HashScanPosData, prevPage) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_prevPage>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashScanPosData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashScanPosData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashScanPosData> as c::expression::OrdinaryField<Field_prevPage>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).prevPage) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HashScanPosData, firstItem) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_firstItem>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashScanPosData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashScanPosData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashScanPosData> as c::expression::OrdinaryField<Field_firstItem>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).firstItem) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HashScanPosData, lastItem) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lastItem>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashScanPosData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashScanPosData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashScanPosData> as c::expression::OrdinaryField<Field_lastItem>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lastItem) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HashScanPosData, itemIndex) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_itemIndex>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashScanPosData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashScanPosData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashScanPosData> as c::expression::OrdinaryField<Field_itemIndex>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).itemIndex) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::HashScanPosItem>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::HashScanPosItem>() == 2);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::HashScanPosItem {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HashScanPosItem, indexOffset) == 6);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indexOffset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashScanPosItem>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 6;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashScanPosItem, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashScanPosItem> as c::expression::OrdinaryField<Field_indexOffset>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indexOffset) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::HashSkewBucket>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::HashSkewBucket>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::HashSkewBucket {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HashSkewBucket, hashvalue) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hashvalue>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashSkewBucket>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HashSkewBucket, <<c::expression::CRecord<crate::__pgrx_c_bindings::HashSkewBucket> as c::expression::OrdinaryField<Field_hashvalue>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hashvalue) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::HashState>() == 240);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::HashState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::HashState {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HashState, parallel_state) == 232);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parallel_state>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HashState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinState>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 232;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::HashState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::HashState> as c::expression::OrdinaryField<
        Field_parallel_state,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).parallel_state) };
#[doc(hidden)]
pub struct Field_rs_nblocks;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::HeapScanDescData>() == 712);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::HeapScanDescData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::HeapScanDescData {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapScanDescData, rs_nblocks) == 56);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rs_nblocks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HeapScanDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData> as c::expression::OrdinaryField<Field_rs_nblocks>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rs_nblocks) };
#[doc(hidden)]
pub struct Field_rs_startblock;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapScanDescData, rs_startblock) == 60
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rs_startblock>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 60;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HeapScanDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData> as c::expression::OrdinaryField<Field_rs_startblock>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rs_startblock) };
#[doc(hidden)]
pub struct Field_rs_numblocks;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapScanDescData, rs_numblocks) == 64
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rs_numblocks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HeapScanDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData> as c::expression::OrdinaryField<Field_rs_numblocks>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rs_numblocks) };
#[doc(hidden)]
pub struct Field_rs_inited;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapScanDescData, rs_inited) == 68);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rs_inited>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 68;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HeapScanDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData> as c::expression::OrdinaryField<Field_rs_inited>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rs_inited) };
#[doc(hidden)]
pub struct Field_rs_cblock;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapScanDescData, rs_cblock) == 72);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rs_cblock>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HeapScanDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData> as c::expression::OrdinaryField<Field_rs_cblock>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rs_cblock) };
#[doc(hidden)]
pub struct Field_rs_cbuf;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapScanDescData, rs_cbuf) == 76);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rs_cbuf>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 76;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HeapScanDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData> as c::expression::OrdinaryField<Field_rs_cbuf>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rs_cbuf) };
#[doc(hidden)]
pub struct Field_rs_cindex;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapScanDescData, rs_cindex) == 120);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rs_cindex>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HeapScanDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData> as c::expression::OrdinaryField<Field_rs_cindex>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rs_cindex) };
#[doc(hidden)]
pub struct Field_rs_ntuples;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapScanDescData, rs_ntuples) == 124);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rs_ntuples>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 124;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HeapScanDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData> as c::expression::OrdinaryField<Field_rs_ntuples>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rs_ntuples) };
#[doc(hidden)]
pub struct Field_t_len;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::HeapTupleData>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::HeapTupleData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::HeapTupleData {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapTupleData, t_len) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_t_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapTupleData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HeapTupleData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HeapTupleData> as c::expression::OrdinaryField<Field_t_len>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).t_len) };
#[doc(hidden)]
pub struct Field_t_tableOid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapTupleData, t_tableOid) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_t_tableOid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapTupleData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HeapTupleData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HeapTupleData> as c::expression::OrdinaryField<Field_t_tableOid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).t_tableOid) };
#[doc(hidden)]
pub struct Field_t_data;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapTupleData, t_data) == 16);
const _: () = {
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::HeapTupleHeaderData>() == 8);
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::HeapTupleHeaderData>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_t_data>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapTupleData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::HeapTupleHeaderData>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HeapTupleData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HeapTupleData> as c::expression::OrdinaryField<Field_t_data>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).t_data) };
#[doc(hidden)]
pub struct Field_t_xmin;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::HeapTupleFields>() == 12);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::HeapTupleFields>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::HeapTupleFields {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapTupleFields, t_xmin) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_t_xmin>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapTupleFields>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HeapTupleFields, <<c::expression::CRecord<crate::__pgrx_c_bindings::HeapTupleFields> as c::expression::OrdinaryField<Field_t_xmin>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).t_xmin) };
#[doc(hidden)]
pub struct Field_t_xmax;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapTupleFields, t_xmax) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_t_xmax>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapTupleFields>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HeapTupleFields, <<c::expression::CRecord<crate::__pgrx_c_bindings::HeapTupleFields> as c::expression::OrdinaryField<Field_t_xmax>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).t_xmax) };
#[doc(hidden)]
pub struct Field_t_field3;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapTupleFields, t_field3) == 8);
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::HeapTupleFields__bindgen_ty_1>() == 4);
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::HeapTupleFields__bindgen_ty_1>() == 4
    );
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_t_field3>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapTupleFields>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::HeapTupleFields__bindgen_ty_1>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HeapTupleFields, <<c::expression::CRecord<crate::__pgrx_c_bindings::HeapTupleFields> as c::expression::OrdinaryField<Field_t_field3>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).t_field3) };
#[doc(hidden)]
pub struct Field_t_choice;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::HeapTupleHeaderData>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::HeapTupleHeaderData>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::HeapTupleHeaderData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapTupleHeaderData, t_choice) == 0);
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::HeapTupleHeaderData__bindgen_ty_1>() == 12
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::HeapTupleHeaderData__bindgen_ty_1>() == 4
    );
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_t_choice>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapTupleHeaderData>
{
    type Member =
        c::expression::CRecord<crate::__pgrx_c_bindings::HeapTupleHeaderData__bindgen_ty_1>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HeapTupleHeaderData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HeapTupleHeaderData> as c::expression::OrdinaryField<Field_t_choice>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).t_choice) };
#[doc(hidden)]
pub struct Field_t_ctid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapTupleHeaderData, t_ctid) == 12);
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ItemPointerData>() == 6);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ItemPointerData>() == 2);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_t_ctid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapTupleHeaderData>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::ItemPointerData>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HeapTupleHeaderData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HeapTupleHeaderData> as c::expression::OrdinaryField<Field_t_ctid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).t_ctid) };
#[doc(hidden)]
pub struct Field_t_infomask2;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapTupleHeaderData, t_infomask2) == 18
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_t_infomask2>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapTupleHeaderData>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 18;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HeapTupleHeaderData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HeapTupleHeaderData> as c::expression::OrdinaryField<Field_t_infomask2>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).t_infomask2) };
#[doc(hidden)]
pub struct Field_t_infomask;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapTupleHeaderData, t_infomask) == 20
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_t_infomask>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapTupleHeaderData>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HeapTupleHeaderData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HeapTupleHeaderData> as c::expression::OrdinaryField<Field_t_infomask>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).t_infomask) };
#[doc(hidden)]
pub struct Field_t_hoff;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapTupleHeaderData, t_hoff) == 22);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_t_hoff>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapTupleHeaderData>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 22;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HeapTupleHeaderData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HeapTupleHeaderData> as c::expression::OrdinaryField<Field_t_hoff>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).t_hoff) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::HeapTupleTableSlot>() == 104);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::HeapTupleTableSlot>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::HeapTupleTableSlot {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapTupleTableSlot, off) == 72);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_off>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapTupleTableSlot>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HeapTupleTableSlot, <<c::expression::CRecord<crate::__pgrx_c_bindings::HeapTupleTableSlot> as c::expression::OrdinaryField<Field_off>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).off) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::IdentifySystemCmd>() == 4);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::IdentifySystemCmd>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::IdentifySystemCmd {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IdentifySystemCmd, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IdentifySystemCmd>
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
const _: Projection<crate::__pgrx_c_bindings::IdentifySystemCmd, <<c::expression::CRecord<crate::__pgrx_c_bindings::IdentifySystemCmd> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ImportForeignSchemaStmt>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ImportForeignSchemaStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ImportForeignSchemaStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ImportForeignSchemaStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ImportForeignSchemaStmt>
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
const _: Projection<crate::__pgrx_c_bindings::ImportForeignSchemaStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::ImportForeignSchemaStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_list_type;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ImportForeignSchemaStmt, list_type) == 32
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_list_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ImportForeignSchemaStmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ImportForeignSchemaType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ImportForeignSchemaStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::ImportForeignSchemaStmt> as c::expression::OrdinaryField<Field_list_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).list_type) };
#[doc(hidden)]
pub struct Field_table_list;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ImportForeignSchemaStmt, table_list) == 40
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_table_list>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ImportForeignSchemaStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ImportForeignSchemaStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::ImportForeignSchemaStmt> as c::expression::OrdinaryField<Field_table_list>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).table_list) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ImportForeignSchemaStmt, options) == 48
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ImportForeignSchemaStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ImportForeignSchemaStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::ImportForeignSchemaStmt> as c::expression::OrdinaryField<Field_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
#[doc(hidden)]
pub struct Field_nPresortedCols;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::IncrementalSort>() == 152);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::IncrementalSort>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::IncrementalSort {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IncrementalSort, nPresortedCols) == 144
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nPresortedCols>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IncrementalSort>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IncrementalSort, <<c::expression::CRecord<crate::__pgrx_c_bindings::IncrementalSort> as c::expression::OrdinaryField<Field_nPresortedCols>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nPresortedCols) };
#[doc(hidden)]
pub struct Field_groupCount;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::IncrementalSortGroupInfo>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::IncrementalSortGroupInfo>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::IncrementalSortGroupInfo {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IncrementalSortGroupInfo, groupCount) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_groupCount>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IncrementalSortGroupInfo>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IncrementalSortGroupInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::IncrementalSortGroupInfo> as c::expression::OrdinaryField<Field_groupCount>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).groupCount) };
#[doc(hidden)]
pub struct Field_maxDiskSpaceUsed;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IncrementalSortGroupInfo, maxDiskSpaceUsed)
        == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_maxDiskSpaceUsed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IncrementalSortGroupInfo>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IncrementalSortGroupInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::IncrementalSortGroupInfo> as c::expression::OrdinaryField<Field_maxDiskSpaceUsed>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).maxDiskSpaceUsed) };
#[doc(hidden)]
pub struct Field_totalDiskSpaceUsed;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IncrementalSortGroupInfo, totalDiskSpaceUsed)
        == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_totalDiskSpaceUsed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IncrementalSortGroupInfo>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IncrementalSortGroupInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::IncrementalSortGroupInfo> as c::expression::OrdinaryField<Field_totalDiskSpaceUsed>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).totalDiskSpaceUsed) };
#[doc(hidden)]
pub struct Field_maxMemorySpaceUsed;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IncrementalSortGroupInfo, maxMemorySpaceUsed)
        == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_maxMemorySpaceUsed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IncrementalSortGroupInfo>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IncrementalSortGroupInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::IncrementalSortGroupInfo> as c::expression::OrdinaryField<Field_maxMemorySpaceUsed>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).maxMemorySpaceUsed) };
#[doc(hidden)]
pub struct Field_totalMemorySpaceUsed;
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::IncrementalSortGroupInfo,
        totalMemorySpaceUsed
    ) == 32
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_totalMemorySpaceUsed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IncrementalSortGroupInfo>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IncrementalSortGroupInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::IncrementalSortGroupInfo> as c::expression::OrdinaryField<Field_totalMemorySpaceUsed>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).totalMemorySpaceUsed) };
#[doc(hidden)]
pub struct Field_sortMethods;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IncrementalSortGroupInfo, sortMethods) == 40
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sortMethods>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IncrementalSortGroupInfo>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IncrementalSortGroupInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::IncrementalSortGroupInfo> as c::expression::OrdinaryField<Field_sortMethods>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sortMethods) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::IncrementalSortPath>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::IncrementalSortPath>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::IncrementalSortPath {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IncrementalSortPath, nPresortedCols) == 80
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nPresortedCols>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IncrementalSortPath>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IncrementalSortPath, <<c::expression::CRecord<crate::__pgrx_c_bindings::IncrementalSortPath> as c::expression::OrdinaryField<Field_nPresortedCols>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nPresortedCols) };
#[doc(hidden)]
pub struct Field_bounded;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::IncrementalSortState>() == 424);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::IncrementalSortState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::IncrementalSortState {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IncrementalSortState, bounded) == 224
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bounded>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IncrementalSortState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 224;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IncrementalSortState, <<c::expression::CRecord<crate::__pgrx_c_bindings::IncrementalSortState> as c::expression::OrdinaryField<Field_bounded>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).bounded) };
#[doc(hidden)]
pub struct Field_bound;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IncrementalSortState, bound) == 232);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bound>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IncrementalSortState>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 232;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IncrementalSortState, <<c::expression::CRecord<crate::__pgrx_c_bindings::IncrementalSortState> as c::expression::OrdinaryField<Field_bound>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).bound) };
#[doc(hidden)]
pub struct Field_outerNodeDone;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IncrementalSortState, outerNodeDone) == 240
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_outerNodeDone>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IncrementalSortState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 240;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IncrementalSortState, <<c::expression::CRecord<crate::__pgrx_c_bindings::IncrementalSortState> as c::expression::OrdinaryField<Field_outerNodeDone>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).outerNodeDone) };
#[doc(hidden)]
pub struct Field_bound_Done;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IncrementalSortState, bound_Done) == 248
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bound_Done>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IncrementalSortState>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 248;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IncrementalSortState, <<c::expression::CRecord<crate::__pgrx_c_bindings::IncrementalSortState> as c::expression::OrdinaryField<Field_bound_Done>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).bound_Done) };
#[doc(hidden)]
pub struct Field_execution_status;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IncrementalSortState, execution_status)
        == 256
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_execution_status>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IncrementalSortState>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_IncrementalSortExecutionStatus,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 256;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IncrementalSortState, <<c::expression::CRecord<crate::__pgrx_c_bindings::IncrementalSortState> as c::expression::OrdinaryField<Field_execution_status>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).execution_status) };
#[doc(hidden)]
pub struct Field_n_fullsort_remaining;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IncrementalSortState, n_fullsort_remaining)
        == 264
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_n_fullsort_remaining>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IncrementalSortState>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 264;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IncrementalSortState, <<c::expression::CRecord<crate::__pgrx_c_bindings::IncrementalSortState> as c::expression::OrdinaryField<Field_n_fullsort_remaining>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).n_fullsort_remaining) };
#[doc(hidden)]
pub struct Field_am_worker;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IncrementalSortState, am_worker) == 408
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_am_worker>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IncrementalSortState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 408;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IncrementalSortState, <<c::expression::CRecord<crate::__pgrx_c_bindings::IncrementalSortState> as c::expression::OrdinaryField<Field_am_worker>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).am_worker) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::IndexAmRoutine>() == 208);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::IndexAmRoutine>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::IndexAmRoutine {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexAmRoutine, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine>
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
const _: Projection<crate::__pgrx_c_bindings::IndexAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_amstrategies;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexAmRoutine, amstrategies) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amstrategies>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine> as c::expression::OrdinaryField<Field_amstrategies>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).amstrategies) };
#[doc(hidden)]
pub struct Field_amsupport;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexAmRoutine, amsupport) == 6);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amsupport>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 6;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine> as c::expression::OrdinaryField<Field_amsupport>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).amsupport) };
#[doc(hidden)]
pub struct Field_amoptsprocnum;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexAmRoutine, amoptsprocnum) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amoptsprocnum>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine> as c::expression::OrdinaryField<Field_amoptsprocnum>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).amoptsprocnum) };
#[doc(hidden)]
pub struct Field_amcanorder;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexAmRoutine, amcanorder) == 10);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amcanorder>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 10;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine> as c::expression::OrdinaryField<Field_amcanorder>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).amcanorder) };
#[doc(hidden)]
pub struct Field_amcanorderbyop;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexAmRoutine, amcanorderbyop) == 11
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amcanorderbyop>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 11;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine> as c::expression::OrdinaryField<Field_amcanorderbyop>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).amcanorderbyop) };
#[doc(hidden)]
pub struct Field_amcanbackward;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexAmRoutine, amcanbackward) == 12);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amcanbackward>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine> as c::expression::OrdinaryField<Field_amcanbackward>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).amcanbackward) };
#[doc(hidden)]
pub struct Field_amcanunique;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexAmRoutine, amcanunique) == 13);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amcanunique>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 13;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine> as c::expression::OrdinaryField<Field_amcanunique>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).amcanunique) };
#[doc(hidden)]
pub struct Field_amcanmulticol;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexAmRoutine, amcanmulticol) == 14);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amcanmulticol>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 14;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine> as c::expression::OrdinaryField<Field_amcanmulticol>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).amcanmulticol) };
#[doc(hidden)]
pub struct Field_amoptionalkey;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexAmRoutine, amoptionalkey) == 15);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amoptionalkey>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 15;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine> as c::expression::OrdinaryField<Field_amoptionalkey>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).amoptionalkey) };
#[doc(hidden)]
pub struct Field_amsearcharray;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexAmRoutine, amsearcharray) == 16);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amsearcharray>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine> as c::expression::OrdinaryField<Field_amsearcharray>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).amsearcharray) };
#[doc(hidden)]
pub struct Field_amsearchnulls;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexAmRoutine, amsearchnulls) == 17);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amsearchnulls>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 17;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine> as c::expression::OrdinaryField<Field_amsearchnulls>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).amsearchnulls) };
#[doc(hidden)]
pub struct Field_amstorage;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexAmRoutine, amstorage) == 18);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amstorage>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 18;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine> as c::expression::OrdinaryField<Field_amstorage>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).amstorage) };
#[doc(hidden)]
pub struct Field_amclusterable;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexAmRoutine, amclusterable) == 19);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amclusterable>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 19;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine> as c::expression::OrdinaryField<Field_amclusterable>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).amclusterable) };
#[doc(hidden)]
pub struct Field_ampredlocks;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexAmRoutine, ampredlocks) == 20);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ampredlocks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine> as c::expression::OrdinaryField<Field_ampredlocks>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ampredlocks) };
#[doc(hidden)]
pub struct Field_amcanparallel;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexAmRoutine, amcanparallel) == 21);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amcanparallel>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 21;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine> as c::expression::OrdinaryField<Field_amcanparallel>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).amcanparallel) };
#[doc(hidden)]
pub struct Field_amcaninclude;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexAmRoutine, amcaninclude) == 22);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amcaninclude>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 22;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine> as c::expression::OrdinaryField<Field_amcaninclude>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).amcaninclude) };
#[doc(hidden)]
pub struct Field_amusemaintenanceworkmem;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexAmRoutine, amusemaintenanceworkmem)
        == 23
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amusemaintenanceworkmem>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 23;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine> as c::expression::OrdinaryField<Field_amusemaintenanceworkmem>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).amusemaintenanceworkmem) };
#[doc(hidden)]
pub struct Field_amparallelvacuumoptions;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexAmRoutine, amparallelvacuumoptions)
        == 24
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amparallelvacuumoptions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine> as c::expression::OrdinaryField<Field_amparallelvacuumoptions>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).amparallelvacuumoptions) };
#[doc(hidden)]
pub struct Field_amkeytype;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexAmRoutine, amkeytype) == 28);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amkeytype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine> as c::expression::OrdinaryField<Field_amkeytype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).amkeytype) };
#[doc(hidden)]
pub struct Field_num_pages;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::IndexBulkDeleteResult>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::IndexBulkDeleteResult>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::IndexBulkDeleteResult {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexBulkDeleteResult, num_pages) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_pages>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexBulkDeleteResult>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexBulkDeleteResult, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexBulkDeleteResult> as c::expression::OrdinaryField<Field_num_pages>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_pages) };
#[doc(hidden)]
pub struct Field_estimated_count;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexBulkDeleteResult, estimated_count) == 4
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_estimated_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexBulkDeleteResult>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexBulkDeleteResult, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexBulkDeleteResult> as c::expression::OrdinaryField<Field_estimated_count>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).estimated_count) };
#[doc(hidden)]
pub struct Field_pages_newly_deleted;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexBulkDeleteResult, pages_newly_deleted)
        == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pages_newly_deleted>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexBulkDeleteResult>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexBulkDeleteResult, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexBulkDeleteResult> as c::expression::OrdinaryField<Field_pages_newly_deleted>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pages_newly_deleted) };
#[doc(hidden)]
pub struct Field_pages_deleted;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexBulkDeleteResult, pages_deleted) == 28
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pages_deleted>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexBulkDeleteResult>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexBulkDeleteResult, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexBulkDeleteResult> as c::expression::OrdinaryField<Field_pages_deleted>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pages_deleted) };
#[doc(hidden)]
pub struct Field_pages_free;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexBulkDeleteResult, pages_free) == 32
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pages_free>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexBulkDeleteResult>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexBulkDeleteResult, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexBulkDeleteResult> as c::expression::OrdinaryField<Field_pages_free>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pages_free) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::IndexClause>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::IndexClause>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::IndexClause {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexClause, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexClause>
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
const _: Projection<crate::__pgrx_c_bindings::IndexClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexClause> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_indexquals;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexClause, indexquals) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indexquals>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexClause>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexClause> as c::expression::OrdinaryField<Field_indexquals>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indexquals) };
#[doc(hidden)]
pub struct Field_lossy;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexClause, lossy) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lossy>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexClause>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexClause> as c::expression::OrdinaryField<Field_lossy>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lossy) };
#[doc(hidden)]
pub struct Field_indexcol;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexClause, indexcol) == 26);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indexcol>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexClause>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 26;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexClause> as c::expression::OrdinaryField<Field_indexcol>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indexcol) };
#[doc(hidden)]
pub struct Field_indexcols;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexClause, indexcols) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indexcols>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexClause>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexClause> as c::expression::OrdinaryField<Field_indexcols>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indexcols) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::IndexElem>() == 64);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::IndexElem>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::IndexElem {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexElem, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexElem>
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
    crate::__pgrx_c_bindings::IndexElem,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexElem> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexElem, collation) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_collation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexElem>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexElem,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexElem> as c::expression::OrdinaryField<
        Field_collation,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).collation) };
#[doc(hidden)]
pub struct Field_opclass;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexElem, opclass) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_opclass>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexElem>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexElem,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexElem> as c::expression::OrdinaryField<
        Field_opclass,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).opclass) };
#[doc(hidden)]
pub struct Field_opclassopts;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexElem, opclassopts) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_opclassopts>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexElem>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexElem,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexElem> as c::expression::OrdinaryField<
        Field_opclassopts,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).opclassopts) };
#[doc(hidden)]
pub struct Field_ordering;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexElem, ordering) == 56);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ordering>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexElem>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__SortByDir,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexElem,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexElem> as c::expression::OrdinaryField<
        Field_ordering,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ordering) };
#[doc(hidden)]
pub struct Field_nulls_ordering;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexElem, nulls_ordering) == 60);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nulls_ordering>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexElem>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__SortByNulls,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 60;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexElem,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexElem> as c::expression::OrdinaryField<
        Field_nulls_ordering,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nulls_ordering) };
#[doc(hidden)]
pub struct Field_xs_cbuf;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::IndexFetchHeapData>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::IndexFetchHeapData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::IndexFetchHeapData {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexFetchHeapData, xs_cbuf) == 8);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xs_cbuf>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexFetchHeapData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexFetchHeapData, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexFetchHeapData> as c::expression::OrdinaryField<Field_xs_cbuf>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xs_cbuf) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::IndexFetchTableData>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::IndexFetchTableData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::IndexFetchTableData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexFetchTableData, rel) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rel>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexFetchTableData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexFetchTableData, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexFetchTableData> as c::expression::OrdinaryField<Field_rel>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rel) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::IndexInfo>() == 200);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::IndexInfo>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::IndexInfo {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexInfo, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexInfo>
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
    crate::__pgrx_c_bindings::IndexInfo,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexInfo> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_ii_NumIndexAttrs;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexInfo, ii_NumIndexAttrs) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ii_NumIndexAttrs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexInfo,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexInfo> as c::expression::OrdinaryField<
        Field_ii_NumIndexAttrs,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ii_NumIndexAttrs) };
#[doc(hidden)]
pub struct Field_ii_NumIndexKeyAttrs;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexInfo, ii_NumIndexKeyAttrs) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ii_NumIndexKeyAttrs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexInfo,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexInfo> as c::expression::OrdinaryField<
        Field_ii_NumIndexKeyAttrs,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ii_NumIndexKeyAttrs) };
#[doc(hidden)]
pub struct Field_ii_Expressions;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexInfo, ii_Expressions) == 80);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ii_Expressions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexInfo,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexInfo> as c::expression::OrdinaryField<
        Field_ii_Expressions,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ii_Expressions) };
#[doc(hidden)]
pub struct Field_ii_ExpressionsState;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexInfo, ii_ExpressionsState) == 88
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ii_ExpressionsState>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexInfo,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexInfo> as c::expression::OrdinaryField<
        Field_ii_ExpressionsState,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ii_ExpressionsState) };
#[doc(hidden)]
pub struct Field_ii_Predicate;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexInfo, ii_Predicate) == 96);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ii_Predicate>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexInfo,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexInfo> as c::expression::OrdinaryField<
        Field_ii_Predicate,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ii_Predicate) };
#[doc(hidden)]
pub struct Field_ii_Unique;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexInfo, ii_Unique) == 168);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ii_Unique>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 168;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexInfo,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexInfo> as c::expression::OrdinaryField<
        Field_ii_Unique,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ii_Unique) };
#[doc(hidden)]
pub struct Field_ii_NullsNotDistinct;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexInfo, ii_NullsNotDistinct) == 169
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ii_NullsNotDistinct>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 169;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexInfo,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexInfo> as c::expression::OrdinaryField<
        Field_ii_NullsNotDistinct,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ii_NullsNotDistinct) };
#[doc(hidden)]
pub struct Field_ii_ReadyForInserts;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexInfo, ii_ReadyForInserts) == 170
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ii_ReadyForInserts>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 170;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexInfo,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexInfo> as c::expression::OrdinaryField<
        Field_ii_ReadyForInserts,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ii_ReadyForInserts) };
#[doc(hidden)]
pub struct Field_ii_CheckedUnchanged;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexInfo, ii_CheckedUnchanged) == 171
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ii_CheckedUnchanged>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 171;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexInfo,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexInfo> as c::expression::OrdinaryField<
        Field_ii_CheckedUnchanged,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ii_CheckedUnchanged) };
#[doc(hidden)]
pub struct Field_ii_IndexUnchanged;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexInfo, ii_IndexUnchanged) == 172);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ii_IndexUnchanged>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 172;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexInfo,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexInfo> as c::expression::OrdinaryField<
        Field_ii_IndexUnchanged,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ii_IndexUnchanged) };
#[doc(hidden)]
pub struct Field_ii_Concurrent;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexInfo, ii_Concurrent) == 173);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ii_Concurrent>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 173;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexInfo,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexInfo> as c::expression::OrdinaryField<
        Field_ii_Concurrent,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ii_Concurrent) };
#[doc(hidden)]
pub struct Field_ii_BrokenHotChain;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexInfo, ii_BrokenHotChain) == 174);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ii_BrokenHotChain>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 174;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexInfo,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexInfo> as c::expression::OrdinaryField<
        Field_ii_BrokenHotChain,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ii_BrokenHotChain) };
#[doc(hidden)]
pub struct Field_ii_ParallelWorkers;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexInfo, ii_ParallelWorkers) == 176
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ii_ParallelWorkers>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 176;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexInfo,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexInfo> as c::expression::OrdinaryField<
        Field_ii_ParallelWorkers,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ii_ParallelWorkers) };
#[doc(hidden)]
pub struct Field_ii_Am;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexInfo, ii_Am) == 180);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ii_Am>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 180;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexInfo,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexInfo> as c::expression::OrdinaryField<
        Field_ii_Am,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ii_Am) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::IndexOnlyScan>() == 160);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::IndexOnlyScan>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::IndexOnlyScan {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOnlyScan, indexid) == 112);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indexid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOnlyScan>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexOnlyScan, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexOnlyScan> as c::expression::OrdinaryField<Field_indexid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indexid) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOnlyScan, indexqual) == 120);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indexqual>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOnlyScan>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexOnlyScan, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexOnlyScan> as c::expression::OrdinaryField<Field_indexqual>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indexqual) };
#[doc(hidden)]
pub struct Field_recheckqual;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOnlyScan, recheckqual) == 128);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_recheckqual>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOnlyScan>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexOnlyScan, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexOnlyScan> as c::expression::OrdinaryField<Field_recheckqual>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).recheckqual) };
#[doc(hidden)]
pub struct Field_indexorderby;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOnlyScan, indexorderby) == 136);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indexorderby>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOnlyScan>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexOnlyScan, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexOnlyScan> as c::expression::OrdinaryField<Field_indexorderby>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indexorderby) };
#[doc(hidden)]
pub struct Field_indextlist;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOnlyScan, indextlist) == 144);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indextlist>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOnlyScan>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexOnlyScan, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexOnlyScan> as c::expression::OrdinaryField<Field_indextlist>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indextlist) };
#[doc(hidden)]
pub struct Field_indexorderdir;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOnlyScan, indexorderdir) == 152);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indexorderdir>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOnlyScan>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ScanDirection,
        c::CInt,
        i32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexOnlyScan, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexOnlyScan> as c::expression::OrdinaryField<Field_indexorderdir>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indexorderdir) };
#[doc(hidden)]
pub struct Field_ioss_NumScanKeys;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::IndexOnlyScanState>() == 344);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::IndexOnlyScanState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::IndexOnlyScanState {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOnlyScanState, ioss_NumScanKeys) == 240
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ioss_NumScanKeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOnlyScanState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 240;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexOnlyScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexOnlyScanState> as c::expression::OrdinaryField<Field_ioss_NumScanKeys>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ioss_NumScanKeys) };
#[doc(hidden)]
pub struct Field_ioss_NumOrderByKeys;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOnlyScanState, ioss_NumOrderByKeys)
        == 256
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ioss_NumOrderByKeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOnlyScanState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 256;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexOnlyScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexOnlyScanState> as c::expression::OrdinaryField<Field_ioss_NumOrderByKeys>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ioss_NumOrderByKeys) };
#[doc(hidden)]
pub struct Field_ioss_NumRuntimeKeys;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOnlyScanState, ioss_NumRuntimeKeys)
        == 272
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ioss_NumRuntimeKeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOnlyScanState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 272;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexOnlyScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexOnlyScanState> as c::expression::OrdinaryField<Field_ioss_NumRuntimeKeys>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ioss_NumRuntimeKeys) };
#[doc(hidden)]
pub struct Field_ioss_RuntimeKeysReady;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOnlyScanState, ioss_RuntimeKeysReady)
        == 276
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ioss_RuntimeKeysReady>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOnlyScanState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 276;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexOnlyScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexOnlyScanState> as c::expression::OrdinaryField<Field_ioss_RuntimeKeysReady>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ioss_RuntimeKeysReady) };
#[doc(hidden)]
pub struct Field_ioss_VMBuffer;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOnlyScanState, ioss_VMBuffer) == 312
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ioss_VMBuffer>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOnlyScanState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 312;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexOnlyScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexOnlyScanState> as c::expression::OrdinaryField<Field_ioss_VMBuffer>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ioss_VMBuffer) };
#[doc(hidden)]
pub struct Field_ioss_PscanLen;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOnlyScanState, ioss_PscanLen) == 320
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ioss_PscanLen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOnlyScanState>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 320;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexOnlyScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexOnlyScanState> as c::expression::OrdinaryField<Field_ioss_PscanLen>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ioss_PscanLen) };
#[doc(hidden)]
pub struct Field_ioss_NameCStringCount;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOnlyScanState, ioss_NameCStringCount)
        == 336
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ioss_NameCStringCount>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOnlyScanState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 336;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexOnlyScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexOnlyScanState> as c::expression::OrdinaryField<Field_ioss_NameCStringCount>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ioss_NameCStringCount) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::IndexOptInfo>() == 192);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::IndexOptInfo>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::IndexOptInfo {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOptInfo, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo>
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
const _: Projection<crate::__pgrx_c_bindings::IndexOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_indexoid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOptInfo, indexoid) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indexoid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo> as c::expression::OrdinaryField<Field_indexoid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indexoid) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOptInfo, reltablespace) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_reltablespace>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo> as c::expression::OrdinaryField<Field_reltablespace>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).reltablespace) };
#[doc(hidden)]
pub struct Field_pages;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOptInfo, pages) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pages>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo> as c::expression::OrdinaryField<Field_pages>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pages) };
#[doc(hidden)]
pub struct Field_tree_height;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOptInfo, tree_height) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tree_height>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo> as c::expression::OrdinaryField<Field_tree_height>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tree_height) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOptInfo, ncolumns) == 44);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ncolumns>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo> as c::expression::OrdinaryField<Field_ncolumns>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ncolumns) };
#[doc(hidden)]
pub struct Field_nkeycolumns;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOptInfo, nkeycolumns) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nkeycolumns>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo> as c::expression::OrdinaryField<Field_nkeycolumns>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nkeycolumns) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOptInfo, relam) == 128);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relam>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo> as c::expression::OrdinaryField<Field_relam>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relam) };
#[doc(hidden)]
pub struct Field_indexprs;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOptInfo, indexprs) == 136);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indexprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo> as c::expression::OrdinaryField<Field_indexprs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indexprs) };
#[doc(hidden)]
pub struct Field_indpred;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOptInfo, indpred) == 144);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indpred>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo> as c::expression::OrdinaryField<Field_indpred>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indpred) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOptInfo, indextlist) == 152);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indextlist>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo> as c::expression::OrdinaryField<Field_indextlist>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indextlist) };
#[doc(hidden)]
pub struct Field_indrestrictinfo;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOptInfo, indrestrictinfo) == 160
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indrestrictinfo>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo> as c::expression::OrdinaryField<Field_indrestrictinfo>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indrestrictinfo) };
#[doc(hidden)]
pub struct Field_predOK;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOptInfo, predOK) == 168);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_predOK>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 168;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo> as c::expression::OrdinaryField<Field_predOK>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).predOK) };
#[doc(hidden)]
pub struct Field_unique;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOptInfo, unique) == 169);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_unique>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 169;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo> as c::expression::OrdinaryField<Field_unique>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).unique) };
#[doc(hidden)]
pub struct Field_immediate;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOptInfo, immediate) == 170);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_immediate>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 170;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo> as c::expression::OrdinaryField<Field_immediate>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).immediate) };
#[doc(hidden)]
pub struct Field_hypothetical;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOptInfo, hypothetical) == 171);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hypothetical>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 171;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo> as c::expression::OrdinaryField<Field_hypothetical>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hypothetical) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOptInfo, amcanorderbyop) == 172);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amcanorderbyop>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 172;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo> as c::expression::OrdinaryField<Field_amcanorderbyop>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).amcanorderbyop) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOptInfo, amoptionalkey) == 173);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amoptionalkey>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 173;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo> as c::expression::OrdinaryField<Field_amoptionalkey>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).amoptionalkey) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOptInfo, amsearcharray) == 174);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amsearcharray>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 174;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo> as c::expression::OrdinaryField<Field_amsearcharray>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).amsearcharray) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOptInfo, amsearchnulls) == 175);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amsearchnulls>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 175;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo> as c::expression::OrdinaryField<Field_amsearchnulls>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).amsearchnulls) };
#[doc(hidden)]
pub struct Field_amhasgettuple;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOptInfo, amhasgettuple) == 176);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amhasgettuple>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 176;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo> as c::expression::OrdinaryField<Field_amhasgettuple>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).amhasgettuple) };
#[doc(hidden)]
pub struct Field_amhasgetbitmap;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOptInfo, amhasgetbitmap) == 177);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amhasgetbitmap>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 177;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo> as c::expression::OrdinaryField<Field_amhasgetbitmap>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).amhasgetbitmap) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOptInfo, amcanparallel) == 178);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amcanparallel>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 178;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo> as c::expression::OrdinaryField<Field_amcanparallel>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).amcanparallel) };
#[doc(hidden)]
pub struct Field_amcanmarkpos;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOptInfo, amcanmarkpos) == 179);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amcanmarkpos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 179;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexOptInfo> as c::expression::OrdinaryField<Field_amcanmarkpos>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).amcanmarkpos) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::IndexOrderByDistance>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::IndexOrderByDistance>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::IndexOrderByDistance {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOrderByDistance, value) == 0);
const _: () = {
    assert!(::core::mem::size_of::<f64>() == 8);
    assert!(::core::mem::align_of::<f64>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_value>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOrderByDistance>
{
    type Member = c::expression::CDouble;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexOrderByDistance, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexOrderByDistance> as c::expression::OrdinaryField<Field_value>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).value) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexOrderByDistance, isnull) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isnull>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexOrderByDistance>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexOrderByDistance, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexOrderByDistance> as c::expression::OrdinaryField<Field_isnull>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isnull) };
#[doc(hidden)]
pub struct Field_indexclauses;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::IndexPath>() == 128);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::IndexPath>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::IndexPath {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexPath, indexclauses) == 80);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indexclauses>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexPath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexPath,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexPath> as c::expression::OrdinaryField<
        Field_indexclauses,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indexclauses) };
#[doc(hidden)]
pub struct Field_indexorderbys;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexPath, indexorderbys) == 88);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indexorderbys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexPath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexPath,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexPath> as c::expression::OrdinaryField<
        Field_indexorderbys,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indexorderbys) };
#[doc(hidden)]
pub struct Field_indexorderbycols;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexPath, indexorderbycols) == 96);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indexorderbycols>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexPath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexPath,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexPath> as c::expression::OrdinaryField<
        Field_indexorderbycols,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indexorderbycols) };
#[doc(hidden)]
pub struct Field_indexscandir;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexPath, indexscandir) == 104);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indexscandir>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexPath>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ScanDirection,
        c::CInt,
        i32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexPath,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexPath> as c::expression::OrdinaryField<
        Field_indexscandir,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indexscandir) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::IndexScan>() == 168);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::IndexScan>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::IndexScan {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexScan, indexid) == 112);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indexid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexScan>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexScan,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexScan> as c::expression::OrdinaryField<
        Field_indexid,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indexid) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexScan, indexqual) == 120);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indexqual>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexScan>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexScan,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexScan> as c::expression::OrdinaryField<
        Field_indexqual,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indexqual) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexScan, indexqualorig) == 128);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indexqualorig>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexScan>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexScan,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexScan> as c::expression::OrdinaryField<
        Field_indexqualorig,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indexqualorig) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexScan, indexorderby) == 136);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indexorderby>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexScan>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexScan,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexScan> as c::expression::OrdinaryField<
        Field_indexorderby,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indexorderby) };
#[doc(hidden)]
pub struct Field_indexorderbyorig;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexScan, indexorderbyorig) == 144);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indexorderbyorig>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexScan>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexScan,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexScan> as c::expression::OrdinaryField<
        Field_indexorderbyorig,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indexorderbyorig) };
#[doc(hidden)]
pub struct Field_indexorderbyops;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexScan, indexorderbyops) == 152);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indexorderbyops>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexScan>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexScan,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexScan> as c::expression::OrdinaryField<
        Field_indexorderbyops,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indexorderbyops) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexScan, indexorderdir) == 160);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indexorderdir>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexScan>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ScanDirection,
        c::CInt,
        i32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexScan,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexScan> as c::expression::OrdinaryField<
        Field_indexorderdir,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indexorderdir) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::IndexScanDescData>() == 152);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::IndexScanDescData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::IndexScanDescData {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexScanDescData, numberOfKeys) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numberOfKeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexScanDescData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexScanDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexScanDescData> as c::expression::OrdinaryField<Field_numberOfKeys>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numberOfKeys) };
#[doc(hidden)]
pub struct Field_numberOfOrderBys;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexScanDescData, numberOfOrderBys) == 28
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numberOfOrderBys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexScanDescData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexScanDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexScanDescData> as c::expression::OrdinaryField<Field_numberOfOrderBys>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numberOfOrderBys) };
#[doc(hidden)]
pub struct Field_xs_want_itup;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexScanDescData, xs_want_itup) == 48
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xs_want_itup>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexScanDescData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexScanDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexScanDescData> as c::expression::OrdinaryField<Field_xs_want_itup>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xs_want_itup) };
#[doc(hidden)]
pub struct Field_xs_temp_snap;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexScanDescData, xs_temp_snap) == 49
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xs_temp_snap>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexScanDescData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 49;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexScanDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexScanDescData> as c::expression::OrdinaryField<Field_xs_temp_snap>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xs_temp_snap) };
#[doc(hidden)]
pub struct Field_kill_prior_tuple;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexScanDescData, kill_prior_tuple) == 50
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_kill_prior_tuple>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexScanDescData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 50;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexScanDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexScanDescData> as c::expression::OrdinaryField<Field_kill_prior_tuple>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).kill_prior_tuple) };
#[doc(hidden)]
pub struct Field_ignore_killed_tuples;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexScanDescData, ignore_killed_tuples)
        == 51
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ignore_killed_tuples>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexScanDescData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 51;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexScanDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexScanDescData> as c::expression::OrdinaryField<Field_ignore_killed_tuples>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ignore_killed_tuples) };
#[doc(hidden)]
pub struct Field_xactStartedInRecovery;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexScanDescData, xactStartedInRecovery)
        == 52
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xactStartedInRecovery>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexScanDescData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexScanDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexScanDescData> as c::expression::OrdinaryField<Field_xactStartedInRecovery>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xactStartedInRecovery) };
#[doc(hidden)]
pub struct Field_xs_heap_continue;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexScanDescData, xs_heap_continue) == 102
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xs_heap_continue>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexScanDescData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 102;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexScanDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexScanDescData> as c::expression::OrdinaryField<Field_xs_heap_continue>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xs_heap_continue) };
#[doc(hidden)]
pub struct Field_xs_recheck;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexScanDescData, xs_recheck) == 112
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xs_recheck>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexScanDescData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexScanDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexScanDescData> as c::expression::OrdinaryField<Field_xs_recheck>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xs_recheck) };
#[doc(hidden)]
pub struct Field_xs_recheckorderby;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexScanDescData, xs_recheckorderby) == 136
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xs_recheckorderby>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexScanDescData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexScanDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexScanDescData> as c::expression::OrdinaryField<Field_xs_recheckorderby>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xs_recheckorderby) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::IndexScanState>() == 376);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::IndexScanState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::IndexScanState {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexScanState, indexorderbyorig) == 232
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indexorderbyorig>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexScanState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 232;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexScanState> as c::expression::OrdinaryField<Field_indexorderbyorig>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indexorderbyorig) };
#[doc(hidden)]
pub struct Field_iss_NumScanKeys;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexScanState, iss_NumScanKeys) == 248
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_iss_NumScanKeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexScanState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 248;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexScanState> as c::expression::OrdinaryField<Field_iss_NumScanKeys>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).iss_NumScanKeys) };
#[doc(hidden)]
pub struct Field_iss_NumOrderByKeys;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexScanState, iss_NumOrderByKeys) == 264
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_iss_NumOrderByKeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexScanState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 264;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexScanState> as c::expression::OrdinaryField<Field_iss_NumOrderByKeys>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).iss_NumOrderByKeys) };
#[doc(hidden)]
pub struct Field_iss_NumRuntimeKeys;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexScanState, iss_NumRuntimeKeys) == 280
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_iss_NumRuntimeKeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexScanState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 280;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexScanState> as c::expression::OrdinaryField<Field_iss_NumRuntimeKeys>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).iss_NumRuntimeKeys) };
#[doc(hidden)]
pub struct Field_iss_RuntimeKeysReady;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexScanState, iss_RuntimeKeysReady) == 284
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_iss_RuntimeKeysReady>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexScanState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 284;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexScanState> as c::expression::OrdinaryField<Field_iss_RuntimeKeysReady>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).iss_RuntimeKeysReady) };
#[doc(hidden)]
pub struct Field_iss_ReachedEnd;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexScanState, iss_ReachedEnd) == 320
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_iss_ReachedEnd>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexScanState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 320;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexScanState> as c::expression::OrdinaryField<Field_iss_ReachedEnd>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).iss_ReachedEnd) };
#[doc(hidden)]
pub struct Field_iss_PscanLen;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexScanState, iss_PscanLen) == 368);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_iss_PscanLen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexScanState>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 368;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexScanState> as c::expression::OrdinaryField<Field_iss_PscanLen>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).iss_PscanLen) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::IndexStmt>() == 120);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::IndexStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::IndexStmt {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt>
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
    crate::__pgrx_c_bindings::IndexStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_indexParams;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexStmt, indexParams) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indexParams>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt> as c::expression::OrdinaryField<
        Field_indexParams,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indexParams) };
#[doc(hidden)]
pub struct Field_indexIncludingParams;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexStmt, indexIncludingParams) == 48
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indexIncludingParams>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt> as c::expression::OrdinaryField<
        Field_indexIncludingParams,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indexIncludingParams) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexStmt, options) == 56);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt>
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
    crate::__pgrx_c_bindings::IndexStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt> as c::expression::OrdinaryField<
        Field_options,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
#[doc(hidden)]
pub struct Field_excludeOpNames;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexStmt, excludeOpNames) == 72);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_excludeOpNames>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt>
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
    crate::__pgrx_c_bindings::IndexStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt> as c::expression::OrdinaryField<
        Field_excludeOpNames,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).excludeOpNames) };
#[doc(hidden)]
pub struct Field_indexOid;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexStmt, indexOid) == 88);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indexOid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt> as c::expression::OrdinaryField<
        Field_indexOid,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indexOid) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_oldNode;
#[cfg(feature = "pg15")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexStmt, oldNode) == 92);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldNode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 92;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt> as c::expression::OrdinaryField<
        Field_oldNode,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oldNode) };
#[doc(hidden)]
pub struct Field_oldCreateSubid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexStmt, oldCreateSubid) == 96);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldCreateSubid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt> as c::expression::OrdinaryField<
        Field_oldCreateSubid,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oldCreateSubid) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_oldFirstRelfilenodeSubid;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexStmt, oldFirstRelfilenodeSubid) == 100
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldFirstRelfilenodeSubid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 100;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt> as c::expression::OrdinaryField<
        Field_oldFirstRelfilenodeSubid,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oldFirstRelfilenodeSubid) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexStmt, unique) == 104);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_unique>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt> as c::expression::OrdinaryField<
        Field_unique,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).unique) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexStmt, nulls_not_distinct) == 105
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nulls_not_distinct>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 105;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt> as c::expression::OrdinaryField<
        Field_nulls_not_distinct,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nulls_not_distinct) };
#[doc(hidden)]
pub struct Field_primary;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexStmt, primary) == 106);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_primary>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 106;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt> as c::expression::OrdinaryField<
        Field_primary,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).primary) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexStmt, isconstraint) == 107);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isconstraint>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 107;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt> as c::expression::OrdinaryField<
        Field_isconstraint,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isconstraint) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexStmt, deferrable) == 108);
