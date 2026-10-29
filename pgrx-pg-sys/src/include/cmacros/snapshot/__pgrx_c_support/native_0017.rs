// Shared generated C macro support.

#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mutex>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelBitmapHeapState>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelBitmapHeapState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelBitmapHeapState> as c::expression::OrdinaryField<Field_mutex>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mutex) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelBitmapHeapState, prefetch_pages)
        == 20
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_prefetch_pages>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelBitmapHeapState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelBitmapHeapState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelBitmapHeapState> as c::expression::OrdinaryField<Field_prefetch_pages>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).prefetch_pages) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelBitmapHeapState, prefetch_target)
        == 24
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_prefetch_target>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelBitmapHeapState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelBitmapHeapState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelBitmapHeapState> as c::expression::OrdinaryField<Field_prefetch_target>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).prefetch_target) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelBitmapHeapState, state) == 28
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_state>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelBitmapHeapState>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_SharedBitmapState,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelBitmapHeapState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelBitmapHeapState> as c::expression::OrdinaryField<Field_state>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).state) };
#[doc(hidden)]
pub struct Field_phs_nblocks;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ParallelBlockTableScanDescData>() == 40
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ParallelBlockTableScanDescData>() == 8
    );
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ParallelBlockTableScanDescData {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelBlockTableScanDescData, phs_nblocks)
        == 16
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_phs_nblocks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelBlockTableScanDescData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelBlockTableScanDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelBlockTableScanDescData> as c::expression::OrdinaryField<Field_phs_nblocks>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).phs_nblocks) };
#[doc(hidden)]
pub struct Field_phs_mutex;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelBlockTableScanDescData, phs_mutex)
        == 20
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_phs_mutex>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelBlockTableScanDescData>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelBlockTableScanDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelBlockTableScanDescData> as c::expression::OrdinaryField<Field_phs_mutex>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).phs_mutex) };
#[doc(hidden)]
pub struct Field_phs_startblock;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ParallelBlockTableScanDescData,
        phs_startblock
    ) == 24
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_phs_startblock>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelBlockTableScanDescData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelBlockTableScanDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelBlockTableScanDescData> as c::expression::OrdinaryField<Field_phs_startblock>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).phs_startblock) };
#[doc(hidden)]
pub struct Field_phsw_nallocated;
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ParallelBlockTableScanWorkerData>() == 16
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ParallelBlockTableScanWorkerData>() == 8
    );
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ParallelBlockTableScanWorkerData {}
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ParallelBlockTableScanWorkerData,
        phsw_nallocated
    ) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_phsw_nallocated>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelBlockTableScanWorkerData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelBlockTableScanWorkerData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelBlockTableScanWorkerData> as c::expression::OrdinaryField<Field_phsw_nallocated>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).phsw_nallocated) };
#[doc(hidden)]
pub struct Field_phsw_chunk_remaining;
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ParallelBlockTableScanWorkerData,
        phsw_chunk_remaining
    ) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_phsw_chunk_remaining>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelBlockTableScanWorkerData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelBlockTableScanWorkerData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelBlockTableScanWorkerData> as c::expression::OrdinaryField<Field_phsw_chunk_remaining>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).phsw_chunk_remaining) };
