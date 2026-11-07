// Shared generated C macro support.

// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_candidate_restart_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlot>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 256;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReplicationSlot, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlot> as c::expression::OrdinaryField<Field_candidate_restart_lsn>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).candidate_restart_lsn) };
#[doc(hidden)]
pub struct Field_database;
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ReplicationSlotPersistentData {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReplicationSlotPersistentData, database)
        == 64
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_database>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlotPersistentData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReplicationSlotPersistentData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlotPersistentData> as c::expression::OrdinaryField<Field_database>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).database) };
#[doc(hidden)]
pub struct Field_persistency;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReplicationSlotPersistentData, persistency)
        == 68
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_persistency>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlotPersistentData>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ReplicationSlotPersistency,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 68;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReplicationSlotPersistentData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlotPersistentData> as c::expression::OrdinaryField<Field_persistency>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).persistency) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReplicationSlotPersistentData, xmin) == 72
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xmin>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlotPersistentData>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReplicationSlotPersistentData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlotPersistentData> as c::expression::OrdinaryField<Field_xmin>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xmin) };
#[doc(hidden)]
pub struct Field_catalog_xmin;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReplicationSlotPersistentData, catalog_xmin)
        == 76
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_catalog_xmin>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlotPersistentData>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 76;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReplicationSlotPersistentData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlotPersistentData> as c::expression::OrdinaryField<Field_catalog_xmin>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).catalog_xmin) };
#[doc(hidden)]
pub struct Field_restart_lsn;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReplicationSlotPersistentData, restart_lsn)
        == 80
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_restart_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlotPersistentData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReplicationSlotPersistentData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlotPersistentData> as c::expression::OrdinaryField<Field_restart_lsn>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).restart_lsn) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_invalidated_at;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ReplicationSlotPersistentData,
        invalidated_at
    ) == 88
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_invalidated_at>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlotPersistentData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReplicationSlotPersistentData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlotPersistentData> as c::expression::OrdinaryField<Field_invalidated_at>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).invalidated_at) };
#[doc(hidden)]
pub struct Field_confirmed_flush;
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ReplicationSlotPersistentData,
        confirmed_flush
    ) == 96
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_confirmed_flush>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlotPersistentData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReplicationSlotPersistentData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlotPersistentData> as c::expression::OrdinaryField<Field_confirmed_flush>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).confirmed_flush) };
#[doc(hidden)]
pub struct Field_two_phase_at;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReplicationSlotPersistentData, two_phase_at)
        == 104
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_two_phase_at>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlotPersistentData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReplicationSlotPersistentData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlotPersistentData> as c::expression::OrdinaryField<Field_two_phase_at>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).two_phase_at) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReplicationSlotPersistentData, two_phase)
        == 112
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_two_phase>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlotPersistentData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReplicationSlotPersistentData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlotPersistentData> as c::expression::OrdinaryField<Field_two_phase>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).two_phase) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ResTarget>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ResTarget>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ResTarget {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ResTarget, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResTarget>
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
    crate::__pgrx_c_bindings::ResTarget,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ResTarget> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ResTarget, indirection) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indirection>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResTarget>
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
    crate::__pgrx_c_bindings::ResTarget,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ResTarget> as c::expression::OrdinaryField<
        Field_indirection,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indirection) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ResTarget, location) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResTarget>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ResTarget,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ResTarget> as c::expression::OrdinaryField<
        Field_location,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RestrictInfo>() == 224);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RestrictInfo>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::RestrictInfo {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RestrictInfo, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo>
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
const _: Projection<crate::__pgrx_c_bindings::RestrictInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_is_pushed_down;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RestrictInfo, is_pushed_down) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_pushed_down>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RestrictInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo> as c::expression::OrdinaryField<Field_is_pushed_down>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_pushed_down) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_outerjoin_delayed;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RestrictInfo, outerjoin_delayed) == 17
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_outerjoin_delayed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 17;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RestrictInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo> as c::expression::OrdinaryField<Field_outerjoin_delayed>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).outerjoin_delayed) };
#[doc(hidden)]
pub struct Field_can_join;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RestrictInfo, can_join) == 18);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_can_join>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 18;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RestrictInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo> as c::expression::OrdinaryField<Field_can_join>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).can_join) };
#[doc(hidden)]
pub struct Field_pseudoconstant;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RestrictInfo, pseudoconstant) == 19);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pseudoconstant>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 19;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RestrictInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo> as c::expression::OrdinaryField<Field_pseudoconstant>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pseudoconstant) };
#[doc(hidden)]
pub struct Field_leakproof;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RestrictInfo, leakproof) == 20);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_leakproof>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RestrictInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo> as c::expression::OrdinaryField<Field_leakproof>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).leakproof) };
#[doc(hidden)]
pub struct Field_has_volatile;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RestrictInfo, has_volatile) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_has_volatile>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__VolatileFunctionStatus,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RestrictInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo> as c::expression::OrdinaryField<Field_has_volatile>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).has_volatile) };
#[doc(hidden)]
pub struct Field_security_level;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RestrictInfo, security_level) == 28);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_security_level>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RestrictInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo> as c::expression::OrdinaryField<Field_security_level>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).security_level) };
#[doc(hidden)]
pub struct Field_required_relids;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RestrictInfo, required_relids) == 40);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_required_relids>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::Bitmapset>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RestrictInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo> as c::expression::OrdinaryField<Field_required_relids>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).required_relids) };
#[doc(hidden)]
pub struct Field_mergeopfamilies;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RestrictInfo, mergeopfamilies) == 128
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mergeopfamilies>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RestrictInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo> as c::expression::OrdinaryField<Field_mergeopfamilies>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mergeopfamilies) };
#[doc(hidden)]
pub struct Field_scansel_cache;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RestrictInfo, scansel_cache) == 168);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_scansel_cache>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 168;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RestrictInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo> as c::expression::OrdinaryField<Field_scansel_cache>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).scansel_cache) };
#[doc(hidden)]
pub struct Field_outer_is_left;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RestrictInfo, outer_is_left) == 176);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_outer_is_left>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 176;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RestrictInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo> as c::expression::OrdinaryField<Field_outer_is_left>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).outer_is_left) };
#[doc(hidden)]
pub struct Field_hashjoinoperator;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RestrictInfo, hashjoinoperator) == 180
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hashjoinoperator>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 180;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RestrictInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo> as c::expression::OrdinaryField<Field_hashjoinoperator>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hashjoinoperator) };
#[doc(hidden)]
pub struct Field_left_hasheqoperator;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RestrictInfo, left_hasheqoperator) == 216
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_left_hasheqoperator>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 216;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RestrictInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo> as c::expression::OrdinaryField<Field_left_hasheqoperator>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).left_hasheqoperator) };
#[doc(hidden)]
pub struct Field_right_hasheqoperator;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RestrictInfo, right_hasheqoperator) == 220
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_right_hasheqoperator>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 220;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RestrictInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo> as c::expression::OrdinaryField<Field_right_hasheqoperator>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).right_hasheqoperator) };
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ResultRelInfo>() == 336);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ResultRelInfo>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ResultRelInfo {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
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
const _: Projection<crate::__pgrx_c_bindings::ResultRelInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_ri_RangeTableIndex;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_RangeTableIndex) == 4
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_RangeTableIndex>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ResultRelInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo> as c::expression::OrdinaryField<Field_ri_RangeTableIndex>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ri_RangeTableIndex) };
#[doc(hidden)]
pub struct Field_ri_NumIndices;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_NumIndices) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_NumIndices>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ResultRelInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo> as c::expression::OrdinaryField<Field_ri_NumIndices>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ri_NumIndices) };
#[doc(hidden)]
pub struct Field_ri_RowIdAttNo;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_RowIdAttNo) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_RowIdAttNo>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ResultRelInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo> as c::expression::OrdinaryField<Field_ri_RowIdAttNo>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ri_RowIdAttNo) };
#[doc(hidden)]
pub struct Field_ri_projectNewInfoValid;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_projectNewInfoValid) == 72
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_projectNewInfoValid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ResultRelInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo> as c::expression::OrdinaryField<Field_ri_projectNewInfoValid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ri_projectNewInfoValid) };
#[doc(hidden)]
pub struct Field_ri_needLockTagTuple;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_needLockTagTuple) == 73
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_needLockTagTuple>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 73;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ResultRelInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo> as c::expression::OrdinaryField<Field_ri_needLockTagTuple>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ri_needLockTagTuple) };
#[doc(hidden)]
pub struct Field_ri_usesFdwDirectModify;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_usesFdwDirectModify) == 152
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_usesFdwDirectModify>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ResultRelInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo> as c::expression::OrdinaryField<Field_ri_usesFdwDirectModify>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ri_usesFdwDirectModify) };
#[doc(hidden)]
pub struct Field_ri_NumSlots;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_NumSlots) == 156);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_NumSlots>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 156;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ResultRelInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo> as c::expression::OrdinaryField<Field_ri_NumSlots>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ri_NumSlots) };
#[doc(hidden)]
pub struct Field_ri_NumSlotsInitialized;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_NumSlotsInitialized) == 160
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_NumSlotsInitialized>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ResultRelInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo> as c::expression::OrdinaryField<Field_ri_NumSlotsInitialized>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ri_NumSlotsInitialized) };
#[doc(hidden)]
pub struct Field_ri_BatchSize;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_BatchSize) == 164);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_BatchSize>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 164;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ResultRelInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo> as c::expression::OrdinaryField<Field_ri_BatchSize>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ri_BatchSize) };
#[doc(hidden)]
pub struct Field_ri_WithCheckOptions;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_WithCheckOptions) == 184
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_WithCheckOptions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 184;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ResultRelInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo> as c::expression::OrdinaryField<Field_ri_WithCheckOptions>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ri_WithCheckOptions) };
#[doc(hidden)]
pub struct Field_ri_WithCheckOptionExprs;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_WithCheckOptionExprs)
        == 192
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_WithCheckOptionExprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 192;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ResultRelInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo> as c::expression::OrdinaryField<Field_ri_WithCheckOptionExprs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ri_WithCheckOptionExprs) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_ri_NumGeneratedNeeded;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_NumGeneratedNeeded) == 216
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_NumGeneratedNeeded>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 216;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ResultRelInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo> as c::expression::OrdinaryField<Field_ri_NumGeneratedNeeded>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ri_NumGeneratedNeeded) };
#[doc(hidden)]
pub struct Field_ri_returningList;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_returningList) == 224
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_returningList>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 224;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ResultRelInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo> as c::expression::OrdinaryField<Field_ri_returningList>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ri_returningList) };
#[doc(hidden)]
pub struct Field_ri_onConflictArbiterIndexes;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_onConflictArbiterIndexes)
        == 240
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_onConflictArbiterIndexes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 240;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ResultRelInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo> as c::expression::OrdinaryField<Field_ri_onConflictArbiterIndexes>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ri_onConflictArbiterIndexes) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
#[doc(hidden)]
pub struct Field_ri_matchedMergeAction;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_matchedMergeAction) == 256
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_matchedMergeAction>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 256;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ResultRelInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo> as c::expression::OrdinaryField<Field_ri_matchedMergeAction>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ri_matchedMergeAction) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
#[doc(hidden)]
pub struct Field_ri_notMatchedMergeAction;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_notMatchedMergeAction)
        == 264
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_notMatchedMergeAction>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 264;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ResultRelInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo> as c::expression::OrdinaryField<Field_ri_notMatchedMergeAction>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ri_notMatchedMergeAction) };
#[doc(hidden)]
pub struct Field_ri_ChildToRootMapValid;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_ChildToRootMapValid) == 312
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_ChildToRootMapValid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 312;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ResultRelInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo> as c::expression::OrdinaryField<Field_ri_ChildToRootMapValid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ri_ChildToRootMapValid) };
#[doc(hidden)]
pub struct Field_ri_ancestorResultRels;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_ancestorResultRels) == 328
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_ancestorResultRels>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 328;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ResultRelInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo> as c::expression::OrdinaryField<Field_ri_ancestorResultRels>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ri_ancestorResultRels) };
#[doc(hidden)]
pub struct Field_rs_done;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ResultState>() == 216);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ResultState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ResultState {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultState, rs_done) == 208);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rs_done>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 208;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ResultState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ResultState> as c::expression::OrdinaryField<Field_rs_done>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rs_done) };
#[doc(hidden)]
pub struct Field_rs_checkqual;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultState, rs_checkqual) == 209);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rs_checkqual>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 209;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ResultState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ResultState> as c::expression::OrdinaryField<Field_rs_checkqual>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rs_checkqual) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ReturnSetInfo>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ReturnSetInfo>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ReturnSetInfo {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReturnSetInfo, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReturnSetInfo>
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
const _: Projection<crate::__pgrx_c_bindings::ReturnSetInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReturnSetInfo> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_allowedModes;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReturnSetInfo, allowedModes) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_allowedModes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReturnSetInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReturnSetInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReturnSetInfo> as c::expression::OrdinaryField<Field_allowedModes>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).allowedModes) };
#[doc(hidden)]
pub struct Field_returnMode;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReturnSetInfo, returnMode) == 28);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_returnMode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReturnSetInfo>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_SetFunctionReturnMode,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReturnSetInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReturnSetInfo> as c::expression::OrdinaryField<Field_returnMode>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).returnMode) };
#[doc(hidden)]
pub struct Field_isDone;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReturnSetInfo, isDone) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isDone>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReturnSetInfo>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_ExprDoneCond,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReturnSetInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReturnSetInfo> as c::expression::OrdinaryField<Field_isDone>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isDone) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ReturnStmt>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ReturnStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ReturnStmt {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReturnStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReturnStmt>
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
const _: Projection<crate::__pgrx_c_bindings::ReturnStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReturnStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_ruleId;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RewriteRule>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RewriteRule>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::RewriteRule {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RewriteRule, ruleId) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ruleId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RewriteRule>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RewriteRule, <<c::expression::CRecord<crate::__pgrx_c_bindings::RewriteRule> as c::expression::OrdinaryField<Field_ruleId>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ruleId) };
#[doc(hidden)]
pub struct Field_event;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RewriteRule, event) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_event>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RewriteRule>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__CmdType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RewriteRule, <<c::expression::CRecord<crate::__pgrx_c_bindings::RewriteRule> as c::expression::OrdinaryField<Field_event>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).event) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RewriteRule, actions) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_actions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RewriteRule>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RewriteRule, <<c::expression::CRecord<crate::__pgrx_c_bindings::RewriteRule> as c::expression::OrdinaryField<Field_actions>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).actions) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RewriteRule, enabled) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_enabled>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RewriteRule>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RewriteRule, <<c::expression::CRecord<crate::__pgrx_c_bindings::RewriteRule> as c::expression::OrdinaryField<Field_enabled>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).enabled) };
#[doc(hidden)]
pub struct Field_isInstead;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RewriteRule, isInstead) == 25);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isInstead>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RewriteRule>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 25;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RewriteRule, <<c::expression::CRecord<crate::__pgrx_c_bindings::RewriteRule> as c::expression::OrdinaryField<Field_isInstead>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isInstead) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RollupData>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RollupData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::RollupData {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RollupData, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RollupData>
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
const _: Projection<crate::__pgrx_c_bindings::RollupData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RollupData> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RollupData, groupClause) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_groupClause>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RollupData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RollupData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RollupData> as c::expression::OrdinaryField<Field_groupClause>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).groupClause) };
#[doc(hidden)]
pub struct Field_gsets;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RollupData, gsets) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_gsets>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RollupData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RollupData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RollupData> as c::expression::OrdinaryField<Field_gsets>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).gsets) };
#[doc(hidden)]
pub struct Field_gsets_data;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RollupData, gsets_data) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_gsets_data>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RollupData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RollupData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RollupData> as c::expression::OrdinaryField<Field_gsets_data>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).gsets_data) };
#[doc(hidden)]
pub struct Field_hashable;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RollupData, hashable) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hashable>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RollupData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RollupData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RollupData> as c::expression::OrdinaryField<Field_hashable>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hashable) };
#[doc(hidden)]
pub struct Field_is_hashed;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RollupData, is_hashed) == 41);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_hashed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RollupData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 41;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RollupData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RollupData> as c::expression::OrdinaryField<Field_is_hashed>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_hashed) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RowCompareExpr>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RowCompareExpr>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::RowCompareExpr {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RowCompareExpr, rctype) == 4);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rctype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RowCompareExpr>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__RowCompareType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RowCompareExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::RowCompareExpr> as c::expression::OrdinaryField<Field_rctype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rctype) };
#[doc(hidden)]
pub struct Field_opnos;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RowCompareExpr, opnos) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_opnos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RowCompareExpr>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RowCompareExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::RowCompareExpr> as c::expression::OrdinaryField<Field_opnos>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).opnos) };
#[doc(hidden)]
pub struct Field_opfamilies;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RowCompareExpr, opfamilies) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_opfamilies>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RowCompareExpr>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RowCompareExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::RowCompareExpr> as c::expression::OrdinaryField<Field_opfamilies>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).opfamilies) };
#[doc(hidden)]
pub struct Field_inputcollids;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RowCompareExpr, inputcollids) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inputcollids>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RowCompareExpr>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RowCompareExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::RowCompareExpr> as c::expression::OrdinaryField<Field_inputcollids>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).inputcollids) };
#[doc(hidden)]
pub struct Field_largs;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RowCompareExpr, largs) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_largs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RowCompareExpr>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RowCompareExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::RowCompareExpr> as c::expression::OrdinaryField<Field_largs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).largs) };
#[doc(hidden)]
pub struct Field_rargs;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RowCompareExpr, rargs) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rargs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RowCompareExpr>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RowCompareExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::RowCompareExpr> as c::expression::OrdinaryField<Field_rargs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rargs) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RowExpr>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RowExpr>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::RowExpr {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RowExpr, args) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_args>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RowExpr>
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
    crate::__pgrx_c_bindings::RowExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::RowExpr> as c::expression::OrdinaryField<
        Field_args,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).args) };
