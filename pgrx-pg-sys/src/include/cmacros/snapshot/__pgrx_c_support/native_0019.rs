// Shared generated C macro support.

// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Query,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Query> as c::expression::OrdinaryField<
        Field_hasModifyingCTE,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hasModifyingCTE) };
#[doc(hidden)]
pub struct Field_hasForUpdate;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, hasForUpdate) == 51);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasForUpdate>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 51;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Query,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Query> as c::expression::OrdinaryField<
        Field_hasForUpdate,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hasForUpdate) };
#[doc(hidden)]
pub struct Field_hasRowSecurity;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, hasRowSecurity) == 52);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hasRowSecurity>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Query,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Query> as c::expression::OrdinaryField<
        Field_hasRowSecurity,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hasRowSecurity) };
#[doc(hidden)]
pub struct Field_isReturn;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, isReturn) == 53);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isReturn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 53;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Query,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Query> as c::expression::OrdinaryField<
        Field_isReturn,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isReturn) };
#[doc(hidden)]
pub struct Field_cteList;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, cteList) == 56);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cteList>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
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
    crate::__pgrx_c_bindings::Query,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Query> as c::expression::OrdinaryField<
        Field_cteList,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cteList) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, rtable) == 64);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rtable>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
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
    crate::__pgrx_c_bindings::Query,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Query> as c::expression::OrdinaryField<
        Field_rtable,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rtable) };
#[doc(hidden)]
pub struct Field_mergeActionList;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, mergeActionList) == 80);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mergeActionList>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
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
    crate::__pgrx_c_bindings::Query,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Query> as c::expression::OrdinaryField<
        Field_mergeActionList,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mergeActionList) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
#[doc(hidden)]
pub struct Field_mergeUseOuterJoin;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, mergeUseOuterJoin) == 88);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mergeUseOuterJoin>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Query,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Query> as c::expression::OrdinaryField<
        Field_mergeUseOuterJoin,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mergeUseOuterJoin) };
#[cfg(feature = "pg15")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, targetList) == 96);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_targetList>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
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
    crate::__pgrx_c_bindings::Query,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Query> as c::expression::OrdinaryField<
        Field_targetList,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).targetList) };
#[cfg(feature = "pg15")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, override_) == 104);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_override>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__OverridingKind,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Query,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Query> as c::expression::OrdinaryField<
        Field_override,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).override_) };
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, returningList) == 120);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_returningList>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
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
    crate::__pgrx_c_bindings::Query,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Query> as c::expression::OrdinaryField<
        Field_returningList,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).returningList) };
#[cfg(feature = "pg15")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, groupClause) == 128);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_groupClause>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
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
    crate::__pgrx_c_bindings::Query,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Query> as c::expression::OrdinaryField<
        Field_groupClause,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).groupClause) };
#[doc(hidden)]
pub struct Field_groupDistinct;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, groupDistinct) == 136);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_groupDistinct>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Query,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Query> as c::expression::OrdinaryField<
        Field_groupDistinct,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).groupDistinct) };
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, groupingSets) == 144);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_groupingSets>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
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
    crate::__pgrx_c_bindings::Query,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Query> as c::expression::OrdinaryField<
        Field_groupingSets,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).groupingSets) };
#[doc(hidden)]
pub struct Field_windowClause;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, windowClause) == 160);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_windowClause>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Query,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Query> as c::expression::OrdinaryField<
        Field_windowClause,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).windowClause) };
#[doc(hidden)]
pub struct Field_distinctClause;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, distinctClause) == 168);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_distinctClause>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 168;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Query,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Query> as c::expression::OrdinaryField<
        Field_distinctClause,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).distinctClause) };
#[doc(hidden)]
pub struct Field_sortClause;
#[cfg(feature = "pg15")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, sortClause) == 176);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sortClause>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 176;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Query,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Query> as c::expression::OrdinaryField<
        Field_sortClause,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sortClause) };
#[cfg(feature = "pg15")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, limitOption) == 200);
#[cfg(feature = "pg15")]
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
    const OFFSET: usize = 200;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Query,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Query> as c::expression::OrdinaryField<
        Field_limitOption,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).limitOption) };
#[cfg(feature = "pg15")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, rowMarks) == 208);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rowMarks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 208;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Query,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Query> as c::expression::OrdinaryField<
        Field_rowMarks,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rowMarks) };
#[doc(hidden)]
pub struct Field_constraintDeps;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, constraintDeps) == 224);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_constraintDeps>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 224;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Query,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Query> as c::expression::OrdinaryField<
        Field_constraintDeps,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).constraintDeps) };
#[doc(hidden)]
pub struct Field_withCheckOptions;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, withCheckOptions) == 232);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_withCheckOptions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 232;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Query,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Query> as c::expression::OrdinaryField<
        Field_withCheckOptions,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).withCheckOptions) };
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, stmt_location) == 240);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmt_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 240;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Query,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Query> as c::expression::OrdinaryField<
        Field_stmt_location,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stmt_location) };
#[cfg(feature = "pg15")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Query, stmt_len) == 244);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmt_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Query>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 244;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Query,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Query> as c::expression::OrdinaryField<
        Field_stmt_len,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stmt_len) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::QueryDesc>() == 112);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::QueryDesc>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::QueryDesc {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::QueryDesc, operation) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_operation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::QueryDesc>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__CmdType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::QueryDesc,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::QueryDesc> as c::expression::OrdinaryField<
        Field_operation,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).operation) };
#[doc(hidden)]
pub struct Field_instrument_options;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::QueryDesc, instrument_options) == 64);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_instrument_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::QueryDesc>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::QueryDesc,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::QueryDesc> as c::expression::OrdinaryField<
        Field_instrument_options,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).instrument_options) };
#[doc(hidden)]
pub struct Field_already_executed;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::QueryDesc, already_executed) == 96);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_already_executed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::QueryDesc>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::QueryDesc,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::QueryDesc> as c::expression::OrdinaryField<
        Field_already_executed,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).already_executed) };