#[doc(hidden)]
pub struct Field_phsw_chunk_size;
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ParallelBlockTableScanWorkerData,
        phsw_chunk_size
    ) == 12
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_phsw_chunk_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelBlockTableScanWorkerData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelBlockTableScanWorkerData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelBlockTableScanWorkerData> as c::expression::OrdinaryField<Field_phsw_chunk_size>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).phsw_chunk_size) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ParallelContext>() == 120);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ParallelContext>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ParallelContext {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelContext, subid) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_subid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelContext>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelContext, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelContext> as c::expression::OrdinaryField<Field_subid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).subid) };
#[doc(hidden)]
pub struct Field_nworkers;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelContext, nworkers) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nworkers>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelContext>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelContext, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelContext> as c::expression::OrdinaryField<Field_nworkers>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nworkers) };
#[doc(hidden)]
pub struct Field_nworkers_to_launch;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelContext, nworkers_to_launch) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nworkers_to_launch>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelContext>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelContext, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelContext> as c::expression::OrdinaryField<Field_nworkers_to_launch>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nworkers_to_launch) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelContext, nworkers_launched) == 28
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nworkers_launched>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelContext>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelContext, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelContext> as c::expression::OrdinaryField<Field_nworkers_launched>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nworkers_launched) };
#[doc(hidden)]
pub struct Field_nknown_attached_workers;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelContext, nknown_attached_workers)
        == 104
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nknown_attached_workers>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelContext>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelContext, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelContext> as c::expression::OrdinaryField<Field_nknown_attached_workers>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nknown_attached_workers) };
#[doc(hidden)]
pub struct Field_param_exec;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ParallelExecutorInfo>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ParallelExecutorInfo>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ParallelExecutorInfo {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelExecutorInfo, param_exec) == 56
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_param_exec>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelExecutorInfo>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelExecutorInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelExecutorInfo> as c::expression::OrdinaryField<Field_param_exec>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).param_exec) };
#[doc(hidden)]
pub struct Field_finished;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelExecutorInfo, finished) == 64
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_finished>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelExecutorInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelExecutorInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelExecutorInfo> as c::expression::OrdinaryField<Field_finished>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).finished) };
#[doc(hidden)]
pub struct Field_buckets;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ParallelHashJoinBatch>() == 96);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ParallelHashJoinBatch>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ParallelHashJoinBatch {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelHashJoinBatch, buckets) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_buckets>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinBatch>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelHashJoinBatch, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinBatch> as c::expression::OrdinaryField<Field_buckets>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).buckets) };
#[doc(hidden)]
pub struct Field_chunks;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelHashJoinBatch, chunks) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_chunks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinBatch>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelHashJoinBatch, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinBatch> as c::expression::OrdinaryField<Field_chunks>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).chunks) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelHashJoinBatch, size) == 56);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinBatch>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelHashJoinBatch, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinBatch> as c::expression::OrdinaryField<Field_size>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).size) };
#[doc(hidden)]
pub struct Field_estimated_size;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelHashJoinBatch, estimated_size) == 64
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_estimated_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinBatch>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelHashJoinBatch, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinBatch> as c::expression::OrdinaryField<Field_estimated_size>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).estimated_size) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelHashJoinBatch, ntuples) == 72
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ntuples>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinBatch>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelHashJoinBatch, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinBatch> as c::expression::OrdinaryField<Field_ntuples>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ntuples) };
#[doc(hidden)]
pub struct Field_old_ntuples;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelHashJoinBatch, old_ntuples) == 80
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_old_ntuples>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinBatch>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelHashJoinBatch, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinBatch> as c::expression::OrdinaryField<Field_old_ntuples>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).old_ntuples) };
#[doc(hidden)]
pub struct Field_space_exhausted;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelHashJoinBatch, space_exhausted) == 88
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_space_exhausted>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinBatch>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelHashJoinBatch, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinBatch> as c::expression::OrdinaryField<Field_space_exhausted>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).space_exhausted) };
#[doc(hidden)]
pub struct Field_preallocated;
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ParallelHashJoinBatchAccessor>() == 72
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ParallelHashJoinBatchAccessor>() == 8
    );
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ParallelHashJoinBatchAccessor {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelHashJoinBatchAccessor, preallocated)
        == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_preallocated>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinBatchAccessor>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelHashJoinBatchAccessor, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinBatchAccessor> as c::expression::OrdinaryField<Field_preallocated>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).preallocated) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelHashJoinBatchAccessor, ntuples) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ntuples>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinBatchAccessor>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelHashJoinBatchAccessor, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinBatchAccessor> as c::expression::OrdinaryField<Field_ntuples>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ntuples) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelHashJoinBatchAccessor, size) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinBatchAccessor>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelHashJoinBatchAccessor, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinBatchAccessor> as c::expression::OrdinaryField<Field_size>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).size) };
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ParallelHashJoinBatchAccessor,
        estimated_size
    ) == 32
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_estimated_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinBatchAccessor>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelHashJoinBatchAccessor, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinBatchAccessor> as c::expression::OrdinaryField<Field_estimated_size>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).estimated_size) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelHashJoinBatchAccessor, old_ntuples)
        == 40
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_old_ntuples>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinBatchAccessor>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelHashJoinBatchAccessor, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinBatchAccessor> as c::expression::OrdinaryField<Field_old_ntuples>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).old_ntuples) };
#[doc(hidden)]
pub struct Field_at_least_one_chunk;
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ParallelHashJoinBatchAccessor,
        at_least_one_chunk
    ) == 48
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_at_least_one_chunk>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinBatchAccessor>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelHashJoinBatchAccessor, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinBatchAccessor> as c::expression::OrdinaryField<Field_at_least_one_chunk>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).at_least_one_chunk) };
#[doc(hidden)]
pub struct Field_done;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelHashJoinBatchAccessor, done) == 49
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_done>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinBatchAccessor>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 49;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelHashJoinBatchAccessor, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinBatchAccessor> as c::expression::OrdinaryField<Field_done>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).done) };
#[doc(hidden)]
pub struct Field_batches;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ParallelHashJoinState>() == 248);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ParallelHashJoinState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ParallelHashJoinState {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelHashJoinState, batches) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_batches>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinState>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelHashJoinState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinState> as c::expression::OrdinaryField<Field_batches>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).batches) };
#[doc(hidden)]
pub struct Field_old_batches;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelHashJoinState, old_batches) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_old_batches>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinState>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelHashJoinState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinState> as c::expression::OrdinaryField<Field_old_batches>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).old_batches) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelHashJoinState, nbatch) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nbatch>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelHashJoinState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinState> as c::expression::OrdinaryField<Field_nbatch>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nbatch) };
#[doc(hidden)]
pub struct Field_old_nbatch;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelHashJoinState, old_nbatch) == 20
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_old_nbatch>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelHashJoinState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinState> as c::expression::OrdinaryField<Field_old_nbatch>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).old_nbatch) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelHashJoinState, nbuckets) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nbuckets>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelHashJoinState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinState> as c::expression::OrdinaryField<Field_nbuckets>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nbuckets) };
#[doc(hidden)]
pub struct Field_growth;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelHashJoinState, growth) == 28);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_growth>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinState>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ParallelHashGrowth,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelHashJoinState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinState> as c::expression::OrdinaryField<Field_growth>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).growth) };
#[doc(hidden)]
pub struct Field_chunk_work_queue;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelHashJoinState, chunk_work_queue)
        == 32
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_chunk_work_queue>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinState>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelHashJoinState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinState> as c::expression::OrdinaryField<Field_chunk_work_queue>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).chunk_work_queue) };
#[doc(hidden)]
pub struct Field_nparticipants;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelHashJoinState, nparticipants) == 40
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nparticipants>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelHashJoinState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinState> as c::expression::OrdinaryField<Field_nparticipants>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nparticipants) };
#[doc(hidden)]
pub struct Field_space_allowed;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelHashJoinState, space_allowed) == 48
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_space_allowed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinState>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelHashJoinState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinState> as c::expression::OrdinaryField<Field_space_allowed>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).space_allowed) };
#[doc(hidden)]
pub struct Field_total_tuples;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelHashJoinState, total_tuples) == 56
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_total_tuples>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinState>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelHashJoinState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelHashJoinState> as c::expression::OrdinaryField<Field_total_tuples>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).total_tuples) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
#[doc(hidden)]
pub struct Field_ps_relid;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ParallelIndexScanDescData>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ParallelIndexScanDescData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ParallelIndexScanDescData {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelIndexScanDescData, ps_relid) == 0
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ps_relid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelIndexScanDescData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelIndexScanDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelIndexScanDescData> as c::expression::OrdinaryField<Field_ps_relid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ps_relid) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
#[doc(hidden)]
pub struct Field_ps_indexid;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelIndexScanDescData, ps_indexid) == 4
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ps_indexid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelIndexScanDescData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelIndexScanDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelIndexScanDescData> as c::expression::OrdinaryField<Field_ps_indexid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ps_indexid) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
#[doc(hidden)]
pub struct Field_ps_offset;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelIndexScanDescData, ps_offset) == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ps_offset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelIndexScanDescData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelIndexScanDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelIndexScanDescData> as c::expression::OrdinaryField<Field_ps_offset>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ps_offset) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
#[doc(hidden)]
pub struct Field_phs_relid;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ParallelTableScanDescData>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ParallelTableScanDescData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ParallelTableScanDescData {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelTableScanDescData, phs_relid) == 0
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_phs_relid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelTableScanDescData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelTableScanDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelTableScanDescData> as c::expression::OrdinaryField<Field_phs_relid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).phs_relid) };
#[doc(hidden)]
pub struct Field_phs_syncscan;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelTableScanDescData, phs_syncscan) == 4
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_phs_syncscan>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelTableScanDescData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelTableScanDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelTableScanDescData> as c::expression::OrdinaryField<Field_phs_syncscan>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).phs_syncscan) };
#[doc(hidden)]
pub struct Field_phs_snapshot_any;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelTableScanDescData, phs_snapshot_any)
        == 5
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_phs_snapshot_any>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelTableScanDescData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 5;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelTableScanDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelTableScanDescData> as c::expression::OrdinaryField<Field_phs_snapshot_any>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).phs_snapshot_any) };
#[doc(hidden)]
pub struct Field_phs_snapshot_off;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelTableScanDescData, phs_snapshot_off)
        == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_phs_snapshot_off>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelTableScanDescData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelTableScanDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelTableScanDescData> as c::expression::OrdinaryField<Field_phs_snapshot_off>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).phs_snapshot_off) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ParallelWorkerInfo>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ParallelWorkerInfo>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ParallelWorkerInfo {}
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelWorkerInfo, pid) == 16);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelWorkerInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelWorkerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelWorkerInfo> as c::expression::OrdinaryField<Field_pid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pid) };
#[doc(hidden)]
pub struct Field_paramkind;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Param>() == 28);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Param>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::Param {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Param, paramkind) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_paramkind>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Param>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ParamKind,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Param,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Param> as c::expression::OrdinaryField<
        Field_paramkind,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).paramkind) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Param, paramid) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_paramid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Param>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Param,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Param> as c::expression::OrdinaryField<
        Field_paramid,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).paramid) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Param, paramtype) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_paramtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Param>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Param,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Param> as c::expression::OrdinaryField<
        Field_paramtype,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).paramtype) };
#[doc(hidden)]
pub struct Field_paramtypmod;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Param, paramtypmod) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_paramtypmod>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Param>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Param,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Param> as c::expression::OrdinaryField<
        Field_paramtypmod,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).paramtypmod) };
#[doc(hidden)]
pub struct Field_paramcollid;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Param, paramcollid) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_paramcollid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Param>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Param,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Param> as c::expression::OrdinaryField<
        Field_paramcollid,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).paramcollid) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Param, location) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Param>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Param,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Param> as c::expression::OrdinaryField<
        Field_location,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ParamExecData>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ParamExecData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ParamExecData {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParamExecData, value) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_value>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParamExecData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, crate::__pgrx_c_bindings::Datum>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParamExecData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParamExecData> as c::expression::OrdinaryField<Field_value>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).value) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParamExecData, isnull) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isnull>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParamExecData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParamExecData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParamExecData> as c::expression::OrdinaryField<Field_isnull>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isnull) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ParamExternData>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ParamExternData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ParamExternData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParamExternData, value) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_value>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParamExternData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, crate::__pgrx_c_bindings::Datum>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParamExternData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParamExternData> as c::expression::OrdinaryField<Field_value>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).value) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParamExternData, isnull) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isnull>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParamExternData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParamExternData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParamExternData> as c::expression::OrdinaryField<Field_isnull>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isnull) };
