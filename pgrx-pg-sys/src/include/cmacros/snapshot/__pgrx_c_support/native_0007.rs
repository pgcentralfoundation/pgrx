// Shared generated C macro support.

#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nextMulti>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::MultiXactId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckPoint, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint> as c::expression::OrdinaryField<Field_nextMulti>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nextMulti) };
#[doc(hidden)]
pub struct Field_nextMultiOffset;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, nextMultiOffset) == 40);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nextMultiOffset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckPoint, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint> as c::expression::OrdinaryField<Field_nextMultiOffset>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nextMultiOffset) };
#[doc(hidden)]
pub struct Field_oldestXid;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, oldestXid) == 44);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldestXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckPoint, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint> as c::expression::OrdinaryField<Field_oldestXid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oldestXid) };
#[doc(hidden)]
pub struct Field_oldestXidDB;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, oldestXidDB) == 48);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldestXidDB>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckPoint, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint> as c::expression::OrdinaryField<Field_oldestXidDB>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oldestXidDB) };
#[doc(hidden)]
pub struct Field_oldestMulti;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, oldestMulti) == 52);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldestMulti>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::MultiXactId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckPoint, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint> as c::expression::OrdinaryField<Field_oldestMulti>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oldestMulti) };
#[doc(hidden)]
pub struct Field_oldestMultiDB;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, oldestMultiDB) == 56);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldestMultiDB>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckPoint, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint> as c::expression::OrdinaryField<Field_oldestMultiDB>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oldestMultiDB) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, time) == 64);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckPoint, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint> as c::expression::OrdinaryField<Field_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).time) };
#[doc(hidden)]
pub struct Field_oldestCommitTsXid;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, oldestCommitTsXid) == 72);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldestCommitTsXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckPoint, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint> as c::expression::OrdinaryField<Field_oldestCommitTsXid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oldestCommitTsXid) };
#[doc(hidden)]
pub struct Field_newestCommitTsXid;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, newestCommitTsXid) == 76);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_newestCommitTsXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 76;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckPoint, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint> as c::expression::OrdinaryField<Field_newestCommitTsXid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).newestCommitTsXid) };
#[doc(hidden)]
pub struct Field_oldestActiveXid;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, oldestActiveXid) == 80);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldestActiveXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckPoint, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint> as c::expression::OrdinaryField<Field_oldestActiveXid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oldestActiveXid) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CheckPointStmt>() == 4);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CheckPointStmt>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CheckPointStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPointStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPointStmt>
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
const _: Projection<crate::__pgrx_c_bindings::CheckPointStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckPointStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_ckpt_start_t;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CheckpointStatsData>() == 80);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CheckpointStatsData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CheckpointStatsData {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckpointStatsData, ckpt_start_t) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ckpt_start_t>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckpointStatsData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData> as c::expression::OrdinaryField<Field_ckpt_start_t>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ckpt_start_t) };
#[doc(hidden)]
pub struct Field_ckpt_write_t;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckpointStatsData, ckpt_write_t) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ckpt_write_t>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckpointStatsData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData> as c::expression::OrdinaryField<Field_ckpt_write_t>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ckpt_write_t) };
#[doc(hidden)]
pub struct Field_ckpt_sync_t;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckpointStatsData, ckpt_sync_t) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ckpt_sync_t>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckpointStatsData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData> as c::expression::OrdinaryField<Field_ckpt_sync_t>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ckpt_sync_t) };
#[doc(hidden)]
pub struct Field_ckpt_sync_end_t;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckpointStatsData, ckpt_sync_end_t) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ckpt_sync_end_t>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckpointStatsData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData> as c::expression::OrdinaryField<Field_ckpt_sync_end_t>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ckpt_sync_end_t) };
#[doc(hidden)]
pub struct Field_ckpt_end_t;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckpointStatsData, ckpt_end_t) == 32
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ckpt_end_t>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckpointStatsData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData> as c::expression::OrdinaryField<Field_ckpt_end_t>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ckpt_end_t) };
#[doc(hidden)]
pub struct Field_ckpt_bufs_written;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckpointStatsData, ckpt_bufs_written) == 40
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ckpt_bufs_written>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckpointStatsData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData> as c::expression::OrdinaryField<Field_ckpt_bufs_written>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ckpt_bufs_written) };
#[doc(hidden)]
pub struct Field_ckpt_segs_added;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckpointStatsData, ckpt_segs_added) == 44
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ckpt_segs_added>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckpointStatsData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData> as c::expression::OrdinaryField<Field_ckpt_segs_added>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ckpt_segs_added) };
#[doc(hidden)]
pub struct Field_ckpt_segs_removed;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckpointStatsData, ckpt_segs_removed) == 48
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ckpt_segs_removed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckpointStatsData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData> as c::expression::OrdinaryField<Field_ckpt_segs_removed>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ckpt_segs_removed) };
#[doc(hidden)]
pub struct Field_ckpt_segs_recycled;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckpointStatsData, ckpt_segs_recycled)
        == 52
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ckpt_segs_recycled>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckpointStatsData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData> as c::expression::OrdinaryField<Field_ckpt_segs_recycled>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ckpt_segs_recycled) };
#[doc(hidden)]
pub struct Field_ckpt_sync_rels;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckpointStatsData, ckpt_sync_rels) == 56
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ckpt_sync_rels>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckpointStatsData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData> as c::expression::OrdinaryField<Field_ckpt_sync_rels>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ckpt_sync_rels) };
#[doc(hidden)]
pub struct Field_ckpt_longest_sync;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckpointStatsData, ckpt_longest_sync) == 64
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ckpt_longest_sync>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckpointStatsData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData> as c::expression::OrdinaryField<Field_ckpt_longest_sync>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ckpt_longest_sync) };
#[doc(hidden)]
pub struct Field_ckpt_agg_sync_time;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckpointStatsData, ckpt_agg_sync_time)
        == 72
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ckpt_agg_sync_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckpointStatsData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckpointStatsData> as c::expression::OrdinaryField<Field_ckpt_agg_sync_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ckpt_agg_sync_time) };
#[doc(hidden)]
pub struct Field_tour2_position;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::City>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::City>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::City {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::City, tour2_position) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tour2_position>
    for c::expression::CRecord<crate::__pgrx_c_bindings::City>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::City,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::City> as c::expression::OrdinaryField<
        Field_tour2_position,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tour2_position) };
#[doc(hidden)]
pub struct Field_tour1_position;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::City, tour1_position) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tour1_position>
    for c::expression::CRecord<crate::__pgrx_c_bindings::City>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::City,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::City> as c::expression::OrdinaryField<
        Field_tour1_position,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tour1_position) };
#[doc(hidden)]
pub struct Field_used;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::City, used) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_used>
    for c::expression::CRecord<crate::__pgrx_c_bindings::City>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::City,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::City> as c::expression::OrdinaryField<
        Field_used,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).used) };
#[doc(hidden)]
pub struct Field_select_list;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::City, select_list) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_select_list>
    for c::expression::CRecord<crate::__pgrx_c_bindings::City>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::City,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::City> as c::expression::OrdinaryField<
        Field_select_list,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).select_list) };
#[doc(hidden)]
pub struct Field_tsId;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CkptSortItem>() == 20);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CkptSortItem>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CkptSortItem {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CkptSortItem, tsId) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tsId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CkptSortItem>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CkptSortItem, <<c::expression::CRecord<crate::__pgrx_c_bindings::CkptSortItem> as c::expression::OrdinaryField<Field_tsId>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tsId) };
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CkptSortItem, relNode) == 4);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relNode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CkptSortItem>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CkptSortItem, <<c::expression::CRecord<crate::__pgrx_c_bindings::CkptSortItem> as c::expression::OrdinaryField<Field_relNode>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relNode) };
#[doc(hidden)]
pub struct Field_forkNum;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CkptSortItem, forkNum) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_forkNum>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CkptSortItem>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ForkNumber,
        c::CInt,
        i32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CkptSortItem, <<c::expression::CRecord<crate::__pgrx_c_bindings::CkptSortItem> as c::expression::OrdinaryField<Field_forkNum>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).forkNum) };
