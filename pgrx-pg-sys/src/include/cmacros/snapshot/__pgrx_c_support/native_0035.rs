// Shared generated C macro support.

#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_withCheckOptionLists>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTablePath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTablePath, returningLists) == 136
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_returningLists>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTablePath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTablePath, rowMarks) == 144);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rowMarks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTablePath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTablePath, epqParam) == 160);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_epqParam>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTablePath>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTablePath, mergeActionLists) == 168
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mergeActionLists>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTablePath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 168;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTablePath, mergeJoinConditions) == 176
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mergeJoinConditions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTablePath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 176;
}
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ModifyTableState>() == 464);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ModifyTableState>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_mt_updateColnosLists;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTableState, mt_updateColnosLists)
        == 440
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mt_updateColnosLists>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTableState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 440;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTableState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTableState> as c::expression::OrdinaryField<Field_mt_updateColnosLists>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mt_updateColnosLists) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_mt_mergeActionLists;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTableState, mt_mergeActionLists) == 448
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mt_mergeActionLists>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTableState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 448;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTableState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTableState> as c::expression::OrdinaryField<Field_mt_mergeActionLists>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mt_mergeActionLists) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_mt_mergeJoinConditions;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ModifyTableState, mt_mergeJoinConditions)
        == 456
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mt_mergeJoinConditions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTableState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 456;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ModifyTableState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ModifyTableState> as c::expression::OrdinaryField<Field_mt_mergeJoinConditions>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mt_mergeJoinConditions) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::OpIndexInterpretation>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::OpIndexInterpretation>() == 4);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::OpIndexInterpretation {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::OpIndexInterpretation, opfamily_id) == 0
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_opfamily_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OpIndexInterpretation>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::OpIndexInterpretation, <<c::expression::CRecord<crate::__pgrx_c_bindings::OpIndexInterpretation> as c::expression::OrdinaryField<Field_opfamily_id>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).opfamily_id) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::OpIndexInterpretation, cmptype) == 4);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cmptype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OpIndexInterpretation>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__CompareType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::OpIndexInterpretation, <<c::expression::CRecord<crate::__pgrx_c_bindings::OpIndexInterpretation> as c::expression::OrdinaryField<Field_cmptype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cmptype) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::OpIndexInterpretation, oplefttype) == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oplefttype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OpIndexInterpretation>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::OpIndexInterpretation, <<c::expression::CRecord<crate::__pgrx_c_bindings::OpIndexInterpretation> as c::expression::OrdinaryField<Field_oplefttype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oplefttype) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::OpIndexInterpretation, oprighttype) == 12
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oprighttype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::OpIndexInterpretation>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::OpIndexInterpretation, <<c::expression::CRecord<crate::__pgrx_c_bindings::OpIndexInterpretation> as c::expression::OrdinaryField<Field_oprighttype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oprighttype) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGOutputData, in_streaming) == 24);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_in_streaming>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGOutputData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGOutputData, protocol_version) == 28
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_protocol_version>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGOutputData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGOutputData, publication_names) == 32
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_publication_names>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGOutputData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGOutputData, publications) == 40);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_publications>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGOutputData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGOutputData, binary) == 48);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_binary>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGOutputData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGOutputData, streaming) == 49);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_streaming>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGOutputData>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 49;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGOutputData, messages) == 50);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_messages>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGOutputData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 50;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGOutputData, two_phase) == 51);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_two_phase>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGOutputData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 51;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGOutputData, publish_no_origin) == 52
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_publish_no_origin>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGOutputData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PGPROC>() == 832);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PGPROC>() == 8);
};
#[cfg(feature = "pg18")]
#[doc(hidden)]
pub struct Field_isRegularBackend;
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, isRegularBackend) == 88);
#[cfg(feature = "pg18")]
impl c::expression::OffsetField<Field_isRegularBackend>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = ();
    const OFFSET: usize =
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, isRegularBackend);
}
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isRegularBackend>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(feature = "pg18")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PGPROC,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC> as c::expression::OrdinaryField<
        Field_isRegularBackend,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isRegularBackend) };