#[doc(hidden)]
pub struct Field_pflags;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParamExternData, pflags) == 10);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pflags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParamExternData>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 10;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParamExternData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParamExternData> as c::expression::OrdinaryField<Field_pflags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pflags) };
#[doc(hidden)]
pub struct Field_ptype;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParamExternData, ptype) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ptype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParamExternData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParamExternData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParamExternData> as c::expression::OrdinaryField<Field_ptype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ptype) };
#[doc(hidden)]
pub struct Field_numParams;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ParamListInfoData>() == 64);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ParamListInfoData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ParamListInfoData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParamListInfoData, numParams) == 56);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numParams>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParamListInfoData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParamListInfoData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParamListInfoData> as c::expression::OrdinaryField<Field_numParams>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numParams) };
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ParamPathInfo>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ParamPathInfo>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ParamPathInfo {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParamPathInfo, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParamPathInfo>
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
const _: Projection<crate::__pgrx_c_bindings::ParamPathInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParamPathInfo> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_ppi_req_outer;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParamPathInfo, ppi_req_outer) == 8);
const _: () = {
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::Bitmapset>() == 8);
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::Bitmapset>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ppi_req_outer>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParamPathInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::Bitmapset>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParamPathInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParamPathInfo> as c::expression::OrdinaryField<Field_ppi_req_outer>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ppi_req_outer) };
#[doc(hidden)]
pub struct Field_ppi_clauses;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParamPathInfo, ppi_clauses) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ppi_clauses>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParamPathInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParamPathInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParamPathInfo> as c::expression::OrdinaryField<Field_ppi_clauses>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ppi_clauses) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ParamRef>() == 12);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ParamRef>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ParamRef {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParamRef, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParamRef>
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
    crate::__pgrx_c_bindings::ParamRef,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ParamRef> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParamRef, number) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_number>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParamRef>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ParamRef,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ParamRef> as c::expression::OrdinaryField<
        Field_number,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).number) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParamRef, location) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParamRef>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ParamRef,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ParamRef> as c::expression::OrdinaryField<
        Field_location,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ParseCallbackState>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ParseCallbackState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ParseCallbackState {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseCallbackState, location) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseCallbackState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParseCallbackState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParseCallbackState> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[doc(hidden)]