#[doc(hidden)]
pub struct Field_is_over;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RBTreeIterator>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RBTreeIterator>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::RBTreeIterator {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RBTreeIterator, is_over) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_over>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RBTreeIterator>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RBTreeIterator, <<c::expression::CRecord<crate::__pgrx_c_bindings::RBTreeIterator> as c::expression::OrdinaryField<Field_is_over>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_over) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RangeFunction>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RangeFunction>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::RangeFunction {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeFunction, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeFunction>
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
const _: Projection<crate::__pgrx_c_bindings::RangeFunction, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeFunction> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_lateral;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeFunction, lateral) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lateral>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeFunction>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeFunction, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeFunction> as c::expression::OrdinaryField<Field_lateral>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lateral) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeFunction, ordinality) == 5);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ordinality>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeFunction>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 5;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeFunction, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeFunction> as c::expression::OrdinaryField<Field_ordinality>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ordinality) };
#[doc(hidden)]
pub struct Field_is_rowsfrom;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeFunction, is_rowsfrom) == 6);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_rowsfrom>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeFunction>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 6;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeFunction, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeFunction> as c::expression::OrdinaryField<Field_is_rowsfrom>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_rowsfrom) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeFunction, functions) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_functions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeFunction>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeFunction, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeFunction> as c::expression::OrdinaryField<Field_functions>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).functions) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeFunction, coldeflist) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_coldeflist>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeFunction>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeFunction, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeFunction> as c::expression::OrdinaryField<Field_coldeflist>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).coldeflist) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RangeSubselect>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RangeSubselect>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::RangeSubselect {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeSubselect, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeSubselect>
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
const _: Projection<crate::__pgrx_c_bindings::RangeSubselect, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeSubselect> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeSubselect, lateral) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lateral>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeSubselect>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeSubselect, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeSubselect> as c::expression::OrdinaryField<Field_lateral>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lateral) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RangeTableFunc>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RangeTableFunc>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::RangeTableFunc {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTableFunc, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTableFunc>
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
const _: Projection<crate::__pgrx_c_bindings::RangeTableFunc, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTableFunc> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTableFunc, lateral) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lateral>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTableFunc>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeTableFunc, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTableFunc> as c::expression::OrdinaryField<Field_lateral>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lateral) };
#[doc(hidden)]
pub struct Field_namespaces;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTableFunc, namespaces) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_namespaces>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTableFunc>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeTableFunc, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTableFunc> as c::expression::OrdinaryField<Field_namespaces>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).namespaces) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTableFunc, columns) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_columns>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTableFunc>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeTableFunc, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTableFunc> as c::expression::OrdinaryField<Field_columns>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).columns) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTableFunc, location) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTableFunc>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeTableFunc, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTableFunc> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RangeTableFuncCol>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RangeTableFuncCol>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::RangeTableFuncCol {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTableFuncCol, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTableFuncCol>
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
const _: Projection<crate::__pgrx_c_bindings::RangeTableFuncCol, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTableFuncCol> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_for_ordinality;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTableFuncCol, for_ordinality) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_for_ordinality>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTableFuncCol>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeTableFuncCol, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTableFuncCol> as c::expression::OrdinaryField<Field_for_ordinality>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).for_ordinality) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTableFuncCol, is_not_null) == 25
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_not_null>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTableFuncCol>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 25;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeTableFuncCol, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTableFuncCol> as c::expression::OrdinaryField<Field_is_not_null>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_not_null) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTableFuncCol, location) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTableFuncCol>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeTableFuncCol, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTableFuncCol> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RangeTableSample>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RangeTableSample>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::RangeTableSample {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTableSample, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTableSample>
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
const _: Projection<crate::__pgrx_c_bindings::RangeTableSample, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTableSample> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_method;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTableSample, method) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_method>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTableSample>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeTableSample, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTableSample> as c::expression::OrdinaryField<Field_method>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).method) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTableSample, args) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_args>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTableSample>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeTableSample, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTableSample> as c::expression::OrdinaryField<Field_args>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).args) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTableSample, location) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTableSample>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeTableSample, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTableSample> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RangeTblEntry>() == 248);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RangeTblEntry>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::RangeTblEntry {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
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
const _: Projection<crate::__pgrx_c_bindings::RangeTblEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_rtekind;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, rtekind) == 4);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rtekind>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__RTEKind,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeTblEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry> as c::expression::OrdinaryField<Field_rtekind>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rtekind) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, relid) == 8);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeTblEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry> as c::expression::OrdinaryField<Field_relid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relid) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, relkind) == 12);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relkind>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeTblEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry> as c::expression::OrdinaryField<Field_relkind>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relkind) };
#[doc(hidden)]
pub struct Field_rellockmode;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, rellockmode) == 16);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rellockmode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeTblEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry> as c::expression::OrdinaryField<Field_rellockmode>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rellockmode) };
#[doc(hidden)]
pub struct Field_security_barrier;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, security_barrier) == 40
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_security_barrier>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeTblEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry> as c::expression::OrdinaryField<Field_security_barrier>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).security_barrier) };
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, jointype) == 44);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_jointype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
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
const _: Projection<crate::__pgrx_c_bindings::RangeTblEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry> as c::expression::OrdinaryField<Field_jointype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).jointype) };
#[doc(hidden)]
pub struct Field_joinmergedcols;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, joinmergedcols) == 48);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_joinmergedcols>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeTblEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry> as c::expression::OrdinaryField<Field_joinmergedcols>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).joinmergedcols) };
#[doc(hidden)]
pub struct Field_joinaliasvars;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, joinaliasvars) == 56);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_joinaliasvars>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeTblEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry> as c::expression::OrdinaryField<Field_joinaliasvars>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).joinaliasvars) };
#[doc(hidden)]
pub struct Field_joinleftcols;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, joinleftcols) == 64);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_joinleftcols>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeTblEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry> as c::expression::OrdinaryField<Field_joinleftcols>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).joinleftcols) };
#[doc(hidden)]
pub struct Field_joinrightcols;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, joinrightcols) == 72);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_joinrightcols>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeTblEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry> as c::expression::OrdinaryField<Field_joinrightcols>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).joinrightcols) };
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, functions) == 88);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_functions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeTblEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry> as c::expression::OrdinaryField<Field_functions>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).functions) };
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, funcordinality) == 96);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_funcordinality>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeTblEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry> as c::expression::OrdinaryField<Field_funcordinality>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).funcordinality) };
#[doc(hidden)]
pub struct Field_values_lists;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, values_lists) == 112);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_values_lists>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeTblEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry> as c::expression::OrdinaryField<Field_values_lists>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).values_lists) };
#[doc(hidden)]
pub struct Field_ctelevelsup;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, ctelevelsup) == 128);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ctelevelsup>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeTblEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry> as c::expression::OrdinaryField<Field_ctelevelsup>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ctelevelsup) };
#[doc(hidden)]
pub struct Field_self_reference;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, self_reference) == 132
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_self_reference>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 132;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeTblEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry> as c::expression::OrdinaryField<Field_self_reference>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).self_reference) };
#[doc(hidden)]
pub struct Field_coltypes;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, coltypes) == 136);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_coltypes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeTblEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry> as c::expression::OrdinaryField<Field_coltypes>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).coltypes) };
#[doc(hidden)]
pub struct Field_coltypmods;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, coltypmods) == 144);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_coltypmods>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeTblEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry> as c::expression::OrdinaryField<Field_coltypmods>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).coltypmods) };
#[doc(hidden)]
pub struct Field_colcollations;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, colcollations) == 152);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_colcollations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeTblEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry> as c::expression::OrdinaryField<Field_colcollations>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).colcollations) };
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, lateral) == 192);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lateral>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 192;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeTblEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry> as c::expression::OrdinaryField<Field_lateral>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lateral) };
#[cfg(feature = "pg15")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, inh) == 193);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inh>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 193;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeTblEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry> as c::expression::OrdinaryField<Field_inh>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).inh) };
#[doc(hidden)]
pub struct Field_inFromCl;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, inFromCl) == 194);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inFromCl>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 194;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeTblEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry> as c::expression::OrdinaryField<Field_inFromCl>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).inFromCl) };
#[doc(hidden)]
pub struct Field_requiredPerms;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, requiredPerms) == 196);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_requiredPerms>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 196;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeTblEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry> as c::expression::OrdinaryField<Field_requiredPerms>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).requiredPerms) };
#[doc(hidden)]
pub struct Field_checkAsUser;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, checkAsUser) == 200);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_checkAsUser>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 200;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeTblEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry> as c::expression::OrdinaryField<Field_checkAsUser>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).checkAsUser) };
#[doc(hidden)]
pub struct Field_securityQuals;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblEntry, securityQuals) == 240);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_securityQuals>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 240;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeTblEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblEntry> as c::expression::OrdinaryField<Field_securityQuals>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).securityQuals) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RangeTblFunction>() == 64);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RangeTblFunction>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::RangeTblFunction {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblFunction, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblFunction>
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
const _: Projection<crate::__pgrx_c_bindings::RangeTblFunction, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblFunction> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_funccolcount;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblFunction, funccolcount) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_funccolcount>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblFunction>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeTblFunction, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblFunction> as c::expression::OrdinaryField<Field_funccolcount>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).funccolcount) };
#[doc(hidden)]
pub struct Field_funccolnames;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblFunction, funccolnames) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_funccolnames>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblFunction>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeTblFunction, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblFunction> as c::expression::OrdinaryField<Field_funccolnames>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).funccolnames) };
#[doc(hidden)]
pub struct Field_funccoltypes;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblFunction, funccoltypes) == 32
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_funccoltypes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblFunction>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeTblFunction, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblFunction> as c::expression::OrdinaryField<Field_funccoltypes>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).funccoltypes) };
#[doc(hidden)]
pub struct Field_funccoltypmods;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblFunction, funccoltypmods) == 40
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_funccoltypmods>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblFunction>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeTblFunction, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblFunction> as c::expression::OrdinaryField<Field_funccoltypmods>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).funccoltypmods) };
#[doc(hidden)]
pub struct Field_funccolcollations;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblFunction, funccolcollations) == 48
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_funccolcollations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblFunction>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeTblFunction, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblFunction> as c::expression::OrdinaryField<Field_funccolcollations>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).funccolcollations) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RangeTblRef>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RangeTblRef>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::RangeTblRef {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblRef, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblRef>
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
const _: Projection<crate::__pgrx_c_bindings::RangeTblRef, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblRef> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeTblRef, rtindex) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rtindex>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblRef>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RangeTblRef, <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeTblRef> as c::expression::OrdinaryField<Field_rtindex>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rtindex) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RawColumnDefault>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RawColumnDefault>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::RawColumnDefault {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RawColumnDefault, attnum) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_attnum>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RawColumnDefault>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RawColumnDefault, <<c::expression::CRecord<crate::__pgrx_c_bindings::RawColumnDefault> as c::expression::OrdinaryField<Field_attnum>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).attnum) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
#[doc(hidden)]
pub struct Field_missingMode;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RawColumnDefault, missingMode) == 16);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_missingMode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RawColumnDefault>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RawColumnDefault, <<c::expression::CRecord<crate::__pgrx_c_bindings::RawColumnDefault> as c::expression::OrdinaryField<Field_missingMode>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).missingMode) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RawColumnDefault, generated) == 17);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_generated>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RawColumnDefault>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 17;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RawColumnDefault, <<c::expression::CRecord<crate::__pgrx_c_bindings::RawColumnDefault> as c::expression::OrdinaryField<Field_generated>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).generated) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RawStmt>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RawStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::RawStmt {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RawStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RawStmt>
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
    crate::__pgrx_c_bindings::RawStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::RawStmt> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RawStmt, stmt_location) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmt_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RawStmt>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::RawStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::RawStmt> as c::expression::OrdinaryField<
        Field_stmt_location,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stmt_location) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RawStmt, stmt_len) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stmt_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RawStmt>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::RawStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::RawStmt> as c::expression::OrdinaryField<
        Field_stmt_len,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stmt_len) };
