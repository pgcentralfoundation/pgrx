// Shared generated C macro support.

#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_deferrable>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 108;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt> as c::expression::OrdinaryField<
        Field_deferrable,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).deferrable) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexStmt, initdeferred) == 109);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_initdeferred>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 109;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt> as c::expression::OrdinaryField<
        Field_initdeferred,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).initdeferred) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexStmt, transformed) == 110);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_transformed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 110;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt> as c::expression::OrdinaryField<
        Field_transformed,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).transformed) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexStmt, concurrent) == 111);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_concurrent>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 111;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt> as c::expression::OrdinaryField<
        Field_concurrent,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).concurrent) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexStmt, if_not_exists) == 112);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_if_not_exists>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt> as c::expression::OrdinaryField<
        Field_if_not_exists,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).if_not_exists) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexStmt, reset_default_tblspc) == 113
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_reset_default_tblspc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 113;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::IndexStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexStmt> as c::expression::OrdinaryField<
        Field_reset_default_tblspc,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).reset_default_tblspc) };
#[doc(hidden)]
pub struct Field_t_tid;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::IndexTupleData>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::IndexTupleData>() == 2);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::IndexTupleData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexTupleData, t_tid) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_t_tid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexTupleData>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::ItemPointerData>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexTupleData, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexTupleData> as c::expression::OrdinaryField<Field_t_tid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).t_tid) };
#[doc(hidden)]
pub struct Field_t_info;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexTupleData, t_info) == 6);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_t_info>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexTupleData>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 6;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexTupleData, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexTupleData> as c::expression::OrdinaryField<Field_t_info>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).t_info) };
#[doc(hidden)]
pub struct Field_analyze_only;
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::IndexVacuumInfo>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::IndexVacuumInfo>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::IndexVacuumInfo {}
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexVacuumInfo, analyze_only) == 8);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_analyze_only>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexVacuumInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexVacuumInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexVacuumInfo> as c::expression::OrdinaryField<Field_analyze_only>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).analyze_only) };
#[doc(hidden)]
pub struct Field_report_progress;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexVacuumInfo, report_progress) == 9
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_report_progress>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexVacuumInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 9;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexVacuumInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexVacuumInfo> as c::expression::OrdinaryField<Field_report_progress>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).report_progress) };
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexVacuumInfo, estimated_count) == 10
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_estimated_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexVacuumInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 10;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexVacuumInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexVacuumInfo> as c::expression::OrdinaryField<Field_estimated_count>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).estimated_count) };
#[doc(hidden)]
pub struct Field_message_level;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::IndexVacuumInfo, message_level) == 12
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_message_level>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IndexVacuumInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IndexVacuumInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::IndexVacuumInfo> as c::expression::OrdinaryField<Field_message_level>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).message_level) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::InferClause>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::InferClause>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::InferClause {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::InferClause, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::InferClause>
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
const _: Projection<crate::__pgrx_c_bindings::InferClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::InferClause> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_indexElems;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::InferClause, indexElems) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indexElems>
    for c::expression::CRecord<crate::__pgrx_c_bindings::InferClause>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::InferClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::InferClause> as c::expression::OrdinaryField<Field_indexElems>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indexElems) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::InferClause, location) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::InferClause>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::InferClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::InferClause> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[doc(hidden)]
pub struct Field_infercollid;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::InferenceElem>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::InferenceElem>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::InferenceElem {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::InferenceElem, infercollid) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_infercollid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::InferenceElem>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::InferenceElem, <<c::expression::CRecord<crate::__pgrx_c_bindings::InferenceElem> as c::expression::OrdinaryField<Field_infercollid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).infercollid) };
#[doc(hidden)]
pub struct Field_inferopclass;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::InferenceElem, inferopclass) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inferopclass>
    for c::expression::CRecord<crate::__pgrx_c_bindings::InferenceElem>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::InferenceElem, <<c::expression::CRecord<crate::__pgrx_c_bindings::InferenceElem> as c::expression::OrdinaryField<Field_inferopclass>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).inferopclass) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::InlineCodeBlock>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::InlineCodeBlock>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::InlineCodeBlock {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::InlineCodeBlock, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::InlineCodeBlock>
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
const _: Projection<crate::__pgrx_c_bindings::InlineCodeBlock, <<c::expression::CRecord<crate::__pgrx_c_bindings::InlineCodeBlock> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_langOid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::InlineCodeBlock, langOid) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_langOid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::InlineCodeBlock>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::InlineCodeBlock, <<c::expression::CRecord<crate::__pgrx_c_bindings::InlineCodeBlock> as c::expression::OrdinaryField<Field_langOid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).langOid) };
#[doc(hidden)]
pub struct Field_langIsTrusted;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::InlineCodeBlock, langIsTrusted) == 20
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_langIsTrusted>
    for c::expression::CRecord<crate::__pgrx_c_bindings::InlineCodeBlock>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::InlineCodeBlock, <<c::expression::CRecord<crate::__pgrx_c_bindings::InlineCodeBlock> as c::expression::OrdinaryField<Field_langIsTrusted>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).langIsTrusted) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::InlineCodeBlock, atomic) == 21);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_atomic>
    for c::expression::CRecord<crate::__pgrx_c_bindings::InlineCodeBlock>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 21;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::InlineCodeBlock, <<c::expression::CRecord<crate::__pgrx_c_bindings::InlineCodeBlock> as c::expression::OrdinaryField<Field_atomic>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).atomic) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::InsertStmt>() == 64);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::InsertStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::InsertStmt {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::InsertStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::InsertStmt>
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
const _: Projection<crate::__pgrx_c_bindings::InsertStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::InsertStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::InsertStmt, cols) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cols>
    for c::expression::CRecord<crate::__pgrx_c_bindings::InsertStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::InsertStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::InsertStmt> as c::expression::OrdinaryField<Field_cols>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cols) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::InsertStmt, returningList) == 40);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_returningList>
    for c::expression::CRecord<crate::__pgrx_c_bindings::InsertStmt>
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
const _: Projection<crate::__pgrx_c_bindings::InsertStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::InsertStmt> as c::expression::OrdinaryField<Field_returningList>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).returningList) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::InsertStmt, override_) == 56);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_override>
    for c::expression::CRecord<crate::__pgrx_c_bindings::InsertStmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__OverridingKind,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::InsertStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::InsertStmt> as c::expression::OrdinaryField<Field_override>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).override_) };
#[doc(hidden)]
pub struct Field_need_timer;
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Instrumentation>() == 448);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Instrumentation>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::Instrumentation {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Instrumentation, need_timer) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_need_timer>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Instrumentation>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Instrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::Instrumentation> as c::expression::OrdinaryField<Field_need_timer>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).need_timer) };
#[doc(hidden)]
pub struct Field_need_bufusage;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Instrumentation, need_bufusage) == 1);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_need_bufusage>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Instrumentation>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Instrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::Instrumentation> as c::expression::OrdinaryField<Field_need_bufusage>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).need_bufusage) };
#[doc(hidden)]
pub struct Field_need_walusage;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Instrumentation, need_walusage) == 2);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_need_walusage>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Instrumentation>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 2;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Instrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::Instrumentation> as c::expression::OrdinaryField<Field_need_walusage>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).need_walusage) };
#[doc(hidden)]
pub struct Field_async_mode;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Instrumentation, async_mode) == 3);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_async_mode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Instrumentation>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 3;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Instrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::Instrumentation> as c::expression::OrdinaryField<Field_async_mode>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).async_mode) };
#[doc(hidden)]
pub struct Field_running;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Instrumentation, running) == 4);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_running>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Instrumentation>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Instrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::Instrumentation> as c::expression::OrdinaryField<Field_running>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).running) };
#[doc(hidden)]
pub struct Field_ntuples2;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Instrumentation, ntuples2) == 248);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ntuples2>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Instrumentation>
{
    type Member = c::expression::CDouble;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 248;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Instrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::Instrumentation> as c::expression::OrdinaryField<Field_ntuples2>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ntuples2) };
#[doc(hidden)]
pub struct Field_nfiltered1;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Instrumentation, nfiltered1) == 264);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nfiltered1>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Instrumentation>
{
    type Member = c::expression::CDouble;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 264;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Instrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::Instrumentation> as c::expression::OrdinaryField<Field_nfiltered1>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nfiltered1) };