pub struct Field_p_varno;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ParseNamespaceColumn>() == 28);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ParseNamespaceColumn>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ParseNamespaceColumn {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseNamespaceColumn, p_varno) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_varno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseNamespaceColumn>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParseNamespaceColumn, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParseNamespaceColumn> as c::expression::OrdinaryField<Field_p_varno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).p_varno) };
#[doc(hidden)]
pub struct Field_p_varattno;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseNamespaceColumn, p_varattno) == 4
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_varattno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseNamespaceColumn>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParseNamespaceColumn, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParseNamespaceColumn> as c::expression::OrdinaryField<Field_p_varattno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).p_varattno) };
#[doc(hidden)]
pub struct Field_p_vartype;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseNamespaceColumn, p_vartype) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_vartype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseNamespaceColumn>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParseNamespaceColumn, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParseNamespaceColumn> as c::expression::OrdinaryField<Field_p_vartype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).p_vartype) };
#[doc(hidden)]
pub struct Field_p_vartypmod;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseNamespaceColumn, p_vartypmod) == 12
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_vartypmod>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseNamespaceColumn>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParseNamespaceColumn, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParseNamespaceColumn> as c::expression::OrdinaryField<Field_p_vartypmod>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).p_vartypmod) };
#[doc(hidden)]
pub struct Field_p_varcollid;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseNamespaceColumn, p_varcollid) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_varcollid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseNamespaceColumn>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParseNamespaceColumn, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParseNamespaceColumn> as c::expression::OrdinaryField<Field_p_varcollid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).p_varcollid) };
#[doc(hidden)]
pub struct Field_p_varnosyn;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseNamespaceColumn, p_varnosyn) == 20
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_varnosyn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseNamespaceColumn>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParseNamespaceColumn, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParseNamespaceColumn> as c::expression::OrdinaryField<Field_p_varnosyn>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).p_varnosyn) };
#[doc(hidden)]
pub struct Field_p_varattnosyn;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseNamespaceColumn, p_varattnosyn) == 24
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_varattnosyn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseNamespaceColumn>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParseNamespaceColumn, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParseNamespaceColumn> as c::expression::OrdinaryField<Field_p_varattnosyn>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).p_varattnosyn) };
#[doc(hidden)]
pub struct Field_p_dontexpand;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseNamespaceColumn, p_dontexpand) == 26
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_dontexpand>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseNamespaceColumn>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 26;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParseNamespaceColumn, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParseNamespaceColumn> as c::expression::OrdinaryField<Field_p_dontexpand>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).p_dontexpand) };
#[doc(hidden)]
pub struct Field_p_rtindex;
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ParseNamespaceItem>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ParseNamespaceItem>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ParseNamespaceItem {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseNamespaceItem, p_rtindex) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_rtindex>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseNamespaceItem>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParseNamespaceItem, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParseNamespaceItem> as c::expression::OrdinaryField<Field_p_rtindex>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).p_rtindex) };
#[doc(hidden)]
pub struct Field_p_rel_visible;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseNamespaceItem, p_rel_visible) == 32
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_rel_visible>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseNamespaceItem>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParseNamespaceItem, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParseNamespaceItem> as c::expression::OrdinaryField<Field_p_rel_visible>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).p_rel_visible) };
#[doc(hidden)]
pub struct Field_p_cols_visible;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseNamespaceItem, p_cols_visible) == 33
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_cols_visible>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseNamespaceItem>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 33;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParseNamespaceItem, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParseNamespaceItem> as c::expression::OrdinaryField<Field_p_cols_visible>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).p_cols_visible) };
#[doc(hidden)]
pub struct Field_p_lateral_only;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseNamespaceItem, p_lateral_only) == 34
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_lateral_only>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseNamespaceItem>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 34;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParseNamespaceItem, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParseNamespaceItem> as c::expression::OrdinaryField<Field_p_lateral_only>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).p_lateral_only) };
#[doc(hidden)]
pub struct Field_p_lateral_ok;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseNamespaceItem, p_lateral_ok) == 35
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_lateral_ok>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseNamespaceItem>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 35;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParseNamespaceItem, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParseNamespaceItem> as c::expression::OrdinaryField<Field_p_lateral_ok>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).p_lateral_ok) };
#[doc(hidden)]
pub struct Field_p_rtable;
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ParseState>() == 208);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ParseState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ParseState {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_rtable) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_rtable>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParseState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParseState> as c::expression::OrdinaryField<Field_p_rtable>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).p_rtable) };
#[doc(hidden)]
pub struct Field_p_joinexprs;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_joinexprs) == 24);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_joinexprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParseState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParseState> as c::expression::OrdinaryField<Field_p_joinexprs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).p_joinexprs) };
#[doc(hidden)]
pub struct Field_p_joinlist;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_joinlist) == 32);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_joinlist>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParseState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParseState> as c::expression::OrdinaryField<Field_p_joinlist>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).p_joinlist) };
#[doc(hidden)]
pub struct Field_p_namespace;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_namespace) == 40);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_namespace>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParseState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParseState> as c::expression::OrdinaryField<Field_p_namespace>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).p_namespace) };
#[doc(hidden)]
pub struct Field_p_lateral_active;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_lateral_active) == 48);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_lateral_active>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParseState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParseState> as c::expression::OrdinaryField<Field_p_lateral_active>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).p_lateral_active) };
#[doc(hidden)]
pub struct Field_p_ctenamespace;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_ctenamespace) == 56);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_ctenamespace>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParseState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParseState> as c::expression::OrdinaryField<Field_p_ctenamespace>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).p_ctenamespace) };
#[doc(hidden)]
pub struct Field_p_future_ctes;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_future_ctes) == 64);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_future_ctes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParseState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParseState> as c::expression::OrdinaryField<Field_p_future_ctes>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).p_future_ctes) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
pub struct Field_p_is_insert;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_is_insert) == 96);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_is_insert>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParseState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParseState> as c::expression::OrdinaryField<Field_p_is_insert>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).p_is_insert) };
#[doc(hidden)]
pub struct Field_p_windowdefs;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_windowdefs) == 104);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_windowdefs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParseState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParseState> as c::expression::OrdinaryField<Field_p_windowdefs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).p_windowdefs) };
#[doc(hidden)]
pub struct Field_p_expr_kind;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_expr_kind) == 112);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_expr_kind>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ParseExprKind,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParseState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParseState> as c::expression::OrdinaryField<Field_p_expr_kind>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).p_expr_kind) };
#[doc(hidden)]
pub struct Field_p_next_resno;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_next_resno) == 116);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_next_resno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 116;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParseState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParseState> as c::expression::OrdinaryField<Field_p_next_resno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).p_next_resno) };
#[doc(hidden)]
pub struct Field_p_multiassign_exprs;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_multiassign_exprs) == 120
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_multiassign_exprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParseState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParseState> as c::expression::OrdinaryField<Field_p_multiassign_exprs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).p_multiassign_exprs) };
#[doc(hidden)]
pub struct Field_p_locking_clause;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_locking_clause) == 128);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_locking_clause>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParseState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParseState> as c::expression::OrdinaryField<Field_p_locking_clause>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).p_locking_clause) };
#[doc(hidden)]
pub struct Field_p_locked_from_parent;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_locked_from_parent) == 136
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_locked_from_parent>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParseState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParseState> as c::expression::OrdinaryField<Field_p_locked_from_parent>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).p_locked_from_parent) };
#[doc(hidden)]
pub struct Field_p_resolve_unknowns;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_resolve_unknowns) == 137
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_resolve_unknowns>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 137;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParseState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParseState> as c::expression::OrdinaryField<Field_p_resolve_unknowns>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).p_resolve_unknowns) };
#[doc(hidden)]
pub struct Field_p_hasAggs;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_hasAggs) == 152);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_hasAggs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParseState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParseState> as c::expression::OrdinaryField<Field_p_hasAggs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).p_hasAggs) };
#[doc(hidden)]
pub struct Field_p_hasWindowFuncs;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_hasWindowFuncs) == 153);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_hasWindowFuncs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 153;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParseState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParseState> as c::expression::OrdinaryField<Field_p_hasWindowFuncs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).p_hasWindowFuncs) };
#[doc(hidden)]
pub struct Field_p_hasTargetSRFs;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_hasTargetSRFs) == 154);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_hasTargetSRFs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 154;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParseState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParseState> as c::expression::OrdinaryField<Field_p_hasTargetSRFs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).p_hasTargetSRFs) };
#[doc(hidden)]
pub struct Field_p_hasSubLinks;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_hasSubLinks) == 155);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_hasSubLinks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 155;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParseState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParseState> as c::expression::OrdinaryField<Field_p_hasSubLinks>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).p_hasSubLinks) };
#[doc(hidden)]
pub struct Field_p_hasModifyingCTE;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_hasModifyingCTE) == 156
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_hasModifyingCTE>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 156;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParseState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParseState> as c::expression::OrdinaryField<Field_p_hasModifyingCTE>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).p_hasModifyingCTE) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PartitionBoundInfoData>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PartitionBoundInfoData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PartitionBoundInfoData {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionBoundInfoData, strategy) == 0
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_strategy>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionBoundInfoData>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionBoundInfoData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionBoundInfoData> as c::expression::OrdinaryField<Field_strategy>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).strategy) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionBoundInfoData, ndatums) == 4
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ndatums>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionBoundInfoData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionBoundInfoData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionBoundInfoData> as c::expression::OrdinaryField<Field_ndatums>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ndatums) };
#[doc(hidden)]
pub struct Field_nindexes;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionBoundInfoData, nindexes) == 32
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nindexes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionBoundInfoData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionBoundInfoData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionBoundInfoData> as c::expression::OrdinaryField<Field_nindexes>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nindexes) };
#[doc(hidden)]
pub struct Field_null_index;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionBoundInfoData, null_index) == 48
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_null_index>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionBoundInfoData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionBoundInfoData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionBoundInfoData> as c::expression::OrdinaryField<Field_null_index>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).null_index) };
#[doc(hidden)]
pub struct Field_default_index;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionBoundInfoData, default_index) == 52
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_default_index>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionBoundInfoData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionBoundInfoData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionBoundInfoData> as c::expression::OrdinaryField<Field_default_index>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).default_index) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PartitionBoundSpec>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PartitionBoundSpec>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PartitionBoundSpec {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionBoundSpec, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionBoundSpec>
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
const _: Projection<crate::__pgrx_c_bindings::PartitionBoundSpec, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionBoundSpec> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionBoundSpec, strategy) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_strategy>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionBoundSpec>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionBoundSpec, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionBoundSpec> as c::expression::OrdinaryField<Field_strategy>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).strategy) };
#[doc(hidden)]
pub struct Field_is_default;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionBoundSpec, is_default) == 5);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_default>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionBoundSpec>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 5;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionBoundSpec, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionBoundSpec> as c::expression::OrdinaryField<Field_is_default>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_default) };
#[doc(hidden)]
pub struct Field_modulus;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionBoundSpec, modulus) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_modulus>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionBoundSpec>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionBoundSpec, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionBoundSpec> as c::expression::OrdinaryField<Field_modulus>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).modulus) };
#[doc(hidden)]
pub struct Field_remainder;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionBoundSpec, remainder) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_remainder>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionBoundSpec>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionBoundSpec, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionBoundSpec> as c::expression::OrdinaryField<Field_remainder>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).remainder) };
#[doc(hidden)]
pub struct Field_listdatums;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionBoundSpec, listdatums) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_listdatums>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionBoundSpec>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionBoundSpec, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionBoundSpec> as c::expression::OrdinaryField<Field_listdatums>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).listdatums) };
#[doc(hidden)]
pub struct Field_lowerdatums;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionBoundSpec, lowerdatums) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lowerdatums>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionBoundSpec>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionBoundSpec, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionBoundSpec> as c::expression::OrdinaryField<Field_lowerdatums>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lowerdatums) };
#[doc(hidden)]
pub struct Field_upperdatums;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionBoundSpec, upperdatums) == 32
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_upperdatums>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionBoundSpec>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionBoundSpec, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionBoundSpec> as c::expression::OrdinaryField<Field_upperdatums>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).upperdatums) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionBoundSpec, location) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionBoundSpec>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionBoundSpec, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionBoundSpec> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PartitionCmd>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PartitionCmd>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PartitionCmd {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionCmd, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionCmd>
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
const _: Projection<crate::__pgrx_c_bindings::PartitionCmd, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionCmd> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionCmd, concurrent) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_concurrent>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionCmd>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionCmd, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionCmd> as c::expression::OrdinaryField<Field_concurrent>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).concurrent) };
#[doc(hidden)]
pub struct Field_nparts;
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PartitionDescData>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PartitionDescData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PartitionDescData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionDescData, nparts) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nparts>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionDescData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionDescData> as c::expression::OrdinaryField<Field_nparts>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nparts) };
#[doc(hidden)]
pub struct Field_detached_exist;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionDescData, detached_exist) == 4
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_detached_exist>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionDescData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionDescData> as c::expression::OrdinaryField<Field_detached_exist>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).detached_exist) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PartitionElem>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PartitionElem>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PartitionElem {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionElem, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionElem>
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
const _: Projection<crate::__pgrx_c_bindings::PartitionElem, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionElem> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionElem, collation) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_collation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionElem>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionElem, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionElem> as c::expression::OrdinaryField<Field_collation>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).collation) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionElem, opclass) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_opclass>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionElem>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionElem, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionElem> as c::expression::OrdinaryField<Field_opclass>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).opclass) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionElem, location) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionElem>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionElem, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionElem> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PartitionPruneContext>() == 80);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PartitionPruneContext>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PartitionPruneContext {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionPruneContext, strategy) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_strategy>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionPruneContext>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionPruneContext, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionPruneContext> as c::expression::OrdinaryField<Field_strategy>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).strategy) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionPruneContext, partnatts) == 4
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_partnatts>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionPruneContext>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionPruneContext, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionPruneContext> as c::expression::OrdinaryField<Field_partnatts>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).partnatts) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionPruneContext, nparts) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nparts>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionPruneContext>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionPruneContext, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionPruneContext> as c::expression::OrdinaryField<Field_nparts>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nparts) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PartitionPruneInfo>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PartitionPruneInfo>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PartitionPruneInfo {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionPruneInfo, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionPruneInfo>
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
const _: Projection<crate::__pgrx_c_bindings::PartitionPruneInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionPruneInfo> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_prune_infos;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionPruneInfo, prune_infos) == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_prune_infos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionPruneInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionPruneInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionPruneInfo> as c::expression::OrdinaryField<Field_prune_infos>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).prune_infos) };
#[doc(hidden)]
pub struct Field_do_initial_prune;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PartitionPruneState>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PartitionPruneState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PartitionPruneState {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionPruneState, do_initial_prune) == 24
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_do_initial_prune>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionPruneState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionPruneState, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionPruneState> as c::expression::OrdinaryField<Field_do_initial_prune>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).do_initial_prune) };
#[doc(hidden)]
pub struct Field_do_exec_prune;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionPruneState, do_exec_prune) == 25
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_do_exec_prune>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionPruneState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 25;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionPruneState, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionPruneState> as c::expression::OrdinaryField<Field_do_exec_prune>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).do_exec_prune) };
#[doc(hidden)]
pub struct Field_num_partprunedata;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionPruneState, num_partprunedata) == 28
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_partprunedata>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionPruneState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionPruneState, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionPruneState> as c::expression::OrdinaryField<Field_num_partprunedata>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_partprunedata) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PartitionPruneStep>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PartitionPruneStep>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PartitionPruneStep {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionPruneStep, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionPruneStep>
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
const _: Projection<crate::__pgrx_c_bindings::PartitionPruneStep, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionPruneStep> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_step_id;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionPruneStep, step_id) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_step_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionPruneStep>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionPruneStep, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionPruneStep> as c::expression::OrdinaryField<Field_step_id>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).step_id) };
#[doc(hidden)]
pub struct Field_combineOp;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PartitionPruneStepCombine>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PartitionPruneStepCombine>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PartitionPruneStepCombine {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionPruneStepCombine, combineOp) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_combineOp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionPruneStepCombine>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PartitionPruneCombineOp,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionPruneStepCombine, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionPruneStepCombine> as c::expression::OrdinaryField<Field_combineOp>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).combineOp) };
#[doc(hidden)]
pub struct Field_source_stepids;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionPruneStepCombine, source_stepids)
        == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_source_stepids>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionPruneStepCombine>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionPruneStepCombine, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionPruneStepCombine> as c::expression::OrdinaryField<Field_source_stepids>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).source_stepids) };
