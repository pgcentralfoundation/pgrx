// Shared generated C macro support.

// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MemoryContextCounters, <<c::expression::CRecord<crate::__pgrx_c_bindings::MemoryContextCounters> as c::expression::OrdinaryField<Field_totalspace>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).totalspace) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::MemoryContextCounters, freespace) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_freespace>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MemoryContextCounters>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MemoryContextCounters, <<c::expression::CRecord<crate::__pgrx_c_bindings::MemoryContextCounters> as c::expression::OrdinaryField<Field_freespace>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).freespace) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MemoryContextData>() == 80);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MemoryContextData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::MemoryContextData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MemoryContextData, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MemoryContextData>
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
const _: Projection<crate::__pgrx_c_bindings::MemoryContextData, <<c::expression::CRecord<crate::__pgrx_c_bindings::MemoryContextData> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MemoryContextData, isReset) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isReset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MemoryContextData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MemoryContextData, <<c::expression::CRecord<crate::__pgrx_c_bindings::MemoryContextData> as c::expression::OrdinaryField<Field_isReset>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isReset) };
#[doc(hidden)]
pub struct Field_allowInCritSection;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::MemoryContextData, allowInCritSection) == 5
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_allowInCritSection>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MemoryContextData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 5;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MemoryContextData, <<c::expression::CRecord<crate::__pgrx_c_bindings::MemoryContextData> as c::expression::OrdinaryField<Field_allowInCritSection>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).allowInCritSection) };
#[doc(hidden)]
pub struct Field_mem_allocated;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::MemoryContextData, mem_allocated) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mem_allocated>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MemoryContextData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MemoryContextData, <<c::expression::CRecord<crate::__pgrx_c_bindings::MemoryContextData> as c::expression::OrdinaryField<Field_mem_allocated>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mem_allocated) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MergeAction>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MergeAction>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::MergeAction {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeAction, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeAction>
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
const _: Projection<crate::__pgrx_c_bindings::MergeAction, <<c::expression::CRecord<crate::__pgrx_c_bindings::MergeAction> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
#[doc(hidden)]
pub struct Field_matched;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeAction, matched) == 4);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_matched>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeAction>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MergeAction, <<c::expression::CRecord<crate::__pgrx_c_bindings::MergeAction> as c::expression::OrdinaryField<Field_matched>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).matched) };
#[doc(hidden)]
pub struct Field_commandType;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeAction, commandType) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_commandType>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeAction>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__CmdType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MergeAction, <<c::expression::CRecord<crate::__pgrx_c_bindings::MergeAction> as c::expression::OrdinaryField<Field_commandType>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).commandType) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeAction, override_) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_override>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeAction>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__OverridingKind,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MergeAction, <<c::expression::CRecord<crate::__pgrx_c_bindings::MergeAction> as c::expression::OrdinaryField<Field_override>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).override_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeAction, targetList) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_targetList>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeAction>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MergeAction, <<c::expression::CRecord<crate::__pgrx_c_bindings::MergeAction> as c::expression::OrdinaryField<Field_targetList>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).targetList) };
#[doc(hidden)]
pub struct Field_updateColnos;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeAction, updateColnos) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_updateColnos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeAction>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MergeAction, <<c::expression::CRecord<crate::__pgrx_c_bindings::MergeAction> as c::expression::OrdinaryField<Field_updateColnos>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).updateColnos) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MergeActionState>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MergeActionState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::MergeActionState {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeActionState, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeActionState>
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
const _: Projection<crate::__pgrx_c_bindings::MergeActionState, <<c::expression::CRecord<crate::__pgrx_c_bindings::MergeActionState> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_mergeplans;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MergeAppend>() == 168);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MergeAppend>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::MergeAppend {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeAppend, mergeplans) == 112);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mergeplans>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeAppend>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MergeAppend, <<c::expression::CRecord<crate::__pgrx_c_bindings::MergeAppend> as c::expression::OrdinaryField<Field_mergeplans>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mergeplans) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeAppend, numCols) == 120);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numCols>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeAppend>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MergeAppend, <<c::expression::CRecord<crate::__pgrx_c_bindings::MergeAppend> as c::expression::OrdinaryField<Field_numCols>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numCols) };
#[doc(hidden)]
pub struct Field_ms_nplans;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MergeAppendState>() == 264);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MergeAppendState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::MergeAppendState {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeAppendState, ms_nplans) == 208);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ms_nplans>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeAppendState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 208;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MergeAppendState, <<c::expression::CRecord<crate::__pgrx_c_bindings::MergeAppendState> as c::expression::OrdinaryField<Field_ms_nplans>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ms_nplans) };
#[doc(hidden)]
pub struct Field_ms_nkeys;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeAppendState, ms_nkeys) == 212);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ms_nkeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeAppendState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 212;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MergeAppendState, <<c::expression::CRecord<crate::__pgrx_c_bindings::MergeAppendState> as c::expression::OrdinaryField<Field_ms_nkeys>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ms_nkeys) };
#[doc(hidden)]
pub struct Field_ms_initialized;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeAppendState, ms_initialized) == 240
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ms_initialized>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeAppendState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 240;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MergeAppendState, <<c::expression::CRecord<crate::__pgrx_c_bindings::MergeAppendState> as c::expression::OrdinaryField<Field_ms_initialized>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ms_initialized) };
#[doc(hidden)]
pub struct Field_skip_mark_restore;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MergeJoin>() == 168);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MergeJoin>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::MergeJoin {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeJoin, skip_mark_restore) == 120);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_skip_mark_restore>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeJoin>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::MergeJoin,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::MergeJoin> as c::expression::OrdinaryField<
        Field_skip_mark_restore,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).skip_mark_restore) };
#[doc(hidden)]
pub struct Field_mergeclauses;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeJoin, mergeclauses) == 128);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mergeclauses>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeJoin>
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
    crate::__pgrx_c_bindings::MergeJoin,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::MergeJoin> as c::expression::OrdinaryField<
        Field_mergeclauses,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mergeclauses) };
#[doc(hidden)]
pub struct Field_mj_NumClauses;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MergeJoinState>() == 304);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MergeJoinState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::MergeJoinState {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeJoinState, mj_NumClauses) == 216
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mj_NumClauses>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeJoinState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 216;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MergeJoinState, <<c::expression::CRecord<crate::__pgrx_c_bindings::MergeJoinState> as c::expression::OrdinaryField<Field_mj_NumClauses>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mj_NumClauses) };
#[doc(hidden)]
pub struct Field_mj_JoinState;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeJoinState, mj_JoinState) == 232);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mj_JoinState>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeJoinState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 232;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MergeJoinState, <<c::expression::CRecord<crate::__pgrx_c_bindings::MergeJoinState> as c::expression::OrdinaryField<Field_mj_JoinState>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mj_JoinState) };
#[doc(hidden)]
pub struct Field_mj_SkipMarkRestore;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeJoinState, mj_SkipMarkRestore) == 236
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mj_SkipMarkRestore>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeJoinState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 236;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MergeJoinState, <<c::expression::CRecord<crate::__pgrx_c_bindings::MergeJoinState> as c::expression::OrdinaryField<Field_mj_SkipMarkRestore>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mj_SkipMarkRestore) };
#[doc(hidden)]
pub struct Field_mj_ExtraMarks;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeJoinState, mj_ExtraMarks) == 237
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mj_ExtraMarks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeJoinState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 237;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MergeJoinState, <<c::expression::CRecord<crate::__pgrx_c_bindings::MergeJoinState> as c::expression::OrdinaryField<Field_mj_ExtraMarks>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mj_ExtraMarks) };
#[doc(hidden)]
pub struct Field_mj_ConstFalseJoin;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeJoinState, mj_ConstFalseJoin) == 238
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mj_ConstFalseJoin>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeJoinState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 238;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MergeJoinState, <<c::expression::CRecord<crate::__pgrx_c_bindings::MergeJoinState> as c::expression::OrdinaryField<Field_mj_ConstFalseJoin>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mj_ConstFalseJoin) };
#[doc(hidden)]
pub struct Field_mj_FillOuter;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeJoinState, mj_FillOuter) == 239);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mj_FillOuter>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeJoinState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 239;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MergeJoinState, <<c::expression::CRecord<crate::__pgrx_c_bindings::MergeJoinState> as c::expression::OrdinaryField<Field_mj_FillOuter>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mj_FillOuter) };
#[doc(hidden)]
pub struct Field_mj_FillInner;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeJoinState, mj_FillInner) == 240);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mj_FillInner>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeJoinState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 240;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MergeJoinState, <<c::expression::CRecord<crate::__pgrx_c_bindings::MergeJoinState> as c::expression::OrdinaryField<Field_mj_FillInner>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mj_FillInner) };
#[doc(hidden)]
pub struct Field_mj_MatchedOuter;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeJoinState, mj_MatchedOuter) == 241
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mj_MatchedOuter>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeJoinState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 241;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MergeJoinState, <<c::expression::CRecord<crate::__pgrx_c_bindings::MergeJoinState> as c::expression::OrdinaryField<Field_mj_MatchedOuter>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mj_MatchedOuter) };
#[doc(hidden)]
pub struct Field_mj_MatchedInner;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeJoinState, mj_MatchedInner) == 242
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mj_MatchedInner>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeJoinState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 242;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MergeJoinState, <<c::expression::CRecord<crate::__pgrx_c_bindings::MergeJoinState> as c::expression::OrdinaryField<Field_mj_MatchedInner>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mj_MatchedInner) };
#[doc(hidden)]
pub struct Field_path_mergeclauses;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MergePath>() == 136);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MergePath>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::MergePath {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MergePath, path_mergeclauses) == 104);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_path_mergeclauses>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergePath>
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
    crate::__pgrx_c_bindings::MergePath,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::MergePath> as c::expression::OrdinaryField<
        Field_path_mergeclauses,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).path_mergeclauses) };
