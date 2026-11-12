// Shared generated C macro support.

#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ii_IndexUnchanged>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 164;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexInfo, ii_Concurrent) == 165);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ii_Concurrent>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 165;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexInfo, ii_BrokenHotChain) == 166);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ii_BrokenHotChain>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 166;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexInfo, ii_Summarizing) == 167);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ii_Summarizing>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 167;
}
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexInfo, ii_ParallelWorkers) == 168
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ii_ParallelWorkers>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 168;
}
#[cfg(feature = "pg17")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexInfo, ii_Am) == 172);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ii_Am>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 172;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_release_phase;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ResourceOwnerDesc>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ResourceOwnerDesc>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ResourceOwnerDesc {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResourceOwnerDesc, release_phase) == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_release_phase>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResourceOwnerDesc>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_ResourceReleasePhase,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ResourceOwnerDesc, <<c::expression::CRecord<crate::__pgrx_c_bindings::ResourceOwnerDesc> as c::expression::OrdinaryField<Field_release_phase>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).release_phase) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_release_priority;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResourceOwnerDesc, release_priority) == 12
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_release_priority>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResourceOwnerDesc>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ResourceOwnerDesc, <<c::expression::CRecord<crate::__pgrx_c_bindings::ResourceOwnerDesc> as c::expression::OrdinaryField<Field_release_priority>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).release_priority) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::TableScanDescData,
                    *mut crate::__pgrx_c_bindings::ReadStream,
                ) -> bool,
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::TableScanDescData,
                    *mut crate::__pgrx_c_bindings::ReadStream,
                ) -> bool,
            >,
        >() == 8
    );
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_scan_analyze_next_block>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__struct__ReadStream__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 264;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TupleTableSlotOps>() == 104);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TupleTableSlotOps>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_is_current_xact_tuple;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleTableSlotOps, is_current_xact_tuple)
        == 48
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TupleTableSlot) -> bool,
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TupleTableSlot) -> bool,
            >,
        >() == 8
    );
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_current_xact_tuple>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlotOps>
{
    type Member = c::expression::CFunction<
        crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__TupleTableSlot__0p__0r,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TupleTableSlotOps, <<c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlotOps> as c::expression::OrdinaryField<Field_is_current_xact_tuple>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_current_xact_tuple) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleTableSlotOps, materialize) == 56
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_materialize>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlotOps>
{
    type Member = c::expression::CFunction<
        crate::__pgrx_c_generated::Signature_void__0l__struct__TupleTableSlot__0p__0r,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleTableSlotOps, copyslot) == 64);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_copyslot>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlotOps>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_void__0l__struct__TupleTableSlot__0p__0c__struct__TupleTableSlot__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleTableSlotOps, copy_heap_tuple) == 88
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_copy_heap_tuple>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlotOps>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_struct__HeapTupleData__0p__0l__struct__TupleTableSlot__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleTableSlotOps, copy_minimal_tuple) == 96
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_copy_minimal_tuple>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlotOps>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_struct__MinimalTupleData__0p__0l__struct__TupleTableSlot__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(feature = "pg17")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::VacuumParams>() == 44);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::VacuumParams>() == 4);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_toast_parent;
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VacuumParams, toast_parent) == 36);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_toast_parent>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacuumParams>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VacuumParams, <<c::expression::CRecord<crate::__pgrx_c_bindings::VacuumParams> as c::expression::OrdinaryField<Field_toast_parent>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).toast_parent) };
#[cfg(feature = "pg17")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VacuumParams, nworkers) == 40);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nworkers>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VacuumParams>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::dshash_parameters>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::dshash_parameters>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::dshash_parameters, tranche_id) == 40);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tranche_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::dshash_parameters>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_iov_base;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::iovec, iov_base) == 0);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_iov_base>
    for c::expression::CRecord<crate::__pgrx_c_bindings::iovec>
{
    type Member = c::expression::CPointer<c::expression::CVoid, c::expression::ReadWrite>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::iovec,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::iovec> as c::expression::OrdinaryField<
        Field_iov_base,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).iov_base) };
#[cfg(feature = "pg17")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AggState>() == 592);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AggState>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AlterReplicationSlotCmd>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AlterReplicationSlotCmd>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AlterReplicationSlotCmd {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterReplicationSlotCmd, type_) == 0);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterReplicationSlotCmd>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterReplicationSlotCmd, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterReplicationSlotCmd> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterReplicationSlotCmd, options) == 16
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterReplicationSlotCmd>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AlterReplicationSlotCmd, <<c::expression::CRecord<crate::__pgrx_c_bindings::AlterReplicationSlotCmd> as c::expression::OrdinaryField<Field_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AlterStatsStmt>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AlterStatsStmt>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AlterStatsStmt, missing_ok) == 24);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_missing_ok>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AlterStatsStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg17")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTArrayKeyInfo, num_elems) == 8);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_elems>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTArrayKeyInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_dir;
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BTReadPageState>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BTReadPageState>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BTReadPageState {}
#[cfg(feature = "pg17")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTReadPageState, dir) == 0);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dir>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTReadPageState>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ScanDirection,
        c::CInt,
        i32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg17")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTReadPageState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTReadPageState> as c::expression::OrdinaryField<Field_dir>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dir) };