#[doc(hidden)]
pub struct Field_blockNum;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CkptSortItem, blockNum) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_blockNum>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CkptSortItem>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CkptSortItem, <<c::expression::CRecord<crate::__pgrx_c_bindings::CkptSortItem> as c::expression::OrdinaryField<Field_blockNum>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).blockNum) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CkptSortItem, buf_id) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_buf_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CkptSortItem>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CkptSortItem, <<c::expression::CRecord<crate::__pgrx_c_bindings::CkptSortItem> as c::expression::OrdinaryField<Field_buf_id>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).buf_id) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ClosePortalStmt>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ClosePortalStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ClosePortalStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ClosePortalStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ClosePortalStmt>
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
const _: Projection<crate::__pgrx_c_bindings::ClosePortalStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::ClosePortalStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ClusterParams>() == 4);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ClusterParams>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ClusterParams {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ClusterParams, options) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ClusterParams>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ClusterParams, <<c::expression::CRecord<crate::__pgrx_c_bindings::ClusterParams> as c::expression::OrdinaryField<Field_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ClusterStmt>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ClusterStmt>() == 8);
};
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ClusterStmt {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ClusterStmt, type_) == 0);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ClusterStmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ClusterStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::ClusterStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_params;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ClusterStmt, params) == 24);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_params>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ClusterStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ClusterStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::ClusterStmt> as c::expression::OrdinaryField<Field_params>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).params) };
#[doc(hidden)]
pub struct Field_coalescetype;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CoalesceExpr>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CoalesceExpr>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CoalesceExpr {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CoalesceExpr, coalescetype) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_coalescetype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CoalesceExpr>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CoalesceExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::CoalesceExpr> as c::expression::OrdinaryField<Field_coalescetype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).coalescetype) };
#[doc(hidden)]
pub struct Field_coalescecollid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CoalesceExpr, coalescecollid) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_coalescecollid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CoalesceExpr>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CoalesceExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::CoalesceExpr> as c::expression::OrdinaryField<Field_coalescecollid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).coalescecollid) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CoalesceExpr, args) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_args>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CoalesceExpr>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CoalesceExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::CoalesceExpr> as c::expression::OrdinaryField<Field_args>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).args) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CoalesceExpr, location) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CoalesceExpr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CoalesceExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::CoalesceExpr> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CoerceToDomain>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CoerceToDomain>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CoerceToDomain {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CoerceToDomain, resulttype) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resulttype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CoerceToDomain>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CoerceToDomain, <<c::expression::CRecord<crate::__pgrx_c_bindings::CoerceToDomain> as c::expression::OrdinaryField<Field_resulttype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).resulttype) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CoerceToDomain, resulttypmod) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resulttypmod>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CoerceToDomain>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CoerceToDomain, <<c::expression::CRecord<crate::__pgrx_c_bindings::CoerceToDomain> as c::expression::OrdinaryField<Field_resulttypmod>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).resulttypmod) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CoerceToDomain, resultcollid) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resultcollid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CoerceToDomain>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CoerceToDomain, <<c::expression::CRecord<crate::__pgrx_c_bindings::CoerceToDomain> as c::expression::OrdinaryField<Field_resultcollid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).resultcollid) };
#[doc(hidden)]
pub struct Field_coercionformat;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CoerceToDomain, coercionformat) == 28
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_coercionformat>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CoerceToDomain>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__CoercionForm,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CoerceToDomain, <<c::expression::CRecord<crate::__pgrx_c_bindings::CoerceToDomain> as c::expression::OrdinaryField<Field_coercionformat>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).coercionformat) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CoerceToDomain, location) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CoerceToDomain>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CoerceToDomain, <<c::expression::CRecord<crate::__pgrx_c_bindings::CoerceToDomain> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CoerceToDomainValue>() == 20);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CoerceToDomainValue>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CoerceToDomainValue {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CoerceToDomainValue, typeId) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typeId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CoerceToDomainValue>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CoerceToDomainValue, <<c::expression::CRecord<crate::__pgrx_c_bindings::CoerceToDomainValue> as c::expression::OrdinaryField<Field_typeId>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typeId) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CoerceToDomainValue, typeMod) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typeMod>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CoerceToDomainValue>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CoerceToDomainValue, <<c::expression::CRecord<crate::__pgrx_c_bindings::CoerceToDomainValue> as c::expression::OrdinaryField<Field_typeMod>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typeMod) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CoerceToDomainValue, collation) == 12
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_collation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CoerceToDomainValue>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CoerceToDomainValue, <<c::expression::CRecord<crate::__pgrx_c_bindings::CoerceToDomainValue> as c::expression::OrdinaryField<Field_collation>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).collation) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CoerceToDomainValue, location) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CoerceToDomainValue>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CoerceToDomainValue, <<c::expression::CRecord<crate::__pgrx_c_bindings::CoerceToDomainValue> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CoerceViaIO>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CoerceViaIO>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CoerceViaIO {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CoerceViaIO, resulttype) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resulttype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CoerceViaIO>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CoerceViaIO, <<c::expression::CRecord<crate::__pgrx_c_bindings::CoerceViaIO> as c::expression::OrdinaryField<Field_resulttype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).resulttype) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CoerceViaIO, resultcollid) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resultcollid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CoerceViaIO>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CoerceViaIO, <<c::expression::CRecord<crate::__pgrx_c_bindings::CoerceViaIO> as c::expression::OrdinaryField<Field_resultcollid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).resultcollid) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CoerceViaIO, coerceformat) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_coerceformat>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CoerceViaIO>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__CoercionForm,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CoerceViaIO, <<c::expression::CRecord<crate::__pgrx_c_bindings::CoerceViaIO> as c::expression::OrdinaryField<Field_coerceformat>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).coerceformat) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CoerceViaIO, location) == 28);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CoerceViaIO>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CoerceViaIO, <<c::expression::CRecord<crate::__pgrx_c_bindings::CoerceViaIO> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CollateClause>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CollateClause>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CollateClause {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CollateClause, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CollateClause>
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
const _: Projection<crate::__pgrx_c_bindings::CollateClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::CollateClause> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CollateClause, collname) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_collname>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CollateClause>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CollateClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::CollateClause> as c::expression::OrdinaryField<Field_collname>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).collname) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CollateClause, location) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CollateClause>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CollateClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::CollateClause> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[doc(hidden)]
pub struct Field_collOid;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CollateExpr>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CollateExpr>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CollateExpr {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CollateExpr, collOid) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_collOid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CollateExpr>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CollateExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::CollateExpr> as c::expression::OrdinaryField<Field_collOid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).collOid) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CollateExpr, location) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CollateExpr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CollateExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::CollateExpr> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CollectedCommand>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CollectedCommand>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CollectedCommand {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CollectedCommand, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CollectedCommand>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__CollectedCommandType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CollectedCommand, <<c::expression::CRecord<crate::__pgrx_c_bindings::CollectedCommand> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_in_extension;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CollectedCommand, in_extension) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_in_extension>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CollectedCommand>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CollectedCommand, <<c::expression::CRecord<crate::__pgrx_c_bindings::CollectedCommand> as c::expression::OrdinaryField<Field_in_extension>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).in_extension) };
const _: () = {
    assert!(
        ::core::mem::size_of::<
            crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_2,
        >() == 16
    );
    assert!(
        ::core::mem::align_of::<
            crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_2,
        >() == 8
    );
};
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_2
{
}
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_2,
        objectId
    ) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_objectId>
    for c::expression::CRecord<
        crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_2,
    >
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_2, <<c::expression::CRecord<crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_2> as c::expression::OrdinaryField<Field_objectId>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).objectId) };
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_2,
        classId
    ) == 4
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_classId>
    for c::expression::CRecord<
        crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_2,
    >
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _:
    Projection<
        crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_2,
        <<c::expression::CRecord<
            crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_2,
        > as c::expression::OrdinaryField<Field_classId>>::Member as c::expression::CType>::Storage,
    > = |base| unsafe { ::core::ptr::addr_of_mut!((*base).classId) };