#[doc(hidden)]
pub struct Field_outersortkeys;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MergePath, outersortkeys) == 112);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_outersortkeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergePath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::MergePath,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::MergePath> as c::expression::OrdinaryField<
        Field_outersortkeys,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).outersortkeys) };
#[doc(hidden)]
pub struct Field_innersortkeys;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MergePath, innersortkeys) == 120);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_innersortkeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergePath>
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
    crate::__pgrx_c_bindings::MergePath,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::MergePath> as c::expression::OrdinaryField<
        Field_innersortkeys,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).innersortkeys) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MergePath, skip_mark_restore) == 128);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_skip_mark_restore>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergePath>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::MergePath,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::MergePath> as c::expression::OrdinaryField<
        Field_skip_mark_restore,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).skip_mark_restore) };
#[doc(hidden)]
pub struct Field_materialize_inner;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MergePath, materialize_inner) == 129);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_materialize_inner>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergePath>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 129;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::MergePath,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::MergePath> as c::expression::OrdinaryField<
        Field_materialize_inner,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).materialize_inner) };
#[doc(hidden)]
pub struct Field_opfamily;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MergeScanSelCache>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MergeScanSelCache>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::MergeScanSelCache {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeScanSelCache, opfamily) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_opfamily>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeScanSelCache>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MergeScanSelCache, <<c::expression::CRecord<crate::__pgrx_c_bindings::MergeScanSelCache> as c::expression::OrdinaryField<Field_opfamily>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).opfamily) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeScanSelCache, collation) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_collation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeScanSelCache>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MergeScanSelCache, <<c::expression::CRecord<crate::__pgrx_c_bindings::MergeScanSelCache> as c::expression::OrdinaryField<Field_collation>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).collation) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeScanSelCache, strategy) == 8);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_strategy>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeScanSelCache>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MergeScanSelCache, <<c::expression::CRecord<crate::__pgrx_c_bindings::MergeScanSelCache> as c::expression::OrdinaryField<Field_strategy>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).strategy) };
#[doc(hidden)]
pub struct Field_nulls_first;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeScanSelCache, nulls_first) == 12
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nulls_first>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeScanSelCache>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MergeScanSelCache, <<c::expression::CRecord<crate::__pgrx_c_bindings::MergeScanSelCache> as c::expression::OrdinaryField<Field_nulls_first>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nulls_first) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MergeStmt>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MergeStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::MergeStmt {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeStmt>
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
    crate::__pgrx_c_bindings::MergeStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::MergeStmt> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_mergeWhenClauses;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeStmt, mergeWhenClauses) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mergeWhenClauses>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeStmt>
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
    crate::__pgrx_c_bindings::MergeStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::MergeStmt> as c::expression::OrdinaryField<
        Field_mergeWhenClauses,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mergeWhenClauses) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MergeWhenClause>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MergeWhenClause>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::MergeWhenClause {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeWhenClause, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeWhenClause>
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
const _: Projection<crate::__pgrx_c_bindings::MergeWhenClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::MergeWhenClause> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeWhenClause, matched) == 4);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_matched>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeWhenClause>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MergeWhenClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::MergeWhenClause> as c::expression::OrdinaryField<Field_matched>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).matched) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeWhenClause, commandType) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_commandType>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeWhenClause>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__CmdType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MergeWhenClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::MergeWhenClause> as c::expression::OrdinaryField<Field_commandType>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).commandType) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeWhenClause, override_) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_override>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeWhenClause>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__OverridingKind,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MergeWhenClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::MergeWhenClause> as c::expression::OrdinaryField<Field_override>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).override_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeWhenClause, targetList) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_targetList>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeWhenClause>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MergeWhenClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::MergeWhenClause> as c::expression::OrdinaryField<Field_targetList>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).targetList) };
#[doc(hidden)]
pub struct Field_values;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MergeWhenClause, values) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_values>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MergeWhenClause>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MergeWhenClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::MergeWhenClause> as c::expression::OrdinaryField<Field_values>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).values) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MinMaxAggInfo>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MinMaxAggInfo>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::MinMaxAggInfo {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MinMaxAggInfo, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MinMaxAggInfo>
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
const _: Projection<crate::__pgrx_c_bindings::MinMaxAggInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::MinMaxAggInfo> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MinMaxAggInfo, aggfnoid) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aggfnoid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MinMaxAggInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MinMaxAggInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::MinMaxAggInfo> as c::expression::OrdinaryField<Field_aggfnoid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).aggfnoid) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MinMaxAggInfo, aggsortop) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aggsortop>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MinMaxAggInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MinMaxAggInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::MinMaxAggInfo> as c::expression::OrdinaryField<Field_aggsortop>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).aggsortop) };
#[doc(hidden)]
pub struct Field_minmaxtype;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MinMaxExpr>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MinMaxExpr>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::MinMaxExpr {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MinMaxExpr, minmaxtype) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_minmaxtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MinMaxExpr>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MinMaxExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::MinMaxExpr> as c::expression::OrdinaryField<Field_minmaxtype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).minmaxtype) };
#[doc(hidden)]
pub struct Field_minmaxcollid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MinMaxExpr, minmaxcollid) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_minmaxcollid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MinMaxExpr>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MinMaxExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::MinMaxExpr> as c::expression::OrdinaryField<Field_minmaxcollid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).minmaxcollid) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MinMaxExpr, inputcollid) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inputcollid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MinMaxExpr>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MinMaxExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::MinMaxExpr> as c::expression::OrdinaryField<Field_inputcollid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).inputcollid) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MinMaxExpr, op) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_op>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MinMaxExpr>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__MinMaxOp,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MinMaxExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::MinMaxExpr> as c::expression::OrdinaryField<Field_op>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).op) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MinMaxExpr, args) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_args>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MinMaxExpr>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MinMaxExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::MinMaxExpr> as c::expression::OrdinaryField<Field_args>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).args) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MinMaxExpr, location) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MinMaxExpr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MinMaxExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::MinMaxExpr> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MinimalTupleData>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MinimalTupleData>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::MinimalTupleData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MinimalTupleData, t_len) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_t_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MinimalTupleData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MinimalTupleData, <<c::expression::CRecord<crate::__pgrx_c_bindings::MinimalTupleData> as c::expression::OrdinaryField<Field_t_len>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).t_len) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MinimalTupleData, t_infomask2) == 10);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_t_infomask2>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MinimalTupleData>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 10;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MinimalTupleData, <<c::expression::CRecord<crate::__pgrx_c_bindings::MinimalTupleData> as c::expression::OrdinaryField<Field_t_infomask2>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).t_infomask2) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MinimalTupleData, t_infomask) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_t_infomask>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MinimalTupleData>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MinimalTupleData, <<c::expression::CRecord<crate::__pgrx_c_bindings::MinimalTupleData> as c::expression::OrdinaryField<Field_t_infomask>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).t_infomask) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MinimalTupleData, t_hoff) == 14);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_t_hoff>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MinimalTupleData>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 14;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MinimalTupleData, <<c::expression::CRecord<crate::__pgrx_c_bindings::MinimalTupleData> as c::expression::OrdinaryField<Field_t_hoff>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).t_hoff) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MinimalTupleTableSlot>() == 112);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MinimalTupleTableSlot>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::MinimalTupleTableSlot {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MinimalTupleTableSlot, off) == 104);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_off>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MinimalTupleTableSlot>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MinimalTupleTableSlot, <<c::expression::CRecord<crate::__pgrx_c_bindings::MinimalTupleTableSlot> as c::expression::OrdinaryField<Field_off>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).off) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ModifyTable>() == 248);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ModifyTable>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ModifyTable {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTable, operation) == 104);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_operation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__CmdType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTable, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable> as c::expression::OrdinaryField<Field_operation>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).operation) };
#[doc(hidden)]
pub struct Field_canSetTag;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTable, canSetTag) == 108);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_canSetTag>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 108;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTable, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable> as c::expression::OrdinaryField<Field_canSetTag>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).canSetTag) };
#[doc(hidden)]
pub struct Field_nominalRelation;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTable, nominalRelation) == 112);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nominalRelation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTable, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable> as c::expression::OrdinaryField<Field_nominalRelation>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nominalRelation) };
#[doc(hidden)]
pub struct Field_rootRelation;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTable, rootRelation) == 116);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rootRelation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 116;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTable, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable> as c::expression::OrdinaryField<Field_rootRelation>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rootRelation) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
pub struct Field_partColsUpdated;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTable, partColsUpdated) == 120);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_partColsUpdated>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTable, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable> as c::expression::OrdinaryField<Field_partColsUpdated>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).partColsUpdated) };
#[doc(hidden)]
pub struct Field_resultRelations;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTable, resultRelations) == 128);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resultRelations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTable, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable> as c::expression::OrdinaryField<Field_resultRelations>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).resultRelations) };
#[doc(hidden)]
pub struct Field_updateColnosLists;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTable, updateColnosLists) == 136
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_updateColnosLists>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTable, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable> as c::expression::OrdinaryField<Field_updateColnosLists>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).updateColnosLists) };
#[doc(hidden)]
pub struct Field_withCheckOptionLists;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTable, withCheckOptionLists) == 144
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_withCheckOptionLists>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTable, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable> as c::expression::OrdinaryField<Field_withCheckOptionLists>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).withCheckOptionLists) };
#[doc(hidden)]
pub struct Field_returningLists;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTable, returningLists) == 152);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_returningLists>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTable, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable> as c::expression::OrdinaryField<Field_returningLists>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).returningLists) };
#[doc(hidden)]
pub struct Field_fdwPrivLists;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTable, fdwPrivLists) == 160);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fdwPrivLists>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTable, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable> as c::expression::OrdinaryField<Field_fdwPrivLists>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fdwPrivLists) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTable, rowMarks) == 176);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rowMarks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 176;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTable, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable> as c::expression::OrdinaryField<Field_rowMarks>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rowMarks) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTable, epqParam) == 184);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_epqParam>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 184;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTable, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable> as c::expression::OrdinaryField<Field_epqParam>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).epqParam) };
#[doc(hidden)]
pub struct Field_onConflictAction;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTable, onConflictAction) == 188
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_onConflictAction>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__OnConflictAction,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 188;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTable, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable> as c::expression::OrdinaryField<Field_onConflictAction>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).onConflictAction) };
#[doc(hidden)]
pub struct Field_arbiterIndexes;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTable, arbiterIndexes) == 192);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_arbiterIndexes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 192;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTable, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable> as c::expression::OrdinaryField<Field_arbiterIndexes>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).arbiterIndexes) };
#[doc(hidden)]
pub struct Field_onConflictSet;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTable, onConflictSet) == 200);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_onConflictSet>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 200;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTable, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable> as c::expression::OrdinaryField<Field_onConflictSet>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).onConflictSet) };
#[doc(hidden)]
pub struct Field_onConflictCols;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTable, onConflictCols) == 208);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_onConflictCols>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 208;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTable, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable> as c::expression::OrdinaryField<Field_onConflictCols>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).onConflictCols) };
#[doc(hidden)]
pub struct Field_exclRelRTI;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTable, exclRelRTI) == 224);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_exclRelRTI>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 224;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTable, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable> as c::expression::OrdinaryField<Field_exclRelRTI>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).exclRelRTI) };
#[doc(hidden)]
pub struct Field_exclRelTlist;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTable, exclRelTlist) == 232);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_exclRelTlist>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 232;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTable, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable> as c::expression::OrdinaryField<Field_exclRelTlist>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).exclRelTlist) };
#[doc(hidden)]
pub struct Field_mergeActionLists;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTable, mergeActionLists) == 240
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mergeActionLists>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 240;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTable, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTable> as c::expression::OrdinaryField<Field_mergeActionLists>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mergeActionLists) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ModifyTablePath>() == 168);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ModifyTablePath>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ModifyTablePath {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTablePath, operation) == 80);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_operation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTablePath>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__CmdType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTablePath, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTablePath> as c::expression::OrdinaryField<Field_operation>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).operation) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTablePath, canSetTag) == 84);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_canSetTag>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTablePath>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 84;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTablePath, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTablePath> as c::expression::OrdinaryField<Field_canSetTag>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).canSetTag) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTablePath, nominalRelation) == 88
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nominalRelation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTablePath>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTablePath, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTablePath> as c::expression::OrdinaryField<Field_nominalRelation>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nominalRelation) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTablePath, rootRelation) == 92);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rootRelation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTablePath>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 92;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTablePath, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTablePath> as c::expression::OrdinaryField<Field_rootRelation>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rootRelation) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTablePath, partColsUpdated) == 96
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_partColsUpdated>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTablePath>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTablePath, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTablePath> as c::expression::OrdinaryField<Field_partColsUpdated>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).partColsUpdated) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTablePath, resultRelations) == 104
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resultRelations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTablePath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTablePath, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTablePath> as c::expression::OrdinaryField<Field_resultRelations>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).resultRelations) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTablePath, updateColnosLists) == 112
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_updateColnosLists>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTablePath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTablePath, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTablePath> as c::expression::OrdinaryField<Field_updateColnosLists>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).updateColnosLists) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTablePath, withCheckOptionLists) == 120
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_withCheckOptionLists>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTablePath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTablePath, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTablePath> as c::expression::OrdinaryField<Field_withCheckOptionLists>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).withCheckOptionLists) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTablePath, returningLists) == 128
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_returningLists>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTablePath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTablePath, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTablePath> as c::expression::OrdinaryField<Field_returningLists>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).returningLists) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTablePath, rowMarks) == 136);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rowMarks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTablePath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTablePath, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTablePath> as c::expression::OrdinaryField<Field_rowMarks>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rowMarks) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTablePath, epqParam) == 152);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_epqParam>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTablePath>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTablePath, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTablePath> as c::expression::OrdinaryField<Field_epqParam>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).epqParam) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTablePath, mergeActionLists) == 160
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mergeActionLists>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTablePath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTablePath, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTablePath> as c::expression::OrdinaryField<Field_mergeActionLists>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mergeActionLists) };
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ModifyTableState>() == 408);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ModifyTableState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ModifyTableState {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTableState, operation) == 200);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_operation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTableState>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__CmdType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 200;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTableState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTableState> as c::expression::OrdinaryField<Field_operation>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).operation) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTableState, canSetTag) == 204);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_canSetTag>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTableState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 204;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTableState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTableState> as c::expression::OrdinaryField<Field_canSetTag>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).canSetTag) };
#[doc(hidden)]
pub struct Field_mt_done;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTableState, mt_done) == 205);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mt_done>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTableState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 205;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTableState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTableState> as c::expression::OrdinaryField<Field_mt_done>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mt_done) };
#[doc(hidden)]
pub struct Field_mt_nrels;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTableState, mt_nrels) == 208);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mt_nrels>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTableState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 208;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTableState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTableState> as c::expression::OrdinaryField<Field_mt_nrels>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mt_nrels) };
#[doc(hidden)]
pub struct Field_fireBSTriggers;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTableState, fireBSTriggers) == 320
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fireBSTriggers>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTableState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 320;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTableState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTableState> as c::expression::OrdinaryField<Field_fireBSTriggers>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fireBSTriggers) };
#[doc(hidden)]
pub struct Field_mt_resultOidAttno;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTableState, mt_resultOidAttno) == 324
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mt_resultOidAttno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTableState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 324;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTableState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTableState> as c::expression::OrdinaryField<Field_mt_resultOidAttno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mt_resultOidAttno) };
#[doc(hidden)]
pub struct Field_mt_lastResultOid;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTableState, mt_lastResultOid) == 328
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mt_lastResultOid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTableState>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 328;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTableState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTableState> as c::expression::OrdinaryField<Field_mt_lastResultOid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mt_lastResultOid) };
#[doc(hidden)]
pub struct Field_mt_lastResultIndex;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTableState, mt_lastResultIndex) == 332
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mt_lastResultIndex>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTableState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 332;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTableState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTableState> as c::expression::OrdinaryField<Field_mt_lastResultIndex>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mt_lastResultIndex) };
#[doc(hidden)]
pub struct Field_mt_merge_subcommands;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTableState, mt_merge_subcommands)
        == 376
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mt_merge_subcommands>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTableState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 376;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTableState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTableState> as c::expression::OrdinaryField<Field_mt_merge_subcommands>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mt_merge_subcommands) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MultiAssignRef>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MultiAssignRef>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::MultiAssignRef {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MultiAssignRef, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MultiAssignRef>
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
const _: Projection<crate::__pgrx_c_bindings::MultiAssignRef, <<c::expression::CRecord<crate::__pgrx_c_bindings::MultiAssignRef> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_colno;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MultiAssignRef, colno) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_colno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MultiAssignRef>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MultiAssignRef, <<c::expression::CRecord<crate::__pgrx_c_bindings::MultiAssignRef> as c::expression::OrdinaryField<Field_colno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).colno) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MultiAssignRef, ncolumns) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ncolumns>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MultiAssignRef>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MultiAssignRef, <<c::expression::CRecord<crate::__pgrx_c_bindings::MultiAssignRef> as c::expression::OrdinaryField<Field_ncolumns>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ncolumns) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MultiSortSupportData>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MultiSortSupportData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::MultiSortSupportData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MultiSortSupportData, ndims) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ndims>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MultiSortSupportData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MultiSortSupportData, <<c::expression::CRecord<crate::__pgrx_c_bindings::MultiSortSupportData> as c::expression::OrdinaryField<Field_ndims>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ndims) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MultiXactMember>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MultiXactMember>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::MultiXactMember {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MultiXactMember, xid) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MultiXactMember>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MultiXactMember, <<c::expression::CRecord<crate::__pgrx_c_bindings::MultiXactMember> as c::expression::OrdinaryField<Field_xid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xid) };
#[doc(hidden)]
pub struct Field_status;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MultiXactMember, status) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_status>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MultiXactMember>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_MultiXactStatus,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MultiXactMember, <<c::expression::CRecord<crate::__pgrx_c_bindings::MultiXactMember> as c::expression::OrdinaryField<Field_status>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).status) };
#[doc(hidden)]
pub struct Field_argnumber;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::NamedArgExpr>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::NamedArgExpr>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::NamedArgExpr {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::NamedArgExpr, argnumber) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_argnumber>
    for c::expression::CRecord<crate::__pgrx_c_bindings::NamedArgExpr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::NamedArgExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::NamedArgExpr> as c::expression::OrdinaryField<Field_argnumber>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).argnumber) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::NamedArgExpr, location) == 28);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::NamedArgExpr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::NamedArgExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::NamedArgExpr> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