#[doc(hidden)]
pub struct Field_row_typeid;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RowExpr, row_typeid) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_row_typeid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RowExpr>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::RowExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::RowExpr> as c::expression::OrdinaryField<
        Field_row_typeid,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).row_typeid) };
#[doc(hidden)]
pub struct Field_row_format;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RowExpr, row_format) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_row_format>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RowExpr>
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
const _: Projection<
    crate::__pgrx_c_bindings::RowExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::RowExpr> as c::expression::OrdinaryField<
        Field_row_format,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).row_format) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RowExpr, colnames) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_colnames>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RowExpr>
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
    crate::__pgrx_c_bindings::RowExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::RowExpr> as c::expression::OrdinaryField<
        Field_colnames,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).colnames) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RowExpr, location) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RowExpr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::RowExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::RowExpr> as c::expression::OrdinaryField<
        Field_location,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RowIdentityVarInfo>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RowIdentityVarInfo>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::RowIdentityVarInfo {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RowIdentityVarInfo, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RowIdentityVarInfo>
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
const _: Projection<crate::__pgrx_c_bindings::RowIdentityVarInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RowIdentityVarInfo> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_rowidwidth;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RowIdentityVarInfo, rowidwidth) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rowidwidth>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RowIdentityVarInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RowIdentityVarInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RowIdentityVarInfo> as c::expression::OrdinaryField<Field_rowidwidth>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rowidwidth) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RowMarkClause>() == 20);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RowMarkClause>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::RowMarkClause {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RowMarkClause, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RowMarkClause>
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
const _: Projection<crate::__pgrx_c_bindings::RowMarkClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::RowMarkClause> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RowMarkClause, rti) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rti>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RowMarkClause>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RowMarkClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::RowMarkClause> as c::expression::OrdinaryField<Field_rti>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rti) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RowMarkClause, strength) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_strength>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RowMarkClause>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__LockClauseStrength,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RowMarkClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::RowMarkClause> as c::expression::OrdinaryField<Field_strength>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).strength) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RowMarkClause, waitPolicy) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_waitPolicy>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RowMarkClause>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__LockWaitPolicy,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RowMarkClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::RowMarkClause> as c::expression::OrdinaryField<Field_waitPolicy>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).waitPolicy) };
#[doc(hidden)]
pub struct Field_pushedDown;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RowMarkClause, pushedDown) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pushedDown>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RowMarkClause>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RowMarkClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::RowMarkClause> as c::expression::OrdinaryField<Field_pushedDown>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pushedDown) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RowSecurityPolicy>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RowSecurityPolicy>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::RowSecurityPolicy {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RowSecurityPolicy, polcmd) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_polcmd>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RowSecurityPolicy>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RowSecurityPolicy, <<c::expression::CRecord<crate::__pgrx_c_bindings::RowSecurityPolicy> as c::expression::OrdinaryField<Field_polcmd>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).polcmd) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RowSecurityPolicy, permissive) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_permissive>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RowSecurityPolicy>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RowSecurityPolicy, <<c::expression::CRecord<crate::__pgrx_c_bindings::RowSecurityPolicy> as c::expression::OrdinaryField<Field_permissive>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).permissive) };
#[doc(hidden)]
pub struct Field_hassublinks;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RowSecurityPolicy, hassublinks) == 48
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hassublinks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RowSecurityPolicy>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RowSecurityPolicy, <<c::expression::CRecord<crate::__pgrx_c_bindings::RowSecurityPolicy> as c::expression::OrdinaryField<Field_hassublinks>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hassublinks) };
#[doc(hidden)]
pub struct Field_numLocks;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RuleLock>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RuleLock>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::RuleLock {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RuleLock, numLocks) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numLocks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RuleLock>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::RuleLock,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::RuleLock> as c::expression::OrdinaryField<
        Field_numLocks,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numLocks) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RuleStmt>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RuleStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::RuleStmt {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RuleStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RuleStmt>
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
    crate::__pgrx_c_bindings::RuleStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::RuleStmt> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RuleStmt, event) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_event>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RuleStmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__CmdType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::RuleStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::RuleStmt> as c::expression::OrdinaryField<
        Field_event,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).event) };