#[doc(hidden)]
pub struct Field_nfiltered2;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Instrumentation, nfiltered2) == 272);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nfiltered2>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Instrumentation>
{
    type Member = c::expression::CDouble;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 272;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Instrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::Instrumentation> as c::expression::OrdinaryField<Field_nfiltered2>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nfiltered2) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Integer>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Integer>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::Integer {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Integer, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Integer>
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
    crate::__pgrx_c_bindings::Integer,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Integer> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_ival;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Integer, ival) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ival>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Integer>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Integer,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Integer> as c::expression::OrdinaryField<
        Field_ival,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ival) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::IntoClause>() == 72);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::IntoClause>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::IntoClause {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IntoClause, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IntoClause>
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
const _: Projection<crate::__pgrx_c_bindings::IntoClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::IntoClause> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_colNames;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IntoClause, colNames) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_colNames>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IntoClause>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IntoClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::IntoClause> as c::expression::OrdinaryField<Field_colNames>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).colNames) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IntoClause, options) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IntoClause>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IntoClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::IntoClause> as c::expression::OrdinaryField<Field_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
#[doc(hidden)]
pub struct Field_onCommit;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IntoClause, onCommit) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_onCommit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IntoClause>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__OnCommitAction,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IntoClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::IntoClause> as c::expression::OrdinaryField<Field_onCommit>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).onCommit) };
#[doc(hidden)]
pub struct Field_skipData;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::IntoClause, skipData) == 64);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_skipData>
    for c::expression::CRecord<crate::__pgrx_c_bindings::IntoClause>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::IntoClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::IntoClause> as c::expression::OrdinaryField<Field_skipData>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).skipData) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ItemIdData>() == 4);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ItemIdData>() == 4);
};
#[doc(hidden)]
pub struct Field_lp_off;
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ItemIdData {}
const _: () = {
    assert!(
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::ItemIdData, _bitfield_1) * 8 + 0 == 0
    );
};
unsafe extern "C" {
    fn __pgrx_bitfield__get__ItemIdData__lp_off(
        p: *const crate::__pgrx_c_bindings::ItemIdData,
    ) -> u32;
    fn __pgrx_bitfield__set__ItemIdData__lp_off(
        p: *mut crate::__pgrx_c_bindings::ItemIdData,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_volatile__ItemIdData__lp_off(
        p: *const crate::__pgrx_c_bindings::ItemIdData,
    ) -> u32;
    fn __pgrx_bitfield__set_volatile__ItemIdData__lp_off(
        p: *mut crate::__pgrx_c_bindings::ItemIdData,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned__ItemIdData__lp_off(
        p: *const crate::__pgrx_c_bindings::ItemIdData,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned__ItemIdData__lp_off(
        p: *mut crate::__pgrx_c_bindings::ItemIdData,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned_volatile__ItemIdData__lp_off(
        p: *const crate::__pgrx_c_bindings::ItemIdData,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned_volatile__ItemIdData__lp_off(
        p: *mut crate::__pgrx_c_bindings::ItemIdData,
        value: u32,
    ) -> u32;
}
#[doc(hidden)]
pub struct Bitfield_ItemIdData__lp_off<Q: c::expression::Qualifier> {
    address: Q::Raw<crate::__pgrx_c_bindings::ItemIdData>,
    access: c::expression::Access,
}
impl<Q: c::expression::Qualifier> Copy for Bitfield_ItemIdData__lp_off<Q> {}
impl<Q: c::expression::Qualifier> Clone for Bitfield_ItemIdData__lp_off<Q> {
    fn clone(&self) -> Self {
        *self
    }
}
// SAFETY: Clang and bindgen layout witnesses agree. This projects the record address without a reference or record read.
unsafe impl<Q: c::expression::Qualifier> c::expression::Field<Field_lp_off, Q>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ItemIdData>
{
    type Output = Bitfield_ItemIdData__lp_off<Q>;
    unsafe fn project(base: c::expression::Place<Self, Q>) -> Self::Output {
        Bitfield_ItemIdData__lp_off::<Q> {
            address: <Q as c::expression::Qualifier>::from_mut(base.pointer().as_mut_address()),
            access: c::expression::Access {
                volatile: base.access().volatile || false,
                unaligned: base.access().unaligned,
            },
        }
    }
}
impl<Q: c::expression::Qualifier> c::expression::VolatilePlace for Bitfield_ItemIdData__lp_off<Q> {
    type Qualified = Self;
    fn qualify_volatile(mut self) -> Self::Qualified {
        self.access.volatile = true;
        self
    }
}
// SAFETY: The same-profile C compiler owns bitfield storage reads, including partially initialized neighbor/padding bits. The marker retains declared size and compiler-proved arithmetic promotion.
unsafe impl<Q: c::expression::Qualifier> c::expression::ReadPlace
    for Bitfield_ItemIdData__lp_off<Q>
{
    type Object = c::CUnsignedInt;
    type Type = c::expression::CBitfield<c::CUnsignedInt, c::CInt>;
    unsafe fn load(self) -> <Self::Type as c::expression::CType>::Value {
        let pointer = Q::into_mut(self.address).cast_const(); // SAFETY: The caller establishes the original C field load contract; helpers perform only that field access.
        let storage = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__get_unaligned_volatile__ItemIdData__lp_off(pointer)
                } else {
                    __pgrx_bitfield__get_unaligned__ItemIdData__lp_off(pointer)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__get_volatile__ItemIdData__lp_off(pointer)
            } else {
                __pgrx_bitfield__get__ItemIdData__lp_off(pointer)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(storage)
    }
}
// SAFETY: Write capability retains ReadWrite qualification. Original C performs declared-base conversion and width narrowing without Rust byte reads.
unsafe impl c::expression::WritePlace for Bitfield_ItemIdData__lp_off<c::expression::ReadWrite> {
    type Assignment = c::CUnsignedInt;
    unsafe fn store(
        self,
        value: <Self::Assignment as c::expression::CType>::Value,
    ) -> <Self::Type as c::expression::CType>::Value {
        let value = <Self::Assignment as c::expression::CType>::into_storage(value); // SAFETY: The caller establishes writable field storage; this helper evaluates one original-C assignment and returns its narrowed result.
        let stored = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__set_unaligned_volatile__ItemIdData__lp_off(self.address, value)
                } else {
                    __pgrx_bitfield__set_unaligned__ItemIdData__lp_off(self.address, value)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__set_volatile__ItemIdData__lp_off(self.address, value)
            } else {
                __pgrx_bitfield__set__ItemIdData__lp_off(self.address, value)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(stored)
    }
}
#[doc(hidden)]
pub struct Field_lp_flags;
const _: () = {
    assert!(
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::ItemIdData, _bitfield_1) * 8 + 15 == 15
    );
};
unsafe extern "C" {
    fn __pgrx_bitfield__get__ItemIdData__lp_flags(
        p: *const crate::__pgrx_c_bindings::ItemIdData,
    ) -> u32;
    fn __pgrx_bitfield__set__ItemIdData__lp_flags(
        p: *mut crate::__pgrx_c_bindings::ItemIdData,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_volatile__ItemIdData__lp_flags(
        p: *const crate::__pgrx_c_bindings::ItemIdData,
    ) -> u32;
    fn __pgrx_bitfield__set_volatile__ItemIdData__lp_flags(
        p: *mut crate::__pgrx_c_bindings::ItemIdData,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned__ItemIdData__lp_flags(
        p: *const crate::__pgrx_c_bindings::ItemIdData,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned__ItemIdData__lp_flags(
        p: *mut crate::__pgrx_c_bindings::ItemIdData,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned_volatile__ItemIdData__lp_flags(
        p: *const crate::__pgrx_c_bindings::ItemIdData,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned_volatile__ItemIdData__lp_flags(
        p: *mut crate::__pgrx_c_bindings::ItemIdData,
        value: u32,
    ) -> u32;
}
#[doc(hidden)]
pub struct Bitfield_ItemIdData__lp_flags<Q: c::expression::Qualifier> {
    address: Q::Raw<crate::__pgrx_c_bindings::ItemIdData>,
    access: c::expression::Access,
}
impl<Q: c::expression::Qualifier> Copy for Bitfield_ItemIdData__lp_flags<Q> {}
impl<Q: c::expression::Qualifier> Clone for Bitfield_ItemIdData__lp_flags<Q> {
    fn clone(&self) -> Self {
        *self
    }
}
// SAFETY: Clang and bindgen layout witnesses agree. This projects the record address without a reference or record read.
unsafe impl<Q: c::expression::Qualifier> c::expression::Field<Field_lp_flags, Q>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ItemIdData>
{
    type Output = Bitfield_ItemIdData__lp_flags<Q>;
    unsafe fn project(base: c::expression::Place<Self, Q>) -> Self::Output {
        Bitfield_ItemIdData__lp_flags::<Q> {
            address: <Q as c::expression::Qualifier>::from_mut(base.pointer().as_mut_address()),
            access: c::expression::Access {
                volatile: base.access().volatile || false,
                unaligned: base.access().unaligned,
            },
        }
    }
}
impl<Q: c::expression::Qualifier> c::expression::VolatilePlace
    for Bitfield_ItemIdData__lp_flags<Q>
{
    type Qualified = Self;
    fn qualify_volatile(mut self) -> Self::Qualified {
        self.access.volatile = true;
        self
    }
}
// SAFETY: The same-profile C compiler owns bitfield storage reads, including partially initialized neighbor/padding bits. The marker retains declared size and compiler-proved arithmetic promotion.
unsafe impl<Q: c::expression::Qualifier> c::expression::ReadPlace
    for Bitfield_ItemIdData__lp_flags<Q>
{
    type Object = c::CUnsignedInt;
    type Type = c::expression::CBitfield<c::CUnsignedInt, c::CInt>;
    unsafe fn load(self) -> <Self::Type as c::expression::CType>::Value {
        let pointer = Q::into_mut(self.address).cast_const(); // SAFETY: The caller establishes the original C field load contract; helpers perform only that field access.
        let storage = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__get_unaligned_volatile__ItemIdData__lp_flags(pointer)
                } else {
                    __pgrx_bitfield__get_unaligned__ItemIdData__lp_flags(pointer)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__get_volatile__ItemIdData__lp_flags(pointer)
            } else {
                __pgrx_bitfield__get__ItemIdData__lp_flags(pointer)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(storage)
    }
}
// SAFETY: Write capability retains ReadWrite qualification. Original C performs declared-base conversion and width narrowing without Rust byte reads.
unsafe impl c::expression::WritePlace for Bitfield_ItemIdData__lp_flags<c::expression::ReadWrite> {
    type Assignment = c::CUnsignedInt;
    unsafe fn store(
        self,
        value: <Self::Assignment as c::expression::CType>::Value,
    ) -> <Self::Type as c::expression::CType>::Value {
        let value = <Self::Assignment as c::expression::CType>::into_storage(value); // SAFETY: The caller establishes writable field storage; this helper evaluates one original-C assignment and returns its narrowed result.
        let stored = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__set_unaligned_volatile__ItemIdData__lp_flags(
                        self.address,
                        value,
                    )
                } else {
                    __pgrx_bitfield__set_unaligned__ItemIdData__lp_flags(self.address, value)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__set_volatile__ItemIdData__lp_flags(self.address, value)
            } else {
                __pgrx_bitfield__set__ItemIdData__lp_flags(self.address, value)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(stored)
    }
}
#[doc(hidden)]
pub struct Field_lp_len;
const _: () = {
    assert!(
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::ItemIdData, _bitfield_1) * 8 + 17 == 17
    );
};
unsafe extern "C" {
    fn __pgrx_bitfield__get__ItemIdData__lp_len(
        p: *const crate::__pgrx_c_bindings::ItemIdData,
    ) -> u32;
    fn __pgrx_bitfield__set__ItemIdData__lp_len(
        p: *mut crate::__pgrx_c_bindings::ItemIdData,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_volatile__ItemIdData__lp_len(
        p: *const crate::__pgrx_c_bindings::ItemIdData,
    ) -> u32;
    fn __pgrx_bitfield__set_volatile__ItemIdData__lp_len(
        p: *mut crate::__pgrx_c_bindings::ItemIdData,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned__ItemIdData__lp_len(
        p: *const crate::__pgrx_c_bindings::ItemIdData,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned__ItemIdData__lp_len(
        p: *mut crate::__pgrx_c_bindings::ItemIdData,
        value: u32,
    ) -> u32;
    fn __pgrx_bitfield__get_unaligned_volatile__ItemIdData__lp_len(
        p: *const crate::__pgrx_c_bindings::ItemIdData,
    ) -> u32;
    fn __pgrx_bitfield__set_unaligned_volatile__ItemIdData__lp_len(
        p: *mut crate::__pgrx_c_bindings::ItemIdData,
        value: u32,
    ) -> u32;
}
#[doc(hidden)]
pub struct Bitfield_ItemIdData__lp_len<Q: c::expression::Qualifier> {
    address: Q::Raw<crate::__pgrx_c_bindings::ItemIdData>,
    access: c::expression::Access,
}
impl<Q: c::expression::Qualifier> Copy for Bitfield_ItemIdData__lp_len<Q> {}
impl<Q: c::expression::Qualifier> Clone for Bitfield_ItemIdData__lp_len<Q> {
    fn clone(&self) -> Self {
        *self
    }
}
// SAFETY: Clang and bindgen layout witnesses agree. This projects the record address without a reference or record read.
unsafe impl<Q: c::expression::Qualifier> c::expression::Field<Field_lp_len, Q>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ItemIdData>
{
    type Output = Bitfield_ItemIdData__lp_len<Q>;
    unsafe fn project(base: c::expression::Place<Self, Q>) -> Self::Output {
        Bitfield_ItemIdData__lp_len::<Q> {
            address: <Q as c::expression::Qualifier>::from_mut(base.pointer().as_mut_address()),
            access: c::expression::Access {
                volatile: base.access().volatile || false,
                unaligned: base.access().unaligned,
            },
        }
    }
}
impl<Q: c::expression::Qualifier> c::expression::VolatilePlace for Bitfield_ItemIdData__lp_len<Q> {
    type Qualified = Self;
    fn qualify_volatile(mut self) -> Self::Qualified {
        self.access.volatile = true;
        self
    }
}
// SAFETY: The same-profile C compiler owns bitfield storage reads, including partially initialized neighbor/padding bits. The marker retains declared size and compiler-proved arithmetic promotion.
unsafe impl<Q: c::expression::Qualifier> c::expression::ReadPlace
    for Bitfield_ItemIdData__lp_len<Q>
{
    type Object = c::CUnsignedInt;
    type Type = c::expression::CBitfield<c::CUnsignedInt, c::CInt>;
    unsafe fn load(self) -> <Self::Type as c::expression::CType>::Value {
        let pointer = Q::into_mut(self.address).cast_const(); // SAFETY: The caller establishes the original C field load contract; helpers perform only that field access.
        let storage = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__get_unaligned_volatile__ItemIdData__lp_len(pointer)
                } else {
                    __pgrx_bitfield__get_unaligned__ItemIdData__lp_len(pointer)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__get_volatile__ItemIdData__lp_len(pointer)
            } else {
                __pgrx_bitfield__get__ItemIdData__lp_len(pointer)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(storage)
    }
}
// SAFETY: Write capability retains ReadWrite qualification. Original C performs declared-base conversion and width narrowing without Rust byte reads.
unsafe impl c::expression::WritePlace for Bitfield_ItemIdData__lp_len<c::expression::ReadWrite> {
    type Assignment = c::CUnsignedInt;
    unsafe fn store(
        self,
        value: <Self::Assignment as c::expression::CType>::Value,
    ) -> <Self::Type as c::expression::CType>::Value {
        let value = <Self::Assignment as c::expression::CType>::into_storage(value); // SAFETY: The caller establishes writable field storage; this helper evaluates one original-C assignment and returns its narrowed result.
        let stored = unsafe {
            if self.access.unaligned {
                if self.access.volatile {
                    __pgrx_bitfield__set_unaligned_volatile__ItemIdData__lp_len(self.address, value)
                } else {
                    __pgrx_bitfield__set_unaligned__ItemIdData__lp_len(self.address, value)
                }
            } else if self.access.volatile {
                __pgrx_bitfield__set_volatile__ItemIdData__lp_len(self.address, value)
            } else {
                __pgrx_bitfield__set__ItemIdData__lp_len(self.address, value)
            }
        };
        <Self::Type as c::expression::CType>::from_storage(stored)
    }
}
#[doc(hidden)]
pub struct Field_ip_blkid;
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ItemPointerData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ItemPointerData, ip_blkid) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ip_blkid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ItemPointerData>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::BlockIdData>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ItemPointerData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ItemPointerData> as c::expression::OrdinaryField<Field_ip_blkid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ip_blkid) };
#[doc(hidden)]
pub struct Field_ip_posid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ItemPointerData, ip_posid) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ip_posid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ItemPointerData>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ItemPointerData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ItemPointerData> as c::expression::OrdinaryField<Field_ip_posid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ip_posid) };
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JitContext>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JitContext>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JitContext {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JitContext, flags) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JitContext>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JitContext, <<c::expression::CRecord<crate::__pgrx_c_bindings::JitContext> as c::expression::OrdinaryField<Field_flags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).flags) };
#[doc(hidden)]
pub struct Field_created_functions;
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JitInstrumentation>() == 72);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JitInstrumentation>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JitInstrumentation {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::JitInstrumentation, created_functions) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_created_functions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JitInstrumentation>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JitInstrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::JitInstrumentation> as c::expression::OrdinaryField<Field_created_functions>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).created_functions) };
#[doc(hidden)]
pub struct Field_jointype;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Join>() == 120);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Join>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::Join {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Join, jointype) == 104);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_jointype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Join>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__JoinType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Join,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Join> as c::expression::OrdinaryField<
        Field_jointype,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).jointype) };