pub struct Field_trancheId;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::NamedLWLockTranche>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::NamedLWLockTranche>() == 8);
};
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::NamedLWLockTranche {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::NamedLWLockTranche, trancheId) == 0);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_trancheId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::NamedLWLockTranche>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::NamedLWLockTranche, <<c::expression::CRecord<crate::__pgrx_c_bindings::NamedLWLockTranche> as c::expression::OrdinaryField<Field_trancheId>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).trancheId) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::NamedTuplestoreScanState>() == 248);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::NamedTuplestoreScanState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::NamedTuplestoreScanState {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::NamedTuplestoreScanState, readptr) == 224
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_readptr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::NamedTuplestoreScanState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 224;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::NamedTuplestoreScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::NamedTuplestoreScanState> as c::expression::OrdinaryField<Field_readptr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).readptr) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::NestLoopParam>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::NestLoopParam>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::NestLoopParam {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::NestLoopParam, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::NestLoopParam>
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
const _: Projection<crate::__pgrx_c_bindings::NestLoopParam, <<c::expression::CRecord<crate::__pgrx_c_bindings::NestLoopParam> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_paramno;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::NestLoopParam, paramno) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_paramno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::NestLoopParam>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::NestLoopParam, <<c::expression::CRecord<crate::__pgrx_c_bindings::NestLoopParam> as c::expression::OrdinaryField<Field_paramno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).paramno) };
#[doc(hidden)]
pub struct Field_nl_NeedNewOuter;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::NestLoopState>() == 232);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::NestLoopState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::NestLoopState {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::NestLoopState, nl_NeedNewOuter) == 216
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nl_NeedNewOuter>
    for c::expression::CRecord<crate::__pgrx_c_bindings::NestLoopState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 216;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::NestLoopState, <<c::expression::CRecord<crate::__pgrx_c_bindings::NestLoopState> as c::expression::OrdinaryField<Field_nl_NeedNewOuter>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nl_NeedNewOuter) };