#[doc(hidden)]
pub struct Field_subcmds;
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_2,
        subcmds
    ) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_subcmds>
    for c::expression::CRecord<
        crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_2,
    >
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _:
    Projection<
        crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_2,
        <<c::expression::CRecord<
            crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_2,
        > as c::expression::OrdinaryField<Field_subcmds>>::Member as c::expression::CType>::Storage,
    > = |base| unsafe { ::core::ptr::addr_of_mut!((*base).subcmds) };
#[doc(hidden)]
pub struct Field_ndicts;
const _: () = {
    assert!(
        ::core::mem::size_of::<
            crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_6,
        >() == 32
    );
    assert!(
        ::core::mem::align_of::<
            crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_6,
        >() == 8
    );
};
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_6
{
}
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_6,
        ndicts
    ) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ndicts>
    for c::expression::CRecord<
        crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_6,
    >
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _:
    Projection<
        crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_6,
        <<c::expression::CRecord<
            crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_6,
        > as c::expression::OrdinaryField<Field_ndicts>>::Member as c::expression::CType>::Storage,
    > = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ndicts) };
const _: () = {
    assert!(
        ::core::mem::size_of::<
            crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_7,
        >() == 4
    );
    assert!(
        ::core::mem::align_of::<
            crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_7,
        >() == 4
    );
};
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_7
{
}
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_7,
        objtype
    ) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_objtype>
    for c::expression::CRecord<
        crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_7,
    >
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ObjectType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _:
    Projection<
        crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_7,
        <<c::expression::CRecord<
            crate::__pgrx_c_bindings::CollectedCommand__bindgen_ty_1__bindgen_ty_7,
        > as c::expression::OrdinaryField<Field_objtype>>::Member as c::expression::CType>::Storage,
    > = |base| unsafe { ::core::ptr::addr_of_mut!((*base).objtype) };
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ColumnDef>() == 120);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ColumnDef>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ColumnDef {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ColumnDef, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ColumnDef>
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
    crate::__pgrx_c_bindings::ColumnDef,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ColumnDef> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_inhcount;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ColumnDef, inhcount) == 32);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inhcount>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ColumnDef>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ColumnDef,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ColumnDef> as c::expression::OrdinaryField<
        Field_inhcount,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).inhcount) };
#[doc(hidden)]
pub struct Field_is_local;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ColumnDef, is_local) == 36);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_local>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ColumnDef>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ColumnDef,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ColumnDef> as c::expression::OrdinaryField<
        Field_is_local,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_local) };
#[doc(hidden)]
pub struct Field_is_not_null;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ColumnDef, is_not_null) == 37);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_not_null>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ColumnDef>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 37;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ColumnDef,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ColumnDef> as c::expression::OrdinaryField<
        Field_is_not_null,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_not_null) };
#[doc(hidden)]
pub struct Field_is_from_type;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ColumnDef, is_from_type) == 38);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_from_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ColumnDef>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 38;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ColumnDef,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ColumnDef> as c::expression::OrdinaryField<
        Field_is_from_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_from_type) };
#[doc(hidden)]
pub struct Field_storage;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ColumnDef, storage) == 39);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_storage>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ColumnDef>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 39;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ColumnDef,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ColumnDef> as c::expression::OrdinaryField<
        Field_storage,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).storage) };
#[doc(hidden)]
pub struct Field_identity;
#[cfg(feature = "pg15")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ColumnDef, identity) == 56);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_identity>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ColumnDef>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ColumnDef,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ColumnDef> as c::expression::OrdinaryField<
        Field_identity,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).identity) };
#[doc(hidden)]
pub struct Field_generated;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ColumnDef, generated) == 72);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_generated>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ColumnDef>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ColumnDef,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ColumnDef> as c::expression::OrdinaryField<
        Field_generated,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).generated) };
#[cfg(feature = "pg15")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ColumnDef, collOid) == 88);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_collOid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ColumnDef>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ColumnDef,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ColumnDef> as c::expression::OrdinaryField<
        Field_collOid,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).collOid) };
#[doc(hidden)]
pub struct Field_constraints;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ColumnDef, constraints) == 96);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_constraints>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ColumnDef>
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
    crate::__pgrx_c_bindings::ColumnDef,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ColumnDef> as c::expression::OrdinaryField<
        Field_constraints,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).constraints) };
#[doc(hidden)]
pub struct Field_fdwoptions;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ColumnDef, fdwoptions) == 104);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fdwoptions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ColumnDef>
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
    crate::__pgrx_c_bindings::ColumnDef,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ColumnDef> as c::expression::OrdinaryField<
        Field_fdwoptions,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fdwoptions) };
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ColumnDef, location) == 112);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ColumnDef>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ColumnDef,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ColumnDef> as c::expression::OrdinaryField<
        Field_location,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ColumnRef>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ColumnRef>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ColumnRef {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ColumnRef, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ColumnRef>
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
    crate::__pgrx_c_bindings::ColumnRef,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ColumnRef> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_fields;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ColumnRef, fields) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fields>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ColumnRef>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ColumnRef,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ColumnRef> as c::expression::OrdinaryField<
        Field_fields,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fields) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ColumnRef, location) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ColumnRef>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ColumnRef,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ColumnRef> as c::expression::OrdinaryField<
        Field_location,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CommentStmt>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CommentStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CommentStmt {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CommentStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CommentStmt>
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
const _: Projection<crate::__pgrx_c_bindings::CommentStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CommentStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CommentStmt, objtype) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_objtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CommentStmt>
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
const _: Projection<crate::__pgrx_c_bindings::CommentStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CommentStmt> as c::expression::OrdinaryField<Field_objtype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).objtype) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CommonTableExpr>() == 104);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CommonTableExpr>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CommonTableExpr {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CommonTableExpr, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CommonTableExpr>
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
const _: Projection<crate::__pgrx_c_bindings::CommonTableExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::CommonTableExpr> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_aliascolnames;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CommonTableExpr, aliascolnames) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aliascolnames>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CommonTableExpr>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CommonTableExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::CommonTableExpr> as c::expression::OrdinaryField<Field_aliascolnames>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).aliascolnames) };
#[doc(hidden)]
pub struct Field_ctematerialized;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CommonTableExpr, ctematerialized) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ctematerialized>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CommonTableExpr>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__CTEMaterialize,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CommonTableExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::CommonTableExpr> as c::expression::OrdinaryField<Field_ctematerialized>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ctematerialized) };
#[doc(hidden)]
pub struct Field_ctequery;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CommonTableExpr, ctequery) == 32);
const _: () = {
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::Node>() == 8);
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::Node>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ctequery>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CommonTableExpr>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::Node>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CommonTableExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::CommonTableExpr> as c::expression::OrdinaryField<Field_ctequery>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ctequery) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CommonTableExpr, location) == 56);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CommonTableExpr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CommonTableExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::CommonTableExpr> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[doc(hidden)]
pub struct Field_cterecursive;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CommonTableExpr, cterecursive) == 60);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cterecursive>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CommonTableExpr>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 60;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CommonTableExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::CommonTableExpr> as c::expression::OrdinaryField<Field_cterecursive>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cterecursive) };
#[doc(hidden)]
pub struct Field_cterefcount;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CommonTableExpr, cterefcount) == 64);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cterefcount>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CommonTableExpr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CommonTableExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::CommonTableExpr> as c::expression::OrdinaryField<Field_cterefcount>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cterefcount) };
#[doc(hidden)]
pub struct Field_ctecolnames;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CommonTableExpr, ctecolnames) == 72);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ctecolnames>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CommonTableExpr>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CommonTableExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::CommonTableExpr> as c::expression::OrdinaryField<Field_ctecolnames>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ctecolnames) };
#[doc(hidden)]
pub struct Field_ctecoltypes;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CommonTableExpr, ctecoltypes) == 80);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ctecoltypes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CommonTableExpr>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CommonTableExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::CommonTableExpr> as c::expression::OrdinaryField<Field_ctecoltypes>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ctecoltypes) };
#[doc(hidden)]
pub struct Field_ctecoltypmods;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CommonTableExpr, ctecoltypmods) == 88
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ctecoltypmods>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CommonTableExpr>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CommonTableExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::CommonTableExpr> as c::expression::OrdinaryField<Field_ctecoltypmods>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ctecoltypmods) };
#[doc(hidden)]
pub struct Field_ctecolcollations;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CommonTableExpr, ctecolcollations) == 96
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ctecolcollations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CommonTableExpr>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CommonTableExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::CommonTableExpr> as c::expression::OrdinaryField<Field_ctecolcollations>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ctecolcollations) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CompositeTypeStmt>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CompositeTypeStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CompositeTypeStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CompositeTypeStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CompositeTypeStmt>
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
const _: Projection<crate::__pgrx_c_bindings::CompositeTypeStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CompositeTypeStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_coldeflist;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CompositeTypeStmt, coldeflist) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_coldeflist>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CompositeTypeStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CompositeTypeStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CompositeTypeStmt> as c::expression::OrdinaryField<Field_coldeflist>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).coldeflist) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ConfigVariable>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ConfigVariable>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ConfigVariable {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ConfigVariable, value) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_value>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ConfigVariable>
{
    type Member = c::expression::CPointer<
        c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ConfigVariable, <<c::expression::CRecord<crate::__pgrx_c_bindings::ConfigVariable> as c::expression::OrdinaryField<Field_value>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).value) };