#[doc(hidden)]
pub struct Field_end_of_wal;
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ReadLocalXLogPageNoWaitPrivate>() == 1
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ReadLocalXLogPageNoWaitPrivate>() == 1
    );
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ReadLocalXLogPageNoWaitPrivate {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReadLocalXLogPageNoWaitPrivate, end_of_wal)
        == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_end_of_wal>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReadLocalXLogPageNoWaitPrivate>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReadLocalXLogPageNoWaitPrivate, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReadLocalXLogPageNoWaitPrivate> as c::expression::OrdinaryField<Field_end_of_wal>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).end_of_wal) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ReadReplicationSlotCmd>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ReadReplicationSlotCmd>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ReadReplicationSlotCmd {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReadReplicationSlotCmd, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReadReplicationSlotCmd>
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
const _: Projection<crate::__pgrx_c_bindings::ReadReplicationSlotCmd, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReadReplicationSlotCmd> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ReassignOwnedStmt>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ReassignOwnedStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ReassignOwnedStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReassignOwnedStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReassignOwnedStmt>
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
const _: Projection<crate::__pgrx_c_bindings::ReassignOwnedStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReassignOwnedStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReassignOwnedStmt, roles) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_roles>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReassignOwnedStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReassignOwnedStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReassignOwnedStmt> as c::expression::OrdinaryField<Field_roles>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).roles) };
#[doc(hidden)]
pub struct Field_wtParam;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RecursiveUnion>() == 144);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RecursiveUnion>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::RecursiveUnion {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RecursiveUnion, wtParam) == 104);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wtParam>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RecursiveUnion>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RecursiveUnion, <<c::expression::CRecord<crate::__pgrx_c_bindings::RecursiveUnion> as c::expression::OrdinaryField<Field_wtParam>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wtParam) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RecursiveUnion, numCols) == 108);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numCols>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RecursiveUnion>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 108;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RecursiveUnion, <<c::expression::CRecord<crate::__pgrx_c_bindings::RecursiveUnion> as c::expression::OrdinaryField<Field_numCols>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numCols) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RecursiveUnion, numGroups) == 136);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numGroups>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RecursiveUnion>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RecursiveUnion, <<c::expression::CRecord<crate::__pgrx_c_bindings::RecursiveUnion> as c::expression::OrdinaryField<Field_numGroups>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numGroups) };
#[doc(hidden)]
pub struct Field_distinctList;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RecursiveUnionPath>() == 112);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RecursiveUnionPath>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::RecursiveUnionPath {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RecursiveUnionPath, distinctList) == 88
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_distinctList>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RecursiveUnionPath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RecursiveUnionPath, <<c::expression::CRecord<crate::__pgrx_c_bindings::RecursiveUnionPath> as c::expression::OrdinaryField<Field_distinctList>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).distinctList) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RecursiveUnionPath, wtParam) == 96);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wtParam>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RecursiveUnionPath>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RecursiveUnionPath, <<c::expression::CRecord<crate::__pgrx_c_bindings::RecursiveUnionPath> as c::expression::OrdinaryField<Field_wtParam>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wtParam) };
#[doc(hidden)]
pub struct Field_recursing;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RecursiveUnionState>() == 264);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RecursiveUnionState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::RecursiveUnionState {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RecursiveUnionState, recursing) == 200
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_recursing>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RecursiveUnionState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 200;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RecursiveUnionState, <<c::expression::CRecord<crate::__pgrx_c_bindings::RecursiveUnionState> as c::expression::OrdinaryField<Field_recursing>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).recursing) };
#[doc(hidden)]
pub struct Field_intermediate_empty;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RecursiveUnionState, intermediate_empty)
        == 201
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_intermediate_empty>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RecursiveUnionState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 201;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RecursiveUnionState, <<c::expression::CRecord<crate::__pgrx_c_bindings::RecursiveUnionState> as c::expression::OrdinaryField<Field_intermediate_empty>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).intermediate_empty) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RefreshMatViewStmt>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RefreshMatViewStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::RefreshMatViewStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RefreshMatViewStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RefreshMatViewStmt>
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
const _: Projection<crate::__pgrx_c_bindings::RefreshMatViewStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::RefreshMatViewStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RefreshMatViewStmt, concurrent) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_concurrent>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RefreshMatViewStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RefreshMatViewStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::RefreshMatViewStmt> as c::expression::OrdinaryField<Field_concurrent>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).concurrent) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RefreshMatViewStmt, skipData) == 5);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_skipData>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RefreshMatViewStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 5;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RefreshMatViewStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::RefreshMatViewStmt> as c::expression::OrdinaryField<Field_skipData>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).skipData) };
#[doc(hidden)]
pub struct Field_rw_pid;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RegisteredBgWorker>() == 584);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RegisteredBgWorker>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::RegisteredBgWorker {}
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RegisteredBgWorker, rw_pid) == 552);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rw_pid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RegisteredBgWorker>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 552;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RegisteredBgWorker, <<c::expression::CRecord<crate::__pgrx_c_bindings::RegisteredBgWorker> as c::expression::OrdinaryField<Field_rw_pid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rw_pid) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
#[doc(hidden)]
pub struct Field_rw_child_slot;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RegisteredBgWorker, rw_child_slot) == 556
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rw_child_slot>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RegisteredBgWorker>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 556;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RegisteredBgWorker, <<c::expression::CRecord<crate::__pgrx_c_bindings::RegisteredBgWorker> as c::expression::OrdinaryField<Field_rw_child_slot>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rw_child_slot) };
#[doc(hidden)]
pub struct Field_rw_crashed_at;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RegisteredBgWorker, rw_crashed_at) == 560
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rw_crashed_at>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RegisteredBgWorker>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 560;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RegisteredBgWorker, <<c::expression::CRecord<crate::__pgrx_c_bindings::RegisteredBgWorker> as c::expression::OrdinaryField<Field_rw_crashed_at>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rw_crashed_at) };
#[doc(hidden)]
pub struct Field_rw_shmem_slot;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RegisteredBgWorker, rw_shmem_slot) == 568
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rw_shmem_slot>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RegisteredBgWorker>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 568;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RegisteredBgWorker, <<c::expression::CRecord<crate::__pgrx_c_bindings::RegisteredBgWorker> as c::expression::OrdinaryField<Field_rw_shmem_slot>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rw_shmem_slot) };
#[doc(hidden)]
pub struct Field_rw_terminate;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RegisteredBgWorker, rw_terminate) == 572
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rw_terminate>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RegisteredBgWorker>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 572;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RegisteredBgWorker, <<c::expression::CRecord<crate::__pgrx_c_bindings::RegisteredBgWorker> as c::expression::OrdinaryField<Field_rw_terminate>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rw_terminate) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ReindexParams>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ReindexParams>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ReindexParams {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReindexParams, options) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReindexParams>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReindexParams, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReindexParams> as c::expression::OrdinaryField<Field_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
#[doc(hidden)]
pub struct Field_tablespaceOid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReindexParams, tablespaceOid) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tablespaceOid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReindexParams>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReindexParams, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReindexParams> as c::expression::OrdinaryField<Field_tablespaceOid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tablespaceOid) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ReindexStmt>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ReindexStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ReindexStmt {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReindexStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReindexStmt>
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
const _: Projection<crate::__pgrx_c_bindings::ReindexStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReindexStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReindexStmt, kind) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_kind>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReindexStmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ReindexObjectType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReindexStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReindexStmt> as c::expression::OrdinaryField<Field_kind>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).kind) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReindexStmt, params) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_params>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReindexStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReindexStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReindexStmt> as c::expression::OrdinaryField<Field_params>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).params) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_node;
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RelFileNodeBackend>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RelFileNodeBackend>() == 4);
};
#[cfg(feature = "pg15")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::RelFileNodeBackend {}
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelFileNodeBackend, node) == 0);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_node>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelFileNodeBackend>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::RelFileNode>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelFileNodeBackend, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelFileNodeBackend> as c::expression::OrdinaryField<Field_node>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).node) };
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelFileNodeBackend, backend) == 12);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_backend>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelFileNodeBackend>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelFileNodeBackend, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelFileNodeBackend> as c::expression::OrdinaryField<Field_backend>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).backend) };
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RelOptInfo>() == 416);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RelOptInfo>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::RelOptInfo {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
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
const _: Projection<crate::__pgrx_c_bindings::RelOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_reloptkind;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, reloptkind) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_reloptkind>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__RelOptKind,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo> as c::expression::OrdinaryField<Field_reloptkind>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).reloptkind) };
#[doc(hidden)]
pub struct Field_consider_startup;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, consider_startup) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_consider_startup>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo> as c::expression::OrdinaryField<Field_consider_startup>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).consider_startup) };
#[doc(hidden)]
pub struct Field_consider_param_startup;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, consider_param_startup) == 25
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_consider_param_startup>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 25;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo> as c::expression::OrdinaryField<Field_consider_param_startup>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).consider_param_startup) };
#[doc(hidden)]
pub struct Field_consider_parallel;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, consider_parallel) == 26);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_consider_parallel>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 26;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo> as c::expression::OrdinaryField<Field_consider_parallel>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).consider_parallel) };
#[doc(hidden)]
pub struct Field_pathlist;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, pathlist) == 40);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pathlist>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo> as c::expression::OrdinaryField<Field_pathlist>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pathlist) };
#[doc(hidden)]
pub struct Field_ppilist;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, ppilist) == 48);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ppilist>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo> as c::expression::OrdinaryField<Field_ppilist>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ppilist) };
#[doc(hidden)]
pub struct Field_partial_pathlist;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, partial_pathlist) == 56);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_partial_pathlist>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo> as c::expression::OrdinaryField<Field_partial_pathlist>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).partial_pathlist) };
#[doc(hidden)]
pub struct Field_cheapest_parameterized_paths;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, cheapest_parameterized_paths)
        == 88
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cheapest_parameterized_paths>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo> as c::expression::OrdinaryField<Field_cheapest_parameterized_paths>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cheapest_parameterized_paths) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, relid) == 112);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo> as c::expression::OrdinaryField<Field_relid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relid) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, reltablespace) == 116);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_reltablespace>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 116;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo> as c::expression::OrdinaryField<Field_reltablespace>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).reltablespace) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, rtekind) == 120);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rtekind>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__RTEKind,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo> as c::expression::OrdinaryField<Field_rtekind>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rtekind) };
#[doc(hidden)]
pub struct Field_min_attr;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, min_attr) == 124);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_min_attr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 124;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo> as c::expression::OrdinaryField<Field_min_attr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).min_attr) };
#[doc(hidden)]
pub struct Field_max_attr;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, max_attr) == 126);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_max_attr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 126;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo> as c::expression::OrdinaryField<Field_max_attr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).max_attr) };
#[doc(hidden)]
pub struct Field_lateral_vars;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, lateral_vars) == 144);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lateral_vars>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo> as c::expression::OrdinaryField<Field_lateral_vars>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lateral_vars) };
#[doc(hidden)]
pub struct Field_indexlist;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, indexlist) == 160);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indexlist>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo> as c::expression::OrdinaryField<Field_indexlist>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indexlist) };
#[doc(hidden)]
pub struct Field_statlist;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, statlist) == 168);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_statlist>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 168;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo> as c::expression::OrdinaryField<Field_statlist>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).statlist) };
#[cfg(feature = "pg15")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, pages) == 176);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pages>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 176;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo> as c::expression::OrdinaryField<Field_pages>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pages) };
#[doc(hidden)]
pub struct Field_subplan_params;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, subplan_params) == 216);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_subplan_params>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 216;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo> as c::expression::OrdinaryField<Field_subplan_params>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).subplan_params) };
#[doc(hidden)]
pub struct Field_rel_parallel_workers;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, rel_parallel_workers) == 224
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rel_parallel_workers>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 224;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo> as c::expression::OrdinaryField<Field_rel_parallel_workers>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rel_parallel_workers) };
#[doc(hidden)]
pub struct Field_amflags;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, amflags) == 228);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_amflags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 228;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo> as c::expression::OrdinaryField<Field_amflags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).amflags) };
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, serverid) == 232);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_serverid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 232;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo> as c::expression::OrdinaryField<Field_serverid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).serverid) };
#[cfg(feature = "pg15")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, userid) == 236);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_userid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 236;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo> as c::expression::OrdinaryField<Field_userid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).userid) };
#[doc(hidden)]
pub struct Field_useridiscurrent;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, useridiscurrent) == 240);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_useridiscurrent>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 240;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo> as c::expression::OrdinaryField<Field_useridiscurrent>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).useridiscurrent) };
#[doc(hidden)]
pub struct Field_unique_for_rels;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, unique_for_rels) == 264);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_unique_for_rels>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 264;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo> as c::expression::OrdinaryField<Field_unique_for_rels>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).unique_for_rels) };
#[doc(hidden)]
pub struct Field_non_unique_for_rels;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, non_unique_for_rels) == 272
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_non_unique_for_rels>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 272;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo> as c::expression::OrdinaryField<Field_non_unique_for_rels>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).non_unique_for_rels) };
#[doc(hidden)]
pub struct Field_baserestrictinfo;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, baserestrictinfo) == 280);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_baserestrictinfo>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 280;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo> as c::expression::OrdinaryField<Field_baserestrictinfo>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).baserestrictinfo) };
#[doc(hidden)]
pub struct Field_baserestrict_min_security;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, baserestrict_min_security) == 304
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_baserestrict_min_security>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 304;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo> as c::expression::OrdinaryField<Field_baserestrict_min_security>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).baserestrict_min_security) };
#[doc(hidden)]
pub struct Field_joininfo;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, joininfo) == 312);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_joininfo>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 312;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo> as c::expression::OrdinaryField<Field_joininfo>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).joininfo) };
#[doc(hidden)]
pub struct Field_has_eclass_joins;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, has_eclass_joins) == 320);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_has_eclass_joins>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 320;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo> as c::expression::OrdinaryField<Field_has_eclass_joins>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).has_eclass_joins) };
#[doc(hidden)]
pub struct Field_consider_partitionwise_join;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, consider_partitionwise_join)
        == 321
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_consider_partitionwise_join>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 321;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo> as c::expression::OrdinaryField<Field_consider_partitionwise_join>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).consider_partitionwise_join) };
#[doc(hidden)]
pub struct Field_part_scheme;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, part_scheme) == 336);
const _: () = {
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::PartitionSchemeData>() == 8);
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::PartitionSchemeData>() == 8);
};
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_part_scheme>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::PartitionSchemeData>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 336;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo> as c::expression::OrdinaryField<Field_part_scheme>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).part_scheme) };
#[cfg(feature = "pg15")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, nparts) == 344);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nparts>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 344;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo> as c::expression::OrdinaryField<Field_nparts>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nparts) };
#[doc(hidden)]
pub struct Field_boundinfo;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, boundinfo) == 352);
const _: () = {
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::PartitionBoundInfoData>() == 8);
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::PartitionBoundInfoData>() == 8);
};
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_boundinfo>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::PartitionBoundInfoData>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 352;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo> as c::expression::OrdinaryField<Field_boundinfo>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).boundinfo) };
#[doc(hidden)]
pub struct Field_partbounds_merged;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, partbounds_merged) == 360
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_partbounds_merged>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 360;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo> as c::expression::OrdinaryField<Field_partbounds_merged>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).partbounds_merged) };
#[doc(hidden)]
pub struct Field_partition_qual;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, partition_qual) == 368);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_partition_qual>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 368;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo> as c::expression::OrdinaryField<Field_partition_qual>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).partition_qual) };
#[doc(hidden)]
pub struct Field_part_rels;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, part_rels) == 376);
const _: () = {
    assert!(::core::mem::size_of::<*mut *mut crate::__pgrx_c_bindings::RelOptInfo>() == 8);
    assert!(::core::mem::align_of::<*mut *mut crate::__pgrx_c_bindings::RelOptInfo>() == 8);
};
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_part_rels>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CPointer<
            c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>,
            c::expression::ReadWrite,
        >,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 376;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo> as c::expression::OrdinaryField<Field_part_rels>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).part_rels) };