#[doc(hidden)]
pub struct Field_nl_MatchedOuter;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::NestLoopState, nl_MatchedOuter) == 217
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nl_MatchedOuter>
    for c::expression::CRecord<crate::__pgrx_c_bindings::NestLoopState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 217;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::NestLoopState, <<c::expression::CRecord<crate::__pgrx_c_bindings::NestLoopState> as c::expression::OrdinaryField<Field_nl_MatchedOuter>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nl_MatchedOuter) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::NextValueExpr>() == 12);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::NextValueExpr>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::NextValueExpr {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::NextValueExpr, seqid) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_seqid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::NextValueExpr>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::NextValueExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::NextValueExpr> as c::expression::OrdinaryField<Field_seqid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).seqid) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::NextValueExpr, typeId) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typeId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::NextValueExpr>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::NextValueExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::NextValueExpr> as c::expression::OrdinaryField<Field_typeId>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typeId) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::NotifyStmt>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::NotifyStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::NotifyStmt {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::NotifyStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::NotifyStmt>
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
const _: Projection<crate::__pgrx_c_bindings::NotifyStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::NotifyStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_nulltesttype;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::NullTest>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::NullTest>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::NullTest {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::NullTest, nulltesttype) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nulltesttype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::NullTest>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NullTestType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::NullTest,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::NullTest> as c::expression::OrdinaryField<
        Field_nulltesttype,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nulltesttype) };
