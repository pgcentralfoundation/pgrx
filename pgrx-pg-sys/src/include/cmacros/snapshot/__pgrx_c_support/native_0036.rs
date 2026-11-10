// Shared generated C macro support.

#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SnapBuildOnDisk, <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuildOnDisk> as c::expression::OrdinaryField<Field_magic>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).magic) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_checksum;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapBuildOnDisk, checksum) == 4);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_checksum>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuildOnDisk>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SnapBuildOnDisk, <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuildOnDisk> as c::expression::OrdinaryField<Field_checksum>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).checksum) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapBuildOnDisk, version) == 8);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_version>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuildOnDisk>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SnapBuildOnDisk, <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuildOnDisk> as c::expression::OrdinaryField<Field_version>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).version) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapBuildOnDisk, length) == 12);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_length>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuildOnDisk>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SnapBuildOnDisk, <<c::expression::CRecord<crate::__pgrx_c_bindings::SnapBuildOnDisk> as c::expression::OrdinaryField<Field_length>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).length) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SnapshotData>() == 104);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SnapshotData>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SnapshotData, snapXactCompletionCount) == 96
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_snapXactCompletionCount>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_reverse_sort;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SortGroupClause, reverse_sort) == 16);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_reverse_sort>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SortGroupClause>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SortGroupClause, <<c::expression::CRecord<crate::__pgrx_c_bindings::SortGroupClause> as c::expression::OrdinaryField<Field_reverse_sort>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).reverse_sort) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SortGroupClause, nulls_first) == 17);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nulls_first>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SortGroupClause>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 17;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SortGroupClause, hashable) == 18);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hashable>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SortGroupClause>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 18;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_argtype;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::StatsArgInfo>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::StatsArgInfo>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::StatsArgInfo {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::StatsArgInfo, argtype) == 8);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_argtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::StatsArgInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::StatsArgInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::StatsArgInfo> as c::expression::OrdinaryField<Field_argtype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).argtype) };
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::StdRdOptions>() == 128);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::StdRdOptions>() == 8);
};
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::StdRdOptions, user_catalog_table) == 104
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_user_catalog_table>
    for c::expression::CRecord<crate::__pgrx_c_bindings::StdRdOptions>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::StdRdOptions, parallel_workers) == 108
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parallel_workers>
    for c::expression::CRecord<crate::__pgrx_c_bindings::StdRdOptions>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 108;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::StdRdOptions, vacuum_index_cleanup) == 112
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vacuum_index_cleanup>
    for c::expression::CRecord<crate::__pgrx_c_bindings::StdRdOptions>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__StdRdOptIndexCleanup,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::StdRdOptions, vacuum_truncate) == 116
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vacuum_truncate>
    for c::expression::CRecord<crate::__pgrx_c_bindings::StdRdOptions>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 116;
}
#[cfg(feature = "pg18")]
#[doc(hidden)]
pub struct Field_vacuum_truncate_set;
#[cfg(feature = "pg18")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::StdRdOptions, vacuum_truncate_set) == 117
);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vacuum_truncate_set>
    for c::expression::CRecord<crate::__pgrx_c_bindings::StdRdOptions>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 117;
}
#[cfg(feature = "pg18")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::StdRdOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::StdRdOptions> as c::expression::OrdinaryField<Field_vacuum_truncate_set>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).vacuum_truncate_set) };
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SubPlanState>() == 192);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SubPlanState>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubPlanState, curArray) == 48);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_curArray>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubPlanState>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, crate::__pgrx_c_bindings::Datum>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubPlanState, havehashrows) == 96);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_havehashrows>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubPlanState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubPlanState, havenullrows) == 97);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_havenullrows>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubPlanState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 97;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubPlanState, numCols) == 128);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numCols>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubPlanState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SupportRequestModifyInPlace>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SupportRequestModifyInPlace>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SupportRequestModifyInPlace {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SupportRequestModifyInPlace, type_) == 0
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestModifyInPlace>
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
const _: Projection<crate::__pgrx_c_bindings::SupportRequestModifyInPlace, <<c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestModifyInPlace> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SupportRequestModifyInPlace, funcid) == 4
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_funcid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestModifyInPlace>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SupportRequestModifyInPlace, <<c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestModifyInPlace> as c::expression::OrdinaryField<Field_funcid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).funcid) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SupportRequestModifyInPlace, args) == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_args>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestModifyInPlace>
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
const _: Projection<crate::__pgrx_c_bindings::SupportRequestModifyInPlace, <<c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestModifyInPlace> as c::expression::OrdinaryField<Field_args>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).args) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SupportRequestModifyInPlace, paramid) == 16
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_paramid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestModifyInPlace>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SupportRequestModifyInPlace, <<c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestModifyInPlace> as c::expression::OrdinaryField<Field_paramid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).paramid) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TBMIterateResult>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TBMIterateResult>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TBMIterateResult, lossy) == 4);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lossy>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TBMIterateResult>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TBMIterateResult, <<c::expression::CRecord<crate::__pgrx_c_bindings::TBMIterateResult> as c::expression::OrdinaryField<Field_lossy>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lossy) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TBMIterateResult, recheck) == 5);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_recheck>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TBMIterateResult>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 5;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TBMIterator>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TBMIterator>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TBMIterator {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TBMIterator, shared) == 0);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_shared>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TBMIterator>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TBMIterator, <<c::expression::CRecord<crate::__pgrx_c_bindings::TBMIterator> as c::expression::OrdinaryField<Field_shared>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).shared) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TBMIterator, i) == 8);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TBMIterator__bindgen_ty_1>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TBMIterator__bindgen_ty_1>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_i>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TBMIterator>
{
    type Member = c::expression::CRecord<crate::__pgrx_c_bindings::TBMIterator__bindgen_ty_1>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TBMIterator, <<c::expression::CRecord<crate::__pgrx_c_bindings::TBMIterator> as c::expression::OrdinaryField<Field_i>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).i) };
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TableScanDescData>() == 64);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TableScanDescData>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TableScanDescData, rs_flags) == 48);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rs_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableScanDescData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TidStoreIterResult>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TidStoreIterResult>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_has_generated_virtual;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleConstr, has_generated_virtual) == 30
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_has_generated_virtual>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TupleConstr>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 30;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TupleConstr, <<c::expression::CRecord<crate::__pgrx_c_bindings::TupleConstr> as c::expression::OrdinaryField<Field_has_generated_virtual>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).has_generated_virtual) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_compact_attrs;
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleDescData, compact_attrs) == 24);
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::OffsetField<Field_compact_attrs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TupleDescData>
{
    type Member = ();
    const OFFSET: usize =
        ::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleDescData, compact_attrs);
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_firstTuple;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TupleHashEntryData>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TupleHashEntryData>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleHashEntryData, firstTuple) == 0);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::MinimalTupleData>() == 8);
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::MinimalTupleData>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_firstTuple>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TupleHashEntryData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::MinimalTupleData>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TupleHashEntryData, <<c::expression::CRecord<crate::__pgrx_c_bindings::TupleHashEntryData> as c::expression::OrdinaryField<Field_firstTuple>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).firstTuple) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleHashEntryData, status) == 8);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_status>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TupleHashEntryData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleHashEntryData, hash) == 12);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hash>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TupleHashEntryData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TupleHashTableData>() == 112);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TupleHashTableData>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_additionalsize;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleHashTableData, additionalsize) == 64
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_additionalsize>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TupleHashTableData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TupleHashTableData, <<c::expression::CRecord<crate::__pgrx_c_bindings::TupleHashTableData> as c::expression::OrdinaryField<Field_additionalsize>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).additionalsize) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TypeCacheEntry>() == 496);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TypeCacheEntry>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, typarray) == 28);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typarray>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TypeCacheEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry> as c::expression::OrdinaryField<Field_typarray>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typarray) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, typcollation) == 32);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typcollation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, btree_opf) == 36);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_btree_opf>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, btree_opintype) == 40
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_btree_opintype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, hash_opf) == 44);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hash_opf>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, hash_opintype) == 48);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hash_opintype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, eq_opr) == 52);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_eq_opr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, lt_opr) == 56);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lt_opr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, gt_opr) == 60);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_gt_opr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 60;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, cmp_proc) == 64);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cmp_proc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, hash_proc) == 68);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hash_proc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 68;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, hash_extended_proc) == 72
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hash_extended_proc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, tupDesc_identifier) == 280
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tupDesc_identifier>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 280;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, rng_opfamily) == 296);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rng_opfamily>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 296;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, rng_collation) == 300
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rng_collation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 300;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, domainBaseType) == 456
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_domainBaseType>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 456;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, domainBaseTypmod) == 460
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_domainBaseTypmod>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 460;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeCacheEntry, flags) == 472);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeCacheEntry>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 472;
}
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::UniquePath>() == 112);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::UniquePath>() == 8);
};
#[cfg(feature = "pg18")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::UniquePath, umethod) == 88);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_umethod>
    for c::expression::CRecord<crate::__pgrx_c_bindings::UniquePath>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__UniquePathMethod,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::UniquePath, in_operators) == 96);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_in_operators>
    for c::expression::CRecord<crate::__pgrx_c_bindings::UniquePath>
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
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::UniquePath, uniq_exprs) == 104);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_uniq_exprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::UniquePath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::UniqueRelInfo>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::UniqueRelInfo>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::UniqueRelInfo {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::UniqueRelInfo, type_) == 0);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::UniqueRelInfo>
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
const _: Projection<crate::__pgrx_c_bindings::UniqueRelInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::UniqueRelInfo> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_self_join;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::UniqueRelInfo, self_join) == 16);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_self_join>
    for c::expression::CRecord<crate::__pgrx_c_bindings::UniqueRelInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::UniqueRelInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::UniqueRelInfo> as c::expression::OrdinaryField<Field_self_join>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).self_join) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_extra_clauses;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::UniqueRelInfo, extra_clauses) == 24);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_extra_clauses>
    for c::expression::CRecord<crate::__pgrx_c_bindings::UniqueRelInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::UniqueRelInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::UniqueRelInfo> as c::expression::OrdinaryField<Field_extra_clauses>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).extra_clauses) };
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::UpperUniquePath>() == 96);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::UpperUniquePath>() == 8);
};
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::UpperUniquePath, numkeys) == 88);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numkeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::UpperUniquePath>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Var>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Var>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Var, varreturningtype) == 36);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_varreturningtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Var>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__VarReturningType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Var,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Var> as c::expression::OrdinaryField<
        Field_varreturningtype,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).varreturningtype) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Var, varnosyn) == 40);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_varnosyn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Var>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Var, varattnosyn) == 44);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_varattnosyn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Var>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Var, location) == 48);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Var>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_jumble_args;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VariableSetStmt, jumble_args) == 24);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_jumble_args>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VariableSetStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VariableSetStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::VariableSetStmt> as c::expression::OrdinaryField<Field_jumble_args>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).jumble_args) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VariableSetStmt, is_local) == 25);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_local>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VariableSetStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 25;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VariableSetStmt, location) == 28);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VariableSetStmt>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::VariableSetStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::VariableSetStmt> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::WalReceiverConn,
                    *const ::core::ffi::c_char,
                    *const bool,
                    *const bool,
                ) -> (),
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::WalReceiverConn,
                    *const ::core::ffi::c_char,
                    *const bool,
                    *const bool,
                ) -> (),
            >,
        >() == 8
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_walrcv_alter_slot>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalReceiverFunctionsType>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_void__0l__struct__WalReceiverConn__0p__0c__const__char__0p__0c__const__0x5f426f6f6c__0p__0c__const__0x5f426f6f6c__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WalSnd, replyTime) == 80);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_replyTime>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalSnd>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WalSnd, kind) == 88);
#[cfg(any(feature = "pg18", feature = "pg19"))]
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
    const OFFSET: usize = 88;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::WindowAgg>() == 232);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::WindowAgg>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAgg, winref) == 112);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_winref>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAgg>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAgg, partNumCols) == 116);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_partNumCols>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAgg>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 116;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAgg, ordNumCols) == 144);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ordNumCols>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAgg>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAgg, frameOptions) == 176);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_frameOptions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAgg>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 176;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAgg, runCondition) == 200);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_runCondition>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAgg>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 200;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAgg, runConditionOrig) == 208);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_runConditionOrig>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAgg>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 208;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAgg, startInRangeFunc) == 216);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_startInRangeFunc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAgg>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 216;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAgg, endInRangeFunc) == 220);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_endInRangeFunc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAgg>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 220;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAgg, inRangeColl) == 224);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inRangeColl>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAgg>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 224;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAgg, inRangeAsc) == 228);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inRangeAsc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAgg>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 228;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAgg, inRangeNullsFirst) == 229);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inRangeNullsFirst>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAgg>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 229;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAgg, topWindow) == 230);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_topWindow>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAgg>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 230;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::WindowAggPath>() == 120);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::WindowAggPath>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggPath, qual) == 96);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_qual>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggPath>
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
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggPath, runCondition) == 104);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_runCondition>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggPath>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggPath, topwindow) == 112);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_topwindow>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggPath>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, use_pass_through) == 494
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_use_pass_through>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 494;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, top_window) == 495);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_top_window>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 495;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, currentgroup) == 504);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_currentgroup>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 504;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, frameheadgroup) == 512
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_frameheadgroup>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 512;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, frametailgroup) == 520
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_frametailgroup>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 520;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, groupheadpos) == 528);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_groupheadpos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 528;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, grouptailpos) == 536);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_grouptailpos>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 536;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, all_first) == 576);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_all_first>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 576;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, partition_spooled) == 577
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_partition_spooled>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 577;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_next_partition;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, next_partition) == 578
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_next_partition>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 578;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowAggState, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState> as c::expression::OrdinaryField<Field_next_partition>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).next_partition) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, more_partitions) == 579
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_more_partitions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 579;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, framehead_valid) == 580
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_framehead_valid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 580;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, frametail_valid) == 581
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_frametail_valid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 581;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowAggState, grouptail_valid) == 582
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_grouptail_valid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowAggState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 582;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<*const ::core::ffi::c_void>() == 8);
    assert!(::core::mem::align_of::<*const ::core::ffi::c_void>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_data>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecData>
{
    type Member = c::expression::CPointer<c::expression::CVoid, c::expression::ReadOnly>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::pg_locale_struct>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::pg_locale_struct>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_collate_is_c;
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_locale_struct, collate_is_c) == 2);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_collate_is_c>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_locale_struct>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 2;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::pg_locale_struct, <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_locale_struct> as c::expression::OrdinaryField<Field_collate_is_c>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).collate_is_c) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_ctype_is_c;
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_locale_struct, ctype_is_c) == 3);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ctype_is_c>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_locale_struct>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 3;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::pg_locale_struct, <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_locale_struct> as c::expression::OrdinaryField<Field_ctype_is_c>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ctype_is_c) };
#[cfg(feature = "pg18")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_locale_struct, is_default) == 4);
#[cfg(feature = "pg18")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_default>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_locale_struct>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::pg_locale_struct, <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_locale_struct> as c::expression::OrdinaryField<Field_is_default>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_default) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_casemap_full;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::pg_locale_struct__bindgen_ty_1__bindgen_ty_1,
        casemap_full
    ) == 8
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_casemap_full>
    for c::expression::CRecord<
        crate::__pgrx_c_bindings::pg_locale_struct__bindgen_ty_1__bindgen_ty_1,
    >
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::pg_locale_struct__bindgen_ty_1__bindgen_ty_1, <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_locale_struct__bindgen_ty_1__bindgen_ty_1> as c::expression::OrdinaryField<Field_casemap_full>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).casemap_full) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::xl_heap_inplace>() == 20);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::xl_heap_inplace>() == 4);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_heap_inplace, dbId) == 4);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dbId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_inplace>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_heap_inplace, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_inplace> as c::expression::OrdinaryField<Field_dbId>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dbId) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_heap_inplace, tsId) == 8);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tsId>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_inplace>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_heap_inplace, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_inplace> as c::expression::OrdinaryField<Field_tsId>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tsId) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_heap_inplace, relcacheInitFileInval) == 12
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relcacheInitFileInval>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_inplace>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_heap_inplace, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_inplace> as c::expression::OrdinaryField<Field_relcacheInitFileInval>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relcacheInitFileInval) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_heap_inplace, nmsgs) == 16);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nmsgs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_inplace>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_heap_inplace, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_inplace> as c::expression::OrdinaryField<Field_nmsgs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nmsgs) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::xl_xact_stats_item>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::xl_xact_stats_item>() == 4);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_objid_lo;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_xact_stats_item, objid_lo) == 8);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_objid_lo>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_xact_stats_item>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_xact_stats_item, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_xact_stats_item> as c::expression::OrdinaryField<Field_objid_lo>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).objid_lo) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_objid_hi;
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_xact_stats_item, objid_hi) == 12);
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_objid_hi>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_xact_stats_item>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_xact_stats_item, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_xact_stats_item> as c::expression::OrdinaryField<Field_objid_hi>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).objid_hi) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_private_iterator;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TBMIterator__bindgen_ty_1 {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TBMIterator__bindgen_ty_1, private_iterator)
        == 0
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::TBMPrivateIterator>() == 8);
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::TBMPrivateIterator>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_private_iterator>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TBMIterator__bindgen_ty_1>
{
    type Member = c::expression::CPointer<
        c::expression::COpaque<crate::__pgrx_c_bindings::TBMPrivateIterator>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TBMIterator__bindgen_ty_1, <<c::expression::CRecord<crate::__pgrx_c_bindings::TBMIterator__bindgen_ty_1> as c::expression::OrdinaryField<Field_private_iterator>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).private_iterator) };
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for crate::__pgrx_c_bindings::TBMPrivateIterator {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeType for crate::__pgrx_c_bindings::TBMPrivateIterator {
    type Marker = c::expression::COpaque<Self>;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::TableScanDescData__bindgen_ty_1>() == 16
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::TableScanDescData__bindgen_ty_1>() == 8
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TableScanDescData__bindgen_ty_1 {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgAioTargetData>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgAioTargetData>() == 4);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgAioTargetData {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for crate::__pgrx_c_bindings::derives_hash {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeType for crate::__pgrx_c_bindings::derives_hash {
    type Marker = c::expression::COpaque<Self>;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TidRangePath>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TidRangePath>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TidPath>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TidPath>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            crate::__pgrx_c_bindings::TableScanDescData__bindgen_ty_1__bindgen_ty_1,
        >() == 12
    );
    assert!(
        ::core::mem::align_of::<
            crate::__pgrx_c_bindings::TableScanDescData__bindgen_ty_1__bindgen_ty_1,
        >() == 2
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::TableScanDescData__bindgen_ty_1__bindgen_ty_1
{
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SubqueryScanPath>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SubqueryScanPath>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SortPath>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SortPath>() == 8);
};
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ReplicationSlotCtlData>() == 288);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ReplicationSlotCtlData>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RelPathStr>() == 72);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RelPathStr>() == 1);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::RelPathStr {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ProjectSetPath>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ProjectSetPath>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for crate::__pgrx_c_bindings::ProcSignalHeader {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeType for crate::__pgrx_c_bindings::ProcSignalHeader {
    type Marker = c::expression::COpaque<Self>;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_PendingIO>() == 2880);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_PendingIO>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgStat_PendingIO {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_BktypeIO>() == 2880);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_BktypeIO>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_BackendPending>() == 2880);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_BackendPending>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgStat_BackendPending {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgAioTargetInfo>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgAioTargetInfo>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgAioTargetInfo {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgAioReturn>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgAioReturn>() == 4);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgAioReturn {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgAioHandleCallbacks>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgAioHandleCallbacks>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgAioHandleCallbacks {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::NestPath>() == 112);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::NestPath>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MinMaxAggPath>() == 96);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MinMaxAggPath>() == 8);
};
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MergeAppendPath>() == 96);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MergeAppendPath>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MaterialPath>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MaterialPath>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::GroupResultPath>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::GroupResultPath>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::GroupPath>() == 104);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::GroupPath>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ForeignPath>() == 104);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ForeignPath>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_48>(
        ) == 8
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_48>(
        ) == 8
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_48
{
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_47>(
        ) == 8
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_47>(
        ) == 8
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord
    for crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_47
{
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_41>(
        ) == 8
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_41>(
        ) == 8
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_40>(
        ) == 8
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_40>(
        ) == 8
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_39>(
        ) == 8
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_39>(
        ) == 8
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_36>(
        ) == 40
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_36>(
        ) == 8
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_29>(
        ) == 16
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_29>(
        ) == 8
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_22>(
        ) == 24
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_22>(
        ) == 8
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_18>(
        ) == 8
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_18>(
        ) == 8
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_17>(
        ) == 32
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_17>(
        ) == 8
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_12>(
        ) == 16
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::ExprEvalStep__bindgen_ty_1__bindgen_ty_12>(
        ) == 8
    );
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for crate::__pgrx_c_bindings::DynamicFileList {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeType for crate::__pgrx_c_bindings::DynamicFileList {
    type Marker = c::expression::COpaque<Self>;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CreateForeignTableStmt>() == 128);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CreateForeignTableStmt>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CopyToRoutine>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CopyToRoutine>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CopyToRoutine {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::CopyFromRoutine>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::CopyFromRoutine>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::CopyFromRoutine {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BitmapOrPath>() == 96);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BitmapOrPath>() == 8);
};
#[cfg(feature = "pg18")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BitmapHeapScanDescData>() == 736);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BitmapHeapScanDescData>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BitmapHeapScanDescData {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BitmapHeapPath>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BitmapHeapPath>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BitmapAndPath>() == 96);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BitmapAndPath>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgAioOpData>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgAioOpData>() == 8);
};
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgAioOpData {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::RelPathStr>>() == 72
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::RelPathStr>>() == 1
);
#[cfg(any(feature = "pg18", feature = "pg19"))]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__GetRelationPath"]
    fn raw_inline_GetRelationPath(
        arg0: crate::__pgrx_c_bindings::Oid,
        arg1: crate::__pgrx_c_bindings::Oid,
        arg2: crate::__pgrx_c_bindings::Oid,
        arg3: i32,
        arg4: i32,
    ) -> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::RelPathStr>;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc = "Call the original C function `GetRelationPath`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_GetRelationPath(
    arg0: crate::__pgrx_c_bindings::Oid,
    arg1: crate::__pgrx_c_bindings::Oid,
    arg2: crate::__pgrx_c_bindings::Oid,
    arg3: i32,
    arg4: i32,
) -> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::RelPathStr> {
    const _: () = assert!(!::core::mem::needs_drop::<crate::__pgrx_c_bindings::Oid>());
    const _: () = assert!(!::core::mem::needs_drop::<crate::__pgrx_c_bindings::Oid>());
    const _: () = assert!(!::core::mem::needs_drop::<crate::__pgrx_c_bindings::Oid>());
    const _: () = assert!(!::core::mem::needs_drop::<i32>());
    const _: () = assert!(!::core::mem::needs_drop::<i32>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe {
        crate::ffi::pg_guard_ffi_boundary(move || {
            raw_inline_GetRelationPath(arg0, arg1, arg2, arg3, arg4)
        })
    }
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__abs"]
    fn raw_inline_abs(arg0: i32) -> i32;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[doc = "Call the original C function `abs`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_abs(arg0: i32) -> i32 {
    const _: () = assert!(!::core::mem::needs_drop::<i32>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_abs(arg0)) }
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::size_of::<usize>() == 8);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::align_of::<usize>() == 8);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::size_of::<i32>() == 4);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::align_of::<i32>() == 4);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::size_of::<i32>() == 4);
#[cfg(any(feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::align_of::<i32>() == 4);
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_SnapBuildState;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for EnumIdentity_SnapBuildState {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::EnumIdentity for EnumIdentity_SnapBuildState {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::enumeration::NumericEnum for EnumIdentity_SnapBuildState {
    type Compatible = c::CInt;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__CompareType;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for EnumIdentity_enum__CompareType {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::EnumIdentity for EnumIdentity_enum__CompareType {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__CompareType {
    type Compatible = c::CUnsignedInt;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__CopyLogVerbosityChoice {
    type Compatible = c::CInt;
}
#[cfg(feature = "pg18")]
// SAFETY: The explicit Rust integer repr matches Clang's compatible enum type and the assertions prove layout. Decoding receives a valid unit Rust variant; encoding checks every discriminant.
unsafe impl c::expression::EnumStorage<EnumIdentity_enum__NodeTag, c::CUnsignedInt>
    for crate::__pgrx_c_bindings::NodeTag
{
    fn decode(value: Self) -> u32 {
        value as u32
    }
    fn encode(value: u32) -> Self {
        match value {
            148 => crate::__pgrx_c_bindings::NodeTag::T_ATAlterConstraint,
            80 => crate::__pgrx_c_bindings::NodeTag::T_A_ArrayExpr,
            72 => crate::__pgrx_c_bindings::NodeTag::T_A_Const,
            71 => crate::__pgrx_c_bindings::NodeTag::T_A_Expr,
            78 => crate::__pgrx_c_bindings::NodeTag::T_A_Indices,
            79 => crate::__pgrx_c_bindings::NodeTag::T_A_Indirection,
            77 => crate::__pgrx_c_bindings::NodeTag::T_A_Star,
            154 => crate::__pgrx_c_bindings::NodeTag::T_AccessPriv,
            365 => crate::__pgrx_c_bindings::NodeTag::T_Agg,
            327 => crate::__pgrx_c_bindings::NodeTag::T_AggInfo,
            307 => crate::__pgrx_c_bindings::NodeTag::T_AggPath,
            429 => crate::__pgrx_c_bindings::NodeTag::T_AggState,
            328 => crate::__pgrx_c_bindings::NodeTag::T_AggTransInfo,
            9 => crate::__pgrx_c_bindings::NodeTag::T_Aggref,
            2 => crate::__pgrx_c_bindings::NodeTag::T_Alias,
            474 => crate::__pgrx_c_bindings::NodeTag::T_AllocSetContext,
            150 => crate::__pgrx_c_bindings::NodeTag::T_AlterCollationStmt,
            234 => crate::__pgrx_c_bindings::NodeTag::T_AlterDatabaseRefreshCollStmt,
            235 => crate::__pgrx_c_bindings::NodeTag::T_AlterDatabaseSetStmt,
            233 => crate::__pgrx_c_bindings::NodeTag::T_AlterDatabaseStmt,
            156 => crate::__pgrx_c_bindings::NodeTag::T_AlterDefaultPrivilegesStmt,
            151 => crate::__pgrx_c_bindings::NodeTag::T_AlterDomainStmt,
            229 => crate::__pgrx_c_bindings::NodeTag::T_AlterEnumStmt,
            183 => crate::__pgrx_c_bindings::NodeTag::T_AlterEventTrigStmt,
            168 => crate::__pgrx_c_bindings::NodeTag::T_AlterExtensionContentsStmt,
            167 => crate::__pgrx_c_bindings::NodeTag::T_AlterExtensionStmt,
            170 => crate::__pgrx_c_bindings::NodeTag::T_AlterFdwStmt,
            172 => crate::__pgrx_c_bindings::NodeTag::T_AlterForeignServerStmt,
            210 => crate::__pgrx_c_bindings::NodeTag::T_AlterFunctionStmt,
            216 => crate::__pgrx_c_bindings::NodeTag::T_AlterObjectDependsStmt,
            217 => crate::__pgrx_c_bindings::NodeTag::T_AlterObjectSchemaStmt,
            196 => crate::__pgrx_c_bindings::NodeTag::T_AlterOpFamilyStmt,
            219 => crate::__pgrx_c_bindings::NodeTag::T_AlterOperatorStmt,
            218 => crate::__pgrx_c_bindings::NodeTag::T_AlterOwnerStmt,
            179 => crate::__pgrx_c_bindings::NodeTag::T_AlterPolicyStmt,
            262 => crate::__pgrx_c_bindings::NodeTag::T_AlterPublicationStmt,
            452 => crate::__pgrx_c_bindings::NodeTag::T_AlterReplicationSlotCmd,
            187 => crate::__pgrx_c_bindings::NodeTag::T_AlterRoleSetStmt,
            186 => crate::__pgrx_c_bindings::NodeTag::T_AlterRoleStmt,
            190 => crate::__pgrx_c_bindings::NodeTag::T_AlterSeqStmt,
            207 => crate::__pgrx_c_bindings::NodeTag::T_AlterStatsStmt,
            264 => crate::__pgrx_c_bindings::NodeTag::T_AlterSubscriptionStmt,
            237 => crate::__pgrx_c_bindings::NodeTag::T_AlterSystemStmt,
            258 => crate::__pgrx_c_bindings::NodeTag::T_AlterTSConfigurationStmt,
            257 => crate::__pgrx_c_bindings::NodeTag::T_AlterTSDictionaryStmt,
            147 => crate::__pgrx_c_bindings::NodeTag::T_AlterTableCmd,
            165 => crate::__pgrx_c_bindings::NodeTag::T_AlterTableMoveAllStmt,
            164 => crate::__pgrx_c_bindings::NodeTag::T_AlterTableSpaceOptionsStmt,
            146 => crate::__pgrx_c_bindings::NodeTag::T_AlterTableStmt,
            220 => crate::__pgrx_c_bindings::NodeTag::T_AlterTypeStmt,
            175 => crate::__pgrx_c_bindings::NodeTag::T_AlterUserMappingStmt,
            24 => crate::__pgrx_c_bindings::NodeTag::T_AlternativeSubPlan,
            334 => crate::__pgrx_c_bindings::NodeTag::T_Append,
            290 => crate::__pgrx_c_bindings::NodeTag::T_AppendPath,
            322 => crate::__pgrx_c_bindings::NodeTag::T_AppendRelInfo,
            397 => crate::__pgrx_c_bindings::NodeTag::T_AppendState,
            29 => crate::__pgrx_c_bindings::NodeTag::T_ArrayCoerceExpr,
            35 => crate::__pgrx_c_bindings::NodeTag::T_ArrayExpr,
            449 => crate::__pgrx_c_bindings::NodeTag::T_BaseBackupCmd,
            469 => crate::__pgrx_c_bindings::NodeTag::T_BitString,
            337 => crate::__pgrx_c_bindings::NodeTag::T_BitmapAnd,
            283 => crate::__pgrx_c_bindings::NodeTag::T_BitmapAndPath,
            400 => crate::__pgrx_c_bindings::NodeTag::T_BitmapAndState,
            282 => crate::__pgrx_c_bindings::NodeTag::T_BitmapHeapPath,
            344 => crate::__pgrx_c_bindings::NodeTag::T_BitmapHeapScan,
            408 => crate::__pgrx_c_bindings::NodeTag::T_BitmapHeapScanState,
            343 => crate::__pgrx_c_bindings::NodeTag::T_BitmapIndexScan,
            407 => crate::__pgrx_c_bindings::NodeTag::T_BitmapIndexScanState,
            338 => crate::__pgrx_c_bindings::NodeTag::T_BitmapOr,
            284 => crate::__pgrx_c_bindings::NodeTag::T_BitmapOrPath,
            401 => crate::__pgrx_c_bindings::NodeTag::T_BitmapOrState,
            445 => crate::__pgrx_c_bindings::NodeTag::T_Bitmapset,
            21 => crate::__pgrx_c_bindings::NodeTag::T_BoolExpr,
            467 => crate::__pgrx_c_bindings::NodeTag::T_Boolean,
            53 => crate::__pgrx_c_bindings::NodeTag::T_BooleanTest,
            477 => crate::__pgrx_c_bindings::NodeTag::T_BumpContext,
            114 => crate::__pgrx_c_bindings::NodeTag::T_CTECycleClause,
            113 => crate::__pgrx_c_bindings::NodeTag::T_CTESearchClause,
            214 => crate::__pgrx_c_bindings::NodeTag::T_CallContext,
            213 => crate::__pgrx_c_bindings::NodeTag::T_CallStmt,
            32 => crate::__pgrx_c_bindings::NodeTag::T_CaseExpr,
            34 => crate::__pgrx_c_bindings::NodeTag::T_CaseTestExpr,
            33 => crate::__pgrx_c_bindings::NodeTag::T_CaseWhen,
            244 => crate::__pgrx_c_bindings::NodeTag::T_CheckPointStmt,
            202 => crate::__pgrx_c_bindings::NodeTag::T_ClosePortalStmt,
            238 => crate::__pgrx_c_bindings::NodeTag::T_ClusterStmt,
            38 => crate::__pgrx_c_bindings::NodeTag::T_CoalesceExpr,
            55 => crate::__pgrx_c_bindings::NodeTag::T_CoerceToDomain,
            56 => crate::__pgrx_c_bindings::NodeTag::T_CoerceToDomainValue,
            28 => crate::__pgrx_c_bindings::NodeTag::T_CoerceViaIO,
            74 => crate::__pgrx_c_bindings::NodeTag::T_CollateClause,
            31 => crate::__pgrx_c_bindings::NodeTag::T_CollateExpr,
            90 => crate::__pgrx_c_bindings::NodeTag::T_ColumnDef,
            69 => crate::__pgrx_c_bindings::NodeTag::T_ColumnRef,
            199 => crate::__pgrx_c_bindings::NodeTag::T_CommentStmt,
            115 => crate::__pgrx_c_bindings::NodeTag::T_CommonTableExpr,
            226 => crate::__pgrx_c_bindings::NodeTag::T_CompositeTypeStmt,
            7 => crate::__pgrx_c_bindings::NodeTag::T_Const,
            161 => crate::__pgrx_c_bindings::NodeTag::T_Constraint,
            247 => crate::__pgrx_c_bindings::NodeTag::T_ConstraintsSetStmt,
            30 => crate::__pgrx_c_bindings::NodeTag::T_ConvertRowtypeExpr,
            157 => crate::__pgrx_c_bindings::NodeTag::T_CopyStmt,
            180 => crate::__pgrx_c_bindings::NodeTag::T_CreateAmStmt,
            250 => crate::__pgrx_c_bindings::NodeTag::T_CreateCastStmt,
            249 => crate::__pgrx_c_bindings::NodeTag::T_CreateConversionStmt,
            192 => crate::__pgrx_c_bindings::NodeTag::T_CreateDomainStmt,
            227 => crate::__pgrx_c_bindings::NodeTag::T_CreateEnumStmt,
            182 => crate::__pgrx_c_bindings::NodeTag::T_CreateEventTrigStmt,
            166 => crate::__pgrx_c_bindings::NodeTag::T_CreateExtensionStmt,
            169 => crate::__pgrx_c_bindings::NodeTag::T_CreateFdwStmt,
            171 => crate::__pgrx_c_bindings::NodeTag::T_CreateForeignServerStmt,
            173 => crate::__pgrx_c_bindings::NodeTag::T_CreateForeignTableStmt,
            208 => crate::__pgrx_c_bindings::NodeTag::T_CreateFunctionStmt,
            194 => crate::__pgrx_c_bindings::NodeTag::T_CreateOpClassItem,
            193 => crate::__pgrx_c_bindings::NodeTag::T_CreateOpClassStmt,
            195 => crate::__pgrx_c_bindings::NodeTag::T_CreateOpFamilyStmt,
            184 => crate::__pgrx_c_bindings::NodeTag::T_CreatePLangStmt,
            178 => crate::__pgrx_c_bindings::NodeTag::T_CreatePolicyStmt,
            261 => crate::__pgrx_c_bindings::NodeTag::T_CreatePublicationStmt,
            228 => crate::__pgrx_c_bindings::NodeTag::T_CreateRangeStmt,
            450 => crate::__pgrx_c_bindings::NodeTag::T_CreateReplicationSlotCmd,
            185 => crate::__pgrx_c_bindings::NodeTag::T_CreateRoleStmt,
            145 => crate::__pgrx_c_bindings::NodeTag::T_CreateSchemaStmt,
            189 => crate::__pgrx_c_bindings::NodeTag::T_CreateSeqStmt,
            205 => crate::__pgrx_c_bindings::NodeTag::T_CreateStatsStmt,
            160 => crate::__pgrx_c_bindings::NodeTag::T_CreateStmt,
            263 => crate::__pgrx_c_bindings::NodeTag::T_CreateSubscriptionStmt,
            242 => crate::__pgrx_c_bindings::NodeTag::T_CreateTableAsStmt,
            162 => crate::__pgrx_c_bindings::NodeTag::T_CreateTableSpaceStmt,
            251 => crate::__pgrx_c_bindings::NodeTag::T_CreateTransformStmt,
            181 => crate::__pgrx_c_bindings::NodeTag::T_CreateTrigStmt,
            174 => crate::__pgrx_c_bindings::NodeTag::T_CreateUserMappingStmt,
            232 => crate::__pgrx_c_bindings::NodeTag::T_CreatedbStmt,
            351 => crate::__pgrx_c_bindings::NodeTag::T_CteScan,
            415 => crate::__pgrx_c_bindings::NodeTag::T_CteScanState,
            58 => crate::__pgrx_c_bindings::NodeTag::T_CurrentOfExpr,
            289 => crate::__pgrx_c_bindings::NodeTag::T_CustomPath,
            355 => crate::__pgrx_c_bindings::NodeTag::T_CustomScan,
            419 => crate::__pgrx_c_bindings::NodeTag::T_CustomScanState,
            254 => crate::__pgrx_c_bindings::NodeTag::T_DeallocateStmt,
            201 => crate::__pgrx_c_bindings::NodeTag::T_DeclareCursorStmt,
            93 => crate::__pgrx_c_bindings::NodeTag::T_DefElem,
            191 => crate::__pgrx_c_bindings::NodeTag::T_DefineStmt,
            138 => crate::__pgrx_c_bindings::NodeTag::T_DeleteStmt,
            245 => crate::__pgrx_c_bindings::NodeTag::T_DiscardStmt,
            18 => crate::__pgrx_c_bindings::NodeTag::T_DistinctExpr,
            211 => crate::__pgrx_c_bindings::NodeTag::T_DoStmt,
            393 => crate::__pgrx_c_bindings::NodeTag::T_DomainConstraintState,
            255 => crate::__pgrx_c_bindings::NodeTag::T_DropOwnedStmt,
            451 => crate::__pgrx_c_bindings::NodeTag::T_DropReplicationSlotCmd,
            188 => crate::__pgrx_c_bindings::NodeTag::T_DropRoleStmt,
            197 => crate::__pgrx_c_bindings::NodeTag::T_DropStmt,
            265 => crate::__pgrx_c_bindings::NodeTag::T_DropSubscriptionStmt,
            163 => crate::__pgrx_c_bindings::NodeTag::T_DropTableSpaceStmt,
            176 => crate::__pgrx_c_bindings::NodeTag::T_DropUserMappingStmt,
            236 => crate::__pgrx_c_bindings::NodeTag::T_DropdbStmt,
            389 => crate::__pgrx_c_bindings::NodeTag::T_EState,
            273 => crate::__pgrx_c_bindings::NodeTag::T_EquivalenceClass,
            274 => crate::__pgrx_c_bindings::NodeTag::T_EquivalenceMember,
            447 => crate::__pgrx_c_bindings::NodeTag::T_ErrorSaveContext,
            441 => crate::__pgrx_c_bindings::NodeTag::T_EventTriggerData,
            253 => crate::__pgrx_c_bindings::NodeTag::T_ExecuteStmt,
            241 => crate::__pgrx_c_bindings::NodeTag::T_ExplainStmt,
            382 => crate::__pgrx_c_bindings::NodeTag::T_ExprContext,
            380 => crate::__pgrx_c_bindings::NodeTag::T_ExprState,
            446 => crate::__pgrx_c_bindings::NodeTag::T_ExtensibleNode,
            444 => crate::__pgrx_c_bindings::NodeTag::T_FdwRoutine,
            203 => crate::__pgrx_c_bindings::NodeTag::T_FetchStmt,
            25 => crate::__pgrx_c_bindings::NodeTag::T_FieldSelect,
            26 => crate::__pgrx_c_bindings::NodeTag::T_FieldStore,
            466 => crate::__pgrx_c_bindings::NodeTag::T_Float,
            470 => crate::__pgrx_c_bindings::NodeTag::T_ForeignKeyCacheInfo,
            270 => crate::__pgrx_c_bindings::NodeTag::T_ForeignKeyOptInfo,
            288 => crate::__pgrx_c_bindings::NodeTag::T_ForeignPath,
            354 => crate::__pgrx_c_bindings::NodeTag::T_ForeignScan,
            418 => crate::__pgrx_c_bindings::NodeTag::T_ForeignScanState,
            65 => crate::__pgrx_c_bindings::NodeTag::T_FromExpr,
            76 => crate::__pgrx_c_bindings::NodeTag::T_FuncCall,
            15 => crate::__pgrx_c_bindings::NodeTag::T_FuncExpr,
            209 => crate::__pgrx_c_bindings::NodeTag::T_FunctionParameter,
            348 => crate::__pgrx_c_bindings::NodeTag::T_FunctionScan,
            412 => crate::__pgrx_c_bindings::NodeTag::T_FunctionScanState,
            368 => crate::__pgrx_c_bindings::NodeTag::T_Gather,
            369 => crate::__pgrx_c_bindings::NodeTag::T_GatherMerge,
            297 => crate::__pgrx_c_bindings::NodeTag::T_GatherMergePath,
            433 => crate::__pgrx_c_bindings::NodeTag::T_GatherMergeState,
            296 => crate::__pgrx_c_bindings::NodeTag::T_GatherPath,
            432 => crate::__pgrx_c_bindings::NodeTag::T_GatherState,
            475 => crate::__pgrx_c_bindings::NodeTag::T_GenerationContext,
            155 => crate::__pgrx_c_bindings::NodeTag::T_GrantRoleStmt,
            152 => crate::__pgrx_c_bindings::NodeTag::T_GrantStmt,
            364 => crate::__pgrx_c_bindings::NodeTag::T_Group,
            276 => crate::__pgrx_c_bindings::NodeTag::T_GroupByOrdering,
            305 => crate::__pgrx_c_bindings::NodeTag::T_GroupPath,
            292 => crate::__pgrx_c_bindings::NodeTag::T_GroupResultPath,
            428 => crate::__pgrx_c_bindings::NodeTag::T_GroupState,
            10 => crate::__pgrx_c_bindings::NodeTag::T_GroupingFunc,
            107 => crate::__pgrx_c_bindings::NodeTag::T_GroupingSet,
            308 => crate::__pgrx_c_bindings::NodeTag::T_GroupingSetData,
            310 => crate::__pgrx_c_bindings::NodeTag::T_GroupingSetsPath,
            370 => crate::__pgrx_c_bindings::NodeTag::T_Hash,
            359 => crate::__pgrx_c_bindings::NodeTag::T_HashJoin,
            423 => crate::__pgrx_c_bindings::NodeTag::T_HashJoinState,
            300 => crate::__pgrx_c_bindings::NodeTag::T_HashPath,
            434 => crate::__pgrx_c_bindings::NodeTag::T_HashState,
            448 => crate::__pgrx_c_bindings::NodeTag::T_IdentifySystemCmd,
            177 => crate::__pgrx_c_bindings::NodeTag::T_ImportForeignSchemaStmt,
            363 => crate::__pgrx_c_bindings::NodeTag::T_IncrementalSort,
            304 => crate::__pgrx_c_bindings::NodeTag::T_IncrementalSortPath,
            427 => crate::__pgrx_c_bindings::NodeTag::T_IncrementalSortState,
            438 => crate::__pgrx_c_bindings::NodeTag::T_IndexAmRoutine,
            281 => crate::__pgrx_c_bindings::NodeTag::T_IndexClause,
            92 => crate::__pgrx_c_bindings::NodeTag::T_IndexElem,
            381 => crate::__pgrx_c_bindings::NodeTag::T_IndexInfo,
            342 => crate::__pgrx_c_bindings::NodeTag::T_IndexOnlyScan,
            406 => crate::__pgrx_c_bindings::NodeTag::T_IndexOnlyScanState,
            269 => crate::__pgrx_c_bindings::NodeTag::T_IndexOptInfo,
            280 => crate::__pgrx_c_bindings::NodeTag::T_IndexPath,
            341 => crate::__pgrx_c_bindings::NodeTag::T_IndexScan,
            405 => crate::__pgrx_c_bindings::NodeTag::T_IndexScanState,
            204 => crate::__pgrx_c_bindings::NodeTag::T_IndexStmt,
            111 => crate::__pgrx_c_bindings::NodeTag::T_InferClause,
            60 => crate::__pgrx_c_bindings::NodeTag::T_InferenceElem,
            212 => crate::__pgrx_c_bindings::NodeTag::T_InlineCodeBlock,
            137 => crate::__pgrx_c_bindings::NodeTag::T_InsertStmt,
            471 => crate::__pgrx_c_bindings::NodeTag::T_IntList,
            465 => crate::__pgrx_c_bindings::NodeTag::T_Integer,
            5 => crate::__pgrx_c_bindings::NodeTag::T_IntoClause,
            0 => crate::__pgrx_c_bindings::NodeTag::T_Invalid,
            272 => crate::__pgrx_c_bindings::NodeTag::T_JoinDomain,
            64 => crate::__pgrx_c_bindings::NodeTag::T_JoinExpr,
            420 => crate::__pgrx_c_bindings::NodeTag::T_JoinState,
            133 => crate::__pgrx_c_bindings::NodeTag::T_JsonAggConstructor,
            121 => crate::__pgrx_c_bindings::NodeTag::T_JsonArgument,
            135 => crate::__pgrx_c_bindings::NodeTag::T_JsonArrayAgg,
            131 => crate::__pgrx_c_bindings::NodeTag::T_JsonArrayConstructor,
            132 => crate::__pgrx_c_bindings::NodeTag::T_JsonArrayQueryConstructor,
            47 => crate::__pgrx_c_bindings::NodeTag::T_JsonBehavior,
            45 => crate::__pgrx_c_bindings::NodeTag::T_JsonConstructorExpr,
            48 => crate::__pgrx_c_bindings::NodeTag::T_JsonExpr,
            42 => crate::__pgrx_c_bindings::NodeTag::T_JsonFormat,
            122 => crate::__pgrx_c_bindings::NodeTag::T_JsonFuncExpr,
            46 => crate::__pgrx_c_bindings::NodeTag::T_JsonIsPredicate,
            126 => crate::__pgrx_c_bindings::NodeTag::T_JsonKeyValue,
            134 => crate::__pgrx_c_bindings::NodeTag::T_JsonObjectAgg,
            130 => crate::__pgrx_c_bindings::NodeTag::T_JsonObjectConstructor,
            120 => crate::__pgrx_c_bindings::NodeTag::T_JsonOutput,
            127 => crate::__pgrx_c_bindings::NodeTag::T_JsonParseExpr,
            43 => crate::__pgrx_c_bindings::NodeTag::T_JsonReturning,
            128 => crate::__pgrx_c_bindings::NodeTag::T_JsonScalarExpr,
            129 => crate::__pgrx_c_bindings::NodeTag::T_JsonSerializeExpr,
            124 => crate::__pgrx_c_bindings::NodeTag::T_JsonTable,
            125 => crate::__pgrx_c_bindings::NodeTag::T_JsonTableColumn,
            49 => crate::__pgrx_c_bindings::NodeTag::T_JsonTablePath,
            50 => crate::__pgrx_c_bindings::NodeTag::T_JsonTablePathScan,
            123 => crate::__pgrx_c_bindings::NodeTag::T_JsonTablePathSpec,
            51 => crate::__pgrx_c_bindings::NodeTag::T_JsonTableSiblingJoin,
            44 => crate::__pgrx_c_bindings::NodeTag::T_JsonValueExpr,
            385 => crate::__pgrx_c_bindings::NodeTag::T_JunkFilter,
            373 => crate::__pgrx_c_bindings::NodeTag::T_Limit,
            317 => crate::__pgrx_c_bindings::NodeTag::T_LimitPath,
            437 => crate::__pgrx_c_bindings::NodeTag::T_LimitState,
            1 => crate::__pgrx_c_bindings::NodeTag::T_List,
            223 => crate::__pgrx_c_bindings::NodeTag::T_ListenStmt,
            231 => crate::__pgrx_c_bindings::NodeTag::T_LoadStmt,
            372 => crate::__pgrx_c_bindings::NodeTag::T_LockRows,
            315 => crate::__pgrx_c_bindings::NodeTag::T_LockRowsPath,
            436 => crate::__pgrx_c_bindings::NodeTag::T_LockRowsState,
            246 => crate::__pgrx_c_bindings::NodeTag::T_LockStmt,
            94 => crate::__pgrx_c_bindings::NodeTag::T_LockingClause,
            360 => crate::__pgrx_c_bindings::NodeTag::T_Material,
            293 => crate::__pgrx_c_bindings::NodeTag::T_MaterialPath,
            424 => crate::__pgrx_c_bindings::NodeTag::T_MaterialState,
            361 => crate::__pgrx_c_bindings::NodeTag::T_Memoize,
            294 => crate::__pgrx_c_bindings::NodeTag::T_MemoizePath,
            425 => crate::__pgrx_c_bindings::NodeTag::T_MemoizeState,
            54 => crate::__pgrx_c_bindings::NodeTag::T_MergeAction,
            387 => crate::__pgrx_c_bindings::NodeTag::T_MergeActionState,
            335 => crate::__pgrx_c_bindings::NodeTag::T_MergeAppend,
            291 => crate::__pgrx_c_bindings::NodeTag::T_MergeAppendPath,
            398 => crate::__pgrx_c_bindings::NodeTag::T_MergeAppendState,
            358 => crate::__pgrx_c_bindings::NodeTag::T_MergeJoin,
            422 => crate::__pgrx_c_bindings::NodeTag::T_MergeJoinState,
            299 => crate::__pgrx_c_bindings::NodeTag::T_MergePath,
            140 => crate::__pgrx_c_bindings::NodeTag::T_MergeStmt,
            13 => crate::__pgrx_c_bindings::NodeTag::T_MergeSupportFunc,
            116 => crate::__pgrx_c_bindings::NodeTag::T_MergeWhenClause,
            325 => crate::__pgrx_c_bindings::NodeTag::T_MinMaxAggInfo,
            311 => crate::__pgrx_c_bindings::NodeTag::T_MinMaxAggPath,
            39 => crate::__pgrx_c_bindings::NodeTag::T_MinMaxExpr,
            333 => crate::__pgrx_c_bindings::NodeTag::T_ModifyTable,
            316 => crate::__pgrx_c_bindings::NodeTag::T_ModifyTablePath,
            396 => crate::__pgrx_c_bindings::NodeTag::T_ModifyTableState,
            82 => crate::__pgrx_c_bindings::NodeTag::T_MultiAssignRef,
            16 => crate::__pgrx_c_bindings::NodeTag::T_NamedArgExpr,
            352 => crate::__pgrx_c_bindings::NodeTag::T_NamedTuplestoreScan,
            416 => crate::__pgrx_c_bindings::NodeTag::T_NamedTuplestoreScanState,
            356 => crate::__pgrx_c_bindings::NodeTag::T_NestLoop,
            357 => crate::__pgrx_c_bindings::NodeTag::T_NestLoopParam,
            421 => crate::__pgrx_c_bindings::NodeTag::T_NestLoopState,
            298 => crate::__pgrx_c_bindings::NodeTag::T_NestPath,
            59 => crate::__pgrx_c_bindings::NodeTag::T_NextValueExpr,
            222 => crate::__pgrx_c_bindings::NodeTag::T_NotifyStmt,
            19 => crate::__pgrx_c_bindings::NodeTag::T_NullIfExpr,
            52 => crate::__pgrx_c_bindings::NodeTag::T_NullTest,
            153 => crate::__pgrx_c_bindings::NodeTag::T_ObjectWithArgs,
            472 => crate::__pgrx_c_bindings::NodeTag::T_OidList,
            112 => crate::__pgrx_c_bindings::NodeTag::T_OnConflictClause,
            66 => crate::__pgrx_c_bindings::NodeTag::T_OnConflictExpr,
            386 => crate::__pgrx_c_bindings::NodeTag::T_OnConflictSetState,
            17 => crate::__pgrx_c_bindings::NodeTag::T_OpExpr,
            321 => crate::__pgrx_c_bindings::NodeTag::T_OuterJoinClauseInfo,
            144 => crate::__pgrx_c_bindings::NodeTag::T_PLAssignStmt,
            8 => crate::__pgrx_c_bindings::NodeTag::T_Param,
            278 => crate::__pgrx_c_bindings::NodeTag::T_ParamPathInfo,
            70 => crate::__pgrx_c_bindings::NodeTag::T_ParamRef,
            98 => crate::__pgrx_c_bindings::NodeTag::T_PartitionBoundSpec,
            100 => crate::__pgrx_c_bindings::NodeTag::T_PartitionCmd,
            96 => crate::__pgrx_c_bindings::NodeTag::T_PartitionElem,
            375 => crate::__pgrx_c_bindings::NodeTag::T_PartitionPruneInfo,
            378 => crate::__pgrx_c_bindings::NodeTag::T_PartitionPruneStepCombine,
            377 => crate::__pgrx_c_bindings::NodeTag::T_PartitionPruneStepOp,
            99 => crate::__pgrx_c_bindings::NodeTag::T_PartitionRangeDatum,
            97 => crate::__pgrx_c_bindings::NodeTag::T_PartitionSpec,
            376 => crate::__pgrx_c_bindings::NodeTag::T_PartitionedRelPruneInfo,
            279 => crate::__pgrx_c_bindings::NodeTag::T_Path,
            275 => crate::__pgrx_c_bindings::NodeTag::T_PathKey,
            277 => crate::__pgrx_c_bindings::NodeTag::T_PathTarget,
            324 => crate::__pgrx_c_bindings::NodeTag::T_PlaceHolderInfo,
            319 => crate::__pgrx_c_bindings::NodeTag::T_PlaceHolderVar,
            379 => crate::__pgrx_c_bindings::NodeTag::T_PlanInvalItem,
            374 => crate::__pgrx_c_bindings::NodeTag::T_PlanRowMark,
            330 => crate::__pgrx_c_bindings::NodeTag::T_PlannedStmt,
            266 => crate::__pgrx_c_bindings::NodeTag::T_PlannerGlobal,
            267 => crate::__pgrx_c_bindings::NodeTag::T_PlannerInfo,
            326 => crate::__pgrx_c_bindings::NodeTag::T_PlannerParamItem,
            252 => crate::__pgrx_c_bindings::NodeTag::T_PrepareStmt,
            332 => crate::__pgrx_c_bindings::NodeTag::T_ProjectSet,
            302 => crate::__pgrx_c_bindings::NodeTag::T_ProjectSetPath,
            395 => crate::__pgrx_c_bindings::NodeTag::T_ProjectSetState,
            384 => crate::__pgrx_c_bindings::NodeTag::T_ProjectionInfo,
            301 => crate::__pgrx_c_bindings::NodeTag::T_ProjectionPath,
            260 => crate::__pgrx_c_bindings::NodeTag::T_PublicationObjSpec,
            259 => crate::__pgrx_c_bindings::NodeTag::T_PublicationTable,
            67 => crate::__pgrx_c_bindings::NodeTag::T_Query,
            102 => crate::__pgrx_c_bindings::NodeTag::T_RTEPermissionInfo,
            86 => crate::__pgrx_c_bindings::NodeTag::T_RangeFunction,
            85 => crate::__pgrx_c_bindings::NodeTag::T_RangeSubselect,
            87 => crate::__pgrx_c_bindings::NodeTag::T_RangeTableFunc,
            88 => crate::__pgrx_c_bindings::NodeTag::T_RangeTableFuncCol,
            89 => crate::__pgrx_c_bindings::NodeTag::T_RangeTableSample,
            101 => crate::__pgrx_c_bindings::NodeTag::T_RangeTblEntry,
            103 => crate::__pgrx_c_bindings::NodeTag::T_RangeTblFunction,
            63 => crate::__pgrx_c_bindings::NodeTag::T_RangeTblRef,
            3 => crate::__pgrx_c_bindings::NodeTag::T_RangeVar,
            136 => crate::__pgrx_c_bindings::NodeTag::T_RawStmt,
            454 => crate::__pgrx_c_bindings::NodeTag::T_ReadReplicationSlotCmd,
            256 => crate::__pgrx_c_bindings::NodeTag::T_ReassignOwnedStmt,
            336 => crate::__pgrx_c_bindings::NodeTag::T_RecursiveUnion,
            314 => crate::__pgrx_c_bindings::NodeTag::T_RecursiveUnionPath,
            399 => crate::__pgrx_c_bindings::NodeTag::T_RecursiveUnionState,
            243 => crate::__pgrx_c_bindings::NodeTag::T_RefreshMatViewStmt,
            248 => crate::__pgrx_c_bindings::NodeTag::T_ReindexStmt,
            268 => crate::__pgrx_c_bindings::NodeTag::T_RelOptInfo,
            27 => crate::__pgrx_c_bindings::NodeTag::T_RelabelType,
            215 => crate::__pgrx_c_bindings::NodeTag::T_RenameStmt,
            149 => crate::__pgrx_c_bindings::NodeTag::T_ReplicaIdentityStmt,
            81 => crate::__pgrx_c_bindings::NodeTag::T_ResTarget,
            318 => crate::__pgrx_c_bindings::NodeTag::T_RestrictInfo,
            331 => crate::__pgrx_c_bindings::NodeTag::T_Result,
            388 => crate::__pgrx_c_bindings::NodeTag::T_ResultRelInfo,
            394 => crate::__pgrx_c_bindings::NodeTag::T_ResultState,
            383 => crate::__pgrx_c_bindings::NodeTag::T_ReturnSetInfo,
            143 => crate::__pgrx_c_bindings::NodeTag::T_ReturnStmt,
            118 => crate::__pgrx_c_bindings::NodeTag::T_ReturningClause,
            61 => crate::__pgrx_c_bindings::NodeTag::T_ReturningExpr,
            117 => crate::__pgrx_c_bindings::NodeTag::T_ReturningOption,
            75 => crate::__pgrx_c_bindings::NodeTag::T_RoleSpec,
            309 => crate::__pgrx_c_bindings::NodeTag::T_RollupData,
            37 => crate::__pgrx_c_bindings::NodeTag::T_RowCompareExpr,
            36 => crate::__pgrx_c_bindings::NodeTag::T_RowExpr,
            323 => crate::__pgrx_c_bindings::NodeTag::T_RowIdentityVarInfo,
            109 => crate::__pgrx_c_bindings::NodeTag::T_RowMarkClause,
            221 => crate::__pgrx_c_bindings::NodeTag::T_RuleStmt,
            40 => crate::__pgrx_c_bindings::NodeTag::T_SQLValueFunction,
            340 => crate::__pgrx_c_bindings::NodeTag::T_SampleScan,
            404 => crate::__pgrx_c_bindings::NodeTag::T_SampleScanState,
            20 => crate::__pgrx_c_bindings::NodeTag::T_ScalarArrayOpExpr,
            402 => crate::__pgrx_c_bindings::NodeTag::T_ScanState,
            200 => crate::__pgrx_c_bindings::NodeTag::T_SecLabelStmt,
            141 => crate::__pgrx_c_bindings::NodeTag::T_SelectStmt,
            339 => crate::__pgrx_c_bindings::NodeTag::T_SeqScan,
            403 => crate::__pgrx_c_bindings::NodeTag::T_SeqScanState,
            391 => crate::__pgrx_c_bindings::NodeTag::T_SetExprState,
            371 => crate::__pgrx_c_bindings::NodeTag::T_SetOp,
            313 => crate::__pgrx_c_bindings::NodeTag::T_SetOpPath,
            435 => crate::__pgrx_c_bindings::NodeTag::T_SetOpState,
            142 => crate::__pgrx_c_bindings::NodeTag::T_SetOperationStmt,
            57 => crate::__pgrx_c_bindings::NodeTag::T_SetToDefault,
            476 => crate::__pgrx_c_bindings::NodeTag::T_SlabContext,
            362 => crate::__pgrx_c_bindings::NodeTag::T_Sort,
            83 => crate::__pgrx_c_bindings::NodeTag::T_SortBy,
            106 => crate::__pgrx_c_bindings::NodeTag::T_SortGroupClause,
            303 => crate::__pgrx_c_bindings::NodeTag::T_SortPath,
            426 => crate::__pgrx_c_bindings::NodeTag::T_SortState,
            320 => crate::__pgrx_c_bindings::NodeTag::T_SpecialJoinInfo,
            453 => crate::__pgrx_c_bindings::NodeTag::T_StartReplicationCmd,
            271 => crate::__pgrx_c_bindings::NodeTag::T_StatisticExtInfo,
            206 => crate::__pgrx_c_bindings::NodeTag::T_StatsElem,
            468 => crate::__pgrx_c_bindings::NodeTag::T_String,
            22 => crate::__pgrx_c_bindings::NodeTag::T_SubLink,
            23 => crate::__pgrx_c_bindings::NodeTag::T_SubPlan,
            392 => crate::__pgrx_c_bindings::NodeTag::T_SubPlanState,
            347 => crate::__pgrx_c_bindings::NodeTag::T_SubqueryScan,
            287 => crate::__pgrx_c_bindings::NodeTag::T_SubqueryScanPath,
            411 => crate::__pgrx_c_bindings::NodeTag::T_SubqueryScanState,
            14 => crate::__pgrx_c_bindings::NodeTag::T_SubscriptingRef,
            459 => crate::__pgrx_c_bindings::NodeTag::T_SupportRequestCost,
            461 => crate::__pgrx_c_bindings::NodeTag::T_SupportRequestIndexCondition,
            464 => crate::__pgrx_c_bindings::NodeTag::T_SupportRequestModifyInPlace,
            463 => crate::__pgrx_c_bindings::NodeTag::T_SupportRequestOptimizeWindowClause,
            460 => crate::__pgrx_c_bindings::NodeTag::T_SupportRequestRows,
            458 => crate::__pgrx_c_bindings::NodeTag::T_SupportRequestSelectivity,
            457 => crate::__pgrx_c_bindings::NodeTag::T_SupportRequestSimplify,
            462 => crate::__pgrx_c_bindings::NodeTag::T_SupportRequestWFuncMonotonic,
            478 => crate::__pgrx_c_bindings::NodeTag::T_TIDBitmap,
            439 => crate::__pgrx_c_bindings::NodeTag::T_TableAmRoutine,
            4 => crate::__pgrx_c_bindings::NodeTag::T_TableFunc,
            350 => crate::__pgrx_c_bindings::NodeTag::T_TableFuncScan,
            414 => crate::__pgrx_c_bindings::NodeTag::T_TableFuncScanState,
            91 => crate::__pgrx_c_bindings::NodeTag::T_TableLikeClause,
            104 => crate::__pgrx_c_bindings::NodeTag::T_TableSampleClause,
            62 => crate::__pgrx_c_bindings::NodeTag::T_TargetEntry,
            285 => crate::__pgrx_c_bindings::NodeTag::T_TidPath,
            286 => crate::__pgrx_c_bindings::NodeTag::T_TidRangePath,
            346 => crate::__pgrx_c_bindings::NodeTag::T_TidRangeScan,
            410 => crate::__pgrx_c_bindings::NodeTag::T_TidRangeScanState,
            345 => crate::__pgrx_c_bindings::NodeTag::T_TidScan,
            409 => crate::__pgrx_c_bindings::NodeTag::T_TidScanState,
            455 => crate::__pgrx_c_bindings::NodeTag::T_TimeLineHistoryCmd,
            225 => crate::__pgrx_c_bindings::NodeTag::T_TransactionStmt,
            442 => crate::__pgrx_c_bindings::NodeTag::T_TriggerData,
            119 => crate::__pgrx_c_bindings::NodeTag::T_TriggerTransition,
            198 => crate::__pgrx_c_bindings::NodeTag::T_TruncateStmt,
            440 => crate::__pgrx_c_bindings::NodeTag::T_TsmRoutine,
            443 => crate::__pgrx_c_bindings::NodeTag::T_TupleTableSlot,
            73 => crate::__pgrx_c_bindings::NodeTag::T_TypeCast,
            68 => crate::__pgrx_c_bindings::NodeTag::T_TypeName,
            367 => crate::__pgrx_c_bindings::NodeTag::T_Unique,
            295 => crate::__pgrx_c_bindings::NodeTag::T_UniquePath,
            329 => crate::__pgrx_c_bindings::NodeTag::T_UniqueRelInfo,
            431 => crate::__pgrx_c_bindings::NodeTag::T_UniqueState,
            224 => crate::__pgrx_c_bindings::NodeTag::T_UnlistenStmt,
            139 => crate::__pgrx_c_bindings::NodeTag::T_UpdateStmt,
            456 => crate::__pgrx_c_bindings::NodeTag::T_UploadManifestCmd,
            306 => crate::__pgrx_c_bindings::NodeTag::T_UpperUniquePath,
            240 => crate::__pgrx_c_bindings::NodeTag::T_VacuumRelation,
            239 => crate::__pgrx_c_bindings::NodeTag::T_VacuumStmt,
            349 => crate::__pgrx_c_bindings::NodeTag::T_ValuesScan,
            413 => crate::__pgrx_c_bindings::NodeTag::T_ValuesScanState,
            6 => crate::__pgrx_c_bindings::NodeTag::T_Var,
            158 => crate::__pgrx_c_bindings::NodeTag::T_VariableSetStmt,
            159 => crate::__pgrx_c_bindings::NodeTag::T_VariableShowStmt,
            230 => crate::__pgrx_c_bindings::NodeTag::T_ViewStmt,
            366 => crate::__pgrx_c_bindings::NodeTag::T_WindowAgg,
            312 => crate::__pgrx_c_bindings::NodeTag::T_WindowAggPath,
            430 => crate::__pgrx_c_bindings::NodeTag::T_WindowAggState,
            108 => crate::__pgrx_c_bindings::NodeTag::T_WindowClause,
            84 => crate::__pgrx_c_bindings::NodeTag::T_WindowDef,
            11 => crate::__pgrx_c_bindings::NodeTag::T_WindowFunc,
            390 => crate::__pgrx_c_bindings::NodeTag::T_WindowFuncExprState,
            12 => crate::__pgrx_c_bindings::NodeTag::T_WindowFuncRunCondition,
            479 => crate::__pgrx_c_bindings::NodeTag::T_WindowObjectData,
            105 => crate::__pgrx_c_bindings::NodeTag::T_WithCheckOption,
            110 => crate::__pgrx_c_bindings::NodeTag::T_WithClause,
            353 => crate::__pgrx_c_bindings::NodeTag::T_WorkTableScan,
            417 => crate::__pgrx_c_bindings::NodeTag::T_WorkTableScanState,
            473 => crate::__pgrx_c_bindings::NodeTag::T_XidList,
            41 => crate::__pgrx_c_bindings::NodeTag::T_XmlExpr,
            95 => crate::__pgrx_c_bindings::NodeTag::T_XmlSerialize,
            _ => panic!("C enum value has no corresponding Rust variant"),
        }
    }
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__PLpgSQL_rwopt;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for EnumIdentity_enum__PLpgSQL_rwopt {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::EnumIdentity for EnumIdentity_enum__PLpgSQL_rwopt {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__PLpgSQL_rwopt {
    type Compatible = c::CUnsignedInt;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__PublishGencolsType;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for EnumIdentity_enum__PublishGencolsType {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::EnumIdentity for EnumIdentity_enum__PublishGencolsType {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__PublishGencolsType {
    type Compatible = c::CUnsignedInt;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__ReturningOptionKind;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for EnumIdentity_enum__ReturningOptionKind {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::EnumIdentity for EnumIdentity_enum__ReturningOptionKind {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__ReturningOptionKind {
    type Compatible = c::CUnsignedInt;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__VarReturningType;
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::sealed::Sealed for EnumIdentity_enum__VarReturningType {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::EnumIdentity for EnumIdentity_enum__VarReturningType {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__VarReturningType {
    type Compatible = c::CUnsignedInt;
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl type_enumerator_seals::Seal_T_ for crate::__pgrx_c_bindings::ATAlterConstraint {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl TypeEnumerator_T_ for crate::__pgrx_c_bindings::ATAlterConstraint {
    const VALUE: c::CValue<c::CInt> =
        c::CValue::<c::CInt>::new(crate::__pgrx_c_bindings::NodeTag::T_ATAlterConstraint as i32);
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl type_enumerator_seals::Seal_T_ for crate::__pgrx_c_bindings::ReturningClause {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl TypeEnumerator_T_ for crate::__pgrx_c_bindings::ReturningClause {
    const VALUE: c::CValue<c::CInt> =
        c::CValue::<c::CInt>::new(crate::__pgrx_c_bindings::NodeTag::T_ReturningClause as i32);
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl type_enumerator_seals::Seal_T_ for crate::__pgrx_c_bindings::ReturningExpr {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl TypeEnumerator_T_ for crate::__pgrx_c_bindings::ReturningExpr {
    const VALUE: c::CValue<c::CInt> =
        c::CValue::<c::CInt>::new(crate::__pgrx_c_bindings::NodeTag::T_ReturningExpr as i32);
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl type_enumerator_seals::Seal_T_ for crate::__pgrx_c_bindings::ReturningOption {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl TypeEnumerator_T_ for crate::__pgrx_c_bindings::ReturningOption {
    const VALUE: c::CValue<c::CInt> =
        c::CValue::<c::CInt>::new(crate::__pgrx_c_bindings::NodeTag::T_ReturningOption as i32);
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl type_enumerator_seals::Seal_T_ for crate::__pgrx_c_bindings::SupportRequestModifyInPlace {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl TypeEnumerator_T_ for crate::__pgrx_c_bindings::SupportRequestModifyInPlace {
    const VALUE: c::CValue<c::CInt> = c::CValue::<c::CInt>::new(
        crate::__pgrx_c_bindings::NodeTag::T_SupportRequestModifyInPlace as i32,
    );
}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl type_enumerator_seals::Seal_T_ for crate::__pgrx_c_bindings::UniqueRelInfo {}
#[cfg(any(feature = "pg18", feature = "pg19"))]
impl TypeEnumerator_T_ for crate::__pgrx_c_bindings::UniqueRelInfo {
    const VALUE: c::CValue<c::CInt> =
        c::CValue::<c::CInt>::new(crate::__pgrx_c_bindings::NodeTag::T_UniqueRelInfo as i32);
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                crate::__pgrx_c_bindings::NodeTag,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                crate::__pgrx_c_bindings::NodeTag,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__const__char__0p__0c__const__char__0p__0c__enum__NodeTag__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed
    for Signature_0x5f426f6f6c__0l__const__char__0p__0c__const__char__0p__0c__enum__NodeTag__0r
{
}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__const__char__0p__0c__const__char__0p__0c__enum__NodeTag__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                crate::__pgrx_c_bindings::NodeTag,
            ) -> bool,
        >,
    >;
}
#[cfg(feature = "pg19")]
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<A0: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::IndexFetchTableData>, c::expression::ReadWrite>>, A1: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::ItemPointerData>, c::expression::ReadWrite>>, A2: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData>, c::expression::ReadWrite>>, A3: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot>, c::expression::ReadWrite>>, A4: c::expression::ImplicitTo<c::expression::CPointer<c::CBool, c::expression::ReadWrite>>, A5: c::expression::ImplicitTo<c::expression::CPointer<c::CBool, c::expression::ReadWrite>>> c::expression::Call<(A0,A1,A2,A3,A4,A5,)> for Signature_0x5f426f6f6c__0l__struct__IndexFetchTableData__0p__0c__struct__ItemPointerData__0p__0c__struct__SnapshotData__0p__0c__struct__TupleTableSlot__0p__0c__0x5f426f6f6c__0p__0c__0x5f426f6f6c__0p__0r {
 type Output = <c::CBool as c::expression::CType>::Value;
 unsafe fn call(pointer: Self::Pointer, args: (A0,A1,A2,A3,A4,A5,)) -> Self::Output {
let function = pointer.expect("C indirect call requires a non-null function pointer");
const { assert!(!::core::mem::needs_drop::<A0>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::IndexFetchTableData>());
let native0 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::IndexFetchTableData>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::IndexFetchTableData>, c::expression::ReadWrite>, _>(args.0));
const { assert!(!::core::mem::needs_drop::<A1>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::ItemPointerData>());
let native1 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::ItemPointerData>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::ItemPointerData>, c::expression::ReadWrite>, _>(args.1));
const { assert!(!::core::mem::needs_drop::<A2>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::SnapshotData>());
let native2 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData>, c::expression::ReadWrite>, _>(args.2));
const { assert!(!::core::mem::needs_drop::<A3>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::TupleTableSlot>());
let native3 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot>, c::expression::ReadWrite>, _>(args.3));
const { assert!(!::core::mem::needs_drop::<A4>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut bool>());
let native4 = <c::expression::CPointer<c::CBool, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::CBool, c::expression::ReadWrite>, _>(args.4));
const { assert!(!::core::mem::needs_drop::<A5>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut bool>());
let native5 = <c::expression::CPointer<c::CBool, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::CBool, c::expression::ReadWrite>, _>(args.5));
// SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
let result = unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0, native1, native2, native3, native4, native5)) };
<c::CBool as c::expression::CType>::from_storage(result)
}
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::List,
                i32,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::List,
                i32,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__RelationData__0p__0c__struct__List__0p__0c__int__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed
    for Signature_0x5f426f6f6c__0l__struct__RelationData__0p__0c__struct__List__0p__0c__int__0r
{
}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__struct__RelationData__0p__0c__struct__List__0p__0c__int__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::List,
                i32,
            ) -> bool,
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::SlruDesc,
                *mut ::core::ffi::c_char,
                i64,
                *mut ::core::ffi::c_void,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::SlruDesc,
                *mut ::core::ffi::c_char,
                i64,
                *mut ::core::ffi::c_void,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__SlruDesc__0p__0c__char__0p__0c__long__0c__void__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed
    for Signature_0x5f426f6f6c__0l__struct__SlruDesc__0p__0c__char__0p__0c__long__0c__void__0p__0r
{
}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__struct__SlruDesc__0p__0c__char__0p__0c__long__0c__void__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_4<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::SlruDesc,
                *mut ::core::ffi::c_char,
                i64,
                *mut ::core::ffi::c_void,
            ) -> bool,
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableScanDescData,
                *mut f64,
                *mut f64,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::TableScanDescData,
                *mut f64,
                *mut f64,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__double__0p__0c__double__0p__0c__struct__TupleTableSlot__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__double__0p__0c__double__0p__0c__struct__TupleTableSlot__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__double__0p__0c__double__0p__0c__struct__TupleTableSlot__0p__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TableScanDescData, *mut f64, *mut f64, *mut crate::__pgrx_c_bindings::TupleTableSlot) -> bool>>;
}
#[cfg(feature = "pg19")]
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<A0: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TableScanDescData>, c::expression::ReadWrite>>, A1: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CDouble, c::expression::ReadWrite>>, A2: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CDouble, c::expression::ReadWrite>>, A3: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot>, c::expression::ReadWrite>>> c::expression::Call<(A0,A1,A2,A3,)> for Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__double__0p__0c__double__0p__0c__struct__TupleTableSlot__0p__0r {
 type Output = <c::CBool as c::expression::CType>::Value;
 unsafe fn call(pointer: Self::Pointer, args: (A0,A1,A2,A3,)) -> Self::Output {
let function = pointer.expect("C indirect call requires a non-null function pointer");
const { assert!(!::core::mem::needs_drop::<A0>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::TableScanDescData>());
let native0 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TableScanDescData>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TableScanDescData>, c::expression::ReadWrite>, _>(args.0));
const { assert!(!::core::mem::needs_drop::<A1>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut f64>());
let native1 = <c::expression::CPointer<c::expression::CDouble, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CDouble, c::expression::ReadWrite>, _>(args.1));
const { assert!(!::core::mem::needs_drop::<A2>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut f64>());
let native2 = <c::expression::CPointer<c::expression::CDouble, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CDouble, c::expression::ReadWrite>, _>(args.2));
const { assert!(!::core::mem::needs_drop::<A3>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::TupleTableSlot>());
let native3 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot>, c::expression::ReadWrite>, _>(args.3));
// SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
let result = unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0, native1, native2, native3)) };
<c::CBool as c::expression::CType>::from_storage(result)
}
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::SampleScanState>() == 8);
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::SampleScanState>() == 8);
#[cfg(feature = "pg19")]
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<A0: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TableScanDescData>, c::expression::ReadWrite>>, A1: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::SampleScanState>, c::expression::ReadWrite>>> c::expression::Call<(A0,A1,)> for Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__struct__SampleScanState__0p__0r {
 type Output = <c::CBool as c::expression::CType>::Value;
 unsafe fn call(pointer: Self::Pointer, args: (A0,A1,)) -> Self::Output {
let function = pointer.expect("C indirect call requires a non-null function pointer");
const { assert!(!::core::mem::needs_drop::<A0>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::TableScanDescData>());
let native0 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TableScanDescData>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TableScanDescData>, c::expression::ReadWrite>, _>(args.0));
const { assert!(!::core::mem::needs_drop::<A1>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::SampleScanState>());
let native1 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::SampleScanState>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::SampleScanState>, c::expression::ReadWrite>, _>(args.1));
// SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
let result = unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0, native1)) };
<c::CBool as c::expression::CType>::from_storage(result)
}
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::SampleScanState>() == 8);
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::SampleScanState>() == 8);
#[cfg(feature = "pg19")]
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<A0: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TableScanDescData>, c::expression::ReadWrite>>, A1: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::SampleScanState>, c::expression::ReadWrite>>, A2: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot>, c::expression::ReadWrite>>> c::expression::Call<(A0,A1,A2,)> for Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__struct__SampleScanState__0p__0c__struct__TupleTableSlot__0p__0r {
 type Output = <c::CBool as c::expression::CType>::Value;
 unsafe fn call(pointer: Self::Pointer, args: (A0,A1,A2,)) -> Self::Output {
let function = pointer.expect("C indirect call requires a non-null function pointer");
const { assert!(!::core::mem::needs_drop::<A0>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::TableScanDescData>());
let native0 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TableScanDescData>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TableScanDescData>, c::expression::ReadWrite>, _>(args.0));
const { assert!(!::core::mem::needs_drop::<A1>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::SampleScanState>());
let native1 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::SampleScanState>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::SampleScanState>, c::expression::ReadWrite>, _>(args.1));
const { assert!(!::core::mem::needs_drop::<A2>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::TupleTableSlot>());
let native2 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot>, c::expression::ReadWrite>, _>(args.2));
// SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
let result = unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0, native1, native2)) };
<c::CBool as c::expression::CType>::from_storage(result)
}
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<*mut u64>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<*mut u64>() == 8);
#[cfg(feature = "pg19")]
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<A0: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TableScanDescData>, c::expression::ReadWrite>>, A1: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot>, c::expression::ReadWrite>>, A2: c::expression::ImplicitTo<c::expression::CPointer<c::CBool, c::expression::ReadWrite>>, A3: c::expression::ImplicitTo<c::expression::CPointer<c::CUnsignedLong, c::expression::ReadWrite>>, A4: c::expression::ImplicitTo<c::expression::CPointer<c::CUnsignedLong, c::expression::ReadWrite>>> c::expression::Call<(A0,A1,A2,A3,A4,)> for Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__struct__TupleTableSlot__0p__0c__0x5f426f6f6c__0p__0c__unsigned__long__0p__0c__unsigned__long__0p__0r {
 type Output = <c::CBool as c::expression::CType>::Value;
 unsafe fn call(pointer: Self::Pointer, args: (A0,A1,A2,A3,A4,)) -> Self::Output {
let function = pointer.expect("C indirect call requires a non-null function pointer");
const { assert!(!::core::mem::needs_drop::<A0>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::TableScanDescData>());
let native0 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TableScanDescData>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TableScanDescData>, c::expression::ReadWrite>, _>(args.0));
const { assert!(!::core::mem::needs_drop::<A1>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::TupleTableSlot>());
let native1 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot>, c::expression::ReadWrite>, _>(args.1));
const { assert!(!::core::mem::needs_drop::<A2>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut bool>());
let native2 = <c::expression::CPointer<c::CBool, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::CBool, c::expression::ReadWrite>, _>(args.2));
const { assert!(!::core::mem::needs_drop::<A3>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut u64>());
let native3 = <c::expression::CPointer<c::CUnsignedLong, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::CUnsignedLong, c::expression::ReadWrite>, _>(args.3));
const { assert!(!::core::mem::needs_drop::<A4>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut u64>());
let native4 = <c::expression::CPointer<c::CUnsignedLong, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::CUnsignedLong, c::expression::ReadWrite>, _>(args.4));
// SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
let result = unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0, native1, native2, native3, native4)) };
<c::CBool as c::expression::CType>::from_storage(result)
}
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                *mut crate::__pgrx_c_bindings::pg_locale_struct,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                *mut crate::__pgrx_c_bindings::pg_locale_struct,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__unsigned__int__0c__struct__pg_locale_struct__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed
    for Signature_0x5f426f6f6c__0l__unsigned__int__0c__struct__pg_locale_struct__0p__0r
{
}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__unsigned__int__0c__struct__pg_locale_struct__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                *mut crate::__pgrx_c_bindings::pg_locale_struct,
            ) -> bool,
        >,
    >;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__enum__SysCacheIdentifier__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__enum__SysCacheIdentifier__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__enum__SysCacheIdentifier__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(i32, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum) -> bool>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn(u32) -> bool>>() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn(u32) -> bool>>()
        == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__enum__vartag_external__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__enum__vartag_external__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__enum__vartag_external__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(u32) -> bool>,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::Fitness>,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::Fitness>,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__Fitness__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__Fitness__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_0x5f426f6f6c__0l__struct__Fitness__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::Fitness>,
            ) -> bool,
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::varatt_external>,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::varatt_external>,
            ) -> bool,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_0x5f426f6f6c__0l__struct__varatt_external__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_0x5f426f6f6c__0l__struct__varatt_external__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_0x5f426f6f6c__0l__struct__varatt_external__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::varatt_external>,
            ) -> bool,
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<*mut u64>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<*mut u64>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::WalReceiverConn,
                *mut u32,
                *mut u64,
            ) -> *mut ::core::ffi::c_char,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::WalReceiverConn,
                *mut u32,
                *mut u64,
            ) -> *mut ::core::ffi::c_char,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_char__0p__0l__struct__WalReceiverConn__0p__0c__unsigned__int__0p__0c__unsigned__long__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_char__0p__0l__struct__WalReceiverConn__0p__0c__unsigned__int__0p__0c__unsigned__long__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_char__0p__0l__struct__WalReceiverConn__0p__0c__unsigned__int__0p__0c__unsigned__long__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::WalReceiverConn, *mut u32, *mut u64) -> *mut ::core::ffi::c_char>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<*mut u64>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<*mut u64>() == 8);