#[doc(hidden)]
pub struct Field_partexprs;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, partexprs) == 400);
const _: () = {
    assert!(::core::mem::size_of::<*mut *mut crate::__pgrx_c_bindings::List>() == 8);
    assert!(::core::mem::align_of::<*mut *mut crate::__pgrx_c_bindings::List>() == 8);
};
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_partexprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CPointer<
            c::expression::CRecord<crate::__pgrx_c_bindings::List>,
            c::expression::ReadWrite,
        >,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 400;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo> as c::expression::OrdinaryField<Field_partexprs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).partexprs) };
#[doc(hidden)]
pub struct Field_nullable_partexprs;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, nullable_partexprs) == 408
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nullable_partexprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CPointer<
            c::expression::CRecord<crate::__pgrx_c_bindings::List>,
            c::expression::ReadWrite,
        >,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 408;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo> as c::expression::OrdinaryField<Field_nullable_partexprs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nullable_partexprs) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RelabelType>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RelabelType>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::RelabelType {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelabelType, resulttype) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resulttype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelabelType>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelabelType, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelabelType> as c::expression::OrdinaryField<Field_resulttype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).resulttype) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelabelType, resulttypmod) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resulttypmod>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelabelType>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelabelType, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelabelType> as c::expression::OrdinaryField<Field_resulttypmod>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).resulttypmod) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelabelType, resultcollid) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resultcollid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelabelType>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelabelType, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelabelType> as c::expression::OrdinaryField<Field_resultcollid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).resultcollid) };
