// Shared generated C macro support.

#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PruneFreezeResult, <<c::expression::CRecord<crate::__pgrx_c_bindings::PruneFreezeResult> as c::expression::OrdinaryField<Field_newly_all_visible>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).newly_all_visible) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_newly_all_visible_frozen;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PruneFreezeResult, newly_all_visible_frozen)
        == 21
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_newly_all_visible_frozen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PruneFreezeResult>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 21;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PruneFreezeResult, <<c::expression::CRecord<crate::__pgrx_c_bindings::PruneFreezeResult> as c::expression::OrdinaryField<Field_newly_all_visible_frozen>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).newly_all_visible_frozen) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_newly_all_frozen;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PruneFreezeResult, newly_all_frozen) == 22
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_newly_all_frozen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PruneFreezeResult>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 22;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PruneFreezeResult, <<c::expression::CRecord<crate::__pgrx_c_bindings::PruneFreezeResult> as c::expression::OrdinaryField<Field_newly_all_frozen>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).newly_all_frozen) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PruneFreezeResult, hastup) == 23);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hastup>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PruneFreezeResult>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 23;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PruneFreezeResult, lpdead_items) == 24
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lpdead_items>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PruneFreezeResult>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_allsequences;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Publication, allsequences) == 17);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_allsequences>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Publication>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 17;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Publication, <<c::expression::CRecord<crate::__pgrx_c_bindings::Publication> as c::expression::OrdinaryField<Field_allsequences>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).allsequences) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Publication, pubviaroot) == 18);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pubviaroot>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Publication>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 18;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PublicationAllObjSpec>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PublicationAllObjSpec>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PublicationAllObjSpec {}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PublicationAllObjSpec, type_) == 0);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PublicationAllObjSpec>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PublicationAllObjSpec, <<c::expression::CRecord<crate::__pgrx_c_bindings::PublicationAllObjSpec> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PublicationAllObjSpec, pubobjtype) == 4
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pubobjtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PublicationAllObjSpec>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__PublicationAllObjType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PublicationAllObjSpec, <<c::expression::CRecord<crate::__pgrx_c_bindings::PublicationAllObjSpec> as c::expression::OrdinaryField<Field_pubobjtype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pubobjtype) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_except_tables;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PublicationAllObjSpec, except_tables) == 8
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_except_tables>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PublicationAllObjSpec>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PublicationAllObjSpec, <<c::expression::CRecord<crate::__pgrx_c_bindings::PublicationAllObjSpec> as c::expression::OrdinaryField<Field_except_tables>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).except_tables) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::PublicationAllObjSpec, location) == 16
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PublicationAllObjSpec>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PublicationAllObjSpec, <<c::expression::CRecord<crate::__pgrx_c_bindings::PublicationAllObjSpec> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PublicationRelInfo>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PublicationRelInfo>() == 8);
};
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PublicationRelInfo, columns) == 16);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_columns>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PublicationRelInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PublicationRelInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PublicationRelInfo> as c::expression::OrdinaryField<Field_columns>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).columns) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_except;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PublicationRelInfo, except) == 24);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_except>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PublicationRelInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PublicationRelInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::PublicationRelInfo> as c::expression::OrdinaryField<Field_except>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).except) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PublicationTable>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PublicationTable>() == 8);
};
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::PublicationTable, except) == 32);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_except>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PublicationTable>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PublicationTable, <<c::expression::CRecord<crate::__pgrx_c_bindings::PublicationTable> as c::expression::OrdinaryField<Field_except>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).except) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_query_instr_options;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::QueryDesc, query_instr_options) == 68
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_query_instr_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::QueryDesc>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 68;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::QueryDesc,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::QueryDesc> as c::expression::OrdinaryField<
        Field_query_instr_options,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).query_instr_options) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReadBuffersOperation, rel) == 0);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rel>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReadBuffersOperation>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::RelationData>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReadBuffersOperation, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReadBuffersOperation> as c::expression::OrdinaryField<Field_rel>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rel) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReadBuffersOperation, smgr) == 8);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_smgr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReadBuffersOperation>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::SMgrRelationData>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReadBuffersOperation, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReadBuffersOperation> as c::expression::OrdinaryField<Field_smgr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).smgr) };
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReadBuffersOperation>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReadBuffersOperation, nblocks) == 46);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nblocks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReadBuffersOperation>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 46;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReadBuffersOperation, nblocks_done) == 48
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nblocks_done>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReadBuffersOperation>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_foreign_io;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReadBuffersOperation, foreign_io) == 50
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_foreign_io>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReadBuffersOperation>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 50;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReadBuffersOperation, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReadBuffersOperation> as c::expression::OrdinaryField<Field_foreign_io>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).foreign_io) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RelAggInfo>() == 64);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RelAggInfo>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::RelAggInfo {}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelAggInfo, type_) == 0);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelAggInfo>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelAggInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelAggInfo> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_group_clauses;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelAggInfo, group_clauses) == 24);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_group_clauses>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelAggInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelAggInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelAggInfo> as c::expression::OrdinaryField<Field_group_clauses>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).group_clauses) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_group_exprs;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelAggInfo, group_exprs) == 32);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_group_exprs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelAggInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelAggInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelAggInfo> as c::expression::OrdinaryField<Field_group_exprs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).group_exprs) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_agg_useful;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelAggInfo, agg_useful) == 56);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_agg_useful>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelAggInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelAggInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelAggInfo> as c::expression::OrdinaryField<Field_agg_useful>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).agg_useful) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RelOptInfo>() == 504);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RelOptInfo>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, relids) == 8);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relids>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::Bitmapset>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo> as c::expression::OrdinaryField<Field_relids>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relids) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, pgs_mask) == 32);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pgs_mask>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo> as c::expression::OrdinaryField<Field_pgs_mask>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pgs_mask) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, pathlist) == 48);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pathlist>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, ppilist) == 56);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ppilist>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, partial_pathlist) == 64);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_partial_pathlist>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_unique_pathkeys;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, unique_pathkeys) == 304);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_unique_pathkeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 304;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo> as c::expression::OrdinaryField<Field_unique_pathkeys>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).unique_pathkeys) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_unique_groupclause;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, unique_groupclause) == 312
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_unique_groupclause>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 312;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo> as c::expression::OrdinaryField<Field_unique_groupclause>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).unique_groupclause) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, baserestrictinfo) == 320);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_baserestrictinfo>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 320;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, baserestrict_min_security) == 344
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_baserestrict_min_security>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 344;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, joininfo) == 352);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_joininfo>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 352;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, has_eclass_joins) == 360);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_has_eclass_joins>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 360;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, consider_partitionwise_join)
        == 361
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_consider_partitionwise_join>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 361;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_agg_info;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, agg_info) == 368);
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::RelAggInfo>() == 8);
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::RelAggInfo>() == 8);
};
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_agg_info>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::RelAggInfo>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 368;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo> as c::expression::OrdinaryField<Field_agg_info>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).agg_info) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, part_scheme) == 408);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_part_scheme>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::PartitionSchemeData>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 408;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, nparts) == 416);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nparts>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 416;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, boundinfo) == 424);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_boundinfo>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::PartitionBoundInfoData>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 424;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, partbounds_merged) == 432
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_partbounds_merged>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 432;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, partition_qual) == 440);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_partition_qual>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 440;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, part_rels) == 448);
#[cfg(feature = "pg19")]
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
    const OFFSET: usize = 448;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, partexprs) == 472);