#[doc(hidden)]
pub struct Field_inner_unique;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Join, inner_unique) == 108);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inner_unique>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Join>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 108;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Join,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Join> as c::expression::OrdinaryField<
        Field_inner_unique,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).inner_unique) };
#[doc(hidden)]
pub struct Field_joinqual;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Join, joinqual) == 112);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_joinqual>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Join>
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
    crate::__pgrx_c_bindings::Join,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Join> as c::expression::OrdinaryField<
        Field_joinqual,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).joinqual) };
#[doc(hidden)]
pub struct Field_numbuckets;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JoinCostWorkspace>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JoinCostWorkspace>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JoinCostWorkspace {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JoinCostWorkspace, numbuckets) == 72);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numbuckets>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JoinCostWorkspace>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JoinCostWorkspace, <<c::expression::CRecord<crate::__pgrx_c_bindings::JoinCostWorkspace> as c::expression::OrdinaryField<Field_numbuckets>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numbuckets) };
#[doc(hidden)]
pub struct Field_numbatches;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JoinCostWorkspace, numbatches) == 76);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numbatches>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JoinCostWorkspace>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 76;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JoinCostWorkspace, <<c::expression::CRecord<crate::__pgrx_c_bindings::JoinCostWorkspace> as c::expression::OrdinaryField<Field_numbatches>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numbatches) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JoinExpr>() == 72);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JoinExpr>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JoinExpr {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JoinExpr, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JoinExpr>
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
    crate::__pgrx_c_bindings::JoinExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::JoinExpr> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JoinExpr, jointype) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_jointype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JoinExpr>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__JoinType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::JoinExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::JoinExpr> as c::expression::OrdinaryField<
        Field_jointype,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).jointype) };
#[doc(hidden)]
pub struct Field_isNatural;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JoinExpr, isNatural) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isNatural>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JoinExpr>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::JoinExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::JoinExpr> as c::expression::OrdinaryField<
        Field_isNatural,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isNatural) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JoinExpr, usingClause) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_usingClause>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JoinExpr>
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
    crate::__pgrx_c_bindings::JoinExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::JoinExpr> as c::expression::OrdinaryField<
        Field_usingClause,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).usingClause) };
#[doc(hidden)]
pub struct Field_rtindex;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JoinExpr, rtindex) == 64);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rtindex>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JoinExpr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::JoinExpr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::JoinExpr> as c::expression::OrdinaryField<
        Field_rtindex,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rtindex) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JoinPath>() == 104);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JoinPath>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JoinPath {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JoinPath, jointype) == 72);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_jointype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JoinPath>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__JoinType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::JoinPath,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::JoinPath> as c::expression::OrdinaryField<
        Field_jointype,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).jointype) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JoinPath, inner_unique) == 76);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inner_unique>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JoinPath>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 76;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::JoinPath,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::JoinPath> as c::expression::OrdinaryField<
        Field_inner_unique,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).inner_unique) };
#[doc(hidden)]
pub struct Field_joinrestrictinfo;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JoinPath, joinrestrictinfo) == 96);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_joinrestrictinfo>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JoinPath>
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
    crate::__pgrx_c_bindings::JoinPath,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::JoinPath> as c::expression::OrdinaryField<
        Field_joinrestrictinfo,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).joinrestrictinfo) };