#[doc(hidden)]
pub struct Field_relabelformat;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelabelType, relabelformat) == 28);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relabelformat>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelabelType>
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
const _: Projection<crate::__pgrx_c_bindings::RelabelType, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelabelType> as c::expression::OrdinaryField<Field_relabelformat>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relabelformat) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelabelType, location) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelabelType>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelabelType, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelabelType> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[doc(hidden)]
pub struct Field_rd_smgr;
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RelationData>() == 464);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RelationData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::RelationData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_smgr) == 16);
const _: () = {
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::SMgrRelationData>() == 8);
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::SMgrRelationData>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_smgr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::SMgrRelationData>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelationData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData> as c::expression::OrdinaryField<Field_rd_smgr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rd_smgr) };
#[doc(hidden)]
pub struct Field_rd_refcnt;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_refcnt) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_refcnt>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelationData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData> as c::expression::OrdinaryField<Field_rd_refcnt>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rd_refcnt) };
#[doc(hidden)]
pub struct Field_rd_backend;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_backend) == 28);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_backend>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelationData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData> as c::expression::OrdinaryField<Field_rd_backend>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rd_backend) };
#[doc(hidden)]
pub struct Field_rd_islocaltemp;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_islocaltemp) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_islocaltemp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelationData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData> as c::expression::OrdinaryField<Field_rd_islocaltemp>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rd_islocaltemp) };
#[doc(hidden)]
pub struct Field_rd_isnailed;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_isnailed) == 33);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_isnailed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 33;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelationData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData> as c::expression::OrdinaryField<Field_rd_isnailed>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rd_isnailed) };
#[doc(hidden)]
pub struct Field_rd_isvalid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_isvalid) == 34);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_isvalid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 34;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelationData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData> as c::expression::OrdinaryField<Field_rd_isvalid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rd_isvalid) };
#[doc(hidden)]
pub struct Field_rd_indexvalid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_indexvalid) == 35);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_indexvalid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 35;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelationData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData> as c::expression::OrdinaryField<Field_rd_indexvalid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rd_indexvalid) };
#[doc(hidden)]
pub struct Field_rd_statvalid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_statvalid) == 36);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_statvalid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelationData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData> as c::expression::OrdinaryField<Field_rd_statvalid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rd_statvalid) };
#[doc(hidden)]
pub struct Field_rd_createSubid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_createSubid) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_createSubid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelationData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData> as c::expression::OrdinaryField<Field_rd_createSubid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rd_createSubid) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_rd_newRelfilenodeSubid;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_newRelfilenodeSubid) == 44
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_newRelfilenodeSubid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelationData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData> as c::expression::OrdinaryField<Field_rd_newRelfilenodeSubid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rd_newRelfilenodeSubid) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_rd_firstRelfilenodeSubid;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_firstRelfilenodeSubid) == 48
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_firstRelfilenodeSubid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelationData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData> as c::expression::OrdinaryField<Field_rd_firstRelfilenodeSubid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rd_firstRelfilenodeSubid) };
#[doc(hidden)]
pub struct Field_rd_droppedSubid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_droppedSubid) == 52);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_droppedSubid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelationData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData> as c::expression::OrdinaryField<Field_rd_droppedSubid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rd_droppedSubid) };
#[doc(hidden)]
pub struct Field_rd_rel;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_rel) == 56);
const _: () = {
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::FormData_pg_class>() == 8);
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::FormData_pg_class>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_rel>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_class>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelationData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData> as c::expression::OrdinaryField<Field_rd_rel>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rd_rel) };
#[doc(hidden)]
pub struct Field_rd_att;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_att) == 64);
const _: () = {
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::TupleDescData>() == 8);
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::TupleDescData>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_att>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::TupleDescData>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelationData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData> as c::expression::OrdinaryField<Field_rd_att>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rd_att) };
#[doc(hidden)]
pub struct Field_rd_id;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_id) == 72);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelationData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData> as c::expression::OrdinaryField<Field_rd_id>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rd_id) };
#[doc(hidden)]
pub struct Field_rd_fkeylist;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_fkeylist) == 120);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_fkeylist>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelationData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData> as c::expression::OrdinaryField<Field_rd_fkeylist>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rd_fkeylist) };
#[doc(hidden)]
pub struct Field_rd_fkeyvalid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_fkeyvalid) == 128);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_fkeyvalid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelationData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData> as c::expression::OrdinaryField<Field_rd_fkeyvalid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rd_fkeyvalid) };
#[doc(hidden)]
pub struct Field_rd_partdesc_nodetached_xmin;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_partdesc_nodetached_xmin)
        == 184
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_partdesc_nodetached_xmin>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 184;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelationData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData> as c::expression::OrdinaryField<Field_rd_partdesc_nodetached_xmin>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rd_partdesc_nodetached_xmin) };
#[doc(hidden)]
pub struct Field_rd_partcheck;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_partcheck) == 192);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_partcheck>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 192;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelationData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData> as c::expression::OrdinaryField<Field_rd_partcheck>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rd_partcheck) };
#[doc(hidden)]
pub struct Field_rd_partcheckvalid;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_partcheckvalid) == 200
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_partcheckvalid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 200;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelationData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData> as c::expression::OrdinaryField<Field_rd_partcheckvalid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rd_partcheckvalid) };
#[doc(hidden)]
pub struct Field_rd_indexlist;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_indexlist) == 216);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_indexlist>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 216;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelationData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData> as c::expression::OrdinaryField<Field_rd_indexlist>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rd_indexlist) };
#[doc(hidden)]
pub struct Field_rd_pkindex;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_pkindex) == 224);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_pkindex>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 224;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelationData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData> as c::expression::OrdinaryField<Field_rd_pkindex>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rd_pkindex) };
#[doc(hidden)]
pub struct Field_rd_replidindex;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_replidindex) == 228);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_replidindex>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 228;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelationData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData> as c::expression::OrdinaryField<Field_rd_replidindex>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rd_replidindex) };
#[doc(hidden)]
pub struct Field_rd_statlist;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_statlist) == 232);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_statlist>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 232;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelationData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData> as c::expression::OrdinaryField<Field_rd_statlist>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rd_statlist) };
#[doc(hidden)]
pub struct Field_rd_options;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_options) == 280);
const _: () = {
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::varlena>() == 8);
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::varlena>() == 8);
};
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::varlena>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 280;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelationData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData> as c::expression::OrdinaryField<Field_rd_options>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rd_options) };
#[doc(hidden)]
pub struct Field_rd_amhandler;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_amhandler) == 288);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_amhandler>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 288;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelationData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData> as c::expression::OrdinaryField<Field_rd_amhandler>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rd_amhandler) };
#[doc(hidden)]
pub struct Field_rd_tableam;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_tableam) == 296);
const _: () = {
    assert!(::core::mem::size_of::<*const crate::__pgrx_c_bindings::TableAmRoutine>() == 8);
    assert!(::core::mem::align_of::<*const crate::__pgrx_c_bindings::TableAmRoutine>() == 8);
};
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_tableam>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>,
        c::expression::ReadOnly,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 296;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelationData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData> as c::expression::OrdinaryField<Field_rd_tableam>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rd_tableam) };