#[doc(hidden)]
pub struct Field_opstrategy;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PartitionPruneStepOp>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PartitionPruneStepOp>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PartitionPruneStepOp {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionPruneStepOp, opstrategy) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_opstrategy>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionPruneStepOp>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionPruneStepOp, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionPruneStepOp> as c::expression::OrdinaryField<Field_opstrategy>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).opstrategy) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionPruneStepOp, exprs) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_exprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionPruneStepOp>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionPruneStepOp, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionPruneStepOp> as c::expression::OrdinaryField<Field_exprs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).exprs) };
#[doc(hidden)]
pub struct Field_cmpfns;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionPruneStepOp, cmpfns) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cmpfns>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionPruneStepOp>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionPruneStepOp, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionPruneStepOp> as c::expression::OrdinaryField<Field_cmpfns>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cmpfns) };
#[doc(hidden)]
pub struct Field_num_partrelprunedata;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PartitionPruningData>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PartitionPruningData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PartitionPruningData {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionPruningData, num_partrelprunedata)
        == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_partrelprunedata>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionPruningData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionPruningData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionPruningData> as c::expression::OrdinaryField<Field_num_partrelprunedata>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_partrelprunedata) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PartitionRangeDatum>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PartitionRangeDatum>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PartitionRangeDatum {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionRangeDatum, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionRangeDatum>
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
const _: Projection<crate::__pgrx_c_bindings::PartitionRangeDatum, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionRangeDatum> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionRangeDatum, kind) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_kind>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionRangeDatum>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PartitionRangeDatumKind,
        c::CInt,
        i32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionRangeDatum, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionRangeDatum> as c::expression::OrdinaryField<Field_kind>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).kind) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionRangeDatum, value) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_value>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionRangeDatum>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::Node>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionRangeDatum, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionRangeDatum> as c::expression::OrdinaryField<Field_value>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).value) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionRangeDatum, location) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionRangeDatum>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionRangeDatum, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionRangeDatum> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PartitionSchemeData>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PartitionSchemeData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PartitionSchemeData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionSchemeData, strategy) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_strategy>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionSchemeData>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionSchemeData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionSchemeData> as c::expression::OrdinaryField<Field_strategy>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).strategy) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionSchemeData, partnatts) == 2);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_partnatts>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionSchemeData>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 2;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionSchemeData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionSchemeData> as c::expression::OrdinaryField<Field_partnatts>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).partnatts) };
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PartitionSpec>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PartitionSpec>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PartitionSpec {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionSpec, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionSpec>
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
const _: Projection<crate::__pgrx_c_bindings::PartitionSpec, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionSpec> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_partParams;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionSpec, partParams) == 16);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_partParams>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionSpec>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionSpec, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionSpec> as c::expression::OrdinaryField<Field_partParams>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).partParams) };
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionSpec, location) == 24);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionSpec>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionSpec, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionSpec> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PartitionedRelPruneInfo>() == 72);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PartitionedRelPruneInfo>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PartitionedRelPruneInfo {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionedRelPruneInfo, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionedRelPruneInfo>
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
const _: Projection<crate::__pgrx_c_bindings::PartitionedRelPruneInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionedRelPruneInfo> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionedRelPruneInfo, rtindex) == 4
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rtindex>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionedRelPruneInfo>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionedRelPruneInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionedRelPruneInfo> as c::expression::OrdinaryField<Field_rtindex>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rtindex) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionedRelPruneInfo, nparts) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nparts>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionedRelPruneInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionedRelPruneInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionedRelPruneInfo> as c::expression::OrdinaryField<Field_nparts>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nparts) };
#[doc(hidden)]
pub struct Field_initial_pruning_steps;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::PartitionedRelPruneInfo,
        initial_pruning_steps
    ) == 48
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_initial_pruning_steps>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionedRelPruneInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionedRelPruneInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionedRelPruneInfo> as c::expression::OrdinaryField<Field_initial_pruning_steps>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).initial_pruning_steps) };
#[doc(hidden)]
pub struct Field_exec_pruning_steps;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionedRelPruneInfo, exec_pruning_steps)
        == 56
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_exec_pruning_steps>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionedRelPruneInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionedRelPruneInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionedRelPruneInfo> as c::expression::OrdinaryField<Field_exec_pruning_steps>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).exec_pruning_steps) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PartitionedRelPruningData>() == 208);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PartitionedRelPruningData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PartitionedRelPruningData {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionedRelPruningData, nparts) == 0
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nparts>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionedRelPruningData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionedRelPruningData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionedRelPruningData> as c::expression::OrdinaryField<Field_nparts>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nparts) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::PartitionedRelPruningData,
        initial_pruning_steps
    ) == 32
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_initial_pruning_steps>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionedRelPruningData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionedRelPruningData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionedRelPruningData> as c::expression::OrdinaryField<Field_initial_pruning_steps>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).initial_pruning_steps) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::PartitionedRelPruningData,
        exec_pruning_steps
    ) == 40
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_exec_pruning_steps>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionedRelPruningData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PartitionedRelPruningData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PartitionedRelPruningData> as c::expression::OrdinaryField<Field_exec_pruning_steps>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).exec_pruning_steps) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Path>() == 72);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Path>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::Path {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Path, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Path>
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
    crate::__pgrx_c_bindings::Path,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Path> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_pathtype;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Path, pathtype) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pathtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Path>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Path,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Path> as c::expression::OrdinaryField<
        Field_pathtype,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pathtype) };
#[doc(hidden)]
pub struct Field_param_info;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Path, param_info) == 24);
const _: () = {
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::ParamPathInfo>() == 8);
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::ParamPathInfo>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_param_info>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Path>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::ParamPathInfo>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Path,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Path> as c::expression::OrdinaryField<
        Field_param_info,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).param_info) };