#[doc(hidden)]
pub struct Field_sourceline;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ConfigVariable, sourceline) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sourceline>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ConfigVariable>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ConfigVariable, <<c::expression::CRecord<crate::__pgrx_c_bindings::ConfigVariable> as c::expression::OrdinaryField<Field_sourceline>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sourceline) };
#[doc(hidden)]
pub struct Field_ignore;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ConfigVariable, ignore) == 36);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ignore>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ConfigVariable>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ConfigVariable, <<c::expression::CRecord<crate::__pgrx_c_bindings::ConfigVariable> as c::expression::OrdinaryField<Field_ignore>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ignore) };
#[doc(hidden)]
pub struct Field_applied;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ConfigVariable, applied) == 37);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_applied>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ConfigVariable>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 37;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ConfigVariable, <<c::expression::CRecord<crate::__pgrx_c_bindings::ConfigVariable> as c::expression::OrdinaryField<Field_applied>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).applied) };
#[doc(hidden)]
pub struct Field_consttype;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Const>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Const>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::Const {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Const, consttype) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_consttype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Const>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Const,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Const> as c::expression::OrdinaryField<
        Field_consttype,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).consttype) };
#[doc(hidden)]
pub struct Field_consttypmod;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Const, consttypmod) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_consttypmod>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Const>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Const,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Const> as c::expression::OrdinaryField<
        Field_consttypmod,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).consttypmod) };
#[doc(hidden)]
pub struct Field_constcollid;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Const, constcollid) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_constcollid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Const>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Const,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Const> as c::expression::OrdinaryField<
        Field_constcollid,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).constcollid) };
#[doc(hidden)]
pub struct Field_constlen;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Const, constlen) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_constlen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Const>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Const,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Const> as c::expression::OrdinaryField<
        Field_constlen,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).constlen) };
#[doc(hidden)]
pub struct Field_constvalue;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Const, constvalue) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_constvalue>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Const>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, crate::__pgrx_c_bindings::Datum>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Const,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Const> as c::expression::OrdinaryField<
        Field_constvalue,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).constvalue) };
#[doc(hidden)]
pub struct Field_constisnull;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Const, constisnull) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_constisnull>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Const>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Const,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Const> as c::expression::OrdinaryField<
        Field_constisnull,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).constisnull) };
#[doc(hidden)]
pub struct Field_constbyval;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Const, constbyval) == 33);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_constbyval>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Const>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 33;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Const,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Const> as c::expression::OrdinaryField<
        Field_constbyval,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).constbyval) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Const, location) == 36);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Const>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Const,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Const> as c::expression::OrdinaryField<
        Field_location,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[doc(hidden)]