#[doc(hidden)]
pub struct Field_instead;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RuleStmt, instead) == 36);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_instead>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RuleStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::RuleStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::RuleStmt> as c::expression::OrdinaryField<
        Field_instead,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).instead) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RuleStmt, actions) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_actions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RuleStmt>
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
    crate::__pgrx_c_bindings::RuleStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::RuleStmt> as c::expression::OrdinaryField<
        Field_actions,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).actions) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RuleStmt, replace) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_replace>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RuleStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::RuleStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::RuleStmt> as c::expression::OrdinaryField<
        Field_replace,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).replace) };
#[doc(hidden)]
pub struct Field_xcnt;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RunningTransactionsData>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RunningTransactionsData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::RunningTransactionsData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RunningTransactionsData, xcnt) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xcnt>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RunningTransactionsData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RunningTransactionsData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RunningTransactionsData> as c::expression::OrdinaryField<Field_xcnt>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xcnt) };
#[doc(hidden)]
pub struct Field_subxcnt;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RunningTransactionsData, subxcnt) == 4
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_subxcnt>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RunningTransactionsData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RunningTransactionsData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RunningTransactionsData> as c::expression::OrdinaryField<Field_subxcnt>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).subxcnt) };
#[doc(hidden)]
pub struct Field_subxid_status;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RunningTransactionsData, subxid_status) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_subxid_status>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RunningTransactionsData>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_subxids_array_status,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RunningTransactionsData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RunningTransactionsData> as c::expression::OrdinaryField<Field_subxid_status>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).subxid_status) };
#[doc(hidden)]
pub struct Field_nextXid;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RunningTransactionsData, nextXid) == 12
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nextXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RunningTransactionsData>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RunningTransactionsData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RunningTransactionsData> as c::expression::OrdinaryField<Field_nextXid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nextXid) };
#[doc(hidden)]
pub struct Field_oldestRunningXid;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RunningTransactionsData, oldestRunningXid)
        == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldestRunningXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RunningTransactionsData>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RunningTransactionsData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RunningTransactionsData> as c::expression::OrdinaryField<Field_oldestRunningXid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oldestRunningXid) };