#[doc(hidden)]
pub struct Field_parallel_aware;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Path, parallel_aware) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parallel_aware>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Path>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Path,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Path> as c::expression::OrdinaryField<
        Field_parallel_aware,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).parallel_aware) };
#[doc(hidden)]
pub struct Field_parallel_safe;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Path, parallel_safe) == 33);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parallel_safe>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Path>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 33;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Path,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Path> as c::expression::OrdinaryField<
        Field_parallel_safe,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).parallel_safe) };
#[doc(hidden)]
pub struct Field_parallel_workers;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Path, parallel_workers) == 36);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parallel_workers>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Path>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Path,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Path> as c::expression::OrdinaryField<
        Field_parallel_workers,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).parallel_workers) };
#[doc(hidden)]
pub struct Field_pathkeys;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Path, pathkeys) == 64);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pathkeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Path>
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
    crate::__pgrx_c_bindings::Path,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Path> as c::expression::OrdinaryField<
        Field_pathkeys,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pathkeys) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PathKey>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PathKey>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PathKey {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PathKey, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PathKey>
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
    crate::__pgrx_c_bindings::PathKey,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PathKey> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_pk_opfamily;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PathKey, pk_opfamily) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pk_opfamily>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PathKey>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PathKey,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PathKey> as c::expression::OrdinaryField<
        Field_pk_opfamily,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pk_opfamily) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
#[doc(hidden)]
pub struct Field_pk_strategy;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PathKey, pk_strategy) == 20);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pk_strategy>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PathKey>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PathKey,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PathKey> as c::expression::OrdinaryField<
        Field_pk_strategy,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pk_strategy) };
#[doc(hidden)]
pub struct Field_pk_nulls_first;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PathKey, pk_nulls_first) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pk_nulls_first>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PathKey>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PathKey,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PathKey> as c::expression::OrdinaryField<
        Field_pk_nulls_first,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pk_nulls_first) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PathTarget>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PathTarget>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PathTarget {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PathTarget, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PathTarget>
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
const _: Projection<crate::__pgrx_c_bindings::PathTarget, <<c::expression::CRecord<crate::__pgrx_c_bindings::PathTarget> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PathTarget, exprs) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_exprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PathTarget>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PathTarget, <<c::expression::CRecord<crate::__pgrx_c_bindings::PathTarget> as c::expression::OrdinaryField<Field_exprs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).exprs) };
#[doc(hidden)]
pub struct Field_sortgrouprefs;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PathTarget, sortgrouprefs) == 16);
const _: () = {
    assert!(::core::mem::size_of::<*mut u32>() == 8);
    assert!(::core::mem::align_of::<*mut u32>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sortgrouprefs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PathTarget>
{
    type Member = c::expression::CPointer<c::CUnsignedInt, c::expression::ReadWrite>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PathTarget, <<c::expression::CRecord<crate::__pgrx_c_bindings::PathTarget> as c::expression::OrdinaryField<Field_sortgrouprefs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sortgrouprefs) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PathTarget, width) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_width>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PathTarget>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PathTarget, <<c::expression::CRecord<crate::__pgrx_c_bindings::PathTarget> as c::expression::OrdinaryField<Field_width>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).width) };
#[doc(hidden)]
pub struct Field_has_volatile_expr;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PathTarget, has_volatile_expr) == 44);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_has_volatile_expr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PathTarget>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__VolatileFunctionStatus,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PathTarget, <<c::expression::CRecord<crate::__pgrx_c_bindings::PathTarget> as c::expression::OrdinaryField<Field_has_volatile_expr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).has_volatile_expr) };
#[doc(hidden)]
pub struct Field_gss_auth;
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgBackendGSSStatus>() == 66);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgBackendGSSStatus>() == 1);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgBackendGSSStatus {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgBackendGSSStatus, gss_auth) == 64);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_gss_auth>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgBackendGSSStatus>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgBackendGSSStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgBackendGSSStatus> as c::expression::OrdinaryField<Field_gss_auth>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).gss_auth) };
#[doc(hidden)]
pub struct Field_gss_enc;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgBackendGSSStatus, gss_enc) == 65);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_gss_enc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgBackendGSSStatus>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 65;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgBackendGSSStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgBackendGSSStatus> as c::expression::OrdinaryField<Field_gss_enc>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).gss_enc) };
#[doc(hidden)]
pub struct Field_ssl_bits;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgBackendSSLStatus>() == 324);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgBackendSSLStatus>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgBackendSSLStatus {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgBackendSSLStatus, ssl_bits) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ssl_bits>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgBackendSSLStatus>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgBackendSSLStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgBackendSSLStatus> as c::expression::OrdinaryField<Field_ssl_bits>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ssl_bits) };
#[doc(hidden)]
pub struct Field_st_changecount;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgBackendStatus>() == 432);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgBackendStatus>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgBackendStatus {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgBackendStatus, st_changecount) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_st_changecount>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgBackendStatus>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgBackendStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgBackendStatus> as c::expression::OrdinaryField<Field_st_changecount>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).st_changecount) };
#[doc(hidden)]
pub struct Field_st_procpid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgBackendStatus, st_procpid) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_st_procpid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgBackendStatus>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgBackendStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgBackendStatus> as c::expression::OrdinaryField<Field_st_procpid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).st_procpid) };
#[doc(hidden)]
pub struct Field_st_backendType;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgBackendStatus, st_backendType) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_st_backendType>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgBackendStatus>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__BackendType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgBackendStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgBackendStatus> as c::expression::OrdinaryField<Field_st_backendType>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).st_backendType) };
#[doc(hidden)]
pub struct Field_st_proc_start_timestamp;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgBackendStatus, st_proc_start_timestamp)
        == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_st_proc_start_timestamp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgBackendStatus>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgBackendStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgBackendStatus> as c::expression::OrdinaryField<Field_st_proc_start_timestamp>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).st_proc_start_timestamp) };
#[doc(hidden)]
pub struct Field_st_xact_start_timestamp;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgBackendStatus, st_xact_start_timestamp)
        == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_st_xact_start_timestamp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgBackendStatus>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgBackendStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgBackendStatus> as c::expression::OrdinaryField<Field_st_xact_start_timestamp>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).st_xact_start_timestamp) };
