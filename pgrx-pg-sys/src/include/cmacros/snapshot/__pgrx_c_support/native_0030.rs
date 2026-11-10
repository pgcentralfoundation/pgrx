// Shared generated C macro support.

#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_newRelfilelocatorSubid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelationData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData> as c::expression::OrdinaryField<Field_rd_newRelfilelocatorSubid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rd_newRelfilelocatorSubid) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_rd_firstRelfilelocatorSubid;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_firstRelfilelocatorSubid)
        == 48
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_firstRelfilelocatorSubid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelationData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData> as c::expression::OrdinaryField<Field_rd_firstRelfilelocatorSubid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rd_firstRelfilelocatorSubid) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_rd_attrsvalid;
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_attrsvalid) == 240);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_attrsvalid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 240;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelationData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData> as c::expression::OrdinaryField<Field_rd_attrsvalid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rd_attrsvalid) };
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_options) == 296);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::varlena>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 296;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_amhandler) == 304);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_amhandler>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 304;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_tableam) == 312);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_tableam>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>,
        c::expression::ReadOnly,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 312;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_index) == 320);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_index>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_index>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 320;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_indexprs) == 392);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_indexprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 392;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_indpred) == 400);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_indpred>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 400;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_toastoid) == 464);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_toastoid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 464;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, pgstat_enabled) == 468);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pgstat_enabled>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 468;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, pgstat_info) == 472);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pgstat_info>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableStatus>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 472;
}
#[cfg(feature = "pg16")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ReorderBuffer>() == 368);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ReorderBuffer>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBuffer, by_txn_last_xid) == 64
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_by_txn_last_xid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBuffer, output_rewrites) == 232
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_output_rewrites>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 232;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBuffer, context) == 240);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_context>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::MemoryContextData>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 240;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBuffer, current_restart_decoding_lsn)
        == 272
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_current_restart_decoding_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 272;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBuffer, outbufsize) == 288);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_outbufsize>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 288;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBuffer, size) == 296);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 296;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBuffer, spillTxns) == 304);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_spillTxns>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 304;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBuffer, spillCount) == 312);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_spillCount>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 312;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBuffer, spillBytes) == 320);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_spillBytes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 320;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBuffer, streamTxns) == 328);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_streamTxns>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 328;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBuffer, streamCount) == 336);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_streamCount>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 336;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBuffer, streamBytes) == 344);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_streamBytes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 344;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBuffer, totalTxns) == 352);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_totalTxns>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 352;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBuffer, totalBytes) == 360);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_totalBytes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 360;
}
#[cfg(feature = "pg16")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ReorderBufferTXN>() == 320);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ReorderBufferTXN>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_toptxn;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBufferTXN, toptxn) == 48);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::ReorderBufferTXN>() == 8);
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::ReorderBufferTXN>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_toptxn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBufferTXN, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN> as c::expression::OrdinaryField<Field_toptxn>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).toptxn) };
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBufferTXN, size) == 272);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 272;
}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBufferTXN, total_size) == 280);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_total_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 280;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBufferTXN, concurrent_abort) == 288
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_concurrent_abort>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 288;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBufferTXN, ninvalidations_distributed)
        == 304
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ninvalidations_distributed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 304;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_invalidated;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReplicationSlotPersistentData, invalidated)
        == 88
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_invalidated>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlotPersistentData>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ReplicationSlotInvalidationCause,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReplicationSlotPersistentData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlotPersistentData> as c::expression::OrdinaryField<Field_invalidated>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).invalidated) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RestrictInfo>() == 240);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RestrictInfo>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RestrictInfo, can_join) == 17);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_can_join>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 17;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RestrictInfo, pseudoconstant) == 18);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pseudoconstant>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 18;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_has_clone;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RestrictInfo, has_clone) == 19);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_has_clone>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 19;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RestrictInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo> as c::expression::OrdinaryField<Field_has_clone>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).has_clone) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_is_clone;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RestrictInfo, is_clone) == 20);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_clone>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RestrictInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo> as c::expression::OrdinaryField<Field_is_clone>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_clone) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RestrictInfo, leakproof) == 21);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_leakproof>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 21;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_num_base_rels;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RestrictInfo, num_base_rels) == 32);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_base_rels>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RestrictInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo> as c::expression::OrdinaryField<Field_num_base_rels>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_base_rels) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RestrictInfo, required_relids) == 48);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_required_relids>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::Bitmapset>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_rinfo_serial;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RestrictInfo, rinfo_serial) == 96);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rinfo_serial>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RestrictInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo> as c::expression::OrdinaryField<Field_rinfo_serial>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rinfo_serial) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RestrictInfo, mergeopfamilies) == 144
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mergeopfamilies>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RestrictInfo, scansel_cache) == 184);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_scansel_cache>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 184;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RestrictInfo, outer_is_left) == 192);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_outer_is_left>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 192;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RestrictInfo, hashjoinoperator) == 196
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hashjoinoperator>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 196;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RestrictInfo, left_hasheqoperator) == 232
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_left_hasheqoperator>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 232;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RestrictInfo, right_hasheqoperator) == 236
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_right_hasheqoperator>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 236;
}
#[cfg(feature = "pg16")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ResultRelInfo>() == 360);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ResultRelInfo>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_projectNewInfoValid) == 80
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_projectNewInfoValid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_needLockTagTuple) == 81
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_needLockTagTuple>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 81;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_usesFdwDirectModify) == 160
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_usesFdwDirectModify>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_NumSlots) == 164);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_NumSlots>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 164;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_NumSlotsInitialized) == 168
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_NumSlotsInitialized>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 168;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_BatchSize) == 172);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_BatchSize>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 172;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_WithCheckOptions) == 192
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_WithCheckOptions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 192;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_WithCheckOptionExprs)
        == 200
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_WithCheckOptionExprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 200;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_ri_NumGeneratedNeededI;
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_NumGeneratedNeededI) == 232
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_NumGeneratedNeededI>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 232;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ResultRelInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo> as c::expression::OrdinaryField<Field_ri_NumGeneratedNeededI>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ri_NumGeneratedNeededI) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_ri_NumGeneratedNeededU;
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_NumGeneratedNeededU) == 236
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_NumGeneratedNeededU>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 236;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ResultRelInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo> as c::expression::OrdinaryField<Field_ri_NumGeneratedNeededU>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ri_NumGeneratedNeededU) };
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_returningList) == 240
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_returningList>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 240;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_onConflictArbiterIndexes)
        == 256
);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_onConflictArbiterIndexes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 256;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_matchedMergeAction) == 272
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_matchedMergeAction>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 272;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_notMatchedMergeAction)
        == 280
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_notMatchedMergeAction>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 280;
}
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_ChildToRootMapValid) == 304
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_ChildToRootMapValid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 304;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_ri_RootToChildMapValid;
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_RootToChildMapValid) == 320
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_RootToChildMapValid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 320;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ResultRelInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo> as c::expression::OrdinaryField<Field_ri_RootToChildMapValid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ri_RootToChildMapValid) };
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_ancestorResultRels) == 352
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_ancestorResultRels>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 352;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SERIALIZABLEXACT>() == 168);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SERIALIZABLEXACT>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SERIALIZABLEXACT, topXid) == 144);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_topXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SERIALIZABLEXACT>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SERIALIZABLEXACT, finishedBefore) == 148
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_finishedBefore>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SERIALIZABLEXACT>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 148;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SERIALIZABLEXACT, xmin) == 152);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xmin>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SERIALIZABLEXACT>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SERIALIZABLEXACT, flags) == 156);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SERIALIZABLEXACT>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 156;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SERIALIZABLEXACT, pid) == 160);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SERIALIZABLEXACT>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SERIALIZABLEXACT, pgprocno) == 164);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pgprocno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SERIALIZABLEXACT>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 164;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_smgr_rlocator;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SMgrRelationData, smgr_rlocator) == 0
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_smgr_rlocator>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SMgrRelationData>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::RelFileLocatorBackend>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SMgrRelationData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SMgrRelationData> as c::expression::OrdinaryField<Field_smgr_rlocator>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).smgr_rlocator) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_sharedsort;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SortCoordinateData, sharedsort) == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::Sharedsort>() == 8);
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::Sharedsort>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sharedsort>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SortCoordinateData>
{
    type Member = c::expression::CPointer<
        c::expression::COpaque<crate::__pgrx_c_bindings::Sharedsort>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SortCoordinateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SortCoordinateData> as c::expression::OrdinaryField<Field_sharedsort>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sharedsort) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_attLeafType;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistCache, attLeafType) == 28);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attLeafType>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistCache>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::SpGistTypeDesc>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpGistCache, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpGistCache> as c::expression::OrdinaryField<Field_attLeafType>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attLeafType) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpGistState, attLeafType) == 36);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attLeafType>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpGistState>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::SpGistTypeDesc>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpGistState, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpGistState> as c::expression::OrdinaryField<Field_attLeafType>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attLeafType) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SpecialJoinInfo>() == 104);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SpecialJoinInfo>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_ojrelid;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpecialJoinInfo, ojrelid) == 44);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ojrelid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpecialJoinInfo>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpecialJoinInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpecialJoinInfo> as c::expression::OrdinaryField<Field_ojrelid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ojrelid) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SpecialJoinInfo, lhs_strict) == 80);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lhs_strict>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpecialJoinInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SpecialJoinInfo, semi_can_btree) == 81
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_semi_can_btree>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpecialJoinInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 81;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SpecialJoinInfo, semi_can_hash) == 82
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_semi_can_hash>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpecialJoinInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 82;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SpecialJoinInfo, semi_operators) == 88
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_semi_operators>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpecialJoinInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SpecialJoinInfo, semi_rhs_exprs) == 96
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_semi_rhs_exprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpecialJoinInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Subscription>() == 80);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Subscription>() == 8);
};
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stream>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Subscription>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 30;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_passwordrequired;
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::Subscription, passwordrequired) == 33
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_passwordrequired>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Subscription>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 33;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Subscription, <<c::expression::CRecord<crate::__pgrx_c_bindings::Subscription> as c::expression::OrdinaryField<Field_passwordrequired>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).passwordrequired) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_runasowner;
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Subscription, runasowner) == 34);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_runasowner>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Subscription>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 34;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Subscription, <<c::expression::CRecord<crate::__pgrx_c_bindings::Subscription> as c::expression::OrdinaryField<Field_runasowner>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).runasowner) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::SupportRequestOptimizeWindowClause>()
            == 32
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::SupportRequestOptimizeWindowClause>()
            == 8
    );
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SupportRequestOptimizeWindowClause {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SupportRequestOptimizeWindowClause, type_)
        == 0
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestOptimizeWindowClause>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SupportRequestOptimizeWindowClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestOptimizeWindowClause> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::SupportRequestOptimizeWindowClause,
        frameOptions
    ) == 24
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_frameOptions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestOptimizeWindowClause>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SupportRequestOptimizeWindowClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestOptimizeWindowClause> as c::expression::OrdinaryField<Field_frameOptions>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).frameOptions) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_firstblocknumber;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TapeShare>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TapeShare>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TapeShare {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TapeShare, firstblocknumber) == 0);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_firstblocknumber>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TapeShare>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::TapeShare,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::TapeShare> as c::expression::OrdinaryField<
        Field_firstblocknumber,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).firstblocknumber) };
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Var>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Var>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Var, varlevelsup) == 32);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_varlevelsup>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Var>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Var, varnosyn) == 36);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_varnosyn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Var>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Var, varattnosyn) == 40);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_varattnosyn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Var>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Var, location) == 44);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Var>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            crate::__pgrx_c_bindings::WalRcvStreamOptions__bindgen_ty_1__bindgen_ty_2,
        >() == 48
    );
    assert!(
        ::core::mem::align_of::<
            crate::__pgrx_c_bindings::WalRcvStreamOptions__bindgen_ty_1__bindgen_ty_2,
        >() == 8
    );
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::WalRcvStreamOptions__bindgen_ty_1__bindgen_ty_2,
        twophase
    ) == 32
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_twophase>
    for c::expression::CRecord<
        crate::__pgrx_c_bindings::WalRcvStreamOptions__bindgen_ty_1__bindgen_ty_2,
    >
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg16")]
const _: () = {
    assert!(
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
    assert!(
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
};
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_walrcv_connect>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalReceiverFunctionsType>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_struct__WalReceiverConn__0p__0l__const__char__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__const__char__0p__0c__char__0p__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(*const ::core::ffi::c_char, bool) -> (),
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(*const ::core::ffi::c_char, bool) -> (),
            >,
        >() == 8
    );
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_walrcv_check_conninfo>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalReceiverFunctionsType>
{
    type Member = c::expression::CFunction<
        crate::__pgrx_c_generated::Signature_void__0l__const__char__0p__0c__0x5f426f6f6c__0r,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::WalSnd>() == 104);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::WalSnd>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WalSnd, kind) == 96);
#[cfg(any(feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_kind>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalSnd>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ReplicationKind,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::WalSnd,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::WalSnd> as c::expression::OrdinaryField<
        Field_kind,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).kind) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::XmlExpr>() == 64);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::XmlExpr>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XmlExpr, indent) == 44);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indent>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XmlExpr>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::XmlExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::XmlExpr> as c::expression::OrdinaryField<
        Field_indent,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indent) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XmlExpr, type_) == 48);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XmlExpr>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XmlExpr, typmod) == 52);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typmod>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XmlExpr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XmlExpr, location) == 56);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XmlExpr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XmlSerialize, indent) == 24);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indent>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XmlSerialize>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XmlSerialize, <<c::expression::CRecord<crate::__pgrx_c_bindings::XmlSerialize> as c::expression::OrdinaryField<Field_indent>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indent) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XmlSerialize, location) == 28);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XmlSerialize>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(feature = "pg16")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::catcache>() == 464);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::catcache>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_cc_nlist;