#[doc(hidden)]
pub struct Field_restrictlist;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JoinPathExtraData>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JoinPathExtraData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JoinPathExtraData {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::JoinPathExtraData, restrictlist) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_restrictlist>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JoinPathExtraData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JoinPathExtraData, <<c::expression::CRecord<crate::__pgrx_c_bindings::JoinPathExtraData> as c::expression::OrdinaryField<Field_restrictlist>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).restrictlist) };
#[doc(hidden)]
pub struct Field_mergeclause_list;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::JoinPathExtraData, mergeclause_list) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mergeclause_list>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JoinPathExtraData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JoinPathExtraData, <<c::expression::CRecord<crate::__pgrx_c_bindings::JoinPathExtraData> as c::expression::OrdinaryField<Field_mergeclause_list>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mergeclause_list) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::JoinPathExtraData, inner_unique) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inner_unique>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JoinPathExtraData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JoinPathExtraData, <<c::expression::CRecord<crate::__pgrx_c_bindings::JoinPathExtraData> as c::expression::OrdinaryField<Field_inner_unique>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).inner_unique) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JoinState>() == 216);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JoinState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JoinState {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JoinState, jointype) == 200);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_jointype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JoinState>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__JoinType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 200;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::JoinState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::JoinState> as c::expression::OrdinaryField<
        Field_jointype,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).jointype) };
#[doc(hidden)]
pub struct Field_single_match;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JoinState, single_match) == 204);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_single_match>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JoinState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 204;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::JoinState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::JoinState> as c::expression::OrdinaryField<
        Field_single_match,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).single_match) };
#[doc(hidden)]
pub struct Field_nElems;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JsonbIterator>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JsonbIterator>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JsonbIterator {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonbIterator, nElems) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nElems>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonbIterator>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonbIterator, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonbIterator> as c::expression::OrdinaryField<Field_nElems>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nElems) };
#[doc(hidden)]
pub struct Field_isScalar;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonbIterator, isScalar) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isScalar>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonbIterator>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonbIterator, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonbIterator> as c::expression::OrdinaryField<Field_isScalar>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isScalar) };
#[doc(hidden)]
pub struct Field_curIndex;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonbIterator, curIndex) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_curIndex>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonbIterator>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonbIterator, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonbIterator> as c::expression::OrdinaryField<Field_curIndex>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).curIndex) };
#[doc(hidden)]
pub struct Field_curDataOffset;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonbIterator, curDataOffset) == 36);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_curDataOffset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonbIterator>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonbIterator, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonbIterator> as c::expression::OrdinaryField<Field_curDataOffset>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).curDataOffset) };
#[doc(hidden)]
pub struct Field_curValueOffset;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonbIterator, curValueOffset) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_curValueOffset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonbIterator>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonbIterator, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonbIterator> as c::expression::OrdinaryField<Field_curValueOffset>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).curValueOffset) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonbIterator, state) == 44);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_state>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonbIterator>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_JsonbIterState,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonbIterator, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonbIterator> as c::expression::OrdinaryField<Field_state>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).state) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JsonbPair>() == 72);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JsonbPair>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JsonbPair {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonbPair, value) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_value>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonbPair>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::JsonbValue>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::JsonbPair,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonbPair> as c::expression::OrdinaryField<
        Field_value,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).value) };
#[doc(hidden)]
pub struct Field_order;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonbPair, order) == 64);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_order>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonbPair>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::JsonbPair,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonbPair> as c::expression::OrdinaryField<
        Field_order,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).order) };
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JsonbParseState>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JsonbParseState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JsonbParseState {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonbParseState, size) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonbParseState>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonbParseState, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonbParseState> as c::expression::OrdinaryField<Field_size>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).size) };
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_1>()
            == 16
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_1>()
            == 8
    );
};
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_1
{
}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_1, len)
        == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_1>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_1, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_1> as c::expression::OrdinaryField<Field_len>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).len) };
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_2>()
            == 24
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_2>()
            == 8
    );
};
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_2
{
}
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_2,
        nElems
    ) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nElems>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_2>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_2, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_2> as c::expression::OrdinaryField<Field_nElems>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nElems) };
#[doc(hidden)]
pub struct Field_rawScalar;
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_2,
        rawScalar
    ) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rawScalar>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_2>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_2, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_2> as c::expression::OrdinaryField<Field_rawScalar>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rawScalar) };
#[doc(hidden)]
pub struct Field_nPairs;
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_3>()
            == 16
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_3>()
            == 8
    );
};
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_3
{
}
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_3,
        nPairs
    ) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nPairs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_3>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_3, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_3> as c::expression::OrdinaryField<Field_nPairs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nPairs) };
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_4>()
            == 16
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_4>()
            == 8
    );
};
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_4
{
}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_4, len)
        == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_4>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_4, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_4> as c::expression::OrdinaryField<Field_len>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).len) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_4, data)
        == 8
);
const _: () = {
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::JsonbContainer>() == 8);
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::JsonbContainer>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_data>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_4>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::JsonbContainer>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_4, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_4> as c::expression::OrdinaryField<Field_data>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).data) };
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_5>()
            == 24
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_5>()
            == 8
    );
};
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_5
{
}
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_5,
        value
    ) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_value>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_5>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, crate::__pgrx_c_bindings::Datum>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_5, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_5> as c::expression::OrdinaryField<Field_value>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).value) };
#[doc(hidden)]
pub struct Field_typid;
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_5,
        typid
    ) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_5>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_5, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_5> as c::expression::OrdinaryField<Field_typid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typid) };
#[doc(hidden)]
pub struct Field_typmod;
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_5,
        typmod
    ) == 12
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typmod>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_5>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_5, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_5> as c::expression::OrdinaryField<Field_typmod>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typmod) };
#[doc(hidden)]
pub struct Field_tz;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_5, tz)
        == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tz>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_5>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_5, <<c::expression::CRecord<crate::__pgrx_c_bindings::JsonbValue__bindgen_ty_1__bindgen_ty_5> as c::expression::OrdinaryField<Field_tz>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tz) };
#[doc(hidden)]
pub struct Field_jumble_len;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JumbleState>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JumbleState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JumbleState {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JumbleState, jumble_len) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_jumble_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JumbleState>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JumbleState, <<c::expression::CRecord<crate::__pgrx_c_bindings::JumbleState> as c::expression::OrdinaryField<Field_jumble_len>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).jumble_len) };
#[doc(hidden)]
pub struct Field_clocations_buf_size;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::JumbleState, clocations_buf_size) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_clocations_buf_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JumbleState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JumbleState, <<c::expression::CRecord<crate::__pgrx_c_bindings::JumbleState> as c::expression::OrdinaryField<Field_clocations_buf_size>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).clocations_buf_size) };
#[doc(hidden)]
pub struct Field_clocations_count;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JumbleState, clocations_count) == 28);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_clocations_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JumbleState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JumbleState, <<c::expression::CRecord<crate::__pgrx_c_bindings::JumbleState> as c::expression::OrdinaryField<Field_clocations_count>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).clocations_count) };
#[doc(hidden)]
pub struct Field_highest_extern_param_id;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::JumbleState, highest_extern_param_id) == 32
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_highest_extern_param_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JumbleState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JumbleState, <<c::expression::CRecord<crate::__pgrx_c_bindings::JumbleState> as c::expression::OrdinaryField<Field_highest_extern_param_id>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).highest_extern_param_id) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::JunkFilter>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::JunkFilter>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::JunkFilter {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JunkFilter, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JunkFilter>
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
const _: Projection<crate::__pgrx_c_bindings::JunkFilter, <<c::expression::CRecord<crate::__pgrx_c_bindings::JunkFilter> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_jf_targetList;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::JunkFilter, jf_targetList) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_jf_targetList>
    for c::expression::CRecord<crate::__pgrx_c_bindings::JunkFilter>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::JunkFilter, <<c::expression::CRecord<crate::__pgrx_c_bindings::JunkFilter> as c::expression::OrdinaryField<Field_jf_targetList>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).jf_targetList) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LOCALLOCK>() == 72);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LOCALLOCK>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::LOCALLOCK {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LOCALLOCK, tag) == 0);
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LOCALLOCKTAG>() == 20);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LOCALLOCKTAG>() == 4);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tag>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LOCALLOCK>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::LOCALLOCKTAG>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::LOCALLOCK,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::LOCALLOCK> as c::expression::OrdinaryField<
        Field_tag,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tag) };
#[doc(hidden)]
pub struct Field_hashcode;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LOCALLOCK, hashcode) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hashcode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LOCALLOCK>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::LOCALLOCK,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::LOCALLOCK> as c::expression::OrdinaryField<
        Field_hashcode,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hashcode) };
#[doc(hidden)]
pub struct Field_nLocks;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LOCALLOCK, nLocks) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nLocks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LOCALLOCK>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::LOCALLOCK,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::LOCALLOCK> as c::expression::OrdinaryField<
        Field_nLocks,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nLocks) };
#[doc(hidden)]
pub struct Field_numLockOwners;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LOCALLOCK, numLockOwners) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numLockOwners>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LOCALLOCK>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::LOCALLOCK,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::LOCALLOCK> as c::expression::OrdinaryField<
        Field_numLockOwners,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numLockOwners) };
#[doc(hidden)]
pub struct Field_maxLockOwners;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LOCALLOCK, maxLockOwners) == 52);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_maxLockOwners>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LOCALLOCK>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::LOCALLOCK,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::LOCALLOCK> as c::expression::OrdinaryField<
        Field_maxLockOwners,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).maxLockOwners) };
#[doc(hidden)]
pub struct Field_holdsStrongLockCount;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LOCALLOCK, holdsStrongLockCount) == 64
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_holdsStrongLockCount>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LOCALLOCK>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::LOCALLOCK,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::LOCALLOCK> as c::expression::OrdinaryField<
        Field_holdsStrongLockCount,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).holdsStrongLockCount) };
#[doc(hidden)]
pub struct Field_lockCleared;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LOCALLOCK, lockCleared) == 65);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lockCleared>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LOCALLOCK>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 65;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::LOCALLOCK,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::LOCALLOCK> as c::expression::OrdinaryField<
        Field_lockCleared,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lockCleared) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LOCALLOCKOWNER>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LOCALLOCKOWNER>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::LOCALLOCKOWNER {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LOCALLOCKOWNER, nLocks) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nLocks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LOCALLOCKOWNER>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LOCALLOCKOWNER, <<c::expression::CRecord<crate::__pgrx_c_bindings::LOCALLOCKOWNER> as c::expression::OrdinaryField<Field_nLocks>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nLocks) };