#[doc(hidden)]
pub struct Field_st_activity_start_timestamp;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgBackendStatus, st_activity_start_timestamp)
        == 32
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_st_activity_start_timestamp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgBackendStatus>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgBackendStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgBackendStatus> as c::expression::OrdinaryField<Field_st_activity_start_timestamp>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).st_activity_start_timestamp) };
#[doc(hidden)]
pub struct Field_st_state_start_timestamp;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgBackendStatus, st_state_start_timestamp)
        == 40
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_st_state_start_timestamp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgBackendStatus>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgBackendStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgBackendStatus> as c::expression::OrdinaryField<Field_st_state_start_timestamp>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).st_state_start_timestamp) };
#[doc(hidden)]
pub struct Field_st_databaseid;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgBackendStatus, st_databaseid) == 48
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_st_databaseid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgBackendStatus>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgBackendStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgBackendStatus> as c::expression::OrdinaryField<Field_st_databaseid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).st_databaseid) };
#[doc(hidden)]
pub struct Field_st_userid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgBackendStatus, st_userid) == 52);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_st_userid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgBackendStatus>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgBackendStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgBackendStatus> as c::expression::OrdinaryField<Field_st_userid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).st_userid) };
#[doc(hidden)]
pub struct Field_st_ssl;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgBackendStatus, st_ssl) == 200);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_st_ssl>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgBackendStatus>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 200;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgBackendStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgBackendStatus> as c::expression::OrdinaryField<Field_st_ssl>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).st_ssl) };
#[doc(hidden)]
pub struct Field_st_gss;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgBackendStatus, st_gss) == 216);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_st_gss>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgBackendStatus>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 216;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgBackendStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgBackendStatus> as c::expression::OrdinaryField<Field_st_gss>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).st_gss) };
#[doc(hidden)]
pub struct Field_st_state;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgBackendStatus, st_state) == 232);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_st_state>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgBackendStatus>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__BackendState,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 232;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgBackendStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgBackendStatus> as c::expression::OrdinaryField<Field_st_state>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).st_state) };
#[doc(hidden)]
pub struct Field_st_progress_command;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgBackendStatus, st_progress_command) == 256
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_st_progress_command>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgBackendStatus>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ProgressCommandType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 256;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgBackendStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgBackendStatus> as c::expression::OrdinaryField<Field_st_progress_command>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).st_progress_command) };
#[doc(hidden)]
pub struct Field_st_progress_command_target;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgBackendStatus, st_progress_command_target)
        == 260
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_st_progress_command_target>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgBackendStatus>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 260;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgBackendStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgBackendStatus> as c::expression::OrdinaryField<Field_st_progress_command_target>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).st_progress_command_target) };
#[doc(hidden)]
pub struct Field_st_query_id;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgBackendStatus, st_query_id) == 424);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_st_query_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgBackendStatus>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 424;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgBackendStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgBackendStatus> as c::expression::OrdinaryField<Field_st_query_id>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).st_query_id) };
#[doc(hidden)]
pub struct Field_archived_count;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_ArchiverStats>() == 136);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_ArchiverStats>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgStat_ArchiverStats {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_ArchiverStats, archived_count) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_archived_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_ArchiverStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_ArchiverStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_ArchiverStats> as c::expression::OrdinaryField<Field_archived_count>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).archived_count) };
#[doc(hidden)]
pub struct Field_last_archived_timestamp;
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::PgStat_ArchiverStats,
        last_archived_timestamp
    ) == 56
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_last_archived_timestamp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_ArchiverStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_ArchiverStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_ArchiverStats> as c::expression::OrdinaryField<Field_last_archived_timestamp>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).last_archived_timestamp) };
#[doc(hidden)]
pub struct Field_failed_count;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_ArchiverStats, failed_count) == 64
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_failed_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_ArchiverStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_ArchiverStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_ArchiverStats> as c::expression::OrdinaryField<Field_failed_count>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).failed_count) };
#[doc(hidden)]
pub struct Field_last_failed_timestamp;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_ArchiverStats, last_failed_timestamp)
        == 120
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_last_failed_timestamp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_ArchiverStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_ArchiverStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_ArchiverStats> as c::expression::OrdinaryField<Field_last_failed_timestamp>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).last_failed_timestamp) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_ArchiverStats, stat_reset_timestamp)
        == 128
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stat_reset_timestamp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_ArchiverStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_ArchiverStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_ArchiverStats> as c::expression::OrdinaryField<Field_stat_reset_timestamp>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stat_reset_timestamp) };
#[doc(hidden)]
pub struct Field_apply_error_count;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_BackendSubEntry>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_BackendSubEntry>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgStat_BackendSubEntry {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_BackendSubEntry, apply_error_count)
        == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_apply_error_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_BackendSubEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_BackendSubEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_BackendSubEntry> as c::expression::OrdinaryField<Field_apply_error_count>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).apply_error_count) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
pub struct Field_sync_error_count;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_BackendSubEntry, sync_error_count)
        == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sync_error_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_BackendSubEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_BackendSubEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_BackendSubEntry> as c::expression::OrdinaryField<Field_sync_error_count>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sync_error_count) };
#[doc(hidden)]
pub struct Field_buf_written_clean;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_BgWriterStats>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_BgWriterStats>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgStat_BgWriterStats {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_BgWriterStats, buf_written_clean) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_buf_written_clean>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_BgWriterStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_BgWriterStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_BgWriterStats> as c::expression::OrdinaryField<Field_buf_written_clean>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).buf_written_clean) };
#[doc(hidden)]
pub struct Field_maxwritten_clean;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_BgWriterStats, maxwritten_clean) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_maxwritten_clean>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_BgWriterStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_BgWriterStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_BgWriterStats> as c::expression::OrdinaryField<Field_maxwritten_clean>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).maxwritten_clean) };
#[doc(hidden)]
pub struct Field_buf_alloc;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_BgWriterStats, buf_alloc) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_buf_alloc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_BgWriterStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_BgWriterStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_BgWriterStats> as c::expression::OrdinaryField<Field_buf_alloc>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).buf_alloc) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_BgWriterStats, stat_reset_timestamp)
        == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stat_reset_timestamp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_BgWriterStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_BgWriterStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_BgWriterStats> as c::expression::OrdinaryField<Field_stat_reset_timestamp>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stat_reset_timestamp) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