#[doc(hidden)]
pub struct Field_rd_index;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_index) == 304);
const _: () = {
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::FormData_pg_index>() == 8);
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::FormData_pg_index>() == 8);
};
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_index>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::FormData_pg_index>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 304;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelationData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData> as c::expression::OrdinaryField<Field_rd_index>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rd_index) };
#[doc(hidden)]
pub struct Field_rd_indexprs;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_indexprs) == 376);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_indexprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 376;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelationData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData> as c::expression::OrdinaryField<Field_rd_indexprs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rd_indexprs) };
#[doc(hidden)]
pub struct Field_rd_indpred;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_indpred) == 384);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_indpred>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 384;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelationData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData> as c::expression::OrdinaryField<Field_rd_indpred>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rd_indpred) };
#[doc(hidden)]
pub struct Field_rd_toastoid;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, rd_toastoid) == 448);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rd_toastoid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 448;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelationData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData> as c::expression::OrdinaryField<Field_rd_toastoid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rd_toastoid) };
#[doc(hidden)]
pub struct Field_pgstat_enabled;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, pgstat_enabled) == 452);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pgstat_enabled>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 452;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelationData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData> as c::expression::OrdinaryField<Field_pgstat_enabled>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pgstat_enabled) };
#[doc(hidden)]
pub struct Field_pgstat_info;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelationData, pgstat_info) == 456);
const _: () = {
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::PgStat_TableStatus>() == 8);
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::PgStat_TableStatus>() == 8);
};
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pgstat_info>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_TableStatus>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 456;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelationData, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData> as c::expression::OrdinaryField<Field_pgstat_info>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pgstat_info) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RenameStmt>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RenameStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::RenameStmt {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RenameStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RenameStmt>
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
const _: Projection<crate::__pgrx_c_bindings::RenameStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::RenameStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_renameType;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RenameStmt, renameType) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_renameType>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RenameStmt>
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
const _: Projection<crate::__pgrx_c_bindings::RenameStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::RenameStmt> as c::expression::OrdinaryField<Field_renameType>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).renameType) };
#[doc(hidden)]
pub struct Field_relationType;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RenameStmt, relationType) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relationType>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RenameStmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ObjectType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RenameStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::RenameStmt> as c::expression::OrdinaryField<Field_relationType>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relationType) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RenameStmt, behavior) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_behavior>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RenameStmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__DropBehavior,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RenameStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::RenameStmt> as c::expression::OrdinaryField<Field_behavior>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).behavior) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RenameStmt, missing_ok) == 52);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_missing_ok>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RenameStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RenameStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::RenameStmt> as c::expression::OrdinaryField<Field_missing_ok>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).missing_ok) };
#[doc(hidden)]
pub struct Field_by_txn_last_xid;
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ReorderBuffer>() == 336);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ReorderBuffer>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ReorderBuffer {}
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBuffer, by_txn_last_xid) == 40
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_by_txn_last_xid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBuffer, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer> as c::expression::OrdinaryField<Field_by_txn_last_xid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).by_txn_last_xid) };
#[doc(hidden)]
pub struct Field_output_rewrites;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBuffer, output_rewrites) == 200
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_output_rewrites>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 200;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBuffer, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer> as c::expression::OrdinaryField<Field_output_rewrites>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).output_rewrites) };
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBuffer, context) == 208);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_context>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::MemoryContextData>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 208;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBuffer, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer> as c::expression::OrdinaryField<Field_context>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).context) };
#[doc(hidden)]
pub struct Field_current_restart_decoding_lsn;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBuffer, current_restart_decoding_lsn)
        == 240
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_current_restart_decoding_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 240;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBuffer, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer> as c::expression::OrdinaryField<Field_current_restart_decoding_lsn>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).current_restart_decoding_lsn) };
#[doc(hidden)]
pub struct Field_outbufsize;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBuffer, outbufsize) == 256);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_outbufsize>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 256;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBuffer, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer> as c::expression::OrdinaryField<Field_outbufsize>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).outbufsize) };
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBuffer, size) == 264);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 264;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBuffer, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer> as c::expression::OrdinaryField<Field_size>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).size) };
#[doc(hidden)]
pub struct Field_spillTxns;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBuffer, spillTxns) == 272);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_spillTxns>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 272;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBuffer, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer> as c::expression::OrdinaryField<Field_spillTxns>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).spillTxns) };
#[doc(hidden)]
pub struct Field_spillCount;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBuffer, spillCount) == 280);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_spillCount>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 280;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBuffer, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer> as c::expression::OrdinaryField<Field_spillCount>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).spillCount) };
#[doc(hidden)]
pub struct Field_spillBytes;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBuffer, spillBytes) == 288);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_spillBytes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 288;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBuffer, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer> as c::expression::OrdinaryField<Field_spillBytes>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).spillBytes) };
#[doc(hidden)]
pub struct Field_streamTxns;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBuffer, streamTxns) == 296);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_streamTxns>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 296;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBuffer, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer> as c::expression::OrdinaryField<Field_streamTxns>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).streamTxns) };
#[doc(hidden)]
pub struct Field_streamCount;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBuffer, streamCount) == 304);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_streamCount>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 304;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBuffer, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer> as c::expression::OrdinaryField<Field_streamCount>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).streamCount) };
#[doc(hidden)]
pub struct Field_streamBytes;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBuffer, streamBytes) == 312);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_streamBytes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 312;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBuffer, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer> as c::expression::OrdinaryField<Field_streamBytes>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).streamBytes) };
#[doc(hidden)]
pub struct Field_totalTxns;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBuffer, totalTxns) == 320);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_totalTxns>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 320;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBuffer, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer> as c::expression::OrdinaryField<Field_totalTxns>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).totalTxns) };
#[doc(hidden)]
pub struct Field_totalBytes;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBuffer, totalBytes) == 328);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_totalBytes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 328;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBuffer, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer> as c::expression::OrdinaryField<Field_totalBytes>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).totalBytes) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ReorderBufferChange>() == 80);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ReorderBufferChange>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ReorderBufferChange {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBufferChange, lsn) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferChange>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBufferChange, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferChange> as c::expression::OrdinaryField<Field_lsn>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lsn) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBufferChange, action) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_action>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferChange>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ReorderBufferChangeType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBufferChange, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferChange> as c::expression::OrdinaryField<Field_action>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).action) };
#[doc(hidden)]
pub struct Field_origin_id;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBufferChange, origin_id) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_origin_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferChange>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBufferChange, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferChange> as c::expression::OrdinaryField<Field_origin_id>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).origin_id) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBufferChange, data) == 32);
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1>() == 32
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1>() == 8
    );
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_data>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferChange>
{
    type Member =
        c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBufferChange, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferChange> as c::expression::OrdinaryField<Field_data>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).data) };