#[cfg(feature = "pg19")]
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
    const OFFSET: usize = 472;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, nullable_partexprs) == 480
);
#[cfg(feature = "pg19")]
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
    const OFFSET: usize = 480;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::RelOptInfo, extension_state_allocated) == 496
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_extension_state_allocated>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 496;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelOptInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelOptInfo> as c::expression::OrdinaryField<Field_extension_state_allocated>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).extension_state_allocated) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ReorderBuffer>() == 384);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ReorderBuffer>() == 8);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_memExceededCount;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBuffer, memExceededCount) == 360
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_memExceededCount>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 360;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBuffer, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer> as c::expression::OrdinaryField<Field_memExceededCount>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).memExceededCount) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBuffer, totalTxns) == 368);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_totalTxns>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 368;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBuffer, totalBytes) == 376);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_totalBytes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBuffer>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 376;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReorderBufferTXN, __bindgen_anon_1) == 80
);
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ReorderBufferTXN,
    crate::__pgrx_c_bindings::ReorderBufferTXN__bindgen_ty_1,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).__bindgen_anon_1) };
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_commit_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBufferTXN, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN> as c::expression::OrdinaryField<Field_commit_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).__bindgen_anon_1.commit_time) };
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_prepare_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBufferTXN, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN> as c::expression::OrdinaryField<Field_prepare_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).__bindgen_anon_1.prepare_time) };
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_abort_time>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReorderBufferTXN, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReorderBufferTXN> as c::expression::OrdinaryField<Field_abort_time>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).__bindgen_anon_1.abort_time) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RepackDecodingState>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RepackDecodingState>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::RepackDecodingState {}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RepackDecodingState, relid) == 0);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RepackDecodingState>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RepackDecodingState, <<c::expression::CRecord<crate::__pgrx_c_bindings::RepackDecodingState> as c::expression::OrdinaryField<Field_relid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relid) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RepackStmt>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RepackStmt>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::RepackStmt {}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RepackStmt, type_) == 0);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RepackStmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RepackStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::RepackStmt> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_command;
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RepackStmt, command) == 4);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_command>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RepackStmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__RepackCommand,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RepackStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::RepackStmt> as c::expression::OrdinaryField<Field_command>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).command) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_usingindex;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RepackStmt, usingindex) == 24);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_usingindex>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RepackStmt>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RepackStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::RepackStmt> as c::expression::OrdinaryField<Field_usingindex>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).usingindex) };
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RepackStmt, params) == 32);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_params>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RepackStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RepackStmt, <<c::expression::CRecord<crate::__pgrx_c_bindings::RepackStmt> as c::expression::OrdinaryField<Field_params>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).params) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ReplOriginXactState>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ReplOriginXactState>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ReplOriginXactState {}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReplOriginXactState, origin) == 0);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_origin>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReplOriginXactState>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReplOriginXactState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReplOriginXactState> as c::expression::OrdinaryField<Field_origin>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).origin) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReplOriginXactState, origin_lsn) == 8
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_origin_lsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReplOriginXactState>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReplOriginXactState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReplOriginXactState> as c::expression::OrdinaryField<Field_origin_lsn>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).origin_lsn) };
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReplOriginXactState, origin_timestamp) == 16
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_origin_timestamp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReplOriginXactState>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReplOriginXactState, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReplOriginXactState> as c::expression::OrdinaryField<Field_origin_timestamp>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).origin_timestamp) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ReplicationSlot>() == 296);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ReplicationSlot>() == 8);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_active_proc;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ReplicationSlot, active_proc) == 4);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_active_proc>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlot>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReplicationSlot, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlot> as c::expression::OrdinaryField<Field_active_proc>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).active_proc) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_slotsync_skip_reason;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ReplicationSlot, slotsync_skip_reason) == 288
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_slotsync_skip_reason>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlot>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__SlotSyncSkipReason,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 288;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ReplicationSlot, <<c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlot> as c::expression::OrdinaryField<Field_slotsync_skip_reason>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).slotsync_skip_reason) };
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_synced>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ReplicationSlotPersistentData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 177;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_result_type;
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Result>() == 128);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Result>() == 8);
};
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Result, result_type) == 104);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_result_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Result>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__ResultType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Result,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Result> as c::expression::OrdinaryField<
        Field_result_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).result_type) };
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Result, relids) == 120);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relids>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Result>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::Bitmapset>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Result,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Result> as c::expression::OrdinaryField<
        Field_relids,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relids) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SeqScanState>() == 240);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SeqScanState>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SharedSeqScanInstrumentation>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SharedSeqScanInstrumentation>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SharedSeqScanInstrumentation {}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SharedSeqScanInstrumentation, num_workers)
        == 0
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_workers>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SharedSeqScanInstrumentation>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SharedSeqScanInstrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::SharedSeqScanInstrumentation> as c::expression::OrdinaryField<Field_num_workers>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_workers) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::SharedTidRangeScanInstrumentation>() == 8
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::SharedTidRangeScanInstrumentation>() == 8
    );
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SharedTidRangeScanInstrumentation {}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::SharedTidRangeScanInstrumentation,
        num_workers
    ) == 0
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_workers>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SharedTidRangeScanInstrumentation>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SharedTidRangeScanInstrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::SharedTidRangeScanInstrumentation> as c::expression::OrdinaryField<Field_num_workers>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_workers) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SlruDesc>() == 120);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SlruDesc>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SlruDesc {}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SlruDesc, nbanks) == 112);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nbanks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SlruDesc>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SlruDesc,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SlruDesc> as c::expression::OrdinaryField<
        Field_nbanks,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nbanks) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_syn_righthand;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SpecialJoinInfo, syn_righthand) == 32
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_syn_righthand>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SpecialJoinInfo>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::Bitmapset>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SpecialJoinInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::SpecialJoinInfo> as c::expression::OrdinaryField<Field_syn_righthand>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).syn_righthand) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::StdRdOptions>() == 136);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::StdRdOptions>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::StdRdOptions, user_catalog_table) == 112
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_user_catalog_table>
    for c::expression::CRecord<crate::__pgrx_c_bindings::StdRdOptions>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::StdRdOptions, parallel_workers) == 116
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parallel_workers>
    for c::expression::CRecord<crate::__pgrx_c_bindings::StdRdOptions>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 116;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::StdRdOptions, vacuum_index_cleanup) == 120
);
#[cfg(feature = "pg19")]
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
    const OFFSET: usize = 120;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::StdRdOptions, vacuum_truncate) == 124
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vacuum_truncate>
    for c::expression::CRecord<crate::__pgrx_c_bindings::StdRdOptions>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__pg_ternary,
        c::CInt,
        i32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 124;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SubPlan>() == 104);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SubPlan>() == 8);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_isInitPlan;
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubPlan, isInitPlan) == 52);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isInitPlan>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubPlan>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SubPlan,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SubPlan> as c::expression::OrdinaryField<
        Field_isInitPlan,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isInitPlan) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubPlan, useHashTable) == 53);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_useHashTable>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubPlan>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 53;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubPlan, unknownEqFalse) == 54);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_unknownEqFalse>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubPlan>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 54;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubPlan, parallel_safe) == 55);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parallel_safe>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubPlan>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 55;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubPlan, disabled_nodes) == 80);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_disabled_nodes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubPlan>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SubPlan,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SubPlan> as c::expression::OrdinaryField<
        Field_disabled_nodes,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).disabled_nodes) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SubPlanRTInfo>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SubPlanRTInfo>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SubPlanRTInfo {}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubPlanRTInfo, type_) == 0);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubPlanRTInfo>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SubPlanRTInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::SubPlanRTInfo> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_rtoffset;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubPlanRTInfo, rtoffset) == 16);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_rtoffset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubPlanRTInfo>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SubPlanRTInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::SubPlanRTInfo> as c::expression::OrdinaryField<Field_rtoffset>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).rtoffset) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubPlanRTInfo, dummy) == 20);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dummy>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubPlanRTInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SubPlanRTInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::SubPlanRTInfo> as c::expression::OrdinaryField<Field_dummy>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dummy) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SubPlanState>() == 184);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SubPlanState>() == 8);
};
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubPlanState, numCols) == 120);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numCols>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubPlanState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Subscription>() == 96);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Subscription>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Subscription, oid) == 8);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Subscription>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Subscription, dbid) == 12);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dbid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Subscription>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Subscription, skiplsn) == 16);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_skiplsn>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Subscription>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Subscription, owner) == 32);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_owner>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Subscription>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Subscription, ownersuperuser) == 36);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ownersuperuser>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Subscription>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 36;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Subscription, enabled) == 37);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_enabled>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Subscription>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 37;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Subscription, binary) == 38);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_binary>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Subscription>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 38;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Subscription, stream) == 39);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stream>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Subscription>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 39;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Subscription, twophasestate) == 40);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_twophasestate>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Subscription>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Subscription, disableonerr) == 41);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_disableonerr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Subscription>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 41;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::Subscription, passwordrequired) == 42
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_passwordrequired>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Subscription>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 42;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Subscription, runasowner) == 43);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_runasowner>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Subscription>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 43;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Subscription, failover) == 44);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_failover>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Subscription>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 44;
}
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_retaindeadtuples;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::Subscription, retaindeadtuples) == 45
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_retaindeadtuples>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Subscription>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 45;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Subscription, <<c::expression::CRecord<crate::__pgrx_c_bindings::Subscription> as c::expression::OrdinaryField<Field_retaindeadtuples>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).retaindeadtuples) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_maxretention;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Subscription, maxretention) == 48);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_maxretention>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Subscription>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Subscription, <<c::expression::CRecord<crate::__pgrx_c_bindings::Subscription> as c::expression::OrdinaryField<Field_maxretention>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).maxretention) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_retentionactive;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Subscription, retentionactive) == 52);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_retentionactive>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Subscription>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::Subscription, <<c::expression::CRecord<crate::__pgrx_c_bindings::Subscription> as c::expression::OrdinaryField<Field_retentionactive>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).retentionactive) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Subscription, publications) == 80);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_publications>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Subscription>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SupportRequestInlineInFrom>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SupportRequestInlineInFrom>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SupportRequestInlineInFrom {}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SupportRequestInlineInFrom, type_) == 0
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestInlineInFrom>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SupportRequestInlineInFrom, <<c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestInlineInFrom> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SupportRequestSimplifyAggref>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SupportRequestSimplifyAggref>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SupportRequestSimplifyAggref {}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SupportRequestSimplifyAggref, type_) == 0
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestSimplifyAggref>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SupportRequestSimplifyAggref, <<c::expression::CRecord<crate::__pgrx_c_bindings::SupportRequestSimplifyAggref> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TableScanDescData>() == 72);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TableScanDescData>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TidRangeScanState>() == 264);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TidRangeScanState>() == 8);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_trss_pscanlen;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TidRangeScanState, trss_pscanlen) == 248
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_trss_pscanlen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TidRangeScanState>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 248;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TidRangeScanState, <<c::expression::CRecord<crate::__pgrx_c_bindings::TidRangeScanState> as c::expression::OrdinaryField<Field_trss_pscanlen>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).trss_pscanlen) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_firings;
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TriggerInstrumentation>() == 368);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TriggerInstrumentation>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TriggerInstrumentation {}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TriggerInstrumentation, firings) == 360
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_firings>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TriggerInstrumentation>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 360;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TriggerInstrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::TriggerInstrumentation> as c::expression::OrdinaryField<Field_firings>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).firings) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TupleDescData>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TupleDescData>() == 8);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_firstNonCachedOffsetAttr;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleDescData, firstNonCachedOffsetAttr)
        == 16
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_firstNonCachedOffsetAttr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TupleDescData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TupleDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::TupleDescData> as c::expression::OrdinaryField<Field_firstNonCachedOffsetAttr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).firstNonCachedOffsetAttr) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_firstNonGuaranteedAttr;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleDescData, firstNonGuaranteedAttr) == 20
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_firstNonGuaranteedAttr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TupleDescData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TupleDescData, <<c::expression::CRecord<crate::__pgrx_c_bindings::TupleDescData> as c::expression::OrdinaryField<Field_firstNonGuaranteedAttr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).firstNonGuaranteedAttr) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleDescData, compact_attrs) == 32);
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TupleTableSlot>() == 72);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TupleTableSlot>() == 8);
};
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_tts_first_nonguaranteed;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleTableSlot, tts_first_nonguaranteed)
        == 40
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tts_first_nonguaranteed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TupleTableSlot, <<c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot> as c::expression::OrdinaryField<Field_tts_first_nonguaranteed>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tts_first_nonguaranteed) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleTableSlot, tts_tableOid) == 64);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tts_tableOid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlot>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::UniquePath>() == 96);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::UniquePath>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::UniquePath, numkeys) == 88);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numkeys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::UniquePath>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::UniquePath, <<c::expression::CRecord<crate::__pgrx_c_bindings::UniquePath> as c::expression::OrdinaryField<Field_numkeys>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numkeys) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::VirtualTupleTableSlot>() == 80);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::VirtualTupleTableSlot>() == 8);
};
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::VirtualTupleTableSlot, data) == 72);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_data>
    for c::expression::CRecord<crate::__pgrx_c_bindings::VirtualTupleTableSlot>
{
    type Member = c::expression::CPointer<
        c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::WaitLSNProcInfo>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::WaitLSNProcInfo>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::WaitLSNProcInfo {}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WaitLSNProcInfo, waitLSN) == 0);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_waitLSN>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WaitLSNProcInfo>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WaitLSNProcInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::WaitLSNProcInfo> as c::expression::OrdinaryField<Field_waitLSN>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).waitLSN) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_lsnType;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WaitLSNProcInfo, lsnType) == 8);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lsnType>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WaitLSNProcInfo>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__WaitLSNType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WaitLSNProcInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::WaitLSNProcInfo> as c::expression::OrdinaryField<Field_lsnType>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lsnType) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WaitLSNProcInfo, procno) == 12);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_procno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WaitLSNProcInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WaitLSNProcInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::WaitLSNProcInfo> as c::expression::OrdinaryField<Field_procno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).procno) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_inHeap;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WaitLSNProcInfo, inHeap) == 16);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inHeap>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WaitLSNProcInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WaitLSNProcInfo, <<c::expression::CRecord<crate::__pgrx_c_bindings::WaitLSNProcInfo> as c::expression::OrdinaryField<Field_inHeap>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).inHeap) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::WaitStmt>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::WaitStmt>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::WaitStmt {}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WaitStmt, type_) == 0);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WaitStmt>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__NodeTag,
        c::CUnsignedInt,
        crate::__pgrx_c_bindings::NodeTag,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::WaitStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::WaitStmt> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WaitStmt, options) == 16);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_options>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WaitStmt>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::WaitStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::WaitStmt> as c::expression::OrdinaryField<
        Field_options,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).options) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_lsn_location;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WaitStmt, lsn_location) == 24);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lsn_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WaitStmt>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::WaitStmt,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::WaitStmt> as c::expression::OrdinaryField<
        Field_lsn_location,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lsn_location) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(
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
    assert!(
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
};
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_walrcv_identify_system>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalReceiverFunctionsType>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_char__0p__0l__struct__WalReceiverConn__0p__0c__unsigned__int__0p__0c__unsigned__long__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowFunc, ignore_nulls) == 56);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ignore_nulls>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowFunc>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WindowFunc, <<c::expression::CRecord<crate::__pgrx_c_bindings::WindowFunc> as c::expression::OrdinaryField<Field_ignore_nulls>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ignore_nulls) };
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WindowFunc, location) == 60);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WindowFunc>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 60;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::WorkerNodeInstrumentation>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::WorkerNodeInstrumentation>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::WorkerNodeInstrumentation {}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::WorkerNodeInstrumentation, num_workers) == 0
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_workers>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WorkerNodeInstrumentation>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::WorkerNodeInstrumentation, <<c::expression::CRecord<crate::__pgrx_c_bindings::WorkerNodeInstrumentation> as c::expression::OrdinaryField<Field_num_workers>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_workers) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_SharedHotStandbyActive;
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::XLogRecoveryCtlData>() == 104);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::XLogRecoveryCtlData>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::XLogRecoveryCtlData {}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogRecoveryCtlData, SharedHotStandbyActive)
        == 0
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_SharedHotStandbyActive>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecoveryCtlData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogRecoveryCtlData, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecoveryCtlData> as c::expression::OrdinaryField<Field_SharedHotStandbyActive>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).SharedHotStandbyActive) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_SharedPromoteIsTriggered;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::XLogRecoveryCtlData,
        SharedPromoteIsTriggered
    ) == 1
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_SharedPromoteIsTriggered>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecoveryCtlData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogRecoveryCtlData, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecoveryCtlData> as c::expression::OrdinaryField<Field_SharedPromoteIsTriggered>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).SharedPromoteIsTriggered) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_SharedRecoverySubtransInitialized;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::XLogRecoveryCtlData,
        SharedRecoverySubtransInitialized
    ) == 2
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_SharedRecoverySubtransInitialized>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecoveryCtlData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 2;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogRecoveryCtlData, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecoveryCtlData> as c::expression::OrdinaryField<Field_SharedRecoverySubtransInitialized>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).SharedRecoverySubtransInitialized) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_lastReplayedReadRecPtr;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogRecoveryCtlData, lastReplayedReadRecPtr)
        == 24
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lastReplayedReadRecPtr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecoveryCtlData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogRecoveryCtlData, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecoveryCtlData> as c::expression::OrdinaryField<Field_lastReplayedReadRecPtr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lastReplayedReadRecPtr) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_lastReplayedEndRecPtr;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogRecoveryCtlData, lastReplayedEndRecPtr)
        == 32
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lastReplayedEndRecPtr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecoveryCtlData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogRecoveryCtlData, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecoveryCtlData> as c::expression::OrdinaryField<Field_lastReplayedEndRecPtr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lastReplayedEndRecPtr) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_lastReplayedTLI;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogRecoveryCtlData, lastReplayedTLI) == 40
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lastReplayedTLI>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecoveryCtlData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogRecoveryCtlData, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecoveryCtlData> as c::expression::OrdinaryField<Field_lastReplayedTLI>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lastReplayedTLI) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_replayEndRecPtr;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogRecoveryCtlData, replayEndRecPtr) == 48
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_replayEndRecPtr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecoveryCtlData>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogRecoveryCtlData, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecoveryCtlData> as c::expression::OrdinaryField<Field_replayEndRecPtr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).replayEndRecPtr) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_replayEndTLI;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogRecoveryCtlData, replayEndTLI) == 56
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_replayEndTLI>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecoveryCtlData>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogRecoveryCtlData, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecoveryCtlData> as c::expression::OrdinaryField<Field_replayEndTLI>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).replayEndTLI) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_recoveryLastXTime;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogRecoveryCtlData, recoveryLastXTime) == 64
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_recoveryLastXTime>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecoveryCtlData>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogRecoveryCtlData, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecoveryCtlData> as c::expression::OrdinaryField<Field_recoveryLastXTime>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).recoveryLastXTime) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_currentChunkStartTime;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogRecoveryCtlData, currentChunkStartTime)
        == 72
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_currentChunkStartTime>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecoveryCtlData>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogRecoveryCtlData, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecoveryCtlData> as c::expression::OrdinaryField<Field_currentChunkStartTime>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).currentChunkStartTime) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_recoveryPauseState;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogRecoveryCtlData, recoveryPauseState)
        == 80
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_recoveryPauseState>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecoveryCtlData>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__RecoveryPauseState,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogRecoveryCtlData, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecoveryCtlData> as c::expression::OrdinaryField<Field_recoveryPauseState>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).recoveryPauseState) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_info_lck;
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::XLogRecoveryCtlData, info_lck) == 96);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_info_lck>
    for c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecoveryCtlData>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::XLogRecoveryCtlData, <<c::expression::CRecord<crate::__pgrx_c_bindings::XLogRecoveryCtlData> as c::expression::OrdinaryField<Field_info_lck>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).info_lck) };
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::catclist, cl_magic) == 16);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cl_magic>
    for c::expression::CRecord<crate::__pgrx_c_bindings::catclist>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::catclist, hash_value) == 20);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hash_value>
    for c::expression::CRecord<crate::__pgrx_c_bindings::catclist>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::catctup, ct_magic) == 16);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ct_magic>
    for c::expression::CRecord<crate::__pgrx_c_bindings::catctup>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::catctup, hash_value) == 20);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hash_value>
    for c::expression::CRecord<crate::__pgrx_c_bindings::catctup>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::config_bool>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::config_bool>() == 8);
};
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_bool, boot_val) == 8);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_boot_val>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_bool>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_bool, reset_val) == 40);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_reset_val>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_bool>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::config_int>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::config_int>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_int, boot_val) == 8);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_boot_val>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_int>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_int, min) == 12);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_min>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_int>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_int, max) == 16);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_max>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_int>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_int, reset_val) == 48);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_reset_val>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_int>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::core_yy_extra_type, saw_non_ascii) == 80
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_saw_non_ascii>
    for c::expression::CRecord<crate::__pgrx_c_bindings::core_yy_extra_type>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::pg_locale_struct>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::pg_locale_struct>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_locale_struct, deterministic) == 0
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_deterministic>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_locale_struct>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_locale_struct, collate_is_c) == 1);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_collate_is_c>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_locale_struct>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_locale_struct, ctype_is_c) == 2);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ctype_is_c>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_locale_struct>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 2;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_locale_struct, is_default) == 3);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_default>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_locale_struct>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 3;
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::relopt_value, __bindgen_anon_1) == 16
);
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::relopt_value,
    crate::__pgrx_c_bindings::relopt_value__bindgen_ty_1,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).__bindgen_anon_1) };
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_bool_val>
    for c::expression::CRecord<crate::__pgrx_c_bindings::relopt_value>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::relopt_value, <<c::expression::CRecord<crate::__pgrx_c_bindings::relopt_value> as c::expression::OrdinaryField<Field_bool_val>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).__bindgen_anon_1.bool_val) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_ternary_val;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::relopt_value__bindgen_ty_1, ternary_val) == 0
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ternary_val>
    for c::expression::CRecord<crate::__pgrx_c_bindings::relopt_value>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__pg_ternary,
        c::CInt,
        i32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::relopt_value, <<c::expression::CRecord<crate::__pgrx_c_bindings::relopt_value> as c::expression::OrdinaryField<Field_ternary_val>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).__bindgen_anon_1.ternary_val) };
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_int_val>
    for c::expression::CRecord<crate::__pgrx_c_bindings::relopt_value>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::relopt_value, <<c::expression::CRecord<crate::__pgrx_c_bindings::relopt_value> as c::expression::OrdinaryField<Field_int_val>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).__bindgen_anon_1.int_val) };
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_enum_val>
    for c::expression::CRecord<crate::__pgrx_c_bindings::relopt_value>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::relopt_value, <<c::expression::CRecord<crate::__pgrx_c_bindings::relopt_value> as c::expression::OrdinaryField<Field_enum_val>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).__bindgen_anon_1.enum_val) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::sequence_magic>() == 4);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::sequence_magic>() == 4);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::sequence_magic {}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::sequence_magic, magic) == 0);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_magic>
    for c::expression::CRecord<crate::__pgrx_c_bindings::sequence_magic>
{
    type Member = c::CUnsignedInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::sequence_magic, <<c::expression::CRecord<crate::__pgrx_c_bindings::sequence_magic> as c::expression::OrdinaryField<Field_magic>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).magic) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::xl_checkpoint_redo>() == 4);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::xl_checkpoint_redo>() == 4);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::xl_checkpoint_redo {}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_checkpoint_redo, wal_level) == 0);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wal_level>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_checkpoint_redo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_checkpoint_redo, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_checkpoint_redo> as c::expression::OrdinaryField<Field_wal_level>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wal_level) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::xl_heap_prune>() == 2);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::xl_heap_prune>() == 2);
};
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_heap_prune, flags) == 0);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_heap_prune>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::xl_multixact_create>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::xl_multixact_create>() == 8);
};
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_multixact_create, moff) == 8);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_moff>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_multixact_create>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg19")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_multixact_create, nmembers) == 16);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nmembers>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_multixact_create>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::xl_multixact_truncate>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::xl_multixact_truncate>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_multixact_truncate, oldestMulti) == 4
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldestMulti>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_multixact_truncate>
{
    type Member =
        c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::MultiXactId>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_multixact_truncate, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_multixact_truncate> as c::expression::OrdinaryField<Field_oldestMulti>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oldestMulti) };