#[doc(hidden)]
pub struct Field_latestCompletedXid;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RunningTransactionsData, latestCompletedXid)
        == 20
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_latestCompletedXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RunningTransactionsData>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RunningTransactionsData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RunningTransactionsData> as c::expression::OrdinaryField<Field_latestCompletedXid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).latestCompletedXid) };
#[doc(hidden)]
pub struct Field_prepareSeqNo;
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SERIALIZABLEXACT>() == 152);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SERIALIZABLEXACT>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SERIALIZABLEXACT {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SERIALIZABLEXACT, prepareSeqNo) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_prepareSeqNo>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SERIALIZABLEXACT>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SERIALIZABLEXACT, <<c::expression::CRecord<crate::__pgrx_c_bindings::SERIALIZABLEXACT> as c::expression::OrdinaryField<Field_prepareSeqNo>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).prepareSeqNo) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SERIALIZABLEXACT, commitSeqNo) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_commitSeqNo>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SERIALIZABLEXACT>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SERIALIZABLEXACT, <<c::expression::CRecord<crate::__pgrx_c_bindings::SERIALIZABLEXACT> as c::expression::OrdinaryField<Field_commitSeqNo>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).commitSeqNo) };
#[doc(hidden)]
pub struct Field_topXid;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SERIALIZABLEXACT, topXid) == 128);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_topXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SERIALIZABLEXACT>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SERIALIZABLEXACT, <<c::expression::CRecord<crate::__pgrx_c_bindings::SERIALIZABLEXACT> as c::expression::OrdinaryField<Field_topXid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).topXid) };
#[doc(hidden)]
pub struct Field_finishedBefore;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SERIALIZABLEXACT, finishedBefore) == 132
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_finishedBefore>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SERIALIZABLEXACT>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 132;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SERIALIZABLEXACT, <<c::expression::CRecord<crate::__pgrx_c_bindings::SERIALIZABLEXACT> as c::expression::OrdinaryField<Field_finishedBefore>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).finishedBefore) };
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SERIALIZABLEXACT, xmin) == 136);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xmin>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SERIALIZABLEXACT>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SERIALIZABLEXACT, <<c::expression::CRecord<crate::__pgrx_c_bindings::SERIALIZABLEXACT> as c::expression::OrdinaryField<Field_xmin>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xmin) };
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SERIALIZABLEXACT, flags) == 140);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SERIALIZABLEXACT>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 140;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SERIALIZABLEXACT, <<c::expression::CRecord<crate::__pgrx_c_bindings::SERIALIZABLEXACT> as c::expression::OrdinaryField<Field_flags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).flags) };
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SERIALIZABLEXACT, pid) == 144);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SERIALIZABLEXACT>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SERIALIZABLEXACT, <<c::expression::CRecord<crate::__pgrx_c_bindings::SERIALIZABLEXACT> as c::expression::OrdinaryField<Field_pid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pid) };
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SERIALIZABLEXACT, pgprocno) == 148);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pgprocno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SERIALIZABLEXACT>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 148;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SERIALIZABLEXACT, <<c::expression::CRecord<crate::__pgrx_c_bindings::SERIALIZABLEXACT> as c::expression::OrdinaryField<Field_pgprocno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pgprocno) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SERIALIZABLEXIDTAG>() == 4);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SERIALIZABLEXIDTAG>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SERIALIZABLEXIDTAG {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SERIALIZABLEXIDTAG, xid) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SERIALIZABLEXIDTAG>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SERIALIZABLEXIDTAG, <<c::expression::CRecord<crate::__pgrx_c_bindings::SERIALIZABLEXIDTAG> as c::expression::OrdinaryField<Field_xid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xid) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_smgr_rnode;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SMgrRelationData>() == 112);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SMgrRelationData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SMgrRelationData {}
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SMgrRelationData, smgr_rnode) == 0);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_smgr_rnode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SMgrRelationData>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::RelFileNodeBackend>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SMgrRelationData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SMgrRelationData> as c::expression::OrdinaryField<Field_smgr_rnode>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).smgr_rnode) };
#[doc(hidden)]
pub struct Field_smgr_targblock;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SMgrRelationData, smgr_targblock) == 24
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_smgr_targblock>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SMgrRelationData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SMgrRelationData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SMgrRelationData> as c::expression::OrdinaryField<Field_smgr_targblock>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).smgr_targblock) };
#[doc(hidden)]
pub struct Field_smgr_which;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SMgrRelationData, smgr_which) == 44);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_smgr_which>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SMgrRelationData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SMgrRelationData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SMgrRelationData> as c::expression::OrdinaryField<Field_smgr_which>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).smgr_which) };
#[doc(hidden)]
pub struct Field_numvals;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SPITupleTable>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SPITupleTable>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SPITupleTable {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SPITupleTable, numvals) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numvals>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SPITupleTable>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SPITupleTable, <<c::expression::CRecord<crate::__pgrx_c_bindings::SPITupleTable> as c::expression::OrdinaryField<Field_numvals>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numvals) };
#[doc(hidden)]
pub struct Field_alloced;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SPITupleTable, alloced) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_alloced>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SPITupleTable>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SPITupleTable, <<c::expression::CRecord<crate::__pgrx_c_bindings::SPITupleTable> as c::expression::OrdinaryField<Field_alloced>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).alloced) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SPITupleTable, subid) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_subid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SPITupleTable>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SPITupleTable, <<c::expression::CRecord<crate::__pgrx_c_bindings::SPITupleTable> as c::expression::OrdinaryField<Field_subid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).subid) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SQLFunctionParseInfo>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SQLFunctionParseInfo>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SQLFunctionParseInfo {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SQLFunctionParseInfo, nargs) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nargs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SQLFunctionParseInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SQLFunctionParseInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::SQLFunctionParseInfo> as c::expression::OrdinaryField<Field_nargs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nargs) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SQLFunctionParseInfo, collation) == 32
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_collation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SQLFunctionParseInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SQLFunctionParseInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::SQLFunctionParseInfo> as c::expression::OrdinaryField<Field_collation>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).collation) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SQLValueFunction>() == 20);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SQLValueFunction>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SQLValueFunction {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SQLValueFunction, op) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_op>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SQLValueFunction>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__SQLValueFunctionOp,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SQLValueFunction, <<c::expression::CRecord<crate::__pgrx_c_bindings::SQLValueFunction> as c::expression::OrdinaryField<Field_op>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).op) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SQLValueFunction, type_) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SQLValueFunction>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SQLValueFunction, <<c::expression::CRecord<crate::__pgrx_c_bindings::SQLValueFunction> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SQLValueFunction, typmod) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typmod>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SQLValueFunction>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SQLValueFunction, <<c::expression::CRecord<crate::__pgrx_c_bindings::SQLValueFunction> as c::expression::OrdinaryField<Field_typmod>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typmod) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SQLValueFunction, location) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SQLValueFunction>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SQLValueFunction, <<c::expression::CRecord<crate::__pgrx_c_bindings::SQLValueFunction> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SampleScanState>() == 280);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SampleScanState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SampleScanState {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SampleScanState, args) == 224);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_args>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SampleScanState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 224;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SampleScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::SampleScanState> as c::expression::OrdinaryField<Field_args>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).args) };
#[doc(hidden)]
pub struct Field_use_bulkread;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SampleScanState, use_bulkread) == 256
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_use_bulkread>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SampleScanState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 256;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SampleScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::SampleScanState> as c::expression::OrdinaryField<Field_use_bulkread>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).use_bulkread) };
#[doc(hidden)]
pub struct Field_use_pagemode;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SampleScanState, use_pagemode) == 257
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_use_pagemode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SampleScanState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 257;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SampleScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::SampleScanState> as c::expression::OrdinaryField<Field_use_pagemode>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).use_pagemode) };
#[doc(hidden)]
pub struct Field_begun;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SampleScanState, begun) == 258);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_begun>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SampleScanState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 258;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SampleScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::SampleScanState> as c::expression::OrdinaryField<Field_begun>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).begun) };
#[doc(hidden)]
pub struct Field_seed;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SampleScanState, seed) == 260);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_seed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SampleScanState>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 260;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SampleScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::SampleScanState> as c::expression::OrdinaryField<Field_seed>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).seed) };
#[doc(hidden)]
pub struct Field_donetuples;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SampleScanState, donetuples) == 264);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_donetuples>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SampleScanState>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 264;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SampleScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::SampleScanState> as c::expression::OrdinaryField<Field_donetuples>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).donetuples) };
#[doc(hidden)]
pub struct Field_haveblock;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SampleScanState, haveblock) == 272);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_haveblock>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SampleScanState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 272;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SampleScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::SampleScanState> as c::expression::OrdinaryField<Field_haveblock>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).haveblock) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SampleScanState, done) == 273);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_done>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SampleScanState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 273;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SampleScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::SampleScanState> as c::expression::OrdinaryField<Field_done>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).done) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ScalarArrayOpExpr>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ScalarArrayOpExpr>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ScalarArrayOpExpr {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ScalarArrayOpExpr, opno) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_opno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ScalarArrayOpExpr>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ScalarArrayOpExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::ScalarArrayOpExpr> as c::expression::OrdinaryField<Field_opno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).opno) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ScalarArrayOpExpr, opfuncid) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_opfuncid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ScalarArrayOpExpr>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ScalarArrayOpExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::ScalarArrayOpExpr> as c::expression::OrdinaryField<Field_opfuncid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).opfuncid) };
#[doc(hidden)]
pub struct Field_hashfuncid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ScalarArrayOpExpr, hashfuncid) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hashfuncid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ScalarArrayOpExpr>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ScalarArrayOpExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::ScalarArrayOpExpr> as c::expression::OrdinaryField<Field_hashfuncid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hashfuncid) };
#[doc(hidden)]
pub struct Field_negfuncid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ScalarArrayOpExpr, negfuncid) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_negfuncid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ScalarArrayOpExpr>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ScalarArrayOpExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::ScalarArrayOpExpr> as c::expression::OrdinaryField<Field_negfuncid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).negfuncid) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ScalarArrayOpExpr, useOr) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_useOr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ScalarArrayOpExpr>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ScalarArrayOpExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::ScalarArrayOpExpr> as c::expression::OrdinaryField<Field_useOr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).useOr) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ScalarArrayOpExpr, inputcollid) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inputcollid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ScalarArrayOpExpr>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ScalarArrayOpExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::ScalarArrayOpExpr> as c::expression::OrdinaryField<Field_inputcollid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).inputcollid) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ScalarArrayOpExpr, args) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_args>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ScalarArrayOpExpr>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ScalarArrayOpExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::ScalarArrayOpExpr> as c::expression::OrdinaryField<Field_args>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).args) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ScalarArrayOpExpr, location) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ScalarArrayOpExpr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ScalarArrayOpExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::ScalarArrayOpExpr> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[doc(hidden)]
pub struct Field_scanrelid;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Scan>() == 112);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Scan>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::Scan {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Scan, scanrelid) == 104);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_scanrelid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Scan>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Scan,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Scan> as c::expression::OrdinaryField<
        Field_scanrelid,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).scanrelid) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ScannerCallbackState>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ScannerCallbackState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ScannerCallbackState {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ScannerCallbackState, location) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ScannerCallbackState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ScannerCallbackState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ScannerCallbackState> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SecLabelStmt>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SecLabelStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SecLabelStmt {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SecLabelStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SecLabelStmt>
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
const _: Projection<crate::__pgrx_c_bindings::SecLabelStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::SecLabelStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SecLabelStmt, objtype) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_objtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SecLabelStmt>
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
const _: Projection<crate::__pgrx_c_bindings::SecLabelStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::SecLabelStmt> as c::expression::OrdinaryField<Field_objtype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).objtype) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SelectStmt>() == 160);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SelectStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SelectStmt {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SelectStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SelectStmt>
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
const _: Projection<crate::__pgrx_c_bindings::SelectStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::SelectStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SelectStmt, distinctClause) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_distinctClause>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SelectStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SelectStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::SelectStmt> as c::expression::OrdinaryField<Field_distinctClause>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).distinctClause) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SelectStmt, targetList) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_targetList>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SelectStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SelectStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::SelectStmt> as c::expression::OrdinaryField<Field_targetList>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).targetList) };
#[doc(hidden)]
pub struct Field_fromClause;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SelectStmt, fromClause) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fromClause>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SelectStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SelectStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::SelectStmt> as c::expression::OrdinaryField<Field_fromClause>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fromClause) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SelectStmt, groupClause) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_groupClause>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SelectStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SelectStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::SelectStmt> as c::expression::OrdinaryField<Field_groupClause>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).groupClause) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SelectStmt, groupDistinct) == 56);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_groupDistinct>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SelectStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SelectStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::SelectStmt> as c::expression::OrdinaryField<Field_groupDistinct>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).groupDistinct) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SelectStmt, windowClause) == 72);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_windowClause>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SelectStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SelectStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::SelectStmt> as c::expression::OrdinaryField<Field_windowClause>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).windowClause) };
#[doc(hidden)]
pub struct Field_valuesLists;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SelectStmt, valuesLists) == 80);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_valuesLists>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SelectStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SelectStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::SelectStmt> as c::expression::OrdinaryField<Field_valuesLists>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).valuesLists) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SelectStmt, sortClause) == 88);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sortClause>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SelectStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SelectStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::SelectStmt> as c::expression::OrdinaryField<Field_sortClause>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sortClause) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SelectStmt, limitOption) == 112);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_limitOption>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SelectStmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__LimitOption,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SelectStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::SelectStmt> as c::expression::OrdinaryField<Field_limitOption>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).limitOption) };
#[doc(hidden)]
pub struct Field_lockingClause;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SelectStmt, lockingClause) == 120);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lockingClause>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SelectStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SelectStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::SelectStmt> as c::expression::OrdinaryField<Field_lockingClause>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lockingClause) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SelectStmt, op) == 136);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_op>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SelectStmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__SetOperation,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SelectStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::SelectStmt> as c::expression::OrdinaryField<Field_op>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).op) };
#[doc(hidden)]
pub struct Field_all;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SelectStmt, all) == 140);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_all>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SelectStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 140;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SelectStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::SelectStmt> as c::expression::OrdinaryField<Field_all>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).all) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SeqScanState>() == 232);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SeqScanState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SeqScanState {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SeqScanState, pscan_len) == 224);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pscan_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SeqScanState>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 224;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SeqScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::SeqScanState> as c::expression::OrdinaryField<Field_pscan_len>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pscan_len) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SetExprState>() == 120);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SetExprState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SetExprState {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetExprState, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetExprState>
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
const _: Projection<crate::__pgrx_c_bindings::SetExprState, <<c::expression::CRecord<crate::__pgrx_c_bindings::SetExprState> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetExprState, args) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_args>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetExprState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SetExprState, <<c::expression::CRecord<crate::__pgrx_c_bindings::SetExprState> as c::expression::OrdinaryField<Field_args>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).args) };
#[doc(hidden)]
pub struct Field_funcReturnsTuple;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SetExprState, funcReturnsTuple) == 104
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_funcReturnsTuple>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetExprState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SetExprState, <<c::expression::CRecord<crate::__pgrx_c_bindings::SetExprState> as c::expression::OrdinaryField<Field_funcReturnsTuple>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).funcReturnsTuple) };
#[doc(hidden)]
pub struct Field_funcReturnsSet;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetExprState, funcReturnsSet) == 105);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_funcReturnsSet>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetExprState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 105;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SetExprState, <<c::expression::CRecord<crate::__pgrx_c_bindings::SetExprState> as c::expression::OrdinaryField<Field_funcReturnsSet>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).funcReturnsSet) };
#[doc(hidden)]
pub struct Field_setArgsValid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetExprState, setArgsValid) == 106);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_setArgsValid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetExprState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 106;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SetExprState, <<c::expression::CRecord<crate::__pgrx_c_bindings::SetExprState> as c::expression::OrdinaryField<Field_setArgsValid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).setArgsValid) };
#[doc(hidden)]
pub struct Field_shutdown_reg;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetExprState, shutdown_reg) == 107);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_shutdown_reg>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetExprState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 107;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SetExprState, <<c::expression::CRecord<crate::__pgrx_c_bindings::SetExprState> as c::expression::OrdinaryField<Field_shutdown_reg>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).shutdown_reg) };
#[doc(hidden)]
pub struct Field_cmd;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SetOp>() == 160);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SetOp>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SetOp {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetOp, cmd) == 104);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cmd>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetOp>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__SetOpCmd,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SetOp,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SetOp> as c::expression::OrdinaryField<
        Field_cmd,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cmd) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetOp, strategy) == 108);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_strategy>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetOp>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__SetOpStrategy,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 108;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SetOp,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SetOp> as c::expression::OrdinaryField<
        Field_strategy,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).strategy) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetOp, numCols) == 112);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numCols>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetOp>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SetOp,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SetOp> as c::expression::OrdinaryField<
        Field_numCols,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numCols) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