#[cfg(feature = "pg18")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, fpVXIDLock) == 784);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fpVXIDLock>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 784;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, fpLocalTransactionId) == 788);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fpLocalTransactionId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PGPROC>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 788;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, lockGroupLeader) == 792);
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, lockGroupMembers) == 800);
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PGPROC, lockGroupLink) == 816);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_expr>() == 128);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_expr>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_expr, target_param) == 32);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_target_param>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_expr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_target_is_local;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_expr, target_is_local) == 36);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_target_is_local>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_expr>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_expr, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_expr> as c::expression::OrdinaryField<Field_target_is_local>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).target_is_local) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_expr, expr_simple_type) == 64
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_expr_simple_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_expr>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_expr, expr_simple_typmod) == 68
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_expr_simple_typmod>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_expr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 68;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_expr, expr_simple_mutable) == 72
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_expr_simple_mutable>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_expr>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_expr_rwopt;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_expr, expr_rwopt) == 76);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_expr_rwopt>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_expr>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PLpgSQL_rwopt,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 76;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_expr, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_expr> as c::expression::OrdinaryField<Field_expr_rwopt>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).expr_rwopt) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_expr, expr_simple_plan_lxid) == 104
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_expr_simple_plan_lxid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_expr>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_expr, expr_simple_in_use) == 120
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_expr_simple_in_use>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_expr>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_expr, expr_simple_lxid) == 124
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_expr_simple_lxid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_expr>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 124;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PLpgSQL_function>() == 568);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PLpgSQL_function>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, fn_oid) == 48);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fn_oid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, fn_is_trigger) == 52
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fn_is_trigger>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PLpgSQL_trigtype,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, fn_input_collation) == 56
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fn_input_collation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, fn_rettype) == 72);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fn_rettype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, fn_rettyplen) == 76
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fn_rettyplen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 76;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, fn_retbyval) == 80);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fn_retbyval>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, fn_retistuple) == 81
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fn_retistuple>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 81;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, fn_retisdomain) == 82
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fn_retisdomain>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 82;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, fn_retset) == 83);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fn_retset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 83;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, fn_readonly) == 84);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fn_readonly>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 84;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, fn_prokind) == 85);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fn_prokind>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 85;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, fn_nargs) == 88);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fn_nargs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, out_param_varno) == 492
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_out_param_varno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 492;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, found_varno) == 496
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_found_varno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 496;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, new_varno) == 500);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_new_varno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 500;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, old_varno) == 504);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_old_varno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 504;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, resolve_option) == 508
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resolve_option>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PLpgSQL_resolve_option,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 508;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, print_strict_params) == 512
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_print_strict_params>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 512;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, extra_warnings) == 516
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_extra_warnings>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 516;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, extra_errors) == 520
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_extra_errors>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 520;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, ndatums) == 524);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ndatums>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 524;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, copiable_size) == 536
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_copiable_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 536;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, nstatements) == 552
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nstatements>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 552;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::PLpgSQL_function,
        requires_procedure_resowner
    ) == 556
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_requires_procedure_resowner>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 556;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_has_exception_block;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PLpgSQL_function, has_exception_block) == 557
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_has_exception_block>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 557;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PLpgSQL_function, <<c::expression::CRecord<crate::__pgrx_c_bindings::PLpgSQL_function> as c::expression::OrdinaryField<Field_has_exception_block>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).has_exception_block) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PROC_HDR>() == 128);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PROC_HDR>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_walwriterProc;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PROC_HDR, walwriterProc) == 112);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_walwriterProc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PROC_HDR>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PROC_HDR,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PROC_HDR> as c::expression::OrdinaryField<
        Field_walwriterProc,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).walwriterProc) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_checkpointerProc;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PROC_HDR, checkpointerProc) == 116);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_checkpointerProc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PROC_HDR>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 116;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PROC_HDR,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PROC_HDR> as c::expression::OrdinaryField<
        Field_checkpointerProc,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).checkpointerProc) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PROC_HDR, spins_per_delay) == 120);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_spins_per_delay>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PROC_HDR>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PROC_HDR, startupBufferPinWaitBufId) == 124
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_startupBufferPinWaitBufId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PROC_HDR>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 124;
}
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ParallelBitmapHeapState>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ParallelBitmapHeapState>() == 8);
};
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelBitmapHeapState, mutex) == 8);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mutex>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelBitmapHeapState>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelBitmapHeapState, state) == 12
);
#[cfg(feature = "pg18")]
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
    const OFFSET: usize = 12;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ParallelBlockTableScanDescData>() == 48
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ParallelBlockTableScanDescData>() == 8
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelBlockTableScanDescData, phs_nblocks)
        == 24
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_phs_nblocks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelBlockTableScanDescData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelBlockTableScanDescData, phs_mutex)
        == 28
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_phs_mutex>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelBlockTableScanDescData>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ParallelBlockTableScanDescData,
        phs_startblock
    ) == 32
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_phs_startblock>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelBlockTableScanDescData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg18")]
#[doc(hidden)]
pub struct Field_ps_offset_ins;
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ParallelIndexScanDescData>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ParallelIndexScanDescData>() == 8);
};
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelIndexScanDescData, ps_offset_ins)
        == 24
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ps_offset_ins>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelIndexScanDescData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg18")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelIndexScanDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelIndexScanDescData> as c::expression::OrdinaryField<Field_ps_offset_ins>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ps_offset_ins) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_ps_offset_am;
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelIndexScanDescData, ps_offset_am)
        == 32
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ps_offset_am>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelIndexScanDescData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParallelIndexScanDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParallelIndexScanDescData> as c::expression::OrdinaryField<Field_ps_offset_am>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ps_offset_am) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ParallelTableScanDescData>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ParallelTableScanDescData>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelTableScanDescData, phs_syncscan)
        == 12
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_phs_syncscan>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelTableScanDescData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelTableScanDescData, phs_snapshot_any)
        == 13
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_phs_snapshot_any>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelTableScanDescData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 13;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParallelTableScanDescData, phs_snapshot_off)
        == 16
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_phs_snapshot_off>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParallelTableScanDescData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ParseNamespaceColumn>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ParseNamespaceColumn>() == 4);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_p_varreturningtype;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseNamespaceColumn, p_varreturningtype)
        == 20
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_varreturningtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseNamespaceColumn>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__VarReturningType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParseNamespaceColumn, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParseNamespaceColumn> as c::expression::OrdinaryField<Field_p_varreturningtype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).p_varreturningtype) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseNamespaceColumn, p_varnosyn) == 24
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_varnosyn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseNamespaceColumn>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseNamespaceColumn, p_varattnosyn) == 28
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_varattnosyn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseNamespaceColumn>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseNamespaceColumn, p_dontexpand) == 30
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_dontexpand>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseNamespaceColumn>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 30;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_p_returning_type;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseNamespaceItem, p_returning_type) == 44
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_returning_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseNamespaceItem>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__VarReturningType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ParseNamespaceItem, <<c::expression::CRecord<crate::__pgrx_c_bindings::ParseNamespaceItem> as c::expression::OrdinaryField<Field_p_returning_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).p_returning_type) };
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ParseState>() == 232);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ParseState>() == 8);
};
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_is_insert) == 120);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_is_insert>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_windowdefs) == 128);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_windowdefs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_expr_kind) == 136);
#[cfg(feature = "pg18")]
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
    const OFFSET: usize = 136;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_next_resno) == 140);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_next_resno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 140;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_multiassign_exprs) == 144
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_multiassign_exprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_locking_clause) == 152);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_locking_clause>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_locked_from_parent) == 160
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_locked_from_parent>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_resolve_unknowns) == 161
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_resolve_unknowns>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 161;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_hasAggs) == 176);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_hasAggs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 176;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_hasWindowFuncs) == 177);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_hasWindowFuncs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 177;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_hasTargetSRFs) == 178);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_hasTargetSRFs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 178;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_hasSubLinks) == 179);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_hasSubLinks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 179;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ParseState, p_hasModifyingCTE) == 180
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_p_hasModifyingCTE>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ParseState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 180;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PartitionPruneInfo>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PartitionPruneInfo>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionPruneInfo, prune_infos) == 16
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_prune_infos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionPruneInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PartitionPruneState>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PartitionPruneState>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionPruneState, do_initial_prune) == 32
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_do_initial_prune>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionPruneState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionPruneState, do_exec_prune) == 33
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_do_exec_prune>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionPruneState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 33;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionPruneState, num_partprunedata) == 36
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_partprunedata>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionPruneState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PartitionedRelPruneInfo>() == 80);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PartitionedRelPruneInfo>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::PartitionedRelPruneInfo,
        initial_pruning_steps
    ) == 56
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_initial_pruning_steps>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionedRelPruneInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionedRelPruneInfo, exec_pruning_steps)
        == 64
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_exec_pruning_steps>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionedRelPruneInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PartitionedRelPruningData>() == 224);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PartitionedRelPruningData>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PartitionedRelPruningData, nparts) == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nparts>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionedRelPruningData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::PartitionedRelPruningData,
        initial_pruning_steps
    ) == 48
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_initial_pruning_steps>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionedRelPruningData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::PartitionedRelPruningData,
        exec_pruning_steps
    ) == 56
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_exec_pruning_steps>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PartitionedRelPruningData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Path>() == 80);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Path>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Path, disabled_nodes) == 48);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_disabled_nodes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Path>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Path,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Path> as c::expression::OrdinaryField<
        Field_disabled_nodes,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).disabled_nodes) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Path, pathkeys) == 72);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pathkeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Path>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_pk_cmptype;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PathKey, pk_cmptype) == 20);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pk_cmptype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PathKey>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__CompareType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PathKey,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PathKey> as c::expression::OrdinaryField<
        Field_pk_cmptype,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pk_cmptype) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_io_handle_off;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgAioBackend>() == 328);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgAioBackend>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgAioBackend {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgAioBackend, io_handle_off) == 0);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_io_handle_off>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgAioBackend>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgAioBackend, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgAioBackend> as c::expression::OrdinaryField<Field_io_handle_off>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).io_handle_off) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_in_batchmode;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgAioBackend, in_batchmode) == 40);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_in_batchmode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgAioBackend>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgAioBackend, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgAioBackend> as c::expression::OrdinaryField<Field_in_batchmode>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).in_batchmode) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_num_staged_ios;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgAioBackend, num_staged_ios) == 42);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_staged_ios>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgAioBackend>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 42;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgAioBackend, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgAioBackend> as c::expression::OrdinaryField<Field_num_staged_ios>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_staged_ios) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_backend_state_count;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgAioCtl>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgAioCtl>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgAioCtl {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgAioCtl, backend_state_count) == 0);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_backend_state_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgAioCtl>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PgAioCtl,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PgAioCtl> as c::expression::OrdinaryField<
        Field_backend_state_count,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).backend_state_count) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_iovec_count;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgAioCtl, iovec_count) == 16);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_iovec_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgAioCtl>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PgAioCtl,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PgAioCtl> as c::expression::OrdinaryField<
        Field_iovec_count,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).iovec_count) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_io_handle_count;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgAioCtl, io_handle_count) == 40);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_io_handle_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgAioCtl>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::PgAioCtl,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::PgAioCtl> as c::expression::OrdinaryField<
        Field_io_handle_count,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).io_handle_count) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgAioHandle>() == 144);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgAioHandle>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgAioHandle {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgAioHandle, state) == 0);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_state>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgAioHandle>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgAioHandle, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgAioHandle> as c::expression::OrdinaryField<Field_state>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).state) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgAioHandle, target) == 1);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_target>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgAioHandle>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgAioHandle, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgAioHandle> as c::expression::OrdinaryField<Field_target>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).target) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgAioHandle, op) == 2);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_op>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgAioHandle>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 2;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgAioHandle, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgAioHandle> as c::expression::OrdinaryField<Field_op>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).op) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgAioHandle, flags) == 3);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgAioHandle>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 3;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgAioHandle, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgAioHandle> as c::expression::OrdinaryField<Field_flags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).flags) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_num_callbacks;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgAioHandle, num_callbacks) == 4);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_callbacks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgAioHandle>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgAioHandle, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgAioHandle> as c::expression::OrdinaryField<Field_num_callbacks>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_callbacks) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_handle_data_len;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgAioHandle, handle_data_len) == 13);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_handle_data_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgAioHandle>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 13;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgAioHandle, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgAioHandle> as c::expression::OrdinaryField<Field_handle_data_len>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).handle_data_len) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_owner_procno;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgAioHandle, owner_procno) == 16);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_owner_procno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgAioHandle>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgAioHandle, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgAioHandle> as c::expression::OrdinaryField<Field_owner_procno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).owner_procno) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgAioHandle, result) == 20);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_result>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgAioHandle>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgAioHandle, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgAioHandle> as c::expression::OrdinaryField<Field_result>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).result) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgAioHandle, generation) == 64);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_generation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgAioHandle>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgAioHandle, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgAioHandle> as c::expression::OrdinaryField<Field_generation>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).generation) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_iovec_off;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgAioHandle, iovec_off) == 92);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_iovec_off>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgAioHandle>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 92;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgAioHandle, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgAioHandle> as c::expression::OrdinaryField<Field_iovec_off>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).iovec_off) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgAioOpData__bindgen_ty_1>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgAioOpData__bindgen_ty_1>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgAioOpData__bindgen_ty_1 {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgAioOpData__bindgen_ty_1, fd) == 0);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fd>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgAioOpData__bindgen_ty_1>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgAioOpData__bindgen_ty_1, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgAioOpData__bindgen_ty_1> as c::expression::OrdinaryField<Field_fd>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fd) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_iov_length;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgAioOpData__bindgen_ty_1, iov_length) == 4
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_iov_length>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgAioOpData__bindgen_ty_1>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgAioOpData__bindgen_ty_1, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgAioOpData__bindgen_ty_1> as c::expression::OrdinaryField<Field_iov_length>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).iov_length) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgAioOpData__bindgen_ty_1, offset) == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_offset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgAioOpData__bindgen_ty_1>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgAioOpData__bindgen_ty_1, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgAioOpData__bindgen_ty_1> as c::expression::OrdinaryField<Field_offset>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).offset) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgAioOpData__bindgen_ty_2>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgAioOpData__bindgen_ty_2>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgAioOpData__bindgen_ty_2 {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgAioOpData__bindgen_ty_2, fd) == 0);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fd>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgAioOpData__bindgen_ty_2>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgAioOpData__bindgen_ty_2, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgAioOpData__bindgen_ty_2> as c::expression::OrdinaryField<Field_fd>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fd) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgAioOpData__bindgen_ty_2, iov_length) == 4
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_iov_length>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgAioOpData__bindgen_ty_2>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgAioOpData__bindgen_ty_2, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgAioOpData__bindgen_ty_2> as c::expression::OrdinaryField<Field_iov_length>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).iov_length) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgAioOpData__bindgen_ty_2, offset) == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_offset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgAioOpData__bindgen_ty_2>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgAioOpData__bindgen_ty_2, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgAioOpData__bindgen_ty_2> as c::expression::OrdinaryField<Field_offset>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).offset) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgAioResult>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgAioResult>() == 4);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgAioResult {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgAioResult, _bitfield_1) * 8 + 0 == 0
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
unsafe extern "C" {
    fn __pgrx_bitfield__get__PgAioResult__id(
        p: *const crate::__pgrx_c_bindings::PgAioResult,
    ) -> u32;
    fn __pgrx_bitfield__set__PgAioResult__id(
        p: *mut crate::__pgrx_c_bindings::PgAioResult,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_volatile__PgAioResult__id(
        p: *const crate::__pgrx_c_bindings::PgAioResult,
    ) -> u32;
    fn __pgrx_bitfield__set_volatile__PgAioResult__id(
        p: *mut crate::__pgrx_c_bindings::PgAioResult,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned__PgAioResult__id(
        p: *const crate::__pgrx_c_bindings::PgAioResult,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned__PgAioResult__id(
        p: *mut crate::__pgrx_c_bindings::PgAioResult,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned_volatile__PgAioResult__id(
        p: *const crate::__pgrx_c_bindings::PgAioResult,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned_volatile__PgAioResult__id(
        p: *mut crate::__pgrx_c_bindings::PgAioResult,
        value: u32,
    ) -> u32;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Bitfield_PgAioResult__id<Q: c::expression::Qualifier> {
    address: Q::Raw<crate::__pgrx_c_bindings::PgAioResult>,
    access: c::expression::Access,
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl<Q: c::expression::Qualifier> Copy for Bitfield_PgAioResult__id<Q> {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl<Q: c::expression::Qualifier> Clone for Bitfield_PgAioResult__id<Q> {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Clang and bindgen layout witnesses agree. This projects the record address without a reference or record read.
unsafe impl<Q: c::expression::Qualifier> c::expression::Field<Field_id, Q>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgAioResult>
{
    type Output = Bitfield_PgAioResult__id<Q>;
    unsafe fn project(base: c::expression::Place<Self, Q>) -> Self::Output {
        Bitfield_PgAioResult__id::<Q> {
            address: <Q as c::expression::Qualifier>::from_mut(base.pointer().as_mut_address()),
            access: c::expression::Access {
                volatile: base.access().volatile || false,
                unaligned: base.access().unaligned,
            },
        }
    }
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl<Q: c::expression::Qualifier> c::expression::VolatilePlace for Bitfield_PgAioResult__id<Q> {
    type Qualified = Self;
    fn qualify_volatile(mut self) -> Self::Qualified {
        self.access.volatile = true;
        self
    }
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: The same-profile C compiler owns bitfield storage reads, including partially initialized neighbor/padding bits. The marker retains declared size and compiler-proved arithmetic promotion.
unsafe impl<Q: c::expression::Qualifier> c::expression::ReadPlace for Bitfield_PgAioResult__id<Q> {
    type Object = c::CUnsignedInt;
    type Type = c::expression::CBitfield<c::CUnsignedInt, c::CInt>;
    unsafe fn load(self) -> <Self::Type as c::expression::CType>::Value {
        let pointer = Q::into_mut(self.address).cast_const(); // SAFETY: The caller establishes the original C field load contract; helpers perform only that field access.
        let storage = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__get_unaligned_volatile__PgAioResult__id(pointer)
                } else {
                    __pgrx_bitfield__get_unaligned__PgAioResult__id(pointer)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__get_volatile__PgAioResult__id(pointer)
            } else {
                __pgrx_bitfield__get__PgAioResult__id(pointer)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(storage)
    }
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Write capability retains ReadWrite qualification. Original C performs declared-base conversion and width narrowing without Rust byte reads.
unsafe impl c::expression::WritePlace for Bitfield_PgAioResult__id<c::expression::ReadWrite> {
    type Assignment = c::CUnsignedInt;
    unsafe fn store(
        self,
        value: <Self::Assignment as c::expression::CType>::Value,
    ) -> <Self::Type as c::expression::CType>::Value {
        let value = <Self::Assignment as c::expression::CType>::into_storage(value); // SAFETY: The caller establishes writable field storage; this helper evaluates one original-C assignment and returns its narrowed result.
        let stored = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__set_unaligned_volatile__PgAioResult__id(self.address, value)
                } else {
                    __pgrx_bitfield__set_unaligned__PgAioResult__id(self.address, value)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__set_volatile__PgAioResult__id(self.address, value)
            } else {
                __pgrx_bitfield__set__PgAioResult__id(self.address, value)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(stored)
    }
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgAioResult, _bitfield_1) * 8 + 6 == 6
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
unsafe extern "C" {
    fn __pgrx_bitfield__get__PgAioResult__status(
        p: *const crate::__pgrx_c_bindings::PgAioResult,
    ) -> u32;
    fn __pgrx_bitfield__set__PgAioResult__status(
        p: *mut crate::__pgrx_c_bindings::PgAioResult,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_volatile__PgAioResult__status(
        p: *const crate::__pgrx_c_bindings::PgAioResult,
    ) -> u32;
    fn __pgrx_bitfield__set_volatile__PgAioResult__status(
        p: *mut crate::__pgrx_c_bindings::PgAioResult,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned__PgAioResult__status(
        p: *const crate::__pgrx_c_bindings::PgAioResult,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned__PgAioResult__status(
        p: *mut crate::__pgrx_c_bindings::PgAioResult,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned_volatile__PgAioResult__status(
        p: *const crate::__pgrx_c_bindings::PgAioResult,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned_volatile__PgAioResult__status(
        p: *mut crate::__pgrx_c_bindings::PgAioResult,
        value: u32,
    ) -> u32;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Bitfield_PgAioResult__status<Q: c::expression::Qualifier> {
    address: Q::Raw<crate::__pgrx_c_bindings::PgAioResult>,
    access: c::expression::Access,
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl<Q: c::expression::Qualifier> Copy for Bitfield_PgAioResult__status<Q> {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl<Q: c::expression::Qualifier> Clone for Bitfield_PgAioResult__status<Q> {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Clang and bindgen layout witnesses agree. This projects the record address without a reference or record read.
unsafe impl<Q: c::expression::Qualifier> c::expression::Field<Field_status, Q>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgAioResult>
{
    type Output = Bitfield_PgAioResult__status<Q>;
    unsafe fn project(base: c::expression::Place<Self, Q>) -> Self::Output {
        Bitfield_PgAioResult__status::<Q> {
            address: <Q as c::expression::Qualifier>::from_mut(base.pointer().as_mut_address()),
            access: c::expression::Access {
                volatile: base.access().volatile || false,
                unaligned: base.access().unaligned,
            },
        }
    }
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl<Q: c::expression::Qualifier> c::expression::VolatilePlace for Bitfield_PgAioResult__status<Q> {
    type Qualified = Self;
    fn qualify_volatile(mut self) -> Self::Qualified {
        self.access.volatile = true;
        self
    }
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: The same-profile C compiler owns bitfield storage reads, including partially initialized neighbor/padding bits. The marker retains declared size and compiler-proved arithmetic promotion.
unsafe impl<Q: c::expression::Qualifier> c::expression::ReadPlace
    for Bitfield_PgAioResult__status<Q>
{
    type Object = c::CUnsignedInt;
    type Type = c::expression::CBitfield<c::CUnsignedInt, c::CInt>;
    unsafe fn load(self) -> <Self::Type as c::expression::CType>::Value {
        let pointer = Q::into_mut(self.address).cast_const(); // SAFETY: The caller establishes the original C field load contract; helpers perform only that field access.
        let storage = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__get_unaligned_volatile__PgAioResult__status(pointer)
                } else {
                    __pgrx_bitfield__get_unaligned__PgAioResult__status(pointer)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__get_volatile__PgAioResult__status(pointer)
            } else {
                __pgrx_bitfield__get__PgAioResult__status(pointer)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(storage)
    }
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Write capability retains ReadWrite qualification. Original C performs declared-base conversion and width narrowing without Rust byte reads.
unsafe impl c::expression::WritePlace for Bitfield_PgAioResult__status<c::expression::ReadWrite> {
    type Assignment = c::CUnsignedInt;
    unsafe fn store(
        self,
        value: <Self::Assignment as c::expression::CType>::Value,
    ) -> <Self::Type as c::expression::CType>::Value {
        let value = <Self::Assignment as c::expression::CType>::into_storage(value); // SAFETY: The caller establishes writable field storage; this helper evaluates one original-C assignment and returns its narrowed result.
        let stored = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__set_unaligned_volatile__PgAioResult__status(
                        self.address,
                        value,
                    )
                } else {
                    __pgrx_bitfield__set_unaligned__PgAioResult__status(self.address, value)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__set_volatile__PgAioResult__status(self.address, value)
            } else {
                __pgrx_bitfield__set__PgAioResult__status(self.address, value)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(stored)
    }
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_error_data;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgAioResult, _bitfield_1) * 8 + 9 == 9
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
unsafe extern "C" {
    fn __pgrx_bitfield__get__PgAioResult__error_data(
        p: *const crate::__pgrx_c_bindings::PgAioResult,
    ) -> u32;
    fn __pgrx_bitfield__set__PgAioResult__error_data(
        p: *mut crate::__pgrx_c_bindings::PgAioResult,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_volatile__PgAioResult__error_data(
        p: *const crate::__pgrx_c_bindings::PgAioResult,
    ) -> u32;
    fn __pgrx_bitfield__set_volatile__PgAioResult__error_data(
        p: *mut crate::__pgrx_c_bindings::PgAioResult,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned__PgAioResult__error_data(
        p: *const crate::__pgrx_c_bindings::PgAioResult,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned__PgAioResult__error_data(
        p: *mut crate::__pgrx_c_bindings::PgAioResult,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned_volatile__PgAioResult__error_data(
        p: *const crate::__pgrx_c_bindings::PgAioResult,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned_volatile__PgAioResult__error_data(
        p: *mut crate::__pgrx_c_bindings::PgAioResult,
        value: u32,
    ) -> u32;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Bitfield_PgAioResult__error_data<Q: c::expression::Qualifier> {
    address: Q::Raw<crate::__pgrx_c_bindings::PgAioResult>,
    access: c::expression::Access,
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl<Q: c::expression::Qualifier> Copy for Bitfield_PgAioResult__error_data<Q> {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl<Q: c::expression::Qualifier> Clone for Bitfield_PgAioResult__error_data<Q> {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Clang and bindgen layout witnesses agree. This projects the record address without a reference or record read.
unsafe impl<Q: c::expression::Qualifier> c::expression::Field<Field_error_data, Q>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgAioResult>
{
    type Output = Bitfield_PgAioResult__error_data<Q>;
    unsafe fn project(base: c::expression::Place<Self, Q>) -> Self::Output {
        Bitfield_PgAioResult__error_data::<Q> {
            address: <Q as c::expression::Qualifier>::from_mut(base.pointer().as_mut_address()),
            access: c::expression::Access {
                volatile: base.access().volatile || false,
                unaligned: base.access().unaligned,
            },
        }
    }
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl<Q: c::expression::Qualifier> c::expression::VolatilePlace
    for Bitfield_PgAioResult__error_data<Q>
{
    type Qualified = Self;
    fn qualify_volatile(mut self) -> Self::Qualified {
        self.access.volatile = true;
        self
    }
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: The same-profile C compiler owns bitfield storage reads, including partially initialized neighbor/padding bits. The marker retains declared size and compiler-proved arithmetic promotion.
unsafe impl<Q: c::expression::Qualifier> c::expression::ReadPlace
    for Bitfield_PgAioResult__error_data<Q>
{
    type Object = c::CUnsignedInt;
    type Type = c::expression::CBitfield<c::CUnsignedInt, c::CInt>;
    unsafe fn load(self) -> <Self::Type as c::expression::CType>::Value {
        let pointer = Q::into_mut(self.address).cast_const(); // SAFETY: The caller establishes the original C field load contract; helpers perform only that field access.
        let storage = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__get_unaligned_volatile__PgAioResult__error_data(pointer)
                } else {
                    __pgrx_bitfield__get_unaligned__PgAioResult__error_data(pointer)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__get_volatile__PgAioResult__error_data(pointer)
            } else {
                __pgrx_bitfield__get__PgAioResult__error_data(pointer)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(storage)
    }
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Write capability retains ReadWrite qualification. Original C performs declared-base conversion and width narrowing without Rust byte reads.
unsafe impl c::expression::WritePlace
    for Bitfield_PgAioResult__error_data<c::expression::ReadWrite>
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
                    __pgrx_bitfield__set_unaligned_volatile__PgAioResult__error_data(
                        self.address,
                        value,
                    )
                } else {
                    __pgrx_bitfield__set_unaligned__PgAioResult__error_data(self.address, value)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__set_volatile__PgAioResult__error_data(self.address, value)
            } else {
                __pgrx_bitfield__set__PgAioResult__error_data(self.address, value)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(stored)
    }
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgAioResult, result) == 4);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_result>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgAioResult>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgAioResult, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgAioResult> as c::expression::OrdinaryField<Field_result>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).result) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::PgAioTargetData__bindgen_ty_1>() == 24
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::PgAioTargetData__bindgen_ty_1>() == 4
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgAioTargetData__bindgen_ty_1 {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgAioTargetData__bindgen_ty_1, blockNum)
        == 12
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_blockNum>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgAioTargetData__bindgen_ty_1>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgAioTargetData__bindgen_ty_1, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgAioTargetData__bindgen_ty_1> as c::expression::OrdinaryField<Field_blockNum>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).blockNum) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgAioTargetData__bindgen_ty_1, nblocks) == 16
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nblocks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgAioTargetData__bindgen_ty_1>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgAioTargetData__bindgen_ty_1, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgAioTargetData__bindgen_ty_1> as c::expression::OrdinaryField<Field_nblocks>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nblocks) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_aio_index;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgAioWaitRef>() == 12);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgAioWaitRef>() == 4);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgAioWaitRef {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgAioWaitRef, aio_index) == 0);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aio_index>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgAioWaitRef>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgAioWaitRef, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgAioWaitRef> as c::expression::OrdinaryField<Field_aio_index>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).aio_index) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_generation_upper;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgAioWaitRef, generation_upper) == 4);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_generation_upper>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgAioWaitRef>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgAioWaitRef, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgAioWaitRef> as c::expression::OrdinaryField<Field_generation_upper>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).generation_upper) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_generation_lower;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgAioWaitRef, generation_lower) == 8);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_generation_lower>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgAioWaitRef>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgAioWaitRef, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgAioWaitRef> as c::expression::OrdinaryField<Field_generation_lower>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).generation_lower) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgBackendStatus>() == 440);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgBackendStatus>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_st_query_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgBackendStatus>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 424;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_st_plan_id;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgBackendStatus, st_plan_id) == 432);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_st_plan_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgBackendStatus>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 432;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgBackendStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgBackendStatus> as c::expression::OrdinaryField<Field_st_plan_id>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).st_plan_id) };
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_Backend>() == 2920);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_Backend>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgStat_Backend {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_Backend, stat_reset_timestamp) == 0
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stat_reset_timestamp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_Backend>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_Backend, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_Backend> as c::expression::OrdinaryField<Field_stat_reset_timestamp>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stat_reset_timestamp) };
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_BackendSubEntry>() == 72);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_BackendSubEntry>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_CheckpointerStats>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_CheckpointerStats>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_num_performed;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_CheckpointerStats, num_performed)
        == 16
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_performed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_CheckpointerStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats> as c::expression::OrdinaryField<Field_num_performed>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_performed) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::PgStat_CheckpointerStats,
        restartpoints_timed
    ) == 24
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_restartpoints_timed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::PgStat_CheckpointerStats,
        restartpoints_requested
    ) == 32
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_restartpoints_requested>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::PgStat_CheckpointerStats,
        restartpoints_performed
    ) == 40
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_restartpoints_performed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_CheckpointerStats, write_time) == 48
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_write_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_CheckpointerStats, sync_time) == 56
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sync_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_CheckpointerStats, buffers_written)
        == 64
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_buffers_written>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_slru_written;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_CheckpointerStats, slru_written) == 72
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_slru_written>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_CheckpointerStats, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats> as c::expression::OrdinaryField<Field_slru_written>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).slru_written) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::PgStat_CheckpointerStats,
        stat_reset_timestamp
    ) == 80
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stat_reset_timestamp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_CheckpointerStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_IO>() == 51848);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_IO>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_StatDBEntry>() == 264);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_StatDBEntry>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_parallel_workers_to_launch;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::PgStat_StatDBEntry,
        parallel_workers_to_launch
    ) == 240
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parallel_workers_to_launch>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 240;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatDBEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry> as c::expression::OrdinaryField<Field_parallel_workers_to_launch>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).parallel_workers_to_launch) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_parallel_workers_launched;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::PgStat_StatDBEntry,
        parallel_workers_launched
    ) == 248
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parallel_workers_launched>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 248;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatDBEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry> as c::expression::OrdinaryField<Field_parallel_workers_launched>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).parallel_workers_launched) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatDBEntry, stat_reset_timestamp)
        == 256
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stat_reset_timestamp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatDBEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 256;
}
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_StatSubEntry>() == 80);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_StatSubEntry>() == 8);
};
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatSubEntry, stat_reset_timestamp)
        == 72
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stat_reset_timestamp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatSubEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_StatTabEntry>() == 216);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_StatTabEntry>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_total_vacuum_time;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatTabEntry, total_vacuum_time)
        == 184
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_total_vacuum_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 184;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatTabEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry> as c::expression::OrdinaryField<Field_total_vacuum_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).total_vacuum_time) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_total_autovacuum_time;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatTabEntry, total_autovacuum_time)
        == 192
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_total_autovacuum_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 192;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatTabEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry> as c::expression::OrdinaryField<Field_total_autovacuum_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).total_autovacuum_time) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_total_analyze_time;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatTabEntry, total_analyze_time)
        == 200
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_total_analyze_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 200;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatTabEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry> as c::expression::OrdinaryField<Field_total_analyze_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).total_analyze_time) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_total_autoanalyze_time;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_StatTabEntry, total_autoanalyze_time)
        == 208
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_total_autoanalyze_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 208;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatTabEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatTabEntry> as c::expression::OrdinaryField<Field_total_autoanalyze_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).total_autoanalyze_time) };
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_WalCounters>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_WalCounters>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgStat_WalCounters {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_WalCounters, wal_records) == 0
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wal_records>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_WalCounters>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_WalCounters, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_WalCounters> as c::expression::OrdinaryField<Field_wal_records>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wal_records) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_WalCounters, wal_fpi) == 8);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wal_fpi>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_WalCounters>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_WalCounters, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_WalCounters> as c::expression::OrdinaryField<Field_wal_fpi>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wal_fpi) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_WalCounters, wal_bytes) == 16);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wal_bytes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_WalCounters>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_WalCounters, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_WalCounters> as c::expression::OrdinaryField<Field_wal_bytes>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wal_bytes) };
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_WalCounters, wal_buffers_full) == 24
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wal_buffers_full>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_WalCounters>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_WalCounters, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_WalCounters> as c::expression::OrdinaryField<Field_wal_buffers_full>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wal_buffers_full) };
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_WalStats>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_WalStats>() == 8);
};
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PgStat_WalStats, stat_reset_timestamp) == 32
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stat_reset_timestamp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_WalStats>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Plan, disabled_nodes) == 4);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_disabled_nodes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Plan>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Plan,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Plan> as c::expression::OrdinaryField<
        Field_disabled_nodes,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).disabled_nodes) };
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PlannedStmt>() == 160);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PlannedStmt>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_queryId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_planId;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, planId) == 16);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_planId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannedStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt> as c::expression::OrdinaryField<Field_planId>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).planId) };
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, hasReturning) == 24);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasReturning>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, hasModifyingCTE) == 25);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasModifyingCTE>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 25;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, canSetTag) == 26);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_canSetTag>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 26;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, transientPlan) == 27);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_transientPlan>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 27;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, dependsOnRole) == 28);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dependsOnRole>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, parallelModeNeeded) == 29
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parallelModeNeeded>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 29;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, jitFlags) == 32);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_jitFlags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_partPruneInfos;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, partPruneInfos) == 48);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_partPruneInfos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannedStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt> as c::expression::OrdinaryField<Field_partPruneInfos>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).partPruneInfos) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, rtable) == 56);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rtable>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, permInfos) == 72);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_permInfos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, resultRelations) == 80);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resultRelations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, appendRelations) == 88);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_appendRelations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, subplans) == 96);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_subplans>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, rowMarks) == 112);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rowMarks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, relationOids) == 120);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relationOids>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, invalItems) == 128);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_invalItems>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, paramExecTypes) == 136);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_paramExecTypes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, stmt_location) == 152);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmt_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannedStmt, stmt_len) == 156);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmt_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannedStmt>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 156;
}
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PlannerGlobal>() == 168);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PlannerGlobal>() == 8);
};
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, finalrteperminfos) == 72
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_finalrteperminfos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, finalrowmarks) == 80);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_finalrowmarks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, resultRelations) == 88
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resultRelations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, appendRelations) == 96
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_appendRelations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, partPruneInfos) == 104
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_partPruneInfos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerGlobal, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal> as c::expression::OrdinaryField<Field_partPruneInfos>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).partPruneInfos) };
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, relationOids) == 112);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relationOids>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, invalItems) == 120);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_invalItems>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, paramExecTypes) == 128
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_paramExecTypes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, lastPHId) == 136);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lastPHId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, lastRowMarkId) == 140);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lastRowMarkId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 140;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, lastPlanNodeId) == 144
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lastPlanNodeId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, transientPlan) == 148);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_transientPlan>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 148;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, dependsOnRole) == 149);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dependsOnRole>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 149;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, parallelModeOK) == 150
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parallelModeOK>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 150;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, parallelModeNeeded) == 151
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parallelModeNeeded>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 151;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerGlobal, maxParallelHazard) == 152
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_maxParallelHazard>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerGlobal>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PlannerInfo>() == 704);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PlannerInfo>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_group_rtindex;
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, group_rtindex) == 604);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_group_rtindex>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 604;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_group_rtindex>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).group_rtindex) };
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PlannerInfo, partPruneInfos) == 696);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_partPruneInfos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 696;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PlannerInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PlannerInfo> as c::expression::OrdinaryField<Field_partPruneInfos>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).partPruneInfos) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PortalData>() == 224);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PortalData>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PortalData, status) == 128);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_status>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PortalData>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PortalStatus,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PortalData, portalPinned) == 132);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_portalPinned>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PortalData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 132;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PortalData, autoHeld) == 133);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_autoHeld>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PortalData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 133;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PortalData, atStart) == 192);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_atStart>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PortalData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 192;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PortalData, atEnd) == 193);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_atEnd>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PortalData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 193;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PortalData, portalPos) == 200);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_portalPos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PortalData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 200;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PortalData, creation_time) == 208);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_creation_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PortalData>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 208;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PortalData, visible) == 216);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_visible>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PortalData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 216;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ProjectionPath>() == 96);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ProjectionPath>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ProjectionPath, dummypp) == 88);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dummypp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ProjectionPath>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Publication>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Publication>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_pubgencols_type;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Publication, pubgencols_type) == 20);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pubgencols_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Publication>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PublishGencolsType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Publication, <<c::expression::CRecord<crate::__pgrx_c_bindings::Publication> as c::expression::OrdinaryField<Field_pubgencols_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pubgencols_type) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PublicationDesc>() == 10);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PublicationDesc>() == 1);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_gencols_valid_for_update;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PublicationDesc, gencols_valid_for_update)
        == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_gencols_valid_for_update>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PublicationDesc>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PublicationDesc, <<c::expression::CRecord<crate::__pgrx_c_bindings::PublicationDesc> as c::expression::OrdinaryField<Field_gencols_valid_for_update>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).gencols_valid_for_update) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_gencols_valid_for_delete;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PublicationDesc, gencols_valid_for_delete)
        == 9
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_gencols_valid_for_delete>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PublicationDesc>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 9;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PublicationDesc, <<c::expression::CRecord<crate::__pgrx_c_bindings::PublicationDesc> as c::expression::OrdinaryField<Field_gencols_valid_for_delete>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).gencols_valid_for_delete) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Query>() == 280);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Query>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_queryId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_hasGroupRTE;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, hasGroupRTE) == 53);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasGroupRTE>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 53;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Query,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Query> as c::expression::OrdinaryField<
        Field_hasGroupRTE,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hasGroupRTE) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, isReturn) == 54);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isReturn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 54;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, returningList) == 152);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_returningList>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, groupClause) == 160);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_groupClause>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, groupDistinct) == 168);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_groupDistinct>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 168;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, groupingSets) == 176);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_groupingSets>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 176;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, windowClause) == 192);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_windowClause>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 192;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, distinctClause) == 200);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_distinctClause>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 200;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, sortClause) == 208);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sortClause>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 208;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, limitOption) == 232);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_limitOption>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__LimitOption,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 232;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, rowMarks) == 240);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rowMarks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 240;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, constraintDeps) == 256);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_constraintDeps>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 256;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, withCheckOptions) == 264);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_withCheckOptions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 264;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, stmt_location) == 272);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmt_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 272;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, stmt_len) == 276);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmt_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 276;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RangeTblEntry>() == 224);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RangeTblEntry>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_groupexprs;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, groupexprs) == 200);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_groupexprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 200;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeTblEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry> as c::expression::OrdinaryField<Field_groupexprs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).groupexprs) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, lateral) == 208);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lateral>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 208;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, inFromCl) == 209);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inFromCl>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 209;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, securityQuals) == 216);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_securityQuals>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 216;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RawColumnDefault, generated) == 16);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_generated>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RawColumnDefault>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_persistence;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ReadBuffersOperation>() == 96);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ReadBuffersOperation>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReadBuffersOperation, persistence) == 16
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_persistence>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReadBuffersOperation>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReadBuffersOperation, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReadBuffersOperation> as c::expression::OrdinaryField<Field_persistence>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).persistence) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_nblocks_done;
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReadBuffersOperation, nblocks_done) == 50
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nblocks_done>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReadBuffersOperation>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 50;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReadBuffersOperation, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReadBuffersOperation> as c::expression::OrdinaryField<Field_nblocks_done>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nblocks_done) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RecursiveUnionPath>() == 120);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RecursiveUnionPath>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RecursiveUnionPath, distinctList) == 96
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_distinctList>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RecursiveUnionPath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RecursiveUnionPath, wtParam) == 104);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wtParam>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RecursiveUnionPath>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RegisteredBgWorker, rw_pid) == 1472);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rw_pid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RegisteredBgWorker>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1472;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RegisteredBgWorker, rw_crashed_at) == 1480
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rw_crashed_at>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RegisteredBgWorker>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1480;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RegisteredBgWorker, rw_shmem_slot) == 1488
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rw_shmem_slot>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RegisteredBgWorker>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1488;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RegisteredBgWorker, rw_terminate) == 1492
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rw_terminate>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RegisteredBgWorker>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1492;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ReorderBufferTXN>() == 336);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ReorderBufferTXN>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBufferTXN, ninvalidations_distributed)
        == 240
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ninvalidations_distributed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 240;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBufferTXN, size) == 312);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 312;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBufferTXN, total_size) == 320);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_total_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 320;
}
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ReplicationSlot>() == 288);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ReplicationSlot>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_last_saved_restart_lsn;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReplicationSlot, last_saved_restart_lsn)
        == 280
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_last_saved_restart_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlot>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 280;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReplicationSlot, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlot> as c::expression::OrdinaryField<Field_last_saved_restart_lsn>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).last_saved_restart_lsn) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_left_relids;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RestrictInfo, left_relids) == 72);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_left_relids>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::Bitmapset>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RestrictInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo> as c::expression::OrdinaryField<Field_left_relids>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).left_relids) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_right_relids;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RestrictInfo, right_relids) == 80);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_right_relids>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::Bitmapset>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RestrictInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RestrictInfo> as c::expression::OrdinaryField<Field_right_relids>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).right_relids) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ResultRelInfo>() == 400);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ResultRelInfo>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_ri_extraUpdatedCols_valid;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_extraUpdatedCols_valid)
        == 56
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_extraUpdatedCols_valid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ResultRelInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo> as c::expression::OrdinaryField<Field_ri_extraUpdatedCols_valid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ri_extraUpdatedCols_valid) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_projectNewInfoValid) == 88
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_projectNewInfoValid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_needLockTagTuple) == 89
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_needLockTagTuple>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 89;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_usesFdwDirectModify) == 176
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_usesFdwDirectModify>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 176;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_NumSlots) == 180);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_NumSlots>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 180;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_NumSlotsInitialized) == 184
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_NumSlotsInitialized>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 184;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_BatchSize) == 188);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_BatchSize>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 188;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_WithCheckOptions) == 208
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_WithCheckOptions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 208;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_WithCheckOptionExprs)
        == 216
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_WithCheckOptionExprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 216;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_NumGeneratedNeededI) == 256
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_NumGeneratedNeededI>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 256;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_NumGeneratedNeededU) == 260
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_NumGeneratedNeededU>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 260;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_returningList) == 264
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_returningList>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 264;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_onConflictArbiterIndexes)
        == 280
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_onConflictArbiterIndexes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 280;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_ChildToRootMapValid) == 344
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_ChildToRootMapValid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 344;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_RootToChildMapValid) == 360
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_RootToChildMapValid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 360;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ResultRelInfo, ri_ancestorResultRels) == 392
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ri_ancestorResultRels>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ResultRelInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 392;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ReturningClause>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ReturningClause>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ReturningClause {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReturningClause, type_) == 0);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReturningClause>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReturningClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReturningClause> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReturningClause, options) == 8);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReturningClause>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReturningClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReturningClause> as c::expression::OrdinaryField<Field_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReturningClause, exprs) == 16);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_exprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReturningClause>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReturningClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReturningClause> as c::expression::OrdinaryField<Field_exprs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).exprs) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_retlevelsup;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ReturningExpr>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ReturningExpr>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ReturningExpr {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReturningExpr, retlevelsup) == 4);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_retlevelsup>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReturningExpr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReturningExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReturningExpr> as c::expression::OrdinaryField<Field_retlevelsup>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).retlevelsup) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_retold;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReturningExpr, retold) == 8);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_retold>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReturningExpr>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReturningExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReturningExpr> as c::expression::OrdinaryField<Field_retold>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).retold) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ReturningOption>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ReturningOption>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ReturningOption {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReturningOption, type_) == 0);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReturningOption>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReturningOption, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReturningOption> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_option;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReturningOption, option) == 4);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_option>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReturningOption>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ReturningOptionKind,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReturningOption, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReturningOption> as c::expression::OrdinaryField<Field_option>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).option) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReturningOption, value) == 8);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_value>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReturningOption>
{
    type Member = c::expression::CPointer<
        c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReturningOption, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReturningOption> as c::expression::OrdinaryField<Field_value>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).value) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReturningOption, location) == 16);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReturningOption>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReturningOption, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReturningOption> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RowCompareExpr, cmptype) == 4);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cmptype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RowCompareExpr>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__CompareType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RowCompareExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::RowCompareExpr> as c::expression::OrdinaryField<Field_cmptype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cmptype) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_bytesSent;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SerializeMetrics>() == 144);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SerializeMetrics>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SerializeMetrics {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SerializeMetrics, bytesSent) == 0);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bytesSent>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SerializeMetrics>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SerializeMetrics, <<c::expression::CRecord<crate::__pgrx_c_bindings::SerializeMetrics> as c::expression::OrdinaryField<Field_bytesSent>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).bytesSent) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SetOpPath>() == 120);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SetOpPath>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetOpPath, cmd) == 96);