#[cfg(feature = "pg19")]
#[doc(hidden)]
pub struct Field_oldestOffset;
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::xl_multixact_truncate, oldestOffset) == 8
);
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oldestOffset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::xl_multixact_truncate>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::xl_multixact_truncate, <<c::expression::CRecord<crate::__pgrx_c_bindings::xl_multixact_truncate> as c::expression::OrdinaryField<Field_oldestOffset>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oldestOffset) };
#[cfg(feature = "pg19")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ternary_val>
    for c::expression::CRecord<crate::__pgrx_c_bindings::relopt_value__bindgen_ty_1>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__pg_ternary,
        c::CInt,
        i32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg19")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::relopt_value__bindgen_ty_1, <<c::expression::CRecord<crate::__pgrx_c_bindings::relopt_value__bindgen_ty_1> as c::expression::OrdinaryField<Field_ternary_val>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ternary_val) };
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Chromosome>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Chromosome>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::pg_locale_struct__bindgen_ty_1>() == 24
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::pg_locale_struct__bindgen_ty_1>() == 8
    );
};
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::config_generic__bindgen_ty_1>() == 64);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::config_generic__bindgen_ty_1>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::config_generic__bindgen_ty_1 {}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BufferDescPadded>() == 64);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BufferDescPadded>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::relopt_ternary>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::relopt_ternary>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::relopt_ternary {}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            crate::__pgrx_c_bindings::pg_locale_struct__bindgen_ty_1__bindgen_ty_2,
        >() == 24
    );
    assert!(
        ::core::mem::align_of::<
            crate::__pgrx_c_bindings::pg_locale_struct__bindgen_ty_1__bindgen_ty_2,
        >() == 8
    );
};
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ctype_methods>() == 136);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ctype_methods>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ctype_methods {}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::config_string>() == 48);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::config_string>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::config_real>() == 64);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::config_real>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::WaitLSNState>() == 128);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::WaitLSNState>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::WaitLSNState {}
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for crate::__pgrx_c_bindings::UCaseMap {}
#[cfg(feature = "pg19")]
impl c::expression::NativeType for crate::__pgrx_c_bindings::UCaseMap {
    type Marker = c::expression::COpaque<Self>;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TidRangeScanInstrumentation>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TidRangeScanInstrumentation>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TidRangeScanInstrumentation {}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TableScanInstrumentation>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TableScanInstrumentation>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TableScanInstrumentation {}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SeqScanInstrumentation>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SeqScanInstrumentation>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SeqScanInstrumentation {}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ReplicationSlotCtlData>() == 296);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ReplicationSlotCtlData>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PgStat_PendingLock>() == 288);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PgStat_PendingLock>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PgStat_PendingLock {}
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for crate::__pgrx_c_bindings::ParallelBitmapHeapState {}
#[cfg(feature = "pg19")]
impl c::expression::NativeType for crate::__pgrx_c_bindings::ParallelBitmapHeapState {
    type Marker = c::expression::COpaque<Self>;
}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::PVWorkerUsage>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::PVWorkerUsage>() == 4);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::PVWorkerUsage {}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::MergeAppendPath>() == 104);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::MergeAppendPath>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BitmapHeapScanDescData>() == 752);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BitmapHeapScanDescData>() == 8);
};
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::BatchMVCCState>() == 7280);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::BatchMVCCState>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::BatchMVCCState {}
#[cfg(feature = "pg19")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AppendPathInput>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AppendPathInput>() == 8);
};
#[cfg(feature = "pg19")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AppendPathInput {}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(feature = "pg19")]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__GetSysCacheHashValue"]
    fn raw_inline_GetSysCacheHashValue(
        arg0: i32,
        arg1: crate::__pgrx_c_bindings::Datum,
        arg2: crate::__pgrx_c_bindings::Datum,
        arg3: crate::__pgrx_c_bindings::Datum,
        arg4: crate::__pgrx_c_bindings::Datum,
    ) -> u32;
}
#[cfg(feature = "pg19")]
#[doc = "Call the original C function `GetSysCacheHashValue`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_GetSysCacheHashValue(
    arg0: i32,
    arg1: crate::__pgrx_c_bindings::Datum,
    arg2: crate::__pgrx_c_bindings::Datum,
    arg3: crate::__pgrx_c_bindings::Datum,
    arg4: crate::__pgrx_c_bindings::Datum,
) -> u32 {
    const _: () = assert!(!::core::mem::needs_drop::<i32>());
    const _: () = assert!(!::core::mem::needs_drop::<crate::__pgrx_c_bindings::Datum>());
    const _: () = assert!(!::core::mem::needs_drop::<crate::__pgrx_c_bindings::Datum>());
    const _: () = assert!(!::core::mem::needs_drop::<crate::__pgrx_c_bindings::Datum>());
    const _: () = assert!(!::core::mem::needs_drop::<crate::__pgrx_c_bindings::Datum>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe {
        crate::ffi::pg_guard_ffi_boundary(move || {
            raw_inline_GetSysCacheHashValue(arg0, arg1, arg2, arg3, arg4)
        })
    }
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<i16>() == 2);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<i16>() == 2);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(feature = "pg19")]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__GetSysCacheOid"]
    fn raw_inline_GetSysCacheOid(
        arg0: i32,
        arg1: i16,
        arg2: crate::__pgrx_c_bindings::Datum,
        arg3: crate::__pgrx_c_bindings::Datum,
        arg4: crate::__pgrx_c_bindings::Datum,
        arg5: crate::__pgrx_c_bindings::Datum,
    ) -> crate::__pgrx_c_bindings::Oid;
}
#[cfg(feature = "pg19")]
#[doc = "Call the original C function `GetSysCacheOid`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_GetSysCacheOid(
    arg0: i32,
    arg1: i16,
    arg2: crate::__pgrx_c_bindings::Datum,
    arg3: crate::__pgrx_c_bindings::Datum,
    arg4: crate::__pgrx_c_bindings::Datum,
    arg5: crate::__pgrx_c_bindings::Datum,
) -> crate::__pgrx_c_bindings::Oid {
    const _: () = assert!(!::core::mem::needs_drop::<i32>());
    const _: () = assert!(!::core::mem::needs_drop::<i16>());
    const _: () = assert!(!::core::mem::needs_drop::<crate::__pgrx_c_bindings::Datum>());
    const _: () = assert!(!::core::mem::needs_drop::<crate::__pgrx_c_bindings::Datum>());
    const _: () = assert!(!::core::mem::needs_drop::<crate::__pgrx_c_bindings::Datum>());
    const _: () = assert!(!::core::mem::needs_drop::<crate::__pgrx_c_bindings::Datum>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe {
        crate::ffi::pg_guard_ffi_boundary(move || {
            raw_inline_GetSysCacheOid(arg0, arg1, arg2, arg3, arg4, arg5)
        })
    }
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(feature = "pg19")]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__LockBuffer"]
    fn raw_inline_LockBuffer(arg0: i32, arg1: u32) -> ();
}
#[cfg(feature = "pg19")]
#[doc = "Call the original C function `LockBuffer`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_LockBuffer(arg0: i32, arg1: u32) -> () {
    const _: () = assert!(!::core::mem::needs_drop::<i32>());
    const _: () = assert!(!::core::mem::needs_drop::<u32>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_LockBuffer(arg0, arg1)) }
}
#[cfg(feature = "pg19")]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__SET_VARTAG_EXTERNAL"]
    fn raw_inline_SET_VARTAG_EXTERNAL(arg0: *mut ::core::ffi::c_void, arg1: u32) -> ();
}
#[cfg(feature = "pg19")]
#[doc = "Call the original C function `SET_VARTAG_EXTERNAL`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_SET_VARTAG_EXTERNAL(arg0: *mut ::core::ffi::c_void, arg1: u32) -> () {
    const _: () = assert!(!::core::mem::needs_drop::<*mut ::core::ffi::c_void>());
    const _: () = assert!(!::core::mem::needs_drop::<u32>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_SET_VARTAG_EXTERNAL(arg0, arg1)) }
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::HeapTupleData>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::HeapTupleData>() == 8);
#[cfg(feature = "pg19")]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__SearchSysCacheCopy"]
    fn raw_inline_SearchSysCacheCopy(
        arg0: i32,
        arg1: crate::__pgrx_c_bindings::Datum,
        arg2: crate::__pgrx_c_bindings::Datum,
        arg3: crate::__pgrx_c_bindings::Datum,
        arg4: crate::__pgrx_c_bindings::Datum,
    ) -> *mut crate::__pgrx_c_bindings::HeapTupleData;
}
#[cfg(feature = "pg19")]
#[doc = "Call the original C function `SearchSysCacheCopy`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_SearchSysCacheCopy(
    arg0: i32,
    arg1: crate::__pgrx_c_bindings::Datum,
    arg2: crate::__pgrx_c_bindings::Datum,
    arg3: crate::__pgrx_c_bindings::Datum,
    arg4: crate::__pgrx_c_bindings::Datum,
) -> *mut crate::__pgrx_c_bindings::HeapTupleData {
    const _: () = assert!(!::core::mem::needs_drop::<i32>());
    const _: () = assert!(!::core::mem::needs_drop::<crate::__pgrx_c_bindings::Datum>());
    const _: () = assert!(!::core::mem::needs_drop::<crate::__pgrx_c_bindings::Datum>());
    const _: () = assert!(!::core::mem::needs_drop::<crate::__pgrx_c_bindings::Datum>());
    const _: () = assert!(!::core::mem::needs_drop::<crate::__pgrx_c_bindings::Datum>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe {
        crate::ffi::pg_guard_ffi_boundary(move || {
            raw_inline_SearchSysCacheCopy(arg0, arg1, arg2, arg3, arg4)
        })
    }
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(feature = "pg19")]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__SearchSysCacheExists"]
    fn raw_inline_SearchSysCacheExists(
        arg0: i32,
        arg1: crate::__pgrx_c_bindings::Datum,
        arg2: crate::__pgrx_c_bindings::Datum,
        arg3: crate::__pgrx_c_bindings::Datum,
        arg4: crate::__pgrx_c_bindings::Datum,
    ) -> bool;
}
#[cfg(feature = "pg19")]
#[doc = "Call the original C function `SearchSysCacheExists`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_SearchSysCacheExists(
    arg0: i32,
    arg1: crate::__pgrx_c_bindings::Datum,
    arg2: crate::__pgrx_c_bindings::Datum,
    arg3: crate::__pgrx_c_bindings::Datum,
    arg4: crate::__pgrx_c_bindings::Datum,
) -> bool {
    const _: () = assert!(!::core::mem::needs_drop::<i32>());
    const _: () = assert!(!::core::mem::needs_drop::<crate::__pgrx_c_bindings::Datum>());
    const _: () = assert!(!::core::mem::needs_drop::<crate::__pgrx_c_bindings::Datum>());
    const _: () = assert!(!::core::mem::needs_drop::<crate::__pgrx_c_bindings::Datum>());
    const _: () = assert!(!::core::mem::needs_drop::<crate::__pgrx_c_bindings::Datum>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe {
        crate::ffi::pg_guard_ffi_boundary(move || {
            raw_inline_SearchSysCacheExists(arg0, arg1, arg2, arg3, arg4)
        })
    }
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Datum>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::catclist>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::catclist>() == 8);
#[cfg(feature = "pg19")]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__SearchSysCacheList"]
    fn raw_inline_SearchSysCacheList(
        arg0: i32,
        arg1: i32,
        arg2: crate::__pgrx_c_bindings::Datum,
        arg3: crate::__pgrx_c_bindings::Datum,
        arg4: crate::__pgrx_c_bindings::Datum,
    ) -> *mut crate::__pgrx_c_bindings::catclist;
}
#[cfg(feature = "pg19")]
#[doc = "Call the original C function `SearchSysCacheList`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_SearchSysCacheList(
    arg0: i32,
    arg1: i32,
    arg2: crate::__pgrx_c_bindings::Datum,
    arg3: crate::__pgrx_c_bindings::Datum,
    arg4: crate::__pgrx_c_bindings::Datum,
) -> *mut crate::__pgrx_c_bindings::catclist {
    const _: () = assert!(!::core::mem::needs_drop::<i32>());
    const _: () = assert!(!::core::mem::needs_drop::<i32>());
    const _: () = assert!(!::core::mem::needs_drop::<crate::__pgrx_c_bindings::Datum>());
    const _: () = assert!(!::core::mem::needs_drop::<crate::__pgrx_c_bindings::Datum>());
    const _: () = assert!(!::core::mem::needs_drop::<crate::__pgrx_c_bindings::Datum>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe {
        crate::ffi::pg_guard_ffi_boundary(move || {
            raw_inline_SearchSysCacheList(arg0, arg1, arg2, arg3, arg4)
        })
    }
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::varatt_external>>()
        == 16
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::varatt_external>>()
        == 4
);
#[cfg(feature = "pg19")]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__VARATT_EXTERNAL_GET_COMPRESS_METHOD"]
    fn raw_inline_VARATT_EXTERNAL_GET_COMPRESS_METHOD(
        arg0: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::varatt_external>,
    ) -> u32;
}
#[cfg(feature = "pg19")]
#[doc = "Call the original C function `VARATT_EXTERNAL_GET_COMPRESS_METHOD`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_VARATT_EXTERNAL_GET_COMPRESS_METHOD(
    arg0: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::varatt_external>,
) -> u32 {
    const _: () = assert!(!::core::mem::needs_drop::<
        ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::varatt_external>,
    >());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe {
        crate::ffi::pg_guard_ffi_boundary(move || {
            raw_inline_VARATT_EXTERNAL_GET_COMPRESS_METHOD(arg0)
        })
    }
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::varatt_external>>()
        == 16
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::varatt_external>>()
        == 4
);
#[cfg(feature = "pg19")]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__VARATT_EXTERNAL_GET_EXTSIZE"]
    fn raw_inline_VARATT_EXTERNAL_GET_EXTSIZE(
        arg0: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::varatt_external>,
    ) -> usize;
}
#[cfg(feature = "pg19")]
#[doc = "Call the original C function `VARATT_EXTERNAL_GET_EXTSIZE`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_VARATT_EXTERNAL_GET_EXTSIZE(
    arg0: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::varatt_external>,
) -> usize {
    const _: () = assert!(!::core::mem::needs_drop::<
        ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::varatt_external>,
    >());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe {
        crate::ffi::pg_guard_ffi_boundary(move || raw_inline_VARATT_EXTERNAL_GET_EXTSIZE(arg0))
    }
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::varatt_external>>()
        == 16
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::varatt_external>>()
        == 4
);
#[cfg(feature = "pg19")]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__VARATT_EXTERNAL_IS_COMPRESSED"]
    fn raw_inline_VARATT_EXTERNAL_IS_COMPRESSED(
        arg0: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::varatt_external>,
    ) -> bool;
}
#[cfg(feature = "pg19")]
#[doc = "Call the original C function `VARATT_EXTERNAL_IS_COMPRESSED`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_VARATT_EXTERNAL_IS_COMPRESSED(
    arg0: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::varatt_external>,
) -> bool {
    const _: () = assert!(!::core::mem::needs_drop::<
        ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::varatt_external>,
    >());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe {
        crate::ffi::pg_guard_ffi_boundary(move || raw_inline_VARATT_EXTERNAL_IS_COMPRESSED(arg0))
    }
}
#[cfg(feature = "pg19")]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__VARTAG_EXTERNAL"]
    fn raw_inline_VARTAG_EXTERNAL(arg0: *const ::core::ffi::c_void) -> u32;
}
#[cfg(feature = "pg19")]
#[doc = "Call the original C function `VARTAG_EXTERNAL`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_VARTAG_EXTERNAL(arg0: *const ::core::ffi::c_void) -> u32 {
    const _: () = assert!(!::core::mem::needs_drop::<*const ::core::ffi::c_void>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_VARTAG_EXTERNAL(arg0)) }
}
#[cfg(feature = "pg19")]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__VARTAG_IS_EXPANDED"]
    fn raw_inline_VARTAG_IS_EXPANDED(arg0: u32) -> bool;
}
#[cfg(feature = "pg19")]
#[doc = "Call the original C function `VARTAG_IS_EXPANDED`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_VARTAG_IS_EXPANDED(arg0: u32) -> bool {
    const _: () = assert!(!::core::mem::needs_drop::<u32>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_VARTAG_IS_EXPANDED(arg0)) }
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<usize>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<usize>() == 8);
#[cfg(feature = "pg19")]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__VARTAG_SIZE"]
    fn raw_inline_VARTAG_SIZE(arg0: u32) -> usize;
}
#[cfg(feature = "pg19")]
#[doc = "Call the original C function `VARTAG_SIZE`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_VARTAG_SIZE(arg0: u32) -> usize {
    const _: () = assert!(!::core::mem::needs_drop::<u32>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_VARTAG_SIZE(arg0)) }
}
#[cfg(feature = "pg19")]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__am_sequencesync_worker"]
    fn raw_inline_am_sequencesync_worker() -> bool;
}
#[cfg(feature = "pg19")]
#[doc = "Call the original C function `am_sequencesync_worker`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_am_sequencesync_worker() -> bool {
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_am_sequencesync_worker()) }
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::Fitness>>() == 16
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::Fitness>>() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::Fitness>>() == 16
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::Fitness>>() == 8
);
#[cfg(feature = "pg19")]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__fitness_compare"]
    fn raw_inline_fitness_compare(
        arg0: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::Fitness>,
        arg1: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::Fitness>,
    ) -> i32;
}
#[cfg(feature = "pg19")]
#[doc = "Call the original C function `fitness_compare`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_fitness_compare(
    arg0: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::Fitness>,
    arg1: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::Fitness>,
) -> i32 {
    const _: () = assert!(!::core::mem::needs_drop::<
        ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::Fitness>,
    >());
    const _: () = assert!(!::core::mem::needs_drop::<
        ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::Fitness>,
    >());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_fitness_compare(arg0, arg1)) }
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::Fitness>>() == 16
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::Fitness>>() == 8
);
#[cfg(feature = "pg19")]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__fitness_is_valid"]
    fn raw_inline_fitness_is_valid(
        arg0: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::Fitness>,
    ) -> bool;
}
#[cfg(feature = "pg19")]
#[doc = "Call the original C function `fitness_is_valid`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_fitness_is_valid(
    arg0: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::Fitness>,
) -> bool {
    const _: () = assert!(!::core::mem::needs_drop::<
        ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::Fitness>,
    >());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_fitness_is_valid(arg0)) }
}
#[cfg(feature = "pg19")]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__get_logical_worker_type"]
    fn raw_inline_get_logical_worker_type() -> u32;
}
#[cfg(feature = "pg19")]
#[doc = "Call the original C function `get_logical_worker_type`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_get_logical_worker_type() -> u32 {
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_get_logical_worker_type()) }
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Oid>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Oid>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>>() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>>() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::List>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::List>() == 8);
#[cfg(feature = "pg19")]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__list_make1_impl"]
    fn raw_inline_list_make1_impl(
        arg0: u32,
        arg1: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
    ) -> *mut crate::__pgrx_c_bindings::List;
}
#[cfg(feature = "pg19")]
#[doc = "Call the original C function `list_make1_impl`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_list_make1_impl(
    arg0: u32,
    arg1: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
) -> *mut crate::__pgrx_c_bindings::List {
    const _: () = assert!(!::core::mem::needs_drop::<u32>());
    const _: () = assert!(!::core::mem::needs_drop::<
        ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
    >());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_list_make1_impl(arg0, arg1)) }
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>>() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>>() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>>() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>>() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::List>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::List>() == 8);
#[cfg(feature = "pg19")]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__list_make2_impl"]
    fn raw_inline_list_make2_impl(
        arg0: u32,
        arg1: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
        arg2: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
    ) -> *mut crate::__pgrx_c_bindings::List;
}
#[cfg(feature = "pg19")]
#[doc = "Call the original C function `list_make2_impl`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_list_make2_impl(
    arg0: u32,
    arg1: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
    arg2: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
) -> *mut crate::__pgrx_c_bindings::List {
    const _: () = assert!(!::core::mem::needs_drop::<u32>());
    const _: () = assert!(!::core::mem::needs_drop::<
        ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
    >());
    const _: () = assert!(!::core::mem::needs_drop::<
        ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
    >());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe {
        crate::ffi::pg_guard_ffi_boundary(move || raw_inline_list_make2_impl(arg0, arg1, arg2))
    }
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>>() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>>() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>>() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>>() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>>() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>>() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::List>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::List>() == 8);
#[cfg(feature = "pg19")]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__list_make3_impl"]
    fn raw_inline_list_make3_impl(
        arg0: u32,
        arg1: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
        arg2: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
        arg3: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
    ) -> *mut crate::__pgrx_c_bindings::List;
}
#[cfg(feature = "pg19")]
#[doc = "Call the original C function `list_make3_impl`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_list_make3_impl(
    arg0: u32,
    arg1: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
    arg2: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
    arg3: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
) -> *mut crate::__pgrx_c_bindings::List {
    const _: () = assert!(!::core::mem::needs_drop::<u32>());
    const _: () = assert!(!::core::mem::needs_drop::<
        ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
    >());
    const _: () = assert!(!::core::mem::needs_drop::<
        ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
    >());
    const _: () = assert!(!::core::mem::needs_drop::<
        ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
    >());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe {
        crate::ffi::pg_guard_ffi_boundary(move || {
            raw_inline_list_make3_impl(arg0, arg1, arg2, arg3)
        })
    }
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<u32>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<u32>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>>() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>>() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>>() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>>() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>>() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>>() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>>() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>>() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::List>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::List>() == 8);
#[cfg(feature = "pg19")]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__list_make4_impl"]
    fn raw_inline_list_make4_impl(
        arg0: u32,
        arg1: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
        arg2: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
        arg3: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
        arg4: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
    ) -> *mut crate::__pgrx_c_bindings::List;
}
#[cfg(feature = "pg19")]
#[doc = "Call the original C function `list_make4_impl`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_list_make4_impl(
    arg0: u32,
    arg1: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
    arg2: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
    arg3: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
    arg4: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
) -> *mut crate::__pgrx_c_bindings::List {
    const _: () = assert!(!::core::mem::needs_drop::<u32>());
    const _: () = assert!(!::core::mem::needs_drop::<
        ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
    >());
    const _: () = assert!(!::core::mem::needs_drop::<
        ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
    >());
    const _: () = assert!(!::core::mem::needs_drop::<
        ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
    >());
    const _: () = assert!(!::core::mem::needs_drop::<
        ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
    >());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe {
        crate::ffi::pg_guard_ffi_boundary(move || {
            raw_inline_list_make4_impl(arg0, arg1, arg2, arg3, arg4)
        })
    }
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<u32>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<u32>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>>() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>>() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>>() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>>() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>>() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>>() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>>() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>>() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>>() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>>() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::List>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::List>() == 8);
#[cfg(feature = "pg19")]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__list_make5_impl"]
    fn raw_inline_list_make5_impl(
        arg0: u32,
        arg1: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
        arg2: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
        arg3: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
        arg4: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
        arg5: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
    ) -> *mut crate::__pgrx_c_bindings::List;
}
#[cfg(feature = "pg19")]
#[doc = "Call the original C function `list_make5_impl`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_list_make5_impl(
    arg0: u32,
    arg1: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
    arg2: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
    arg3: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
    arg4: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
    arg5: ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
) -> *mut crate::__pgrx_c_bindings::List {
    const _: () = assert!(!::core::mem::needs_drop::<u32>());
    const _: () = assert!(!::core::mem::needs_drop::<
        ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
    >());
    const _: () = assert!(!::core::mem::needs_drop::<
        ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
    >());
    const _: () = assert!(!::core::mem::needs_drop::<
        ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
    >());
    const _: () = assert!(!::core::mem::needs_drop::<
        ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
    >());
    const _: () = assert!(!::core::mem::needs_drop::<
        ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>,
    >());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe {
        crate::ffi::pg_guard_ffi_boundary(move || {
            raw_inline_list_make5_impl(arg0, arg1, arg2, arg3, arg4, arg5)
        })
    }
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<i32>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<i32>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>>() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>>() == 8
);
#[cfg(feature = "pg19")]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__list_make_int_cell"]
    fn raw_inline_list_make_int_cell(
        arg0: i32,
    ) -> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>;
}
#[cfg(feature = "pg19")]
#[doc = "Call the original C function `list_make_int_cell`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_list_make_int_cell(
    arg0: i32,
) -> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell> {
    const _: () = assert!(!::core::mem::needs_drop::<i32>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_list_make_int_cell(arg0)) }
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Oid>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Oid>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>>() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>>() == 8
);
#[cfg(feature = "pg19")]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__list_make_oid_cell"]
    fn raw_inline_list_make_oid_cell(
        arg0: crate::__pgrx_c_bindings::Oid,
    ) -> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>;
}
#[cfg(feature = "pg19")]
#[doc = "Call the original C function `list_make_oid_cell`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_list_make_oid_cell(
    arg0: crate::__pgrx_c_bindings::Oid,
) -> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell> {
    const _: () = assert!(!::core::mem::needs_drop::<crate::__pgrx_c_bindings::Oid>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_list_make_oid_cell(arg0)) }
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>>() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>>() == 8
);
#[cfg(feature = "pg19")]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__list_make_ptr_cell"]
    fn raw_inline_list_make_ptr_cell(
        arg0: *mut ::core::ffi::c_void,
    ) -> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>;
}
#[cfg(feature = "pg19")]
#[doc = "Call the original C function `list_make_ptr_cell`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_list_make_ptr_cell(
    arg0: *mut ::core::ffi::c_void,
) -> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell> {
    const _: () = assert!(!::core::mem::needs_drop::<*mut ::core::ffi::c_void>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_list_make_ptr_cell(arg0)) }
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TransactionId>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TransactionId>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>>() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>>() == 8
);
#[cfg(feature = "pg19")]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__list_make_xid_cell"]
    fn raw_inline_list_make_xid_cell(
        arg0: crate::__pgrx_c_bindings::TransactionId,
    ) -> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell>;
}
#[cfg(feature = "pg19")]
#[doc = "Call the original C function `list_make_xid_cell`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_list_make_xid_cell(
    arg0: crate::__pgrx_c_bindings::TransactionId,
) -> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::ListCell> {
    const _: () = assert!(!::core::mem::needs_drop::<crate::__pgrx_c_bindings::TransactionId>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_list_make_xid_cell(arg0)) }
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<*mut ::core::ffi::c_void>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<*mut ::core::ffi::c_void>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<usize>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<usize>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<u32>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<u32>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<u32>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<u32>() == 4);
#[cfg(feature = "pg19")]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__pg_current_timing_clock_source"]
    fn raw_inline_pg_current_timing_clock_source() -> u32;
}
#[cfg(feature = "pg19")]
#[doc = "Call the original C function `pg_current_timing_clock_source`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_pg_current_timing_clock_source() -> u32 {
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe {
        crate::ffi::pg_guard_ffi_boundary(move || raw_inline_pg_current_timing_clock_source())
    }
}
#[cfg(feature = "pg19")]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__pg_get_ticks"]
    fn raw_inline_pg_get_ticks() -> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::instr_time>;
}
#[cfg(feature = "pg19")]
#[doc = "Call the original C function `pg_get_ticks`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_pg_get_ticks() -> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::instr_time>
{
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_pg_get_ticks()) }
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::instr_time>>() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::instr_time>>() == 8
);
#[cfg(feature = "pg19")]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__pg_get_ticks_fast"]
    fn raw_inline_pg_get_ticks_fast()
    -> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::instr_time>;
}
#[cfg(feature = "pg19")]
#[doc = "Call the original C function `pg_get_ticks_fast`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_pg_get_ticks_fast()
-> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::instr_time> {
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_pg_get_ticks_fast()) }
}
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::size_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::instr_time>>() == 8
);
#[cfg(feature = "pg19")]
const _: () = assert!(
    ::core::mem::align_of::<::core::mem::MaybeUninit<crate::__pgrx_c_bindings::instr_time>>() == 8
);
#[cfg(feature = "pg19")]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__pg_get_ticks_system"]
    fn raw_inline_pg_get_ticks_system()
    -> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::instr_time>;
}
#[cfg(feature = "pg19")]
#[doc = "Call the original C function `pg_get_ticks_system`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_pg_get_ticks_system()
-> ::core::mem::MaybeUninit<crate::__pgrx_c_bindings::instr_time> {
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_pg_get_ticks_system()) }
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<i32>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<i32>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<u32>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<u32>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<*mut ::core::ffi::c_void>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<*mut ::core::ffi::c_void>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<i32>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<i32>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<u32>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<u32>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<*const ::core::ffi::c_void>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<*const ::core::ffi::c_void>() == 8);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<i32>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<i32>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<i32>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<i32>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<i32>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<i32>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<i32>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<i32>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<i32>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<i32>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<bool>() == 1);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<bool>() == 1);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<u32>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<u32>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<u32>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<u32>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<bool>() == 1);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<bool>() == 1);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<u32>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<u32>() == 4);
#[cfg(feature = "pg19")]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__table_tuple_delete"]
    fn raw_inline_table_tuple_delete(
        arg0: *mut crate::__pgrx_c_bindings::RelationData,
        arg1: *mut crate::__pgrx_c_bindings::ItemPointerData,
        arg2: u32,
        arg3: u32,
        arg4: *mut crate::__pgrx_c_bindings::SnapshotData,
        arg5: *mut crate::__pgrx_c_bindings::SnapshotData,
        arg6: bool,
        arg7: *mut crate::__pgrx_c_bindings::TM_FailureData,
    ) -> u32;
}
#[cfg(feature = "pg19")]
#[doc = "Call the original C function `table_tuple_delete`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_table_tuple_delete(
    arg0: *mut crate::__pgrx_c_bindings::RelationData,
    arg1: *mut crate::__pgrx_c_bindings::ItemPointerData,
    arg2: u32,
    arg3: u32,
    arg4: *mut crate::__pgrx_c_bindings::SnapshotData,
    arg5: *mut crate::__pgrx_c_bindings::SnapshotData,
    arg6: bool,
    arg7: *mut crate::__pgrx_c_bindings::TM_FailureData,
) -> u32 {
    const _: () =
        assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::RelationData>());
    const _: () =
        assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::ItemPointerData>());
    const _: () = assert!(!::core::mem::needs_drop::<u32>());
    const _: () = assert!(!::core::mem::needs_drop::<u32>());
    const _: () =
        assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::SnapshotData>());
    const _: () =
        assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::SnapshotData>());
    const _: () = assert!(!::core::mem::needs_drop::<bool>());
    const _: () =
        assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::TM_FailureData>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe {
        crate::ffi::pg_guard_ffi_boundary(move || {
            raw_inline_table_tuple_delete(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7)
        })
    }
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<u32>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<u32>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<u32>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<u32>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<u32>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<u32>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<u32>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<u32>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<u32>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<u32>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<u32>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<u32>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<bool>() == 1);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<bool>() == 1);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<u32>() == 4);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<u32>() == 4);
#[cfg(feature = "pg19")]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__table_tuple_update"]
    fn raw_inline_table_tuple_update(
        arg0: *mut crate::__pgrx_c_bindings::RelationData,
        arg1: *mut crate::__pgrx_c_bindings::ItemPointerData,
        arg2: *mut crate::__pgrx_c_bindings::TupleTableSlot,
        arg3: u32,
        arg4: u32,
        arg5: *mut crate::__pgrx_c_bindings::SnapshotData,
        arg6: *mut crate::__pgrx_c_bindings::SnapshotData,
        arg7: bool,
        arg8: *mut crate::__pgrx_c_bindings::TM_FailureData,
        arg9: *mut u32,
        arg10: *mut u32,
    ) -> u32;
}
#[cfg(feature = "pg19")]
#[doc = "Call the original C function `table_tuple_update`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_table_tuple_update(
    arg0: *mut crate::__pgrx_c_bindings::RelationData,
    arg1: *mut crate::__pgrx_c_bindings::ItemPointerData,
    arg2: *mut crate::__pgrx_c_bindings::TupleTableSlot,
    arg3: u32,
    arg4: u32,
    arg5: *mut crate::__pgrx_c_bindings::SnapshotData,
    arg6: *mut crate::__pgrx_c_bindings::SnapshotData,
    arg7: bool,
    arg8: *mut crate::__pgrx_c_bindings::TM_FailureData,
    arg9: *mut u32,
    arg10: *mut u32,
) -> u32 {
    const _: () =
        assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::RelationData>());
    const _: () =
        assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::ItemPointerData>());
    const _: () =
        assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::TupleTableSlot>());
    const _: () = assert!(!::core::mem::needs_drop::<u32>());
    const _: () = assert!(!::core::mem::needs_drop::<u32>());
    const _: () =
        assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::SnapshotData>());
    const _: () =
        assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::SnapshotData>());
    const _: () = assert!(!::core::mem::needs_drop::<bool>());
    const _: () =
        assert!(!::core::mem::needs_drop::<*mut crate::__pgrx_c_bindings::TM_FailureData>());
    const _: () = assert!(!::core::mem::needs_drop::<*mut u32>());
    const _: () = assert!(!::core::mem::needs_drop::<*mut u32>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe {
        crate::ffi::pg_guard_ffi_boundary(move || {
            raw_inline_table_tuple_update(
                arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10,
            )
        })
    }
}
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<i8>() == 1);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<i8>() == 1);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::size_of::<u8>() == 1);
#[cfg(feature = "pg19")]
const _: () = assert!(::core::mem::align_of::<u8>() == 1);
#[cfg(feature = "pg19")]
unsafe extern "C-unwind" {
    #[link_name = "__pgrx_inline__fn__typalign_to_alignby"]
    fn raw_inline_typalign_to_alignby(arg0: i8) -> u8;
}
#[cfg(feature = "pg19")]
#[doc = "Call the original C function `typalign_to_alignby`.\n\n# Safety\nPointer arguments must remain live and valid for every access the original function performs, with its required alignment, initialization, bounds, and aliasing. By-value records must have every field the original function reads initialized, and preserve their C object representation. The caller must uphold the original function's resource and thread requirements. Calls must run on the backend thread. Rust callbacks must use the appropriate pgrx callback guard and cannot unwind through the native call. Invariants must already hold when PostgreSQL can raise ERROR."]
#[inline]
pub unsafe fn Inline_typalign_to_alignby(arg0: i8) -> u8 {
    const _: () = assert!(!::core::mem::needs_drop::<i8>());
    // SAFETY: The caller establishes the native function's contract, backend thread, and guarded callback obligations. Captured ABI arguments are scalars, raw pointers, or MaybeUninit aggregates; the assertions above exclude destructors. The closure only makes the native call and cannot run Rust conversion code.
    unsafe { crate::ffi::pg_guard_ffi_boundary(move || raw_inline_typalign_to_alignby(arg0)) }
}
#[cfg(feature = "pg19")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_PruneReason;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for EnumIdentity_PruneReason {}
#[cfg(feature = "pg19")]
impl c::expression::EnumIdentity for EnumIdentity_PruneReason {}
#[cfg(feature = "pg19")]
impl c::expression::enumeration::NumericEnum for EnumIdentity_PruneReason {
    type Compatible = c::CUnsignedInt;
}
#[cfg(feature = "pg19")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_TimingClockSourceType;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for EnumIdentity_TimingClockSourceType {}
#[cfg(feature = "pg19")]
impl c::expression::EnumIdentity for EnumIdentity_TimingClockSourceType {}
#[cfg(feature = "pg19")]
impl c::expression::enumeration::NumericEnum for EnumIdentity_TimingClockSourceType {
    type Compatible = c::CUnsignedInt;
}
#[cfg(feature = "pg19")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__AlterDomainType;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for EnumIdentity_enum__AlterDomainType {}
#[cfg(feature = "pg19")]
impl c::expression::EnumIdentity for EnumIdentity_enum__AlterDomainType {}
#[cfg(feature = "pg19")]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__AlterDomainType {
    type Compatible = c::CUnsignedInt;
}
#[cfg(feature = "pg19")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__BufferLockMode;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for EnumIdentity_enum__BufferLockMode {}
#[cfg(feature = "pg19")]
impl c::expression::EnumIdentity for EnumIdentity_enum__BufferLockMode {}
#[cfg(feature = "pg19")]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__BufferLockMode {
    type Compatible = c::CUnsignedInt;
}
#[cfg(feature = "pg19")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__CopyFormat;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for EnumIdentity_enum__CopyFormat {}
#[cfg(feature = "pg19")]
impl c::expression::EnumIdentity for EnumIdentity_enum__CopyFormat {}
#[cfg(feature = "pg19")]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__CopyFormat {
    type Compatible = c::CUnsignedInt;
}
#[cfg(feature = "pg19")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__FetchDirectionKeywords;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for EnumIdentity_enum__FetchDirectionKeywords {}
#[cfg(feature = "pg19")]
impl c::expression::EnumIdentity for EnumIdentity_enum__FetchDirectionKeywords {}
#[cfg(feature = "pg19")]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__FetchDirectionKeywords {
    type Compatible = c::CUnsignedInt;
}
#[cfg(feature = "pg19")]
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
            369 => crate::__pgrx_c_bindings::NodeTag::T_Agg,
            328 => crate::__pgrx_c_bindings::NodeTag::T_AggClauseInfo,
            331 => crate::__pgrx_c_bindings::NodeTag::T_AggInfo,
            309 => crate::__pgrx_c_bindings::NodeTag::T_AggPath,
            435 => crate::__pgrx_c_bindings::NodeTag::T_AggState,
            332 => crate::__pgrx_c_bindings::NodeTag::T_AggTransInfo,
            9 => crate::__pgrx_c_bindings::NodeTag::T_Aggref,
            2 => crate::__pgrx_c_bindings::NodeTag::T_Alias,
            482 => crate::__pgrx_c_bindings::NodeTag::T_AllocSetContext,
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
            263 => crate::__pgrx_c_bindings::NodeTag::T_AlterPublicationStmt,
            458 => crate::__pgrx_c_bindings::NodeTag::T_AlterReplicationSlotCmd,
            187 => crate::__pgrx_c_bindings::NodeTag::T_AlterRoleSetStmt,
            186 => crate::__pgrx_c_bindings::NodeTag::T_AlterRoleStmt,
            190 => crate::__pgrx_c_bindings::NodeTag::T_AlterSeqStmt,
            207 => crate::__pgrx_c_bindings::NodeTag::T_AlterStatsStmt,
            265 => crate::__pgrx_c_bindings::NodeTag::T_AlterSubscriptionStmt,
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
            338 => crate::__pgrx_c_bindings::NodeTag::T_Append,
            293 => crate::__pgrx_c_bindings::NodeTag::T_AppendPath,
            324 => crate::__pgrx_c_bindings::NodeTag::T_AppendRelInfo,
            403 => crate::__pgrx_c_bindings::NodeTag::T_AppendState,
            29 => crate::__pgrx_c_bindings::NodeTag::T_ArrayCoerceExpr,
            35 => crate::__pgrx_c_bindings::NodeTag::T_ArrayExpr,
            455 => crate::__pgrx_c_bindings::NodeTag::T_BaseBackupCmd,
            477 => crate::__pgrx_c_bindings::NodeTag::T_BitString,
            341 => crate::__pgrx_c_bindings::NodeTag::T_BitmapAnd,
            286 => crate::__pgrx_c_bindings::NodeTag::T_BitmapAndPath,
            406 => crate::__pgrx_c_bindings::NodeTag::T_BitmapAndState,
            285 => crate::__pgrx_c_bindings::NodeTag::T_BitmapHeapPath,
            348 => crate::__pgrx_c_bindings::NodeTag::T_BitmapHeapScan,
            414 => crate::__pgrx_c_bindings::NodeTag::T_BitmapHeapScanState,
            347 => crate::__pgrx_c_bindings::NodeTag::T_BitmapIndexScan,
            413 => crate::__pgrx_c_bindings::NodeTag::T_BitmapIndexScanState,
            342 => crate::__pgrx_c_bindings::NodeTag::T_BitmapOr,
            287 => crate::__pgrx_c_bindings::NodeTag::T_BitmapOrPath,
            407 => crate::__pgrx_c_bindings::NodeTag::T_BitmapOrState,
            451 => crate::__pgrx_c_bindings::NodeTag::T_Bitmapset,
            21 => crate::__pgrx_c_bindings::NodeTag::T_BoolExpr,
            475 => crate::__pgrx_c_bindings::NodeTag::T_Boolean,
            53 => crate::__pgrx_c_bindings::NodeTag::T_BooleanTest,
            485 => crate::__pgrx_c_bindings::NodeTag::T_BumpContext,
            114 => crate::__pgrx_c_bindings::NodeTag::T_CTECycleClause,
            113 => crate::__pgrx_c_bindings::NodeTag::T_CTESearchClause,
            214 => crate::__pgrx_c_bindings::NodeTag::T_CallContext,
            213 => crate::__pgrx_c_bindings::NodeTag::T_CallStmt,
            32 => crate::__pgrx_c_bindings::NodeTag::T_CaseExpr,
            34 => crate::__pgrx_c_bindings::NodeTag::T_CaseTestExpr,
            33 => crate::__pgrx_c_bindings::NodeTag::T_CaseWhen,
            244 => crate::__pgrx_c_bindings::NodeTag::T_CheckPointStmt,
            202 => crate::__pgrx_c_bindings::NodeTag::T_ClosePortalStmt,
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
            262 => crate::__pgrx_c_bindings::NodeTag::T_CreatePublicationStmt,
            228 => crate::__pgrx_c_bindings::NodeTag::T_CreateRangeStmt,
            456 => crate::__pgrx_c_bindings::NodeTag::T_CreateReplicationSlotCmd,
            185 => crate::__pgrx_c_bindings::NodeTag::T_CreateRoleStmt,
            145 => crate::__pgrx_c_bindings::NodeTag::T_CreateSchemaStmt,
            189 => crate::__pgrx_c_bindings::NodeTag::T_CreateSeqStmt,
            205 => crate::__pgrx_c_bindings::NodeTag::T_CreateStatsStmt,
            160 => crate::__pgrx_c_bindings::NodeTag::T_CreateStmt,
            264 => crate::__pgrx_c_bindings::NodeTag::T_CreateSubscriptionStmt,
            242 => crate::__pgrx_c_bindings::NodeTag::T_CreateTableAsStmt,
            162 => crate::__pgrx_c_bindings::NodeTag::T_CreateTableSpaceStmt,
            251 => crate::__pgrx_c_bindings::NodeTag::T_CreateTransformStmt,
            181 => crate::__pgrx_c_bindings::NodeTag::T_CreateTrigStmt,
            174 => crate::__pgrx_c_bindings::NodeTag::T_CreateUserMappingStmt,
            232 => crate::__pgrx_c_bindings::NodeTag::T_CreatedbStmt,
            355 => crate::__pgrx_c_bindings::NodeTag::T_CteScan,
            421 => crate::__pgrx_c_bindings::NodeTag::T_CteScanState,
            58 => crate::__pgrx_c_bindings::NodeTag::T_CurrentOfExpr,
            292 => crate::__pgrx_c_bindings::NodeTag::T_CustomPath,
            359 => crate::__pgrx_c_bindings::NodeTag::T_CustomScan,
            425 => crate::__pgrx_c_bindings::NodeTag::T_CustomScanState,
            254 => crate::__pgrx_c_bindings::NodeTag::T_DeallocateStmt,
            201 => crate::__pgrx_c_bindings::NodeTag::T_DeclareCursorStmt,
            93 => crate::__pgrx_c_bindings::NodeTag::T_DefElem,
            191 => crate::__pgrx_c_bindings::NodeTag::T_DefineStmt,
            138 => crate::__pgrx_c_bindings::NodeTag::T_DeleteStmt,
            245 => crate::__pgrx_c_bindings::NodeTag::T_DiscardStmt,
            18 => crate::__pgrx_c_bindings::NodeTag::T_DistinctExpr,
            211 => crate::__pgrx_c_bindings::NodeTag::T_DoStmt,
            399 => crate::__pgrx_c_bindings::NodeTag::T_DomainConstraintState,
            255 => crate::__pgrx_c_bindings::NodeTag::T_DropOwnedStmt,
            457 => crate::__pgrx_c_bindings::NodeTag::T_DropReplicationSlotCmd,
            188 => crate::__pgrx_c_bindings::NodeTag::T_DropRoleStmt,
            197 => crate::__pgrx_c_bindings::NodeTag::T_DropStmt,
            266 => crate::__pgrx_c_bindings::NodeTag::T_DropSubscriptionStmt,
            163 => crate::__pgrx_c_bindings::NodeTag::T_DropTableSpaceStmt,
            176 => crate::__pgrx_c_bindings::NodeTag::T_DropUserMappingStmt,
            236 => crate::__pgrx_c_bindings::NodeTag::T_DropdbStmt,
            395 => crate::__pgrx_c_bindings::NodeTag::T_EState,
            385 => crate::__pgrx_c_bindings::NodeTag::T_ElidedNode,
            276 => crate::__pgrx_c_bindings::NodeTag::T_EquivalenceClass,
            277 => crate::__pgrx_c_bindings::NodeTag::T_EquivalenceMember,
            453 => crate::__pgrx_c_bindings::NodeTag::T_ErrorSaveContext,
            447 => crate::__pgrx_c_bindings::NodeTag::T_EventTriggerData,
            253 => crate::__pgrx_c_bindings::NodeTag::T_ExecuteStmt,
            241 => crate::__pgrx_c_bindings::NodeTag::T_ExplainStmt,
            388 => crate::__pgrx_c_bindings::NodeTag::T_ExprContext,
            386 => crate::__pgrx_c_bindings::NodeTag::T_ExprState,
            452 => crate::__pgrx_c_bindings::NodeTag::T_ExtensibleNode,
            450 => crate::__pgrx_c_bindings::NodeTag::T_FdwRoutine,
            203 => crate::__pgrx_c_bindings::NodeTag::T_FetchStmt,
            25 => crate::__pgrx_c_bindings::NodeTag::T_FieldSelect,
            26 => crate::__pgrx_c_bindings::NodeTag::T_FieldStore,
            474 => crate::__pgrx_c_bindings::NodeTag::T_Float,
            478 => crate::__pgrx_c_bindings::NodeTag::T_ForeignKeyCacheInfo,
            273 => crate::__pgrx_c_bindings::NodeTag::T_ForeignKeyOptInfo,
            291 => crate::__pgrx_c_bindings::NodeTag::T_ForeignPath,
            358 => crate::__pgrx_c_bindings::NodeTag::T_ForeignScan,
            424 => crate::__pgrx_c_bindings::NodeTag::T_ForeignScanState,
            65 => crate::__pgrx_c_bindings::NodeTag::T_FromExpr,
            76 => crate::__pgrx_c_bindings::NodeTag::T_FuncCall,
            15 => crate::__pgrx_c_bindings::NodeTag::T_FuncExpr,
            209 => crate::__pgrx_c_bindings::NodeTag::T_FunctionParameter,
            352 => crate::__pgrx_c_bindings::NodeTag::T_FunctionScan,
            418 => crate::__pgrx_c_bindings::NodeTag::T_FunctionScanState,
            372 => crate::__pgrx_c_bindings::NodeTag::T_Gather,
            373 => crate::__pgrx_c_bindings::NodeTag::T_GatherMerge,
            299 => crate::__pgrx_c_bindings::NodeTag::T_GatherMergePath,
            439 => crate::__pgrx_c_bindings::NodeTag::T_GatherMergeState,
            298 => crate::__pgrx_c_bindings::NodeTag::T_GatherPath,
            438 => crate::__pgrx_c_bindings::NodeTag::T_GatherState,
            483 => crate::__pgrx_c_bindings::NodeTag::T_GenerationContext,
            155 => crate::__pgrx_c_bindings::NodeTag::T_GrantRoleStmt,
            152 => crate::__pgrx_c_bindings::NodeTag::T_GrantStmt,
            368 => crate::__pgrx_c_bindings::NodeTag::T_Group,
            279 => crate::__pgrx_c_bindings::NodeTag::T_GroupByOrdering,
            307 => crate::__pgrx_c_bindings::NodeTag::T_GroupPath,
            295 => crate::__pgrx_c_bindings::NodeTag::T_GroupResultPath,
            434 => crate::__pgrx_c_bindings::NodeTag::T_GroupState,
            329 => crate::__pgrx_c_bindings::NodeTag::T_GroupingExprInfo,
            10 => crate::__pgrx_c_bindings::NodeTag::T_GroupingFunc,
            107 => crate::__pgrx_c_bindings::NodeTag::T_GroupingSet,
            310 => crate::__pgrx_c_bindings::NodeTag::T_GroupingSetData,
            312 => crate::__pgrx_c_bindings::NodeTag::T_GroupingSetsPath,
            374 => crate::__pgrx_c_bindings::NodeTag::T_Hash,
            363 => crate::__pgrx_c_bindings::NodeTag::T_HashJoin,
            429 => crate::__pgrx_c_bindings::NodeTag::T_HashJoinState,
            302 => crate::__pgrx_c_bindings::NodeTag::T_HashPath,
            440 => crate::__pgrx_c_bindings::NodeTag::T_HashState,
            454 => crate::__pgrx_c_bindings::NodeTag::T_IdentifySystemCmd,
            177 => crate::__pgrx_c_bindings::NodeTag::T_ImportForeignSchemaStmt,
            367 => crate::__pgrx_c_bindings::NodeTag::T_IncrementalSort,
            306 => crate::__pgrx_c_bindings::NodeTag::T_IncrementalSortPath,
            433 => crate::__pgrx_c_bindings::NodeTag::T_IncrementalSortState,
            444 => crate::__pgrx_c_bindings::NodeTag::T_IndexAmRoutine,
            284 => crate::__pgrx_c_bindings::NodeTag::T_IndexClause,
            92 => crate::__pgrx_c_bindings::NodeTag::T_IndexElem,
            387 => crate::__pgrx_c_bindings::NodeTag::T_IndexInfo,
            346 => crate::__pgrx_c_bindings::NodeTag::T_IndexOnlyScan,
            412 => crate::__pgrx_c_bindings::NodeTag::T_IndexOnlyScanState,
            272 => crate::__pgrx_c_bindings::NodeTag::T_IndexOptInfo,
            283 => crate::__pgrx_c_bindings::NodeTag::T_IndexPath,
            345 => crate::__pgrx_c_bindings::NodeTag::T_IndexScan,
            411 => crate::__pgrx_c_bindings::NodeTag::T_IndexScanState,
            204 => crate::__pgrx_c_bindings::NodeTag::T_IndexStmt,
            111 => crate::__pgrx_c_bindings::NodeTag::T_InferClause,
            60 => crate::__pgrx_c_bindings::NodeTag::T_InferenceElem,
            212 => crate::__pgrx_c_bindings::NodeTag::T_InlineCodeBlock,
            137 => crate::__pgrx_c_bindings::NodeTag::T_InsertStmt,
            479 => crate::__pgrx_c_bindings::NodeTag::T_IntList,
            473 => crate::__pgrx_c_bindings::NodeTag::T_Integer,
            5 => crate::__pgrx_c_bindings::NodeTag::T_IntoClause,
            0 => crate::__pgrx_c_bindings::NodeTag::T_Invalid,
            275 => crate::__pgrx_c_bindings::NodeTag::T_JoinDomain,
            64 => crate::__pgrx_c_bindings::NodeTag::T_JoinExpr,
            426 => crate::__pgrx_c_bindings::NodeTag::T_JoinState,
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
            391 => crate::__pgrx_c_bindings::NodeTag::T_JunkFilter,
            377 => crate::__pgrx_c_bindings::NodeTag::T_Limit,
            319 => crate::__pgrx_c_bindings::NodeTag::T_LimitPath,
            443 => crate::__pgrx_c_bindings::NodeTag::T_LimitState,
            1 => crate::__pgrx_c_bindings::NodeTag::T_List,
            223 => crate::__pgrx_c_bindings::NodeTag::T_ListenStmt,
            231 => crate::__pgrx_c_bindings::NodeTag::T_LoadStmt,
            376 => crate::__pgrx_c_bindings::NodeTag::T_LockRows,
            317 => crate::__pgrx_c_bindings::NodeTag::T_LockRowsPath,
            442 => crate::__pgrx_c_bindings::NodeTag::T_LockRowsState,
            246 => crate::__pgrx_c_bindings::NodeTag::T_LockStmt,
            94 => crate::__pgrx_c_bindings::NodeTag::T_LockingClause,
            364 => crate::__pgrx_c_bindings::NodeTag::T_Material,
            296 => crate::__pgrx_c_bindings::NodeTag::T_MaterialPath,
            430 => crate::__pgrx_c_bindings::NodeTag::T_MaterialState,
            365 => crate::__pgrx_c_bindings::NodeTag::T_Memoize,
            297 => crate::__pgrx_c_bindings::NodeTag::T_MemoizePath,
            431 => crate::__pgrx_c_bindings::NodeTag::T_MemoizeState,
            54 => crate::__pgrx_c_bindings::NodeTag::T_MergeAction,
            393 => crate::__pgrx_c_bindings::NodeTag::T_MergeActionState,
            339 => crate::__pgrx_c_bindings::NodeTag::T_MergeAppend,
            294 => crate::__pgrx_c_bindings::NodeTag::T_MergeAppendPath,
            404 => crate::__pgrx_c_bindings::NodeTag::T_MergeAppendState,
            362 => crate::__pgrx_c_bindings::NodeTag::T_MergeJoin,
            428 => crate::__pgrx_c_bindings::NodeTag::T_MergeJoinState,
            301 => crate::__pgrx_c_bindings::NodeTag::T_MergePath,
            140 => crate::__pgrx_c_bindings::NodeTag::T_MergeStmt,
            13 => crate::__pgrx_c_bindings::NodeTag::T_MergeSupportFunc,
            116 => crate::__pgrx_c_bindings::NodeTag::T_MergeWhenClause,
            327 => crate::__pgrx_c_bindings::NodeTag::T_MinMaxAggInfo,
            313 => crate::__pgrx_c_bindings::NodeTag::T_MinMaxAggPath,
            39 => crate::__pgrx_c_bindings::NodeTag::T_MinMaxExpr,
            337 => crate::__pgrx_c_bindings::NodeTag::T_ModifyTable,
            318 => crate::__pgrx_c_bindings::NodeTag::T_ModifyTablePath,
            402 => crate::__pgrx_c_bindings::NodeTag::T_ModifyTableState,
            82 => crate::__pgrx_c_bindings::NodeTag::T_MultiAssignRef,
            16 => crate::__pgrx_c_bindings::NodeTag::T_NamedArgExpr,
            356 => crate::__pgrx_c_bindings::NodeTag::T_NamedTuplestoreScan,
            422 => crate::__pgrx_c_bindings::NodeTag::T_NamedTuplestoreScanState,
            360 => crate::__pgrx_c_bindings::NodeTag::T_NestLoop,
            361 => crate::__pgrx_c_bindings::NodeTag::T_NestLoopParam,
            427 => crate::__pgrx_c_bindings::NodeTag::T_NestLoopState,
            300 => crate::__pgrx_c_bindings::NodeTag::T_NestPath,
            59 => crate::__pgrx_c_bindings::NodeTag::T_NextValueExpr,
            222 => crate::__pgrx_c_bindings::NodeTag::T_NotifyStmt,
            19 => crate::__pgrx_c_bindings::NodeTag::T_NullIfExpr,
            52 => crate::__pgrx_c_bindings::NodeTag::T_NullTest,
            153 => crate::__pgrx_c_bindings::NodeTag::T_ObjectWithArgs,
            480 => crate::__pgrx_c_bindings::NodeTag::T_OidList,
            392 => crate::__pgrx_c_bindings::NodeTag::T_OnConflictActionState,
            112 => crate::__pgrx_c_bindings::NodeTag::T_OnConflictClause,
            66 => crate::__pgrx_c_bindings::NodeTag::T_OnConflictExpr,
            17 => crate::__pgrx_c_bindings::NodeTag::T_OpExpr,
            323 => crate::__pgrx_c_bindings::NodeTag::T_OuterJoinClauseInfo,
            144 => crate::__pgrx_c_bindings::NodeTag::T_PLAssignStmt,
            8 => crate::__pgrx_c_bindings::NodeTag::T_Param,
            281 => crate::__pgrx_c_bindings::NodeTag::T_ParamPathInfo,
            70 => crate::__pgrx_c_bindings::NodeTag::T_ParamRef,
            98 => crate::__pgrx_c_bindings::NodeTag::T_PartitionBoundSpec,
            100 => crate::__pgrx_c_bindings::NodeTag::T_PartitionCmd,
            96 => crate::__pgrx_c_bindings::NodeTag::T_PartitionElem,
            379 => crate::__pgrx_c_bindings::NodeTag::T_PartitionPruneInfo,
            382 => crate::__pgrx_c_bindings::NodeTag::T_PartitionPruneStepCombine,
            381 => crate::__pgrx_c_bindings::NodeTag::T_PartitionPruneStepOp,
            99 => crate::__pgrx_c_bindings::NodeTag::T_PartitionRangeDatum,
            97 => crate::__pgrx_c_bindings::NodeTag::T_PartitionSpec,
            380 => crate::__pgrx_c_bindings::NodeTag::T_PartitionedRelPruneInfo,
            282 => crate::__pgrx_c_bindings::NodeTag::T_Path,
            278 => crate::__pgrx_c_bindings::NodeTag::T_PathKey,
            280 => crate::__pgrx_c_bindings::NodeTag::T_PathTarget,
            326 => crate::__pgrx_c_bindings::NodeTag::T_PlaceHolderInfo,
            321 => crate::__pgrx_c_bindings::NodeTag::T_PlaceHolderVar,
            383 => crate::__pgrx_c_bindings::NodeTag::T_PlanInvalItem,
            378 => crate::__pgrx_c_bindings::NodeTag::T_PlanRowMark,
            334 => crate::__pgrx_c_bindings::NodeTag::T_PlannedStmt,
            268 => crate::__pgrx_c_bindings::NodeTag::T_PlannerGlobal,
            269 => crate::__pgrx_c_bindings::NodeTag::T_PlannerInfo,
            330 => crate::__pgrx_c_bindings::NodeTag::T_PlannerParamItem,
            252 => crate::__pgrx_c_bindings::NodeTag::T_PrepareStmt,
            336 => crate::__pgrx_c_bindings::NodeTag::T_ProjectSet,
            304 => crate::__pgrx_c_bindings::NodeTag::T_ProjectSetPath,
            401 => crate::__pgrx_c_bindings::NodeTag::T_ProjectSetState,
            390 => crate::__pgrx_c_bindings::NodeTag::T_ProjectionInfo,
            303 => crate::__pgrx_c_bindings::NodeTag::T_ProjectionPath,
            261 => crate::__pgrx_c_bindings::NodeTag::T_PublicationAllObjSpec,
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
            460 => crate::__pgrx_c_bindings::NodeTag::T_ReadReplicationSlotCmd,
            256 => crate::__pgrx_c_bindings::NodeTag::T_ReassignOwnedStmt,
            340 => crate::__pgrx_c_bindings::NodeTag::T_RecursiveUnion,
            316 => crate::__pgrx_c_bindings::NodeTag::T_RecursiveUnionPath,
            405 => crate::__pgrx_c_bindings::NodeTag::T_RecursiveUnionState,
            243 => crate::__pgrx_c_bindings::NodeTag::T_RefreshMatViewStmt,
            248 => crate::__pgrx_c_bindings::NodeTag::T_ReindexStmt,
            271 => crate::__pgrx_c_bindings::NodeTag::T_RelAggInfo,
            270 => crate::__pgrx_c_bindings::NodeTag::T_RelOptInfo,
            27 => crate::__pgrx_c_bindings::NodeTag::T_RelabelType,
            215 => crate::__pgrx_c_bindings::NodeTag::T_RenameStmt,
            240 => crate::__pgrx_c_bindings::NodeTag::T_RepackStmt,
            149 => crate::__pgrx_c_bindings::NodeTag::T_ReplicaIdentityStmt,
            81 => crate::__pgrx_c_bindings::NodeTag::T_ResTarget,
            320 => crate::__pgrx_c_bindings::NodeTag::T_RestrictInfo,
            335 => crate::__pgrx_c_bindings::NodeTag::T_Result,
            394 => crate::__pgrx_c_bindings::NodeTag::T_ResultRelInfo,
            400 => crate::__pgrx_c_bindings::NodeTag::T_ResultState,
            389 => crate::__pgrx_c_bindings::NodeTag::T_ReturnSetInfo,
            143 => crate::__pgrx_c_bindings::NodeTag::T_ReturnStmt,
            118 => crate::__pgrx_c_bindings::NodeTag::T_ReturningClause,
            61 => crate::__pgrx_c_bindings::NodeTag::T_ReturningExpr,
            117 => crate::__pgrx_c_bindings::NodeTag::T_ReturningOption,
            75 => crate::__pgrx_c_bindings::NodeTag::T_RoleSpec,
            311 => crate::__pgrx_c_bindings::NodeTag::T_RollupData,
            37 => crate::__pgrx_c_bindings::NodeTag::T_RowCompareExpr,
            36 => crate::__pgrx_c_bindings::NodeTag::T_RowExpr,
            325 => crate::__pgrx_c_bindings::NodeTag::T_RowIdentityVarInfo,
            109 => crate::__pgrx_c_bindings::NodeTag::T_RowMarkClause,
            221 => crate::__pgrx_c_bindings::NodeTag::T_RuleStmt,
            40 => crate::__pgrx_c_bindings::NodeTag::T_SQLValueFunction,
            344 => crate::__pgrx_c_bindings::NodeTag::T_SampleScan,
            410 => crate::__pgrx_c_bindings::NodeTag::T_SampleScanState,
            20 => crate::__pgrx_c_bindings::NodeTag::T_ScalarArrayOpExpr,
            408 => crate::__pgrx_c_bindings::NodeTag::T_ScanState,
            200 => crate::__pgrx_c_bindings::NodeTag::T_SecLabelStmt,
            141 => crate::__pgrx_c_bindings::NodeTag::T_SelectStmt,
            343 => crate::__pgrx_c_bindings::NodeTag::T_SeqScan,
            409 => crate::__pgrx_c_bindings::NodeTag::T_SeqScanState,
            397 => crate::__pgrx_c_bindings::NodeTag::T_SetExprState,
            375 => crate::__pgrx_c_bindings::NodeTag::T_SetOp,
            315 => crate::__pgrx_c_bindings::NodeTag::T_SetOpPath,
            441 => crate::__pgrx_c_bindings::NodeTag::T_SetOpState,
            142 => crate::__pgrx_c_bindings::NodeTag::T_SetOperationStmt,
            57 => crate::__pgrx_c_bindings::NodeTag::T_SetToDefault,
            484 => crate::__pgrx_c_bindings::NodeTag::T_SlabContext,
            366 => crate::__pgrx_c_bindings::NodeTag::T_Sort,
            83 => crate::__pgrx_c_bindings::NodeTag::T_SortBy,
            106 => crate::__pgrx_c_bindings::NodeTag::T_SortGroupClause,
            305 => crate::__pgrx_c_bindings::NodeTag::T_SortPath,
            432 => crate::__pgrx_c_bindings::NodeTag::T_SortState,
            322 => crate::__pgrx_c_bindings::NodeTag::T_SpecialJoinInfo,
            459 => crate::__pgrx_c_bindings::NodeTag::T_StartReplicationCmd,
            274 => crate::__pgrx_c_bindings::NodeTag::T_StatisticExtInfo,
            206 => crate::__pgrx_c_bindings::NodeTag::T_StatsElem,
            476 => crate::__pgrx_c_bindings::NodeTag::T_String,
            22 => crate::__pgrx_c_bindings::NodeTag::T_SubLink,
            23 => crate::__pgrx_c_bindings::NodeTag::T_SubPlan,
            384 => crate::__pgrx_c_bindings::NodeTag::T_SubPlanRTInfo,
            398 => crate::__pgrx_c_bindings::NodeTag::T_SubPlanState,
            351 => crate::__pgrx_c_bindings::NodeTag::T_SubqueryScan,
            290 => crate::__pgrx_c_bindings::NodeTag::T_SubqueryScanPath,
            417 => crate::__pgrx_c_bindings::NodeTag::T_SubqueryScanState,
            14 => crate::__pgrx_c_bindings::NodeTag::T_SubscriptingRef,
            467 => crate::__pgrx_c_bindings::NodeTag::T_SupportRequestCost,
            469 => crate::__pgrx_c_bindings::NodeTag::T_SupportRequestIndexCondition,
            465 => crate::__pgrx_c_bindings::NodeTag::T_SupportRequestInlineInFrom,
            472 => crate::__pgrx_c_bindings::NodeTag::T_SupportRequestModifyInPlace,
            471 => crate::__pgrx_c_bindings::NodeTag::T_SupportRequestOptimizeWindowClause,
            468 => crate::__pgrx_c_bindings::NodeTag::T_SupportRequestRows,
            466 => crate::__pgrx_c_bindings::NodeTag::T_SupportRequestSelectivity,
            463 => crate::__pgrx_c_bindings::NodeTag::T_SupportRequestSimplify,
            464 => crate::__pgrx_c_bindings::NodeTag::T_SupportRequestSimplifyAggref,
            470 => crate::__pgrx_c_bindings::NodeTag::T_SupportRequestWFuncMonotonic,
            486 => crate::__pgrx_c_bindings::NodeTag::T_TIDBitmap,
            445 => crate::__pgrx_c_bindings::NodeTag::T_TableAmRoutine,
            4 => crate::__pgrx_c_bindings::NodeTag::T_TableFunc,
            354 => crate::__pgrx_c_bindings::NodeTag::T_TableFuncScan,
            420 => crate::__pgrx_c_bindings::NodeTag::T_TableFuncScanState,
            91 => crate::__pgrx_c_bindings::NodeTag::T_TableLikeClause,
            104 => crate::__pgrx_c_bindings::NodeTag::T_TableSampleClause,
            62 => crate::__pgrx_c_bindings::NodeTag::T_TargetEntry,
            288 => crate::__pgrx_c_bindings::NodeTag::T_TidPath,
            289 => crate::__pgrx_c_bindings::NodeTag::T_TidRangePath,
            350 => crate::__pgrx_c_bindings::NodeTag::T_TidRangeScan,
            416 => crate::__pgrx_c_bindings::NodeTag::T_TidRangeScanState,
            349 => crate::__pgrx_c_bindings::NodeTag::T_TidScan,
            415 => crate::__pgrx_c_bindings::NodeTag::T_TidScanState,
            461 => crate::__pgrx_c_bindings::NodeTag::T_TimeLineHistoryCmd,
            225 => crate::__pgrx_c_bindings::NodeTag::T_TransactionStmt,
            448 => crate::__pgrx_c_bindings::NodeTag::T_TriggerData,
            119 => crate::__pgrx_c_bindings::NodeTag::T_TriggerTransition,
            198 => crate::__pgrx_c_bindings::NodeTag::T_TruncateStmt,
            446 => crate::__pgrx_c_bindings::NodeTag::T_TsmRoutine,
            449 => crate::__pgrx_c_bindings::NodeTag::T_TupleTableSlot,
            73 => crate::__pgrx_c_bindings::NodeTag::T_TypeCast,
            68 => crate::__pgrx_c_bindings::NodeTag::T_TypeName,
            371 => crate::__pgrx_c_bindings::NodeTag::T_Unique,
            308 => crate::__pgrx_c_bindings::NodeTag::T_UniquePath,
            333 => crate::__pgrx_c_bindings::NodeTag::T_UniqueRelInfo,
            437 => crate::__pgrx_c_bindings::NodeTag::T_UniqueState,
            224 => crate::__pgrx_c_bindings::NodeTag::T_UnlistenStmt,
            139 => crate::__pgrx_c_bindings::NodeTag::T_UpdateStmt,
            462 => crate::__pgrx_c_bindings::NodeTag::T_UploadManifestCmd,
            239 => crate::__pgrx_c_bindings::NodeTag::T_VacuumRelation,
            238 => crate::__pgrx_c_bindings::NodeTag::T_VacuumStmt,
            353 => crate::__pgrx_c_bindings::NodeTag::T_ValuesScan,
            419 => crate::__pgrx_c_bindings::NodeTag::T_ValuesScanState,
            6 => crate::__pgrx_c_bindings::NodeTag::T_Var,
            158 => crate::__pgrx_c_bindings::NodeTag::T_VariableSetStmt,
            159 => crate::__pgrx_c_bindings::NodeTag::T_VariableShowStmt,
            230 => crate::__pgrx_c_bindings::NodeTag::T_ViewStmt,
            267 => crate::__pgrx_c_bindings::NodeTag::T_WaitStmt,
            370 => crate::__pgrx_c_bindings::NodeTag::T_WindowAgg,
            314 => crate::__pgrx_c_bindings::NodeTag::T_WindowAggPath,
            436 => crate::__pgrx_c_bindings::NodeTag::T_WindowAggState,
            108 => crate::__pgrx_c_bindings::NodeTag::T_WindowClause,
            84 => crate::__pgrx_c_bindings::NodeTag::T_WindowDef,
            11 => crate::__pgrx_c_bindings::NodeTag::T_WindowFunc,
            396 => crate::__pgrx_c_bindings::NodeTag::T_WindowFuncExprState,
            12 => crate::__pgrx_c_bindings::NodeTag::T_WindowFuncRunCondition,
            487 => crate::__pgrx_c_bindings::NodeTag::T_WindowObjectData,
            105 => crate::__pgrx_c_bindings::NodeTag::T_WithCheckOption,
            110 => crate::__pgrx_c_bindings::NodeTag::T_WithClause,
            357 => crate::__pgrx_c_bindings::NodeTag::T_WorkTableScan,
            423 => crate::__pgrx_c_bindings::NodeTag::T_WorkTableScanState,
            481 => crate::__pgrx_c_bindings::NodeTag::T_XidList,
            41 => crate::__pgrx_c_bindings::NodeTag::T_XmlExpr,
            95 => crate::__pgrx_c_bindings::NodeTag::T_XmlSerialize,
            _ => panic!("C enum value has no corresponding Rust variant"),
        }
    }
}
#[cfg(feature = "pg19")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__PlannedStmtOrigin;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for EnumIdentity_enum__PlannedStmtOrigin {}
#[cfg(feature = "pg19")]
impl c::expression::EnumIdentity for EnumIdentity_enum__PlannedStmtOrigin {}
#[cfg(feature = "pg19")]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__PlannedStmtOrigin {
    type Compatible = c::CUnsignedInt;
}
#[cfg(feature = "pg19")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__PublicationAllObjType;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for EnumIdentity_enum__PublicationAllObjType {}
#[cfg(feature = "pg19")]
impl c::expression::EnumIdentity for EnumIdentity_enum__PublicationAllObjType {}
#[cfg(feature = "pg19")]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__PublicationAllObjType {
    type Compatible = c::CUnsignedInt;
}
#[cfg(feature = "pg19")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__RecoveryPauseState;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for EnumIdentity_enum__RecoveryPauseState {}
#[cfg(feature = "pg19")]
impl c::expression::EnumIdentity for EnumIdentity_enum__RecoveryPauseState {}
#[cfg(feature = "pg19")]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__RecoveryPauseState {
    type Compatible = c::CUnsignedInt;
}
#[cfg(feature = "pg19")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__RepackCommand;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for EnumIdentity_enum__RepackCommand {}
#[cfg(feature = "pg19")]
impl c::expression::EnumIdentity for EnumIdentity_enum__RepackCommand {}
#[cfg(feature = "pg19")]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__RepackCommand {
    type Compatible = c::CUnsignedInt;
}
#[cfg(feature = "pg19")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__ResultType;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for EnumIdentity_enum__ResultType {}
#[cfg(feature = "pg19")]
impl c::expression::EnumIdentity for EnumIdentity_enum__ResultType {}
#[cfg(feature = "pg19")]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__ResultType {
    type Compatible = c::CUnsignedInt;
}
#[cfg(feature = "pg19")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__SlotSyncSkipReason;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for EnumIdentity_enum__SlotSyncSkipReason {}
#[cfg(feature = "pg19")]
impl c::expression::EnumIdentity for EnumIdentity_enum__SlotSyncSkipReason {}
#[cfg(feature = "pg19")]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__SlotSyncSkipReason {
    type Compatible = c::CUnsignedInt;
}
#[cfg(feature = "pg19")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__SysCacheIdentifier;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for EnumIdentity_enum__SysCacheIdentifier {}
#[cfg(feature = "pg19")]
impl c::expression::EnumIdentity for EnumIdentity_enum__SysCacheIdentifier {}
#[cfg(feature = "pg19")]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__SysCacheIdentifier {
    type Compatible = c::CInt;
}
#[cfg(feature = "pg19")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__WaitLSNType;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for EnumIdentity_enum__WaitLSNType {}
#[cfg(feature = "pg19")]
impl c::expression::EnumIdentity for EnumIdentity_enum__WaitLSNType {}
#[cfg(feature = "pg19")]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__WaitLSNType {
    type Compatible = c::CUnsignedInt;
}
#[cfg(feature = "pg19")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__pg_ternary;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for EnumIdentity_enum__pg_ternary {}
#[cfg(feature = "pg19")]
impl c::expression::EnumIdentity for EnumIdentity_enum__pg_ternary {}
#[cfg(feature = "pg19")]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__pg_ternary {
    type Compatible = c::CInt;
}
#[cfg(feature = "pg19")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnumIdentity_enum__vartag_external;
#[cfg(feature = "pg19")]
impl c::sealed::Sealed for EnumIdentity_enum__vartag_external {}
#[cfg(feature = "pg19")]
impl c::expression::EnumIdentity for EnumIdentity_enum__vartag_external {}
#[cfg(feature = "pg19")]
impl c::expression::enumeration::NumericEnum for EnumIdentity_enum__vartag_external {
    type Compatible = c::CUnsignedInt;
}