#[doc(hidden)]
pub struct Field_flagColIdx;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetOp, flagColIdx) == 144);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_flagColIdx>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetOp>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SetOp,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SetOp> as c::expression::OrdinaryField<
        Field_flagColIdx,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).flagColIdx) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
#[doc(hidden)]
pub struct Field_firstFlag;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetOp, firstFlag) == 148);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_firstFlag>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetOp>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 148;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SetOp,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SetOp> as c::expression::OrdinaryField<
        Field_firstFlag,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).firstFlag) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetOp, numGroups) == 152);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numGroups>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetOp>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SetOp,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SetOp> as c::expression::OrdinaryField<
        Field_numGroups,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numGroups) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SetOpPath>() == 112);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SetOpPath>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SetOpPath {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetOpPath, cmd) == 80);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cmd>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetOpPath>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__SetOpCmd,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SetOpPath,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SetOpPath> as c::expression::OrdinaryField<
        Field_cmd,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cmd) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetOpPath, strategy) == 84);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_strategy>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetOpPath>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__SetOpStrategy,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 84;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SetOpPath,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SetOpPath> as c::expression::OrdinaryField<
        Field_strategy,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).strategy) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetOpPath, distinctList) == 88);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_distinctList>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetOpPath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SetOpPath,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SetOpPath> as c::expression::OrdinaryField<
        Field_distinctList,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).distinctList) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetOpPath, flagColIdx) == 96);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_flagColIdx>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetOpPath>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SetOpPath,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SetOpPath> as c::expression::OrdinaryField<
        Field_flagColIdx,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).flagColIdx) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetOpPath, firstFlag) == 100);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_firstFlag>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetOpPath>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 100;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SetOpPath,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SetOpPath> as c::expression::OrdinaryField<
        Field_firstFlag,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).firstFlag) };