#[doc(hidden)]
pub struct Field_argisrow;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::NullTest, argisrow) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_argisrow>
    for c::expression::CRecord<crate::__pgrx_c_bindings::NullTest>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::NullTest,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::NullTest> as c::expression::OrdinaryField<
        Field_argisrow,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).argisrow) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::NullTest, location) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::NullTest>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::NullTest,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::NullTest> as c::expression::OrdinaryField<
        Field_location,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::NullableDatum>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::NullableDatum>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::NullableDatum {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::NullableDatum, value) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_value>
    for c::expression::CRecord<crate::__pgrx_c_bindings::NullableDatum>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, crate::__pgrx_c_bindings::Datum>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::NullableDatum, <<c::expression::CRecord<crate::__pgrx_c_bindings::NullableDatum> as c::expression::OrdinaryField<Field_value>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).value) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::NullableDatum, isnull) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isnull>
    for c::expression::CRecord<crate::__pgrx_c_bindings::NullableDatum>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::NullableDatum, <<c::expression::CRecord<crate::__pgrx_c_bindings::NullableDatum> as c::expression::OrdinaryField<Field_isnull>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isnull) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ObjectWithArgs>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ObjectWithArgs>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ObjectWithArgs {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ObjectWithArgs, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ObjectWithArgs>
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
const _: Projection<crate::__pgrx_c_bindings::ObjectWithArgs, <<c::expression::CRecord<crate::__pgrx_c_bindings::ObjectWithArgs> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_objname;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ObjectWithArgs, objname) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_objname>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ObjectWithArgs>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ObjectWithArgs, <<c::expression::CRecord<crate::__pgrx_c_bindings::ObjectWithArgs> as c::expression::OrdinaryField<Field_objname>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).objname) };
#[doc(hidden)]
pub struct Field_objargs;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ObjectWithArgs, objargs) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_objargs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ObjectWithArgs>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ObjectWithArgs, <<c::expression::CRecord<crate::__pgrx_c_bindings::ObjectWithArgs> as c::expression::OrdinaryField<Field_objargs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).objargs) };
#[doc(hidden)]
pub struct Field_objfuncargs;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ObjectWithArgs, objfuncargs) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_objfuncargs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ObjectWithArgs>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ObjectWithArgs, <<c::expression::CRecord<crate::__pgrx_c_bindings::ObjectWithArgs> as c::expression::OrdinaryField<Field_objfuncargs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).objfuncargs) };
#[doc(hidden)]
pub struct Field_args_unspecified;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ObjectWithArgs, args_unspecified) == 32
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_args_unspecified>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ObjectWithArgs>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ObjectWithArgs, <<c::expression::CRecord<crate::__pgrx_c_bindings::ObjectWithArgs> as c::expression::OrdinaryField<Field_args_unspecified>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).args_unspecified) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::OnConflictClause>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::OnConflictClause>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::OnConflictClause {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::OnConflictClause, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OnConflictClause>
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
const _: Projection<crate::__pgrx_c_bindings::OnConflictClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::OnConflictClause> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::OnConflictClause, action) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_action>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OnConflictClause>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__OnConflictAction,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::OnConflictClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::OnConflictClause> as c::expression::OrdinaryField<Field_action>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).action) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::OnConflictClause, targetList) == 16);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_targetList>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OnConflictClause>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::OnConflictClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::OnConflictClause> as c::expression::OrdinaryField<Field_targetList>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).targetList) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::OnConflictClause, location) == 32);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OnConflictClause>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::OnConflictClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::OnConflictClause> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::OnConflictExpr>() == 64);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::OnConflictExpr>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::OnConflictExpr {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::OnConflictExpr, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OnConflictExpr>
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
const _: Projection<crate::__pgrx_c_bindings::OnConflictExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::OnConflictExpr> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::OnConflictExpr, action) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_action>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OnConflictExpr>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__OnConflictAction,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::OnConflictExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::OnConflictExpr> as c::expression::OrdinaryField<Field_action>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).action) };
#[doc(hidden)]
pub struct Field_arbiterElems;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::OnConflictExpr, arbiterElems) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_arbiterElems>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OnConflictExpr>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::OnConflictExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::OnConflictExpr> as c::expression::OrdinaryField<Field_arbiterElems>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).arbiterElems) };
#[doc(hidden)]
pub struct Field_constraint;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::OnConflictExpr, constraint) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_constraint>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OnConflictExpr>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::OnConflictExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::OnConflictExpr> as c::expression::OrdinaryField<Field_constraint>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).constraint) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::OnConflictExpr, onConflictSet) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_onConflictSet>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OnConflictExpr>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::OnConflictExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::OnConflictExpr> as c::expression::OrdinaryField<Field_onConflictSet>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).onConflictSet) };
#[doc(hidden)]
pub struct Field_exclRelIndex;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::OnConflictExpr, exclRelIndex) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_exclRelIndex>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OnConflictExpr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::OnConflictExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::OnConflictExpr> as c::expression::OrdinaryField<Field_exclRelIndex>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).exclRelIndex) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::OnConflictExpr, exclRelTlist) == 56);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_exclRelTlist>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OnConflictExpr>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::OnConflictExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::OnConflictExpr> as c::expression::OrdinaryField<Field_exclRelTlist>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).exclRelTlist) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::OnConflictSetState>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::OnConflictSetState>() == 8);
};
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::OnConflictSetState {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::OnConflictSetState, type_) == 0);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OnConflictSetState>
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
const _: Projection<crate::__pgrx_c_bindings::OnConflictSetState, <<c::expression::CRecord<crate::__pgrx_c_bindings::OnConflictSetState> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_opfamily_id;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::OpBtreeInterpretation>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::OpBtreeInterpretation>() == 4);
};
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::OpBtreeInterpretation {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::OpBtreeInterpretation, opfamily_id) == 0
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_opfamily_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OpBtreeInterpretation>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::OpBtreeInterpretation, <<c::expression::CRecord<crate::__pgrx_c_bindings::OpBtreeInterpretation> as c::expression::OrdinaryField<Field_opfamily_id>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).opfamily_id) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::OpBtreeInterpretation, strategy) == 4
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_strategy>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OpBtreeInterpretation>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::OpBtreeInterpretation, <<c::expression::CRecord<crate::__pgrx_c_bindings::OpBtreeInterpretation> as c::expression::OrdinaryField<Field_strategy>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).strategy) };
#[doc(hidden)]
pub struct Field_oplefttype;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::OpBtreeInterpretation, oplefttype) == 8
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oplefttype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OpBtreeInterpretation>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::OpBtreeInterpretation, <<c::expression::CRecord<crate::__pgrx_c_bindings::OpBtreeInterpretation> as c::expression::OrdinaryField<Field_oplefttype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oplefttype) };
#[doc(hidden)]
pub struct Field_oprighttype;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::OpBtreeInterpretation, oprighttype) == 12
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oprighttype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OpBtreeInterpretation>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::OpBtreeInterpretation, <<c::expression::CRecord<crate::__pgrx_c_bindings::OpBtreeInterpretation> as c::expression::OrdinaryField<Field_oprighttype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oprighttype) };
#[doc(hidden)]
pub struct Field_opno;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::OpExpr>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::OpExpr>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::OpExpr {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::OpExpr, opno) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_opno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OpExpr>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::OpExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::OpExpr> as c::expression::OrdinaryField<
        Field_opno,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).opno) };
#[doc(hidden)]
pub struct Field_opfuncid;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::OpExpr, opfuncid) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_opfuncid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OpExpr>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::OpExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::OpExpr> as c::expression::OrdinaryField<
        Field_opfuncid,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).opfuncid) };
#[doc(hidden)]
pub struct Field_opresulttype;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::OpExpr, opresulttype) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_opresulttype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OpExpr>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::OpExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::OpExpr> as c::expression::OrdinaryField<
        Field_opresulttype,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).opresulttype) };
#[doc(hidden)]
pub struct Field_opretset;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::OpExpr, opretset) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_opretset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OpExpr>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::OpExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::OpExpr> as c::expression::OrdinaryField<
        Field_opretset,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).opretset) };
#[doc(hidden)]
pub struct Field_opcollid;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::OpExpr, opcollid) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_opcollid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OpExpr>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::OpExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::OpExpr> as c::expression::OrdinaryField<
        Field_opcollid,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).opcollid) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::OpExpr, inputcollid) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inputcollid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OpExpr>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::OpExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::OpExpr> as c::expression::OrdinaryField<
        Field_inputcollid,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).inputcollid) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::OpExpr, args) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_args>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OpExpr>
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
    crate::__pgrx_c_bindings::OpExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::OpExpr> as c::expression::OrdinaryField<
        Field_args,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).args) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::OpExpr, location) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OpExpr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::OpExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::OpExpr> as c::expression::OrdinaryField<
        Field_location,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[doc(hidden)]