pub struct Field_ccvalid;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ConstrCheck>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ConstrCheck>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ConstrCheck {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ConstrCheck, ccvalid) == 16);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ccvalid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ConstrCheck>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ConstrCheck, <<c::expression::CRecord<crate::__pgrx_c_bindings::ConstrCheck> as c::expression::OrdinaryField<Field_ccvalid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ccvalid) };
#[doc(hidden)]
pub struct Field_ccnoinherit;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ConstrCheck, ccnoinherit) == 17);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ccnoinherit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ConstrCheck>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 17;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ConstrCheck, <<c::expression::CRecord<crate::__pgrx_c_bindings::ConstrCheck> as c::expression::OrdinaryField<Field_ccnoinherit>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ccnoinherit) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Constraint>() == 184);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Constraint>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::Constraint {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Constraint, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Constraint>
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
const _: Projection<crate::__pgrx_c_bindings::Constraint, <<c::expression::CRecord<crate::__pgrx_c_bindings::Constraint> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_contype;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Constraint, contype) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_contype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Constraint>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ConstrType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Constraint, <<c::expression::CRecord<crate::__pgrx_c_bindings::Constraint> as c::expression::OrdinaryField<Field_contype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).contype) };
#[doc(hidden)]
pub struct Field_deferrable;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Constraint, deferrable) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_deferrable>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Constraint>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Constraint, <<c::expression::CRecord<crate::__pgrx_c_bindings::Constraint> as c::expression::OrdinaryField<Field_deferrable>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).deferrable) };
#[doc(hidden)]
pub struct Field_initdeferred;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Constraint, initdeferred) == 17);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_initdeferred>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Constraint>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 17;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Constraint, <<c::expression::CRecord<crate::__pgrx_c_bindings::Constraint> as c::expression::OrdinaryField<Field_initdeferred>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).initdeferred) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Constraint, location) == 20);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Constraint>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Constraint, <<c::expression::CRecord<crate::__pgrx_c_bindings::Constraint> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[doc(hidden)]
pub struct Field_is_no_inherit;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Constraint, is_no_inherit) == 24);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_no_inherit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Constraint>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Constraint, <<c::expression::CRecord<crate::__pgrx_c_bindings::Constraint> as c::expression::OrdinaryField<Field_is_no_inherit>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_no_inherit) };
#[doc(hidden)]
pub struct Field_generated_when;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Constraint, generated_when) == 48);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_generated_when>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Constraint>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Constraint, <<c::expression::CRecord<crate::__pgrx_c_bindings::Constraint> as c::expression::OrdinaryField<Field_generated_when>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).generated_when) };
#[doc(hidden)]
pub struct Field_nulls_not_distinct;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::Constraint, nulls_not_distinct) == 49
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nulls_not_distinct>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Constraint>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 49;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Constraint, <<c::expression::CRecord<crate::__pgrx_c_bindings::Constraint> as c::expression::OrdinaryField<Field_nulls_not_distinct>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nulls_not_distinct) };
#[doc(hidden)]
pub struct Field_keys;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Constraint, keys) == 56);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_keys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Constraint>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Constraint, <<c::expression::CRecord<crate::__pgrx_c_bindings::Constraint> as c::expression::OrdinaryField<Field_keys>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).keys) };
#[doc(hidden)]
pub struct Field_including;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Constraint, including) == 64);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_including>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Constraint>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Constraint, <<c::expression::CRecord<crate::__pgrx_c_bindings::Constraint> as c::expression::OrdinaryField<Field_including>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).including) };
#[doc(hidden)]
pub struct Field_exclusions;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Constraint, exclusions) == 72);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_exclusions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Constraint>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Constraint, <<c::expression::CRecord<crate::__pgrx_c_bindings::Constraint> as c::expression::OrdinaryField<Field_exclusions>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).exclusions) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Constraint, options) == 80);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Constraint>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Constraint, <<c::expression::CRecord<crate::__pgrx_c_bindings::Constraint> as c::expression::OrdinaryField<Field_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
#[doc(hidden)]
pub struct Field_reset_default_tblspc;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::Constraint, reset_default_tblspc) == 104
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_reset_default_tblspc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Constraint>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Constraint, <<c::expression::CRecord<crate::__pgrx_c_bindings::Constraint> as c::expression::OrdinaryField<Field_reset_default_tblspc>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).reset_default_tblspc) };
#[doc(hidden)]
pub struct Field_fk_attrs;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Constraint, fk_attrs) == 136);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fk_attrs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Constraint>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Constraint, <<c::expression::CRecord<crate::__pgrx_c_bindings::Constraint> as c::expression::OrdinaryField<Field_fk_attrs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fk_attrs) };
#[doc(hidden)]
pub struct Field_pk_attrs;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Constraint, pk_attrs) == 144);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pk_attrs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Constraint>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Constraint, <<c::expression::CRecord<crate::__pgrx_c_bindings::Constraint> as c::expression::OrdinaryField<Field_pk_attrs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pk_attrs) };
#[doc(hidden)]
pub struct Field_fk_matchtype;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Constraint, fk_matchtype) == 152);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fk_matchtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Constraint>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Constraint, <<c::expression::CRecord<crate::__pgrx_c_bindings::Constraint> as c::expression::OrdinaryField<Field_fk_matchtype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fk_matchtype) };
#[doc(hidden)]
pub struct Field_fk_upd_action;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Constraint, fk_upd_action) == 153);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fk_upd_action>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Constraint>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 153;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Constraint, <<c::expression::CRecord<crate::__pgrx_c_bindings::Constraint> as c::expression::OrdinaryField<Field_fk_upd_action>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fk_upd_action) };
#[doc(hidden)]
pub struct Field_fk_del_action;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Constraint, fk_del_action) == 154);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fk_del_action>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Constraint>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 154;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Constraint, <<c::expression::CRecord<crate::__pgrx_c_bindings::Constraint> as c::expression::OrdinaryField<Field_fk_del_action>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fk_del_action) };
#[doc(hidden)]
pub struct Field_fk_del_set_cols;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Constraint, fk_del_set_cols) == 160);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fk_del_set_cols>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Constraint>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Constraint, <<c::expression::CRecord<crate::__pgrx_c_bindings::Constraint> as c::expression::OrdinaryField<Field_fk_del_set_cols>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fk_del_set_cols) };
#[doc(hidden)]
pub struct Field_old_conpfeqop;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Constraint, old_conpfeqop) == 168);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_old_conpfeqop>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Constraint>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 168;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Constraint, <<c::expression::CRecord<crate::__pgrx_c_bindings::Constraint> as c::expression::OrdinaryField<Field_old_conpfeqop>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).old_conpfeqop) };
#[doc(hidden)]
pub struct Field_old_pktable_oid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Constraint, old_pktable_oid) == 176);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_old_pktable_oid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Constraint>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 176;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Constraint, <<c::expression::CRecord<crate::__pgrx_c_bindings::Constraint> as c::expression::OrdinaryField<Field_old_pktable_oid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).old_pktable_oid) };
#[doc(hidden)]
pub struct Field_skip_validation;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Constraint, skip_validation) == 180);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_skip_validation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Constraint>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 180;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Constraint, <<c::expression::CRecord<crate::__pgrx_c_bindings::Constraint> as c::expression::OrdinaryField<Field_skip_validation>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).skip_validation) };
#[doc(hidden)]
pub struct Field_initially_valid;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Constraint, initially_valid) == 181);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_initially_valid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Constraint>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 181;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Constraint, <<c::expression::CRecord<crate::__pgrx_c_bindings::Constraint> as c::expression::OrdinaryField<Field_initially_valid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).initially_valid) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ConstraintsSetStmt>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ConstraintsSetStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ConstraintsSetStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ConstraintsSetStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ConstraintsSetStmt>
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
const _: Projection<crate::__pgrx_c_bindings::ConstraintsSetStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::ConstraintsSetStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ConstraintsSetStmt, constraints) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_constraints>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ConstraintsSetStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ConstraintsSetStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::ConstraintsSetStmt> as c::expression::OrdinaryField<Field_constraints>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).constraints) };
#[doc(hidden)]
pub struct Field_deferred;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ConstraintsSetStmt, deferred) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_deferred>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ConstraintsSetStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ConstraintsSetStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::ConstraintsSetStmt> as c::expression::OrdinaryField<Field_deferred>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).deferred) };
#[doc(hidden)]
pub struct Field_system_identifier;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ControlFileData>() == 296);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ControlFileData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ControlFileData {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, system_identifier) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_system_identifier>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ControlFileData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData> as c::expression::OrdinaryField<Field_system_identifier>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).system_identifier) };
#[doc(hidden)]
pub struct Field_pg_control_version;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, pg_control_version) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pg_control_version>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ControlFileData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData> as c::expression::OrdinaryField<Field_pg_control_version>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pg_control_version) };
#[doc(hidden)]
pub struct Field_catalog_version_no;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, catalog_version_no) == 12
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_catalog_version_no>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ControlFileData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData> as c::expression::OrdinaryField<Field_catalog_version_no>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).catalog_version_no) };
#[doc(hidden)]
pub struct Field_state;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, state) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_state>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__DBState,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ControlFileData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData> as c::expression::OrdinaryField<Field_state>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).state) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, time) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ControlFileData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData> as c::expression::OrdinaryField<Field_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).time) };
#[doc(hidden)]
pub struct Field_checkPoint;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, checkPoint) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_checkPoint>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ControlFileData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData> as c::expression::OrdinaryField<Field_checkPoint>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).checkPoint) };
#[doc(hidden)]
pub struct Field_unloggedLSN;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, unloggedLSN) == 128);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_unloggedLSN>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ControlFileData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData> as c::expression::OrdinaryField<Field_unloggedLSN>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).unloggedLSN) };
#[doc(hidden)]
pub struct Field_minRecoveryPoint;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, minRecoveryPoint) == 136
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_minRecoveryPoint>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ControlFileData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData> as c::expression::OrdinaryField<Field_minRecoveryPoint>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).minRecoveryPoint) };
#[doc(hidden)]
pub struct Field_minRecoveryPointTLI;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, minRecoveryPointTLI) == 144
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_minRecoveryPointTLI>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ControlFileData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData> as c::expression::OrdinaryField<Field_minRecoveryPointTLI>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).minRecoveryPointTLI) };
#[doc(hidden)]
pub struct Field_backupStartPoint;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, backupStartPoint) == 152
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_backupStartPoint>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ControlFileData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData> as c::expression::OrdinaryField<Field_backupStartPoint>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).backupStartPoint) };
#[doc(hidden)]
pub struct Field_backupEndPoint;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, backupEndPoint) == 160
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_backupEndPoint>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ControlFileData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData> as c::expression::OrdinaryField<Field_backupEndPoint>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).backupEndPoint) };
#[doc(hidden)]
pub struct Field_backupEndRequired;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, backupEndRequired) == 168
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_backupEndRequired>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 168;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ControlFileData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData> as c::expression::OrdinaryField<Field_backupEndRequired>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).backupEndRequired) };
#[doc(hidden)]
pub struct Field_wal_level;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, wal_level) == 172);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wal_level>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 172;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ControlFileData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData> as c::expression::OrdinaryField<Field_wal_level>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wal_level) };
#[doc(hidden)]
pub struct Field_wal_log_hints;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, wal_log_hints) == 176
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wal_log_hints>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 176;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ControlFileData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData> as c::expression::OrdinaryField<Field_wal_log_hints>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wal_log_hints) };
#[doc(hidden)]
pub struct Field_MaxConnections;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, MaxConnections) == 180
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_MaxConnections>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 180;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ControlFileData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData> as c::expression::OrdinaryField<Field_MaxConnections>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).MaxConnections) };
#[doc(hidden)]
pub struct Field_max_worker_processes;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, max_worker_processes) == 184
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_max_worker_processes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 184;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ControlFileData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData> as c::expression::OrdinaryField<Field_max_worker_processes>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).max_worker_processes) };
#[doc(hidden)]
pub struct Field_max_wal_senders;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, max_wal_senders) == 188
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_max_wal_senders>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 188;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ControlFileData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData> as c::expression::OrdinaryField<Field_max_wal_senders>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).max_wal_senders) };
#[doc(hidden)]
pub struct Field_max_prepared_xacts;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, max_prepared_xacts) == 192
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_max_prepared_xacts>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 192;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ControlFileData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData> as c::expression::OrdinaryField<Field_max_prepared_xacts>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).max_prepared_xacts) };
#[doc(hidden)]
pub struct Field_max_locks_per_xact;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, max_locks_per_xact) == 196
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_max_locks_per_xact>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 196;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ControlFileData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData> as c::expression::OrdinaryField<Field_max_locks_per_xact>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).max_locks_per_xact) };
#[doc(hidden)]
pub struct Field_track_commit_timestamp;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, track_commit_timestamp)
        == 200
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_track_commit_timestamp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 200;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ControlFileData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData> as c::expression::OrdinaryField<Field_track_commit_timestamp>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).track_commit_timestamp) };
#[doc(hidden)]
pub struct Field_maxAlign;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, maxAlign) == 204);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_maxAlign>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 204;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ControlFileData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData> as c::expression::OrdinaryField<Field_maxAlign>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).maxAlign) };
#[doc(hidden)]
pub struct Field_blcksz;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, blcksz) == 216);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_blcksz>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 216;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ControlFileData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData> as c::expression::OrdinaryField<Field_blcksz>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).blcksz) };
#[doc(hidden)]
pub struct Field_relseg_size;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, relseg_size) == 220);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relseg_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 220;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ControlFileData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData> as c::expression::OrdinaryField<Field_relseg_size>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relseg_size) };
#[doc(hidden)]
pub struct Field_xlog_blcksz;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, xlog_blcksz) == 224);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xlog_blcksz>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 224;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ControlFileData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData> as c::expression::OrdinaryField<Field_xlog_blcksz>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xlog_blcksz) };
#[doc(hidden)]
pub struct Field_xlog_seg_size;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, xlog_seg_size) == 228
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xlog_seg_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 228;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ControlFileData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData> as c::expression::OrdinaryField<Field_xlog_seg_size>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xlog_seg_size) };
#[doc(hidden)]
pub struct Field_nameDataLen;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, nameDataLen) == 232);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nameDataLen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 232;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ControlFileData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData> as c::expression::OrdinaryField<Field_nameDataLen>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nameDataLen) };
#[doc(hidden)]
pub struct Field_indexMaxKeys;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, indexMaxKeys) == 236
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indexMaxKeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 236;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ControlFileData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData> as c::expression::OrdinaryField<Field_indexMaxKeys>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indexMaxKeys) };
#[doc(hidden)]
pub struct Field_toast_max_chunk_size;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, toast_max_chunk_size) == 240
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_toast_max_chunk_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 240;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ControlFileData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData> as c::expression::OrdinaryField<Field_toast_max_chunk_size>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).toast_max_chunk_size) };
#[doc(hidden)]
pub struct Field_loblksize;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, loblksize) == 244);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_loblksize>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 244;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ControlFileData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData> as c::expression::OrdinaryField<Field_loblksize>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).loblksize) };
#[doc(hidden)]
pub struct Field_float8ByVal;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, float8ByVal) == 248);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_float8ByVal>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 248;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ControlFileData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData> as c::expression::OrdinaryField<Field_float8ByVal>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).float8ByVal) };
#[doc(hidden)]
pub struct Field_data_checksum_version;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, data_checksum_version)
        == 252
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_data_checksum_version>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 252;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ControlFileData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData> as c::expression::OrdinaryField<Field_data_checksum_version>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).data_checksum_version) };
#[doc(hidden)]
pub struct Field_crc;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ControlFileData, crc) == 288);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_crc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 288;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ControlFileData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ControlFileData> as c::expression::OrdinaryField<Field_crc>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).crc) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ConvertRowtypeExpr>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ConvertRowtypeExpr>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ConvertRowtypeExpr {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ConvertRowtypeExpr, resulttype) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resulttype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ConvertRowtypeExpr>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ConvertRowtypeExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::ConvertRowtypeExpr> as c::expression::OrdinaryField<Field_resulttype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).resulttype) };
#[doc(hidden)]
pub struct Field_convertformat;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ConvertRowtypeExpr, convertformat) == 20
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_convertformat>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ConvertRowtypeExpr>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__CoercionForm,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ConvertRowtypeExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::ConvertRowtypeExpr> as c::expression::OrdinaryField<Field_convertformat>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).convertformat) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ConvertRowtypeExpr, location) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ConvertRowtypeExpr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ConvertRowtypeExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::ConvertRowtypeExpr> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CookedConstraint>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CookedConstraint>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CookedConstraint {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CookedConstraint, contype) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_contype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CookedConstraint>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ConstrType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CookedConstraint, <<c::expression::CRecord<crate::__pgrx_c_bindings::CookedConstraint> as c::expression::OrdinaryField<Field_contype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).contype) };
#[doc(hidden)]
pub struct Field_conoid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CookedConstraint, conoid) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_conoid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CookedConstraint>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CookedConstraint, <<c::expression::CRecord<crate::__pgrx_c_bindings::CookedConstraint> as c::expression::OrdinaryField<Field_conoid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).conoid) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CookedConstraint, attnum) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attnum>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CookedConstraint>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CookedConstraint, <<c::expression::CRecord<crate::__pgrx_c_bindings::CookedConstraint> as c::expression::OrdinaryField<Field_attnum>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attnum) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CookedConstraint, skip_validation) == 32
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_skip_validation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CookedConstraint>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CookedConstraint, <<c::expression::CRecord<crate::__pgrx_c_bindings::CookedConstraint> as c::expression::OrdinaryField<Field_skip_validation>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).skip_validation) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CookedConstraint, is_local) == 33);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_local>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CookedConstraint>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 33;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CookedConstraint, <<c::expression::CRecord<crate::__pgrx_c_bindings::CookedConstraint> as c::expression::OrdinaryField<Field_is_local>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_local) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CookedConstraint, inhcount) == 36);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inhcount>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CookedConstraint>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CookedConstraint, <<c::expression::CRecord<crate::__pgrx_c_bindings::CookedConstraint> as c::expression::OrdinaryField<Field_inhcount>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).inhcount) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CookedConstraint, is_no_inherit) == 40
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_no_inherit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CookedConstraint>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CookedConstraint, <<c::expression::CRecord<crate::__pgrx_c_bindings::CookedConstraint> as c::expression::OrdinaryField<Field_is_no_inherit>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_no_inherit) };
#[doc(hidden)]
pub struct Field_file_encoding;
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CopyFormatOptions>() == 136);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CopyFormatOptions>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CopyFormatOptions {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, file_encoding) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_file_encoding>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFormatOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions> as c::expression::OrdinaryField<Field_file_encoding>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).file_encoding) };
#[doc(hidden)]
pub struct Field_binary;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, binary) == 4);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_binary>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFormatOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions> as c::expression::OrdinaryField<Field_binary>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).binary) };
#[doc(hidden)]
pub struct Field_freeze;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, freeze) == 5);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_freeze>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 5;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFormatOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions> as c::expression::OrdinaryField<Field_freeze>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).freeze) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
pub struct Field_csv_mode;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, csv_mode) == 6);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_csv_mode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 6;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFormatOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions> as c::expression::OrdinaryField<Field_csv_mode>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).csv_mode) };
#[doc(hidden)]
pub struct Field_header_line;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, header_line) == 8);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_header_line>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__CopyHeaderChoice,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFormatOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions> as c::expression::OrdinaryField<Field_header_line>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).header_line) };
#[doc(hidden)]
pub struct Field_null_print_len;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, null_print_len) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_null_print_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFormatOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions> as c::expression::OrdinaryField<Field_null_print_len>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).null_print_len) };
#[doc(hidden)]
pub struct Field_force_quote;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, force_quote) == 64
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_force_quote>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFormatOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions> as c::expression::OrdinaryField<Field_force_quote>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).force_quote) };
#[doc(hidden)]
pub struct Field_force_quote_all;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, force_quote_all) == 72
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_force_quote_all>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFormatOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions> as c::expression::OrdinaryField<Field_force_quote_all>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).force_quote_all) };
#[doc(hidden)]
pub struct Field_force_notnull;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, force_notnull) == 88
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_force_notnull>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFormatOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions> as c::expression::OrdinaryField<Field_force_notnull>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).force_notnull) };
#[doc(hidden)]
pub struct Field_force_null;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, force_null) == 104
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_force_null>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFormatOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions> as c::expression::OrdinaryField<Field_force_null>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).force_null) };
#[doc(hidden)]
pub struct Field_convert_selectively;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, convert_selectively)
        == 120
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_convert_selectively>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFormatOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions> as c::expression::OrdinaryField<Field_convert_selectively>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).convert_selectively) };
#[doc(hidden)]
pub struct Field_convert_select;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, convert_select) == 128
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_convert_select>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFormatOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions> as c::expression::OrdinaryField<Field_convert_select>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).convert_select) };
#[doc(hidden)]
pub struct Field_copy_src;
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CopyFromStateData>() == 472);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CopyFromStateData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CopyFromStateData {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, copy_src) == 0);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_copy_src>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__CopySource,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFromStateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData> as c::expression::OrdinaryField<Field_copy_src>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).copy_src) };
#[doc(hidden)]
pub struct Field_eol_type;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, eol_type) == 24);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_eol_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__EolType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFromStateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData> as c::expression::OrdinaryField<Field_eol_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).eol_type) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, file_encoding) == 28
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_file_encoding>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFromStateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData> as c::expression::OrdinaryField<Field_file_encoding>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).file_encoding) };
#[doc(hidden)]
pub struct Field_need_transcoding;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, need_transcoding) == 32
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_need_transcoding>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFromStateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData> as c::expression::OrdinaryField<Field_need_transcoding>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).need_transcoding) };
#[doc(hidden)]
pub struct Field_conversion_proc;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, conversion_proc) == 36
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_conversion_proc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFromStateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData> as c::expression::OrdinaryField<Field_conversion_proc>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).conversion_proc) };
#[doc(hidden)]
pub struct Field_attnumlist;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, attnumlist) == 48);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attnumlist>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFromStateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData> as c::expression::OrdinaryField<Field_attnumlist>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attnumlist) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, is_program) == 64);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_program>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFromStateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData> as c::expression::OrdinaryField<Field_is_program>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_program) };
#[doc(hidden)]
pub struct Field_cur_lineno;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, cur_lineno) == 240
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cur_lineno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 240;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFromStateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData> as c::expression::OrdinaryField<Field_cur_lineno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cur_lineno) };
#[doc(hidden)]
pub struct Field_num_defaults;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, num_defaults) == 272
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_defaults>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 272;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFromStateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData> as c::expression::OrdinaryField<Field_num_defaults>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_defaults) };
#[doc(hidden)]
pub struct Field_volatile_defexprs;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, volatile_defexprs) == 312
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_volatile_defexprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 312;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFromStateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData> as c::expression::OrdinaryField<Field_volatile_defexprs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).volatile_defexprs) };
#[doc(hidden)]
pub struct Field_range_table;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, range_table) == 320
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_range_table>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 320;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFromStateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData> as c::expression::OrdinaryField<Field_range_table>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).range_table) };
#[doc(hidden)]
pub struct Field_max_fields;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, max_fields) == 368
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_max_fields>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 368;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFromStateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData> as c::expression::OrdinaryField<Field_max_fields>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).max_fields) };
#[doc(hidden)]
pub struct Field_line_buf_valid;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, line_buf_valid) == 408
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_line_buf_valid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 408;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFromStateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData> as c::expression::OrdinaryField<Field_line_buf_valid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).line_buf_valid) };
#[doc(hidden)]
pub struct Field_input_buf_index;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, input_buf_index) == 424
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_input_buf_index>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 424;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFromStateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData> as c::expression::OrdinaryField<Field_input_buf_index>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).input_buf_index) };
#[doc(hidden)]
pub struct Field_input_buf_len;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, input_buf_len) == 428
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_input_buf_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 428;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFromStateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData> as c::expression::OrdinaryField<Field_input_buf_len>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).input_buf_len) };
#[doc(hidden)]
pub struct Field_input_reached_eof;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, input_reached_eof) == 432
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_input_reached_eof>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 432;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFromStateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData> as c::expression::OrdinaryField<Field_input_reached_eof>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).input_reached_eof) };
#[doc(hidden)]
pub struct Field_input_reached_error;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, input_reached_error)
        == 433
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_input_reached_error>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 433;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFromStateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData> as c::expression::OrdinaryField<Field_input_reached_error>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).input_reached_error) };
#[doc(hidden)]
pub struct Field_raw_buf_index;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, raw_buf_index) == 448
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_raw_buf_index>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 448;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFromStateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData> as c::expression::OrdinaryField<Field_raw_buf_index>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).raw_buf_index) };
#[doc(hidden)]
pub struct Field_raw_buf_len;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, raw_buf_len) == 452
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_raw_buf_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 452;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFromStateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData> as c::expression::OrdinaryField<Field_raw_buf_len>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).raw_buf_len) };
#[doc(hidden)]
pub struct Field_raw_reached_eof;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, raw_reached_eof) == 456
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_raw_reached_eof>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 456;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFromStateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData> as c::expression::OrdinaryField<Field_raw_reached_eof>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).raw_reached_eof) };
#[doc(hidden)]
pub struct Field_bytes_processed;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, bytes_processed) == 464
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bytes_processed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 464;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFromStateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData> as c::expression::OrdinaryField<Field_bytes_processed>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).bytes_processed) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CreateAmStmt>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CreateAmStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CreateAmStmt {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateAmStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateAmStmt>
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
const _: Projection<crate::__pgrx_c_bindings::CreateAmStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateAmStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_handler_name;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateAmStmt, handler_name) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_handler_name>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateAmStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CreateAmStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateAmStmt> as c::expression::OrdinaryField<Field_handler_name>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).handler_name) };
#[doc(hidden)]
pub struct Field_amtype;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateAmStmt, amtype) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateAmStmt>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CreateAmStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateAmStmt> as c::expression::OrdinaryField<Field_amtype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).amtype) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CreateCastStmt>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CreateCastStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CreateCastStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateCastStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateCastStmt>
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
const _: Projection<crate::__pgrx_c_bindings::CreateCastStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateCastStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateCastStmt, context) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_context>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateCastStmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__CoercionContext,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CreateCastStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateCastStmt> as c::expression::OrdinaryField<Field_context>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).context) };
#[doc(hidden)]
pub struct Field_inout;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateCastStmt, inout) == 36);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inout>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateCastStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CreateCastStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateCastStmt> as c::expression::OrdinaryField<Field_inout>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).inout) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CreateConversionStmt>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CreateConversionStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CreateConversionStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateConversionStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateConversionStmt>
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
const _: Projection<crate::__pgrx_c_bindings::CreateConversionStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateConversionStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_conversion_name;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateConversionStmt, conversion_name) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_conversion_name>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateConversionStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CreateConversionStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateConversionStmt> as c::expression::OrdinaryField<Field_conversion_name>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).conversion_name) };
#[doc(hidden)]
pub struct Field_func_name;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateConversionStmt, func_name) == 32
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_func_name>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateConversionStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CreateConversionStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateConversionStmt> as c::expression::OrdinaryField<Field_func_name>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).func_name) };
#[doc(hidden)]
pub struct Field_def;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateConversionStmt, def) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_def>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateConversionStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CreateConversionStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateConversionStmt> as c::expression::OrdinaryField<Field_def>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).def) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CreateDomainStmt>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CreateDomainStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CreateDomainStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateDomainStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateDomainStmt>
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
const _: Projection<crate::__pgrx_c_bindings::CreateDomainStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateDomainStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_domainname;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateDomainStmt, domainname) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_domainname>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateDomainStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CreateDomainStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateDomainStmt> as c::expression::OrdinaryField<Field_domainname>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).domainname) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateDomainStmt, constraints) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_constraints>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateDomainStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CreateDomainStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateDomainStmt> as c::expression::OrdinaryField<Field_constraints>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).constraints) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CreateEnumStmt>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CreateEnumStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CreateEnumStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateEnumStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateEnumStmt>
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
const _: Projection<crate::__pgrx_c_bindings::CreateEnumStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateEnumStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateEnumStmt, typeName) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typeName>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateEnumStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CreateEnumStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateEnumStmt> as c::expression::OrdinaryField<Field_typeName>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typeName) };
#[doc(hidden)]
pub struct Field_vals;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateEnumStmt, vals) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vals>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateEnumStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CreateEnumStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateEnumStmt> as c::expression::OrdinaryField<Field_vals>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).vals) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CreateEventTrigStmt>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CreateEventTrigStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CreateEventTrigStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateEventTrigStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateEventTrigStmt>
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
const _: Projection<crate::__pgrx_c_bindings::CreateEventTrigStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateEventTrigStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_whenclause;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateEventTrigStmt, whenclause) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_whenclause>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateEventTrigStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CreateEventTrigStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateEventTrigStmt> as c::expression::OrdinaryField<Field_whenclause>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).whenclause) };
#[doc(hidden)]
pub struct Field_funcname;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateEventTrigStmt, funcname) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_funcname>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateEventTrigStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CreateEventTrigStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateEventTrigStmt> as c::expression::OrdinaryField<Field_funcname>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).funcname) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CreateExtensionStmt>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CreateExtensionStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CreateExtensionStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateExtensionStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateExtensionStmt>
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
const _: Projection<crate::__pgrx_c_bindings::CreateExtensionStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateExtensionStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_if_not_exists;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateExtensionStmt, if_not_exists) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_if_not_exists>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateExtensionStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CreateExtensionStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateExtensionStmt> as c::expression::OrdinaryField<Field_if_not_exists>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).if_not_exists) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateExtensionStmt, options) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateExtensionStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CreateExtensionStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateExtensionStmt> as c::expression::OrdinaryField<Field_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CreateFdwStmt>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CreateFdwStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CreateFdwStmt {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateFdwStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateFdwStmt>
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
const _: Projection<crate::__pgrx_c_bindings::CreateFdwStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateFdwStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateFdwStmt, func_options) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_func_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateFdwStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CreateFdwStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateFdwStmt> as c::expression::OrdinaryField<Field_func_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).func_options) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateFdwStmt, options) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateFdwStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CreateFdwStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateFdwStmt> as c::expression::OrdinaryField<Field_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CreateForeignServerStmt>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CreateForeignServerStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CreateForeignServerStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateForeignServerStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateForeignServerStmt>
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
const _: Projection<crate::__pgrx_c_bindings::CreateForeignServerStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateForeignServerStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateForeignServerStmt, if_not_exists) == 40
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_if_not_exists>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateForeignServerStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CreateForeignServerStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateForeignServerStmt> as c::expression::OrdinaryField<Field_if_not_exists>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).if_not_exists) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateForeignServerStmt, options) == 48
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateForeignServerStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CreateForeignServerStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateForeignServerStmt> as c::expression::OrdinaryField<Field_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CreateFunctionStmt>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CreateFunctionStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CreateFunctionStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateFunctionStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateFunctionStmt>
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
const _: Projection<crate::__pgrx_c_bindings::CreateFunctionStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateFunctionStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_is_procedure;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateFunctionStmt, is_procedure) == 4
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_procedure>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateFunctionStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CreateFunctionStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateFunctionStmt> as c::expression::OrdinaryField<Field_is_procedure>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_procedure) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateFunctionStmt, replace) == 5);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_replace>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateFunctionStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 5;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CreateFunctionStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateFunctionStmt> as c::expression::OrdinaryField<Field_replace>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).replace) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateFunctionStmt, funcname) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_funcname>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateFunctionStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CreateFunctionStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateFunctionStmt> as c::expression::OrdinaryField<Field_funcname>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).funcname) };
#[doc(hidden)]
pub struct Field_parameters;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateFunctionStmt, parameters) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parameters>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateFunctionStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CreateFunctionStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateFunctionStmt> as c::expression::OrdinaryField<Field_parameters>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).parameters) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateFunctionStmt, options) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateFunctionStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CreateFunctionStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateFunctionStmt> as c::expression::OrdinaryField<Field_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CreateOpClassItem>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CreateOpClassItem>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CreateOpClassItem {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateOpClassItem, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateOpClassItem>
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
const _: Projection<crate::__pgrx_c_bindings::CreateOpClassItem, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateOpClassItem> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_itemtype;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateOpClassItem, itemtype) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_itemtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateOpClassItem>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CreateOpClassItem, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateOpClassItem> as c::expression::OrdinaryField<Field_itemtype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).itemtype) };
#[doc(hidden)]
pub struct Field_number;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateOpClassItem, number) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_number>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateOpClassItem>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CreateOpClassItem, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateOpClassItem> as c::expression::OrdinaryField<Field_number>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).number) };
#[doc(hidden)]
pub struct Field_order_family;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateOpClassItem, order_family) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_order_family>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateOpClassItem>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CreateOpClassItem, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateOpClassItem> as c::expression::OrdinaryField<Field_order_family>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).order_family) };
#[doc(hidden)]
pub struct Field_class_args;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateOpClassItem, class_args) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_class_args>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateOpClassItem>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CreateOpClassItem, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateOpClassItem> as c::expression::OrdinaryField<Field_class_args>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).class_args) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CreateOpClassStmt>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CreateOpClassStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CreateOpClassStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateOpClassStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateOpClassStmt>
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
const _: Projection<crate::__pgrx_c_bindings::CreateOpClassStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateOpClassStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_opclassname;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateOpClassStmt, opclassname) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_opclassname>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateOpClassStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CreateOpClassStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateOpClassStmt> as c::expression::OrdinaryField<Field_opclassname>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).opclassname) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateOpClassStmt, opfamilyname) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_opfamilyname>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CreateOpClassStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CreateOpClassStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::CreateOpClassStmt> as c::expression::OrdinaryField<Field_opfamilyname>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).opfamilyname) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CreateOpClassStmt, items) == 40);