#[doc(hidden)]
pub struct Field_lock;
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::LOCALLOCKTAG {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LOCALLOCKTAG, lock) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lock>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LOCALLOCKTAG>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::LOCKTAG>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LOCALLOCKTAG, <<c::expression::CRecord<crate::__pgrx_c_bindings::LOCALLOCKTAG> as c::expression::OrdinaryField<Field_lock>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lock) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LOCALLOCKTAG, mode) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LOCALLOCKTAG>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LOCALLOCKTAG, <<c::expression::CRecord<crate::__pgrx_c_bindings::LOCALLOCKTAG> as c::expression::OrdinaryField<Field_mode>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mode) };
#[doc(hidden)]
pub struct Field_held;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LOCALPREDICATELOCK>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LOCALPREDICATELOCK>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::LOCALPREDICATELOCK {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LOCALPREDICATELOCK, held) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_held>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LOCALPREDICATELOCK>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LOCALPREDICATELOCK, <<c::expression::CRecord<crate::__pgrx_c_bindings::LOCALPREDICATELOCK> as c::expression::OrdinaryField<Field_held>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).held) };
#[doc(hidden)]
pub struct Field_childLocks;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LOCALPREDICATELOCK, childLocks) == 20
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_childLocks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LOCALPREDICATELOCK>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LOCALPREDICATELOCK, <<c::expression::CRecord<crate::__pgrx_c_bindings::LOCALPREDICATELOCK> as c::expression::OrdinaryField<Field_childLocks>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).childLocks) };
#[doc(hidden)]
pub struct Field_tranche;
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::LWLock {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LWLock, tranche) == 0);
impl c::expression::OffsetField<Field_tranche>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LWLock>
{
    type Member = ();
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::LWLock, tranche);
}
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tranche>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LWLock>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::LWLock,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::LWLock> as c::expression::OrdinaryField<
        Field_tranche,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tranche) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LWLock, state) == 4);
impl c::expression::OffsetField<Field_state>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LWLock>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::pg_atomic_uint32>;
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::LWLock, state);
}
#[doc(hidden)]
pub struct Field_waiters;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LWLock, waiters) == 8);
impl c::expression::OffsetField<Field_waiters>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LWLock>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::proclist_head>;
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::LWLock, waiters);
}
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LargeObjectDesc>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LargeObjectDesc>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::LargeObjectDesc {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LargeObjectDesc, id) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LargeObjectDesc>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LargeObjectDesc, <<c::expression::CRecord<crate::__pgrx_c_bindings::LargeObjectDesc> as c::expression::OrdinaryField<Field_id>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).id) };
#[doc(hidden)]
pub struct Field_subid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LargeObjectDesc, subid) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_subid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LargeObjectDesc>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LargeObjectDesc, <<c::expression::CRecord<crate::__pgrx_c_bindings::LargeObjectDesc> as c::expression::OrdinaryField<Field_subid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).subid) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LargeObjectDesc, offset) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_offset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LargeObjectDesc>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LargeObjectDesc, <<c::expression::CRecord<crate::__pgrx_c_bindings::LargeObjectDesc> as c::expression::OrdinaryField<Field_offset>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).offset) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LargeObjectDesc, flags) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LargeObjectDesc>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LargeObjectDesc, <<c::expression::CRecord<crate::__pgrx_c_bindings::LargeObjectDesc> as c::expression::OrdinaryField<Field_flags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).flags) };
#[doc(hidden)]
pub struct Field_is_set;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Latch>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Latch>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::Latch {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Latch, is_set) == 0);
impl c::expression::OffsetField<Field_is_set>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Latch>
{
    type Member = ();
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::Latch, is_set);
}
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_set>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Latch>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Latch,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Latch> as c::expression::OrdinaryField<
        Field_is_set,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_set) };
#[doc(hidden)]
pub struct Field_maybe_sleeping;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Latch, maybe_sleeping) == 4);
impl c::expression::OffsetField<Field_maybe_sleeping>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Latch>
{
    type Member = ();
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::Latch, maybe_sleeping);
}
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_maybe_sleeping>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Latch>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Latch,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Latch> as c::expression::OrdinaryField<
        Field_maybe_sleeping,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).maybe_sleeping) };
#[doc(hidden)]
pub struct Field_is_shared;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Latch, is_shared) == 8);
impl c::expression::OffsetField<Field_is_shared>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Latch>
{
    type Member = ();
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::Latch, is_shared);
}
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_shared>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Latch>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Latch,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Latch> as c::expression::OrdinaryField<
        Field_is_shared,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_shared) };
#[doc(hidden)]
pub struct Field_owner_pid;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Latch, owner_pid) == 12);
impl c::expression::OffsetField<Field_owner_pid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Latch>
{
    type Member = ();
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::Latch, owner_pid);
}
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_owner_pid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Latch>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Latch,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Latch> as c::expression::OrdinaryField<
        Field_owner_pid,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).owner_pid) };
#[doc(hidden)]
pub struct Field_limitOption;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Limit>() == 152);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Limit>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::Limit {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Limit, limitOption) == 120);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_limitOption>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Limit>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__LimitOption,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Limit,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Limit> as c::expression::OrdinaryField<
        Field_limitOption,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).limitOption) };
#[doc(hidden)]
pub struct Field_uniqNumCols;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Limit, uniqNumCols) == 124);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_uniqNumCols>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Limit>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 124;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Limit,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Limit> as c::expression::OrdinaryField<
        Field_uniqNumCols,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).uniqNumCols) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LimitPath>() == 104);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LimitPath>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::LimitPath {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LimitPath, limitOption) == 96);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_limitOption>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LimitPath>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__LimitOption,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::LimitPath,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::LimitPath> as c::expression::OrdinaryField<
        Field_limitOption,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).limitOption) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LimitState>() == 280);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LimitState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::LimitState {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LimitState, limitOption) == 216);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_limitOption>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LimitState>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__LimitOption,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 216;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LimitState, <<c::expression::CRecord<crate::__pgrx_c_bindings::LimitState> as c::expression::OrdinaryField<Field_limitOption>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).limitOption) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LimitState, offset) == 224);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_offset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LimitState>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 224;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LimitState, <<c::expression::CRecord<crate::__pgrx_c_bindings::LimitState> as c::expression::OrdinaryField<Field_offset>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).offset) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LimitState, count) == 232);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LimitState>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 232;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LimitState, <<c::expression::CRecord<crate::__pgrx_c_bindings::LimitState> as c::expression::OrdinaryField<Field_count>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).count) };
#[doc(hidden)]
pub struct Field_noCount;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LimitState, noCount) == 240);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_noCount>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LimitState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 240;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LimitState, <<c::expression::CRecord<crate::__pgrx_c_bindings::LimitState> as c::expression::OrdinaryField<Field_noCount>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).noCount) };
#[doc(hidden)]
pub struct Field_lstate;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LimitState, lstate) == 244);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lstate>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LimitState>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_LimitStateCond,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 244;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LimitState, <<c::expression::CRecord<crate::__pgrx_c_bindings::LimitState> as c::expression::OrdinaryField<Field_lstate>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lstate) };
#[doc(hidden)]
pub struct Field_position;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LimitState, position) == 248);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_position>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LimitState>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 248;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LimitState, <<c::expression::CRecord<crate::__pgrx_c_bindings::LimitState> as c::expression::OrdinaryField<Field_position>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).position) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ListenStmt>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ListenStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ListenStmt {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ListenStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ListenStmt>
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
const _: Projection<crate::__pgrx_c_bindings::ListenStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::ListenStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LoadStmt>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LoadStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::LoadStmt {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LoadStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LoadStmt>
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
    crate::__pgrx_c_bindings::LoadStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::LoadStmt> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_backend_xid;
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LocalPgBackendStatus>() == 440);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LocalPgBackendStatus>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::LocalPgBackendStatus {}
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LocalPgBackendStatus, backend_xid) == 432
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_backend_xid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LocalPgBackendStatus>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 432;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LocalPgBackendStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::LocalPgBackendStatus> as c::expression::OrdinaryField<Field_backend_xid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).backend_xid) };
#[doc(hidden)]
pub struct Field_backend_xmin;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LocalPgBackendStatus, backend_xmin) == 436
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_backend_xmin>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LocalPgBackendStatus>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 436;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LocalPgBackendStatus, <<c::expression::CRecord<crate::__pgrx_c_bindings::LocalPgBackendStatus> as c::expression::OrdinaryField<Field_backend_xmin>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).backend_xmin) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LocationLen>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LocationLen>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::LocationLen {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LocationLen, location) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LocationLen>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LocationLen, <<c::expression::CRecord<crate::__pgrx_c_bindings::LocationLen> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LocationLen, length) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_length>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LocationLen>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LocationLen, <<c::expression::CRecord<crate::__pgrx_c_bindings::LocationLen> as c::expression::OrdinaryField<Field_length>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).length) };
#[doc(hidden)]
pub struct Field_nelements;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LockData>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LockData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::LockData {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LockData, nelements) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nelements>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LockData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::LockData,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::LockData> as c::expression::OrdinaryField<
        Field_nelements,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nelements) };