#[doc(hidden)]
pub struct Field_nrelids;
const _: () = {
    assert!(
        ::core::mem::size_of::<
            crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_2,
        >() == 24
    );
    assert!(
        ::core::mem::align_of::<
            crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_2,
        >() == 8
    );
};
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_2
{
}
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_2,
        nrelids
    ) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nrelids>
    for c::expression::CRecord<
        crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_2,
    >
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_2,
    <<c::expression::CRecord<
        crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_2,
    > as c::expression::OrdinaryField<Field_nrelids>>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nrelids) };
#[doc(hidden)]
pub struct Field_cascade;
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_2,
        cascade
    ) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cascade>
    for c::expression::CRecord<
        crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_2,
    >
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_2,
    <<c::expression::CRecord<
        crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_2,
    > as c::expression::OrdinaryField<Field_cascade>>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cascade) };
#[doc(hidden)]
pub struct Field_restart_seqs;
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_2,
        restart_seqs
    ) == 9
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_restart_seqs>
    for c::expression::CRecord<
        crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_2,
    >
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 9;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_2, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_2> as c::expression::OrdinaryField<Field_restart_seqs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).restart_seqs) };
#[doc(hidden)]
pub struct Field_message_size;
const _: () = {
    assert!(
        ::core::mem::size_of::<
            crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_3,
        >() == 24
    );
    assert!(
        ::core::mem::align_of::<
            crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_3,
        >() == 8
    );
};
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_3
{
}
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_3,
        message_size
    ) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_message_size>
    for c::expression::CRecord<
        crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_3,
    >
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_3, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_3> as c::expression::OrdinaryField<Field_message_size>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).message_size) };
#[doc(hidden)]
pub struct Field_cmin;
const _: () = {
    assert!(
        ::core::mem::size_of::<
            crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_4,
        >() == 32
    );
    assert!(
        ::core::mem::align_of::<
            crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_4,
        >() == 4
    );
};
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_4
{
}
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_4,
        cmin
    ) == 20
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cmin>
    for c::expression::CRecord<
        crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_4,
    >
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_4,
    <<c::expression::CRecord<
        crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_4,
    > as c::expression::OrdinaryField<Field_cmin>>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cmin) };
#[doc(hidden)]
pub struct Field_cmax;
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_4,
        cmax
    ) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cmax>
    for c::expression::CRecord<
        crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_4,
    >
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_4,
    <<c::expression::CRecord<
        crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_4,
    > as c::expression::OrdinaryField<Field_cmax>>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cmax) };
#[doc(hidden)]
pub struct Field_combocid;
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_4,
        combocid
    ) == 28
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_combocid>
    for c::expression::CRecord<
        crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_4,
    >
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_4,
    <<c::expression::CRecord<
        crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_4,
    > as c::expression::OrdinaryField<Field_combocid>>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).combocid) };