#[cfg(feature = "pg19")]
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<A0: c::expression::ImplicitTo<c::expression::CPointer<c::expression::COpaque<crate::__pgrx_c_bindings::WalReceiverConn>, c::expression::ReadWrite>>, A1: c::expression::ImplicitTo<c::expression::CPointer<c::CUnsignedInt, c::expression::ReadWrite>>, A2: c::expression::ImplicitTo<c::expression::CPointer<c::CUnsignedLong, c::expression::ReadWrite>>> c::expression::Call<(A0,A1,A2,)> for Signature_char__0p__0l__struct__WalReceiverConn__0p__0c__unsigned__int__0p__0c__unsigned__long__0p__0r {
 type Output = <c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadWrite> as c::expression::CType>::Value;
 unsafe fn call(pointer: Self::Pointer, args: (A0,A1,A2,)) -> Self::Output {
let function = pointer.expect("C indirect call requires a non-null function pointer");
const { assert!(!::core::mem::needs_drop::<A0>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::WalReceiverConn>());
let native0 = <c::expression::CPointer<c::expression::COpaque<crate::__pgrx_c_bindings::WalReceiverConn>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::COpaque<crate::__pgrx_c_bindings::WalReceiverConn>, c::expression::ReadWrite>, _>(args.0));
const { assert!(!::core::mem::needs_drop::<A1>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut u32>());
let native1 = <c::expression::CPointer<c::CUnsignedInt, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::CUnsignedInt, c::expression::ReadWrite>, _>(args.1));
const { assert!(!::core::mem::needs_drop::<A2>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut u64>());
let native2 = <c::expression::CPointer<c::CUnsignedLong, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::CUnsignedLong, c::expression::ReadWrite>, _>(args.2));
// SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
let result = unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0, native1, native2)) };
<c::expression::CPointer<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, c::expression::ReadWrite> as c::expression::CType>::from_storage(result)
}
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const ::core::ffi::c_void) -> *mut ::core::ffi::c_char,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const ::core::ffi::c_void) -> *mut ::core::ffi::c_char,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_char__0p__0l__const__void__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_char__0p__0l__const__void__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_char__0p__0l__const__void__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const ::core::ffi::c_void) -> *mut ::core::ffi::c_char,
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(f64, f64, *mut crate::__pgrx_c_bindings::Node) -> f64,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(f64, f64, *mut crate::__pgrx_c_bindings::Node) -> f64,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_double__0l__double__0c__double__0c__struct__Node__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_double__0l__double__0c__double__0c__struct__Node__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_double__0l__double__0c__double__0c__struct__Node__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(f64, f64, *mut crate::__pgrx_c_bindings::Node) -> f64,
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::ItemPointerData,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
                u32,
                u32,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                bool,
                *mut crate::__pgrx_c_bindings::TM_FailureData,
                *mut u32,
                *mut u32,
            ) -> u32,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::ItemPointerData,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
                u32,
                u32,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                bool,
                *mut crate::__pgrx_c_bindings::TM_FailureData,
                *mut u32,
                *mut u32,
            ) -> u32,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_enum__TM_Result__0l__struct__RelationData__0p__0c__struct__ItemPointerData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__unsigned__int__0c__struct__SnapshotData__0p__0c__struct__SnapshotData__0p__0c__0x5f426f6f6c__0c__struct__TM_FailureData__0p__0c__enum__LockTupleMode__0p__0c__enum__TU_UpdateIndexes__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_enum__TM_Result__0l__struct__RelationData__0p__0c__struct__ItemPointerData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__unsigned__int__0c__struct__SnapshotData__0p__0c__struct__SnapshotData__0p__0c__0x5f426f6f6c__0c__struct__TM_FailureData__0p__0c__enum__LockTupleMode__0p__0c__enum__TU_UpdateIndexes__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_enum__TM_Result__0l__struct__RelationData__0p__0c__struct__ItemPointerData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__unsigned__int__0c__struct__SnapshotData__0p__0c__struct__SnapshotData__0p__0c__0x5f426f6f6c__0c__struct__TM_FailureData__0p__0c__enum__LockTupleMode__0p__0c__enum__TU_UpdateIndexes__0p__0r {
 type Physical = PhysicalFunction_C_unwind_11<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::ItemPointerData, *mut crate::__pgrx_c_bindings::TupleTableSlot, u32, u32, *mut crate::__pgrx_c_bindings::SnapshotData, *mut crate::__pgrx_c_bindings::SnapshotData, bool, *mut crate::__pgrx_c_bindings::TM_FailureData, *mut u32, *mut u32) -> u32>>;
}
#[cfg(feature = "pg19")]
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<A0: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>, c::expression::ReadWrite>>, A1: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::ItemPointerData>, c::expression::ReadWrite>>, A2: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot>, c::expression::ReadWrite>>, A3: c::expression::ImplicitTo<c::CUnsignedInt>, A4: c::expression::ImplicitTo<c::CUnsignedInt>, A5: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData>, c::expression::ReadWrite>>, A6: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData>, c::expression::ReadWrite>>, A7: c::expression::ImplicitTo<c::CBool>, A8: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TM_FailureData>, c::expression::ReadWrite>>, A9: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CEnumObject<crate::__pgrx_c_generated::EnumIdentity_enum__LockTupleMode, c::CUnsignedInt, u32>, c::expression::ReadWrite>>, A10: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CEnumObject<crate::__pgrx_c_generated::EnumIdentity_enum__TU_UpdateIndexes, c::CUnsignedInt, u32>, c::expression::ReadWrite>>> c::expression::Call<(A0,A1,A2,A3,A4,A5,A6,A7,A8,A9,A10,)> for Signature_enum__TM_Result__0l__struct__RelationData__0p__0c__struct__ItemPointerData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__unsigned__int__0c__struct__SnapshotData__0p__0c__struct__SnapshotData__0p__0c__0x5f426f6f6c__0c__struct__TM_FailureData__0p__0c__enum__LockTupleMode__0p__0c__enum__TU_UpdateIndexes__0p__0r {
 type Output = <c::expression::CEnumObject<crate::__pgrx_c_generated::EnumIdentity_enum__TM_Result, c::CUnsignedInt, u32> as c::expression::CType>::Value;
 unsafe fn call(pointer: Self::Pointer, args: (A0,A1,A2,A3,A4,A5,A6,A7,A8,A9,A10,)) -> Self::Output {
let function = pointer.expect("C indirect call requires a non-null function pointer");
const { assert!(!::core::mem::needs_drop::<A0>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::RelationData>());
let native0 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>, c::expression::ReadWrite>, _>(args.0));
const { assert!(!::core::mem::needs_drop::<A1>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::ItemPointerData>());
let native1 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::ItemPointerData>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::ItemPointerData>, c::expression::ReadWrite>, _>(args.1));
const { assert!(!::core::mem::needs_drop::<A2>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::TupleTableSlot>());
let native2 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot>, c::expression::ReadWrite>, _>(args.2));
const { assert!(!::core::mem::needs_drop::<A3>()); }
const _: () = assert!(!::core::mem::needs_drop::<u32>());
let native3 = <c::CUnsignedInt as c::expression::CType>::into_storage(c::expression::implicit::<c::CUnsignedInt, _>(args.3));
const { assert!(!::core::mem::needs_drop::<A4>()); }
const _: () = assert!(!::core::mem::needs_drop::<u32>());
let native4 = <c::CUnsignedInt as c::expression::CType>::into_storage(c::expression::implicit::<c::CUnsignedInt, _>(args.4));
const { assert!(!::core::mem::needs_drop::<A5>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::SnapshotData>());
let native5 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData>, c::expression::ReadWrite>, _>(args.5));
const { assert!(!::core::mem::needs_drop::<A6>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::SnapshotData>());
let native6 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData>, c::expression::ReadWrite>, _>(args.6));
const { assert!(!::core::mem::needs_drop::<A7>()); }
const _: () = assert!(!::core::mem::needs_drop::<bool>());
let native7 = <c::CBool as c::expression::CType>::into_storage(c::expression::implicit::<c::CBool, _>(args.7));
const { assert!(!::core::mem::needs_drop::<A8>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::TM_FailureData>());
let native8 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TM_FailureData>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TM_FailureData>, c::expression::ReadWrite>, _>(args.8));
const { assert!(!::core::mem::needs_drop::<A9>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut u32>());
let native9 = <c::expression::CPointer<c::expression::CEnumObject<crate::__pgrx_c_generated::EnumIdentity_enum__LockTupleMode, c::CUnsignedInt, u32>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CEnumObject<crate::__pgrx_c_generated::EnumIdentity_enum__LockTupleMode, c::CUnsignedInt, u32>, c::expression::ReadWrite>, _>(args.9));
const { assert!(!::core::mem::needs_drop::<A10>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut u32>());
let native10 = <c::expression::CPointer<c::expression::CEnumObject<crate::__pgrx_c_generated::EnumIdentity_enum__TU_UpdateIndexes, c::CUnsignedInt, u32>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CEnumObject<crate::__pgrx_c_generated::EnumIdentity_enum__TU_UpdateIndexes, c::CUnsignedInt, u32>, c::expression::ReadWrite>, _>(args.10));
// SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
let result = unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0, native1, native2, native3, native4, native5, native6, native7, native8, native9, native10)) };
<c::expression::CEnumObject<crate::__pgrx_c_generated::EnumIdentity_enum__TM_Result, c::CUnsignedInt, u32> as c::expression::CType>::from_storage(result)
}
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::ItemPointerData,
                u32,
                u32,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                bool,
                *mut crate::__pgrx_c_bindings::TM_FailureData,
            ) -> u32,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::ItemPointerData,
                u32,
                u32,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                bool,
                *mut crate::__pgrx_c_bindings::TM_FailureData,
            ) -> u32,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_enum__TM_Result__0l__struct__RelationData__0p__0c__struct__ItemPointerData__0p__0c__unsigned__int__0c__unsigned__int__0c__struct__SnapshotData__0p__0c__struct__SnapshotData__0p__0c__0x5f426f6f6c__0c__struct__TM_FailureData__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_enum__TM_Result__0l__struct__RelationData__0p__0c__struct__ItemPointerData__0p__0c__unsigned__int__0c__unsigned__int__0c__struct__SnapshotData__0p__0c__struct__SnapshotData__0p__0c__0x5f426f6f6c__0c__struct__TM_FailureData__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_enum__TM_Result__0l__struct__RelationData__0p__0c__struct__ItemPointerData__0p__0c__unsigned__int__0c__unsigned__int__0c__struct__SnapshotData__0p__0c__struct__SnapshotData__0p__0c__0x5f426f6f6c__0c__struct__TM_FailureData__0p__0r {
 type Physical = PhysicalFunction_C_unwind_8<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::ItemPointerData, u32, u32, *mut crate::__pgrx_c_bindings::SnapshotData, *mut crate::__pgrx_c_bindings::SnapshotData, bool, *mut crate::__pgrx_c_bindings::TM_FailureData) -> u32>>;
}
#[cfg(feature = "pg19")]
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<A0: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>, c::expression::ReadWrite>>, A1: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::ItemPointerData>, c::expression::ReadWrite>>, A2: c::expression::ImplicitTo<c::CUnsignedInt>, A3: c::expression::ImplicitTo<c::CUnsignedInt>, A4: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData>, c::expression::ReadWrite>>, A5: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData>, c::expression::ReadWrite>>, A6: c::expression::ImplicitTo<c::CBool>, A7: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TM_FailureData>, c::expression::ReadWrite>>> c::expression::Call<(A0,A1,A2,A3,A4,A5,A6,A7,)> for Signature_enum__TM_Result__0l__struct__RelationData__0p__0c__struct__ItemPointerData__0p__0c__unsigned__int__0c__unsigned__int__0c__struct__SnapshotData__0p__0c__struct__SnapshotData__0p__0c__0x5f426f6f6c__0c__struct__TM_FailureData__0p__0r {
 type Output = <c::expression::CEnumObject<crate::__pgrx_c_generated::EnumIdentity_enum__TM_Result, c::CUnsignedInt, u32> as c::expression::CType>::Value;
 unsafe fn call(pointer: Self::Pointer, args: (A0,A1,A2,A3,A4,A5,A6,A7,)) -> Self::Output {
let function = pointer.expect("C indirect call requires a non-null function pointer");
const { assert!(!::core::mem::needs_drop::<A0>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::RelationData>());
let native0 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>, c::expression::ReadWrite>, _>(args.0));
const { assert!(!::core::mem::needs_drop::<A1>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::ItemPointerData>());
let native1 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::ItemPointerData>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::ItemPointerData>, c::expression::ReadWrite>, _>(args.1));
const { assert!(!::core::mem::needs_drop::<A2>()); }
const _: () = assert!(!::core::mem::needs_drop::<u32>());
let native2 = <c::CUnsignedInt as c::expression::CType>::into_storage(c::expression::implicit::<c::CUnsignedInt, _>(args.2));
const { assert!(!::core::mem::needs_drop::<A3>()); }
const _: () = assert!(!::core::mem::needs_drop::<u32>());
let native3 = <c::CUnsignedInt as c::expression::CType>::into_storage(c::expression::implicit::<c::CUnsignedInt, _>(args.3));
const { assert!(!::core::mem::needs_drop::<A4>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::SnapshotData>());
let native4 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData>, c::expression::ReadWrite>, _>(args.4));
const { assert!(!::core::mem::needs_drop::<A5>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::SnapshotData>());
let native5 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData>, c::expression::ReadWrite>, _>(args.5));
const { assert!(!::core::mem::needs_drop::<A6>()); }
const _: () = assert!(!::core::mem::needs_drop::<bool>());
let native6 = <c::CBool as c::expression::CType>::into_storage(c::expression::implicit::<c::CBool, _>(args.6));
const { assert!(!::core::mem::needs_drop::<A7>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::TM_FailureData>());
let native7 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TM_FailureData>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TM_FailureData>, c::expression::ReadWrite>, _>(args.7));
// SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
let result = unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0, native1, native2, native3, native4, native5, native6, native7)) };
<c::expression::CEnumObject<crate::__pgrx_c_generated::EnumIdentity_enum__TM_Result, c::CUnsignedInt, u32> as c::expression::CType>::from_storage(result)
}
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_void) -> u32>,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_void) -> u32>,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_enum__vartag_external__0l__const__void__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_enum__vartag_external__0l__const__void__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_enum__vartag_external__0l__const__void__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_void) -> u32>,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *mut crate::__pgrx_c_bindings::pg_locale_struct,
            ) -> i32,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                *mut crate::__pgrx_c_bindings::pg_locale_struct,
            ) -> i32,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__const__char__0p__0c__const__char__0p__0c__struct__pg_locale_struct__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_int__0l__const__char__0p__0c__const__char__0p__0c__struct__pg_locale_struct__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_int__0l__const__char__0p__0c__const__char__0p__0c__struct__pg_locale_struct__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_char, *const ::core::ffi::c_char, *mut crate::__pgrx_c_bindings::pg_locale_struct) -> i32>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_char,
                usize,
                *const ::core::ffi::c_char,
                usize,
                *mut crate::__pgrx_c_bindings::pg_locale_struct,
            ) -> i32,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_char,
                usize,
                *const ::core::ffi::c_char,
                usize,
                *mut crate::__pgrx_c_bindings::pg_locale_struct,
            ) -> i32,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__const__char__0p__0c__unsigned__long__0c__const__char__0p__0c__unsigned__long__0c__struct__pg_locale_struct__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_int__0l__const__char__0p__0c__unsigned__long__0c__const__char__0p__0c__unsigned__long__0c__struct__pg_locale_struct__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_int__0l__const__char__0p__0c__unsigned__long__0c__const__char__0p__0c__unsigned__long__0c__struct__pg_locale_struct__0p__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_char, usize, *const ::core::ffi::c_char, usize, *mut crate::__pgrx_c_bindings::pg_locale_struct) -> i32>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_void) -> i32>,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_void) -> i32>,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__const__void__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_int__0l__const__void__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_int__0l__const__void__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_void) -> i32>,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::Fitness>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::Fitness>,
            ) -> i32,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::Fitness>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::Fitness>,
            ) -> i32,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__struct__Fitness__0c__struct__Fitness__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_int__0l__struct__Fitness__0c__struct__Fitness__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_int__0l__struct__Fitness__0c__struct__Fitness__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::Fitness>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::Fitness>,
            ) -> i32,
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::GinState,
                u16,
                crate::__pgrx_c_bindings::Datum,
                i8,
                crate::__pgrx_c_bindings::Datum,
                i8,
            ) -> i32,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::GinState,
                u16,
                crate::__pgrx_c_bindings::Datum,
                i8,
                crate::__pgrx_c_bindings::Datum,
                i8,
            ) -> i32,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_int__0l__struct__GinState__0p__0c__unsigned__short__0c__unsigned__long__0c__signed__char__0c__unsigned__long__0c__signed__char__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_int__0l__struct__GinState__0p__0c__unsigned__short__0c__unsigned__long__0c__signed__char__0c__unsigned__long__0c__signed__char__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_int__0l__struct__GinState__0p__0c__unsigned__short__0c__unsigned__long__0c__signed__char__0c__unsigned__long__0c__signed__char__0r {
 type Physical = PhysicalFunction_C_unwind_6<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::GinState, u16, crate::__pgrx_c_bindings::Datum, i8, crate::__pgrx_c_bindings::Datum, i8) -> i32>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::MultiXactId) -> i64,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::MultiXactId) -> i64,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_long__0l__unsigned__int__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_long__0l__unsigned__int__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_long__0l__unsigned__int__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::MultiXactId) -> i64,
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn() -> i64>>() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn() -> i64>>() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_long__0l__void__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_long__0l__void__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_long__0l__void__0r {
    type Physical =
        PhysicalFunction_C_unwind_0<::core::option::Option<unsafe extern "C-unwind" fn() -> i64>>;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__HeapTupleData__0p__0l__enum__SysCacheIdentifier__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_struct__HeapTupleData__0p__0l__enum__SysCacheIdentifier__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_struct__HeapTupleData__0p__0l__enum__SysCacheIdentifier__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(i32, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum) -> *mut crate::__pgrx_c_bindings::HeapTupleData>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                u32,
            )
                -> *mut crate::__pgrx_c_bindings::IndexFetchTableData,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                u32,
            )
                -> *mut crate::__pgrx_c_bindings::IndexFetchTableData,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__IndexFetchTableData__0p__0l__struct__RelationData__0p__0c__unsigned__int__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_struct__IndexFetchTableData__0p__0l__struct__RelationData__0p__0c__unsigned__int__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_struct__IndexFetchTableData__0p__0l__struct__RelationData__0p__0c__unsigned__int__0r {
 type Physical = PhysicalFunction_C_unwind_2<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, u32) -> *mut crate::__pgrx_c_bindings::IndexFetchTableData>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
            ) -> *mut crate::__pgrx_c_bindings::List,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
            ) -> *mut crate::__pgrx_c_bindings::List,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__List__0p__0l__enum__NodeTag__0c__union__ListCell__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_struct__List__0p__0l__enum__NodeTag__0c__union__ListCell__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_struct__List__0p__0l__enum__NodeTag__0c__union__ListCell__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
            ) -> *mut crate::__pgrx_c_bindings::List,
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
            ) -> *mut crate::__pgrx_c_bindings::List,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
            ) -> *mut crate::__pgrx_c_bindings::List,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__List__0p__0l__enum__NodeTag__0c__union__ListCell__0c__union__ListCell__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed
    for Signature_struct__List__0p__0l__enum__NodeTag__0c__union__ListCell__0c__union__ListCell__0r
{
}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_struct__List__0p__0l__enum__NodeTag__0c__union__ListCell__0c__union__ListCell__0r
{
    type Physical = PhysicalFunction_C_unwind_3<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
            ) -> *mut crate::__pgrx_c_bindings::List,
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
            ) -> *mut crate::__pgrx_c_bindings::List,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
            ) -> *mut crate::__pgrx_c_bindings::List,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__List__0p__0l__enum__NodeTag__0c__union__ListCell__0c__union__ListCell__0c__union__ListCell__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_struct__List__0p__0l__enum__NodeTag__0c__union__ListCell__0c__union__ListCell__0c__union__ListCell__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_struct__List__0p__0l__enum__NodeTag__0c__union__ListCell__0c__union__ListCell__0c__union__ListCell__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(u32, ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>, ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>, ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>) -> *mut crate::__pgrx_c_bindings::List>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
            ) -> *mut crate::__pgrx_c_bindings::List,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
            ) -> *mut crate::__pgrx_c_bindings::List,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__List__0p__0l__enum__NodeTag__0c__union__ListCell__0c__union__ListCell__0c__union__ListCell__0c__union__ListCell__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_struct__List__0p__0l__enum__NodeTag__0c__union__ListCell__0c__union__ListCell__0c__union__ListCell__0c__union__ListCell__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_struct__List__0p__0l__enum__NodeTag__0c__union__ListCell__0c__union__ListCell__0c__union__ListCell__0c__union__ListCell__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(u32, ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>, ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>, ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>, ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>) -> *mut crate::__pgrx_c_bindings::List>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
            ) -> *mut crate::__pgrx_c_bindings::List,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
            ) -> *mut crate::__pgrx_c_bindings::List,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__List__0p__0l__enum__NodeTag__0c__union__ListCell__0c__union__ListCell__0c__union__ListCell__0c__union__ListCell__0c__union__ListCell__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_struct__List__0p__0l__enum__NodeTag__0c__union__ListCell__0c__union__ListCell__0c__union__ListCell__0c__union__ListCell__0c__union__ListCell__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_struct__List__0p__0l__enum__NodeTag__0c__union__ListCell__0c__union__ListCell__0c__union__ListCell__0c__union__ListCell__0c__union__ListCell__0r {
 type Physical = PhysicalFunction_C_unwind_6<::core::option::Option<unsafe extern "C-unwind" fn(u32, ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>, ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>, ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>, ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>, ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>) -> *mut crate::__pgrx_c_bindings::List>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::Var,
                *mut crate::__pgrx_c_bindings::replace_rte_variables_context,
            ) -> *mut crate::__pgrx_c_bindings::Node,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const crate::__pgrx_c_bindings::Var,
                *mut crate::__pgrx_c_bindings::replace_rte_variables_context,
            ) -> *mut crate::__pgrx_c_bindings::Node,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__Node__0p__0l__const__struct__Var__0p__0c__struct__replace_rte_variables_context__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_struct__Node__0p__0l__const__struct__Var__0p__0c__struct__replace_rte_variables_context__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_struct__Node__0p__0l__const__struct__Var__0p__0c__struct__replace_rte_variables_context__0p__0r {
 type Physical = PhysicalFunction_C_unwind_2<::core::option::Option<unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::Var, *mut crate::__pgrx_c_bindings::replace_rte_variables_context) -> *mut crate::__pgrx_c_bindings::Node>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::Query,
                *const ::core::ffi::c_char,
                i32,
                *mut crate::__pgrx_c_bindings::ParamListInfoData,
                *mut crate::__pgrx_c_bindings::ExplainState,
            ) -> *mut crate::__pgrx_c_bindings::PlannedStmt,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::Query,
                *const ::core::ffi::c_char,
                i32,
                *mut crate::__pgrx_c_bindings::ParamListInfoData,
                *mut crate::__pgrx_c_bindings::ExplainState,
            ) -> *mut crate::__pgrx_c_bindings::PlannedStmt,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__PlannedStmt__0p__0l__struct__Query__0p__0c__const__char__0p__0c__int__0c__struct__ParamListInfoData__0p__0c__struct__ExplainState__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_struct__PlannedStmt__0p__0l__struct__Query__0p__0c__const__char__0p__0c__int__0c__struct__ParamListInfoData__0p__0c__struct__ExplainState__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_struct__PlannedStmt__0p__0l__struct__Query__0p__0c__const__char__0p__0c__int__0c__struct__ParamListInfoData__0p__0c__struct__ExplainState__0p__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::Query, *const ::core::ffi::c_char, i32, *mut crate::__pgrx_c_bindings::ParamListInfoData, *mut crate::__pgrx_c_bindings::ExplainState) -> *mut crate::__pgrx_c_bindings::PlannedStmt>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                i32,
                *mut crate::__pgrx_c_bindings::ScanKeyData,
                *mut crate::__pgrx_c_bindings::ParallelTableScanDescData,
                u32,
                u32,
            )
                -> *mut crate::__pgrx_c_bindings::TableScanDescData,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                i32,
                *mut crate::__pgrx_c_bindings::ScanKeyData,
                *mut crate::__pgrx_c_bindings::ParallelTableScanDescData,
                u32,
                u32,
            )
                -> *mut crate::__pgrx_c_bindings::TableScanDescData,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0c__struct__SnapshotData__0p__0c__int__0c__struct__ScanKeyData__0p__0c__struct__ParallelTableScanDescData__0p__0c__unsigned__int__0c__unsigned__int__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0c__struct__SnapshotData__0p__0c__int__0c__struct__ScanKeyData__0p__0c__struct__ParallelTableScanDescData__0p__0c__unsigned__int__0c__unsigned__int__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_struct__TableScanDescData__0p__0l__struct__RelationData__0p__0c__struct__SnapshotData__0p__0c__int__0c__struct__ScanKeyData__0p__0c__struct__ParallelTableScanDescData__0p__0c__unsigned__int__0c__unsigned__int__0r {
 type Physical = PhysicalFunction_C_unwind_7<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::SnapshotData, i32, *mut crate::__pgrx_c_bindings::ScanKeyData, *mut crate::__pgrx_c_bindings::ParallelTableScanDescData, u32, u32) -> *mut crate::__pgrx_c_bindings::TableScanDescData>>;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_struct__catclist__0p__0l__enum__SysCacheIdentifier__0c__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_struct__catclist__0p__0l__enum__SysCacheIdentifier__0c__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_struct__catclist__0p__0l__enum__SysCacheIdentifier__0c__int__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(i32, i32, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum) -> *mut crate::__pgrx_c_bindings::catclist>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                i32,
            ) -> ::core::mem::MaybeUninit<
                crate::__pgrx_c_bindings::ListCell,
            >,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                i32,
            ) -> ::core::mem::MaybeUninit<
                crate::__pgrx_c_bindings::ListCell,
            >,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_union__ListCell__0l__int__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_union__ListCell__0l__int__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_union__ListCell__0l__int__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                i32,
            ) -> ::core::mem::MaybeUninit<
                crate::__pgrx_c_bindings::ListCell,
            >,
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
            ) -> ::core::mem::MaybeUninit<
                crate::__pgrx_c_bindings::ListCell,
            >,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
            ) -> ::core::mem::MaybeUninit<
                crate::__pgrx_c_bindings::ListCell,
            >,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_union__ListCell__0l__void__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_union__ListCell__0l__void__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_union__ListCell__0l__void__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_void,
            ) -> ::core::mem::MaybeUninit<
                crate::__pgrx_c_bindings::ListCell,
            >,
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn(i8) -> u8>>() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn(i8) -> u8>>() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__char__0l__char__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_unsigned__char__0l__char__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_unsigned__char__0l__char__0r {
    type Physical =
        PhysicalFunction_C_unwind_1<::core::option::Option<unsafe extern "C-unwind" fn(i8) -> u8>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                *mut crate::__pgrx_c_bindings::pg_locale_struct,
            ) -> u32,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                *mut crate::__pgrx_c_bindings::pg_locale_struct,
            ) -> u32,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__int__0l__unsigned__int__0c__struct__pg_locale_struct__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed
    for Signature_unsigned__int__0l__unsigned__int__0c__struct__pg_locale_struct__0p__0r
{
}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__int__0l__unsigned__int__0c__struct__pg_locale_struct__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                u32,
                *mut crate::__pgrx_c_bindings::pg_locale_struct,
            ) -> u32,
        >,
    >;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__int__0l__enum__SysCacheIdentifier__0c__short__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_unsigned__int__0l__enum__SysCacheIdentifier__0c__short__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_unsigned__int__0l__enum__SysCacheIdentifier__0c__short__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_6<::core::option::Option<unsafe extern "C-unwind" fn(i32, i16, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum) -> crate::__pgrx_c_bindings::Oid>>;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__int__0l__enum__SysCacheIdentifier__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_unsigned__int__0l__enum__SysCacheIdentifier__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_unsigned__int__0l__enum__SysCacheIdentifier__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(i32, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum, crate::__pgrx_c_bindings::Datum) -> u32>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::varatt_external>,
            ) -> u32,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::varatt_external>,
            ) -> u32,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__int__0l__struct__varatt_external__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_unsigned__int__0l__struct__varatt_external__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__int__0l__struct__varatt_external__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::varatt_external>,
            ) -> u32,
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_char,
                usize,
                *const ::core::ffi::c_char,
                *mut crate::__pgrx_c_bindings::pg_locale_struct,
            ) -> usize,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_char,
                usize,
                *const ::core::ffi::c_char,
                *mut crate::__pgrx_c_bindings::pg_locale_struct,
            ) -> usize,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__char__0p__0c__unsigned__long__0c__const__char__0p__0c__struct__pg_locale_struct__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_unsigned__long__0l__char__0p__0c__unsigned__long__0c__const__char__0p__0c__struct__pg_locale_struct__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__char__0p__0c__unsigned__long__0c__const__char__0p__0c__struct__pg_locale_struct__0p__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_char, usize, *const ::core::ffi::c_char, *mut crate::__pgrx_c_bindings::pg_locale_struct) -> usize>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_char,
                usize,
                *const ::core::ffi::c_char,
                usize,
                *mut crate::__pgrx_c_bindings::pg_locale_struct,
            ) -> usize,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_char,
                usize,
                *const ::core::ffi::c_char,
                usize,
                *mut crate::__pgrx_c_bindings::pg_locale_struct,
            ) -> usize,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__char__0p__0c__unsigned__long__0c__const__char__0p__0c__unsigned__long__0c__struct__pg_locale_struct__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_unsigned__long__0l__char__0p__0c__unsigned__long__0c__const__char__0p__0c__unsigned__long__0c__struct__pg_locale_struct__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__char__0p__0c__unsigned__long__0c__const__char__0p__0c__unsigned__long__0c__struct__pg_locale_struct__0p__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_char, usize, *const ::core::ffi::c_char, usize, *mut crate::__pgrx_c_bindings::pg_locale_struct) -> usize>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<*mut bool>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<*mut bool>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<*mut bool>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<*mut bool>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_char,
                *mut u32,
                bool,
                i32,
                u8,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *const ::core::ffi::c_char,
                *mut u32,
                bool,
                i32,
                u8,
            ) -> crate::__pgrx_c_bindings::Datum,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__const__char__0p__0c__unsigned__int__0p__0c__0x5f426f6f6c__0c__int__0c__unsigned__char__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_unsigned__long__0l__const__char__0p__0c__unsigned__int__0p__0c__0x5f426f6f6c__0c__int__0c__unsigned__char__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__const__char__0p__0c__unsigned__int__0p__0c__0x5f426f6f6c__0c__int__0c__unsigned__char__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_char, *mut u32, bool, i32, u8) -> crate::__pgrx_c_bindings::Datum>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_void) -> usize>,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_void) -> usize>,
    >() == 8
);
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__const__void__0p__0r {
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(*const ::core::ffi::c_void) -> usize>,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::PageXLogRecPtr) -> u64,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::PageXLogRecPtr) -> u64,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__const__volatile__PageXLogRecPtr__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_unsigned__long__0l__const__volatile__PageXLogRecPtr__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__long__0l__const__volatile__PageXLogRecPtr__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(*const crate::__pgrx_c_bindings::PageXLogRecPtr) -> u64,
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<::core::option::Option<unsafe extern "C-unwind" fn(u32) -> usize>>()
        == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<::core::option::Option<unsafe extern "C-unwind" fn(u32) -> usize>>()
        == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__enum__vartag_external__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_unsigned__long__0l__enum__vartag_external__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__long__0l__enum__vartag_external__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<unsafe extern "C-unwind" fn(u32) -> usize>,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::BufferDesc,
                u64,
                u64,
                u64,
                i32,
            ) -> u64,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::BufferDesc,
                u64,
                u64,
                u64,
                i32,
            ) -> u64,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__struct__BufferDesc__0p__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__int__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_unsigned__long__0l__struct__BufferDesc__0p__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__int__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_unsigned__long__0l__struct__BufferDesc__0p__0c__unsigned__long__0c__unsigned__long__0c__unsigned__long__0c__int__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::BufferDesc, u64, u64, u64, i32) -> u64>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::varatt_external>,
            ) -> usize,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::varatt_external>,
            ) -> usize,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__long__0l__struct__varatt_external__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_unsigned__long__0l__struct__varatt_external__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_unsigned__long__0l__struct__varatt_external__0r
{
    type Physical = PhysicalFunction_C_unwind_1<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::varatt_external>,
            ) -> usize,
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_char,
                *const ::core::ffi::c_void,
                usize,
                u16,
                i32,
            ) -> u16,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut ::core::ffi::c_char,
                *const ::core::ffi::c_void,
                usize,
                u16,
                i32,
            ) -> u16,
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_unsigned__short__0l__char__0p__0c__const__void__0p__0c__unsigned__long__0c__unsigned__short__0c__int__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_unsigned__short__0l__char__0p__0c__const__void__0p__0c__unsigned__long__0c__unsigned__short__0c__int__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_unsigned__short__0l__char__0p__0c__const__void__0p__0c__unsigned__long__0c__unsigned__short__0c__int__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(*mut ::core::ffi::c_char, *const ::core::ffi::c_void, usize, u16, i32) -> u16>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(i32, *const crate::__pgrx_c_bindings::pg_signal_info) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(i32, *const crate::__pgrx_c_bindings::pg_signal_info) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__int__0c__const__struct__pg_signal_info__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0l__int__0c__const__struct__pg_signal_info__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature
    for Signature_void__0l__int__0c__const__struct__pg_signal_info__0p__0r
{
    type Physical = PhysicalFunction_C_unwind_2<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(i32, *const crate::__pgrx_c_bindings::pg_signal_info) -> (),
        >,
    >;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::FullTransactionId,
                u16,
                *mut ::core::ffi::c_void,
                u32,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                crate::__pgrx_c_bindings::FullTransactionId,
                u16,
                *mut ::core::ffi::c_void,
                u32,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__FullTransactionId__0c__unsigned__short__0c__void__0p__0c__unsigned__int__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0l__struct__FullTransactionId__0c__unsigned__short__0c__void__0p__0c__unsigned__int__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__FullTransactionId__0c__unsigned__short__0c__void__0p__0c__unsigned__int__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(crate::__pgrx_c_bindings::FullTransactionId, u16, *mut ::core::ffi::c_void, u32) -> ()>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ParseState,
                *mut crate::__pgrx_c_bindings::Query,
                *const crate::__pgrx_c_bindings::JumbleState,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::ParseState,
                *mut crate::__pgrx_c_bindings::Query,
                *const crate::__pgrx_c_bindings::JumbleState,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__ParseState__0p__0c__struct__Query__0p__0c__const__struct__JumbleState__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0l__struct__ParseState__0p__0c__struct__Query__0p__0c__const__struct__JumbleState__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__ParseState__0p__0c__struct__Query__0p__0c__const__struct__JumbleState__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::ParseState, *mut crate::__pgrx_c_bindings::Query, *const crate::__pgrx_c_bindings::JumbleState) -> ()>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerGlobal,
                *mut crate::__pgrx_c_bindings::Query,
                *const ::core::ffi::c_char,
                i32,
                *mut f64,
                *mut crate::__pgrx_c_bindings::ExplainState,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerGlobal,
                *mut crate::__pgrx_c_bindings::Query,
                *const ::core::ffi::c_char,
                i32,
                *mut f64,
                *mut crate::__pgrx_c_bindings::ExplainState,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__PlannerGlobal__0p__0c__struct__Query__0p__0c__const__char__0p__0c__int__0c__double__0p__0c__struct__ExplainState__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0l__struct__PlannerGlobal__0p__0c__struct__Query__0p__0c__const__char__0p__0c__int__0c__double__0p__0c__struct__ExplainState__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__PlannerGlobal__0p__0c__struct__Query__0p__0c__const__char__0p__0c__int__0c__double__0p__0c__struct__ExplainState__0p__0r {
 type Physical = PhysicalFunction_C_unwind_6<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PlannerGlobal, *mut crate::__pgrx_c_bindings::Query, *const ::core::ffi::c_char, i32, *mut f64, *mut crate::__pgrx_c_bindings::ExplainState) -> ()>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerGlobal,
                *mut crate::__pgrx_c_bindings::Query,
                *const ::core::ffi::c_char,
                *mut crate::__pgrx_c_bindings::PlannedStmt,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerGlobal,
                *mut crate::__pgrx_c_bindings::Query,
                *const ::core::ffi::c_char,
                *mut crate::__pgrx_c_bindings::PlannedStmt,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__PlannerGlobal__0p__0c__struct__Query__0p__0c__const__char__0p__0c__struct__PlannedStmt__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0l__struct__PlannerGlobal__0p__0c__struct__Query__0p__0c__const__char__0p__0c__struct__PlannedStmt__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__PlannerGlobal__0p__0c__struct__Query__0p__0c__const__char__0p__0c__struct__PlannedStmt__0p__0r {
 type Physical = PhysicalFunction_C_unwind_4<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PlannerGlobal, *mut crate::__pgrx_c_bindings::Query, *const ::core::ffi::c_char, *mut crate::__pgrx_c_bindings::PlannedStmt) -> ()>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerInfo,
                *mut crate::__pgrx_c_bindings::RelOptInfo,
                *mut crate::__pgrx_c_bindings::RangeTblEntry,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerInfo,
                *mut crate::__pgrx_c_bindings::RelOptInfo,
                *mut crate::__pgrx_c_bindings::RangeTblEntry,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__RangeTblEntry__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__RangeTblEntry__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__RangeTblEntry__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PlannerInfo, *mut crate::__pgrx_c_bindings::RelOptInfo, *mut crate::__pgrx_c_bindings::RangeTblEntry) -> ()>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerInfo,
                *mut crate::__pgrx_c_bindings::RelOptInfo,
                *mut crate::__pgrx_c_bindings::RelOptInfo,
                *mut crate::__pgrx_c_bindings::RelOptInfo,
                *mut crate::__pgrx_c_bindings::SpecialJoinInfo,
                *mut crate::__pgrx_c_bindings::List,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::PlannerInfo,
                *mut crate::__pgrx_c_bindings::RelOptInfo,
                *mut crate::__pgrx_c_bindings::RelOptInfo,
                *mut crate::__pgrx_c_bindings::RelOptInfo,
                *mut crate::__pgrx_c_bindings::SpecialJoinInfo,
                *mut crate::__pgrx_c_bindings::List,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__SpecialJoinInfo__0p__0c__struct__List__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__SpecialJoinInfo__0p__0c__struct__List__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__PlannerInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__RelOptInfo__0p__0c__struct__SpecialJoinInfo__0p__0c__struct__List__0p__0r {
 type Physical = PhysicalFunction_C_unwind_6<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::PlannerInfo, *mut crate::__pgrx_c_bindings::RelOptInfo, *mut crate::__pgrx_c_bindings::RelOptInfo, *mut crate::__pgrx_c_bindings::RelOptInfo, *mut crate::__pgrx_c_bindings::SpecialJoinInfo, *mut crate::__pgrx_c_bindings::List) -> ()>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *const crate::__pgrx_c_bindings::VacuumParams,
                *mut crate::__pgrx_c_bindings::BufferAccessStrategyData,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *const crate::__pgrx_c_bindings::VacuumParams,
                *mut crate::__pgrx_c_bindings::BufferAccessStrategyData,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__RelationData__0p__0c__const__struct__VacuumParams__0p__0c__struct__BufferAccessStrategyData__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0l__struct__RelationData__0p__0c__const__struct__VacuumParams__0p__0c__struct__BufferAccessStrategyData__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__RelationData__0p__0c__const__struct__VacuumParams__0p__0c__struct__BufferAccessStrategyData__0p__0r {
 type Physical = PhysicalFunction_C_unwind_3<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *const crate::__pgrx_c_bindings::VacuumParams, *mut crate::__pgrx_c_bindings::BufferAccessStrategyData) -> ()>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<*const crate::__pgrx_c_bindings::VacuumParams>() == 8);
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::align_of::<*const crate::__pgrx_c_bindings::VacuumParams>() == 8);
#[cfg(feature = "pg19")]
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<A0: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>, c::expression::ReadWrite>>, A1: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::VacuumParams>, c::expression::ReadOnly>>, A2: c::expression::ImplicitTo<c::expression::CPointer<c::expression::COpaque<crate::__pgrx_c_bindings::BufferAccessStrategyData>, c::expression::ReadWrite>>> c::expression::Call<(A0,A1,A2,)> for Signature_void__0l__struct__RelationData__0p__0c__const__struct__VacuumParams__0p__0c__struct__BufferAccessStrategyData__0p__0r {
 type Output = ();
 unsafe fn call(pointer: Self::Pointer, args: (A0,A1,A2,)) -> Self::Output {
let function = pointer.expect("C indirect call requires a non-null function pointer");
const { assert!(!::core::mem::needs_drop::<A0>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::RelationData>());
let native0 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>, c::expression::ReadWrite>, _>(args.0));
const { assert!(!::core::mem::needs_drop::<A1>()); }
const _: () = assert!(!::core::mem::needs_drop::<*const crate::__pgrx_c_bindings::VacuumParams>());
let native1 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::VacuumParams>, c::expression::ReadOnly> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::VacuumParams>, c::expression::ReadOnly>, _>(args.1));
const { assert!(!::core::mem::needs_drop::<A2>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::BufferAccessStrategyData>());
let native2 = <c::expression::CPointer<c::expression::COpaque<crate::__pgrx_c_bindings::BufferAccessStrategyData>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::COpaque<crate::__pgrx_c_bindings::BufferAccessStrategyData>, c::expression::ReadWrite>, _>(args.2));
// SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
let result = unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0, native1, native2)) };
result
}
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::RelationData,
                bool,
                crate::__pgrx_c_bindings::TransactionId,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                *mut crate::__pgrx_c_bindings::TransactionId,
                *mut crate::__pgrx_c_bindings::MultiXactId,
                *mut f64,
                *mut f64,
                *mut f64,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::RelationData,
                bool,
                crate::__pgrx_c_bindings::TransactionId,
                *mut crate::__pgrx_c_bindings::SnapshotData,
                *mut crate::__pgrx_c_bindings::TransactionId,
                *mut crate::__pgrx_c_bindings::MultiXactId,
                *mut f64,
                *mut f64,
                *mut f64,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__RelationData__0p__0c__struct__RelationData__0p__0c__struct__RelationData__0p__0c__0x5f426f6f6c__0c__unsigned__int__0c__struct__SnapshotData__0p__0c__unsigned__int__0p__0c__unsigned__int__0p__0c__double__0p__0c__double__0p__0c__double__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0l__struct__RelationData__0p__0c__struct__RelationData__0p__0c__struct__RelationData__0p__0c__0x5f426f6f6c__0c__unsigned__int__0c__struct__SnapshotData__0p__0c__unsigned__int__0p__0c__unsigned__int__0p__0c__double__0p__0c__double__0p__0c__double__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__RelationData__0p__0c__struct__RelationData__0p__0c__struct__RelationData__0p__0c__0x5f426f6f6c__0c__unsigned__int__0c__struct__SnapshotData__0p__0c__unsigned__int__0p__0c__unsigned__int__0p__0c__double__0p__0c__double__0p__0c__double__0p__0r {
 type Physical = PhysicalFunction_C_unwind_11<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::RelationData, bool, crate::__pgrx_c_bindings::TransactionId, *mut crate::__pgrx_c_bindings::SnapshotData, *mut crate::__pgrx_c_bindings::TransactionId, *mut crate::__pgrx_c_bindings::MultiXactId, *mut f64, *mut f64, *mut f64) -> ()>>;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::SnapshotData>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::SnapshotData>() == 8);