#[doc(hidden)]
pub struct Field_holdMask;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LockInstanceData>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LockInstanceData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::LockInstanceData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LockInstanceData, holdMask) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_holdMask>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LockInstanceData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LockInstanceData, <<c::expression::CRecord<crate::__pgrx_c_bindings::LockInstanceData> as c::expression::OrdinaryField<Field_holdMask>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).holdMask) };
#[doc(hidden)]
pub struct Field_waitLockMode;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LockInstanceData, waitLockMode) == 20
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_waitLockMode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LockInstanceData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LockInstanceData, <<c::expression::CRecord<crate::__pgrx_c_bindings::LockInstanceData> as c::expression::OrdinaryField<Field_waitLockMode>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).waitLockMode) };
#[doc(hidden)]
pub struct Field_backend;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LockInstanceData, backend) == 24);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_backend>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LockInstanceData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LockInstanceData, <<c::expression::CRecord<crate::__pgrx_c_bindings::LockInstanceData> as c::expression::OrdinaryField<Field_backend>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).backend) };
#[doc(hidden)]
pub struct Field_lxid;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LockInstanceData, lxid) == 28);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lxid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LockInstanceData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LockInstanceData, <<c::expression::CRecord<crate::__pgrx_c_bindings::LockInstanceData> as c::expression::OrdinaryField<Field_lxid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lxid) };
#[doc(hidden)]
pub struct Field_waitStart;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LockInstanceData, waitStart) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_waitStart>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LockInstanceData>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LockInstanceData, <<c::expression::CRecord<crate::__pgrx_c_bindings::LockInstanceData> as c::expression::OrdinaryField<Field_waitStart>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).waitStart) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LockInstanceData, pid) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LockInstanceData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LockInstanceData, <<c::expression::CRecord<crate::__pgrx_c_bindings::LockInstanceData> as c::expression::OrdinaryField<Field_pid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pid) };
#[doc(hidden)]
pub struct Field_leaderPid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LockInstanceData, leaderPid) == 44);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_leaderPid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LockInstanceData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LockInstanceData, <<c::expression::CRecord<crate::__pgrx_c_bindings::LockInstanceData> as c::expression::OrdinaryField<Field_leaderPid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).leaderPid) };
#[doc(hidden)]
pub struct Field_fastpath;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LockInstanceData, fastpath) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fastpath>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LockInstanceData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LockInstanceData, <<c::expression::CRecord<crate::__pgrx_c_bindings::LockInstanceData> as c::expression::OrdinaryField<Field_fastpath>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fastpath) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LockRelId>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LockRelId>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::LockRelId {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LockRelId, relId) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LockRelId>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::LockRelId,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::LockRelId> as c::expression::OrdinaryField<
        Field_relId,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relId) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LockRelId, dbId) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dbId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LockRelId>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::LockRelId,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::LockRelId> as c::expression::OrdinaryField<
        Field_dbId,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dbId) };
#[doc(hidden)]
pub struct Field_rowMarks;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LockRows>() == 120);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LockRows>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::LockRows {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LockRows, rowMarks) == 104);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rowMarks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LockRows>
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
    crate::__pgrx_c_bindings::LockRows,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::LockRows> as c::expression::OrdinaryField<
        Field_rowMarks,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rowMarks) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LockRows, epqParam) == 112);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_epqParam>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LockRows>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::LockRows,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::LockRows> as c::expression::OrdinaryField<
        Field_epqParam,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).epqParam) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LockRowsPath>() == 96);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LockRowsPath>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::LockRowsPath {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LockRowsPath, rowMarks) == 80);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rowMarks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LockRowsPath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LockRowsPath, <<c::expression::CRecord<crate::__pgrx_c_bindings::LockRowsPath> as c::expression::OrdinaryField<Field_rowMarks>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rowMarks) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LockRowsPath, epqParam) == 88);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_epqParam>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LockRowsPath>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LockRowsPath, <<c::expression::CRecord<crate::__pgrx_c_bindings::LockRowsPath> as c::expression::OrdinaryField<Field_epqParam>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).epqParam) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LockStmt>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LockStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::LockStmt {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LockStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LockStmt>
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
    crate::__pgrx_c_bindings::LockStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::LockStmt> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LockStmt, relations) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LockStmt>
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
    crate::__pgrx_c_bindings::LockStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::LockStmt> as c::expression::OrdinaryField<
        Field_relations,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relations) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LockStmt, mode) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LockStmt>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::LockStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::LockStmt> as c::expression::OrdinaryField<
        Field_mode,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mode) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LockStmt, nowait) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nowait>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LockStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::LockStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::LockStmt> as c::expression::OrdinaryField<
        Field_nowait,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nowait) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LockingClause>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LockingClause>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::LockingClause {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LockingClause, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LockingClause>
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
const _: Projection<crate::__pgrx_c_bindings::LockingClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::LockingClause> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_lockedRels;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LockingClause, lockedRels) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lockedRels>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LockingClause>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LockingClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::LockingClause> as c::expression::OrdinaryField<Field_lockedRels>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lockedRels) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LockingClause, strength) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_strength>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LockingClause>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__LockClauseStrength,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LockingClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::LockingClause> as c::expression::OrdinaryField<Field_strength>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).strength) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LockingClause, waitPolicy) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_waitPolicy>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LockingClause>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__LockWaitPolicy,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LockingClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::LockingClause> as c::expression::OrdinaryField<Field_waitPolicy>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).waitPolicy) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LogicalDecodingContext>() == 304);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LogicalDecodingContext>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::LogicalDecodingContext {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalDecodingContext, context) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_context>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalDecodingContext>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::MemoryContextData>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalDecodingContext, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalDecodingContext> as c::expression::OrdinaryField<Field_context>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).context) };
#[doc(hidden)]
pub struct Field_fast_forward;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalDecodingContext, fast_forward) == 40
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fast_forward>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalDecodingContext>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalDecodingContext, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalDecodingContext> as c::expression::OrdinaryField<Field_fast_forward>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fast_forward) };
#[doc(hidden)]
pub struct Field_output_plugin_options;
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::LogicalDecodingContext,
        output_plugin_options
    ) == 224
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_output_plugin_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalDecodingContext>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 224;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalDecodingContext, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalDecodingContext> as c::expression::OrdinaryField<Field_output_plugin_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).output_plugin_options) };
#[doc(hidden)]
pub struct Field_streaming;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalDecodingContext, streaming) == 280
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_streaming>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalDecodingContext>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 280;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalDecodingContext, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalDecodingContext> as c::expression::OrdinaryField<Field_streaming>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).streaming) };
#[doc(hidden)]
pub struct Field_twophase;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalDecodingContext, twophase) == 281
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_twophase>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalDecodingContext>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 281;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalDecodingContext, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalDecodingContext> as c::expression::OrdinaryField<Field_twophase>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).twophase) };
#[doc(hidden)]
pub struct Field_twophase_opt_given;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalDecodingContext, twophase_opt_given)
        == 282
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_twophase_opt_given>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalDecodingContext>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 282;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalDecodingContext, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalDecodingContext> as c::expression::OrdinaryField<Field_twophase_opt_given>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).twophase_opt_given) };
#[doc(hidden)]
pub struct Field_accept_writes;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalDecodingContext, accept_writes) == 283
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_accept_writes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalDecodingContext>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 283;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalDecodingContext, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalDecodingContext> as c::expression::OrdinaryField<Field_accept_writes>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).accept_writes) };
#[doc(hidden)]
pub struct Field_prepared_write;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalDecodingContext, prepared_write)
        == 284
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_prepared_write>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalDecodingContext>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 284;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalDecodingContext, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalDecodingContext> as c::expression::OrdinaryField<Field_prepared_write>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).prepared_write) };
#[doc(hidden)]
pub struct Field_write_location;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalDecodingContext, write_location)
        == 288
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_write_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalDecodingContext>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 288;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalDecodingContext, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalDecodingContext> as c::expression::OrdinaryField<Field_write_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).write_location) };
#[doc(hidden)]
pub struct Field_write_xid;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalDecodingContext, write_xid) == 296
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_write_xid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalDecodingContext>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 296;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalDecodingContext, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalDecodingContext> as c::expression::OrdinaryField<Field_write_xid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).write_xid) };
#[doc(hidden)]
pub struct Field_end_xact;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalDecodingContext, end_xact) == 300
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_end_xact>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalDecodingContext>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 300;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalDecodingContext, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalDecodingContext> as c::expression::OrdinaryField<Field_end_xact>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).end_xact) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
#[doc(hidden)]
pub struct Field_in_create;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalDecodingContext, in_create) == 301
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_in_create>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalDecodingContext>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 301;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalDecodingContext, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalDecodingContext> as c::expression::OrdinaryField<Field_in_create>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).in_create) };
#[doc(hidden)]
pub struct Field_final_lsn;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LogicalRepBeginData>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LogicalRepBeginData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::LogicalRepBeginData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepBeginData, final_lsn) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_final_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepBeginData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepBeginData, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepBeginData> as c::expression::OrdinaryField<Field_final_lsn>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).final_lsn) };
#[doc(hidden)]
pub struct Field_committime;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepBeginData, committime) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_committime>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepBeginData>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepBeginData, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepBeginData> as c::expression::OrdinaryField<Field_committime>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).committime) };
#[doc(hidden)]
pub struct Field_xid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepBeginData, xid) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepBeginData>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepBeginData, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepBeginData> as c::expression::OrdinaryField<Field_xid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xid) };
#[doc(hidden)]
pub struct Field_commit_lsn;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LogicalRepCommitData>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LogicalRepCommitData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::LogicalRepCommitData {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepCommitData, commit_lsn) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_commit_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepCommitData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepCommitData, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepCommitData> as c::expression::OrdinaryField<Field_commit_lsn>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).commit_lsn) };
#[doc(hidden)]
pub struct Field_end_lsn;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepCommitData, end_lsn) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_end_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepCommitData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepCommitData, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepCommitData> as c::expression::OrdinaryField<Field_end_lsn>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).end_lsn) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepCommitData, committime) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_committime>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepCommitData>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepCommitData, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepCommitData> as c::expression::OrdinaryField<Field_committime>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).committime) };
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::LogicalRepCommitPreparedTxnData>() == 232
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::LogicalRepCommitPreparedTxnData>() == 8
    );
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::LogicalRepCommitPreparedTxnData {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepCommitPreparedTxnData, commit_lsn)
        == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_commit_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepCommitPreparedTxnData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepCommitPreparedTxnData, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepCommitPreparedTxnData> as c::expression::OrdinaryField<Field_commit_lsn>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).commit_lsn) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepCommitPreparedTxnData, end_lsn)
        == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_end_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepCommitPreparedTxnData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepCommitPreparedTxnData, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepCommitPreparedTxnData> as c::expression::OrdinaryField<Field_end_lsn>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).end_lsn) };