#[doc(hidden)]
pub struct Field_timed_checkpoints;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_CheckpointerStats>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_CheckpointerStats>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgStat_CheckpointerStats {}
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_CheckpointerStats, timed_checkpoints)
        == 0
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_timed_checkpoints>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_CheckpointerStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats> as c::expression::OrdinaryField<Field_timed_checkpoints>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).timed_checkpoints) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
#[doc(hidden)]
pub struct Field_requested_checkpoints;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::PgStat_CheckpointerStats,
        requested_checkpoints
    ) == 8
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_requested_checkpoints>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_CheckpointerStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats> as c::expression::OrdinaryField<Field_requested_checkpoints>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).requested_checkpoints) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
#[doc(hidden)]
pub struct Field_checkpoint_write_time;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::PgStat_CheckpointerStats,
        checkpoint_write_time
    ) == 16
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_checkpoint_write_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_CheckpointerStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats> as c::expression::OrdinaryField<Field_checkpoint_write_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).checkpoint_write_time) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
#[doc(hidden)]
pub struct Field_checkpoint_sync_time;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::PgStat_CheckpointerStats,
        checkpoint_sync_time
    ) == 24
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_checkpoint_sync_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_CheckpointerStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats> as c::expression::OrdinaryField<Field_checkpoint_sync_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).checkpoint_sync_time) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
#[doc(hidden)]
pub struct Field_buf_written_checkpoints;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::PgStat_CheckpointerStats,
        buf_written_checkpoints
    ) == 32
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_buf_written_checkpoints>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_CheckpointerStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats> as c::expression::OrdinaryField<Field_buf_written_checkpoints>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).buf_written_checkpoints) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
#[doc(hidden)]
pub struct Field_buf_written_backend;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::PgStat_CheckpointerStats,
        buf_written_backend
    ) == 40
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_buf_written_backend>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_CheckpointerStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats> as c::expression::OrdinaryField<Field_buf_written_backend>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).buf_written_backend) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
#[doc(hidden)]
pub struct Field_buf_fsync_backend;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_CheckpointerStats, buf_fsync_backend)
        == 48
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_buf_fsync_backend>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_CheckpointerStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats> as c::expression::OrdinaryField<Field_buf_fsync_backend>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).buf_fsync_backend) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_f_numcalls;
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_FunctionCounts>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_FunctionCounts>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgStat_FunctionCounts {}
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_FunctionCounts, f_numcalls) == 0
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_f_numcalls>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_FunctionCounts>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_FunctionCounts, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_FunctionCounts> as c::expression::OrdinaryField<Field_f_numcalls>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).f_numcalls) };
#[doc(hidden)]
pub struct Field_blocks_zeroed;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_SLRUStats>() == 64);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_SLRUStats>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgStat_SLRUStats {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_SLRUStats, blocks_zeroed) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_blocks_zeroed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_SLRUStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_SLRUStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_SLRUStats> as c::expression::OrdinaryField<Field_blocks_zeroed>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).blocks_zeroed) };
#[doc(hidden)]
pub struct Field_blocks_hit;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_SLRUStats, blocks_hit) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_blocks_hit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_SLRUStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_SLRUStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_SLRUStats> as c::expression::OrdinaryField<Field_blocks_hit>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).blocks_hit) };
#[doc(hidden)]
pub struct Field_blocks_read;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_SLRUStats, blocks_read) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_blocks_read>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_SLRUStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_SLRUStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_SLRUStats> as c::expression::OrdinaryField<Field_blocks_read>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).blocks_read) };
#[doc(hidden)]
pub struct Field_blocks_written;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_SLRUStats, blocks_written) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_blocks_written>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_SLRUStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_SLRUStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_SLRUStats> as c::expression::OrdinaryField<Field_blocks_written>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).blocks_written) };
#[doc(hidden)]
pub struct Field_blocks_exists;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_SLRUStats, blocks_exists) == 32
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_blocks_exists>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_SLRUStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_SLRUStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_SLRUStats> as c::expression::OrdinaryField<Field_blocks_exists>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).blocks_exists) };
#[doc(hidden)]
pub struct Field_flush;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_SLRUStats, flush) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_flush>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_SLRUStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_SLRUStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_SLRUStats> as c::expression::OrdinaryField<Field_flush>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).flush) };
#[doc(hidden)]
pub struct Field_truncate;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_SLRUStats, truncate) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_truncate>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_SLRUStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_SLRUStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_SLRUStats> as c::expression::OrdinaryField<Field_truncate>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).truncate) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_SLRUStats, stat_reset_timestamp) == 56
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stat_reset_timestamp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_SLRUStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_SLRUStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_SLRUStats> as c::expression::OrdinaryField<Field_stat_reset_timestamp>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stat_reset_timestamp) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_n_xact_commit;
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_StatDBEntry>() == 240);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_StatDBEntry>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgStat_StatDBEntry {}
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatDBEntry, n_xact_commit) == 0
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_n_xact_commit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatDBEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry> as c::expression::OrdinaryField<Field_n_xact_commit>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).n_xact_commit) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_n_xact_rollback;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatDBEntry, n_xact_rollback) == 8
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_n_xact_rollback>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatDBEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry> as c::expression::OrdinaryField<Field_n_xact_rollback>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).n_xact_rollback) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_n_blocks_fetched;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatDBEntry, n_blocks_fetched) == 16
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_n_blocks_fetched>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatDBEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry> as c::expression::OrdinaryField<Field_n_blocks_fetched>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).n_blocks_fetched) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_n_blocks_hit;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatDBEntry, n_blocks_hit) == 24
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_n_blocks_hit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatDBEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry> as c::expression::OrdinaryField<Field_n_blocks_hit>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).n_blocks_hit) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_n_tuples_returned;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatDBEntry, n_tuples_returned) == 32
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_n_tuples_returned>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatDBEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry> as c::expression::OrdinaryField<Field_n_tuples_returned>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).n_tuples_returned) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_n_tuples_fetched;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatDBEntry, n_tuples_fetched) == 40
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_n_tuples_fetched>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatDBEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry> as c::expression::OrdinaryField<Field_n_tuples_fetched>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).n_tuples_fetched) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_n_tuples_inserted;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatDBEntry, n_tuples_inserted) == 48
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_n_tuples_inserted>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatDBEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry> as c::expression::OrdinaryField<Field_n_tuples_inserted>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).n_tuples_inserted) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_n_tuples_updated;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatDBEntry, n_tuples_updated) == 56
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_n_tuples_updated>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatDBEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry> as c::expression::OrdinaryField<Field_n_tuples_updated>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).n_tuples_updated) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_n_tuples_deleted;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatDBEntry, n_tuples_deleted) == 64
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_n_tuples_deleted>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatDBEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry> as c::expression::OrdinaryField<Field_n_tuples_deleted>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).n_tuples_deleted) };
#[doc(hidden)]
pub struct Field_last_autovac_time;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatDBEntry, last_autovac_time) == 72
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_last_autovac_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatDBEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry> as c::expression::OrdinaryField<Field_last_autovac_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).last_autovac_time) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_n_conflict_tablespace;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatDBEntry, n_conflict_tablespace)
        == 80
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_n_conflict_tablespace>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatDBEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry> as c::expression::OrdinaryField<Field_n_conflict_tablespace>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).n_conflict_tablespace) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_n_conflict_lock;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatDBEntry, n_conflict_lock) == 88
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_n_conflict_lock>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatDBEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry> as c::expression::OrdinaryField<Field_n_conflict_lock>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).n_conflict_lock) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_n_conflict_snapshot;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatDBEntry, n_conflict_snapshot)
        == 96
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_n_conflict_snapshot>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatDBEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry> as c::expression::OrdinaryField<Field_n_conflict_snapshot>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).n_conflict_snapshot) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_n_conflict_bufferpin;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatDBEntry, n_conflict_bufferpin)
        == 104
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_n_conflict_bufferpin>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatDBEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry> as c::expression::OrdinaryField<Field_n_conflict_bufferpin>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).n_conflict_bufferpin) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_n_conflict_startup_deadlock;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::PgStat_StatDBEntry,
        n_conflict_startup_deadlock
    ) == 112
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_n_conflict_startup_deadlock>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatDBEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry> as c::expression::OrdinaryField<Field_n_conflict_startup_deadlock>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).n_conflict_startup_deadlock) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_n_temp_files;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatDBEntry, n_temp_files) == 120
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_n_temp_files>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatDBEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry> as c::expression::OrdinaryField<Field_n_temp_files>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).n_temp_files) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_n_temp_bytes;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatDBEntry, n_temp_bytes) == 128
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_n_temp_bytes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatDBEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry> as c::expression::OrdinaryField<Field_n_temp_bytes>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).n_temp_bytes) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_n_deadlocks;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatDBEntry, n_deadlocks) == 136
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_n_deadlocks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatDBEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry> as c::expression::OrdinaryField<Field_n_deadlocks>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).n_deadlocks) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_n_checksum_failures;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatDBEntry, n_checksum_failures)
        == 144
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_n_checksum_failures>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatDBEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry> as c::expression::OrdinaryField<Field_n_checksum_failures>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).n_checksum_failures) };
#[doc(hidden)]
pub struct Field_last_checksum_failure;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatDBEntry, last_checksum_failure)
        == 152
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_last_checksum_failure>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatDBEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry> as c::expression::OrdinaryField<Field_last_checksum_failure>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).last_checksum_failure) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_n_block_read_time;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatDBEntry, n_block_read_time) == 160
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_n_block_read_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatDBEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry> as c::expression::OrdinaryField<Field_n_block_read_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).n_block_read_time) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_n_block_write_time;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatDBEntry, n_block_write_time)
        == 168
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_n_block_write_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 168;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatDBEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry> as c::expression::OrdinaryField<Field_n_block_write_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).n_block_write_time) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_n_sessions;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatDBEntry, n_sessions) == 176
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_n_sessions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 176;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatDBEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry> as c::expression::OrdinaryField<Field_n_sessions>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).n_sessions) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_total_session_time;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatDBEntry, total_session_time)
        == 184
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_total_session_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 184;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatDBEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry> as c::expression::OrdinaryField<Field_total_session_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).total_session_time) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_total_active_time;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatDBEntry, total_active_time) == 192
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_total_active_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 192;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatDBEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry> as c::expression::OrdinaryField<Field_total_active_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).total_active_time) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_total_idle_in_xact_time;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatDBEntry, total_idle_in_xact_time)
        == 200
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_total_idle_in_xact_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 200;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatDBEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry> as c::expression::OrdinaryField<Field_total_idle_in_xact_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).total_idle_in_xact_time) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_n_sessions_abandoned;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatDBEntry, n_sessions_abandoned)
        == 208
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_n_sessions_abandoned>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 208;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatDBEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry> as c::expression::OrdinaryField<Field_n_sessions_abandoned>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).n_sessions_abandoned) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_n_sessions_fatal;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatDBEntry, n_sessions_fatal) == 216
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_n_sessions_fatal>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 216;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatDBEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry> as c::expression::OrdinaryField<Field_n_sessions_fatal>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).n_sessions_fatal) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_n_sessions_killed;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatDBEntry, n_sessions_killed) == 224
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_n_sessions_killed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 224;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatDBEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry> as c::expression::OrdinaryField<Field_n_sessions_killed>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).n_sessions_killed) };
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatDBEntry, stat_reset_timestamp)
        == 232
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stat_reset_timestamp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 232;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatDBEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry> as c::expression::OrdinaryField<Field_stat_reset_timestamp>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stat_reset_timestamp) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_StatFuncEntry>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_StatFuncEntry>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgStat_StatFuncEntry {}
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatFuncEntry, f_numcalls) == 0
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_f_numcalls>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatFuncEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatFuncEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatFuncEntry> as c::expression::OrdinaryField<Field_f_numcalls>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).f_numcalls) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_f_total_time;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatFuncEntry, f_total_time) == 8
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_f_total_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatFuncEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