#[cfg(feature = "pg19")]
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<A0: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>, c::expression::ReadWrite>>, A1: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>, c::expression::ReadWrite>>, A2: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>, c::expression::ReadWrite>>, A3: c::expression::ImplicitTo<c::CBool>, A4: c::expression::ImplicitTo<c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>>, A5: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData>, c::expression::ReadWrite>>, A6: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>, c::expression::ReadWrite>>, A7: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::MultiXactId>, c::expression::ReadWrite>>, A8: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CDouble, c::expression::ReadWrite>>, A9: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CDouble, c::expression::ReadWrite>>, A10: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CDouble, c::expression::ReadWrite>>> c::expression::Call<(A0,A1,A2,A3,A4,A5,A6,A7,A8,A9,A10,)> for Signature_void__0l__struct__RelationData__0p__0c__struct__RelationData__0p__0c__struct__RelationData__0p__0c__0x5f426f6f6c__0c__unsigned__int__0c__struct__SnapshotData__0p__0c__unsigned__int__0p__0c__unsigned__int__0p__0c__double__0p__0c__double__0p__0c__double__0p__0r {
 type Output = ();
 unsafe fn call(pointer: Self::Pointer, args: (A0,A1,A2,A3,A4,A5,A6,A7,A8,A9,A10,)) -> Self::Output {
let function = pointer.expect("C indirect call requires a non-null function pointer");
const { assert!(!::core::mem::needs_drop::<A0>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::RelationData>());
let native0 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>, c::expression::ReadWrite>, _>(args.0));
const { assert!(!::core::mem::needs_drop::<A1>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::RelationData>());
let native1 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>, c::expression::ReadWrite>, _>(args.1));
const { assert!(!::core::mem::needs_drop::<A2>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::RelationData>());
let native2 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>, c::expression::ReadWrite>, _>(args.2));
const { assert!(!::core::mem::needs_drop::<A3>()); }
const _: () = assert!(!::core::mem::needs_drop::<bool>());
let native3 = <c::CBool as c::expression::CType>::into_storage(c::expression::implicit::<c::CBool, _>(args.3));
const { assert!(!::core::mem::needs_drop::<A4>()); }
const _: () = assert!(!::core::mem::needs_drop::<crate::__pgrx_c_bindings::TransactionId>());
let native4 = <c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>, _>(args.4));
const { assert!(!::core::mem::needs_drop::<A5>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::SnapshotData>());
let native5 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::SnapshotData>, c::expression::ReadWrite>, _>(args.5));
const { assert!(!::core::mem::needs_drop::<A6>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::TransactionId>());
let native6 = <c::expression::CPointer<c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::TransactionId>, c::expression::ReadWrite>, _>(args.6));
const { assert!(!::core::mem::needs_drop::<A7>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::MultiXactId>());
let native7 = <c::expression::CPointer<c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::MultiXactId>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::MultiXactId>, c::expression::ReadWrite>, _>(args.7));
const { assert!(!::core::mem::needs_drop::<A8>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut f64>());
let native8 = <c::expression::CPointer<c::expression::CDouble, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CDouble, c::expression::ReadWrite>, _>(args.8));
const { assert!(!::core::mem::needs_drop::<A9>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut f64>());
let native9 = <c::expression::CPointer<c::expression::CDouble, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CDouble, c::expression::ReadWrite>, _>(args.9));
const { assert!(!::core::mem::needs_drop::<A10>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut f64>());
let native10 = <c::expression::CPointer<c::expression::CDouble, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CDouble, c::expression::ReadWrite>, _>(args.10));
// SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
let result = unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0, native1, native2, native3, native4, native5, native6, native7, native8, native9, native10)) };
result
}
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut *mut crate::__pgrx_c_bindings::TupleTableSlot,
                i32,
                u32,
                u32,
                *mut crate::__pgrx_c_bindings::BulkInsertStateData,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut *mut crate::__pgrx_c_bindings::TupleTableSlot,
                i32,
                u32,
                u32,
                *mut crate::__pgrx_c_bindings::BulkInsertStateData,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__RelationData__0p__0c__struct__TupleTableSlot__0p__0p__0c__int__0c__unsigned__int__0c__unsigned__int__0c__struct__BulkInsertStateData__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0l__struct__RelationData__0p__0c__struct__TupleTableSlot__0p__0p__0c__int__0c__unsigned__int__0c__unsigned__int__0c__struct__BulkInsertStateData__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__RelationData__0p__0c__struct__TupleTableSlot__0p__0p__0c__int__0c__unsigned__int__0c__unsigned__int__0c__struct__BulkInsertStateData__0p__0r {
 type Physical = PhysicalFunction_C_unwind_6<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *mut *mut crate::__pgrx_c_bindings::TupleTableSlot, i32, u32, u32, *mut crate::__pgrx_c_bindings::BulkInsertStateData) -> ()>>;
}
#[cfg(feature = "pg19")]
// SAFETY: This adapter validates null and converts arguments before the native guard, captures only destructor-free ABI storage, and decodes results after the native call.
unsafe impl<A0: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>, c::expression::ReadWrite>>, A1: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot>, c::expression::ReadWrite>, c::expression::ReadWrite>>, A2: c::expression::ImplicitTo<c::CInt>, A3: c::expression::ImplicitTo<c::CUnsignedInt>, A4: c::expression::ImplicitTo<c::CUnsignedInt>, A5: c::expression::ImplicitTo<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::BulkInsertStateData>, c::expression::ReadWrite>>> c::expression::Call<(A0,A1,A2,A3,A4,A5,)> for Signature_void__0l__struct__RelationData__0p__0c__struct__TupleTableSlot__0p__0p__0c__int__0c__unsigned__int__0c__unsigned__int__0c__struct__BulkInsertStateData__0p__0r {
 type Output = ();
 unsafe fn call(pointer: Self::Pointer, args: (A0,A1,A2,A3,A4,A5,)) -> Self::Output {
let function = pointer.expect("C indirect call requires a non-null function pointer");
const { assert!(!::core::mem::needs_drop::<A0>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::RelationData>());
let native0 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>, c::expression::ReadWrite>, _>(args.0));
const { assert!(!::core::mem::needs_drop::<A1>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut *mut crate::__pgrx_c_bindings::TupleTableSlot>());
let native1 = <c::expression::CPointer<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot>, c::expression::ReadWrite>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot>, c::expression::ReadWrite>, c::expression::ReadWrite>, _>(args.1));
const { assert!(!::core::mem::needs_drop::<A2>()); }
const _: () = assert!(!::core::mem::needs_drop::<i32>());
let native2 = <c::CInt as c::expression::CType>::into_storage(c::expression::implicit::<c::CInt, _>(args.2));
const { assert!(!::core::mem::needs_drop::<A3>()); }
const _: () = assert!(!::core::mem::needs_drop::<u32>());
let native3 = <c::CUnsignedInt as c::expression::CType>::into_storage(c::expression::implicit::<c::CUnsignedInt, _>(args.3));
const { assert!(!::core::mem::needs_drop::<A4>()); }
const _: () = assert!(!::core::mem::needs_drop::<u32>());
let native4 = <c::CUnsignedInt as c::expression::CType>::into_storage(c::expression::implicit::<c::CUnsignedInt, _>(args.4));
const { assert!(!::core::mem::needs_drop::<A5>()); }
const _: () = assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::BulkInsertStateData>());
let native5 = <c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::BulkInsertStateData>, c::expression::ReadWrite> as c::expression::CType>::into_storage(c::expression::implicit::<c::expression::CPointer<c::expression::CRecord<crate::__pgrx_c_bindings::BulkInsertStateData>, c::expression::ReadWrite>, _>(args.5));
// SAFETY: The caller establishes the exact native target contract, backend thread, and guarded callbacks. Conversions and the null check are complete; captured native storage and the function pointer have no destructors. The closure performs only the native call.
let result = unsafe { crate::ffi::pg_guard_ffi_boundary(move || function(native0, native1, native2, native3, native4, native5)) };
result
}
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
                u32,
                u32,
                *mut crate::__pgrx_c_bindings::BulkInsertStateData,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<
        ::core::option::Option<
            unsafe extern "C-unwind" fn(
                *mut crate::__pgrx_c_bindings::RelationData,
                *mut crate::__pgrx_c_bindings::TupleTableSlot,
                u32,
                u32,
                *mut crate::__pgrx_c_bindings::BulkInsertStateData,
            ) -> (),
        >,
    >() == 8
);
#[cfg(feature = "pg19")]
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Signature_void__0l__struct__RelationData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__unsigned__int__0c__struct__BulkInsertStateData__0p__0r;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for Signature_void__0l__struct__RelationData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__unsigned__int__0c__struct__BulkInsertStateData__0p__0r {}
#[cfg(feature = "pg19")]
impl c::expression::NativeFunctionSignature for Signature_void__0l__struct__RelationData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__unsigned__int__0c__struct__BulkInsertStateData__0p__0r {
 type Physical = PhysicalFunction_C_unwind_5<::core::option::Option<unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, *mut crate::__pgrx_c_bindings::TupleTableSlot, u32, u32, *mut crate::__pgrx_c_bindings::BulkInsertStateData) -> ()>>;
}