pub struct Field_is_func;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::OpFamilyMember>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::OpFamilyMember>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::OpFamilyMember {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::OpFamilyMember, is_func) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_func>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OpFamilyMember>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::OpFamilyMember, <<c::expression::CRecord<crate::__pgrx_c_bindings::OpFamilyMember> as c::expression::OrdinaryField<Field_is_func>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_func) };
#[doc(hidden)]
pub struct Field_object;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::OpFamilyMember, object) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_object>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OpFamilyMember>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::OpFamilyMember, <<c::expression::CRecord<crate::__pgrx_c_bindings::OpFamilyMember> as c::expression::OrdinaryField<Field_object>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).object) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::OpFamilyMember, number) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_number>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OpFamilyMember>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::OpFamilyMember, <<c::expression::CRecord<crate::__pgrx_c_bindings::OpFamilyMember> as c::expression::OrdinaryField<Field_number>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).number) };
#[doc(hidden)]
pub struct Field_lefttype;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::OpFamilyMember, lefttype) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lefttype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OpFamilyMember>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::OpFamilyMember, <<c::expression::CRecord<crate::__pgrx_c_bindings::OpFamilyMember> as c::expression::OrdinaryField<Field_lefttype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lefttype) };
#[doc(hidden)]
pub struct Field_righttype;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::OpFamilyMember, righttype) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_righttype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OpFamilyMember>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::OpFamilyMember, <<c::expression::CRecord<crate::__pgrx_c_bindings::OpFamilyMember> as c::expression::OrdinaryField<Field_righttype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).righttype) };
#[doc(hidden)]
pub struct Field_sortfamily;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::OpFamilyMember, sortfamily) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sortfamily>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OpFamilyMember>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::OpFamilyMember, <<c::expression::CRecord<crate::__pgrx_c_bindings::OpFamilyMember> as c::expression::OrdinaryField<Field_sortfamily>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sortfamily) };
#[doc(hidden)]
pub struct Field_ref_is_hard;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::OpFamilyMember, ref_is_hard) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ref_is_hard>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OpFamilyMember>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::OpFamilyMember, <<c::expression::CRecord<crate::__pgrx_c_bindings::OpFamilyMember> as c::expression::OrdinaryField<Field_ref_is_hard>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ref_is_hard) };
#[doc(hidden)]
pub struct Field_ref_is_family;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::OpFamilyMember, ref_is_family) == 25);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ref_is_family>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OpFamilyMember>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 25;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::OpFamilyMember, <<c::expression::CRecord<crate::__pgrx_c_bindings::OpFamilyMember> as c::expression::OrdinaryField<Field_ref_is_family>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ref_is_family) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::OpFamilyMember, refobjid) == 28);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_refobjid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OpFamilyMember>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::OpFamilyMember, <<c::expression::CRecord<crate::__pgrx_c_bindings::OpFamilyMember> as c::expression::OrdinaryField<Field_refobjid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).refobjid) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::OpFamilyOpFuncGroup>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::OpFamilyOpFuncGroup>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::OpFamilyOpFuncGroup {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::OpFamilyOpFuncGroup, lefttype) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lefttype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OpFamilyOpFuncGroup>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::OpFamilyOpFuncGroup, <<c::expression::CRecord<crate::__pgrx_c_bindings::OpFamilyOpFuncGroup> as c::expression::OrdinaryField<Field_lefttype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lefttype) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::OpFamilyOpFuncGroup, righttype) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_righttype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OpFamilyOpFuncGroup>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::OpFamilyOpFuncGroup, <<c::expression::CRecord<crate::__pgrx_c_bindings::OpFamilyOpFuncGroup> as c::expression::OrdinaryField<Field_righttype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).righttype) };
#[doc(hidden)]
pub struct Field_operatorset;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::OpFamilyOpFuncGroup, operatorset) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_operatorset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OpFamilyOpFuncGroup>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::OpFamilyOpFuncGroup, <<c::expression::CRecord<crate::__pgrx_c_bindings::OpFamilyOpFuncGroup> as c::expression::OrdinaryField<Field_operatorset>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).operatorset) };
#[doc(hidden)]
pub struct Field_functionset;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::OpFamilyOpFuncGroup, functionset) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_functionset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OpFamilyOpFuncGroup>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::OpFamilyOpFuncGroup, <<c::expression::CRecord<crate::__pgrx_c_bindings::OpFamilyOpFuncGroup> as c::expression::OrdinaryField<Field_functionset>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).functionset) };
#[doc(hidden)]
pub struct Field_output_type;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::OutputPluginOptions>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::OutputPluginOptions>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::OutputPluginOptions {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::OutputPluginOptions, output_type) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_output_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OutputPluginOptions>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__OutputPluginOutputType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::OutputPluginOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::OutputPluginOptions> as c::expression::OrdinaryField<Field_output_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).output_type) };
#[doc(hidden)]
pub struct Field_receive_rewrites;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::OutputPluginOptions, receive_rewrites) == 4
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_receive_rewrites>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OutputPluginOptions>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::OutputPluginOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::OutputPluginOptions> as c::expression::OrdinaryField<Field_receive_rewrites>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).receive_rewrites) };
#[doc(hidden)]
pub struct Field_schemas;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::OverrideSearchPath>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::OverrideSearchPath>() == 8);
};
#[cfg(any(feature = "pg15", feature = "pg16"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::OverrideSearchPath {}
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::OverrideSearchPath, schemas) == 0);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_schemas>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OverrideSearchPath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::OverrideSearchPath, <<c::expression::CRecord<crate::__pgrx_c_bindings::OverrideSearchPath> as c::expression::OrdinaryField<Field_schemas>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).schemas) };
#[doc(hidden)]
pub struct Field_addCatalog;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::OverrideSearchPath, addCatalog) == 8);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_addCatalog>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OverrideSearchPath>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::OverrideSearchPath, <<c::expression::CRecord<crate::__pgrx_c_bindings::OverrideSearchPath> as c::expression::OrdinaryField<Field_addCatalog>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).addCatalog) };
#[doc(hidden)]
pub struct Field_addTemp;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::OverrideSearchPath, addTemp) == 9);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_addTemp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OverrideSearchPath>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 9;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::OverrideSearchPath, <<c::expression::CRecord<crate::__pgrx_c_bindings::OverrideSearchPath> as c::expression::OrdinaryField<Field_addTemp>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).addTemp) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::OverrideSearchPath, generation) == 16
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_generation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OverrideSearchPath>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::OverrideSearchPath, <<c::expression::CRecord<crate::__pgrx_c_bindings::OverrideSearchPath> as c::expression::OrdinaryField<Field_generation>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).generation) };
#[cfg(any(feature = "pg15", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PGOutputData>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PGOutputData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PGOutputData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGOutputData, context) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_context>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGOutputData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::MemoryContextData>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PGOutputData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PGOutputData> as c::expression::OrdinaryField<Field_context>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).context) };
#[doc(hidden)]
pub struct Field_protocol_version;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGOutputData, protocol_version) == 16
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_protocol_version>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGOutputData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PGOutputData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PGOutputData> as c::expression::OrdinaryField<Field_protocol_version>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).protocol_version) };
#[doc(hidden)]
pub struct Field_publication_names;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGOutputData, publication_names) == 24
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_publication_names>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGOutputData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PGOutputData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PGOutputData> as c::expression::OrdinaryField<Field_publication_names>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).publication_names) };
#[doc(hidden)]
pub struct Field_publications;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGOutputData, publications) == 32);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_publications>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGOutputData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PGOutputData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PGOutputData> as c::expression::OrdinaryField<Field_publications>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).publications) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGOutputData, binary) == 40);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_binary>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGOutputData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PGOutputData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PGOutputData> as c::expression::OrdinaryField<Field_binary>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).binary) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGOutputData, streaming) == 41);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_streaming>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGOutputData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 41;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PGOutputData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PGOutputData> as c::expression::OrdinaryField<Field_streaming>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).streaming) };
#[doc(hidden)]
pub struct Field_messages;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGOutputData, messages) == 42);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_messages>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGOutputData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 42;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PGOutputData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PGOutputData> as c::expression::OrdinaryField<Field_messages>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).messages) };
#[doc(hidden)]
pub struct Field_two_phase;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGOutputData, two_phase) == 43);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_two_phase>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGOutputData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 43;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PGOutputData, <<c::expression::CRecord<crate::__pgrx_c_bindings::PGOutputData> as c::expression::OrdinaryField<Field_two_phase>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).two_phase) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
pub struct Field_links;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PGPROC>() == 880);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PGPROC>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PGPROC {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, links) == 0);
#[cfg(feature = "pg15")]
impl c::expression::OffsetField<Field_links>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::SHM_QUEUE>;
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, links);
}
#[doc(hidden)]
pub struct Field_procgloballist;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, procgloballist) == 16);
impl c::expression::OffsetField<Field_procgloballist>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = ();
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, procgloballist);
}
#[doc(hidden)]
pub struct Field_sem;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, sem) == 24);
impl c::expression::OffsetField<Field_sem>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = ();
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, sem);
}
#[doc(hidden)]
pub struct Field_waitStatus;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, waitStatus) == 32);
impl c::expression::OffsetField<Field_waitStatus>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = ();
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, waitStatus);
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_waitStatus>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_ProcWaitStatus,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PGPROC,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC> as c::expression::OrdinaryField<
        Field_waitStatus,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).waitStatus) };
#[doc(hidden)]
pub struct Field_procLatch;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, procLatch) == 36);
impl c::expression::OffsetField<Field_procLatch>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::Latch>;
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, procLatch);
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, xid) == 52);
impl c::expression::OffsetField<Field_xid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = ();
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, xid);
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PGPROC,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC> as c::expression::OrdinaryField<
        Field_xid,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xid) };
#[doc(hidden)]
pub struct Field_xmin;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, xmin) == 56);
impl c::expression::OffsetField<Field_xmin>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = ();
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, xmin);
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xmin>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PGPROC,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC> as c::expression::OrdinaryField<
        Field_xmin,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xmin) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, lxid) == 60);
#[cfg(any(feature = "pg15", feature = "pg16"))]
impl c::expression::OffsetField<Field_lxid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = ();
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, lxid);
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lxid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 60;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PGPROC,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC> as c::expression::OrdinaryField<
        Field_lxid,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lxid) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, pid) == 64);
impl c::expression::OffsetField<Field_pid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = ();
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, pid);
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PGPROC,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC> as c::expression::OrdinaryField<
        Field_pid,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pid) };
#[doc(hidden)]
pub struct Field_pgxactoff;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, pgxactoff) == 68);
impl c::expression::OffsetField<Field_pgxactoff>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = ();
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, pgxactoff);
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pgxactoff>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 68;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PGPROC,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC> as c::expression::OrdinaryField<
        Field_pgxactoff,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pgxactoff) };
#[doc(hidden)]
pub struct Field_pgprocno;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, pgprocno) == 72);
#[cfg(any(feature = "pg15", feature = "pg16"))]
impl c::expression::OffsetField<Field_pgprocno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = ();
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, pgprocno);
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pgprocno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PGPROC,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC> as c::expression::OrdinaryField<
        Field_pgprocno,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pgprocno) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, backendId) == 76);
#[cfg(any(feature = "pg15", feature = "pg16"))]
impl c::expression::OffsetField<Field_backendId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = ();
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, backendId);
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_backendId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 76;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PGPROC,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC> as c::expression::OrdinaryField<
        Field_backendId,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).backendId) };