#[cfg(any(feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
pub struct Field_minoff;
#[cfg(feature = "pg17")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTReadPageState, minoff) == 4);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_minoff>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTReadPageState>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTReadPageState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTReadPageState> as c::expression::OrdinaryField<Field_minoff>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).minoff) };
#[cfg(feature = "pg17")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTReadPageState, maxoff) == 6);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_maxoff>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTReadPageState>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 6;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTReadPageState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTReadPageState> as c::expression::OrdinaryField<Field_maxoff>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).maxoff) };
#[cfg(feature = "pg17")]
#[doc(hidden)]
pub struct Field_prev_scan_page;
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BTReadPageState, prev_scan_page) == 16
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_prev_scan_page>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTReadPageState>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg17")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTReadPageState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTReadPageState> as c::expression::OrdinaryField<Field_prev_scan_page>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).prev_scan_page) };
#[cfg(feature = "pg17")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTReadPageState, page) == 24);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_page>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTReadPageState>
{
    type Member = c::expression::CPointer<
        c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTReadPageState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTReadPageState> as c::expression::OrdinaryField<Field_page>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).page) };
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTReadPageState, offnum) == 32);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_offnum>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTReadPageState>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTReadPageState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTReadPageState> as c::expression::OrdinaryField<Field_offnum>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).offnum) };
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTReadPageState, skip) == 34);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_skip>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTReadPageState>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 34;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTReadPageState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTReadPageState> as c::expression::OrdinaryField<Field_skip>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).skip) };
#[cfg(any(feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
pub struct Field_continuescan;
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTReadPageState, continuescan) == 36);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_continuescan>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTReadPageState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTReadPageState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTReadPageState> as c::expression::OrdinaryField<Field_continuescan>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).continuescan) };
#[cfg(feature = "pg17")]
#[doc(hidden)]
pub struct Field_prechecked;
#[cfg(feature = "pg17")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTReadPageState, prechecked) == 37);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_prechecked>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTReadPageState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 37;
}
#[cfg(feature = "pg17")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTReadPageState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTReadPageState> as c::expression::OrdinaryField<Field_prechecked>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).prechecked) };
#[cfg(feature = "pg17")]
#[doc(hidden)]
pub struct Field_firstmatch;
#[cfg(feature = "pg17")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTReadPageState, firstmatch) == 38);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_firstmatch>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTReadPageState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 38;
}
#[cfg(feature = "pg17")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTReadPageState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTReadPageState> as c::expression::OrdinaryField<Field_firstmatch>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).firstmatch) };
#[cfg(any(feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
pub struct Field_rechecks;
#[cfg(feature = "pg17")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTReadPageState, rechecks) == 40);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rechecks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTReadPageState>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTReadPageState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTReadPageState> as c::expression::OrdinaryField<Field_rechecks>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rechecks) };
#[cfg(any(feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
pub struct Field_targetdistance;
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BTReadPageState, targetdistance) == 42
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_targetdistance>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTReadPageState>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 42;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTReadPageState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTReadPageState> as c::expression::OrdinaryField<Field_targetdistance>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).targetdistance) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_backward;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTScanInsertData, backward) == 4);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_backward>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTScanInsertData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTScanInsertData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTScanInsertData> as c::expression::OrdinaryField<Field_backward>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).backward) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BTScanOpaqueData>() == 27352);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BTScanOpaqueData>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BTScanOpaqueData, numArrayKeys) == 16
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numArrayKeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTScanOpaqueData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_needPrimScan;
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BTScanOpaqueData, needPrimScan) == 20
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_needPrimScan>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTScanOpaqueData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTScanOpaqueData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTScanOpaqueData> as c::expression::OrdinaryField<Field_needPrimScan>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).needPrimScan) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_scanBehind;
#[cfg(feature = "pg17")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTScanOpaqueData, scanBehind) == 21);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_scanBehind>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTScanOpaqueData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 21;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTScanOpaqueData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTScanOpaqueData> as c::expression::OrdinaryField<Field_scanBehind>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).scanBehind) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BTScanPosData>() == 13632);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BTScanPosData>() == 8);
};
#[cfg(feature = "pg17")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTScanPosData, dir) == 28);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dir>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTScanPosData>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ScanDirection,
        c::CInt,
        i32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BTScanPosData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BTScanPosData> as c::expression::OrdinaryField<Field_dir>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dir) };
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BTScanPosData, nextTupleOffset) == 32
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nextTupleOffset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTScanPosData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTScanPosData, firstItem) == 36);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_firstItem>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTScanPosData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTScanPosData, lastItem) == 40);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lastItem>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTScanPosData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BTScanPosData, itemIndex) == 44);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_itemIndex>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BTScanPosData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_canAcceptConnections;
#[cfg(feature = "pg17")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BackendStartupData>() == 4);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BackendStartupData>() == 4);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BackendStartupData {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BackendStartupData, canAcceptConnections)
        == 0
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_canAcceptConnections>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BackendStartupData>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__CAC_state,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BackendStartupData, <<c::expression::CRecord<crate::__pgrx_c_bindings::BackendStartupData> as c::expression::OrdinaryField<Field_canAcceptConnections>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).canAcceptConnections) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BackgroundWorker>() == 1472);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BackgroundWorker>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BackgroundWorker, bgw_main_arg) == 1328
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bgw_main_arg>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BackgroundWorker>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, crate::__pgrx_c_bindings::Datum>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1328;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BackgroundWorker, bgw_notify_pid) == 1464
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bgw_notify_pid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BackgroundWorker>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1464;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BackupState>() == 1112);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BackupState>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_istartpoint;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BackupState, istartpoint) == 1072);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_istartpoint>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BackupState>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1072;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BackupState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BackupState> as c::expression::OrdinaryField<Field_istartpoint>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).istartpoint) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_istarttli;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BackupState, istarttli) == 1080);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_istarttli>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BackupState>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1080;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::BackupState, <<c::expression::CRecord<crate::__pgrx_c_bindings::BackupState> as c::expression::OrdinaryField<Field_istarttli>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).istarttli) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BackupState, stoppoint) == 1088);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stoppoint>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BackupState>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1088;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BackupState, stoptli) == 1096);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stoptli>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BackupState>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1096;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::BackupState, stoptime) == 1104);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stoptime>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BackupState>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1104;
}
#[cfg(feature = "pg17")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BitmapHeapScanState>() == 328);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BitmapHeapScanState>() == 8);
};
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BitmapHeapScanState, pvmbuffer) == 256
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pvmbuffer>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BitmapHeapScanState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 256;
}
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BitmapHeapScanState, exact_pages) == 264
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_exact_pages>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BitmapHeapScanState>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 264;
}
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BitmapHeapScanState, lossy_pages) == 272
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lossy_pages>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BitmapHeapScanState>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 272;
}
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BitmapHeapScanState, prefetch_pages) == 288
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_prefetch_pages>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BitmapHeapScanState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 288;
}
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BitmapHeapScanState, prefetch_target) == 292
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_prefetch_target>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BitmapHeapScanState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 292;
}
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BitmapHeapScanState, prefetch_maximum) == 296
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_prefetch_maximum>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BitmapHeapScanState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 296;
}
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::BitmapHeapScanState, initialized) == 300
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_initialized>
    for c::expression::CRecord<crate::__pgrx_c_bindings::BitmapHeapScanState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 300;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CheckPoint, wal_level) == 20);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wal_level>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CheckPoint, <<c::expression::CRecord<crate::__pgrx_c_bindings::CheckPoint> as c::expression::OrdinaryField<Field_wal_level>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wal_level) };