#[doc(hidden)]
pub struct Field_setop_done;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SetOpState>() == 288);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SetOpState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SetOpState {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetOpState, setop_done) == 224);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_setop_done>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetOpState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 224;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SetOpState, <<c::expression::CRecord<crate::__pgrx_c_bindings::SetOpState> as c::expression::OrdinaryField<Field_setop_done>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).setop_done) };
#[doc(hidden)]
pub struct Field_numOutput;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetOpState, numOutput) == 232);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numOutput>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetOpState>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 232;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SetOpState, <<c::expression::CRecord<crate::__pgrx_c_bindings::SetOpState> as c::expression::OrdinaryField<Field_numOutput>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numOutput) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetOpState, table_filled) == 272);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_table_filled>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetOpState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 272;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SetOpState, <<c::expression::CRecord<crate::__pgrx_c_bindings::SetOpState> as c::expression::OrdinaryField<Field_table_filled>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).table_filled) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SetOperationStmt>() == 64);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SetOperationStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SetOperationStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetOperationStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetOperationStmt>
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
const _: Projection<crate::__pgrx_c_bindings::SetOperationStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::SetOperationStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetOperationStmt, op) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_op>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetOperationStmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__SetOperation,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SetOperationStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::SetOperationStmt> as c::expression::OrdinaryField<Field_op>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).op) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetOperationStmt, all) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_all>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetOperationStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SetOperationStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::SetOperationStmt> as c::expression::OrdinaryField<Field_all>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).all) };
#[doc(hidden)]
pub struct Field_colTypes;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetOperationStmt, colTypes) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_colTypes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetOperationStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SetOperationStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::SetOperationStmt> as c::expression::OrdinaryField<Field_colTypes>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).colTypes) };
#[doc(hidden)]
pub struct Field_colTypmods;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetOperationStmt, colTypmods) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_colTypmods>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetOperationStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SetOperationStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::SetOperationStmt> as c::expression::OrdinaryField<Field_colTypmods>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).colTypmods) };
#[doc(hidden)]
pub struct Field_colCollations;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SetOperationStmt, colCollations) == 48
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_colCollations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetOperationStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SetOperationStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::SetOperationStmt> as c::expression::OrdinaryField<Field_colCollations>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).colCollations) };
#[doc(hidden)]
pub struct Field_groupClauses;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SetOperationStmt, groupClauses) == 56
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_groupClauses>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetOperationStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SetOperationStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::SetOperationStmt> as c::expression::OrdinaryField<Field_groupClauses>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).groupClauses) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SetToDefault>() == 20);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SetToDefault>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SetToDefault {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetToDefault, typeId) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typeId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetToDefault>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SetToDefault, <<c::expression::CRecord<crate::__pgrx_c_bindings::SetToDefault> as c::expression::OrdinaryField<Field_typeId>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typeId) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetToDefault, typeMod) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typeMod>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetToDefault>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SetToDefault, <<c::expression::CRecord<crate::__pgrx_c_bindings::SetToDefault> as c::expression::OrdinaryField<Field_typeMod>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typeMod) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetToDefault, collation) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_collation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetToDefault>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SetToDefault, <<c::expression::CRecord<crate::__pgrx_c_bindings::SetToDefault> as c::expression::OrdinaryField<Field_collation>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).collation) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetToDefault, location) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetToDefault>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SetToDefault, <<c::expression::CRecord<crate::__pgrx_c_bindings::SetToDefault> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SharedAggInfo>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SharedAggInfo>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SharedAggInfo {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SharedAggInfo, num_workers) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_workers>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SharedAggInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SharedAggInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::SharedAggInfo> as c::expression::OrdinaryField<Field_num_workers>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_workers) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SharedFileSet>() == 52);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SharedFileSet>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SharedFileSet {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SharedFileSet, mutex) == 44);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mutex>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SharedFileSet>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SharedFileSet, <<c::expression::CRecord<crate::__pgrx_c_bindings::SharedFileSet> as c::expression::OrdinaryField<Field_mutex>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mutex) };
#[doc(hidden)]
pub struct Field_refcnt;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SharedFileSet, refcnt) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_refcnt>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SharedFileSet>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SharedFileSet, <<c::expression::CRecord<crate::__pgrx_c_bindings::SharedFileSet> as c::expression::OrdinaryField<Field_refcnt>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).refcnt) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SharedHashInfo>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SharedHashInfo>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SharedHashInfo {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SharedHashInfo, num_workers) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_workers>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SharedHashInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SharedHashInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::SharedHashInfo> as c::expression::OrdinaryField<Field_num_workers>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_workers) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SharedIncrementalSortInfo>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SharedIncrementalSortInfo>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SharedIncrementalSortInfo {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SharedIncrementalSortInfo, num_workers) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_workers>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SharedIncrementalSortInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SharedIncrementalSortInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::SharedIncrementalSortInfo> as c::expression::OrdinaryField<Field_num_workers>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_workers) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SharedJitInstrumentation>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SharedJitInstrumentation>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SharedJitInstrumentation {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SharedJitInstrumentation, num_workers) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_workers>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SharedJitInstrumentation>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SharedJitInstrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::SharedJitInstrumentation> as c::expression::OrdinaryField<Field_num_workers>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_workers) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SharedMemoizeInfo>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SharedMemoizeInfo>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SharedMemoizeInfo {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SharedMemoizeInfo, num_workers) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_workers>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SharedMemoizeInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SharedMemoizeInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::SharedMemoizeInfo> as c::expression::OrdinaryField<Field_num_workers>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_workers) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SharedSortInfo>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SharedSortInfo>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SharedSortInfo {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SharedSortInfo, num_workers) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_workers>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SharedSortInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SharedSortInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::SharedSortInfo> as c::expression::OrdinaryField<Field_num_workers>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_workers) };
#[doc(hidden)]
pub struct Field_sync_handler;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SlruCtlData>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SlruCtlData>() == 8);
};
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SlruCtlData {}
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SlruCtlData, sync_handler) == 8);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sync_handler>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SlruCtlData>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__SyncRequestHandler,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SlruCtlData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SlruCtlData> as c::expression::OrdinaryField<Field_sync_handler>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sync_handler) };
#[doc(hidden)]
pub struct Field_num_slots;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SlruSharedData>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SlruSharedData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SlruSharedData {}
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SlruSharedData, num_slots) == 8);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_slots>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SlruSharedData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SlruSharedData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SlruSharedData> as c::expression::OrdinaryField<Field_num_slots>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_slots) };
#[doc(hidden)]
pub struct Field_lsn_groups_per_page;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SlruSharedData, lsn_groups_per_page) == 72
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lsn_groups_per_page>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SlruSharedData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SlruSharedData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SlruSharedData> as c::expression::OrdinaryField<Field_lsn_groups_per_page>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lsn_groups_per_page) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
#[doc(hidden)]
pub struct Field_cur_lru_count;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SlruSharedData, cur_lru_count) == 76);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cur_lru_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SlruSharedData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 76;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SlruSharedData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SlruSharedData> as c::expression::OrdinaryField<Field_cur_lru_count>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cur_lru_count) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
#[doc(hidden)]
pub struct Field_latest_page_number;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SlruSharedData, latest_page_number) == 80
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_latest_page_number>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SlruSharedData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
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
pub struct Field_comparator;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SortSupportData, comparator) == 24);
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    crate::__pgrx_c_bindings::Datum,
                    crate::__pgrx_c_bindings::Datum,
                    *mut crate::__pgrx_c_bindings::SortSupportData,
                ) -> i32,
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    crate::__pgrx_c_bindings::Datum,
                    crate::__pgrx_c_bindings::Datum,
                    *mut crate::__pgrx_c_bindings::SortSupportData,
                ) -> i32,
            >,
        >() == 8
    );
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_comparator>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SortSupportData>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_int__0l__unsigned__long__0c__unsigned__long__0c__struct__SortSupportData__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SortSupportData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SortSupportData> as c::expression::OrdinaryField<Field_comparator>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).comparator) };
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
pub struct Field_abbrev_full_comparator;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SortSupportData, abbrev_full_comparator)
        == 56
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_abbrev_full_comparator>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SortSupportData>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_int__0l__unsigned__long__0c__unsigned__long__0c__struct__SortSupportData__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SortSupportData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SortSupportData> as c::expression::OrdinaryField<Field_abbrev_full_comparator>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).abbrev_full_comparator) };
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