#[doc(hidden)]
pub struct Field_databaseId;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, databaseId) == 80);
impl c::expression::OffsetField<Field_databaseId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = ();
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, databaseId);
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_databaseId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PGPROC,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC> as c::expression::OrdinaryField<
        Field_databaseId,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).databaseId) };
#[doc(hidden)]
pub struct Field_roleId;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, roleId) == 84);
impl c::expression::OffsetField<Field_roleId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = ();
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, roleId);
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_roleId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 84;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PGPROC,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC> as c::expression::OrdinaryField<
        Field_roleId,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).roleId) };
#[doc(hidden)]
pub struct Field_tempNamespaceId;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, tempNamespaceId) == 88);
impl c::expression::OffsetField<Field_tempNamespaceId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = ();
    const OFFSET: usize =
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, tempNamespaceId);
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tempNamespaceId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PGPROC,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC> as c::expression::OrdinaryField<
        Field_tempNamespaceId,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tempNamespaceId) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
#[doc(hidden)]
pub struct Field_isBackgroundWorker;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, isBackgroundWorker) == 92);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
impl c::expression::OffsetField<Field_isBackgroundWorker>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = ();
    const OFFSET: usize =
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, isBackgroundWorker);
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isBackgroundWorker>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 92;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PGPROC,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC> as c::expression::OrdinaryField<
        Field_isBackgroundWorker,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isBackgroundWorker) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
pub struct Field_recoveryConflictPending;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, recoveryConflictPending) == 93
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
impl c::expression::OffsetField<Field_recoveryConflictPending>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = ();
    const OFFSET: usize =
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, recoveryConflictPending);
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_recoveryConflictPending>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 93;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PGPROC,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC> as c::expression::OrdinaryField<
        Field_recoveryConflictPending,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).recoveryConflictPending) };
#[doc(hidden)]
pub struct Field_lwWaiting;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, lwWaiting) == 94);
impl c::expression::OffsetField<Field_lwWaiting>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = ();
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, lwWaiting);
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lwWaiting>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 94;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PGPROC,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC> as c::expression::OrdinaryField<
        Field_lwWaiting,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lwWaiting) };
#[doc(hidden)]
pub struct Field_lwWaitMode;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, lwWaitMode) == 95);
impl c::expression::OffsetField<Field_lwWaitMode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = ();
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, lwWaitMode);
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lwWaitMode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 95;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PGPROC,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC> as c::expression::OrdinaryField<
        Field_lwWaitMode,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lwWaitMode) };
#[doc(hidden)]
pub struct Field_lwWaitLink;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, lwWaitLink) == 96);
impl c::expression::OffsetField<Field_lwWaitLink>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::proclist_node>;
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, lwWaitLink);
}
#[doc(hidden)]
pub struct Field_cvWaitLink;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, cvWaitLink) == 104);
impl c::expression::OffsetField<Field_cvWaitLink>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::proclist_node>;
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, cvWaitLink);
}
#[doc(hidden)]
pub struct Field_waitLock;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, waitLock) == 112);
impl c::expression::OffsetField<Field_waitLock>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = ();
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, waitLock);
}
#[doc(hidden)]
pub struct Field_waitProcLock;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, waitProcLock) == 120);
impl c::expression::OffsetField<Field_waitProcLock>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = ();
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, waitProcLock);
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, waitLockMode) == 128);
impl c::expression::OffsetField<Field_waitLockMode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = ();
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, waitLockMode);
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_waitLockMode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PGPROC,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC> as c::expression::OrdinaryField<
        Field_waitLockMode,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).waitLockMode) };
#[doc(hidden)]
pub struct Field_heldLocks;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, heldLocks) == 132);
impl c::expression::OffsetField<Field_heldLocks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = ();
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, heldLocks);
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_heldLocks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 132;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PGPROC,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC> as c::expression::OrdinaryField<
        Field_heldLocks,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).heldLocks) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, waitStart) == 136);
impl c::expression::OffsetField<Field_waitStart>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::pg_atomic_uint64>;
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, waitStart);
}
#[doc(hidden)]
pub struct Field_delayChkptFlags;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, delayChkptFlags) == 144);
impl c::expression::OffsetField<Field_delayChkptFlags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = ();
    const OFFSET: usize =
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, delayChkptFlags);
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_delayChkptFlags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PGPROC,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC> as c::expression::OrdinaryField<
        Field_delayChkptFlags,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).delayChkptFlags) };
#[doc(hidden)]
pub struct Field_statusFlags;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, statusFlags) == 148);
impl c::expression::OffsetField<Field_statusFlags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = ();
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, statusFlags);
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_statusFlags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 148;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PGPROC,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC> as c::expression::OrdinaryField<
        Field_statusFlags,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).statusFlags) };
#[doc(hidden)]
pub struct Field_waitLSN;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, waitLSN) == 152);
impl c::expression::OffsetField<Field_waitLSN>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = ();
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, waitLSN);
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_waitLSN>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PGPROC,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC> as c::expression::OrdinaryField<
        Field_waitLSN,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).waitLSN) };
#[doc(hidden)]
pub struct Field_syncRepState;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, syncRepState) == 160);
impl c::expression::OffsetField<Field_syncRepState>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = ();
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, syncRepState);
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_syncRepState>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PGPROC,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC> as c::expression::OrdinaryField<
        Field_syncRepState,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).syncRepState) };
#[doc(hidden)]
pub struct Field_syncRepLinks;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, syncRepLinks) == 168);
#[cfg(feature = "pg15")]
impl c::expression::OffsetField<Field_syncRepLinks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::SHM_QUEUE>;
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, syncRepLinks);
}
#[doc(hidden)]
pub struct Field_myProcLocks;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, myProcLocks) == 184);
impl c::expression::OffsetField<Field_myProcLocks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = ();
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, myProcLocks);
}
#[doc(hidden)]
pub struct Field_subxidStatus;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, subxidStatus) == 440);
impl c::expression::OffsetField<Field_subxidStatus>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::XidCacheStatus>;
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, subxidStatus);
}
#[doc(hidden)]
pub struct Field_subxids;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, subxids) == 444);
impl c::expression::OffsetField<Field_subxids>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::XidCache>;
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, subxids);
}
#[doc(hidden)]
pub struct Field_procArrayGroupMember;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, procArrayGroupMember) == 700);
impl c::expression::OffsetField<Field_procArrayGroupMember>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = ();
    const OFFSET: usize =
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, procArrayGroupMember);
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_procArrayGroupMember>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 700;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PGPROC,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC> as c::expression::OrdinaryField<
        Field_procArrayGroupMember,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).procArrayGroupMember) };
#[doc(hidden)]
pub struct Field_procArrayGroupNext;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, procArrayGroupNext) == 704);
impl c::expression::OffsetField<Field_procArrayGroupNext>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::pg_atomic_uint32>;
    const OFFSET: usize =
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, procArrayGroupNext);
}
#[doc(hidden)]
pub struct Field_procArrayGroupMemberXid;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, procArrayGroupMemberXid) == 708
);
impl c::expression::OffsetField<Field_procArrayGroupMemberXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = ();
    const OFFSET: usize =
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, procArrayGroupMemberXid);
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_procArrayGroupMemberXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 708;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PGPROC,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC> as c::expression::OrdinaryField<
        Field_procArrayGroupMemberXid,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).procArrayGroupMemberXid) };
#[doc(hidden)]
pub struct Field_wait_event_info;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, wait_event_info) == 712);
impl c::expression::OffsetField<Field_wait_event_info>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = ();
    const OFFSET: usize =
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, wait_event_info);
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wait_event_info>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 712;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PGPROC,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC> as c::expression::OrdinaryField<
        Field_wait_event_info,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wait_event_info) };
#[doc(hidden)]
pub struct Field_clogGroupMember;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, clogGroupMember) == 716);
impl c::expression::OffsetField<Field_clogGroupMember>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = ();
    const OFFSET: usize =
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, clogGroupMember);
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_clogGroupMember>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 716;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PGPROC,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC> as c::expression::OrdinaryField<
        Field_clogGroupMember,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).clogGroupMember) };
#[doc(hidden)]
pub struct Field_clogGroupNext;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, clogGroupNext) == 720);
impl c::expression::OffsetField<Field_clogGroupNext>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::pg_atomic_uint32>;
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, clogGroupNext);
}
#[doc(hidden)]
pub struct Field_clogGroupMemberXid;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, clogGroupMemberXid) == 724);
impl c::expression::OffsetField<Field_clogGroupMemberXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = ();
    const OFFSET: usize =
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, clogGroupMemberXid);
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_clogGroupMemberXid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 724;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PGPROC,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC> as c::expression::OrdinaryField<
        Field_clogGroupMemberXid,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).clogGroupMemberXid) };