#[cfg(feature = "pg17")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Constraint, skip_validation) == 18);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_skip_validation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Constraint>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 18;
}
#[cfg(feature = "pg17")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Constraint, initially_valid) == 19);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_initially_valid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Constraint>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 19;
}
#[cfg(feature = "pg17")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Constraint, is_no_inherit) == 20);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_no_inherit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Constraint>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Constraint, generated_when) == 40);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_generated_when>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Constraint>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(feature = "pg17")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Constraint, inhcount) == 44);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inhcount>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Constraint>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
#[cfg(feature = "pg17")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Constraint, <<c::expression::CRecord<crate::__pgrx_c_bindings::Constraint> as c::expression::OrdinaryField<Field_inhcount>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).inhcount) };
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::Constraint, nulls_not_distinct) == 48
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nulls_not_distinct>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Constraint>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Constraint, location) == 180);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Constraint>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 180;
}
#[cfg(feature = "pg17")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CopyFormatOptions>() == 176);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CopyFormatOptions>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_force_notnull_all;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, force_notnull_all) == 112
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_force_notnull_all>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFormatOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions> as c::expression::OrdinaryField<Field_force_notnull_all>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).force_notnull_all) };
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, force_null) == 128
);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_force_null>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_force_null_all;
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, force_null_all) == 136
);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_force_null_all>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFormatOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions> as c::expression::OrdinaryField<Field_force_null_all>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).force_null_all) };
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, convert_selectively)
        == 152
);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_convert_selectively>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_on_error;
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, on_error) == 156);
#[cfg(any(feature = "pg17", feature = "pg18"))]
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
    const OFFSET: usize = 156;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFormatOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions> as c::expression::OrdinaryField<Field_on_error>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).on_error) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_log_verbosity;
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, log_verbosity) == 160
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_log_verbosity>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__CopyLogVerbosityChoice,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFormatOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions> as c::expression::OrdinaryField<Field_log_verbosity>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).log_verbosity) };
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFormatOptions, convert_select) == 168
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_convert_select>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFormatOptions>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 168;
}
#[cfg(feature = "pg17")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CopyFromStateData>() == 552);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CopyFromStateData>() == 8);
};
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, cur_lineno) == 280
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cur_lineno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 280;
}
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, relname_only) == 304
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relname_only>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 304;
}
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, num_defaults) == 320
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_defaults>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 320;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_num_errors;
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, num_errors) == 352
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_errors>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 352;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CopyFromStateData, <<c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData> as c::expression::OrdinaryField<Field_num_errors>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_errors) };
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, volatile_defexprs) == 384
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_volatile_defexprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 384;
}
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, range_table) == 392
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_range_table>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 392;
}
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, rteperminfos) == 400
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rteperminfos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 400;
}
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, max_fields) == 448
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_max_fields>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 448;
}
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, line_buf_valid) == 488
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_line_buf_valid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 488;
}
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, input_buf_index) == 504
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_input_buf_index>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 504;
}
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, input_buf_len) == 508
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_input_buf_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 508;
}
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, input_reached_eof) == 512
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_input_reached_eof>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 512;
}
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, input_reached_error)
        == 513
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_input_reached_error>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 513;
}
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, raw_buf_index) == 528
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_raw_buf_index>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 528;
}
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, raw_buf_len) == 532
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_raw_buf_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 532;
}
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, raw_reached_eof) == 536
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_raw_reached_eof>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 536;
}
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CopyFromStateData, bytes_processed) == 544
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bytes_processed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CopyFromStateData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 544;
}
#[cfg(feature = "pg17")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CustomPath>() == 112);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CustomPath>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_custom_restrictinfo;
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::CustomPath, custom_restrictinfo) == 88
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_custom_restrictinfo>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CustomPath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::CustomPath, <<c::expression::CRecord<crate::__pgrx_c_bindings::CustomPath> as c::expression::OrdinaryField<Field_custom_restrictinfo>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).custom_restrictinfo) };
#[cfg(feature = "pg17")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::CustomPath, custom_private) == 96);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_custom_private>
    for c::expression::CRecord<crate::__pgrx_c_bindings::CustomPath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::DeallocateStmt>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::DeallocateStmt>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_isall;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DeallocateStmt, isall) == 16);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isall>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DeallocateStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DeallocateStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::DeallocateStmt> as c::expression::OrdinaryField<Field_isall>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isall) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::DeallocateStmt, location) == 20);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::DeallocateStmt>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::DeallocateStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::DeallocateStmt> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_memory;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExplainState, memory) == 15);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_memory>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 15;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExplainState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState> as c::expression::OrdinaryField<Field_memory>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).memory) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExplainState, settings) == 16);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_settings>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExplainState, generic) == 17);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_generic>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 17;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_serialize;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExplainState, serialize) == 20);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_serialize>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ExplainSerializeOption,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExplainState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState> as c::expression::OrdinaryField<Field_serialize>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).serialize) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExplainState, format) == 24);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_format>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ExplainFormat,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ExplainState, indent) == 28);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indent>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExplainState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(feature = "pg17")]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_29>(
        ) == 40
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_29>(
        ) == 8
    );
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_targettype;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_46>(
        ) == 32
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_46>(
        ) == 8
    );
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_46
{
}
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_46,
        targettype
    ) == 0
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_targettype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_46>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg17")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_46, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_46> as c::expression::OrdinaryField<Field_targettype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).targettype) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_targettypmod;
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_46,
        targettypmod
    ) == 4
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_targettypmod>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_46>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(feature = "pg17")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_46, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_46> as c::expression::OrdinaryField<Field_targettypmod>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).targettypmod) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_omit_quotes;
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_46,
        omit_quotes
    ) == 8
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_omit_quotes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_46>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg17")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_46, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_46> as c::expression::OrdinaryField<Field_omit_quotes>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).omit_quotes) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_exists_coerce;
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_46,
        exists_coerce
    ) == 9
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_exists_coerce>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_46>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 9;
}
#[cfg(feature = "pg17")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_46, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_46> as c::expression::OrdinaryField<Field_exists_coerce>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).exists_coerce) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_exists_cast_to_int;
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_46,
        exists_cast_to_int
    ) == 10
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_exists_cast_to_int>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_46>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 10;
}
#[cfg(feature = "pg17")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_46, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_46> as c::expression::OrdinaryField<Field_exists_cast_to_int>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).exists_cast_to_int) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_exists_check_domain;
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_46,
        exists_check_domain
    ) == 11
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_exists_check_domain>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_46>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 11;
}
#[cfg(feature = "pg17")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_46, <<c::expression::CRecord<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_46> as c::expression::OrdinaryField<Field_exists_check_domain>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).exists_check_domain) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ExprState>() == 120);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ExprState>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_dathasloginevt;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_database, dathasloginevt) == 79
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dathasloginevt>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_database>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 79;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_database, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_database> as c::expression::OrdinaryField<Field_dathasloginevt>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dathasloginevt) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::FormData_pg_statistic_ext>() == 104);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::FormData_pg_statistic_ext>() == 4);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_subfailover;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::FormData_pg_subscription, subfailover) == 91
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_subfailover>
    for c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_subscription>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 91;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::FormData_pg_subscription, <<c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_subscription> as c::expression::OrdinaryField<Field_subfailover>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).subfailover) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::GroupByOrdering>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::GroupByOrdering>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::GroupByOrdering {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GroupByOrdering, type_) == 0);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GroupByOrdering>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::GroupByOrdering, <<c::expression::CRecord<crate::__pgrx_c_bindings::GroupByOrdering> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GroupByOrdering, pathkeys) == 8);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pathkeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GroupByOrdering>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::GroupByOrdering, <<c::expression::CRecord<crate::__pgrx_c_bindings::GroupByOrdering> as c::expression::OrdinaryField<Field_pathkeys>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pathkeys) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_clauses;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::GroupByOrdering, clauses) == 16);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_clauses>
    for c::expression::CRecord<crate::__pgrx_c_bindings::GroupByOrdering>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::GroupByOrdering, <<c::expression::CRecord<crate::__pgrx_c_bindings::GroupByOrdering> as c::expression::OrdinaryField<Field_clauses>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).clauses) };
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::HeapScanDescData>() == 736);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::HeapScanDescData>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_rs_dir;
#[cfg(feature = "pg17")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapScanDescData, rs_dir) == 120);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rs_dir>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ScanDirection,
        c::CInt,
        i32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HeapScanDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData> as c::expression::OrdinaryField<Field_rs_dir>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rs_dir) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_rs_prefetch_block;
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapScanDescData, rs_prefetch_block) == 124
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rs_prefetch_block>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 124;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HeapScanDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData> as c::expression::OrdinaryField<Field_rs_prefetch_block>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rs_prefetch_block) };
#[cfg(any(feature = "pg17", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_rs_vmbuffer;
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapScanDescData, rs_vmbuffer) == 136
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rs_vmbuffer>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
#[cfg(any(feature = "pg17", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HeapScanDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData> as c::expression::OrdinaryField<Field_rs_vmbuffer>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rs_vmbuffer) };
#[cfg(feature = "pg17")]
#[doc(hidden)]
pub struct Field_rs_empty_tuples_pending;
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapScanDescData, rs_empty_tuples_pending)
        == 140
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rs_empty_tuples_pending>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 140;
}
#[cfg(feature = "pg17")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::HeapScanDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData> as c::expression::OrdinaryField<Field_rs_empty_tuples_pending>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rs_empty_tuples_pending) };
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapScanDescData, rs_cindex) == 144);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rs_cindex>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::HeapScanDescData, rs_ntuples) == 148);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rs_ntuples>
    for c::expression::CRecord<crate::__pgrx_c_bindings::HeapScanDescData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 148;
}
#[cfg(feature = "pg17")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::IndexAmRoutine>() == 216);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::IndexAmRoutine>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_amcanbuildparallel;
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexAmRoutine, amcanbuildparallel) == 22
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amcanbuildparallel>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 22;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine> as c::expression::OrdinaryField<Field_amcanbuildparallel>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).amcanbuildparallel) };
#[cfg(feature = "pg17")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexAmRoutine, amcaninclude) == 23);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amcaninclude>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 23;
}
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexAmRoutine, amusemaintenanceworkmem)
        == 24
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amusemaintenanceworkmem>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg17")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexAmRoutine, amsummarizing) == 25);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amsummarizing>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 25;
}
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexAmRoutine, amparallelvacuumoptions)
        == 26
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amparallelvacuumoptions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexAmRoutine>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 26;
}
#[cfg(feature = "pg17")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Instrumentation>() == 400);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Instrumentation>() == 8);
};
#[cfg(feature = "pg17")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Instrumentation, ntuples2) == 216);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ntuples2>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Instrumentation>
{
    type Member = c::expression::CDouble;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 216;
}
#[cfg(feature = "pg17")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Instrumentation, nfiltered1) == 232);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nfiltered1>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Instrumentation>
{
    type Member = c::expression::CDouble;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 232;
}
#[cfg(feature = "pg17")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Instrumentation, nfiltered2) == 240);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nfiltered2>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Instrumentation>
{
    type Member = c::expression::CDouble;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 240;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JitInstrumentation>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JitInstrumentation>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JsonArgument>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JsonArgument>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JsonArgument {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonArgument, type_) == 0);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonArgument>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonArgument, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonArgument> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JsonBehavior>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JsonBehavior>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JsonBehavior {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonBehavior, type_) == 0);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonBehavior>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonBehavior, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonBehavior> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_btype;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonBehavior, btype) == 4);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_btype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonBehavior>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__JsonBehaviorType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonBehavior, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonBehavior> as c::expression::OrdinaryField<Field_btype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).btype) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_coerce;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonBehavior, coerce) == 16);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_coerce>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonBehavior>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonBehavior, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonBehavior> as c::expression::OrdinaryField<Field_coerce>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).coerce) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonBehavior, location) == 20);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonBehavior>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonBehavior, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonBehavior> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JsonExpr>() == 104);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JsonExpr>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JsonExpr {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonExpr, op) == 4);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_op>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonExpr>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__JsonExprOp,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::JsonExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonExpr> as c::expression::OrdinaryField<
        Field_op,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).op) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_passing_names;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonExpr, passing_names) == 48);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_passing_names>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonExpr>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::JsonExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonExpr> as c::expression::OrdinaryField<
        Field_passing_names,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).passing_names) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_passing_values;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonExpr, passing_values) == 56);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_passing_values>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonExpr>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::JsonExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonExpr> as c::expression::OrdinaryField<
        Field_passing_values,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).passing_values) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_use_io_coercion;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonExpr, use_io_coercion) == 80);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_use_io_coercion>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonExpr>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::JsonExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonExpr> as c::expression::OrdinaryField<
        Field_use_io_coercion,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).use_io_coercion) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_use_json_coercion;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonExpr, use_json_coercion) == 81);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_use_json_coercion>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonExpr>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 81;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::JsonExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonExpr> as c::expression::OrdinaryField<
        Field_use_json_coercion,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).use_json_coercion) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_wrapper;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonExpr, wrapper) == 84);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wrapper>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonExpr>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__JsonWrapper,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 84;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::JsonExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonExpr> as c::expression::OrdinaryField<
        Field_wrapper,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wrapper) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonExpr, omit_quotes) == 88);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_omit_quotes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonExpr>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::JsonExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonExpr> as c::expression::OrdinaryField<
        Field_omit_quotes,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).omit_quotes) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonExpr, collation) == 92);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_collation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonExpr>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 92;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::JsonExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonExpr> as c::expression::OrdinaryField<
        Field_collation,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).collation) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonExpr, location) == 96);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonExpr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::JsonExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonExpr> as c::expression::OrdinaryField<
        Field_location,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JsonExprState>() == 120);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JsonExprState>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JsonExprState {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonExprState, args) == 40);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_args>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonExprState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonExprState, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonExprState> as c::expression::OrdinaryField<Field_args>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).args) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_jump_empty;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonExprState, jump_empty) == 80);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_jump_empty>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonExprState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonExprState, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonExprState> as c::expression::OrdinaryField<Field_jump_empty>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).jump_empty) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_jump_error;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonExprState, jump_error) == 84);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_jump_error>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonExprState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 84;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonExprState, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonExprState> as c::expression::OrdinaryField<Field_jump_error>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).jump_error) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_jump_eval_coercion;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonExprState, jump_eval_coercion) == 88
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_jump_eval_coercion>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonExprState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonExprState, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonExprState> as c::expression::OrdinaryField<Field_jump_eval_coercion>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).jump_eval_coercion) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_jump_end;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonExprState, jump_end) == 92);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_jump_end>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonExprState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 92;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonExprState, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonExprState> as c::expression::OrdinaryField<Field_jump_end>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).jump_end) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JsonFuncExpr>() == 80);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JsonFuncExpr>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JsonFuncExpr {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonFuncExpr, type_) == 0);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonFuncExpr>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonFuncExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonFuncExpr> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonFuncExpr, op) == 4);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_op>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonFuncExpr>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__JsonExprOp,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonFuncExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonFuncExpr> as c::expression::OrdinaryField<Field_op>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).op) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_passing;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonFuncExpr, passing) == 32);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_passing>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonFuncExpr>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonFuncExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonFuncExpr> as c::expression::OrdinaryField<Field_passing>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).passing) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonFuncExpr, wrapper) == 64);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wrapper>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonFuncExpr>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__JsonWrapper,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonFuncExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonFuncExpr> as c::expression::OrdinaryField<Field_wrapper>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wrapper) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_quotes;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonFuncExpr, quotes) == 68);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_quotes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonFuncExpr>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__JsonQuotes,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 68;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonFuncExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonFuncExpr> as c::expression::OrdinaryField<Field_quotes>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).quotes) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonFuncExpr, location) == 72);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonFuncExpr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonFuncExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonFuncExpr> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JsonParseExpr>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JsonParseExpr>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JsonParseExpr {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonParseExpr, type_) == 0);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonParseExpr>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonParseExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonParseExpr> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonParseExpr, unique_keys) == 24);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_unique_keys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonParseExpr>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonParseExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonParseExpr> as c::expression::OrdinaryField<Field_unique_keys>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).unique_keys) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonParseExpr, location) == 28);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonParseExpr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonParseExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonParseExpr> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JsonScalarExpr>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JsonScalarExpr>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JsonScalarExpr {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonScalarExpr, type_) == 0);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonScalarExpr>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonScalarExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonScalarExpr> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonScalarExpr, location) == 24);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonScalarExpr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonScalarExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonScalarExpr> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JsonSerializeExpr>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JsonSerializeExpr>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JsonSerializeExpr {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonSerializeExpr, type_) == 0);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonSerializeExpr>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonSerializeExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonSerializeExpr> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonSerializeExpr, location) == 24);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonSerializeExpr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonSerializeExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonSerializeExpr> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JsonTable>() == 64);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JsonTable>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JsonTable {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonTable, type_) == 0);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonTable>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::JsonTable,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonTable> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonTable, passing) == 24);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_passing>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonTable>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::JsonTable,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonTable> as c::expression::OrdinaryField<
        Field_passing,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).passing) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonTable, columns) == 32);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_columns>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonTable>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::JsonTable,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonTable> as c::expression::OrdinaryField<
        Field_columns,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).columns) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonTable, lateral) == 56);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lateral>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonTable>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::JsonTable,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonTable> as c::expression::OrdinaryField<
        Field_lateral,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lateral) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonTable, location) == 60);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonTable>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 60;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::JsonTable,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonTable> as c::expression::OrdinaryField<
        Field_location,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JsonTableColumn>() == 80);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JsonTableColumn>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JsonTableColumn {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonTableColumn, type_) == 0);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonTableColumn>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonTableColumn, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonTableColumn> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_coltype;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonTableColumn, coltype) == 4);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_coltype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonTableColumn>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__JsonTableColumnType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonTableColumn, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonTableColumn> as c::expression::OrdinaryField<Field_coltype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).coltype) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonTableColumn, wrapper) == 40);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wrapper>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonTableColumn>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__JsonWrapper,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonTableColumn, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonTableColumn> as c::expression::OrdinaryField<Field_wrapper>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wrapper) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonTableColumn, quotes) == 44);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_quotes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonTableColumn>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__JsonQuotes,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonTableColumn, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonTableColumn> as c::expression::OrdinaryField<Field_quotes>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).quotes) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonTableColumn, columns) == 48);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_columns>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonTableColumn>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonTableColumn, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonTableColumn> as c::expression::OrdinaryField<Field_columns>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).columns) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonTableColumn, location) == 72);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonTableColumn>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonTableColumn, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonTableColumn> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JsonTablePath>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JsonTablePath>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JsonTablePath {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonTablePath, type_) == 0);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonTablePath>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonTablePath, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonTablePath> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonTablePath, value) == 8);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::Const>() == 8);
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::Const>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_value>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonTablePath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::Const>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonTablePath, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonTablePath> as c::expression::OrdinaryField<Field_value>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).value) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_errorOnError;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JsonTablePathScan>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JsonTablePathScan>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JsonTablePathScan {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonTablePathScan, errorOnError) == 16
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_errorOnError>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonTablePathScan>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonTablePathScan, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonTablePathScan> as c::expression::OrdinaryField<Field_errorOnError>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).errorOnError) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_colMin;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonTablePathScan, colMin) == 32);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_colMin>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonTablePathScan>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonTablePathScan, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonTablePathScan> as c::expression::OrdinaryField<Field_colMin>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).colMin) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_colMax;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonTablePathScan, colMax) == 36);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_colMax>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonTablePathScan>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonTablePathScan, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonTablePathScan> as c::expression::OrdinaryField<Field_colMax>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).colMax) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JsonTablePathSpec>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JsonTablePathSpec>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JsonTablePathSpec {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonTablePathSpec, type_) == 0);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonTablePathSpec>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonTablePathSpec, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonTablePathSpec> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_name_location;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonTablePathSpec, name_location) == 24
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_name_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonTablePathSpec>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonTablePathSpec, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonTablePathSpec> as c::expression::OrdinaryField<Field_name_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).name_location) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonTablePathSpec, location) == 28);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonTablePathSpec>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonTablePathSpec, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonTablePathSpec> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JsonTablePlan>() == 4);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JsonTablePlan>() == 4);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JsonTablePlan {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonTablePlan, type_) == 0);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonTablePlan>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonTablePlan, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonTablePlan> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_proc_number;
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LocalPgBackendStatus, proc_number) == 432
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_proc_number>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LocalPgBackendStatus>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 432;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LocalPgBackendStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::LocalPgBackendStatus> as c::expression::OrdinaryField<Field_proc_number>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).proc_number) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_processing_required;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalDecodingContext, processing_required)
        == 301
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_processing_required>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalDecodingContext>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 301;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalDecodingContext, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalDecodingContext> as c::expression::OrdinaryField<Field_processing_required>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).processing_required) };
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LogicalRepWorker>() == 128);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LogicalRepWorker>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, type_) == 0);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__LogicalRepWorkerType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepWorker, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, launch_time) == 8);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_launch_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, in_use) == 16);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_in_use>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, generation) == 18);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_generation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 18;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, dbid) == 32);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dbid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, userid) == 36);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_userid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, subid) == 40);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_subid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, relid) == 44);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, relstate) == 48);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relstate>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, relstate_lsn) == 56
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relstate_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, relmutex) == 64);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relmutex>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, leader_pid) == 80);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_leader_pid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, parallel_apply) == 84
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parallel_apply>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 84;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, last_lsn) == 88);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_last_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, last_send_time) == 96
);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_last_send_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, last_recv_time) == 104
);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_last_recv_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, reply_lsn) == 112);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_reply_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, reply_time) == 120);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_reply_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_matchKind;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeAction, matchKind) == 4);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_matchKind>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeAction>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__MergeMatchKind,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MergeAction, <<c::expression::CRecord<crate::__pgrx_c_bindings::MergeAction> as c::expression::OrdinaryField<Field_matchKind>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).matchKind) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MergeStmt>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MergeStmt>() == 8);
};
#[cfg(feature = "pg17")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeStmt, returningList) == 40);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_returningList>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(feature = "pg17")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::MergeStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::MergeStmt> as c::expression::OrdinaryField<
        Field_returningList,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).returningList) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_msftype;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MergeSupportFunc>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MergeSupportFunc>() == 4);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::MergeSupportFunc {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeSupportFunc, msftype) == 4);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_msftype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeSupportFunc>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MergeSupportFunc, <<c::expression::CRecord<crate::__pgrx_c_bindings::MergeSupportFunc> as c::expression::OrdinaryField<Field_msftype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).msftype) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_msfcollid;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeSupportFunc, msfcollid) == 8);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_msfcollid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeSupportFunc>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MergeSupportFunc, <<c::expression::CRecord<crate::__pgrx_c_bindings::MergeSupportFunc> as c::expression::OrdinaryField<Field_msfcollid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).msfcollid) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeSupportFunc, location) == 12);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeSupportFunc>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MergeSupportFunc, <<c::expression::CRecord<crate::__pgrx_c_bindings::MergeSupportFunc> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeWhenClause, matchKind) == 4);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_matchKind>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeWhenClause>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__MergeMatchKind,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MergeWhenClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::MergeWhenClause> as c::expression::OrdinaryField<Field_matchKind>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).matchKind) };