#[cfg(feature = "pg16")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::catcache, cc_nlist) == 448);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cc_nlist>
    for c::expression::CRecord<crate::__pgrx_c_bindings::catcache>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 448;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::catcache,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::catcache> as c::expression::OrdinaryField<
        Field_cc_nlist,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cc_nlist) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_cc_nlbuckets;
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::catcache, cc_nlbuckets) == 452);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cc_nlbuckets>
    for c::expression::CRecord<crate::__pgrx_c_bindings::catcache>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 452;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::catcache,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::catcache> as c::expression::OrdinaryField<
        Field_cc_nlbuckets,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cc_nlbuckets) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::config_bool>() == 200);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::config_bool>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_bool, boot_val) == 152);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_boot_val>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_bool>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_bool, reset_val) == 184);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_reset_val>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_bool>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 184;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::config_enum>() == 208);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::config_enum>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_enum, boot_val) == 152);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_boot_val>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_enum>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_enum, reset_val) == 192);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_reset_val>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_enum>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 192;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::config_generic>() == 144);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::config_generic>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_generic, sourceline) == 136);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sourceline>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_generic>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::config_int>() == 208);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::config_int>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_int, boot_val) == 152);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_boot_val>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_int>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_int, min) == 156);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_min>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_int>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 156;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_int, max) == 160);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_max>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_int>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_int, reset_val) == 192);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_reset_val>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_int>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 192;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_snapshotConflictHorizon;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::gistxlogDelete, snapshotConflictHorizon) == 0
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_snapshotConflictHorizon>
    for c::expression::CRecord<crate::__pgrx_c_bindings::gistxlogDelete>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::gistxlogDelete, <<c::expression::CRecord<crate::__pgrx_c_bindings::gistxlogDelete> as c::expression::OrdinaryField<Field_snapshotConflictHorizon>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).snapshotConflictHorizon) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_isCatalogRel;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::gistxlogDelete, isCatalogRel) == 6);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isCatalogRel>
    for c::expression::CRecord<crate::__pgrx_c_bindings::gistxlogDelete>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 6;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::gistxlogDelete, <<c::expression::CRecord<crate::__pgrx_c_bindings::gistxlogDelete> as c::expression::OrdinaryField<Field_isCatalogRel>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isCatalogRel) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::gistxlogPageReuse>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::gistxlogPageReuse>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::gistxlogPageReuse, isCatalogRel) == 24
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isCatalogRel>
    for c::expression::CRecord<crate::__pgrx_c_bindings::gistxlogPageReuse>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::gistxlogPageReuse, <<c::expression::CRecord<crate::__pgrx_c_bindings::gistxlogPageReuse> as c::expression::OrdinaryField<Field_isCatalogRel>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isCatalogRel) };
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::pg_locale_struct>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::pg_locale_struct>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::spgxlogVacuumRedirect>() == 12);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::spgxlogVacuumRedirect>() == 4);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::spgxlogVacuumRedirect,
        snapshotConflictHorizon
    ) == 4
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_snapshotConflictHorizon>
    for c::expression::CRecord<crate::__pgrx_c_bindings::spgxlogVacuumRedirect>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::spgxlogVacuumRedirect, <<c::expression::CRecord<crate::__pgrx_c_bindings::spgxlogVacuumRedirect> as c::expression::OrdinaryField<Field_snapshotConflictHorizon>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).snapshotConflictHorizon) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::spgxlogVacuumRedirect, isCatalogRel) == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isCatalogRel>
    for c::expression::CRecord<crate::__pgrx_c_bindings::spgxlogVacuumRedirect>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::spgxlogVacuumRedirect, <<c::expression::CRecord<crate::__pgrx_c_bindings::spgxlogVacuumRedirect> as c::expression::OrdinaryField<Field_isCatalogRel>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isCatalogRel) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::xl_btree_delete>() == 12);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::xl_btree_delete>() == 4);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_btree_delete, snapshotConflictHorizon)
        == 0
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_snapshotConflictHorizon>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_btree_delete>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_btree_delete, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_btree_delete> as c::expression::OrdinaryField<Field_snapshotConflictHorizon>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).snapshotConflictHorizon) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_btree_delete, isCatalogRel) == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isCatalogRel>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_btree_delete>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_btree_delete, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_btree_delete> as c::expression::OrdinaryField<Field_isCatalogRel>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isCatalogRel) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::xl_btree_reuse_page>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::xl_btree_reuse_page>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_btree_reuse_page, isCatalogRel) == 24
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isCatalogRel>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_btree_reuse_page>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_btree_reuse_page, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_btree_reuse_page> as c::expression::OrdinaryField<Field_isCatalogRel>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isCatalogRel) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::xl_hash_vacuum_one_page,
        snapshotConflictHorizon
    ) == 0
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_snapshotConflictHorizon>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_hash_vacuum_one_page>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_hash_vacuum_one_page, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_hash_vacuum_one_page> as c::expression::OrdinaryField<Field_snapshotConflictHorizon>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).snapshotConflictHorizon) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ntuples>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_hash_vacuum_one_page>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_hash_vacuum_one_page, isCatalogRel) == 6
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isCatalogRel>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_hash_vacuum_one_page>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 6;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_hash_vacuum_one_page, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_hash_vacuum_one_page> as c::expression::OrdinaryField<Field_isCatalogRel>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isCatalogRel) };
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_heap_freeze_page, snapshotConflictHorizon)
        == 0
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_snapshotConflictHorizon>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_freeze_page>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg16")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_heap_freeze_page, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_freeze_page> as c::expression::OrdinaryField<Field_snapshotConflictHorizon>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).snapshotConflictHorizon) };
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_heap_freeze_page, nplans) == 4);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nplans>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_freeze_page>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(feature = "pg16")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_heap_freeze_page, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_freeze_page> as c::expression::OrdinaryField<Field_nplans>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nplans) };
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_heap_freeze_page, isCatalogRel) == 6
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isCatalogRel>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_freeze_page>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 6;
}
#[cfg(feature = "pg16")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_heap_freeze_page, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_freeze_page> as c::expression::OrdinaryField<Field_isCatalogRel>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isCatalogRel) };
#[cfg(feature = "pg16")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::xl_heap_freeze_plan>() == 12);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::xl_heap_freeze_plan>() == 4);
};
#[cfg(feature = "pg16")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::xl_heap_freeze_plan {}
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_heap_freeze_plan, xmax) == 0);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xmax>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_freeze_plan>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg16")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_heap_freeze_plan, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_freeze_plan> as c::expression::OrdinaryField<Field_xmax>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xmax) };
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_heap_freeze_plan, t_infomask2) == 4
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_t_infomask2>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_freeze_plan>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(feature = "pg16")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_heap_freeze_plan, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_freeze_plan> as c::expression::OrdinaryField<Field_t_infomask2>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).t_infomask2) };
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_heap_freeze_plan, t_infomask) == 6
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_t_infomask>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_freeze_plan>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 6;
}
#[cfg(feature = "pg16")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_heap_freeze_plan, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_freeze_plan> as c::expression::OrdinaryField<Field_t_infomask>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).t_infomask) };
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_heap_freeze_plan, frzflags) == 8);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_frzflags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_freeze_plan>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg16")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_heap_freeze_plan, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_freeze_plan> as c::expression::OrdinaryField<Field_frzflags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).frzflags) };
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_heap_freeze_plan, ntuples) == 10);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ntuples>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_freeze_plan>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 10;
}
#[cfg(feature = "pg16")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_heap_freeze_plan, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_freeze_plan> as c::expression::OrdinaryField<Field_ntuples>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ntuples) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_heap_lock, xmax) == 0);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xmax>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_lock>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_heap_lock, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_lock> as c::expression::OrdinaryField<Field_xmax>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xmax) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_infobits_set>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_lock>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 6;
}
#[cfg(feature = "pg16")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::xl_heap_prune>() == 12);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::xl_heap_prune>() == 4);
};
#[cfg(feature = "pg16")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_heap_prune, snapshotConflictHorizon) == 0
);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_snapshotConflictHorizon>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_prune>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg16")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_heap_prune, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_prune> as c::expression::OrdinaryField<Field_snapshotConflictHorizon>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).snapshotConflictHorizon) };
#[cfg(feature = "pg16")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_heap_prune, isCatalogRel) == 8);
#[cfg(feature = "pg16")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isCatalogRel>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_prune>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg16")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_heap_prune, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_prune> as c::expression::OrdinaryField<Field_isCatalogRel>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isCatalogRel) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_heap_visible, snapshotConflictHorizon)
        == 0
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_snapshotConflictHorizon>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_visible>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_heap_visible, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_visible> as c::expression::OrdinaryField<Field_snapshotConflictHorizon>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).snapshotConflictHorizon) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::xl_xact_relfilelocators>() == 4);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::xl_xact_relfilelocators>() == 4);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::xl_xact_relfilelocators {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_xact_relfilelocators, nrels) == 0);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nrels>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_xact_relfilelocators>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_xact_relfilelocators, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_xact_relfilelocators> as c::expression::OrdinaryField<Field_nrels>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nrels) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PGAlignedXLogBlock>() == 8192);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PGAlignedXLogBlock>() == 4096);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PGIOAlignedBlock>() == 8192);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PGIOAlignedBlock>() == 4096);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PGIOAlignedBlock {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGIOAlignedBlock, data) == 0);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_data>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGIOAlignedBlock>
{
    type Member =
        c::expression::CArray<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, 8192>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PGIOAlignedBlock, <<c::expression::CRecord<crate::__pgrx_c_bindings::PGIOAlignedBlock> as c::expression::OrdinaryField<Field_data>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).data) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGIOAlignedBlock, force_align_i64) == 0
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_force_align_i64>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGIOAlignedBlock>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PGIOAlignedBlock, <<c::expression::CRecord<crate::__pgrx_c_bindings::PGIOAlignedBlock> as c::expression::OrdinaryField<Field_force_align_i64>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).force_align_i64) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBufferTXN__bindgen_ty_1, abort_time)
        == 0
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_abort_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN__bindgen_ty_1>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBufferTXN__bindgen_ty_1, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN__bindgen_ty_1> as c::expression::OrdinaryField<Field_abort_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).abort_time) };
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::pg_locale_struct__bindgen_ty_1>() == 16
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::pg_locale_struct__bindgen_ty_1>() == 8
    );
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::WalRcvStreamOptions__bindgen_ty_1>() == 48
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::WalRcvStreamOptions__bindgen_ty_1>() == 8
    );
};
#[cfg(any(feature = "pg16", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            crate::__pgrx_c_bindings::pg_locale_struct__bindgen_ty_1__bindgen_ty_1,
        >() == 16
    );
    assert!(
        ::core::mem::align_of::<
            crate::__pgrx_c_bindings::pg_locale_struct__bindgen_ty_1__bindgen_ty_1,
        >() == 8
    );
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::pg_locale_struct__bindgen_ty_1__bindgen_ty_1
{
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::config_string>() == 200);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::config_string>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::config_real>() == 216);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::config_real>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for crate::__pgrx_c_bindings::UCollator {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeType for crate::__pgrx_c_bindings::UCollator {
    type Marker = c::expression::COpaque<Self>;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_FunctionCallUsage>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_FunctionCallUsage>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_BktypeIO>() == 1024);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_BktypeIO>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgStat_BktypeIO {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MemoryContextMethods>() == 80);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MemoryContextMethods>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for crate::__pgrx_c_bindings::LogicalTape {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeType for crate::__pgrx_c_bindings::LogicalTape {
    type Marker = c::expression::COpaque<Self>;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LockRowsState>() == 312);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LockRowsState>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_44>(
        ) == 8
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_44>(
        ) == 8
    );
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_44
{
}
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_38>(
        ) == 8
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_38>(
        ) == 8
    );
};
#[cfg(any(feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_34>(
        ) == 8
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_34>(
        ) == 8
    );
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::DateTimeErrorExtra>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::DateTimeErrorExtra>() == 8);
};
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::DateTimeErrorExtra {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::size_of::<*const crate::__pgrx_c_bindings::buftag>() == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::align_of::<*const crate::__pgrx_c_bindings::buftag>() == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::RelFileLocator>>()
        == 12
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::RelFileLocator>>()
        == 4
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__BufTagGetRelFileLocator"]
    fn raw_inline_BufTagGetRelFileLocator(
        arg0: *const crate::__pgrx_c_bindings::buftag,
    ) -> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::RelFileLocator>;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc = "Call the original C function `BufTagGetRelFileLocator`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_BufTagGetRelFileLocator(
    arg0: *const crate::__pgrx_c_bindings::buftag,
) -> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::RelFileLocator> {
    const _: () = assert!(!::core::mem::needs_drop::<*const crate::__pgrx_c_bindings::buftag>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_BufTagGetRelFileLocator(arg0)) }
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::buftag>() == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::buftag>() == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__BufTagSetRelForkDetails"]
    fn raw_inline_BufTagSetRelForkDetails(
        arg0: *mut crate::__pgrx_c_bindings::buftag,
        arg1: crate::__pgrx_c_bindings::Oid,
        arg2: i32,
    ) -> ();
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc = "Call the original C function `BufTagSetRelForkDetails`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_BufTagSetRelForkDetails(
    arg0: *mut crate::__pgrx_c_bindings::buftag,
    arg1: crate::__pgrx_c_bindings::Oid,
    arg2: i32,
) -> () {
    const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::buftag>());
    const _: () = assert!(!::core::mem::needs_drop::<crate::__pgrx_c_bindings::Oid>());
    const _: () = assert!(!::core::mem::needs_drop::<i32>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe {
        crate::ffi::pg_guard_ffi_boundary(move || {
            raw_inline_BufTagSetRelForkDetails(arg0, arg1, arg2)
        })
    }
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__CharGetDatum"]
    fn raw_inline_CharGetDatum(arg0: i8) -> crate::__pgrx_c_bindings::Datum;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc = "Call the original C function `CharGetDatum`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_CharGetDatum(arg0: i8) -> crate::__pgrx_c_bindings::Datum {
    const _: () = assert!(!::core::mem::needs_drop::<i8>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_CharGetDatum(arg0)) }
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__DatumGetChar"]
    fn raw_inline_DatumGetChar(arg0: crate::__pgrx_c_bindings::Datum) -> i8;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc = "Call the original C function `DatumGetChar`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_DatumGetChar(arg0: crate::__pgrx_c_bindings::Datum) -> i8 {
    const _: () = assert!(!::core::mem::needs_drop::<crate::__pgrx_c_bindings::Datum>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_DatumGetChar(arg0)) }
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__GinTernaryValueGetDatum"]
    fn raw_inline_GinTernaryValueGetDatum(arg0: i8) -> crate::__pgrx_c_bindings::Datum;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc = "Call the original C function `GinTernaryValueGetDatum`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_GinTernaryValueGetDatum(arg0: i8) -> crate::__pgrx_c_bindings::Datum {
    const _: () = assert!(!::core::mem::needs_drop::<i8>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_GinTernaryValueGetDatum(arg0)) }
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::size_of::<bool>() == 1);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::align_of::<bool>() == 1);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::PageXLogRecPtr>>()
        == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::PageXLogRecPtr>>()
        == 4
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__PageXLogRecPtrGet"]
    fn raw_inline_PageXLogRecPtrGet(
        arg0: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::PageXLogRecPtr>,
    ) -> u64;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc = "Call the original C function `PageXLogRecPtrGet`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_PageXLogRecPtrGet(
    arg0: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::PageXLogRecPtr>,
) -> u64 {
    const _: () = assert!(!::core::mem::needs_drop::<
        ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::PageXLogRecPtr>,
    >());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_PageXLogRecPtrGet(arg0)) }
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::size_of::<bool>() == 1);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::align_of::<bool>() == 1);
#[cfg(feature = "pg16")]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__am_tablesync_worker"]
    fn raw_inline_am_tablesync_worker() -> bool;
}
#[cfg(feature = "pg16")]
#[doc = "Call the original C function `am_tablesync_worker`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_am_tablesync_worker() -> bool {
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_am_tablesync_worker()) }
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::size_of::<i8>() == 1);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::align_of::<i8>() == 1);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Oid>() == 4);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Oid>() == 4);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::size_of::<i8>() == 1);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::align_of::<i8>() == 1);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::size_of::<i64>() == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::align_of::<i64>() == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__labs"]
    fn raw_inline_labs(arg0: i64) -> i64;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc = "Call the original C function `labs`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_labs(arg0: i64) -> i64 {
    const _: () = assert!(!::core::mem::needs_drop::<i64>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_labs(arg0)) }
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::instr_time>>() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::instr_time>>() == 8
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__pg_clock_gettime_ns"]
    fn raw_inline_pg_clock_gettime_ns()
    -> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::instr_time>;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc = "Call the original C function `pg_clock_gettime_ns`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_pg_clock_gettime_ns()
-> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::instr_time> {
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_pg_clock_gettime_ns()) }
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::size_of::<i8>() == 1);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::align_of::<i8>() == 1);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::RelFileLocator>>()
        == 12
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::RelFileLocator>>()
        == 4
);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__smgropen"]
    fn raw_inline_smgropen(
        arg0: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::RelFileLocator>,
        arg1: i32,
    ) -> *mut crate::__pgrx_c_bindings::SMgrRelationData;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc = "Call the original C function `smgropen`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_smgropen(
    arg0: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::RelFileLocator>,
    arg1: i32,
) -> *mut crate::__pgrx_c_bindings::SMgrRelationData {
    const _: () = assert!(!::core::mem::needs_drop::<
        ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::RelFileLocator>,
    >());
    const _: () = assert!(!::core::mem::needs_drop::<i32>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_smgropen(arg0, arg1)) }
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::size_of::<i64>() == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::align_of::<i64>() == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::size_of::<u64>() == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::align_of::<u64>() == 8);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::size_of::<bool>() == 1);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::align_of::<bool>() == 1);
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__JsonConstructorType;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for EnumIdentity_enum__JsonConstructorType {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::EnumIdentity for EnumIdentity_enum__JsonConstructorType {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__JsonConstructorType {
    type Compatible = c::CUnsignedInt;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__JsonEncoding;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for EnumIdentity_enum__JsonEncoding {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::EnumIdentity for EnumIdentity_enum__JsonEncoding {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__JsonEncoding {
    type Compatible = c::CUnsignedInt;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__JsonFormatType;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for EnumIdentity_enum__JsonFormatType {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::EnumIdentity for EnumIdentity_enum__JsonFormatType {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__JsonFormatType {
    type Compatible = c::CUnsignedInt;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__JsonValueType;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for EnumIdentity_enum__JsonValueType {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::EnumIdentity for EnumIdentity_enum__JsonValueType {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__JsonValueType {
    type Compatible = c::CUnsignedInt;
}
#[cfg(feature = "pg16")]
// SAFETY: The explicit Rust integer repr matches Clang's compatible enum type and the assertions prove layout. Decoding receives a valid unit Rust variant; encoding checks every discriminant.
unsafe impl c::expression::EnumStorage<EnumIdentity_enum__NodeTag, c::CUnsignedInt>
    for crate::__pgrx_c_bindings::NodeTag
{
    fn decode(value: Self) -> u32 {
        value as u32
    }
    fn encode(value: u32) -> Self {
        match value {
            71 => crate::__pgrx_c_bindings::NodeTag::T_A_ArrayExpr,
            63 => crate::__pgrx_c_bindings::NodeTag::T_A_Const,
            62 => crate::__pgrx_c_bindings::NodeTag::T_A_Expr,
            69 => crate::__pgrx_c_bindings::NodeTag::T_A_Indices,
            70 => crate::__pgrx_c_bindings::NodeTag::T_A_Indirection,
            68 => crate::__pgrx_c_bindings::NodeTag::T_A_Star,
            135 => crate::__pgrx_c_bindings::NodeTag::T_AccessPriv,
            344 => crate::__pgrx_c_bindings::NodeTag::T_Agg,
            307 => crate::__pgrx_c_bindings::NodeTag::T_AggInfo,
            287 => crate::__pgrx_c_bindings::NodeTag::T_AggPath,
            408 => crate::__pgrx_c_bindings::NodeTag::T_AggState,
            308 => crate::__pgrx_c_bindings::NodeTag::T_AggTransInfo,
            9 => crate::__pgrx_c_bindings::NodeTag::T_Aggref,
            2 => crate::__pgrx_c_bindings::NodeTag::T_Alias,
            450 => crate::__pgrx_c_bindings::NodeTag::T_AllocSetContext,
            131 => crate::__pgrx_c_bindings::NodeTag::T_AlterCollationStmt,
            215 => crate::__pgrx_c_bindings::NodeTag::T_AlterDatabaseRefreshCollStmt,
            216 => crate::__pgrx_c_bindings::NodeTag::T_AlterDatabaseSetStmt,
            214 => crate::__pgrx_c_bindings::NodeTag::T_AlterDatabaseStmt,
            137 => crate::__pgrx_c_bindings::NodeTag::T_AlterDefaultPrivilegesStmt,
            132 => crate::__pgrx_c_bindings::NodeTag::T_AlterDomainStmt,
            210 => crate::__pgrx_c_bindings::NodeTag::T_AlterEnumStmt,
            164 => crate::__pgrx_c_bindings::NodeTag::T_AlterEventTrigStmt,
            149 => crate::__pgrx_c_bindings::NodeTag::T_AlterExtensionContentsStmt,
            148 => crate::__pgrx_c_bindings::NodeTag::T_AlterExtensionStmt,
            151 => crate::__pgrx_c_bindings::NodeTag::T_AlterFdwStmt,
            153 => crate::__pgrx_c_bindings::NodeTag::T_AlterForeignServerStmt,
            191 => crate::__pgrx_c_bindings::NodeTag::T_AlterFunctionStmt,
            197 => crate::__pgrx_c_bindings::NodeTag::T_AlterObjectDependsStmt,
            198 => crate::__pgrx_c_bindings::NodeTag::T_AlterObjectSchemaStmt,
            177 => crate::__pgrx_c_bindings::NodeTag::T_AlterOpFamilyStmt,
            200 => crate::__pgrx_c_bindings::NodeTag::T_AlterOperatorStmt,
            199 => crate::__pgrx_c_bindings::NodeTag::T_AlterOwnerStmt,
            160 => crate::__pgrx_c_bindings::NodeTag::T_AlterPolicyStmt,
            243 => crate::__pgrx_c_bindings::NodeTag::T_AlterPublicationStmt,
            168 => crate::__pgrx_c_bindings::NodeTag::T_AlterRoleSetStmt,
            167 => crate::__pgrx_c_bindings::NodeTag::T_AlterRoleStmt,
            171 => crate::__pgrx_c_bindings::NodeTag::T_AlterSeqStmt,
            188 => crate::__pgrx_c_bindings::NodeTag::T_AlterStatsStmt,
            245 => crate::__pgrx_c_bindings::NodeTag::T_AlterSubscriptionStmt,
            218 => crate::__pgrx_c_bindings::NodeTag::T_AlterSystemStmt,
            239 => crate::__pgrx_c_bindings::NodeTag::T_AlterTSConfigurationStmt,
            238 => crate::__pgrx_c_bindings::NodeTag::T_AlterTSDictionaryStmt,
            130 => crate::__pgrx_c_bindings::NodeTag::T_AlterTableCmd,
            146 => crate::__pgrx_c_bindings::NodeTag::T_AlterTableMoveAllStmt,
            145 => crate::__pgrx_c_bindings::NodeTag::T_AlterTableSpaceOptionsStmt,
            128 => crate::__pgrx_c_bindings::NodeTag::T_AlterTableStmt,
            201 => crate::__pgrx_c_bindings::NodeTag::T_AlterTypeStmt,
            156 => crate::__pgrx_c_bindings::NodeTag::T_AlterUserMappingStmt,
            22 => crate::__pgrx_c_bindings::NodeTag::T_AlternativeSubPlan,
            313 => crate::__pgrx_c_bindings::NodeTag::T_Append,
            270 => crate::__pgrx_c_bindings::NodeTag::T_AppendPath,
            302 => crate::__pgrx_c_bindings::NodeTag::T_AppendRelInfo,
            376 => crate::__pgrx_c_bindings::NodeTag::T_AppendState,
            27 => crate::__pgrx_c_bindings::NodeTag::T_ArrayCoerceExpr,
            33 => crate::__pgrx_c_bindings::NodeTag::T_ArrayExpr,
            428 => crate::__pgrx_c_bindings::NodeTag::T_BaseBackupCmd,
            445 => crate::__pgrx_c_bindings::NodeTag::T_BitString,
            316 => crate::__pgrx_c_bindings::NodeTag::T_BitmapAnd,
            263 => crate::__pgrx_c_bindings::NodeTag::T_BitmapAndPath,
            379 => crate::__pgrx_c_bindings::NodeTag::T_BitmapAndState,
            262 => crate::__pgrx_c_bindings::NodeTag::T_BitmapHeapPath,
            323 => crate::__pgrx_c_bindings::NodeTag::T_BitmapHeapScan,
            387 => crate::__pgrx_c_bindings::NodeTag::T_BitmapHeapScanState,
            322 => crate::__pgrx_c_bindings::NodeTag::T_BitmapIndexScan,
            386 => crate::__pgrx_c_bindings::NodeTag::T_BitmapIndexScanState,
            317 => crate::__pgrx_c_bindings::NodeTag::T_BitmapOr,
            264 => crate::__pgrx_c_bindings::NodeTag::T_BitmapOrPath,
            380 => crate::__pgrx_c_bindings::NodeTag::T_BitmapOrState,
            424 => crate::__pgrx_c_bindings::NodeTag::T_Bitmapset,
            19 => crate::__pgrx_c_bindings::NodeTag::T_BoolExpr,
            443 => crate::__pgrx_c_bindings::NodeTag::T_Boolean,
            46 => crate::__pgrx_c_bindings::NodeTag::T_BooleanTest,
            105 => crate::__pgrx_c_bindings::NodeTag::T_CTECycleClause,
            104 => crate::__pgrx_c_bindings::NodeTag::T_CTESearchClause,
            195 => crate::__pgrx_c_bindings::NodeTag::T_CallContext,
            194 => crate::__pgrx_c_bindings::NodeTag::T_CallStmt,
            30 => crate::__pgrx_c_bindings::NodeTag::T_CaseExpr,
            32 => crate::__pgrx_c_bindings::NodeTag::T_CaseTestExpr,
            31 => crate::__pgrx_c_bindings::NodeTag::T_CaseWhen,
            225 => crate::__pgrx_c_bindings::NodeTag::T_CheckPointStmt,
            183 => crate::__pgrx_c_bindings::NodeTag::T_ClosePortalStmt,
            219 => crate::__pgrx_c_bindings::NodeTag::T_ClusterStmt,
            36 => crate::__pgrx_c_bindings::NodeTag::T_CoalesceExpr,
            47 => crate::__pgrx_c_bindings::NodeTag::T_CoerceToDomain,
            48 => crate::__pgrx_c_bindings::NodeTag::T_CoerceToDomainValue,
            26 => crate::__pgrx_c_bindings::NodeTag::T_CoerceViaIO,
            65 => crate::__pgrx_c_bindings::NodeTag::T_CollateClause,
            29 => crate::__pgrx_c_bindings::NodeTag::T_CollateExpr,
            81 => crate::__pgrx_c_bindings::NodeTag::T_ColumnDef,
            60 => crate::__pgrx_c_bindings::NodeTag::T_ColumnRef,
            180 => crate::__pgrx_c_bindings::NodeTag::T_CommentStmt,
            106 => crate::__pgrx_c_bindings::NodeTag::T_CommonTableExpr,
            207 => crate::__pgrx_c_bindings::NodeTag::T_CompositeTypeStmt,
            7 => crate::__pgrx_c_bindings::NodeTag::T_Const,
            142 => crate::__pgrx_c_bindings::NodeTag::T_Constraint,
            228 => crate::__pgrx_c_bindings::NodeTag::T_ConstraintsSetStmt,
            28 => crate::__pgrx_c_bindings::NodeTag::T_ConvertRowtypeExpr,
            138 => crate::__pgrx_c_bindings::NodeTag::T_CopyStmt,
            161 => crate::__pgrx_c_bindings::NodeTag::T_CreateAmStmt,
            231 => crate::__pgrx_c_bindings::NodeTag::T_CreateCastStmt,
            230 => crate::__pgrx_c_bindings::NodeTag::T_CreateConversionStmt,
            173 => crate::__pgrx_c_bindings::NodeTag::T_CreateDomainStmt,
            208 => crate::__pgrx_c_bindings::NodeTag::T_CreateEnumStmt,
            163 => crate::__pgrx_c_bindings::NodeTag::T_CreateEventTrigStmt,
            147 => crate::__pgrx_c_bindings::NodeTag::T_CreateExtensionStmt,
            150 => crate::__pgrx_c_bindings::NodeTag::T_CreateFdwStmt,
            152 => crate::__pgrx_c_bindings::NodeTag::T_CreateForeignServerStmt,
            154 => crate::__pgrx_c_bindings::NodeTag::T_CreateForeignTableStmt,
            189 => crate::__pgrx_c_bindings::NodeTag::T_CreateFunctionStmt,
            175 => crate::__pgrx_c_bindings::NodeTag::T_CreateOpClassItem,
            174 => crate::__pgrx_c_bindings::NodeTag::T_CreateOpClassStmt,
            176 => crate::__pgrx_c_bindings::NodeTag::T_CreateOpFamilyStmt,
            165 => crate::__pgrx_c_bindings::NodeTag::T_CreatePLangStmt,
            159 => crate::__pgrx_c_bindings::NodeTag::T_CreatePolicyStmt,
            242 => crate::__pgrx_c_bindings::NodeTag::T_CreatePublicationStmt,
            209 => crate::__pgrx_c_bindings::NodeTag::T_CreateRangeStmt,
            429 => crate::__pgrx_c_bindings::NodeTag::T_CreateReplicationSlotCmd,
            166 => crate::__pgrx_c_bindings::NodeTag::T_CreateRoleStmt,
            127 => crate::__pgrx_c_bindings::NodeTag::T_CreateSchemaStmt,
            170 => crate::__pgrx_c_bindings::NodeTag::T_CreateSeqStmt,
            186 => crate::__pgrx_c_bindings::NodeTag::T_CreateStatsStmt,
            141 => crate::__pgrx_c_bindings::NodeTag::T_CreateStmt,
            244 => crate::__pgrx_c_bindings::NodeTag::T_CreateSubscriptionStmt,
            223 => crate::__pgrx_c_bindings::NodeTag::T_CreateTableAsStmt,
            143 => crate::__pgrx_c_bindings::NodeTag::T_CreateTableSpaceStmt,
            232 => crate::__pgrx_c_bindings::NodeTag::T_CreateTransformStmt,
            162 => crate::__pgrx_c_bindings::NodeTag::T_CreateTrigStmt,
            155 => crate::__pgrx_c_bindings::NodeTag::T_CreateUserMappingStmt,
            213 => crate::__pgrx_c_bindings::NodeTag::T_CreatedbStmt,
            330 => crate::__pgrx_c_bindings::NodeTag::T_CteScan,
            394 => crate::__pgrx_c_bindings::NodeTag::T_CteScanState,
            50 => crate::__pgrx_c_bindings::NodeTag::T_CurrentOfExpr,
            269 => crate::__pgrx_c_bindings::NodeTag::T_CustomPath,
            334 => crate::__pgrx_c_bindings::NodeTag::T_CustomScan,
            398 => crate::__pgrx_c_bindings::NodeTag::T_CustomScanState,
            235 => crate::__pgrx_c_bindings::NodeTag::T_DeallocateStmt,
            182 => crate::__pgrx_c_bindings::NodeTag::T_DeclareCursorStmt,
            84 => crate::__pgrx_c_bindings::NodeTag::T_DefElem,
            172 => crate::__pgrx_c_bindings::NodeTag::T_DefineStmt,
            120 => crate::__pgrx_c_bindings::NodeTag::T_DeleteStmt,
            226 => crate::__pgrx_c_bindings::NodeTag::T_DiscardStmt,
            16 => crate::__pgrx_c_bindings::NodeTag::T_DistinctExpr,
            192 => crate::__pgrx_c_bindings::NodeTag::T_DoStmt,
            372 => crate::__pgrx_c_bindings::NodeTag::T_DomainConstraintState,
            236 => crate::__pgrx_c_bindings::NodeTag::T_DropOwnedStmt,
            430 => crate::__pgrx_c_bindings::NodeTag::T_DropReplicationSlotCmd,
            169 => crate::__pgrx_c_bindings::NodeTag::T_DropRoleStmt,
            178 => crate::__pgrx_c_bindings::NodeTag::T_DropStmt,
            246 => crate::__pgrx_c_bindings::NodeTag::T_DropSubscriptionStmt,
            144 => crate::__pgrx_c_bindings::NodeTag::T_DropTableSpaceStmt,
            157 => crate::__pgrx_c_bindings::NodeTag::T_DropUserMappingStmt,
            217 => crate::__pgrx_c_bindings::NodeTag::T_DropdbStmt,
            368 => crate::__pgrx_c_bindings::NodeTag::T_EState,
            254 => crate::__pgrx_c_bindings::NodeTag::T_EquivalenceClass,
            255 => crate::__pgrx_c_bindings::NodeTag::T_EquivalenceMember,
            426 => crate::__pgrx_c_bindings::NodeTag::T_ErrorSaveContext,
            420 => crate::__pgrx_c_bindings::NodeTag::T_EventTriggerData,
            234 => crate::__pgrx_c_bindings::NodeTag::T_ExecuteStmt,
            222 => crate::__pgrx_c_bindings::NodeTag::T_ExplainStmt,
            361 => crate::__pgrx_c_bindings::NodeTag::T_ExprContext,
            359 => crate::__pgrx_c_bindings::NodeTag::T_ExprState,
            425 => crate::__pgrx_c_bindings::NodeTag::T_ExtensibleNode,
            423 => crate::__pgrx_c_bindings::NodeTag::T_FdwRoutine,
            184 => crate::__pgrx_c_bindings::NodeTag::T_FetchStmt,
            23 => crate::__pgrx_c_bindings::NodeTag::T_FieldSelect,
            24 => crate::__pgrx_c_bindings::NodeTag::T_FieldStore,
            442 => crate::__pgrx_c_bindings::NodeTag::T_Float,
            446 => crate::__pgrx_c_bindings::NodeTag::T_ForeignKeyCacheInfo,
            251 => crate::__pgrx_c_bindings::NodeTag::T_ForeignKeyOptInfo,
            268 => crate::__pgrx_c_bindings::NodeTag::T_ForeignPath,
            333 => crate::__pgrx_c_bindings::NodeTag::T_ForeignScan,
            397 => crate::__pgrx_c_bindings::NodeTag::T_ForeignScanState,
            56 => crate::__pgrx_c_bindings::NodeTag::T_FromExpr,
            67 => crate::__pgrx_c_bindings::NodeTag::T_FuncCall,
            13 => crate::__pgrx_c_bindings::NodeTag::T_FuncExpr,
            190 => crate::__pgrx_c_bindings::NodeTag::T_FunctionParameter,
            327 => crate::__pgrx_c_bindings::NodeTag::T_FunctionScan,
            391 => crate::__pgrx_c_bindings::NodeTag::T_FunctionScanState,
            347 => crate::__pgrx_c_bindings::NodeTag::T_Gather,
            348 => crate::__pgrx_c_bindings::NodeTag::T_GatherMerge,
            277 => crate::__pgrx_c_bindings::NodeTag::T_GatherMergePath,
            412 => crate::__pgrx_c_bindings::NodeTag::T_GatherMergeState,
            276 => crate::__pgrx_c_bindings::NodeTag::T_GatherPath,
            411 => crate::__pgrx_c_bindings::NodeTag::T_GatherState,
            451 => crate::__pgrx_c_bindings::NodeTag::T_GenerationContext,
            136 => crate::__pgrx_c_bindings::NodeTag::T_GrantRoleStmt,
            133 => crate::__pgrx_c_bindings::NodeTag::T_GrantStmt,
            343 => crate::__pgrx_c_bindings::NodeTag::T_Group,
            285 => crate::__pgrx_c_bindings::NodeTag::T_GroupPath,
            272 => crate::__pgrx_c_bindings::NodeTag::T_GroupResultPath,
            407 => crate::__pgrx_c_bindings::NodeTag::T_GroupState,
            10 => crate::__pgrx_c_bindings::NodeTag::T_GroupingFunc,
            98 => crate::__pgrx_c_bindings::NodeTag::T_GroupingSet,
            288 => crate::__pgrx_c_bindings::NodeTag::T_GroupingSetData,
            290 => crate::__pgrx_c_bindings::NodeTag::T_GroupingSetsPath,
            349 => crate::__pgrx_c_bindings::NodeTag::T_Hash,
            338 => crate::__pgrx_c_bindings::NodeTag::T_HashJoin,
            402 => crate::__pgrx_c_bindings::NodeTag::T_HashJoinState,
            280 => crate::__pgrx_c_bindings::NodeTag::T_HashPath,
            413 => crate::__pgrx_c_bindings::NodeTag::T_HashState,
            427 => crate::__pgrx_c_bindings::NodeTag::T_IdentifySystemCmd,
            158 => crate::__pgrx_c_bindings::NodeTag::T_ImportForeignSchemaStmt,
            342 => crate::__pgrx_c_bindings::NodeTag::T_IncrementalSort,
            284 => crate::__pgrx_c_bindings::NodeTag::T_IncrementalSortPath,
            406 => crate::__pgrx_c_bindings::NodeTag::T_IncrementalSortState,
            417 => crate::__pgrx_c_bindings::NodeTag::T_IndexAmRoutine,
            261 => crate::__pgrx_c_bindings::NodeTag::T_IndexClause,
            83 => crate::__pgrx_c_bindings::NodeTag::T_IndexElem,
            360 => crate::__pgrx_c_bindings::NodeTag::T_IndexInfo,
            321 => crate::__pgrx_c_bindings::NodeTag::T_IndexOnlyScan,
            385 => crate::__pgrx_c_bindings::NodeTag::T_IndexOnlyScanState,
            250 => crate::__pgrx_c_bindings::NodeTag::T_IndexOptInfo,
            260 => crate::__pgrx_c_bindings::NodeTag::T_IndexPath,
            320 => crate::__pgrx_c_bindings::NodeTag::T_IndexScan,
            384 => crate::__pgrx_c_bindings::NodeTag::T_IndexScanState,
            185 => crate::__pgrx_c_bindings::NodeTag::T_IndexStmt,
            102 => crate::__pgrx_c_bindings::NodeTag::T_InferClause,
            52 => crate::__pgrx_c_bindings::NodeTag::T_InferenceElem,
            193 => crate::__pgrx_c_bindings::NodeTag::T_InlineCodeBlock,
            119 => crate::__pgrx_c_bindings::NodeTag::T_InsertStmt,
            447 => crate::__pgrx_c_bindings::NodeTag::T_IntList,
            441 => crate::__pgrx_c_bindings::NodeTag::T_Integer,
            5 => crate::__pgrx_c_bindings::NodeTag::T_IntoClause,
            0 => crate::__pgrx_c_bindings::NodeTag::T_Invalid,
            253 => crate::__pgrx_c_bindings::NodeTag::T_JoinDomain,
            55 => crate::__pgrx_c_bindings::NodeTag::T_JoinExpr,
            399 => crate::__pgrx_c_bindings::NodeTag::T_JoinState,
            115 => crate::__pgrx_c_bindings::NodeTag::T_JsonAggConstructor,
            117 => crate::__pgrx_c_bindings::NodeTag::T_JsonArrayAgg,
            113 => crate::__pgrx_c_bindings::NodeTag::T_JsonArrayConstructor,
            114 => crate::__pgrx_c_bindings::NodeTag::T_JsonArrayQueryConstructor,
            43 => crate::__pgrx_c_bindings::NodeTag::T_JsonConstructorExpr,
            40 => crate::__pgrx_c_bindings::NodeTag::T_JsonFormat,
            44 => crate::__pgrx_c_bindings::NodeTag::T_JsonIsPredicate,
            111 => crate::__pgrx_c_bindings::NodeTag::T_JsonKeyValue,
            116 => crate::__pgrx_c_bindings::NodeTag::T_JsonObjectAgg,
            112 => crate::__pgrx_c_bindings::NodeTag::T_JsonObjectConstructor,
            110 => crate::__pgrx_c_bindings::NodeTag::T_JsonOutput,
            41 => crate::__pgrx_c_bindings::NodeTag::T_JsonReturning,
            42 => crate::__pgrx_c_bindings::NodeTag::T_JsonValueExpr,
            364 => crate::__pgrx_c_bindings::NodeTag::T_JunkFilter,
            352 => crate::__pgrx_c_bindings::NodeTag::T_Limit,
            297 => crate::__pgrx_c_bindings::NodeTag::T_LimitPath,
            416 => crate::__pgrx_c_bindings::NodeTag::T_LimitState,
            1 => crate::__pgrx_c_bindings::NodeTag::T_List,
            204 => crate::__pgrx_c_bindings::NodeTag::T_ListenStmt,
            212 => crate::__pgrx_c_bindings::NodeTag::T_LoadStmt,
            351 => crate::__pgrx_c_bindings::NodeTag::T_LockRows,
            295 => crate::__pgrx_c_bindings::NodeTag::T_LockRowsPath,
            415 => crate::__pgrx_c_bindings::NodeTag::T_LockRowsState,
            227 => crate::__pgrx_c_bindings::NodeTag::T_LockStmt,
            85 => crate::__pgrx_c_bindings::NodeTag::T_LockingClause,
            339 => crate::__pgrx_c_bindings::NodeTag::T_Material,
            273 => crate::__pgrx_c_bindings::NodeTag::T_MaterialPath,
            403 => crate::__pgrx_c_bindings::NodeTag::T_MaterialState,
            340 => crate::__pgrx_c_bindings::NodeTag::T_Memoize,
            274 => crate::__pgrx_c_bindings::NodeTag::T_MemoizePath,
            404 => crate::__pgrx_c_bindings::NodeTag::T_MemoizeState,
            108 => crate::__pgrx_c_bindings::NodeTag::T_MergeAction,
            366 => crate::__pgrx_c_bindings::NodeTag::T_MergeActionState,
            314 => crate::__pgrx_c_bindings::NodeTag::T_MergeAppend,
            271 => crate::__pgrx_c_bindings::NodeTag::T_MergeAppendPath,
            377 => crate::__pgrx_c_bindings::NodeTag::T_MergeAppendState,
            337 => crate::__pgrx_c_bindings::NodeTag::T_MergeJoin,
            401 => crate::__pgrx_c_bindings::NodeTag::T_MergeJoinState,
            279 => crate::__pgrx_c_bindings::NodeTag::T_MergePath,
            122 => crate::__pgrx_c_bindings::NodeTag::T_MergeStmt,
            107 => crate::__pgrx_c_bindings::NodeTag::T_MergeWhenClause,
            305 => crate::__pgrx_c_bindings::NodeTag::T_MinMaxAggInfo,
            291 => crate::__pgrx_c_bindings::NodeTag::T_MinMaxAggPath,
            37 => crate::__pgrx_c_bindings::NodeTag::T_MinMaxExpr,
            312 => crate::__pgrx_c_bindings::NodeTag::T_ModifyTable,
            296 => crate::__pgrx_c_bindings::NodeTag::T_ModifyTablePath,
            375 => crate::__pgrx_c_bindings::NodeTag::T_ModifyTableState,
            73 => crate::__pgrx_c_bindings::NodeTag::T_MultiAssignRef,
            14 => crate::__pgrx_c_bindings::NodeTag::T_NamedArgExpr,
            331 => crate::__pgrx_c_bindings::NodeTag::T_NamedTuplestoreScan,
            395 => crate::__pgrx_c_bindings::NodeTag::T_NamedTuplestoreScanState,
            335 => crate::__pgrx_c_bindings::NodeTag::T_NestLoop,
            336 => crate::__pgrx_c_bindings::NodeTag::T_NestLoopParam,
            400 => crate::__pgrx_c_bindings::NodeTag::T_NestLoopState,
            278 => crate::__pgrx_c_bindings::NodeTag::T_NestPath,
            51 => crate::__pgrx_c_bindings::NodeTag::T_NextValueExpr,
            203 => crate::__pgrx_c_bindings::NodeTag::T_NotifyStmt,
            17 => crate::__pgrx_c_bindings::NodeTag::T_NullIfExpr,
            45 => crate::__pgrx_c_bindings::NodeTag::T_NullTest,
            134 => crate::__pgrx_c_bindings::NodeTag::T_ObjectWithArgs,
            448 => crate::__pgrx_c_bindings::NodeTag::T_OidList,
            103 => crate::__pgrx_c_bindings::NodeTag::T_OnConflictClause,
            57 => crate::__pgrx_c_bindings::NodeTag::T_OnConflictExpr,
            365 => crate::__pgrx_c_bindings::NodeTag::T_OnConflictSetState,
            15 => crate::__pgrx_c_bindings::NodeTag::T_OpExpr,
            301 => crate::__pgrx_c_bindings::NodeTag::T_OuterJoinClauseInfo,
            126 => crate::__pgrx_c_bindings::NodeTag::T_PLAssignStmt,
            8 => crate::__pgrx_c_bindings::NodeTag::T_Param,
            258 => crate::__pgrx_c_bindings::NodeTag::T_ParamPathInfo,
            61 => crate::__pgrx_c_bindings::NodeTag::T_ParamRef,
            89 => crate::__pgrx_c_bindings::NodeTag::T_PartitionBoundSpec,
            91 => crate::__pgrx_c_bindings::NodeTag::T_PartitionCmd,
            87 => crate::__pgrx_c_bindings::NodeTag::T_PartitionElem,
            354 => crate::__pgrx_c_bindings::NodeTag::T_PartitionPruneInfo,
            357 => crate::__pgrx_c_bindings::NodeTag::T_PartitionPruneStepCombine,
            356 => crate::__pgrx_c_bindings::NodeTag::T_PartitionPruneStepOp,
            90 => crate::__pgrx_c_bindings::NodeTag::T_PartitionRangeDatum,
            88 => crate::__pgrx_c_bindings::NodeTag::T_PartitionSpec,
            355 => crate::__pgrx_c_bindings::NodeTag::T_PartitionedRelPruneInfo,
            259 => crate::__pgrx_c_bindings::NodeTag::T_Path,
            256 => crate::__pgrx_c_bindings::NodeTag::T_PathKey,
            257 => crate::__pgrx_c_bindings::NodeTag::T_PathTarget,
            304 => crate::__pgrx_c_bindings::NodeTag::T_PlaceHolderInfo,
            299 => crate::__pgrx_c_bindings::NodeTag::T_PlaceHolderVar,
            358 => crate::__pgrx_c_bindings::NodeTag::T_PlanInvalItem,
            353 => crate::__pgrx_c_bindings::NodeTag::T_PlanRowMark,
            309 => crate::__pgrx_c_bindings::NodeTag::T_PlannedStmt,
            247 => crate::__pgrx_c_bindings::NodeTag::T_PlannerGlobal,
            248 => crate::__pgrx_c_bindings::NodeTag::T_PlannerInfo,
            306 => crate::__pgrx_c_bindings::NodeTag::T_PlannerParamItem,
            233 => crate::__pgrx_c_bindings::NodeTag::T_PrepareStmt,
            311 => crate::__pgrx_c_bindings::NodeTag::T_ProjectSet,
            282 => crate::__pgrx_c_bindings::NodeTag::T_ProjectSetPath,
            374 => crate::__pgrx_c_bindings::NodeTag::T_ProjectSetState,
            363 => crate::__pgrx_c_bindings::NodeTag::T_ProjectionInfo,
            281 => crate::__pgrx_c_bindings::NodeTag::T_ProjectionPath,
            241 => crate::__pgrx_c_bindings::NodeTag::T_PublicationObjSpec,
            240 => crate::__pgrx_c_bindings::NodeTag::T_PublicationTable,
            58 => crate::__pgrx_c_bindings::NodeTag::T_Query,
            93 => crate::__pgrx_c_bindings::NodeTag::T_RTEPermissionInfo,
            77 => crate::__pgrx_c_bindings::NodeTag::T_RangeFunction,
            76 => crate::__pgrx_c_bindings::NodeTag::T_RangeSubselect,
            78 => crate::__pgrx_c_bindings::NodeTag::T_RangeTableFunc,
            79 => crate::__pgrx_c_bindings::NodeTag::T_RangeTableFuncCol,
            80 => crate::__pgrx_c_bindings::NodeTag::T_RangeTableSample,
            92 => crate::__pgrx_c_bindings::NodeTag::T_RangeTblEntry,
            94 => crate::__pgrx_c_bindings::NodeTag::T_RangeTblFunction,
            54 => crate::__pgrx_c_bindings::NodeTag::T_RangeTblRef,
            3 => crate::__pgrx_c_bindings::NodeTag::T_RangeVar,
            118 => crate::__pgrx_c_bindings::NodeTag::T_RawStmt,
            432 => crate::__pgrx_c_bindings::NodeTag::T_ReadReplicationSlotCmd,
            237 => crate::__pgrx_c_bindings::NodeTag::T_ReassignOwnedStmt,
            315 => crate::__pgrx_c_bindings::NodeTag::T_RecursiveUnion,
            294 => crate::__pgrx_c_bindings::NodeTag::T_RecursiveUnionPath,
            378 => crate::__pgrx_c_bindings::NodeTag::T_RecursiveUnionState,
            224 => crate::__pgrx_c_bindings::NodeTag::T_RefreshMatViewStmt,
            229 => crate::__pgrx_c_bindings::NodeTag::T_ReindexStmt,
            249 => crate::__pgrx_c_bindings::NodeTag::T_RelOptInfo,
            25 => crate::__pgrx_c_bindings::NodeTag::T_RelabelType,
            196 => crate::__pgrx_c_bindings::NodeTag::T_RenameStmt,
            129 => crate::__pgrx_c_bindings::NodeTag::T_ReplicaIdentityStmt,
            72 => crate::__pgrx_c_bindings::NodeTag::T_ResTarget,
            298 => crate::__pgrx_c_bindings::NodeTag::T_RestrictInfo,
            310 => crate::__pgrx_c_bindings::NodeTag::T_Result,
            367 => crate::__pgrx_c_bindings::NodeTag::T_ResultRelInfo,
            373 => crate::__pgrx_c_bindings::NodeTag::T_ResultState,
            362 => crate::__pgrx_c_bindings::NodeTag::T_ReturnSetInfo,
            125 => crate::__pgrx_c_bindings::NodeTag::T_ReturnStmt,
            66 => crate::__pgrx_c_bindings::NodeTag::T_RoleSpec,
            289 => crate::__pgrx_c_bindings::NodeTag::T_RollupData,
            35 => crate::__pgrx_c_bindings::NodeTag::T_RowCompareExpr,
            34 => crate::__pgrx_c_bindings::NodeTag::T_RowExpr,
            303 => crate::__pgrx_c_bindings::NodeTag::T_RowIdentityVarInfo,
            100 => crate::__pgrx_c_bindings::NodeTag::T_RowMarkClause,
            202 => crate::__pgrx_c_bindings::NodeTag::T_RuleStmt,
            38 => crate::__pgrx_c_bindings::NodeTag::T_SQLValueFunction,
            319 => crate::__pgrx_c_bindings::NodeTag::T_SampleScan,
            383 => crate::__pgrx_c_bindings::NodeTag::T_SampleScanState,
            18 => crate::__pgrx_c_bindings::NodeTag::T_ScalarArrayOpExpr,
            381 => crate::__pgrx_c_bindings::NodeTag::T_ScanState,
            181 => crate::__pgrx_c_bindings::NodeTag::T_SecLabelStmt,
            123 => crate::__pgrx_c_bindings::NodeTag::T_SelectStmt,
            318 => crate::__pgrx_c_bindings::NodeTag::T_SeqScan,
            382 => crate::__pgrx_c_bindings::NodeTag::T_SeqScanState,
            370 => crate::__pgrx_c_bindings::NodeTag::T_SetExprState,
            350 => crate::__pgrx_c_bindings::NodeTag::T_SetOp,
            293 => crate::__pgrx_c_bindings::NodeTag::T_SetOpPath,
            414 => crate::__pgrx_c_bindings::NodeTag::T_SetOpState,
            124 => crate::__pgrx_c_bindings::NodeTag::T_SetOperationStmt,
            49 => crate::__pgrx_c_bindings::NodeTag::T_SetToDefault,
            452 => crate::__pgrx_c_bindings::NodeTag::T_SlabContext,
            341 => crate::__pgrx_c_bindings::NodeTag::T_Sort,
            74 => crate::__pgrx_c_bindings::NodeTag::T_SortBy,
            97 => crate::__pgrx_c_bindings::NodeTag::T_SortGroupClause,
            283 => crate::__pgrx_c_bindings::NodeTag::T_SortPath,
            405 => crate::__pgrx_c_bindings::NodeTag::T_SortState,
            300 => crate::__pgrx_c_bindings::NodeTag::T_SpecialJoinInfo,
            431 => crate::__pgrx_c_bindings::NodeTag::T_StartReplicationCmd,
            252 => crate::__pgrx_c_bindings::NodeTag::T_StatisticExtInfo,
            187 => crate::__pgrx_c_bindings::NodeTag::T_StatsElem,
            444 => crate::__pgrx_c_bindings::NodeTag::T_String,
            20 => crate::__pgrx_c_bindings::NodeTag::T_SubLink,
            21 => crate::__pgrx_c_bindings::NodeTag::T_SubPlan,
            371 => crate::__pgrx_c_bindings::NodeTag::T_SubPlanState,
            326 => crate::__pgrx_c_bindings::NodeTag::T_SubqueryScan,
            267 => crate::__pgrx_c_bindings::NodeTag::T_SubqueryScanPath,
            390 => crate::__pgrx_c_bindings::NodeTag::T_SubqueryScanState,
            12 => crate::__pgrx_c_bindings::NodeTag::T_SubscriptingRef,
            436 => crate::__pgrx_c_bindings::NodeTag::T_SupportRequestCost,
            438 => crate::__pgrx_c_bindings::NodeTag::T_SupportRequestIndexCondition,
            440 => crate::__pgrx_c_bindings::NodeTag::T_SupportRequestOptimizeWindowClause,
            437 => crate::__pgrx_c_bindings::NodeTag::T_SupportRequestRows,
            435 => crate::__pgrx_c_bindings::NodeTag::T_SupportRequestSelectivity,
            434 => crate::__pgrx_c_bindings::NodeTag::T_SupportRequestSimplify,
            439 => crate::__pgrx_c_bindings::NodeTag::T_SupportRequestWFuncMonotonic,
            453 => crate::__pgrx_c_bindings::NodeTag::T_TIDBitmap,
            418 => crate::__pgrx_c_bindings::NodeTag::T_TableAmRoutine,
            4 => crate::__pgrx_c_bindings::NodeTag::T_TableFunc,
            329 => crate::__pgrx_c_bindings::NodeTag::T_TableFuncScan,
            393 => crate::__pgrx_c_bindings::NodeTag::T_TableFuncScanState,
            82 => crate::__pgrx_c_bindings::NodeTag::T_TableLikeClause,
            95 => crate::__pgrx_c_bindings::NodeTag::T_TableSampleClause,
            53 => crate::__pgrx_c_bindings::NodeTag::T_TargetEntry,
            265 => crate::__pgrx_c_bindings::NodeTag::T_TidPath,
            266 => crate::__pgrx_c_bindings::NodeTag::T_TidRangePath,
            325 => crate::__pgrx_c_bindings::NodeTag::T_TidRangeScan,
            389 => crate::__pgrx_c_bindings::NodeTag::T_TidRangeScanState,
            324 => crate::__pgrx_c_bindings::NodeTag::T_TidScan,
            388 => crate::__pgrx_c_bindings::NodeTag::T_TidScanState,
            433 => crate::__pgrx_c_bindings::NodeTag::T_TimeLineHistoryCmd,
            206 => crate::__pgrx_c_bindings::NodeTag::T_TransactionStmt,
            421 => crate::__pgrx_c_bindings::NodeTag::T_TriggerData,
            109 => crate::__pgrx_c_bindings::NodeTag::T_TriggerTransition,
            179 => crate::__pgrx_c_bindings::NodeTag::T_TruncateStmt,
            419 => crate::__pgrx_c_bindings::NodeTag::T_TsmRoutine,
            422 => crate::__pgrx_c_bindings::NodeTag::T_TupleTableSlot,
            64 => crate::__pgrx_c_bindings::NodeTag::T_TypeCast,
            59 => crate::__pgrx_c_bindings::NodeTag::T_TypeName,
            346 => crate::__pgrx_c_bindings::NodeTag::T_Unique,
            275 => crate::__pgrx_c_bindings::NodeTag::T_UniquePath,
            410 => crate::__pgrx_c_bindings::NodeTag::T_UniqueState,
            205 => crate::__pgrx_c_bindings::NodeTag::T_UnlistenStmt,
            121 => crate::__pgrx_c_bindings::NodeTag::T_UpdateStmt,
            286 => crate::__pgrx_c_bindings::NodeTag::T_UpperUniquePath,
            221 => crate::__pgrx_c_bindings::NodeTag::T_VacuumRelation,
            220 => crate::__pgrx_c_bindings::NodeTag::T_VacuumStmt,
            328 => crate::__pgrx_c_bindings::NodeTag::T_ValuesScan,
            392 => crate::__pgrx_c_bindings::NodeTag::T_ValuesScanState,
            6 => crate::__pgrx_c_bindings::NodeTag::T_Var,
            139 => crate::__pgrx_c_bindings::NodeTag::T_VariableSetStmt,
            140 => crate::__pgrx_c_bindings::NodeTag::T_VariableShowStmt,
            211 => crate::__pgrx_c_bindings::NodeTag::T_ViewStmt,
            345 => crate::__pgrx_c_bindings::NodeTag::T_WindowAgg,
            292 => crate::__pgrx_c_bindings::NodeTag::T_WindowAggPath,
            409 => crate::__pgrx_c_bindings::NodeTag::T_WindowAggState,
            99 => crate::__pgrx_c_bindings::NodeTag::T_WindowClause,
            75 => crate::__pgrx_c_bindings::NodeTag::T_WindowDef,
            11 => crate::__pgrx_c_bindings::NodeTag::T_WindowFunc,
            369 => crate::__pgrx_c_bindings::NodeTag::T_WindowFuncExprState,
            454 => crate::__pgrx_c_bindings::NodeTag::T_WindowObjectData,
            96 => crate::__pgrx_c_bindings::NodeTag::T_WithCheckOption,
            101 => crate::__pgrx_c_bindings::NodeTag::T_WithClause,
            332 => crate::__pgrx_c_bindings::NodeTag::T_WorkTableScan,
            396 => crate::__pgrx_c_bindings::NodeTag::T_WorkTableScanState,
            449 => crate::__pgrx_c_bindings::NodeTag::T_XidList,
            39 => crate::__pgrx_c_bindings::NodeTag::T_XmlExpr,
            86 => crate::__pgrx_c_bindings::NodeTag::T_XmlSerialize,
            _ => panic!("C enum value has no corresponding Rust variant"),
        }
    }
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__ParallelTransState;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for EnumIdentity_enum__ParallelTransState {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::EnumIdentity for EnumIdentity_enum__ParallelTransState {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__ParallelTransState {
    type Compatible = c::CUnsignedInt;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__PartialFileSetState;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for EnumIdentity_enum__PartialFileSetState {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::EnumIdentity for EnumIdentity_enum__PartialFileSetState {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__PartialFileSetState {
    type Compatible = c::CUnsignedInt;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__PartitionStrategy;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for EnumIdentity_enum__PartitionStrategy {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::EnumIdentity for EnumIdentity_enum__PartitionStrategy {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__PartitionStrategy {
    type Compatible = c::CUnsignedInt;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__ReplicationSlotInvalidationCause;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for EnumIdentity_enum__ReplicationSlotInvalidationCause {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::EnumIdentity for EnumIdentity_enum__ReplicationSlotInvalidationCause {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::enumeration::NumericEnum
    for EnumIdentity_enum__ReplicationSlotInvalidationCause
{
    type Compatible = c::CUnsignedInt;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__TU_UpdateIndexes;
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for EnumIdentity_enum__TU_UpdateIndexes {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::EnumIdentity for EnumIdentity_enum__TU_UpdateIndexes {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__TU_UpdateIndexes {
    type Compatible = c::CUnsignedInt;
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl type_enumerator_seals::Seal_T_ for crate::__pgrx_c_bindings::AggInfo {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl TypeEnumerator_T_ for crate::__pgrx_c_bindings::AggInfo {
    type Type = Self;
    const VALUE: c::CValue<c::CInt> =
        c::CValue::<c::CInt>::new(crate::__pgrx_c_bindings::NodeTag::T_AggInfo as i32);
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl type_enumerator_seals::Seal_T_ for crate::__pgrx_c_bindings::AggTransInfo {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl TypeEnumerator_T_ for crate::__pgrx_c_bindings::AggTransInfo {
    type Type = Self;
    const VALUE: c::CValue<c::CInt> =
        c::CValue::<c::CInt>::new(crate::__pgrx_c_bindings::NodeTag::T_AggTransInfo as i32);
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl type_enumerator_seals::Seal_T_ for crate::__pgrx_c_bindings::Bitmapset {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl TypeEnumerator_T_ for crate::__pgrx_c_bindings::Bitmapset {
    type Type = Self;
    const VALUE: c::CValue<c::CInt> =
        c::CValue::<c::CInt>::new(crate::__pgrx_c_bindings::NodeTag::T_Bitmapset as i32);
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl type_enumerator_seals::Seal_T_ for crate::__pgrx_c_bindings::ErrorSaveContext {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl TypeEnumerator_T_ for crate::__pgrx_c_bindings::ErrorSaveContext {
    type Type = Self;
    const VALUE: c::CValue<c::CInt> =
        c::CValue::<c::CInt>::new(crate::__pgrx_c_bindings::NodeTag::T_ErrorSaveContext as i32);
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl type_enumerator_seals::Seal_T_ for crate::__pgrx_c_bindings::JoinDomain {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl TypeEnumerator_T_ for crate::__pgrx_c_bindings::JoinDomain {
    type Type = Self;
    const VALUE: c::CValue<c::CInt> =
        c::CValue::<c::CInt>::new(crate::__pgrx_c_bindings::NodeTag::T_JoinDomain as i32);
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl type_enumerator_seals::Seal_T_ for crate::__pgrx_c_bindings::JsonAggConstructor {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl TypeEnumerator_T_ for crate::__pgrx_c_bindings::JsonAggConstructor {
    type Type = Self;
    const VALUE: c::CValue<c::CInt> =
        c::CValue::<c::CInt>::new(crate::__pgrx_c_bindings::NodeTag::T_JsonAggConstructor as i32);
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl type_enumerator_seals::Seal_T_ for crate::__pgrx_c_bindings::JsonArrayAgg {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl TypeEnumerator_T_ for crate::__pgrx_c_bindings::JsonArrayAgg {
    type Type = Self;
    const VALUE: c::CValue<c::CInt> =
        c::CValue::<c::CInt>::new(crate::__pgrx_c_bindings::NodeTag::T_JsonArrayAgg as i32);
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl type_enumerator_seals::Seal_T_ for crate::__pgrx_c_bindings::JsonArrayConstructor {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl TypeEnumerator_T_ for crate::__pgrx_c_bindings::JsonArrayConstructor {
    type Type = Self;
    const VALUE: c::CValue<c::CInt> =
        c::CValue::<c::CInt>::new(crate::__pgrx_c_bindings::NodeTag::T_JsonArrayConstructor as i32);
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl type_enumerator_seals::Seal_T_ for crate::__pgrx_c_bindings::JsonArrayQueryConstructor {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl TypeEnumerator_T_ for crate::__pgrx_c_bindings::JsonArrayQueryConstructor {
    type Type = Self;
    const VALUE: c::CValue<c::CInt> = c::CValue::<c::CInt>::new(
        crate::__pgrx_c_bindings::NodeTag::T_JsonArrayQueryConstructor as i32,
    );
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl type_enumerator_seals::Seal_T_ for crate::__pgrx_c_bindings::JsonConstructorExpr {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl TypeEnumerator_T_ for crate::__pgrx_c_bindings::JsonConstructorExpr {
    type Type = Self;
    const VALUE: c::CValue<c::CInt> =
        c::CValue::<c::CInt>::new(crate::__pgrx_c_bindings::NodeTag::T_JsonConstructorExpr as i32);
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl type_enumerator_seals::Seal_T_ for crate::__pgrx_c_bindings::JsonFormat {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl TypeEnumerator_T_ for crate::__pgrx_c_bindings::JsonFormat {
    type Type = Self;
    const VALUE: c::CValue<c::CInt> =
        c::CValue::<c::CInt>::new(crate::__pgrx_c_bindings::NodeTag::T_JsonFormat as i32);
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl type_enumerator_seals::Seal_T_ for crate::__pgrx_c_bindings::JsonIsPredicate {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl TypeEnumerator_T_ for crate::__pgrx_c_bindings::JsonIsPredicate {
    type Type = Self;
    const VALUE: c::CValue<c::CInt> =
        c::CValue::<c::CInt>::new(crate::__pgrx_c_bindings::NodeTag::T_JsonIsPredicate as i32);
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl type_enumerator_seals::Seal_T_ for crate::__pgrx_c_bindings::JsonKeyValue {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl TypeEnumerator_T_ for crate::__pgrx_c_bindings::JsonKeyValue {
    type Type = Self;
    const VALUE: c::CValue<c::CInt> =
        c::CValue::<c::CInt>::new(crate::__pgrx_c_bindings::NodeTag::T_JsonKeyValue as i32);
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl type_enumerator_seals::Seal_T_ for crate::__pgrx_c_bindings::JsonObjectAgg {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl TypeEnumerator_T_ for crate::__pgrx_c_bindings::JsonObjectAgg {
    type Type = Self;
    const VALUE: c::CValue<c::CInt> =
        c::CValue::<c::CInt>::new(crate::__pgrx_c_bindings::NodeTag::T_JsonObjectAgg as i32);
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl type_enumerator_seals::Seal_T_ for crate::__pgrx_c_bindings::JsonObjectConstructor {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl TypeEnumerator_T_ for crate::__pgrx_c_bindings::JsonObjectConstructor {
    type Type = Self;
    const VALUE: c::CValue<c::CInt> = c::CValue::<c::CInt>::new(
        crate::__pgrx_c_bindings::NodeTag::T_JsonObjectConstructor as i32,
    );
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl type_enumerator_seals::Seal_T_ for crate::__pgrx_c_bindings::JsonOutput {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl TypeEnumerator_T_ for crate::__pgrx_c_bindings::JsonOutput {
    type Type = Self;
    const VALUE: c::CValue<c::CInt> =
        c::CValue::<c::CInt>::new(crate::__pgrx_c_bindings::NodeTag::T_JsonOutput as i32);
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl type_enumerator_seals::Seal_T_ for crate::__pgrx_c_bindings::JsonReturning {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl TypeEnumerator_T_ for crate::__pgrx_c_bindings::JsonReturning {
    type Type = Self;
    const VALUE: c::CValue<c::CInt> =
        c::CValue::<c::CInt>::new(crate::__pgrx_c_bindings::NodeTag::T_JsonReturning as i32);
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl type_enumerator_seals::Seal_T_ for crate::__pgrx_c_bindings::JsonValueExpr {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl TypeEnumerator_T_ for crate::__pgrx_c_bindings::JsonValueExpr {
    type Type = Self;
    const VALUE: c::CValue<c::CInt> =
        c::CValue::<c::CInt>::new(crate::__pgrx_c_bindings::NodeTag::T_JsonValueExpr as i32);
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl type_enumerator_seals::Seal_T_ for crate::__pgrx_c_bindings::OuterJoinClauseInfo {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl TypeEnumerator_T_ for crate::__pgrx_c_bindings::OuterJoinClauseInfo {
    type Type = Self;
    const VALUE: c::CValue<c::CInt> =
        c::CValue::<c::CInt>::new(crate::__pgrx_c_bindings::NodeTag::T_OuterJoinClauseInfo as i32);
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl type_enumerator_seals::Seal_T_ for crate::__pgrx_c_bindings::RTEPermissionInfo {}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl TypeEnumerator_T_ for crate::__pgrx_c_bindings::RTEPermissionInfo {
    type Type = Self;
    const VALUE: c::CValue<c::CInt> =
        c::CValue::<c::CInt>::new(crate::__pgrx_c_bindings::NodeTag::T_RTEPermissionInfo as i32);
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl type_enumerator_seals::Seal_T_
    for crate::__pgrx_c_bindings::SupportRequestOptimizeWindowClause
{
}
#[cfg(any(feature = "pg16", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl TypeEnumerator_T_ for crate::__pgrx_c_bindings::SupportRequestOptimizeWindowClause {
    type Type = Self;
    const VALUE: c::CValue<c::CInt> = c::CValue::<c::CInt>::new(
        crate::__pgrx_c_bindings::NodeTag::T_SupportRequestOptimizeWindowClause as i32,
    );
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn(i64, i64) -> bool>>()
        == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn(i64, i64) -> bool>>(
    ) == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__long__0c__long__0r;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__long__0c__long__0r {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__long__0c__long__0r {
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<unsafe extern "C-unwind" fn(i64, i64) -> bool>,
    >;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::SlruCtlData,
                *mut ::core::ffi::c_char,
                i64,
                *mut ::core::ffi::c_void,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::SlruCtlData,
                *mut ::core::ffi::c_char,
                i64,
                *mut ::core::ffi::c_void,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__SlruCtlData__0p__0c__char__0p__0c__long__0c__void__0p__0r;
#[cfg(any(feature = "pg17", feature = "pg18"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__SlruCtlData__0p__0c__char__0p__0c__long__0c__void__0p__0r {}
#[cfg(any(feature = "pg17", feature = "pg18"))]
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__struct__SlruCtlData__0p__0c__char__0p__0c__long__0c__void__0p__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::SlruCtlData, *mut ::core::ffi::c_char, i64, *mut ::core::ffi::c_void) -> bool>>;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableScanDescData,
                *mut crate::__pgrx_c_bindings::ReadStream,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableScanDescData,
                *mut crate::__pgrx_c_bindings::ReadStream,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__struct__ReadStream__0p__0r;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed
    for Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__struct__ReadStream__0p__0r
{
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__struct__ReadStream__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableScanDescData,
                *mut crate::__pgrx_c_bindings::ReadStream,
            ) -> bool,
        >,
    >;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::ReadStream>() == 8);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::ReadStream>() == 8);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<
    A0: c::expression::ImplicitTo<
            c::expression::CPointer<
                c::expression::CRecord<crate::__pgrx_c_bindings::TableScanDescData>,
                c::expression::ReadWrite,
            >,
        >,
    A1: c::expression::ImplicitTo<
            c::expression::CPointer<
                c::expression::COpaque<crate::__pgrx_c_bindings::ReadStream>,
                c::expression::ReadWrite,
            >,
        >,
> c::expression::Call<(A0, A1)>
    for Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__struct__ReadStream__0p__0r
{
    type Output = <c::CBool as c::expression::CType>::Value;
    unsafe fn call(pointer: Self::Pointer, args: (A0, A1)) -> Self::Output {
        let function = pointer.expect("C indirect call requires a non-null function pointer");
        const {
            assert!(!::core::mem::needs_drop::<A0>());
        }
        const _: () =
            assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::TableScanDescData>());
        let native0 = <c::expression::CPointer<
            c::expression::CRecord<crate::__pgrx_c_bindings::TableScanDescData>,
            c::expression::ReadWrite,
        > as c::expression::CType>::into_storage(c::expression::implicit::<
            c::expression::CPointer<
                c::expression::CRecord<crate::__pgrx_c_bindings::TableScanDescData>,
                c::expression::ReadWrite,
            >,
            _,
        >(args.0));
        const {
            assert!(!::core::mem::needs_drop::<A1>());
        }
        const _: () =
            assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::ReadStream>());
        let native1 = <c::expression::CPointer<
            c::expression::COpaque<crate::__pgrx_c_bindings::ReadStream>,
            c::expression::ReadWrite,
        > as c::expression::CType>::into_storage(c::expression::implicit::<
            c::expression::CPointer<
                c::expression::COpaque<crate::__pgrx_c_bindings::ReadStream>,
                c::expression::ReadWrite,
            >,
            _,
        >(args.1));
        // SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
        let result =
            unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0, native1)) };
        <c::CBool as c::expression::CType>::from_storage(result)
    }
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TupleTableSlot) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TupleTableSlot) -> bool,
        >,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__TupleTableSlot__0p__0r;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__TupleTableSlot__0p__0r {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__struct__TupleTableSlot__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TupleTableSlot) -> bool,
        >,
    >;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<
    A0: c::expression::ImplicitTo<
            c::expression::CPointer<
                c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot>,
                c::expression::ReadWrite,
            >,
        >,
> c::expression::Call<(A0,)> for Signature_0x5f426f6f6c__0l__struct__TupleTableSlot__0p__0r
{
    type Output = <c::CBool as c::expression::CType>::Value;
    unsafe fn call(pointer: Self::Pointer, args: (A0,)) -> Self::Output {
        let function = pointer.expect("C indirect call requires a non-null function pointer");
        const {
            assert!(!::core::mem::needs_drop::<A0>());
        }
        const _: () =
            assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::TupleTableSlot>());
        let native0 = <c::expression::CPointer<
            c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot>,
            c::expression::ReadWrite,
        > as c::expression::CType>::into_storage(c::expression::implicit::<
            c::expression::CPointer<
                c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot>,
                c::expression::ReadWrite,
            >,
            _,
        >(args.0));
        // SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
        let result = unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0)) };
        <c::CBool as c::expression::CType>::from_storage(result)
    }
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(i32, i32, *mut i32) -> bool>,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(i32, i32, *mut i32) -> bool>,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__int__0c__int__0c__int__0p__0r;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__int__0c__int__0c__int__0p__0r {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__int__0c__int__0c__int__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<unsafe extern "C-unwind" fn(i32, i32, *mut i32) -> bool>,
    >;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(i64, i64, *mut i64) -> bool>,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(i64, i64, *mut i64) -> bool>,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__long__0c__long__0c__long__0p__0r;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__long__0c__long__0c__long__0p__0r {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__long__0c__long__0c__long__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<unsafe extern "C-unwind" fn(i64, i64, *mut i64) -> bool>,
    >;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(i16, i16, *mut i16) -> bool>,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(i16, i16, *mut i16) -> bool>,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__short__0c__short__0c__short__0p__0r;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__short__0c__short__0c__short__0p__0r {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__short__0c__short__0c__short__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<unsafe extern "C-unwind" fn(i16, i16, *mut i16) -> bool>,
    >;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(u32, u32, *mut u32) -> bool>,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(u32, u32, *mut u32) -> bool>,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__unsigned__int__0c__unsigned__int__0c__unsigned__int__0p__0r;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed
    for Signature_0x5f426f6f6c__0l__unsigned__int__0c__unsigned__int__0c__unsigned__int__0p__0r
{
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__unsigned__int__0c__unsigned__int__0c__unsigned__int__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<unsafe extern "C-unwind" fn(u32, u32, *mut u32) -> bool>,
    >;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(u16, u16, *mut u16) -> bool>,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(u16, u16, *mut u16) -> bool>,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__unsigned__short__0c__unsigned__short__0c__unsigned__short__0p__0r;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__unsigned__short__0c__unsigned__short__0c__unsigned__short__0p__0r {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__unsigned__short__0c__unsigned__short__0c__unsigned__short__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(u16, u16, *mut u16) -> bool>>;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<
    A0: c::expression::ImplicitTo<
            c::expression::CPointer<
                c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
                c::expression::ReadOnly,
            >,
        >,
> c::expression::Call<(A0,)> for Signature_char__0p__0l__const__char__0p__0r
{
    type Output = <c::expression::CPointer<
        c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
        c::expression::ReadWrite,
    > as c::expression::CType>::Value;
    unsafe fn call(pointer: Self::Pointer, args: (A0,)) -> Self::Output {
        let function = pointer.expect("C indirect call requires a non-null function pointer");
        const {
            assert!(!::core::mem::needs_drop::<A0>());
        }
        const _: () = assert!(!::core::mem::needs_drop::<*const ::core::ffi::c_char>());
        let native0 = <c::expression::CPointer<
            c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
            c::expression::ReadOnly,
        > as c::expression::CType>::into_storage(c::expression::implicit::<
            c::expression::CPointer<
                c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
                c::expression::ReadOnly,
            >,
            _,
        >(args.0));
        // SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
        let result = unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0)) };
        <c::expression::CPointer<
            c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
            c::expression::ReadWrite,
        > as c::expression::CType>::from_storage(result)
    }
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::WalReceiverConn,
                *const ::core::ffi::c_char,
                bool,
                bool,
                bool,
                u32,
                *mut u64,
            ) -> *mut ::core::ffi::c_char,
        >,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::WalReceiverConn,
                *const ::core::ffi::c_char,
                bool,
                bool,
                bool,
                u32,
                *mut u64,
            ) -> *mut ::core::ffi::c_char,
        >,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_char__0p__0l__struct__WalReceiverConn__0p__0c__const__char__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__CRSSnapshotAction__0c__unsigned__long__0p__0r;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_char__0p__0l__struct__WalReceiverConn__0p__0c__const__char__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__CRSSnapshotAction__0c__unsigned__long__0p__0r {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_char__0p__0l__struct__WalReceiverConn__0p__0c__const__char__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__CRSSnapshotAction__0c__unsigned__long__0p__0r {
 type Physical = PhysicalFunction_C_unwind_7<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::WalReceiverConn, *const ::core::ffi::c_char, bool, bool, bool, u32, *mut u64) -> *mut ::core::ffi::c_char>>;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<A0: c::expression::ImplicitTo<c::expression::CPointer<c::expression::COpaque<crate::__pgrx_c_bindings::WalReceiverConn>, c::expression::ReadWrite>>, A1: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadOnly>>, A2: c::expression::ImplicitTo<c::CBool>, A3: c::expression::ImplicitTo<c::CBool>, A4: c::expression::ImplicitTo<c::CBool>, A5: c::expression::ImplicitTo<c::expression::CEnumObject<crate::__pgrx_c_generated::EnumIdentity_CRSSnapshotAction, c::CUnsignedInt, u32>>, A6: c::expression::ImplicitTo<c::expression::CPointer<c::CUnsignedLong, c::expression::ReadWrite>>> c::expression::Call<(A0,A1,A2,A3,A4,A5,A6,)> for Signature_char__0p__0l__struct__WalReceiverConn__0p__0c__const__char__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__CRSSnapshotAction__0c__unsigned__long__0p__0r {
 type Output = <c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadWrite> as c::expression::CType>::Value;
 unsafe fn call(pointer: Self::Pointer, args: (A0,A1,A2,A3,A4,A5,A6,)) -> Self::Output {
let function = pointer.expect("C indirect call requires a non-null function pointer");
const { assert!(!::core::mem::needs_drop::<A0>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::WalReceiverConn>());
let native0 = <c::expression::CPointer<c::expression::COpaque<crate::__pgrx_c_bindings::WalReceiverConn>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::COpaque<crate::__pgrx_c_bindings::WalReceiverConn>, c::expression::ReadWrite>, _>(args.0));
const { assert!(!::core::mem::needs_drop::<A1>()); }
const _: () = assert!(!::core::mem::needs_drop::<*const ::core::ffi::c_char>());
let native1 = <c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadOnly> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadOnly>, _>(args.1));
const { assert!(!::core::mem::needs_drop::<A2>()); }
const _: () = assert!(!::core::mem::needs_drop::<bool>());
let native2 = <c::CBool as c::expression::CType>::into_storage(c::expression::implicit::<c::CBool, _>(args.2));
const { assert!(!::core::mem::needs_drop::<A3>()); }
const _: () = assert!(!::core::mem::needs_drop::<bool>());
let native3 = <c::CBool as c::expression::CType>::into_storage(c::expression::implicit::<c::CBool, _>(args.3));
const { assert!(!::core::mem::needs_drop::<A4>()); }
const _: () = assert!(!::core::mem::needs_drop::<bool>());
let native4 = <c::CBool as c::expression::CType>::into_storage(c::expression::implicit::<c::CBool, _>(args.4));
const { assert!(!::core::mem::needs_drop::<A5>()); }
const _: () = assert!(!::core::mem::needs_drop::<u32>());
let native5 = <c::expression::CEnumObject<crate::__pgrx_c_generated::EnumIdentity_CRSSnapshotAction, c::CUnsignedInt, u32> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CEnumObject<crate::__pgrx_c_generated::EnumIdentity_CRSSnapshotAction, c::CUnsignedInt, u32>, _>(args.5));
const { assert!(!::core::mem::needs_drop::<A6>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut u64>());
let native6 = <c::expression::CPointer<c::CUnsignedLong, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::CUnsignedLong, c::expression::ReadWrite>, _>(args.6));
// SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
let result = unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0, native1, native2, native3, native4, native5, native6)) };
<c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadWrite> as c::expression::CType>::from_storage(result)
}
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<A0: c::expression::ImplicitTo<c::CUnsignedLong>> c::expression::Call<(A0,)>
    for Signature_int__0l__unsigned__long__0r
{
    type Output = <c::CInt as c::expression::CType>::Value;
    unsafe fn call(pointer: Self::Pointer, args: (A0,)) -> Self::Output {
        let function = pointer.expect("C indirect call requires a non-null function pointer");
        const {
            assert!(!::core::mem::needs_drop::<A0>());
        }
        const _: () = assert!(!::core::mem::needs_drop::<u64>());
        let native0 =
            <c::CUnsignedLong as c::expression::CType>::into_storage(c::expression::implicit::<
                c::CUnsignedLong,
                _,
            >(args.0));
        // SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
        let result = unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0)) };
        <c::CInt as c::expression::CType>::from_storage(result)
    }
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn(u64, u64) -> i32>>()
        == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn(u64, u64) -> i32>>()
        == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__unsigned__long__0c__unsigned__long__0r;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_int__0l__unsigned__long__0c__unsigned__long__0r {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_int__0l__unsigned__long__0c__unsigned__long__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<unsafe extern "C-unwind" fn(u64, u64) -> i32>,
    >;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                i32,
                *const crate::__pgrx_c_bindings::iovec,
                i32,
                i64,
            ) -> i64,
        >,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                i32,
                *const crate::__pgrx_c_bindings::iovec,
                i32,
                i64,
            ) -> i64,
        >,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_long__0l__int__0c__const__struct__iovec__0p__0c__int__0c__long__0r;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed
    for Signature_long__0l__int__0c__const__struct__iovec__0p__0c__int__0c__long__0r
{
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_long__0l__int__0c__const__struct__iovec__0p__0c__int__0c__long__0r
{
    type Physical = PhysicalFunction_C_unwind_4<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                i32,
                *const crate::__pgrx_c_bindings::iovec,
                i32,
                i64,
            ) -> i64,
        >,
    >;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(i32, *const ::core::ffi::c_void, u64, i64) -> i64,
        >,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(i32, *const ::core::ffi::c_void, u64, i64) -> i64,
        >,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_long__0l__int__0c__const__void__0p__0c__unsigned__long__0c__long__0r;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed
    for Signature_long__0l__int__0c__const__void__0p__0c__unsigned__long__0c__long__0r
{
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_long__0l__int__0c__const__void__0p__0c__unsigned__long__0c__long__0r
{
    type Physical = PhysicalFunction_C_unwind_4<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(i32, *const ::core::ffi::c_void, u64, i64) -> i64,
        >,
    >;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(i32, *const ::core::ffi::c_void, usize, i64, u32) -> isize,
        >,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(i32, *const ::core::ffi::c_void, usize, i64, u32) -> isize,
        >,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_long__0l__int__0c__const__void__0p__0c__unsigned__long__0c__long__0c__unsigned__int__0r;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_long__0l__int__0c__const__void__0p__0c__unsigned__long__0c__long__0c__unsigned__int__0r {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_long__0l__int__0c__const__void__0p__0c__unsigned__long__0c__long__0c__unsigned__int__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(i32, *const ::core::ffi::c_void, usize, i64, u32) -> isize>>;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(i32, *mut ::core::ffi::c_void, u64, i64) -> i64,
        >,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(i32, *mut ::core::ffi::c_void, u64, i64) -> i64,
        >,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_long__0l__int__0c__void__0p__0c__unsigned__long__0c__long__0r;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_long__0l__int__0c__void__0p__0c__unsigned__long__0c__long__0r {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_long__0l__int__0c__void__0p__0c__unsigned__long__0c__long__0r
{
    type Physical = PhysicalFunction_C_unwind_4<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(i32, *mut ::core::ffi::c_void, u64, i64) -> i64,
        >,
    >;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(i32, *mut ::core::ffi::c_void, usize, i64, u32) -> isize,
        >,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(i32, *mut ::core::ffi::c_void, usize, i64, u32) -> isize,
        >,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_long__0l__int__0c__void__0p__0c__unsigned__long__0c__long__0c__unsigned__int__0r;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed
    for Signature_long__0l__int__0c__void__0p__0c__unsigned__long__0c__long__0c__unsigned__int__0r
{
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_long__0l__int__0c__void__0p__0c__unsigned__long__0c__long__0c__unsigned__int__0r
{
    type Physical = PhysicalFunction_C_unwind_5<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(i32, *mut ::core::ffi::c_void, usize, i64, u32) -> isize,
        >,
    >;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>,
                crate::__pgrx_c_bindings::TransactionId,
            ) -> ::core::mem::MaybeUninit<
                crate::__pgrx_c_bindings::FullTransactionId,
            >,
        >,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>,
                crate::__pgrx_c_bindings::TransactionId,
            ) -> ::core::mem::MaybeUninit<
                crate::__pgrx_c_bindings::FullTransactionId,
            >,
        >,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__FullTransactionId__0l__struct__FullTransactionId__0c__unsigned__int__0r;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed
    for Signature_struct__FullTransactionId__0l__struct__FullTransactionId__0c__unsigned__int__0r
{
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_struct__FullTransactionId__0l__struct__FullTransactionId__0c__unsigned__int__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::FullTransactionId>,
                crate::__pgrx_c_bindings::TransactionId,
            ) -> ::core::mem::MaybeUninit<
                crate::__pgrx_c_bindings::FullTransactionId,
            >,
        >,
    >;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(usize, u32) -> *mut crate::__pgrx_c_bindings::Node,
        >,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(usize, u32) -> *mut crate::__pgrx_c_bindings::Node,
        >,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__Node__0p__0l__unsigned__long__0c__enum__NodeTag__0r;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_struct__Node__0p__0l__unsigned__long__0c__enum__NodeTag__0r {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_struct__Node__0p__0l__unsigned__long__0c__enum__NodeTag__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(usize, u32) -> *mut crate::__pgrx_c_bindings::Node,
        >,
    >;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_char,
                bool,
                bool,
                bool,
                *const ::core::ffi::c_char,
                *mut *mut ::core::ffi::c_char,
            )
                -> *mut crate::__pgrx_c_bindings::WalReceiverConn,
        >,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_char,
                bool,
                bool,
                bool,
                *const ::core::ffi::c_char,
                *mut *mut ::core::ffi::c_char,
            )
                -> *mut crate::__pgrx_c_bindings::WalReceiverConn,
        >,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__WalReceiverConn__0p__0l__const__char__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__const__char__0p__0c__char__0p__0p__0r;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_struct__WalReceiverConn__0p__0l__const__char__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__const__char__0p__0c__char__0p__0p__0r {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_struct__WalReceiverConn__0p__0l__const__char__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__const__char__0p__0c__char__0p__0p__0r {
 type Physical = PhysicalFunction_C_unwind_6<::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_char, bool, bool, bool, *const ::core::ffi::c_char, *mut *mut ::core::ffi::c_char) -> *mut crate::__pgrx_c_bindings::WalReceiverConn>>;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<A0: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadOnly>>, A1: c::expression::ImplicitTo<c::CBool>, A2: c::expression::ImplicitTo<c::CBool>, A3: c::expression::ImplicitTo<c::CBool>, A4: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadOnly>>, A5: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadWrite>, c::expression::ReadWrite>>> c::expression::Call<(A0,A1,A2,A3,A4,A5,)> for Signature_struct__WalReceiverConn__0p__0l__const__char__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__const__char__0p__0c__char__0p__0p__0r {
 type Output = <c::expression::CPointer<c::expression::COpaque<crate::__pgrx_c_bindings::WalReceiverConn>, c::expression::ReadWrite> as c::expression::CType>::Value;
 unsafe fn call(pointer: Self::Pointer, args: (A0,A1,A2,A3,A4,A5,)) -> Self::Output {
let function = pointer.expect("C indirect call requires a non-null function pointer");
const { assert!(!::core::mem::needs_drop::<A0>()); }
const _: () = assert!(!::core::mem::needs_drop::<*const ::core::ffi::c_char>());
let native0 = <c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadOnly> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadOnly>, _>(args.0));
const { assert!(!::core::mem::needs_drop::<A1>()); }
const _: () = assert!(!::core::mem::needs_drop::<bool>());
let native1 = <c::CBool as c::expression::CType>::into_storage(c::expression::implicit::<c::CBool, _>(args.1));
const { assert!(!::core::mem::needs_drop::<A2>()); }
const _: () = assert!(!::core::mem::needs_drop::<bool>());
let native2 = <c::CBool as c::expression::CType>::into_storage(c::expression::implicit::<c::CBool, _>(args.2));
const { assert!(!::core::mem::needs_drop::<A3>()); }
const _: () = assert!(!::core::mem::needs_drop::<bool>());
let native3 = <c::CBool as c::expression::CType>::into_storage(c::expression::implicit::<c::CBool, _>(args.3));
const { assert!(!::core::mem::needs_drop::<A4>()); }
const _: () = assert!(!::core::mem::needs_drop::<*const ::core::ffi::c_char>());
let native4 = <c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadOnly> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadOnly>, _>(args.4));
const { assert!(!::core::mem::needs_drop::<A5>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut *mut ::core::ffi::c_char>());
let native5 = <c::expression::CPointer<c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadWrite>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadWrite>, c::expression::ReadWrite>, _>(args.5));
// SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
let result = unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0, native1, native2, native3, native4, native5)) };
<c::expression::CPointer<c::expression::COpaque<crate::__pgrx_c_bindings::WalReceiverConn>, c::expression::ReadWrite> as c::expression::CType>::from_storage(result)
}
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                i32,
                usize,
                usize,
            ) -> *mut crate::__pgrx_c_bindings::dsa_area,
        >,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                i32,
                usize,
                usize,
            ) -> *mut crate::__pgrx_c_bindings::dsa_area,
        >,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__dsa_area__0p__0l__int__0c__unsigned__long__0c__unsigned__long__0r;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed
    for Signature_struct__dsa_area__0p__0l__int__0c__unsigned__long__0c__unsigned__long__0r
{
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_struct__dsa_area__0p__0l__int__0c__unsigned__long__0c__unsigned__long__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                i32,
                usize,
                usize,
            ) -> *mut crate::__pgrx_c_bindings::dsa_area,
        >,
    >;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
                usize,
                i32,
                *mut crate::__pgrx_c_bindings::dsm_segment,
                usize,
                usize,
            ) -> *mut crate::__pgrx_c_bindings::dsa_area,
        >,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
                usize,
                i32,
                *mut crate::__pgrx_c_bindings::dsm_segment,
                usize,
                usize,
            ) -> *mut crate::__pgrx_c_bindings::dsa_area,
        >,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__dsa_area__0p__0l__void__0p__0c__unsigned__long__0c__int__0c__struct__dsm_segment__0p__0c__unsigned__long__0c__unsigned__long__0r;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_struct__dsa_area__0p__0l__void__0p__0c__unsigned__long__0c__int__0c__struct__dsm_segment__0p__0c__unsigned__long__0c__unsigned__long__0r {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_struct__dsa_area__0p__0l__void__0p__0c__unsigned__long__0c__int__0c__struct__dsm_segment__0p__0c__unsigned__long__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_6<::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_void, usize, i32, *mut crate::__pgrx_c_bindings::dsm_segment, usize, usize) -> *mut crate::__pgrx_c_bindings::dsa_area>>;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ReadStream,
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
            ) -> u32,
        >,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ReadStream,
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
            ) -> u32,
        >,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__int__0l__struct__ReadStream__0p__0c__void__0p__0c__void__0p__0r;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed
    for Signature_unsigned__int__0l__struct__ReadStream__0p__0c__void__0p__0c__void__0p__0r
{
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__int__0l__struct__ReadStream__0p__0c__void__0p__0c__void__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ReadStream,
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
            ) -> u32,
        >,
    >;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_char, i32) -> u64>,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_char, i32) -> u64>,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__const__char__0p__0c__int__0r;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_unsigned__long__0l__const__char__0p__0c__int__0r {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__long__0l__const__char__0p__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_char, i32) -> u64>,
    >;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const ::core::ffi::c_char, i32, u8) -> u64,
        >,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const ::core::ffi::c_char, i32, u8) -> u64,
        >,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__const__char__0p__0c__int__0c__unsigned__char__0r;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed
    for Signature_unsigned__long__0l__const__char__0p__0c__int__0c__unsigned__char__0r
{
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__long__0l__const__char__0p__0c__int__0c__unsigned__char__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const ::core::ffi::c_char, i32, u8) -> u64,
        >,
    >;
}
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn(i32, i32) -> usize>>(
    ) == 8
);
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn(i32, i32) -> usize>>(
    ) == 8
);
#[cfg(feature = "pg17")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__int__0c__int__0r;
#[cfg(feature = "pg17")]
impl c::sealed::Sealed for Signature_unsigned__long__0l__int__0c__int__0r {}
#[cfg(feature = "pg17")]
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__int__0c__int__0r {
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<unsafe extern "C-unwind" fn(i32, i32) -> usize>,
    >;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::IndexInfo,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::IndexInfo,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__RelationData__0p__0c__struct__IndexInfo__0p__0r;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed
    for Signature_void__0l__struct__RelationData__0p__0c__struct__IndexInfo__0p__0r
{
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__RelationData__0p__0c__struct__IndexInfo__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::IndexInfo,
            ) -> (),
        >,
    >;
}
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::WalReceiverConn,
                *const ::core::ffi::c_char,
                bool,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::WalReceiverConn,
                *const ::core::ffi::c_char,
                bool,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg17")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__WalReceiverConn__0p__0c__const__char__0p__0c__0x5f426f6f6c__0r;
#[cfg(feature = "pg17")]
impl c::sealed::Sealed
    for Signature_void__0l__struct__WalReceiverConn__0p__0c__const__char__0p__0c__0x5f426f6f6c__0r
{
}
#[cfg(feature = "pg17")]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__struct__WalReceiverConn__0p__0c__const__char__0p__0c__0x5f426f6f6c__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::WalReceiverConn,
                *const ::core::ffi::c_char,
                bool,
            ) -> (),
        >,
    >;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::size_of::<bool>() == 1);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::align_of::<bool>() == 1);
#[cfg(feature = "pg17")]
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<
    A0: c::expression::ImplicitTo<
            c::expression::CPointer<
                c::expression::COpaque<crate::__pgrx_c_bindings::WalReceiverConn>,
                c::expression::ReadWrite,
            >,
        >,
    A1: c::expression::ImplicitTo<
            c::expression::CPointer<
                c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
                c::expression::ReadOnly,
            >,
        >,
    A2: c::expression::ImplicitTo<c::CBool>,
> c::expression::Call<(A0, A1, A2)>
    for Signature_void__0l__struct__WalReceiverConn__0p__0c__const__char__0p__0c__0x5f426f6f6c__0r
{
    type Output = ();
    unsafe fn call(pointer: Self::Pointer, args: (A0, A1, A2)) -> Self::Output {
        let function = pointer.expect("C indirect call requires a non-null function pointer");
        const {
            assert!(!::core::mem::needs_drop::<A0>());
        }
        const _: () =
            assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::WalReceiverConn>());
        let native0 = <c::expression::CPointer<
            c::expression::COpaque<crate::__pgrx_c_bindings::WalReceiverConn>,
            c::expression::ReadWrite,
        > as c::expression::CType>::into_storage(c::expression::implicit::<
            c::expression::CPointer<
                c::expression::COpaque<crate::__pgrx_c_bindings::WalReceiverConn>,
                c::expression::ReadWrite,
            >,
            _,
        >(args.0));
        const {
            assert!(!::core::mem::needs_drop::<A1>());
        }
        const _: () = assert!(!::core::mem::needs_drop::<*const ::core::ffi::c_char>());
        let native1 = <c::expression::CPointer<
            c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
            c::expression::ReadOnly,
        > as c::expression::CType>::into_storage(c::expression::implicit::<
            c::expression::CPointer<
                c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
                c::expression::ReadOnly,
            >,
            _,
        >(args.1));
        const {
            assert!(!::core::mem::needs_drop::<A2>());
        }
        const _: () = assert!(!::core::mem::needs_drop::<bool>());
        let native2 = <c::CBool as c::expression::CType>::into_storage(c::expression::implicit::<
            c::CBool,
            _,
        >(args.2));
        // SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
        let result = unsafe {
            crate::ffi::pg_guard_ffi_boundary(move || function(native0, native1, native2))
        };
        result
    }
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::WalReceiverConn>() == 8);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::WalReceiverConn>() == 8);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                usize,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                usize,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__void__0p__0c__const__void__0p__0c__unsigned__long__0c__void__0p__0r;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed
    for Signature_void__0l__void__0p__0c__const__void__0p__0c__unsigned__long__0c__void__0p__0r
{
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__void__0p__0c__const__void__0p__0c__unsigned__long__0c__void__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_4<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                usize,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
    >;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ResourceOwnerData,
                crate::__pgrx_c_bindings::Datum,
                *const crate::__pgrx_c_bindings::ResourceOwnerDesc,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ResourceOwnerData,
                crate::__pgrx_c_bindings::Datum,
                *const crate::__pgrx_c_bindings::ResourceOwnerDesc,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__ResourceOwnerData__0p__0c__unsigned__long__0c__const__struct__ResourceOwnerDesc__0p__0r;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_void__0l__struct__ResourceOwnerData__0p__0c__unsigned__long__0c__const__struct__ResourceOwnerDesc__0p__0r {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__ResourceOwnerData__0p__0c__unsigned__long__0c__const__struct__ResourceOwnerDesc__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ResourceOwnerData, crate::__pgrx_c_bindings::Datum, *const crate::__pgrx_c_bindings::ResourceOwnerDesc) -> ()>>;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::SMgrRelationData,
                i32,
                u32,
                *mut *const ::core::ffi::c_void,
                u32,
                bool,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::SMgrRelationData,
                i32,
                u32,
                *mut *const ::core::ffi::c_void,
                u32,
                bool,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__SMgrRelationData__0p__0c__enum__ForkNumber__0c__unsigned__int__0c__const__void__0p__0p__0c__unsigned__int__0c__0x5f426f6f6c__0r;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_void__0l__struct__SMgrRelationData__0p__0c__enum__ForkNumber__0c__unsigned__int__0c__const__void__0p__0p__0c__unsigned__int__0c__0x5f426f6f6c__0r {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__SMgrRelationData__0p__0c__enum__ForkNumber__0c__unsigned__int__0c__const__void__0p__0p__0c__unsigned__int__0c__0x5f426f6f6c__0r {
 type Physical = PhysicalFunction_C_unwind_6<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::SMgrRelationData, i32, u32, *mut *const ::core::ffi::c_void, u32, bool) -> ()>>;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::SMgrRelationData,
                i32,
                u32,
                *mut *mut ::core::ffi::c_void,
                u32,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::SMgrRelationData,
                i32,
                u32,
                *mut *mut ::core::ffi::c_void,
                u32,
            ) -> (),
        >,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__SMgrRelationData__0p__0c__enum__ForkNumber__0c__unsigned__int__0c__void__0p__0p__0c__unsigned__int__0r;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_void__0l__struct__SMgrRelationData__0p__0c__enum__ForkNumber__0c__unsigned__int__0c__void__0p__0p__0c__unsigned__int__0r {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__SMgrRelationData__0p__0c__enum__ForkNumber__0c__unsigned__int__0c__void__0p__0p__0c__unsigned__int__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::SMgrRelationData, i32, u32, *mut *mut ::core::ffi::c_void, u32) -> ()>>;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::MemoryContextData,
                usize,
                i32,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::MemoryContextData,
                usize,
                i32,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0p__0l__struct__MemoryContextData__0p__0c__unsigned__long__0c__int__0r;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed
    for Signature_void__0p__0l__struct__MemoryContextData__0p__0c__unsigned__long__0c__int__0r
{
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0p__0l__struct__MemoryContextData__0p__0c__unsigned__long__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::MemoryContextData,
                usize,
                i32,
            ) -> *mut ::core::ffi::c_void,
        >,
    >;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
                usize,
                i32,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
                usize,
                i32,
            ) -> *mut ::core::ffi::c_void,
        >,
    >() == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0p__0l__void__0p__0c__unsigned__long__0c__int__0r;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for Signature_void__0p__0l__void__0p__0c__unsigned__long__0c__int__0r {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeFunctionSignature
    for Signature_void__0p__0l__void__0p__0c__unsigned__long__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
                usize,
                i32,
            ) -> *mut ::core::ffi::c_void,
        >,
    >;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ParsedWord>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ParsedWord>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParsedWord, flags) == 0);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParsedWord>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParsedWord, len) == 2);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParsedWord>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 2;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParsedWord, nvariant) == 4);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nvariant>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParsedWord>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParsedWord, alen) == 6);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_alen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParsedWord>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 6;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TuplesortPublic>() == 112);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TuplesortPublic>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TuplesortPublic, haveDatum1) == 72);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_haveDatum1>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TuplesortPublic>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TuplesortPublic, nKeys) == 76);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nKeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TuplesortPublic>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 76;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TuplesortPublic, sortopt) == 96);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sortopt>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TuplesortPublic>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TuplesortPublic, tuples) == 100);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tuples>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TuplesortPublic>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 100;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_procNumber;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::VirtualTransactionId, procNumber) == 0
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_procNumber>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VirtualTransactionId>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VirtualTransactionId, <<c::expression::CRecord<crate::__pgrx_c_bindings::VirtualTransactionId> as c::expression::OrdinaryField<Field_procNumber>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).procNumber) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::WalSndCtlData>() == 112);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::WalSndCtlData>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BufferUsage>() == 128);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BufferUsage>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::FileTag>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::FileTag>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_segno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FileTag>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg17")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::FormData_pg_attribute>() == 104);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::FormData_pg_attribute>() == 4);
};
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_attribute, attcollation) == 100
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attcollation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_attribute>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 100;
}
#[cfg(feature = "pg17")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::IndexInfo>() == 192);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::IndexInfo>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexInfo, ii_Unique) == 160);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ii_Unique>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexInfo, ii_NullsNotDistinct) == 161
);