#[doc(hidden)]
pub struct Field_commit_time;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepCommitPreparedTxnData, commit_time)
        == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_commit_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepCommitPreparedTxnData>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepCommitPreparedTxnData, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepCommitPreparedTxnData> as c::expression::OrdinaryField<Field_commit_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).commit_time) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepCommitPreparedTxnData, xid) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepCommitPreparedTxnData>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepCommitPreparedTxnData, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepCommitPreparedTxnData> as c::expression::OrdinaryField<Field_xid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xid) };
#[doc(hidden)]
pub struct Field_prepare_lsn;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LogicalRepPreparedTxnData>() == 232);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LogicalRepPreparedTxnData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::LogicalRepPreparedTxnData {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepPreparedTxnData, prepare_lsn) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_prepare_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepPreparedTxnData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepPreparedTxnData, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepPreparedTxnData> as c::expression::OrdinaryField<Field_prepare_lsn>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).prepare_lsn) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepPreparedTxnData, end_lsn) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_end_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepPreparedTxnData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepPreparedTxnData, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepPreparedTxnData> as c::expression::OrdinaryField<Field_end_lsn>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).end_lsn) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepPreparedTxnData, prepare_time)
        == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_prepare_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepPreparedTxnData>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepPreparedTxnData, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepPreparedTxnData> as c::expression::OrdinaryField<Field_prepare_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).prepare_time) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepPreparedTxnData, xid) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepPreparedTxnData>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepPreparedTxnData, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepPreparedTxnData> as c::expression::OrdinaryField<Field_xid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xid) };
#[doc(hidden)]
pub struct Field_localrelvalid;
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LogicalRepRelMapEntry>() == 104);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LogicalRepRelMapEntry>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::LogicalRepRelMapEntry {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepRelMapEntry, localrelvalid) == 64
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_localrelvalid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepRelMapEntry>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepRelMapEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepRelMapEntry> as c::expression::OrdinaryField<Field_localrelvalid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).localrelvalid) };
#[doc(hidden)]
pub struct Field_localreloid;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepRelMapEntry, localreloid) == 68
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_localreloid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepRelMapEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 68;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepRelMapEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepRelMapEntry> as c::expression::OrdinaryField<Field_localreloid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).localreloid) };
#[doc(hidden)]
pub struct Field_updatable;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepRelMapEntry, updatable) == 88
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_updatable>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepRelMapEntry>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepRelMapEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepRelMapEntry> as c::expression::OrdinaryField<Field_updatable>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).updatable) };
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepRelMapEntry, state) == 89);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_state>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepRelMapEntry>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 89;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepRelMapEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepRelMapEntry> as c::expression::OrdinaryField<Field_state>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).state) };
#[doc(hidden)]
pub struct Field_statelsn;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepRelMapEntry, statelsn) == 96
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_statelsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepRelMapEntry>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepRelMapEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepRelMapEntry> as c::expression::OrdinaryField<Field_statelsn>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).statelsn) };
#[doc(hidden)]
pub struct Field_remoteid;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LogicalRepRelation>() == 64);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LogicalRepRelation>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::LogicalRepRelation {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepRelation, remoteid) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_remoteid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepRelation>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepRelation, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepRelation> as c::expression::OrdinaryField<Field_remoteid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).remoteid) };
#[doc(hidden)]
pub struct Field_natts;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepRelation, natts) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_natts>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepRelation>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepRelation, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepRelation> as c::expression::OrdinaryField<Field_natts>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).natts) };
#[doc(hidden)]
pub struct Field_replident;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepRelation, replident) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_replident>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepRelation>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepRelation, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepRelation> as c::expression::OrdinaryField<Field_replident>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).replident) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepRelation, relkind) == 49);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relkind>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepRelation>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 49;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepRelation, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepRelation> as c::expression::OrdinaryField<Field_relkind>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relkind) };
#[doc(hidden)]
pub struct Field_prepare_end_lsn;
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::LogicalRepRollbackPreparedTxnData>()
            == 240
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::LogicalRepRollbackPreparedTxnData>() == 8
    );
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::LogicalRepRollbackPreparedTxnData {}
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::LogicalRepRollbackPreparedTxnData,
        prepare_end_lsn
    ) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_prepare_end_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepRollbackPreparedTxnData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepRollbackPreparedTxnData, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepRollbackPreparedTxnData> as c::expression::OrdinaryField<Field_prepare_end_lsn>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).prepare_end_lsn) };
#[doc(hidden)]
pub struct Field_rollback_end_lsn;
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::LogicalRepRollbackPreparedTxnData,
        rollback_end_lsn
    ) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rollback_end_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepRollbackPreparedTxnData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepRollbackPreparedTxnData, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepRollbackPreparedTxnData> as c::expression::OrdinaryField<Field_rollback_end_lsn>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rollback_end_lsn) };
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::LogicalRepRollbackPreparedTxnData,
        prepare_time
    ) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_prepare_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepRollbackPreparedTxnData>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepRollbackPreparedTxnData, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepRollbackPreparedTxnData> as c::expression::OrdinaryField<Field_prepare_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).prepare_time) };
#[doc(hidden)]
pub struct Field_rollback_time;
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::LogicalRepRollbackPreparedTxnData,
        rollback_time
    ) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rollback_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepRollbackPreparedTxnData>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepRollbackPreparedTxnData, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepRollbackPreparedTxnData> as c::expression::OrdinaryField<Field_rollback_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rollback_time) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepRollbackPreparedTxnData, xid) == 32
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepRollbackPreparedTxnData>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepRollbackPreparedTxnData, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepRollbackPreparedTxnData> as c::expression::OrdinaryField<Field_xid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xid) };
#[doc(hidden)]
pub struct Field_ncols;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LogicalRepTupleData>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LogicalRepTupleData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::LogicalRepTupleData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepTupleData, ncols) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ncols>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepTupleData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepTupleData, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepTupleData> as c::expression::OrdinaryField<Field_ncols>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ncols) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LogicalRepTyp>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LogicalRepTyp>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::LogicalRepTyp {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepTyp, remoteid) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_remoteid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepTyp>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepTyp, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepTyp> as c::expression::OrdinaryField<Field_remoteid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).remoteid) };
#[doc(hidden)]
pub struct Field_launch_time;
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::LogicalRepWorker>() == 112);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::LogicalRepWorker>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::LogicalRepWorker {}
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, launch_time) == 0);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_launch_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepWorker, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker> as c::expression::OrdinaryField<Field_launch_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).launch_time) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, in_use) == 8);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_in_use>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepWorker, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker> as c::expression::OrdinaryField<Field_in_use>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).in_use) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, generation) == 10);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_generation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 10;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepWorker, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker> as c::expression::OrdinaryField<Field_generation>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).generation) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, dbid) == 24);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dbid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepWorker, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker> as c::expression::OrdinaryField<Field_dbid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dbid) };
#[doc(hidden)]
pub struct Field_userid;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, userid) == 28);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_userid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepWorker, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker> as c::expression::OrdinaryField<Field_userid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).userid) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, subid) == 32);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_subid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepWorker, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker> as c::expression::OrdinaryField<Field_subid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).subid) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, relid) == 36);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepWorker, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker> as c::expression::OrdinaryField<Field_relid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relid) };
#[doc(hidden)]
pub struct Field_relstate;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, relstate) == 40);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relstate>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepWorker, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker> as c::expression::OrdinaryField<Field_relstate>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relstate) };
#[doc(hidden)]
pub struct Field_relstate_lsn;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, relstate_lsn) == 48
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relstate_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepWorker, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker> as c::expression::OrdinaryField<Field_relstate_lsn>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relstate_lsn) };
#[doc(hidden)]
pub struct Field_relmutex;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, relmutex) == 56);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relmutex>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepWorker, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker> as c::expression::OrdinaryField<Field_relmutex>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relmutex) };
#[doc(hidden)]
pub struct Field_last_lsn;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, last_lsn) == 72);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_last_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepWorker, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker> as c::expression::OrdinaryField<Field_last_lsn>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).last_lsn) };
#[doc(hidden)]
pub struct Field_last_send_time;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, last_send_time) == 80
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_last_send_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepWorker, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker> as c::expression::OrdinaryField<Field_last_send_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).last_send_time) };
#[doc(hidden)]
pub struct Field_last_recv_time;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, last_recv_time) == 88
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_last_recv_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepWorker, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker> as c::expression::OrdinaryField<Field_last_recv_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).last_recv_time) };
#[doc(hidden)]
pub struct Field_reply_lsn;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, reply_lsn) == 96);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_reply_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepWorker, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker> as c::expression::OrdinaryField<Field_reply_lsn>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).reply_lsn) };
#[doc(hidden)]
pub struct Field_reply_time;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::LogicalRepWorker, reply_time) == 104);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_reply_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::LogicalRepWorker, <<c::expression::CRecord<crate::__pgrx_c_bindings::LogicalRepWorker> as c::expression::OrdinaryField<Field_reply_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).reply_time) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MCVItem>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MCVItem>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::MCVItem {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MCVItem, isnull) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isnull>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MCVItem>
{
    type Member = c::expression::CPointer<c::CBool, c::expression::ReadWrite>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::MCVItem,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::MCVItem> as c::expression::OrdinaryField<
        Field_isnull,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isnull) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MCVList>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MCVList>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::MCVList {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MCVList, magic) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_magic>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MCVList>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::MCVList,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::MCVList> as c::expression::OrdinaryField<
        Field_magic,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).magic) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MCVList, type_) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MCVList>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::MCVList,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::MCVList> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MCVList, nitems) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nitems>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MCVList>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::MCVList,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::MCVList> as c::expression::OrdinaryField<
        Field_nitems,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nitems) };