#[cfg(feature = "pg17")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ModifyTable>() == 256);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ModifyTable>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_mergeJoinConditions;
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTable, mergeJoinConditions) == 248
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mergeJoinConditions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 248;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTable, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable> as c::expression::OrdinaryField<Field_mergeJoinConditions>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mergeJoinConditions) };
#[cfg(any(feature = "pg17", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ModifyTablePath>() == 176);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ModifyTablePath>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTablePath, mergeJoinConditions) == 168
);
#[cfg(any(feature = "pg17", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mergeJoinConditions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTablePath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 168;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTablePath, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTablePath> as c::expression::OrdinaryField<Field_mergeJoinConditions>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mergeJoinConditions) };
#[cfg(feature = "pg17")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ModifyTableState>() == 440);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ModifyTableState>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_in_streaming;
#[cfg(feature = "pg17")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGOutputData, in_streaming) == 16);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_in_streaming>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGOutputData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PGOutputData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PGOutputData> as c::expression::OrdinaryField<Field_in_streaming>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).in_streaming) };
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGOutputData, protocol_version) == 20
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_protocol_version>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGOutputData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_publish_no_origin;
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGOutputData, publish_no_origin) == 44
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_publish_no_origin>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGOutputData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PGOutputData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PGOutputData> as c::expression::OrdinaryField<Field_publish_no_origin>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).publish_no_origin) };
#[cfg(feature = "pg17")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PGPROC>() == 888);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PGPROC>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, pid) == 60);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 60;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, pgxactoff) == 64);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pgxactoff>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_vxid;
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, vxid) == 68);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::OffsetField<Field_vxid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC__bindgen_ty_1>;
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, vxid);
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PGPROC__bindgen_ty_1>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PGPROC__bindgen_ty_1>() == 4);
};
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vxid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC__bindgen_ty_1>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 68;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PGPROC,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC> as c::expression::OrdinaryField<
        Field_vxid,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).vxid) };
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, databaseId) == 76);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_databaseId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 76;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, roleId) == 80);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_roleId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, tempNamespaceId) == 84);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tempNamespaceId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 84;
}
#[cfg(feature = "pg17")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, isBackgroundWorker) == 88);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isBackgroundWorker>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, recoveryConflictPending) == 89
);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_recoveryConflictPending>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 89;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, lwWaiting) == 90);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lwWaiting>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 90;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, lwWaitMode) == 91);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lwWaitMode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 91;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, lwWaitLink) == 92);
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, cvWaitLink) == 100);
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, clogGroupMemberPage) == 736);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_clogGroupMemberPage>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 736;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, clogGroupMemberLsn) == 744);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_clogGroupMemberLsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 744;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, fpInfoLock) == 752);
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, fpLockBits) == 768);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fpLockBits>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 768;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, fpRelId) == 776);
#[cfg(feature = "pg17")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, fpVXIDLock) == 840);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fpVXIDLock>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 840;
}
#[cfg(feature = "pg17")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, fpLocalTransactionId) == 844);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fpLocalTransactionId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 844;
}
#[cfg(feature = "pg17")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, lockGroupLeader) == 848);
#[cfg(feature = "pg17")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, lockGroupMembers) == 856);
#[cfg(feature = "pg17")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, lockGroupLink) == 872);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PGPROC__bindgen_ty_1 {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC__bindgen_ty_1, procNumber) == 0
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::OffsetField<Field_procNumber>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC__bindgen_ty_1>
{
    type Member = ();
    const OFFSET: usize =
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC__bindgen_ty_1, procNumber);
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_procNumber>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC__bindgen_ty_1>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PGPROC__bindgen_ty_1, <<c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC__bindgen_ty_1> as c::expression::OrdinaryField<Field_procNumber>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).procNumber) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC__bindgen_ty_1, lxid) == 4);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::OffsetField<Field_lxid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC__bindgen_ty_1>
{
    type Member = ();
    const OFFSET: usize =
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC__bindgen_ty_1, lxid);
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lxid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC__bindgen_ty_1>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PGPROC__bindgen_ty_1, <<c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC__bindgen_ty_1> as c::expression::OrdinaryField<Field_lxid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lxid) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_num_timed;
#[cfg(feature = "pg17")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_CheckpointerStats>() == 72);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_CheckpointerStats>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_CheckpointerStats, num_timed) == 0
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_timed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_CheckpointerStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats> as c::expression::OrdinaryField<Field_num_timed>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_timed) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_num_requested;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_CheckpointerStats, num_requested) == 8
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_requested>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_CheckpointerStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats> as c::expression::OrdinaryField<Field_num_requested>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_requested) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_restartpoints_timed;
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::PgStat_CheckpointerStats,
        restartpoints_timed
    ) == 16
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_restartpoints_timed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_CheckpointerStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats> as c::expression::OrdinaryField<Field_restartpoints_timed>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).restartpoints_timed) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_restartpoints_requested;
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::PgStat_CheckpointerStats,
        restartpoints_requested
    ) == 24
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_restartpoints_requested>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_CheckpointerStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats> as c::expression::OrdinaryField<Field_restartpoints_requested>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).restartpoints_requested) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_restartpoints_performed;
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::PgStat_CheckpointerStats,
        restartpoints_performed
    ) == 32
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_restartpoints_performed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_CheckpointerStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats> as c::expression::OrdinaryField<Field_restartpoints_performed>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).restartpoints_performed) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_write_time;
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_CheckpointerStats, write_time) == 40
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_write_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_CheckpointerStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats> as c::expression::OrdinaryField<Field_write_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).write_time) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_sync_time;
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_CheckpointerStats, sync_time) == 48
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sync_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_CheckpointerStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats> as c::expression::OrdinaryField<Field_sync_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sync_time) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_buffers_written;
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_CheckpointerStats, buffers_written)
        == 56
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_buffers_written>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_CheckpointerStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats> as c::expression::OrdinaryField<Field_buffers_written>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).buffers_written) };
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::PgStat_CheckpointerStats,
        stat_reset_timestamp
    ) == 64
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stat_reset_timestamp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_CheckpointerStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats> as c::expression::OrdinaryField<Field_stat_reset_timestamp>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stat_reset_timestamp) };
#[cfg(feature = "pg17")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_IO>() == 16392);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_IO>() == 8);
};
#[cfg(feature = "pg17")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PlannerGlobal>() == 144);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PlannerGlobal>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, subpaths) == 24);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_subpaths>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerGlobal, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal> as c::expression::OrdinaryField<Field_subpaths>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).subpaths) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, subroots) == 32);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_subroots>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, finalrtable) == 48);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_finalrtable>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, finalrteperminfos) == 56
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_finalrteperminfos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(feature = "pg17")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, finalrowmarks) == 64);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_finalrowmarks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, resultRelations) == 72
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resultRelations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, appendRelations) == 80
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_appendRelations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(feature = "pg17")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, relationOids) == 88);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relationOids>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(feature = "pg17")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, invalItems) == 96);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_invalItems>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, paramExecTypes) == 104
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_paramExecTypes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(feature = "pg17")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, lastPHId) == 112);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lastPHId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
#[cfg(feature = "pg17")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, lastRowMarkId) == 116);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lastRowMarkId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 116;
}
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, lastPlanNodeId) == 120
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lastPlanNodeId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
#[cfg(feature = "pg17")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, transientPlan) == 124);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_transientPlan>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 124;
}
#[cfg(feature = "pg17")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, dependsOnRole) == 125);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dependsOnRole>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 125;
}
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, parallelModeOK) == 126
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parallelModeOK>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 126;
}
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, parallelModeNeeded) == 127
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parallelModeNeeded>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 127;
}
#[cfg(feature = "pg17")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, maxParallelHazard) == 128
);
#[cfg(feature = "pg17")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_maxParallelHazard>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
#[cfg(feature = "pg17")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PlannerInfo>() == 696);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PlannerInfo>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_setop_pathkeys;
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, setop_pathkeys) == 360);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_setop_pathkeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 360;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_setop_pathkeys>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).setop_pathkeys) };
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, part_schemes) == 368);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_part_schemes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 368;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, initial_rels) == 376);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_initial_rels>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 376;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, processed_groupClause) == 512
);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_processed_groupClause>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 512;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, processed_distinctClause) == 520
);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_processed_distinctClause>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 520;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, processed_tlist) == 528);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_processed_tlist>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 528;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, update_colnos) == 536);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_update_colnos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 536;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, minmax_aggs) == 552);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_minmax_aggs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 552;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, qual_security_level) == 592
);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_qual_security_level>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 592;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, hasJoinRTEs) == 596);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasJoinRTEs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 596;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, hasLateralRTEs) == 597);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasLateralRTEs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 597;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, hasHavingQual) == 598);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasHavingQual>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 598;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, hasPseudoConstantQuals) == 599
);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasPseudoConstantQuals>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 599;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, hasAlternativeSubPlans) == 600
);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasAlternativeSubPlans>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 600;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, placeholdersFrozen) == 601
);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_placeholdersFrozen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 601;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, hasRecursion) == 602);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasRecursion>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 602;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, agginfos) == 608);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_agginfos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 608;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, aggtransinfos) == 616);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aggtransinfos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 616;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, numOrderedAggs) == 624);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numOrderedAggs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 624;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, hasNonPartialAggs) == 628
);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasNonPartialAggs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 628;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, hasNonSerialAggs) == 629
);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasNonSerialAggs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 629;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, wt_param_id) == 632);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wt_param_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 632;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, curOuterParams) == 656);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_curOuterParams>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 656;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, partColsUpdated) == 688);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_partColsUpdated>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 688;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ProjectionInfo>() == 136);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ProjectionInfo>() == 8);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ProjectionInfo, pi_exprContext) == 128
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pi_exprContext>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ProjectionInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::ExprContext>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PruneFreezeResult>() == 620);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PruneFreezeResult>() == 4);
};
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PruneFreezeResult {}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PruneFreezeResult, ndeleted) == 0);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ndeleted>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PruneFreezeResult>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PruneFreezeResult, <<c::expression::CRecord<crate::__pgrx_c_bindings::PruneFreezeResult> as c::expression::OrdinaryField<Field_ndeleted>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ndeleted) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_nnewlpdead;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PruneFreezeResult, nnewlpdead) == 4);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nnewlpdead>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PruneFreezeResult>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PruneFreezeResult, <<c::expression::CRecord<crate::__pgrx_c_bindings::PruneFreezeResult> as c::expression::OrdinaryField<Field_nnewlpdead>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nnewlpdead) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_nfrozen;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PruneFreezeResult, nfrozen) == 8);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nfrozen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PruneFreezeResult>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PruneFreezeResult, <<c::expression::CRecord<crate::__pgrx_c_bindings::PruneFreezeResult> as c::expression::OrdinaryField<Field_nfrozen>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nfrozen) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PruneFreezeResult, live_tuples) == 12
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_live_tuples>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PruneFreezeResult>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PruneFreezeResult, <<c::expression::CRecord<crate::__pgrx_c_bindings::PruneFreezeResult> as c::expression::OrdinaryField<Field_live_tuples>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).live_tuples) };
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_recently_dead_tuples;
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PruneFreezeResult, recently_dead_tuples)
        == 16
);
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_recently_dead_tuples>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PruneFreezeResult>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PruneFreezeResult, <<c::expression::CRecord<crate::__pgrx_c_bindings::PruneFreezeResult> as c::expression::OrdinaryField<Field_recently_dead_tuples>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).recently_dead_tuples) };
#[cfg(any(feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
pub struct Field_all_visible;
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PruneFreezeResult, all_visible) == 20
);
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_all_visible>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PruneFreezeResult>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(any(feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PruneFreezeResult, <<c::expression::CRecord<crate::__pgrx_c_bindings::PruneFreezeResult> as c::expression::OrdinaryField<Field_all_visible>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).all_visible) };
#[cfg(any(feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
pub struct Field_all_frozen;
#[cfg(any(feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PruneFreezeResult, all_frozen) == 21);