#[doc(hidden)]
pub struct Field_clogGroupMemberXidStatus;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, clogGroupMemberXidStatus) == 728
);
impl c::expression::OffsetField<Field_clogGroupMemberXidStatus>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = ();
    const OFFSET: usize =
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, clogGroupMemberXidStatus);
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_clogGroupMemberXidStatus>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 728;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PGPROC,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC> as c::expression::OrdinaryField<
        Field_clogGroupMemberXidStatus,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).clogGroupMemberXidStatus) };
#[doc(hidden)]
pub struct Field_clogGroupMemberPage;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, clogGroupMemberPage) == 732);
impl c::expression::OffsetField<Field_clogGroupMemberPage>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = ();
    const OFFSET: usize =
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, clogGroupMemberPage);
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_clogGroupMemberPage>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 732;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PGPROC,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC> as c::expression::OrdinaryField<
        Field_clogGroupMemberPage,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).clogGroupMemberPage) };
#[doc(hidden)]
pub struct Field_clogGroupMemberLsn;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, clogGroupMemberLsn) == 736);
impl c::expression::OffsetField<Field_clogGroupMemberLsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = ();
    const OFFSET: usize =
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, clogGroupMemberLsn);
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_clogGroupMemberLsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 736;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PGPROC,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC> as c::expression::OrdinaryField<
        Field_clogGroupMemberLsn,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).clogGroupMemberLsn) };
#[doc(hidden)]
pub struct Field_fpInfoLock;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, fpInfoLock) == 744);
impl c::expression::OffsetField<Field_fpInfoLock>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::LWLock>;
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, fpInfoLock);
}
#[doc(hidden)]
pub struct Field_fpLockBits;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, fpLockBits) == 760);
impl c::expression::OffsetField<Field_fpLockBits>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = ();
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, fpLockBits);
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fpLockBits>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 760;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PGPROC,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC> as c::expression::OrdinaryField<
        Field_fpLockBits,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fpLockBits) };
#[doc(hidden)]
pub struct Field_fpRelId;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, fpRelId) == 768);
impl c::expression::OffsetField<Field_fpRelId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = ();
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, fpRelId);
}
#[doc(hidden)]
pub struct Field_fpVXIDLock;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, fpVXIDLock) == 832);
impl c::expression::OffsetField<Field_fpVXIDLock>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = ();
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, fpVXIDLock);
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fpVXIDLock>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 832;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PGPROC,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC> as c::expression::OrdinaryField<
        Field_fpVXIDLock,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fpVXIDLock) };
#[doc(hidden)]
pub struct Field_fpLocalTransactionId;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, fpLocalTransactionId) == 836);
impl c::expression::OffsetField<Field_fpLocalTransactionId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = ();
    const OFFSET: usize =
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, fpLocalTransactionId);
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fpLocalTransactionId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 836;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PGPROC,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC> as c::expression::OrdinaryField<
        Field_fpLocalTransactionId,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fpLocalTransactionId) };
#[doc(hidden)]
pub struct Field_lockGroupLeader;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, lockGroupLeader) == 840);
impl c::expression::OffsetField<Field_lockGroupLeader>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = ();
    const OFFSET: usize =
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, lockGroupLeader);
}
#[doc(hidden)]
pub struct Field_lockGroupMembers;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, lockGroupMembers) == 848);
impl c::expression::OffsetField<Field_lockGroupMembers>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::dlist_head>;
    const OFFSET: usize =
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, lockGroupMembers);
}
#[doc(hidden)]
pub struct Field_lockGroupLink;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, lockGroupLink) == 864);
impl c::expression::OffsetField<Field_lockGroupLink>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::dlist_node>;
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, lockGroupLink);
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PGShmemHeader>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PGShmemHeader>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PGShmemHeader {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGShmemHeader, magic) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_magic>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGShmemHeader>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PGShmemHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::PGShmemHeader> as c::expression::OrdinaryField<Field_magic>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).magic) };
#[doc(hidden)]
pub struct Field_creatorPID;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGShmemHeader, creatorPID) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_creatorPID>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGShmemHeader>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PGShmemHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::PGShmemHeader> as c::expression::OrdinaryField<Field_creatorPID>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).creatorPID) };
#[doc(hidden)]
pub struct Field_totalsize;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGShmemHeader, totalsize) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_totalsize>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGShmemHeader>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PGShmemHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::PGShmemHeader> as c::expression::OrdinaryField<Field_totalsize>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).totalsize) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
pub struct Field_freeoffset;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGShmemHeader, freeoffset) == 16);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_freeoffset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGShmemHeader>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PGShmemHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::PGShmemHeader> as c::expression::OrdinaryField<Field_freeoffset>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).freeoffset) };
#[doc(hidden)]
pub struct Field_dsm_control;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGShmemHeader, dsm_control) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dsm_control>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGShmemHeader>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PGShmemHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::PGShmemHeader> as c::expression::OrdinaryField<Field_dsm_control>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dsm_control) };
#[doc(hidden)]
pub struct Field_device;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGShmemHeader, device) == 40);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_device>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGShmemHeader>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PGShmemHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::PGShmemHeader> as c::expression::OrdinaryField<Field_device>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).device) };
#[doc(hidden)]
pub struct Field_inode;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGShmemHeader, inode) == 48);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGShmemHeader>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PGShmemHeader, <<c::expression::CRecord<crate::__pgrx_c_bindings::PGShmemHeader> as c::expression::OrdinaryField<Field_inode>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).inode) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLAssignStmt>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLAssignStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLAssignStmt {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLAssignStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLAssignStmt>
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
const _: Projection<crate::__pgrx_c_bindings::PLAssignStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLAssignStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLAssignStmt, indirection) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indirection>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLAssignStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLAssignStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLAssignStmt> as c::expression::OrdinaryField<Field_indirection>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indirection) };
#[doc(hidden)]
pub struct Field_nnames;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLAssignStmt, nnames) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nnames>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLAssignStmt>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLAssignStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLAssignStmt> as c::expression::OrdinaryField<Field_nnames>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nnames) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLAssignStmt, location) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLAssignStmt>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLAssignStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLAssignStmt> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_case_when>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_case_when>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_case_when {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_case_when, lineno) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lineno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_case_when>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_case_when, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_case_when> as c::expression::OrdinaryField<Field_lineno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lineno) };
#[doc(hidden)]
pub struct Field_stmts;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_case_when, stmts) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmts>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_case_when>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_case_when, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_case_when> as c::expression::OrdinaryField<Field_stmts>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stmts) };
#[doc(hidden)]
pub struct Field_sqlerrstate;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_condition>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_condition>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_condition {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_condition, sqlerrstate) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sqlerrstate>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_condition>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_condition, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_condition> as c::expression::OrdinaryField<Field_sqlerrstate>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sqlerrstate) };
#[doc(hidden)]
pub struct Field_dtype;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_datum>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_datum>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_datum {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_datum, dtype) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_datum>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PLpgSQL_datum_type,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_datum, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_datum> as c::expression::OrdinaryField<Field_dtype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dtype) };
#[doc(hidden)]
pub struct Field_dno;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_datum, dno) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_datum>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_datum, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_datum> as c::expression::OrdinaryField<Field_dno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dno) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_diag_item>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_diag_item>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_diag_item {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_diag_item, kind) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_kind>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_diag_item>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PLpgSQL_getdiag_kind,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_diag_item, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_diag_item> as c::expression::OrdinaryField<Field_kind>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).kind) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_diag_item, target) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_target>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_diag_item>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_diag_item, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_diag_item> as c::expression::OrdinaryField<Field_target>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).target) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_exception>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_exception>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_exception {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_exception, lineno) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lineno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_exception>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_exception, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_exception> as c::expression::OrdinaryField<Field_lineno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lineno) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_exception, action) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_action>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_exception>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_exception, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_exception> as c::expression::OrdinaryField<Field_action>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).action) };
#[doc(hidden)]
pub struct Field_sqlstate_varno;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_exception_block>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_exception_block>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_exception_block {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_exception_block, sqlstate_varno) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sqlstate_varno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_exception_block>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_exception_block, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_exception_block> as c::expression::OrdinaryField<Field_sqlstate_varno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sqlstate_varno) };
#[doc(hidden)]
pub struct Field_sqlerrm_varno;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_exception_block, sqlerrm_varno) == 4
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sqlerrm_varno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_exception_block>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_exception_block, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_exception_block> as c::expression::OrdinaryField<Field_sqlerrm_varno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sqlerrm_varno) };
#[doc(hidden)]
pub struct Field_exc_list;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_exception_block, exc_list) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_exc_list>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_exception_block>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_exception_block, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_exception_block> as c::expression::OrdinaryField<Field_exc_list>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).exc_list) };
#[doc(hidden)]
pub struct Field_retval;
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_execstate>() == 248);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_execstate>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PLpgSQL_execstate {}