#[cfg(any(feature = "pg18", feature = "pg19"))]
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
    const OFFSET: usize = 96;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetOpPath, strategy) == 100);
#[cfg(any(feature = "pg18", feature = "pg19"))]
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
    const OFFSET: usize = 100;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_groupList;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetOpPath, groupList) == 104);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_groupList>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetOpPath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SetOpPath,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SetOpPath> as c::expression::OrdinaryField<
        Field_groupList,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).groupList) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SetOpState>() == 352);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SetOpState>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetOpState, setop_done) == 200);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_setop_done>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetOpState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 200;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetOpState, numOutput) == 208);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numOutput>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetOpState>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 208;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetOpState, numCols) == 216);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numCols>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetOpState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 216;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SetOpState, <<c::expression::CRecord<crate::__pgrx_c_bindings::SetOpState> as c::expression::OrdinaryField<Field_numCols>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numCols) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_need_init;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetOpState, need_init) == 296);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_need_init>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetOpState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 296;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SetOpState, <<c::expression::CRecord<crate::__pgrx_c_bindings::SetOpState> as c::expression::OrdinaryField<Field_need_init>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).need_init) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetOpState, table_filled) == 336);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_table_filled>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetOpState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 336;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_numTuples;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SetOpStatePerInput>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SetOpStatePerInput>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SetOpStatePerInput {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetOpStatePerInput, numTuples) == 8);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numTuples>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetOpStatePerInput>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SetOpStatePerInput, <<c::expression::CRecord<crate::__pgrx_c_bindings::SetOpStatePerInput> as c::expression::OrdinaryField<Field_numTuples>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numTuples) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_needGroup;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SetOpStatePerInput, needGroup) == 24);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_needGroup>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SetOpStatePerInput>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SetOpStatePerInput, <<c::expression::CRecord<crate::__pgrx_c_bindings::SetOpStatePerInput> as c::expression::OrdinaryField<Field_needGroup>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).needGroup) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::SharedBitmapHeapInstrumentation>() == 8
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::SharedBitmapHeapInstrumentation>() == 8
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SharedBitmapHeapInstrumentation {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SharedBitmapHeapInstrumentation, num_workers)
        == 0
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_workers>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SharedBitmapHeapInstrumentation>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SharedBitmapHeapInstrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::SharedBitmapHeapInstrumentation> as c::expression::OrdinaryField<Field_num_workers>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_workers) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::SharedIndexScanInstrumentation>() == 8
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::SharedIndexScanInstrumentation>() == 8
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SharedIndexScanInstrumentation {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SharedIndexScanInstrumentation, num_workers)
        == 0
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_workers>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SharedIndexScanInstrumentation>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SharedIndexScanInstrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::SharedIndexScanInstrumentation> as c::expression::OrdinaryField<Field_num_workers>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_workers) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_low_elem;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SkipSupportData>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SkipSupportData>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SkipSupportData {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SkipSupportData, low_elem) == 0);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_low_elem>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SkipSupportData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, crate::__pgrx_c_bindings::Datum>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SkipSupportData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SkipSupportData> as c::expression::OrdinaryField<Field_low_elem>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).low_elem) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_high_elem;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SkipSupportData, high_elem) == 8);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_high_elem>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SkipSupportData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, crate::__pgrx_c_bindings::Datum>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SkipSupportData, <<c::expression::CRecord<crate::__pgrx_c_bindings::SkipSupportData> as c::expression::OrdinaryField<Field_high_elem>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).high_elem) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SnapBuild>() == 128);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SnapBuild>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SnapBuild {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapBuild, state) == 0);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_state>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_SnapBuildState,
        c::CInt,
        i32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SnapBuild,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild> as c::expression::OrdinaryField<
        Field_state,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).state) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapBuild, context) == 8);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_context>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::MemoryContextData>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SnapBuild,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild> as c::expression::OrdinaryField<
        Field_context,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).context) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapBuild, xmin) == 16);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xmin>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SnapBuild,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild> as c::expression::OrdinaryField<
        Field_xmin,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xmin) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapBuild, xmax) == 20);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xmax>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SnapBuild,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild> as c::expression::OrdinaryField<
        Field_xmax,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xmax) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_start_decoding_at;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapBuild, start_decoding_at) == 24);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_start_decoding_at>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SnapBuild,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild> as c::expression::OrdinaryField<
        Field_start_decoding_at,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).start_decoding_at) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapBuild, two_phase_at) == 32);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_two_phase_at>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SnapBuild,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild> as c::expression::OrdinaryField<
        Field_two_phase_at,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).two_phase_at) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_initial_xmin_horizon;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapBuild, initial_xmin_horizon) == 40
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_initial_xmin_horizon>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SnapBuild,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild> as c::expression::OrdinaryField<
        Field_initial_xmin_horizon,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).initial_xmin_horizon) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_building_full_snapshot;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapBuild, building_full_snapshot) == 44
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_building_full_snapshot>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SnapBuild,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild> as c::expression::OrdinaryField<
        Field_building_full_snapshot,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).building_full_snapshot) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_in_slot_creation;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapBuild, in_slot_creation) == 45);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_in_slot_creation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 45;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SnapBuild,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild> as c::expression::OrdinaryField<
        Field_in_slot_creation,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).in_slot_creation) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_last_serialized_snapshot;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapBuild, last_serialized_snapshot) == 56
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_last_serialized_snapshot>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SnapBuild,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild> as c::expression::OrdinaryField<
        Field_last_serialized_snapshot,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).last_serialized_snapshot) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_next_phase_at;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapBuild, next_phase_at) == 72);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_next_phase_at>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SnapBuild,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild> as c::expression::OrdinaryField<
        Field_next_phase_at,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).next_phase_at) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SnapBuild__bindgen_ty_1>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SnapBuild__bindgen_ty_1>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SnapBuild__bindgen_ty_1 {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapBuild__bindgen_ty_1, xcnt) == 0);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xcnt>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuild__bindgen_ty_1>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