#[doc(hidden)]
pub struct Field_ndimensions;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MCVList, ndimensions) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ndimensions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MCVList>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::MCVList,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::MCVList> as c::expression::OrdinaryField<
        Field_ndimensions,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ndimensions) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MVDependencies>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MVDependencies>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::MVDependencies {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MVDependencies, magic) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_magic>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MVDependencies>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MVDependencies, <<c::expression::CRecord<crate::__pgrx_c_bindings::MVDependencies> as c::expression::OrdinaryField<Field_magic>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).magic) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MVDependencies, type_) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MVDependencies>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MVDependencies, <<c::expression::CRecord<crate::__pgrx_c_bindings::MVDependencies> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_ndeps;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MVDependencies, ndeps) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ndeps>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MVDependencies>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MVDependencies, <<c::expression::CRecord<crate::__pgrx_c_bindings::MVDependencies> as c::expression::OrdinaryField<Field_ndeps>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ndeps) };
#[doc(hidden)]
pub struct Field_nattributes;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MVDependency>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MVDependency>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::MVDependency {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MVDependency, nattributes) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nattributes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MVDependency>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MVDependency, <<c::expression::CRecord<crate::__pgrx_c_bindings::MVDependency> as c::expression::OrdinaryField<Field_nattributes>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nattributes) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MVNDistinct>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MVNDistinct>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::MVNDistinct {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MVNDistinct, magic) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_magic>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MVNDistinct>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MVNDistinct, <<c::expression::CRecord<crate::__pgrx_c_bindings::MVNDistinct> as c::expression::OrdinaryField<Field_magic>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).magic) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MVNDistinct, type_) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MVNDistinct>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MVNDistinct, <<c::expression::CRecord<crate::__pgrx_c_bindings::MVNDistinct> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MVNDistinct, nitems) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nitems>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MVNDistinct>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MVNDistinct, <<c::expression::CRecord<crate::__pgrx_c_bindings::MVNDistinct> as c::expression::OrdinaryField<Field_nitems>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nitems) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MVNDistinctItem>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MVNDistinctItem>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::MVNDistinctItem {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MVNDistinctItem, nattributes) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nattributes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MVNDistinctItem>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MVNDistinctItem, <<c::expression::CRecord<crate::__pgrx_c_bindings::MVNDistinctItem> as c::expression::OrdinaryField<Field_nattributes>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nattributes) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MaterialState>() == 240);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MaterialState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::MaterialState {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MaterialState, eflags) == 224);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_eflags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MaterialState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 224;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MaterialState, <<c::expression::CRecord<crate::__pgrx_c_bindings::MaterialState> as c::expression::OrdinaryField<Field_eflags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).eflags) };
#[doc(hidden)]
pub struct Field_eof_underlying;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::MaterialState, eof_underlying) == 228
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_eof_underlying>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MaterialState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 228;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MaterialState, <<c::expression::CRecord<crate::__pgrx_c_bindings::MaterialState> as c::expression::OrdinaryField<Field_eof_underlying>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).eof_underlying) };
#[doc(hidden)]
pub struct Field_numKeys;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Memoize>() == 152);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Memoize>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::Memoize {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Memoize, numKeys) == 104);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numKeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Memoize>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Memoize,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Memoize> as c::expression::OrdinaryField<
        Field_numKeys,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numKeys) };
#[doc(hidden)]
pub struct Field_param_exprs;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Memoize, param_exprs) == 128);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_param_exprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Memoize>
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
    crate::__pgrx_c_bindings::Memoize,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Memoize> as c::expression::OrdinaryField<
        Field_param_exprs,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).param_exprs) };
#[doc(hidden)]
pub struct Field_singlerow;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Memoize, singlerow) == 136);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_singlerow>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Memoize>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Memoize,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Memoize> as c::expression::OrdinaryField<
        Field_singlerow,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).singlerow) };
#[doc(hidden)]
pub struct Field_binary_mode;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Memoize, binary_mode) == 137);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_binary_mode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Memoize>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 137;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Memoize,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Memoize> as c::expression::OrdinaryField<
        Field_binary_mode,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).binary_mode) };
#[doc(hidden)]
pub struct Field_est_entries;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Memoize, est_entries) == 140);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_est_entries>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Memoize>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 140;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Memoize,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Memoize> as c::expression::OrdinaryField<
        Field_est_entries,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).est_entries) };
#[doc(hidden)]
pub struct Field_cache_hits;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MemoizeInstrumentation>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MemoizeInstrumentation>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::MemoizeInstrumentation {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::MemoizeInstrumentation, cache_hits) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cache_hits>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MemoizeInstrumentation>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MemoizeInstrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::MemoizeInstrumentation> as c::expression::OrdinaryField<Field_cache_hits>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cache_hits) };
#[doc(hidden)]
pub struct Field_cache_misses;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::MemoizeInstrumentation, cache_misses) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cache_misses>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MemoizeInstrumentation>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MemoizeInstrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::MemoizeInstrumentation> as c::expression::OrdinaryField<Field_cache_misses>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cache_misses) };
#[doc(hidden)]
pub struct Field_cache_evictions;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::MemoizeInstrumentation, cache_evictions)
        == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cache_evictions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MemoizeInstrumentation>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MemoizeInstrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::MemoizeInstrumentation> as c::expression::OrdinaryField<Field_cache_evictions>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cache_evictions) };
#[doc(hidden)]
pub struct Field_cache_overflows;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::MemoizeInstrumentation, cache_overflows)
        == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cache_overflows>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MemoizeInstrumentation>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MemoizeInstrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::MemoizeInstrumentation> as c::expression::OrdinaryField<Field_cache_overflows>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cache_overflows) };
#[doc(hidden)]
pub struct Field_mem_peak;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::MemoizeInstrumentation, mem_peak) == 32
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mem_peak>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MemoizeInstrumentation>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MemoizeInstrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::MemoizeInstrumentation> as c::expression::OrdinaryField<Field_mem_peak>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mem_peak) };
#[doc(hidden)]
pub struct Field_hash_operators;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MemoizePath>() == 120);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MemoizePath>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::MemoizePath {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MemoizePath, hash_operators) == 80);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hash_operators>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MemoizePath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MemoizePath, <<c::expression::CRecord<crate::__pgrx_c_bindings::MemoizePath> as c::expression::OrdinaryField<Field_hash_operators>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hash_operators) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MemoizePath, param_exprs) == 88);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_param_exprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MemoizePath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MemoizePath, <<c::expression::CRecord<crate::__pgrx_c_bindings::MemoizePath> as c::expression::OrdinaryField<Field_param_exprs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).param_exprs) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MemoizePath, singlerow) == 96);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_singlerow>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MemoizePath>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MemoizePath, <<c::expression::CRecord<crate::__pgrx_c_bindings::MemoizePath> as c::expression::OrdinaryField<Field_singlerow>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).singlerow) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MemoizePath, binary_mode) == 97);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_binary_mode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MemoizePath>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 97;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MemoizePath, <<c::expression::CRecord<crate::__pgrx_c_bindings::MemoizePath> as c::expression::OrdinaryField<Field_binary_mode>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).binary_mode) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MemoizePath, est_entries) == 112);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_est_entries>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MemoizePath>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MemoizePath, <<c::expression::CRecord<crate::__pgrx_c_bindings::MemoizePath> as c::expression::OrdinaryField<Field_est_entries>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).est_entries) };
#[doc(hidden)]
pub struct Field_mstatus;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MemoizeState>() == 416);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MemoizeState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::MemoizeState {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MemoizeState, mstatus) == 224);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mstatus>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MemoizeState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 224;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MemoizeState, <<c::expression::CRecord<crate::__pgrx_c_bindings::MemoizeState> as c::expression::OrdinaryField<Field_mstatus>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mstatus) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MemoizeState, nkeys) == 228);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nkeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MemoizeState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 228;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MemoizeState, <<c::expression::CRecord<crate::__pgrx_c_bindings::MemoizeState> as c::expression::OrdinaryField<Field_nkeys>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nkeys) };
#[doc(hidden)]
pub struct Field_mem_used;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MemoizeState, mem_used) == 296);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mem_used>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MemoizeState>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 296;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MemoizeState, <<c::expression::CRecord<crate::__pgrx_c_bindings::MemoizeState> as c::expression::OrdinaryField<Field_mem_used>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mem_used) };
#[doc(hidden)]
pub struct Field_mem_limit;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MemoizeState, mem_limit) == 304);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mem_limit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MemoizeState>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 304;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MemoizeState, <<c::expression::CRecord<crate::__pgrx_c_bindings::MemoizeState> as c::expression::OrdinaryField<Field_mem_limit>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mem_limit) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MemoizeState, singlerow) == 352);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_singlerow>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MemoizeState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 352;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MemoizeState, <<c::expression::CRecord<crate::__pgrx_c_bindings::MemoizeState> as c::expression::OrdinaryField<Field_singlerow>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).singlerow) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MemoizeState, binary_mode) == 353);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_binary_mode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MemoizeState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 353;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MemoizeState, <<c::expression::CRecord<crate::__pgrx_c_bindings::MemoizeState> as c::expression::OrdinaryField<Field_binary_mode>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).binary_mode) };
#[doc(hidden)]
pub struct Field_nblocks;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MemoryContextCounters>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MemoryContextCounters>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::MemoryContextCounters {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::MemoryContextCounters, nblocks) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nblocks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MemoryContextCounters>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MemoryContextCounters, <<c::expression::CRecord<crate::__pgrx_c_bindings::MemoryContextCounters> as c::expression::OrdinaryField<Field_nblocks>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nblocks) };
#[doc(hidden)]
pub struct Field_freechunks;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::MemoryContextCounters, freechunks) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_freechunks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MemoryContextCounters>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::MemoryContextCounters, <<c::expression::CRecord<crate::__pgrx_c_bindings::MemoryContextCounters> as c::expression::OrdinaryField<Field_freechunks>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).freechunks) };
#[doc(hidden)]
pub struct Field_totalspace;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::MemoryContextCounters, totalspace) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_totalspace>
    for c::expression::CRecord<crate::__pgrx_c_bindings::MemoryContextCounters>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