#[doc(hidden)]
pub struct Field_ninvalidations;
const _: () = {
    assert!(
        ::core::mem::size_of::<
            crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_5,
        >() == 16
    );
    assert!(
        ::core::mem::align_of::<
            crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_5,
        >() == 8
    );
};
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_5
{
}
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_5,
        ninvalidations
    ) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ninvalidations>
    for c::expression::CRecord<
        crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_5,
    >
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_5, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_5> as c::expression::OrdinaryField<Field_ninvalidations>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ninvalidations) };
#[doc(hidden)]
pub struct Field_clear_toast_afterwards;
const _: () = {
    assert!(
        ::core::mem::size_of::<
            crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_1,
        >() == 32
    );
    assert!(
        ::core::mem::align_of::<
            crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_1,
        >() == 8
    );
};
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_1
{
}
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_1,
        clear_toast_afterwards
    ) == 12
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_clear_toast_afterwards>
    for c::expression::CRecord<
        crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_1,
    >
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_1, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferChange__bindgen_ty_1__bindgen_ty_1> as c::expression::OrdinaryField<Field_clear_toast_afterwards>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).clear_toast_afterwards) };
#[doc(hidden)]
pub struct Field_txn_flags;
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ReorderBufferTXN>() == 304);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ReorderBufferTXN>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ReorderBufferTXN {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBufferTXN, txn_flags) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_txn_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBufferTXN, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN> as c::expression::OrdinaryField<Field_txn_flags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).txn_flags) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBufferTXN, xid) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_xid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBufferTXN, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN> as c::expression::OrdinaryField<Field_xid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).xid) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBufferTXN, toplevel_xid) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_toplevel_xid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBufferTXN, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN> as c::expression::OrdinaryField<Field_toplevel_xid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).toplevel_xid) };
#[doc(hidden)]
pub struct Field_first_lsn;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBufferTXN, first_lsn) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_first_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBufferTXN, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN> as c::expression::OrdinaryField<Field_first_lsn>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).first_lsn) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBufferTXN, final_lsn) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_final_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBufferTXN, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN> as c::expression::OrdinaryField<Field_final_lsn>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).final_lsn) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBufferTXN, end_lsn) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_end_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBufferTXN, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN> as c::expression::OrdinaryField<Field_end_lsn>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).end_lsn) };
#[doc(hidden)]
pub struct Field_restart_decoding_lsn;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBufferTXN, restart_decoding_lsn) == 56
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_restart_decoding_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBufferTXN, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN> as c::expression::OrdinaryField<Field_restart_decoding_lsn>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).restart_decoding_lsn) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBufferTXN, origin_id) == 64);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_origin_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBufferTXN, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN> as c::expression::OrdinaryField<Field_origin_id>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).origin_id) };
#[doc(hidden)]
pub struct Field_origin_lsn;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBufferTXN, origin_lsn) == 72);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_origin_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBufferTXN, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN> as c::expression::OrdinaryField<Field_origin_lsn>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).origin_lsn) };
#[doc(hidden)]
pub struct Field_base_snapshot_lsn;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBufferTXN, base_snapshot_lsn) == 96
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_base_snapshot_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBufferTXN, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN> as c::expression::OrdinaryField<Field_base_snapshot_lsn>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).base_snapshot_lsn) };
#[doc(hidden)]
pub struct Field_command_id;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBufferTXN, command_id) == 128);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_command_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBufferTXN, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN> as c::expression::OrdinaryField<Field_command_id>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).command_id) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBufferTXN, nentries) == 136);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nentries>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 136;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBufferTXN, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN> as c::expression::OrdinaryField<Field_nentries>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nentries) };
#[doc(hidden)]
pub struct Field_nentries_mem;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBufferTXN, nentries_mem) == 144
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nentries_mem>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBufferTXN, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN> as c::expression::OrdinaryField<Field_nentries_mem>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nentries_mem) };
#[doc(hidden)]
pub struct Field_ntuplecids;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBufferTXN, ntuplecids) == 184);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ntuplecids>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 184;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBufferTXN, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN> as c::expression::OrdinaryField<Field_ntuplecids>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ntuplecids) };
#[doc(hidden)]
pub struct Field_nsubtxns;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBufferTXN, nsubtxns) == 224);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nsubtxns>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 224;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBufferTXN, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN> as c::expression::OrdinaryField<Field_nsubtxns>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nsubtxns) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBufferTXN, ninvalidations) == 228
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ninvalidations>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 228;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBufferTXN, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN> as c::expression::OrdinaryField<Field_ninvalidations>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ninvalidations) };
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBufferTXN, size) == 256);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 256;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBufferTXN, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN> as c::expression::OrdinaryField<Field_size>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).size) };
#[doc(hidden)]
pub struct Field_total_size;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBufferTXN, total_size) == 264);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_total_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 264;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBufferTXN, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN> as c::expression::OrdinaryField<Field_total_size>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).total_size) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
#[doc(hidden)]
pub struct Field_concurrent_abort;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBufferTXN, concurrent_abort) == 272
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_concurrent_abort>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 272;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBufferTXN, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN> as c::expression::OrdinaryField<Field_concurrent_abort>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).concurrent_abort) };
#[doc(hidden)]
pub struct Field_ninvalidations_distributed;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBufferTXN, ninvalidations_distributed)
        == 288
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ninvalidations_distributed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 288;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBufferTXN, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN> as c::expression::OrdinaryField<Field_ninvalidations_distributed>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ninvalidations_distributed) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
#[doc(hidden)]
pub struct Field_alloc_tuple_size;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ReorderBufferTupleBuf>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ReorderBufferTupleBuf>() == 8);
};
#[cfg(any(feature = "pg15", feature = "pg16"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ReorderBufferTupleBuf {}
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBufferTupleBuf, alloc_tuple_size)
        == 32
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_alloc_tuple_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTupleBuf>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBufferTupleBuf, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTupleBuf> as c::expression::OrdinaryField<Field_alloc_tuple_size>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).alloc_tuple_size) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ReplicaIdentityStmt>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ReplicaIdentityStmt>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ReplicaIdentityStmt {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReplicaIdentityStmt, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReplicaIdentityStmt>
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
const _: Projection<crate::__pgrx_c_bindings::ReplicaIdentityStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReplicaIdentityStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_identity_type;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReplicaIdentityStmt, identity_type) == 4
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_identity_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReplicaIdentityStmt>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReplicaIdentityStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReplicaIdentityStmt> as c::expression::OrdinaryField<Field_identity_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).identity_type) };
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ReplicationSlot>() == 264);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ReplicationSlot>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ReplicationSlot {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReplicationSlot, mutex) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_mutex>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlot>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReplicationSlot, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlot> as c::expression::OrdinaryField<Field_mutex>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).mutex) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReplicationSlot, in_use) == 1);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_in_use>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlot>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReplicationSlot, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlot> as c::expression::OrdinaryField<Field_in_use>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).in_use) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
pub struct Field_active_pid;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReplicationSlot, active_pid) == 4);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_active_pid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlot>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReplicationSlot, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlot> as c::expression::OrdinaryField<Field_active_pid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).active_pid) };
#[doc(hidden)]
pub struct Field_just_dirtied;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReplicationSlot, just_dirtied) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_just_dirtied>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlot>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReplicationSlot, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlot> as c::expression::OrdinaryField<Field_just_dirtied>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).just_dirtied) };
#[doc(hidden)]
pub struct Field_dirty;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReplicationSlot, dirty) == 9);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dirty>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlot>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 9;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReplicationSlot, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlot> as c::expression::OrdinaryField<Field_dirty>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dirty) };
#[doc(hidden)]
pub struct Field_effective_xmin;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReplicationSlot, effective_xmin) == 12
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_effective_xmin>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlot>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReplicationSlot, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlot> as c::expression::OrdinaryField<Field_effective_xmin>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).effective_xmin) };
#[doc(hidden)]
pub struct Field_effective_catalog_xmin;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReplicationSlot, effective_catalog_xmin)
        == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_effective_catalog_xmin>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlot>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReplicationSlot, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlot> as c::expression::OrdinaryField<Field_effective_catalog_xmin>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).effective_catalog_xmin) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReplicationSlot, data) == 24);
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ReplicationSlotPersistentData>() == 184
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ReplicationSlotPersistentData>() == 8
    );
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_data>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlot>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlotPersistentData>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReplicationSlot, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlot> as c::expression::OrdinaryField<Field_data>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).data) };
#[doc(hidden)]
pub struct Field_candidate_catalog_xmin;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReplicationSlot, candidate_catalog_xmin)
        == 236
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_candidate_catalog_xmin>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlot>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 236;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReplicationSlot, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlot> as c::expression::OrdinaryField<Field_candidate_catalog_xmin>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).candidate_catalog_xmin) };
#[doc(hidden)]
pub struct Field_candidate_xmin_lsn;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReplicationSlot, candidate_xmin_lsn) == 240
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_candidate_xmin_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlot>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 240;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReplicationSlot, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlot> as c::expression::OrdinaryField<Field_candidate_xmin_lsn>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).candidate_xmin_lsn) };
#[doc(hidden)]
pub struct Field_candidate_restart_valid;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReplicationSlot, candidate_restart_valid)
        == 248
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_candidate_restart_valid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlot>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 248;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReplicationSlot, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlot> as c::expression::OrdinaryField<Field_candidate_restart_valid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).candidate_restart_valid) };
#[doc(hidden)]
pub struct Field_candidate_restart_lsn;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReplicationSlot, candidate_restart_lsn)
        == 256
);
