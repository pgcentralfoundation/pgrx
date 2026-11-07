// Shared generated C macro support.

#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_total_bytes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry> as c::expression::OrdinaryField<Field_total_bytes>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).total_bytes) };
#[doc(hidden)]
pub struct Field_stat_reset_timestamp;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry,
        stat_reset_timestamp
    ) == 128
);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_stat_reset_timestamp>
    for c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::PgStat_StatReplSlotEntry> as c::expression::OrdinaryField<Field_stat_reset_timestamp>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).stat_reset_timestamp) };
#[doc(hidden)]
pub struct Field_commandTag;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::QueryCompletion>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::QueryCompletion>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::QueryCompletion {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::QueryCompletion, commandTag) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_commandTag>
    for c::expression::CRecord<crate::__pgrx_c_bindings::QueryCompletion>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__CommandTag,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::QueryCompletion, <<c::expression::CRecord<crate::__pgrx_c_bindings::QueryCompletion> as c::expression::OrdinaryField<Field_commandTag>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).commandTag) };
#[doc(hidden)]
pub struct Field_nprocessed;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::QueryCompletion, nprocessed) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nprocessed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::QueryCompletion>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::QueryCompletion, <<c::expression::CRecord<crate::__pgrx_c_bindings::QueryCompletion> as c::expression::OrdinaryField<Field_nprocessed>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nprocessed) };
#[doc(hidden)]
pub struct Field_color;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RBTNode>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RBTNode>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::RBTNode {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RBTNode, color) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_color>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RBTNode>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::RBTNode,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::RBTNode> as c::expression::OrdinaryField<
        Field_color,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).color) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RangeVar>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RangeVar>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::RangeVar {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeVar, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeVar>
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
    crate::__pgrx_c_bindings::RangeVar,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeVar> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_inh;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeVar, inh) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_inh>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeVar>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::RangeVar,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeVar> as c::expression::OrdinaryField<
        Field_inh,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).inh) };
#[doc(hidden)]
pub struct Field_relpersistence;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeVar, relpersistence) == 33);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relpersistence>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeVar>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 33;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::RangeVar,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeVar> as c::expression::OrdinaryField<
        Field_relpersistence,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relpersistence) };
#[doc(hidden)]
pub struct Field_location;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RangeVar, location) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RangeVar>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::RangeVar,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::RangeVar> as c::expression::OrdinaryField<
        Field_location,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_spcNode;
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RelFileNode>() == 12);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RelFileNode>() == 4);
};
#[cfg(feature = "pg15")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::RelFileNode {}
#[cfg(feature = "pg15")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelFileNode, spcNode) == 0);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_spcNode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelFileNode>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelFileNode, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelFileNode> as c::expression::OrdinaryField<Field_spcNode>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).spcNode) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_dbNode;
#[cfg(feature = "pg15")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelFileNode, dbNode) == 4);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dbNode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelFileNode>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelFileNode, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelFileNode> as c::expression::OrdinaryField<Field_dbNode>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dbNode) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_relNode;
#[cfg(feature = "pg15")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RelFileNode, relNode) == 8);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relNode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RelFileNode>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::RelFileNode, <<c::expression::CRecord<crate::__pgrx_c_bindings::RelFileNode> as c::expression::OrdinaryField<Field_relNode>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relNode) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::RoleSpec>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::RoleSpec>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::RoleSpec {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RoleSpec, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RoleSpec>
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
    crate::__pgrx_c_bindings::RoleSpec,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::RoleSpec> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_roletype;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RoleSpec, roletype) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_roletype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RoleSpec>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__RoleSpecType,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::RoleSpec,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::RoleSpec> as c::expression::OrdinaryField<
        Field_roletype,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).roletype) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::RoleSpec, location) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::RoleSpec>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::RoleSpec,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::RoleSpec> as c::expression::OrdinaryField<
        Field_location,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SHM_QUEUE>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SHM_QUEUE>() == 8);
};
#[cfg(feature = "pg15")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SHM_QUEUE {}
#[cfg(feature = "pg15")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SHM_QUEUE, prev) == 0);
#[cfg(feature = "pg15")]
impl c::expression::OffsetField<Field_prev>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SHM_QUEUE>
{
    type Member = ();
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::SHM_QUEUE, prev);
}
#[doc(hidden)]
pub struct Field_next;
#[cfg(feature = "pg15")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SHM_QUEUE, next) == 8);
#[cfg(feature = "pg15")]
impl c::expression::OffsetField<Field_next>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SHM_QUEUE>
{
    type Member = ();
    const OFFSET: usize = ::core::mem::offset_of!(crate::__pgrx_c_bindings::SHM_QUEUE, next);
}
#[doc(hidden)]
pub struct Field_read_only;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SPIExecuteOptions>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SPIExecuteOptions>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SPIExecuteOptions {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SPIExecuteOptions, read_only) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_read_only>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SPIExecuteOptions>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SPIExecuteOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::SPIExecuteOptions> as c::expression::OrdinaryField<Field_read_only>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).read_only) };
#[doc(hidden)]
pub struct Field_allow_nonatomic;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SPIExecuteOptions, allow_nonatomic) == 9
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_allow_nonatomic>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SPIExecuteOptions>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 9;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SPIExecuteOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::SPIExecuteOptions> as c::expression::OrdinaryField<Field_allow_nonatomic>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).allow_nonatomic) };
#[doc(hidden)]
pub struct Field_must_return_tuples;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SPIExecuteOptions, must_return_tuples) == 10
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_must_return_tuples>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SPIExecuteOptions>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 10;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SPIExecuteOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::SPIExecuteOptions> as c::expression::OrdinaryField<Field_must_return_tuples>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).must_return_tuples) };
#[doc(hidden)]
pub struct Field_tcount;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SPIExecuteOptions, tcount) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tcount>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SPIExecuteOptions>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SPIExecuteOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::SPIExecuteOptions> as c::expression::OrdinaryField<Field_tcount>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tcount) };
#[doc(hidden)]
pub struct Field_cursorOptions;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SPIParseOpenOptions>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SPIParseOpenOptions>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SPIParseOpenOptions {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SPIParseOpenOptions, cursorOptions) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cursorOptions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SPIParseOpenOptions>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SPIParseOpenOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::SPIParseOpenOptions> as c::expression::OrdinaryField<Field_cursorOptions>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cursorOptions) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SPIParseOpenOptions, read_only) == 12
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_read_only>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SPIParseOpenOptions>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SPIParseOpenOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::SPIParseOpenOptions> as c::expression::OrdinaryField<Field_read_only>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).read_only) };
#[doc(hidden)]
pub struct Field_parseMode;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SPIPrepareOptions>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SPIPrepareOptions>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SPIPrepareOptions {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SPIPrepareOptions, parseMode) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parseMode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SPIPrepareOptions>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_RawParseMode,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SPIPrepareOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::SPIPrepareOptions> as c::expression::OrdinaryField<Field_parseMode>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).parseMode) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SPIPrepareOptions, cursorOptions) == 20
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cursorOptions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SPIPrepareOptions>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SPIPrepareOptions, <<c::expression::CRecord<crate::__pgrx_c_bindings::SPIPrepareOptions> as c::expression::OrdinaryField<Field_cursorOptions>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cursorOptions) };
#[doc(hidden)]
pub struct Field_save_XactIsoLevel;
const _: () = {
    assert!(
        ::core::mem::size_of::<crate::__pgrx_c_bindings::SavedTransactionCharacteristics>() == 8
    );
    assert!(
        ::core::mem::align_of::<crate::__pgrx_c_bindings::SavedTransactionCharacteristics>() == 4
    );
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SavedTransactionCharacteristics {}
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::SavedTransactionCharacteristics,
        save_XactIsoLevel
    ) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_save_XactIsoLevel>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SavedTransactionCharacteristics>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SavedTransactionCharacteristics, <<c::expression::CRecord<crate::__pgrx_c_bindings::SavedTransactionCharacteristics> as c::expression::OrdinaryField<Field_save_XactIsoLevel>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).save_XactIsoLevel) };
#[doc(hidden)]
pub struct Field_save_XactReadOnly;
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::SavedTransactionCharacteristics,
        save_XactReadOnly
    ) == 4
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_save_XactReadOnly>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SavedTransactionCharacteristics>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SavedTransactionCharacteristics, <<c::expression::CRecord<crate::__pgrx_c_bindings::SavedTransactionCharacteristics> as c::expression::OrdinaryField<Field_save_XactReadOnly>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).save_XactReadOnly) };
#[doc(hidden)]
pub struct Field_save_XactDeferrable;
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::SavedTransactionCharacteristics,
        save_XactDeferrable
    ) == 5
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_save_XactDeferrable>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SavedTransactionCharacteristics>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 5;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SavedTransactionCharacteristics, <<c::expression::CRecord<crate::__pgrx_c_bindings::SavedTransactionCharacteristics> as c::expression::OrdinaryField<Field_save_XactDeferrable>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).save_XactDeferrable) };
#[doc(hidden)]
pub struct Field_sk_flags;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ScanKeyData>() == 72);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ScanKeyData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ScanKeyData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ScanKeyData, sk_flags) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sk_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ScanKeyData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ScanKeyData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ScanKeyData> as c::expression::OrdinaryField<Field_sk_flags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sk_flags) };
#[doc(hidden)]
pub struct Field_sk_attno;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ScanKeyData, sk_attno) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sk_attno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ScanKeyData>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ScanKeyData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ScanKeyData> as c::expression::OrdinaryField<Field_sk_attno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sk_attno) };
#[doc(hidden)]
pub struct Field_sk_strategy;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ScanKeyData, sk_strategy) == 6);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sk_strategy>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ScanKeyData>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 6;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ScanKeyData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ScanKeyData> as c::expression::OrdinaryField<Field_sk_strategy>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sk_strategy) };
#[doc(hidden)]
pub struct Field_sk_subtype;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ScanKeyData, sk_subtype) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sk_subtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ScanKeyData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ScanKeyData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ScanKeyData> as c::expression::OrdinaryField<Field_sk_subtype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sk_subtype) };
#[doc(hidden)]
pub struct Field_sk_collation;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ScanKeyData, sk_collation) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sk_collation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ScanKeyData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ScanKeyData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ScanKeyData> as c::expression::OrdinaryField<Field_sk_collation>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sk_collation) };
#[doc(hidden)]
pub struct Field_sk_argument;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ScanKeyData, sk_argument) == 64);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_sk_argument>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ScanKeyData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, crate::__pgrx_c_bindings::Datum>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ScanKeyData, <<c::expression::CRecord<crate::__pgrx_c_bindings::ScanKeyData> as c::expression::OrdinaryField<Field_sk_argument>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).sk_argument) };
#[doc(hidden)]
pub struct Field_kw_string;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ScanKeywordList>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ScanKeywordList>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ScanKeywordList {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ScanKeywordList, kw_string) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_kw_string>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ScanKeywordList>
{
    type Member = c::expression::CPointer<
        c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
        c::expression::ReadOnly,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ScanKeywordList, <<c::expression::CRecord<crate::__pgrx_c_bindings::ScanKeywordList> as c::expression::OrdinaryField<Field_kw_string>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).kw_string) };
#[doc(hidden)]
pub struct Field_kw_offsets;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ScanKeywordList, kw_offsets) == 8);
const _: () = {
    assert!(::core::mem::size_of::<*const u16>() == 8);
    assert!(::core::mem::align_of::<*const u16>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_kw_offsets>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ScanKeywordList>
{
    type Member = c::expression::CPointer<c::CUnsignedShort, c::expression::ReadOnly>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ScanKeywordList, <<c::expression::CRecord<crate::__pgrx_c_bindings::ScanKeywordList> as c::expression::OrdinaryField<Field_kw_offsets>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).kw_offsets) };
#[doc(hidden)]
pub struct Field_num_keywords;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ScanKeywordList, num_keywords) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_keywords>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ScanKeywordList>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ScanKeywordList, <<c::expression::CRecord<crate::__pgrx_c_bindings::ScanKeywordList> as c::expression::OrdinaryField<Field_num_keywords>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_keywords) };
#[doc(hidden)]
pub struct Field_max_kw_len;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ScanKeywordList, max_kw_len) == 28);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_max_kw_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ScanKeywordList>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ScanKeywordList, <<c::expression::CRecord<crate::__pgrx_c_bindings::ScanKeywordList> as c::expression::OrdinaryField<Field_max_kw_len>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).max_kw_len) };
#[doc(hidden)]
pub struct Field_isnull;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SortItem>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SortItem>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SortItem {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SortItem, isnull) == 8);
const _: () = {
    assert!(::core::mem::size_of::<*mut bool>() == 8);
    assert!(::core::mem::align_of::<*mut bool>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isnull>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SortItem>
{
    type Member = c::expression::CPointer<c::CBool, c::expression::ReadWrite>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SortItem,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SortItem> as c::expression::OrdinaryField<
        Field_isnull,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isnull) };
#[doc(hidden)]
pub struct Field_count;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SortItem, count) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_count>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SortItem>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::SortItem,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::SortItem> as c::expression::OrdinaryField<
        Field_count,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).count) };
#[doc(hidden)]
pub struct Field_fetch_strict;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SubscriptRoutines>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SubscriptRoutines>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SubscriptRoutines {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SubscriptRoutines, fetch_strict) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fetch_strict>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubscriptRoutines>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SubscriptRoutines, <<c::expression::CRecord<crate::__pgrx_c_bindings::SubscriptRoutines> as c::expression::OrdinaryField<Field_fetch_strict>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fetch_strict) };
#[doc(hidden)]
pub struct Field_fetch_leakproof;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SubscriptRoutines, fetch_leakproof) == 17
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fetch_leakproof>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubscriptRoutines>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 17;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SubscriptRoutines, <<c::expression::CRecord<crate::__pgrx_c_bindings::SubscriptRoutines> as c::expression::OrdinaryField<Field_fetch_leakproof>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fetch_leakproof) };
#[doc(hidden)]
pub struct Field_store_leakproof;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SubscriptRoutines, store_leakproof) == 18
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_store_leakproof>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubscriptRoutines>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 18;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SubscriptRoutines, <<c::expression::CRecord<crate::__pgrx_c_bindings::SubscriptRoutines> as c::expression::OrdinaryField<Field_store_leakproof>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).store_leakproof) };
#[doc(hidden)]
pub struct Field_refcontainertype;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SubscriptingRef>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SubscriptingRef>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SubscriptingRef {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SubscriptingRef, refcontainertype) == 4
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_refcontainertype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubscriptingRef>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SubscriptingRef, <<c::expression::CRecord<crate::__pgrx_c_bindings::SubscriptingRef> as c::expression::OrdinaryField<Field_refcontainertype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).refcontainertype) };
#[doc(hidden)]
pub struct Field_refelemtype;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubscriptingRef, refelemtype) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_refelemtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubscriptingRef>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SubscriptingRef, <<c::expression::CRecord<crate::__pgrx_c_bindings::SubscriptingRef> as c::expression::OrdinaryField<Field_refelemtype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).refelemtype) };
#[doc(hidden)]
pub struct Field_refrestype;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubscriptingRef, refrestype) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_refrestype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubscriptingRef>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SubscriptingRef, <<c::expression::CRecord<crate::__pgrx_c_bindings::SubscriptingRef> as c::expression::OrdinaryField<Field_refrestype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).refrestype) };
#[doc(hidden)]
pub struct Field_reftypmod;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubscriptingRef, reftypmod) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_reftypmod>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubscriptingRef>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SubscriptingRef, <<c::expression::CRecord<crate::__pgrx_c_bindings::SubscriptingRef> as c::expression::OrdinaryField<Field_reftypmod>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).reftypmod) };
#[doc(hidden)]
pub struct Field_refcollid;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SubscriptingRef, refcollid) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_refcollid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubscriptingRef>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SubscriptingRef, <<c::expression::CRecord<crate::__pgrx_c_bindings::SubscriptingRef> as c::expression::OrdinaryField<Field_refcollid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).refcollid) };
#[doc(hidden)]
pub struct Field_refupperindexpr;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SubscriptingRef, refupperindexpr) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_refupperindexpr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubscriptingRef>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SubscriptingRef, <<c::expression::CRecord<crate::__pgrx_c_bindings::SubscriptingRef> as c::expression::OrdinaryField<Field_refupperindexpr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).refupperindexpr) };
#[doc(hidden)]
pub struct Field_reflowerindexpr;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::SubscriptingRef, reflowerindexpr) == 32
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_reflowerindexpr>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SubscriptingRef>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SubscriptingRef, <<c::expression::CRecord<crate::__pgrx_c_bindings::SubscriptingRef> as c::expression::OrdinaryField<Field_reflowerindexpr>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).reflowerindexpr) };
#[doc(hidden)]
pub struct Field_fk_table;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::SysFKRelationship>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::SysFKRelationship>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::SysFKRelationship {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SysFKRelationship, fk_table) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_fk_table>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SysFKRelationship>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SysFKRelationship, <<c::expression::CRecord<crate::__pgrx_c_bindings::SysFKRelationship> as c::expression::OrdinaryField<Field_fk_table>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).fk_table) };
#[doc(hidden)]
pub struct Field_pk_table;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SysFKRelationship, pk_table) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pk_table>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SysFKRelationship>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SysFKRelationship, <<c::expression::CRecord<crate::__pgrx_c_bindings::SysFKRelationship> as c::expression::OrdinaryField<Field_pk_table>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pk_table) };
#[doc(hidden)]
pub struct Field_is_array;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SysFKRelationship, is_array) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_array>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SysFKRelationship>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SysFKRelationship, <<c::expression::CRecord<crate::__pgrx_c_bindings::SysFKRelationship> as c::expression::OrdinaryField<Field_is_array>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_array) };
#[doc(hidden)]
pub struct Field_is_opt;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::SysFKRelationship, is_opt) == 25);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_opt>
    for c::expression::CRecord<crate::__pgrx_c_bindings::SysFKRelationship>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 25;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::SysFKRelationship, <<c::expression::CRecord<crate::__pgrx_c_bindings::SysFKRelationship> as c::expression::OrdinaryField<Field_is_opt>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_opt) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TableAmRoutine>() == 368);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TableAmRoutine>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TableAmRoutine {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TableAmRoutine, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
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
const _: Projection<crate::__pgrx_c_bindings::TableAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_scan_end;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TableAmRoutine, scan_end) == 24);
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TableScanDescData) -> (),
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TableScanDescData) -> (),
            >,
        >() == 8
    );
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_scan_end>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<
        crate::__pgrx_c_generated::Signature_void__0l__struct__TableScanDescData__0p__0r,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine> as c::expression::OrdinaryField<Field_scan_end>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).scan_end) };
#[doc(hidden)]
pub struct Field_scan_rescan;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TableAmRoutine, scan_rescan) == 32);
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::TableScanDescData,
                    *mut crate::__pgrx_c_bindings::ScanKeyData,
                    bool,
                    bool,
                    bool,
                    bool,
                ) -> (),
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::TableScanDescData,
                    *mut crate::__pgrx_c_bindings::ScanKeyData,
                    bool,
                    bool,
                    bool,
                    bool,
                ) -> (),
            >,
        >() == 8
    );
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_scan_rescan>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_void__0l__struct__TableScanDescData__0p__0c__struct__ScanKeyData__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine> as c::expression::OrdinaryField<Field_scan_rescan>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).scan_rescan) };
#[doc(hidden)]
pub struct Field_parallelscan_reinitialize;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TableAmRoutine, parallelscan_reinitialize)
        == 80
);
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut crate::__pgrx_c_bindings::ParallelTableScanDescData,
                ) -> (),
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut crate::__pgrx_c_bindings::ParallelTableScanDescData,
                ) -> (),
            >,
        >() == 8
    );
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_parallelscan_reinitialize>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_void__0l__struct__RelationData__0p__0c__struct__ParallelTableScanDescData__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine> as c::expression::OrdinaryField<Field_parallelscan_reinitialize>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).parallelscan_reinitialize) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
pub struct Field_index_fetch_begin;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TableAmRoutine, index_fetch_begin) == 88
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                )
                    -> *mut crate::__pgrx_c_bindings::IndexFetchTableData,
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                )
                    -> *mut crate::__pgrx_c_bindings::IndexFetchTableData,
            >,
        >() == 8
    );
};
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_index_fetch_begin>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_struct__IndexFetchTableData__0p__0l__struct__RelationData__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine> as c::expression::OrdinaryField<Field_index_fetch_begin>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).index_fetch_begin) };
#[doc(hidden)]
pub struct Field_index_fetch_reset;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TableAmRoutine, index_fetch_reset) == 96
);
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::IndexFetchTableData,
                ) -> (),
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::IndexFetchTableData,
                ) -> (),
            >,
        >() == 8
    );
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_index_fetch_reset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<
        crate::__pgrx_c_generated::Signature_void__0l__struct__IndexFetchTableData__0p__0r,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine> as c::expression::OrdinaryField<Field_index_fetch_reset>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).index_fetch_reset) };
#[doc(hidden)]
pub struct Field_index_fetch_end;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TableAmRoutine, index_fetch_end) == 104
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_index_fetch_end>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<
        crate::__pgrx_c_generated::Signature_void__0l__struct__IndexFetchTableData__0p__0r,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine> as c::expression::OrdinaryField<Field_index_fetch_end>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).index_fetch_end) };
#[doc(hidden)]
pub struct Field_tuple_tid_valid;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TableAmRoutine, tuple_tid_valid) == 128
);
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::TableScanDescData,
                    *mut crate::__pgrx_c_bindings::ItemPointerData,
                ) -> bool,
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::TableScanDescData,
                    *mut crate::__pgrx_c_bindings::ItemPointerData,
                ) -> bool,
            >,
        >() == 8
    );
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tuple_tid_valid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__struct__ItemPointerData__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 128;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine> as c::expression::OrdinaryField<Field_tuple_tid_valid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tuple_tid_valid) };
#[doc(hidden)]
pub struct Field_tuple_satisfies_snapshot;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TableAmRoutine, tuple_satisfies_snapshot)
        == 144
);
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut crate::__pgrx_c_bindings::TupleTableSlot,
                    *mut crate::__pgrx_c_bindings::SnapshotData,
                ) -> bool,
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut crate::__pgrx_c_bindings::TupleTableSlot,
                    *mut crate::__pgrx_c_bindings::SnapshotData,
                ) -> bool,
            >,
        >() == 8
    );
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tuple_satisfies_snapshot>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__RelationData__0p__0c__struct__TupleTableSlot__0p__0c__struct__SnapshotData__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine> as c::expression::OrdinaryField<Field_tuple_satisfies_snapshot>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tuple_satisfies_snapshot) };
#[doc(hidden)]
pub struct Field_index_delete_tuples;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TableAmRoutine, index_delete_tuples) == 152
);
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut crate::__pgrx_c_bindings::TM_IndexDeleteOp,
                )
                    -> crate::__pgrx_c_bindings::TransactionId,
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut crate::__pgrx_c_bindings::TM_IndexDeleteOp,
                )
                    -> crate::__pgrx_c_bindings::TransactionId,
            >,
        >() == 8
    );
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_index_delete_tuples>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_unsigned__int__0l__struct__RelationData__0p__0c__struct__TM_IndexDeleteOp__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine> as c::expression::OrdinaryField<Field_index_delete_tuples>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).index_delete_tuples) };
#[doc(hidden)]
pub struct Field_tuple_insert;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TableAmRoutine, tuple_insert) == 160);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut crate::__pgrx_c_bindings::TupleTableSlot,
                    u32,
                    i32,
                    *mut crate::__pgrx_c_bindings::BulkInsertStateData,
                ) -> (),
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut crate::__pgrx_c_bindings::TupleTableSlot,
                    u32,
                    i32,
                    *mut crate::__pgrx_c_bindings::BulkInsertStateData,
                ) -> (),
            >,
        >() == 8
    );
};
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tuple_insert>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_void__0l__struct__RelationData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__int__0c__struct__BulkInsertStateData__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 160;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine> as c::expression::OrdinaryField<Field_tuple_insert>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tuple_insert) };
#[doc(hidden)]
pub struct Field_tuple_insert_speculative;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TableAmRoutine, tuple_insert_speculative)
        == 168
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut crate::__pgrx_c_bindings::TupleTableSlot,
                    u32,
                    i32,
                    *mut crate::__pgrx_c_bindings::BulkInsertStateData,
                    u32,
                ) -> (),
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut crate::__pgrx_c_bindings::TupleTableSlot,
                    u32,
                    i32,
                    *mut crate::__pgrx_c_bindings::BulkInsertStateData,
                    u32,
                ) -> (),
            >,
        >() == 8
    );
};
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tuple_insert_speculative>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_void__0l__struct__RelationData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__int__0c__struct__BulkInsertStateData__0p__0c__unsigned__int__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 168;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine> as c::expression::OrdinaryField<Field_tuple_insert_speculative>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tuple_insert_speculative) };
#[doc(hidden)]
pub struct Field_tuple_complete_speculative;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TableAmRoutine, tuple_complete_speculative)
        == 176
);
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut crate::__pgrx_c_bindings::TupleTableSlot,
                    u32,
                    bool,
                ) -> (),
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut crate::__pgrx_c_bindings::TupleTableSlot,
                    u32,
                    bool,
                ) -> (),
            >,
        >() == 8
    );
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tuple_complete_speculative>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_void__0l__struct__RelationData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__0x5f426f6f6c__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 176;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine> as c::expression::OrdinaryField<Field_tuple_complete_speculative>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tuple_complete_speculative) };
#[doc(hidden)]
pub struct Field_multi_insert;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TableAmRoutine, multi_insert) == 184);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut *mut crate::__pgrx_c_bindings::TupleTableSlot,
                    i32,
                    u32,
                    i32,
                    *mut crate::__pgrx_c_bindings::BulkInsertStateData,
                ) -> (),
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut *mut crate::__pgrx_c_bindings::TupleTableSlot,
                    i32,
                    u32,
                    i32,
                    *mut crate::__pgrx_c_bindings::BulkInsertStateData,
                ) -> (),
            >,
        >() == 8
    );
};
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_multi_insert>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_void__0l__struct__RelationData__0p__0c__struct__TupleTableSlot__0p__0p__0c__int__0c__unsigned__int__0c__int__0c__struct__BulkInsertStateData__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 184;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine> as c::expression::OrdinaryField<Field_multi_insert>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).multi_insert) };
#[doc(hidden)]
pub struct Field_tuple_delete;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TableAmRoutine, tuple_delete) == 192);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut crate::__pgrx_c_bindings::ItemPointerData,
                    u32,
                    *mut crate::__pgrx_c_bindings::SnapshotData,
                    *mut crate::__pgrx_c_bindings::SnapshotData,
                    bool,
                    *mut crate::__pgrx_c_bindings::TM_FailureData,
                    bool,
                ) -> u32,
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut crate::__pgrx_c_bindings::ItemPointerData,
                    u32,
                    *mut crate::__pgrx_c_bindings::SnapshotData,
                    *mut crate::__pgrx_c_bindings::SnapshotData,
                    bool,
                    *mut crate::__pgrx_c_bindings::TM_FailureData,
                    bool,
                ) -> u32,
            >,
        >() == 8
    );
};
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tuple_delete>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_enum__TM_Result__0l__struct__RelationData__0p__0c__struct__ItemPointerData__0p__0c__unsigned__int__0c__struct__SnapshotData__0p__0c__struct__SnapshotData__0p__0c__0x5f426f6f6c__0c__struct__TM_FailureData__0p__0c__0x5f426f6f6c__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 192;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine> as c::expression::OrdinaryField<Field_tuple_delete>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tuple_delete) };
#[doc(hidden)]
pub struct Field_tuple_update;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TableAmRoutine, tuple_update) == 200);
#[cfg(feature = "pg15")]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut crate::__pgrx_c_bindings::ItemPointerData,
                    *mut crate::__pgrx_c_bindings::TupleTableSlot,
                    u32,
                    *mut crate::__pgrx_c_bindings::SnapshotData,
                    *mut crate::__pgrx_c_bindings::SnapshotData,
                    bool,
                    *mut crate::__pgrx_c_bindings::TM_FailureData,
                    *mut u32,
                    *mut bool,
                ) -> u32,
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut crate::__pgrx_c_bindings::ItemPointerData,
                    *mut crate::__pgrx_c_bindings::TupleTableSlot,
                    u32,
                    *mut crate::__pgrx_c_bindings::SnapshotData,
                    *mut crate::__pgrx_c_bindings::SnapshotData,
                    bool,
                    *mut crate::__pgrx_c_bindings::TM_FailureData,
                    *mut u32,
                    *mut bool,
                ) -> u32,
            >,
        >() == 8
    );
};
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tuple_update>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_enum__TM_Result__0l__struct__RelationData__0p__0c__struct__ItemPointerData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__struct__SnapshotData__0p__0c__struct__SnapshotData__0p__0c__0x5f426f6f6c__0c__struct__TM_FailureData__0p__0c__enum__LockTupleMode__0p__0c__0x5f426f6f6c__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 200;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine> as c::expression::OrdinaryField<Field_tuple_update>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tuple_update) };
#[doc(hidden)]
pub struct Field_tuple_lock;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TableAmRoutine, tuple_lock) == 208);
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut crate::__pgrx_c_bindings::ItemPointerData,
                    *mut crate::__pgrx_c_bindings::SnapshotData,
                    *mut crate::__pgrx_c_bindings::TupleTableSlot,
                    u32,
                    u32,
                    u32,
                    u8,
                    *mut crate::__pgrx_c_bindings::TM_FailureData,
                ) -> u32,
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut crate::__pgrx_c_bindings::ItemPointerData,
                    *mut crate::__pgrx_c_bindings::SnapshotData,
                    *mut crate::__pgrx_c_bindings::TupleTableSlot,
                    u32,
                    u32,
                    u32,
                    u8,
                    *mut crate::__pgrx_c_bindings::TM_FailureData,
                ) -> u32,
            >,
        >() == 8
    );
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tuple_lock>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_enum__TM_Result__0l__struct__RelationData__0p__0c__struct__ItemPointerData__0p__0c__struct__SnapshotData__0p__0c__struct__TupleTableSlot__0p__0c__unsigned__int__0c__enum__LockTupleMode__0c__enum__LockWaitPolicy__0c__unsigned__char__0c__struct__TM_FailureData__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 208;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine> as c::expression::OrdinaryField<Field_tuple_lock>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tuple_lock) };
#[doc(hidden)]
pub struct Field_finish_bulk_insert;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TableAmRoutine, finish_bulk_insert) == 216
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, i32) -> (),
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData, i32) -> (),
            >,
        >() == 8
    );
};
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_finish_bulk_insert>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<
        crate::__pgrx_c_generated::Signature_void__0l__struct__RelationData__0p__0c__int__0r,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 216;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine> as c::expression::OrdinaryField<Field_finish_bulk_insert>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).finish_bulk_insert) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_relation_set_new_filenode;
#[cfg(feature = "pg15")]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TableAmRoutine, relation_set_new_filenode)
        == 224
);
#[cfg(feature = "pg15")]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *const crate::__pgrx_c_bindings::RelFileNode,
                    ::core::ffi::c_char,
                    *mut crate::__pgrx_c_bindings::TransactionId,
                    *mut crate::__pgrx_c_bindings::MultiXactId,
                ) -> (),
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *const crate::__pgrx_c_bindings::RelFileNode,
                    ::core::ffi::c_char,
                    *mut crate::__pgrx_c_bindings::TransactionId,
                    *mut crate::__pgrx_c_bindings::MultiXactId,
                ) -> (),
            >,
        >() == 8
    );
};
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relation_set_new_filenode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_void__0l__struct__RelationData__0p__0c__const__struct__RelFileNode__0p__0c__char__0c__unsigned__int__0p__0c__unsigned__int__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 224;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine> as c::expression::OrdinaryField<Field_relation_set_new_filenode>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relation_set_new_filenode) };
#[doc(hidden)]
pub struct Field_relation_nontransactional_truncate;
const _: () = assert!(
    ::core::mem::offset_of!(
        crate::__pgrx_c_bindings::TableAmRoutine,
        relation_nontransactional_truncate
    ) == 232
);
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData) -> (),
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData) -> (),
            >,
        >() == 8
    );
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relation_nontransactional_truncate>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<
        crate::__pgrx_c_generated::Signature_void__0l__struct__RelationData__0p__0r,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 232;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine> as c::expression::OrdinaryField<Field_relation_nontransactional_truncate>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relation_nontransactional_truncate) };
#[doc(hidden)]
pub struct Field_relation_copy_data;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TableAmRoutine, relation_copy_data) == 240
);
#[cfg(feature = "pg15")]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *const crate::__pgrx_c_bindings::RelFileNode,
                ) -> (),
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *const crate::__pgrx_c_bindings::RelFileNode,
                ) -> (),
            >,
        >() == 8
    );
};
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relation_copy_data>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_void__0l__struct__RelationData__0p__0c__const__struct__RelFileNode__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 240;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine> as c::expression::OrdinaryField<Field_relation_copy_data>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relation_copy_data) };
#[doc(hidden)]
pub struct Field_relation_copy_for_cluster;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TableAmRoutine, relation_copy_for_cluster)
        == 248
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut crate::__pgrx_c_bindings::RelationData,
                    bool,
                    crate::__pgrx_c_bindings::TransactionId,
                    *mut crate::__pgrx_c_bindings::TransactionId,
                    *mut crate::__pgrx_c_bindings::MultiXactId,
                    *mut f64,
                    *mut f64,
                    *mut f64,
                ) -> (),
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut crate::__pgrx_c_bindings::RelationData,
                    bool,
                    crate::__pgrx_c_bindings::TransactionId,
                    *mut crate::__pgrx_c_bindings::TransactionId,
                    *mut crate::__pgrx_c_bindings::MultiXactId,
                    *mut f64,
                    *mut f64,
                    *mut f64,
                ) -> (),
            >,
        >() == 8
    );
};
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relation_copy_for_cluster>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_void__0l__struct__RelationData__0p__0c__struct__RelationData__0p__0c__struct__RelationData__0p__0c__0x5f426f6f6c__0c__unsigned__int__0c__unsigned__int__0p__0c__unsigned__int__0p__0c__double__0p__0c__double__0p__0c__double__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 248;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine> as c::expression::OrdinaryField<Field_relation_copy_for_cluster>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relation_copy_for_cluster) };
#[doc(hidden)]
pub struct Field_relation_vacuum;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TableAmRoutine, relation_vacuum) == 256
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut crate::__pgrx_c_bindings::VacuumParams,
                    *mut crate::__pgrx_c_bindings::BufferAccessStrategyData,
                ) -> (),
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut crate::__pgrx_c_bindings::VacuumParams,
                    *mut crate::__pgrx_c_bindings::BufferAccessStrategyData,
                ) -> (),
            >,
        >() == 8
    );
};
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relation_vacuum>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_void__0l__struct__RelationData__0p__0c__struct__VacuumParams__0p__0c__struct__BufferAccessStrategyData__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 256;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine> as c::expression::OrdinaryField<Field_relation_vacuum>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relation_vacuum) };
#[doc(hidden)]
pub struct Field_scan_analyze_next_block;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TableAmRoutine, scan_analyze_next_block)
        == 264
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::TableScanDescData,
                    u32,
                    *mut crate::__pgrx_c_bindings::BufferAccessStrategyData,
                ) -> bool,
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::TableScanDescData,
                    u32,
                    *mut crate::__pgrx_c_bindings::BufferAccessStrategyData,
                ) -> bool,
            >,
        >() == 8
    );
};
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_scan_analyze_next_block>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__unsigned__int__0c__struct__BufferAccessStrategyData__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 264;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine> as c::expression::OrdinaryField<Field_scan_analyze_next_block>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).scan_analyze_next_block) };
#[doc(hidden)]
pub struct Field_scan_analyze_next_tuple;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TableAmRoutine, scan_analyze_next_tuple)
        == 272
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::TableScanDescData,
                    crate::__pgrx_c_bindings::TransactionId,
                    *mut f64,
                    *mut f64,
                    *mut crate::__pgrx_c_bindings::TupleTableSlot,
                ) -> bool,
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::TableScanDescData,
                    crate::__pgrx_c_bindings::TransactionId,
                    *mut f64,
                    *mut f64,
                    *mut crate::__pgrx_c_bindings::TupleTableSlot,
                ) -> bool,
            >,
        >() == 8
    );
};
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_scan_analyze_next_tuple>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__TableScanDescData__0p__0c__unsigned__int__0c__double__0p__0c__double__0p__0c__struct__TupleTableSlot__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 272;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine> as c::expression::OrdinaryField<Field_scan_analyze_next_tuple>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).scan_analyze_next_tuple) };
#[doc(hidden)]
pub struct Field_index_build_range_scan;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TableAmRoutine, index_build_range_scan)
        == 280
);
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut crate::__pgrx_c_bindings::IndexInfo,
                    bool,
                    bool,
                    bool,
                    u32,
                    u32,
                    ::core::option::Option<
                        unsafe extern "C-unwind" fn(
                            *mut crate::__pgrx_c_bindings::RelationData,
                            *mut crate::__pgrx_c_bindings::ItemPointerData,
                            *mut crate::__pgrx_c_bindings::Datum,
                            *mut bool,
                            bool,
                            *mut ::core::ffi::c_void,
                        ) -> (),
                    >,
                    *mut ::core::ffi::c_void,
                    *mut crate::__pgrx_c_bindings::TableScanDescData,
                ) -> f64,
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut crate::__pgrx_c_bindings::IndexInfo,
                    bool,
                    bool,
                    bool,
                    u32,
                    u32,
                    ::core::option::Option<
                        unsafe extern "C-unwind" fn(
                            *mut crate::__pgrx_c_bindings::RelationData,
                            *mut crate::__pgrx_c_bindings::ItemPointerData,
                            *mut crate::__pgrx_c_bindings::Datum,
                            *mut bool,
                            bool,
                            *mut ::core::ffi::c_void,
                        ) -> (),
                    >,
                    *mut ::core::ffi::c_void,
                    *mut crate::__pgrx_c_bindings::TableScanDescData,
                ) -> f64,
            >,
        >() == 8
    );
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_index_build_range_scan>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_double__0l__struct__RelationData__0p__0c__struct__RelationData__0p__0c__struct__IndexInfo__0p__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__0x5f426f6f6c__0c__unsigned__int__0c__unsigned__int__0c__void__0l__0p__0r__0l__struct__RelationData__0p__0c__struct__ItemPointerData__0p__0c__unsigned__long__0p__0c__0x5f426f6f6c__0p__0c__0x5f426f6f6c__0c__void__0p__0r__0c__void__0p__0c__struct__TableScanDescData__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 280;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine> as c::expression::OrdinaryField<Field_index_build_range_scan>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).index_build_range_scan) };
#[doc(hidden)]
pub struct Field_index_validate_scan;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TableAmRoutine, index_validate_scan) == 288
);
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut crate::__pgrx_c_bindings::IndexInfo,
                    *mut crate::__pgrx_c_bindings::SnapshotData,
                    *mut crate::__pgrx_c_bindings::ValidateIndexState,
                ) -> (),
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut crate::__pgrx_c_bindings::IndexInfo,
                    *mut crate::__pgrx_c_bindings::SnapshotData,
                    *mut crate::__pgrx_c_bindings::ValidateIndexState,
                ) -> (),
            >,
        >() == 8
    );
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_index_validate_scan>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_void__0l__struct__RelationData__0p__0c__struct__RelationData__0p__0c__struct__IndexInfo__0p__0c__struct__SnapshotData__0p__0c__struct__ValidateIndexState__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 288;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine> as c::expression::OrdinaryField<Field_index_validate_scan>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).index_validate_scan) };
#[doc(hidden)]
pub struct Field_relation_size;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TableAmRoutine, relation_size) == 296
);
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    i32,
                ) -> u64,
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    i32,
                ) -> u64,
            >,
        >() == 8
    );
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relation_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_unsigned__long__0l__struct__RelationData__0p__0c__enum__ForkNumber__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 296;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine> as c::expression::OrdinaryField<Field_relation_size>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relation_size) };
#[doc(hidden)]
pub struct Field_relation_needs_toast_table;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TableAmRoutine, relation_needs_toast_table)
        == 304
);
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData) -> bool,
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::RelationData) -> bool,
            >,
        >() == 8
    );
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relation_needs_toast_table>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<
        crate::__pgrx_c_generated::Signature_0x5f426f6f6c__0l__struct__RelationData__0p__0r,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 304;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine> as c::expression::OrdinaryField<Field_relation_needs_toast_table>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relation_needs_toast_table) };
#[doc(hidden)]
pub struct Field_relation_toast_am;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TableAmRoutine, relation_toast_am) == 312
);
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                ) -> crate::__pgrx_c_bindings::Oid,
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                ) -> crate::__pgrx_c_bindings::Oid,
            >,
        >() == 8
    );
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relation_toast_am>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<
        crate::__pgrx_c_generated::Signature_unsigned__int__0l__struct__RelationData__0p__0r,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 312;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine> as c::expression::OrdinaryField<Field_relation_toast_am>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relation_toast_am) };
#[doc(hidden)]
pub struct Field_relation_fetch_toast_slice;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TableAmRoutine, relation_fetch_toast_slice)
        == 320
);
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    crate::__pgrx_c_bindings::Oid,
                    i32,
                    i32,
                    i32,
                    *mut crate::__pgrx_c_bindings::varlena,
                ) -> (),
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    crate::__pgrx_c_bindings::Oid,
                    i32,
                    i32,
                    i32,
                    *mut crate::__pgrx_c_bindings::varlena,
                ) -> (),
            >,
        >() == 8
    );
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relation_fetch_toast_slice>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_void__0l__struct__RelationData__0p__0c__unsigned__int__0c__int__0c__int__0c__int__0c__struct__varlena__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 320;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine> as c::expression::OrdinaryField<Field_relation_fetch_toast_slice>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relation_fetch_toast_slice) };
#[doc(hidden)]
pub struct Field_relation_estimate_size;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TableAmRoutine, relation_estimate_size)
        == 328
);
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut i32,
                    *mut u32,
                    *mut f64,
                    *mut f64,
                ) -> (),
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::RelationData,
                    *mut i32,
                    *mut u32,
                    *mut f64,
                    *mut f64,
                ) -> (),
            >,
        >() == 8
    );
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_relation_estimate_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_void__0l__struct__RelationData__0p__0c__int__0p__0c__unsigned__int__0p__0c__double__0p__0c__double__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 328;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TableAmRoutine, <<c::expression::CRecord<crate::__pgrx_c_bindings::TableAmRoutine> as c::expression::OrdinaryField<Field_relation_estimate_size>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).relation_estimate_size) };
#[doc(hidden)]
pub struct Field_base_slot_size;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TupleTableSlotOps>() == 96);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TupleTableSlotOps>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TupleTableSlotOps {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleTableSlotOps, base_slot_size) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_base_slot_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlotOps>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TupleTableSlotOps, <<c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlotOps> as c::expression::OrdinaryField<Field_base_slot_size>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).base_slot_size) };
#[doc(hidden)]
pub struct Field_clear;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleTableSlotOps, clear) == 24);
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TupleTableSlot) -> (),
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(*mut crate::__pgrx_c_bindings::TupleTableSlot) -> (),
            >,
        >() == 8
    );
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_clear>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlotOps>
{
    type Member = c::expression::CFunction<
        crate::__pgrx_c_generated::Signature_void__0l__struct__TupleTableSlot__0p__0r,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TupleTableSlotOps, <<c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlotOps> as c::expression::OrdinaryField<Field_clear>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).clear) };
#[doc(hidden)]
pub struct Field_materialize;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleTableSlotOps, materialize) == 48
);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_materialize>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlotOps>
{
    type Member = c::expression::CFunction<
        crate::__pgrx_c_generated::Signature_void__0l__struct__TupleTableSlot__0p__0r,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TupleTableSlotOps, <<c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlotOps> as c::expression::OrdinaryField<Field_materialize>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).materialize) };
#[doc(hidden)]
pub struct Field_copy_minimal_tuple;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::TupleTableSlotOps, copy_minimal_tuple) == 88
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(
        ::core::mem::size_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::TupleTableSlot,
                )
                    -> *mut crate::__pgrx_c_bindings::MinimalTupleData,
            >,
        >() == 8
    );
    assert!(
        ::core::mem::align_of::<
            ::core::option::Option<
                unsafe extern "C-unwind" fn(
                    *mut crate::__pgrx_c_bindings::TupleTableSlot,
                )
                    -> *mut crate::__pgrx_c_bindings::MinimalTupleData,
            >,
        >() == 8
    );
};
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_copy_minimal_tuple>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlotOps>
{
    type Member = c::expression::CFunction<crate::__pgrx_c_generated::Signature_struct__MinimalTupleData__0p__0l__struct__TupleTableSlot__0p__0r>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 88;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::TupleTableSlotOps, <<c::expression::CRecord<crate::__pgrx_c_bindings::TupleTableSlotOps> as c::expression::OrdinaryField<Field_copy_minimal_tuple>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).copy_minimal_tuple) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::TypeName>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::TypeName>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::TypeName {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeName, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeName>
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
    crate::__pgrx_c_bindings::TypeName,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::TypeName> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_names;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeName, names) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_names>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeName>
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
    crate::__pgrx_c_bindings::TypeName,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::TypeName> as c::expression::OrdinaryField<
        Field_names,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).names) };
#[doc(hidden)]
pub struct Field_typeOid;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeName, typeOid) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typeOid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeName>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::TypeName,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::TypeName> as c::expression::OrdinaryField<
        Field_typeOid,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typeOid) };
#[doc(hidden)]
pub struct Field_setof;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeName, setof) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_setof>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeName>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::TypeName,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::TypeName> as c::expression::OrdinaryField<
        Field_setof,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).setof) };
#[doc(hidden)]
pub struct Field_pct_type;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeName, pct_type) == 21);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_pct_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeName>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 21;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::TypeName,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::TypeName> as c::expression::OrdinaryField<
        Field_pct_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).pct_type) };
#[doc(hidden)]
pub struct Field_typmods;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeName, typmods) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typmods>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeName>
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
    crate::__pgrx_c_bindings::TypeName,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::TypeName> as c::expression::OrdinaryField<
        Field_typmods,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typmods) };
#[doc(hidden)]
pub struct Field_typemod;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeName, typemod) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_typemod>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeName>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::TypeName,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::TypeName> as c::expression::OrdinaryField<
        Field_typemod,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).typemod) };
#[doc(hidden)]
pub struct Field_arrayBounds;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeName, arrayBounds) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_arrayBounds>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeName>
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
    crate::__pgrx_c_bindings::TypeName,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::TypeName> as c::expression::OrdinaryField<
        Field_arrayBounds,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).arrayBounds) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::TypeName, location) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::TypeName>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::TypeName,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::TypeName> as c::expression::OrdinaryField<
        Field_location,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[doc(hidden)]
pub struct Field_wal_records;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::WalUsage>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::WalUsage>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::WalUsage {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WalUsage, wal_records) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wal_records>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalUsage>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::WalUsage,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::WalUsage> as c::expression::OrdinaryField<
        Field_wal_records,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wal_records) };
#[doc(hidden)]
pub struct Field_wal_fpi;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WalUsage, wal_fpi) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wal_fpi>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalUsage>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::WalUsage,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::WalUsage> as c::expression::OrdinaryField<
        Field_wal_fpi,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wal_fpi) };
#[doc(hidden)]
pub struct Field_wal_bytes;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::WalUsage, wal_bytes) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_wal_bytes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::WalUsage>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::WalUsage,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::WalUsage> as c::expression::OrdinaryField<
        Field_wal_bytes,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).wal_bytes) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::config_enum_entry>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::config_enum_entry>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::config_enum_entry {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_enum_entry, val) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_val>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_enum_entry>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::config_enum_entry, <<c::expression::CRecord<crate::__pgrx_c_bindings::config_enum_entry> as c::expression::OrdinaryField<Field_val>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).val) };
#[doc(hidden)]
pub struct Field_hidden;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::config_enum_entry, hidden) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hidden>
    for c::expression::CRecord<crate::__pgrx_c_bindings::config_enum_entry>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::config_enum_entry, <<c::expression::CRecord<crate::__pgrx_c_bindings::config_enum_entry> as c::expression::OrdinaryField<Field_hidden>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hidden) };
#[doc(hidden)]
pub struct Field_d_ino;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::dirent>() == 280);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::dirent>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::dirent {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::dirent, d_ino) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_d_ino>
    for c::expression::CRecord<crate::__pgrx_c_bindings::dirent>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::dirent,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::dirent> as c::expression::OrdinaryField<
        Field_d_ino,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).d_ino) };
#[doc(hidden)]
pub struct Field_d_off;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::dirent, d_off) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_d_off>
    for c::expression::CRecord<crate::__pgrx_c_bindings::dirent>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::dirent,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::dirent> as c::expression::OrdinaryField<
        Field_d_off,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).d_off) };
#[doc(hidden)]
pub struct Field_d_reclen;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::dirent, d_reclen) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_d_reclen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::dirent>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::dirent,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::dirent> as c::expression::OrdinaryField<
        Field_d_reclen,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).d_reclen) };
#[doc(hidden)]
pub struct Field_d_type;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::dirent, d_type) == 18);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_d_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::dirent>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 18;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::dirent,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::dirent> as c::expression::OrdinaryField<
        Field_d_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).d_type) };
#[doc(hidden)]
pub struct Field_key_size;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::dshash_parameters>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::dshash_parameters>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::dshash_parameters {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::dshash_parameters, key_size) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_key_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::dshash_parameters>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::dshash_parameters, <<c::expression::CRecord<crate::__pgrx_c_bindings::dshash_parameters> as c::expression::OrdinaryField<Field_key_size>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).key_size) };
#[doc(hidden)]
pub struct Field_entry_size;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::dshash_parameters, entry_size) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_entry_size>
    for c::expression::CRecord<crate::__pgrx_c_bindings::dshash_parameters>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::dshash_parameters, <<c::expression::CRecord<crate::__pgrx_c_bindings::dshash_parameters> as c::expression::OrdinaryField<Field_entry_size>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).entry_size) };
#[doc(hidden)]
pub struct Field_tranche_id;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::dshash_parameters, tranche_id) == 32);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tranche_id>
    for c::expression::CRecord<crate::__pgrx_c_bindings::dshash_parameters>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::dshash_parameters, <<c::expression::CRecord<crate::__pgrx_c_bindings::dshash_parameters> as c::expression::OrdinaryField<Field_tranche_id>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tranche_id) };
#[cfg(any(feature = "pg15", feature = "pg17", feature = "pg18", feature = "pg19"))]
#[doc(hidden)]
pub struct Field_iov_len;
#[cfg(any(feature = "pg15", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::iovec>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::iovec>() == 8);
};
#[cfg(any(feature = "pg15", feature = "pg17", feature = "pg18", feature = "pg19"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::iovec {}
#[cfg(any(feature = "pg15", feature = "pg17", feature = "pg18", feature = "pg19"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::iovec, iov_len) == 8);
#[cfg(any(feature = "pg15", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_iov_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::iovec>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(any(feature = "pg15", feature = "pg17", feature = "pg18", feature = "pg19"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::iovec,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::iovec> as c::expression::OrdinaryField<
        Field_iov_len,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).iov_len) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::nameData>() == 64);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::nameData>() == 1);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::nameData {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::nameData, data) == 0);
const _: () = {
    assert!(::core::mem::size_of::<[::core::ffi::c_char; 64]>() == 64);
    assert!(::core::mem::align_of::<[::core::ffi::c_char; 64]>() == 1);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_data>
    for c::expression::CRecord<crate::__pgrx_c_bindings::nameData>
{
    type Member =
        c::expression::CArray<c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>, 64>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::nameData,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::nameData> as c::expression::OrdinaryField<
        Field_data,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).data) };
#[doc(hidden)]
pub struct Field_first_child;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::pairingheap_node>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::pairingheap_node>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::pairingheap_node {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pairingheap_node, first_child) == 0);
const _: () = {
    assert!(::core::mem::size_of::<*mut crate::__pgrx_c_bindings::pairingheap_node>() == 8);
    assert!(::core::mem::align_of::<*mut crate::__pgrx_c_bindings::pairingheap_node>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_first_child>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pairingheap_node>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::pairingheap_node>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::pairingheap_node, <<c::expression::CRecord<crate::__pgrx_c_bindings::pairingheap_node> as c::expression::OrdinaryField<Field_first_child>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).first_child) };
#[doc(hidden)]
pub struct Field_encoding;
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::pg_enc2gettext>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::pg_enc2gettext>() == 8);
};
#[cfg(any(feature = "pg15", feature = "pg16"))]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::pg_enc2gettext {}
#[cfg(any(feature = "pg15", feature = "pg16"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_enc2gettext, encoding) == 0);
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_encoding>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_enc2gettext>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__pg_enc,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(any(feature = "pg15", feature = "pg16"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::pg_enc2gettext, <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_enc2gettext> as c::expression::OrdinaryField<Field_encoding>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).encoding) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::pg_enc2name>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::pg_enc2name>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::pg_enc2name {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_enc2name, encoding) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_encoding>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_enc2name>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__pg_enc,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::pg_enc2name, <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_enc2name> as c::expression::OrdinaryField<Field_encoding>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).encoding) };
#[doc(hidden)]
pub struct Field_tm_sec;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::pg_tm>() == 56);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::pg_tm>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::pg_tm {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_tm, tm_sec) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tm_sec>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_tm>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::pg_tm,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_tm> as c::expression::OrdinaryField<
        Field_tm_sec,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tm_sec) };
#[doc(hidden)]
pub struct Field_tm_min;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_tm, tm_min) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tm_min>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_tm>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::pg_tm,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_tm> as c::expression::OrdinaryField<
        Field_tm_min,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tm_min) };
#[doc(hidden)]
pub struct Field_tm_hour;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_tm, tm_hour) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tm_hour>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_tm>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::pg_tm,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_tm> as c::expression::OrdinaryField<
        Field_tm_hour,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tm_hour) };
#[doc(hidden)]
pub struct Field_tm_mday;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_tm, tm_mday) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tm_mday>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_tm>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::pg_tm,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_tm> as c::expression::OrdinaryField<
        Field_tm_mday,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tm_mday) };
#[doc(hidden)]
pub struct Field_tm_mon;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_tm, tm_mon) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tm_mon>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_tm>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::pg_tm,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_tm> as c::expression::OrdinaryField<
        Field_tm_mon,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tm_mon) };
#[doc(hidden)]
pub struct Field_tm_year;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_tm, tm_year) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tm_year>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_tm>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::pg_tm,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_tm> as c::expression::OrdinaryField<
        Field_tm_year,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tm_year) };
#[doc(hidden)]
pub struct Field_tm_wday;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_tm, tm_wday) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tm_wday>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_tm>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::pg_tm,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_tm> as c::expression::OrdinaryField<
        Field_tm_wday,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tm_wday) };
#[doc(hidden)]
pub struct Field_tm_yday;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_tm, tm_yday) == 28);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tm_yday>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_tm>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::pg_tm,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_tm> as c::expression::OrdinaryField<
        Field_tm_yday,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tm_yday) };
#[doc(hidden)]
pub struct Field_tm_isdst;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_tm, tm_isdst) == 32);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tm_isdst>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_tm>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::pg_tm,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_tm> as c::expression::OrdinaryField<
        Field_tm_isdst,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tm_isdst) };
#[doc(hidden)]
pub struct Field_tm_gmtoff;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::pg_tm, tm_gmtoff) == 40);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tm_gmtoff>
    for c::expression::CRecord<crate::__pgrx_c_bindings::pg_tm>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 40;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::pg_tm,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::pg_tm> as c::expression::OrdinaryField<
        Field_tm_gmtoff,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tm_gmtoff) };
#[doc(hidden)]
pub struct Field_ss_family;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::sockaddr_storage>() == 128);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::sockaddr_storage>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::sockaddr_storage {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::sockaddr_storage, ss_family) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ss_family>
    for c::expression::CRecord<crate::__pgrx_c_bindings::sockaddr_storage>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::sockaddr_storage, <<c::expression::CRecord<crate::__pgrx_c_bindings::sockaddr_storage> as c::expression::OrdinaryField<Field_ss_family>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ss_family) };
#[doc(hidden)]
pub struct Field___ss_align;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::sockaddr_storage, __ss_align) == 120);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field___ss_align>
    for c::expression::CRecord<crate::__pgrx_c_bindings::sockaddr_storage>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 120;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::sockaddr_storage, <<c::expression::CRecord<crate::__pgrx_c_bindings::sockaddr_storage> as c::expression::OrdinaryField<Field___ss_align>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).__ss_align) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_tv_sec;
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::timespec>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::timespec>() == 8);
};
#[cfg(feature = "pg15")]
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::timespec {}
#[cfg(feature = "pg15")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::timespec, tv_sec) == 0);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tv_sec>
    for c::expression::CRecord<crate::__pgrx_c_bindings::timespec>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::timespec,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::timespec> as c::expression::OrdinaryField<
        Field_tv_sec,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tv_sec) };
#[cfg(feature = "pg15")]
#[doc(hidden)]
pub struct Field_tv_nsec;
#[cfg(feature = "pg15")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::timespec, tv_nsec) == 8);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_tv_nsec>
    for c::expression::CRecord<crate::__pgrx_c_bindings::timespec>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
#[cfg(feature = "pg15")]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::timespec,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::timespec> as c::expression::OrdinaryField<
        Field_tv_nsec,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).tv_nsec) };
#[doc(hidden)]
pub struct Field_ptr_value;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ListCell>() == 8);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ListCell>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ListCell {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ListCell, ptr_value) == 0);
const _: () = {
    assert!(::core::mem::size_of::<*mut ::core::ffi::c_void>() == 8);
    assert!(::core::mem::align_of::<*mut ::core::ffi::c_void>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ptr_value>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ListCell>
{
    type Member = c::expression::CPointer<c::expression::CVoid, c::expression::ReadWrite>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ListCell,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ListCell> as c::expression::OrdinaryField<
        Field_ptr_value,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ptr_value) };
#[doc(hidden)]
pub struct Field_int_value;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ListCell, int_value) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_int_value>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ListCell>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ListCell,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ListCell> as c::expression::OrdinaryField<
        Field_int_value,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).int_value) };
#[doc(hidden)]
pub struct Field_oid_value;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ListCell, oid_value) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_oid_value>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ListCell>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::ListCell,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::ListCell> as c::expression::OrdinaryField<
        Field_oid_value,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).oid_value) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::datetkn>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::datetkn>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::datetkn {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::datetkn, type_) == 11);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::datetkn>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 11;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::datetkn,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::datetkn> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::datetkn, value) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_value>
    for c::expression::CRecord<crate::__pgrx_c_bindings::datetkn>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::datetkn,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::datetkn> as c::expression::OrdinaryField<
        Field_value,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).value) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ginxlogInsert>() == 2);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ginxlogInsert>() == 2);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ginxlogInsert {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ginxlogInsert, flags) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_flags>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ginxlogInsert>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ginxlogInsert, <<c::expression::CRecord<crate::__pgrx_c_bindings::ginxlogInsert> as c::expression::OrdinaryField<Field_flags>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).flags) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ginxlogInsertDataInternal>() == 12);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ginxlogInsertDataInternal>() == 2);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ginxlogInsertDataInternal {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ginxlogInsertDataInternal, offset) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_offset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ginxlogInsertDataInternal>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ginxlogInsertDataInternal, <<c::expression::CRecord<crate::__pgrx_c_bindings::ginxlogInsertDataInternal> as c::expression::OrdinaryField<Field_offset>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).offset) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ginxlogInsertEntry>() == 12);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ginxlogInsertEntry>() == 2);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ginxlogInsertEntry {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ginxlogInsertEntry, offset) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_offset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ginxlogInsertEntry>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ginxlogInsertEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::ginxlogInsertEntry> as c::expression::OrdinaryField<Field_offset>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).offset) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ginxlogInsertEntry, isDelete) == 2);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isDelete>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ginxlogInsertEntry>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 2;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ginxlogInsertEntry, <<c::expression::CRecord<crate::__pgrx_c_bindings::ginxlogInsertEntry> as c::expression::OrdinaryField<Field_isDelete>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isDelete) };
#[doc(hidden)]
pub struct Field_nactions;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ginxlogRecompressDataLeaf>() == 2);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ginxlogRecompressDataLeaf>() == 2);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ginxlogRecompressDataLeaf {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::ginxlogRecompressDataLeaf, nactions) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_nactions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ginxlogRecompressDataLeaf>
{
    type Member = c::CUnsignedShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ginxlogRecompressDataLeaf, <<c::expression::CRecord<crate::__pgrx_c_bindings::ginxlogRecompressDataLeaf> as c::expression::OrdinaryField<Field_nactions>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).nactions) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::ginxlogSegmentAction>() == 2);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::ginxlogSegmentAction>() == 1);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::ginxlogSegmentAction {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ginxlogSegmentAction, segno) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_segno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ginxlogSegmentAction>
{
    type Member = c::CUnsignedChar;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ginxlogSegmentAction, <<c::expression::CRecord<crate::__pgrx_c_bindings::ginxlogSegmentAction> as c::expression::OrdinaryField<Field_segno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).segno) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::ginxlogSegmentAction, type_) == 1);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::ginxlogSegmentAction>
{
    type Member = c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 1;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::ginxlogSegmentAction, <<c::expression::CRecord<crate::__pgrx_c_bindings::ginxlogSegmentAction> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::int2vector>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::int2vector>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::int2vector {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::int2vector, vl_len_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_vl_len_>
    for c::expression::CRecord<crate::__pgrx_c_bindings::int2vector>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::int2vector, <<c::expression::CRecord<crate::__pgrx_c_bindings::int2vector> as c::expression::OrdinaryField<Field_vl_len_>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).vl_len_) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::int2vector, ndim) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_ndim>
    for c::expression::CRecord<crate::__pgrx_c_bindings::int2vector>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::int2vector, <<c::expression::CRecord<crate::__pgrx_c_bindings::int2vector> as c::expression::OrdinaryField<Field_ndim>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).ndim) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::int2vector, dataoffset) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dataoffset>
    for c::expression::CRecord<crate::__pgrx_c_bindings::int2vector>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::int2vector, <<c::expression::CRecord<crate::__pgrx_c_bindings::int2vector> as c::expression::OrdinaryField<Field_dataoffset>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dataoffset) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::int2vector, elemtype) == 12);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_elemtype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::int2vector>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::int2vector, <<c::expression::CRecord<crate::__pgrx_c_bindings::int2vector> as c::expression::OrdinaryField<Field_elemtype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).elemtype) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::int2vector, dim1) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_dim1>
    for c::expression::CRecord<crate::__pgrx_c_bindings::int2vector>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::int2vector, <<c::expression::CRecord<crate::__pgrx_c_bindings::int2vector> as c::expression::OrdinaryField<Field_dim1>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).dim1) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::int2vector, lbound1) == 20);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_lbound1>
    for c::expression::CRecord<crate::__pgrx_c_bindings::int2vector>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::int2vector, <<c::expression::CRecord<crate::__pgrx_c_bindings::int2vector> as c::expression::OrdinaryField<Field_lbound1>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).lbound1) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::shm_mq_iovec>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::shm_mq_iovec>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::shm_mq_iovec {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::shm_mq_iovec, data) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_data>
    for c::expression::CRecord<crate::__pgrx_c_bindings::shm_mq_iovec>
{
    type Member = c::expression::CPointer<
        c::expression::CIntegerStorage<c::CChar, ::core::ffi::c_char>,
        c::expression::ReadOnly,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::shm_mq_iovec, <<c::expression::CRecord<crate::__pgrx_c_bindings::shm_mq_iovec> as c::expression::OrdinaryField<Field_data>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).data) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::shm_mq_iovec, len) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_len>
    for c::expression::CRecord<crate::__pgrx_c_bindings::shm_mq_iovec>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::shm_mq_iovec, <<c::expression::CRecord<crate::__pgrx_c_bindings::shm_mq_iovec> as c::expression::OrdinaryField<Field_len>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).len) };
#[doc(hidden)]
pub struct Field_space_for_chunks;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::shm_toc_estimator>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::shm_toc_estimator>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::shm_toc_estimator {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::shm_toc_estimator, space_for_chunks) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_space_for_chunks>
    for c::expression::CRecord<crate::__pgrx_c_bindings::shm_toc_estimator>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::shm_toc_estimator, <<c::expression::CRecord<crate::__pgrx_c_bindings::shm_toc_estimator> as c::expression::OrdinaryField<Field_space_for_chunks>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).space_for_chunks) };
#[doc(hidden)]
pub struct Field_number_of_keys;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::shm_toc_estimator, number_of_keys) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_number_of_keys>
    for c::expression::CRecord<crate::__pgrx_c_bindings::shm_toc_estimator>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::shm_toc_estimator, <<c::expression::CRecord<crate::__pgrx_c_bindings::shm_toc_estimator> as c::expression::OrdinaryField<Field_number_of_keys>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).number_of_keys) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::A_ArrayExpr>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::A_ArrayExpr>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::A_ArrayExpr {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::A_ArrayExpr, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::A_ArrayExpr>
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
const _: Projection<crate::__pgrx_c_bindings::A_ArrayExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::A_ArrayExpr> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::A_ArrayExpr, elements) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_elements>
    for c::expression::CRecord<crate::__pgrx_c_bindings::A_ArrayExpr>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::A_ArrayExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::A_ArrayExpr> as c::expression::OrdinaryField<Field_elements>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).elements) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::A_ArrayExpr, location) == 16);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::A_ArrayExpr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::A_ArrayExpr, <<c::expression::CRecord<crate::__pgrx_c_bindings::A_ArrayExpr> as c::expression::OrdinaryField<Field_location>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::A_Const>() == 32);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::A_Const>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::A_Const {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::A_Const, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::A_Const>
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
    crate::__pgrx_c_bindings::A_Const,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::A_Const> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::A_Const, isnull) == 24);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_isnull>
    for c::expression::CRecord<crate::__pgrx_c_bindings::A_Const>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::A_Const,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::A_Const> as c::expression::OrdinaryField<
        Field_isnull,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).isnull) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::A_Const, location) == 28);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::A_Const>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::A_Const,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::A_Const> as c::expression::OrdinaryField<
        Field_location,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::A_Expr>() == 40);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::A_Expr>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::A_Expr {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::A_Expr, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::A_Expr>
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
    crate::__pgrx_c_bindings::A_Expr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::A_Expr> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_kind;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::A_Expr, kind) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_kind>
    for c::expression::CRecord<crate::__pgrx_c_bindings::A_Expr>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__A_Expr_Kind,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::A_Expr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::A_Expr> as c::expression::OrdinaryField<
        Field_kind,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).kind) };
#[doc(hidden)]
pub struct Field_name;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::A_Expr, name) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_name>
    for c::expression::CRecord<crate::__pgrx_c_bindings::A_Expr>
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
    crate::__pgrx_c_bindings::A_Expr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::A_Expr> as c::expression::OrdinaryField<
        Field_name,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).name) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::A_Expr, location) == 32);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_location>
    for c::expression::CRecord<crate::__pgrx_c_bindings::A_Expr>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::A_Expr,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::A_Expr> as c::expression::OrdinaryField<
        Field_location,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).location) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::A_Indices>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::A_Indices>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::A_Indices {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::A_Indices, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::A_Indices>
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
    crate::__pgrx_c_bindings::A_Indices,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::A_Indices> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_is_slice;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::A_Indices, is_slice) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_is_slice>
    for c::expression::CRecord<crate::__pgrx_c_bindings::A_Indices>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::A_Indices,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::A_Indices> as c::expression::OrdinaryField<
        Field_is_slice,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).is_slice) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::A_Indirection>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::A_Indirection>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::A_Indirection {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::A_Indirection, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::A_Indirection>
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
const _: Projection<crate::__pgrx_c_bindings::A_Indirection, <<c::expression::CRecord<crate::__pgrx_c_bindings::A_Indirection> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_indirection;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::A_Indirection, indirection) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_indirection>
    for c::expression::CRecord<crate::__pgrx_c_bindings::A_Indirection>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::A_Indirection, <<c::expression::CRecord<crate::__pgrx_c_bindings::A_Indirection> as c::expression::OrdinaryField<Field_indirection>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).indirection) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::A_Star>() == 4);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::A_Star>() == 4);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::A_Star {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::A_Star, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::A_Star>
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
    crate::__pgrx_c_bindings::A_Star,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::A_Star> as c::expression::OrdinaryField<
        Field_type,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AccessPriv>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AccessPriv>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AccessPriv {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AccessPriv, type_) == 0);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_type>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AccessPriv>
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
const _: Projection<crate::__pgrx_c_bindings::AccessPriv, <<c::expression::CRecord<crate::__pgrx_c_bindings::AccessPriv> as c::expression::OrdinaryField<Field_type>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).type_) };
#[doc(hidden)]
pub struct Field_cols;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AccessPriv, cols) == 16);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_cols>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AccessPriv>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AccessPriv, <<c::expression::CRecord<crate::__pgrx_c_bindings::AccessPriv> as c::expression::OrdinaryField<Field_cols>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).cols) };
#[doc(hidden)]
pub struct Field_aggstrategy;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::Agg>() == 184);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::Agg>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::Agg {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Agg, aggstrategy) == 104);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aggstrategy>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Agg>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__AggStrategy,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 104;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Agg,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Agg> as c::expression::OrdinaryField<
        Field_aggstrategy,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).aggstrategy) };
#[doc(hidden)]
pub struct Field_aggsplit;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Agg, aggsplit) == 108);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aggsplit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Agg>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__AggSplit,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 108;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Agg,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Agg> as c::expression::OrdinaryField<
        Field_aggsplit,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).aggsplit) };
#[doc(hidden)]
pub struct Field_numCols;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Agg, numCols) == 112);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numCols>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Agg>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 112;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Agg,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Agg> as c::expression::OrdinaryField<
        Field_numCols,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numCols) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
#[doc(hidden)]
pub struct Field_numGroups;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Agg, numGroups) == 144);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numGroups>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Agg>
{
    type Member = c::CLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 144;
}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17", feature = "pg18"))]
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Agg,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Agg> as c::expression::OrdinaryField<
        Field_numGroups,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numGroups) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Agg, transitionSpace) == 152);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_transitionSpace>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Agg>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 152;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::Agg,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Agg> as c::expression::OrdinaryField<
        Field_transitionSpace,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).transitionSpace) };
#[doc(hidden)]
pub struct Field_groupingSets;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Agg, groupingSets) == 168);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_groupingSets>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Agg>
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
    crate::__pgrx_c_bindings::Agg,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Agg> as c::expression::OrdinaryField<
        Field_groupingSets,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).groupingSets) };
#[doc(hidden)]
pub struct Field_chain;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::Agg, chain) == 176);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_chain>
    for c::expression::CRecord<crate::__pgrx_c_bindings::Agg>
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
    crate::__pgrx_c_bindings::Agg,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::Agg> as c::expression::OrdinaryField<
        Field_chain,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).chain) };
#[doc(hidden)]
pub struct Field_transno;
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AggInfo>() == 24);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AggInfo>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AggInfo {}
#[cfg(feature = "pg15")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggInfo, transno) == 8);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_transno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggInfo>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::AggInfo,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::AggInfo> as c::expression::OrdinaryField<
        Field_transno,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).transno) };
#[doc(hidden)]
pub struct Field_shareable;
#[cfg(feature = "pg15")]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggInfo, shareable) == 12);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_shareable>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggInfo>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::AggInfo,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::AggInfo> as c::expression::OrdinaryField<
        Field_shareable,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).shareable) };
#[doc(hidden)]
pub struct Field_finalfn_oid;
#[cfg(feature = "pg15")]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggInfo, finalfn_oid) == 16);
#[cfg(feature = "pg15")]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_finalfn_oid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggInfo>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::AggInfo,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::AggInfo> as c::expression::OrdinaryField<
        Field_finalfn_oid,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).finalfn_oid) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AggPath>() == 120);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AggPath>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AggPath {}
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggPath, aggstrategy) == 80);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aggstrategy>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggPath>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__AggStrategy,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::AggPath,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::AggPath> as c::expression::OrdinaryField<
        Field_aggstrategy,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).aggstrategy) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggPath, aggsplit) == 84);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aggsplit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggPath>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__AggSplit,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 84;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::AggPath,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::AggPath> as c::expression::OrdinaryField<
        Field_aggsplit,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).aggsplit) };
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggPath, transitionSpace) == 96);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_transitionSpace>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggPath>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 96;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::AggPath,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::AggPath> as c::expression::OrdinaryField<
        Field_transitionSpace,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).transitionSpace) };
#[doc(hidden)]
pub struct Field_groupClause;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggPath, groupClause) == 104);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_groupClause>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggPath>
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
    crate::__pgrx_c_bindings::AggPath,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::AggPath> as c::expression::OrdinaryField<
        Field_groupClause,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).groupClause) };
#[doc(hidden)]
pub struct Field_qual;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggPath, qual) == 112);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_qual>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggPath>
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
    crate::__pgrx_c_bindings::AggPath,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::AggPath> as c::expression::OrdinaryField<
        Field_qual,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).qual) };
#[doc(hidden)]
pub struct Field_aggs;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg18", feature = "pg19"))]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AggState>() == 600);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AggState>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AggState {}
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggState, aggs) == 224);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aggs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggState>
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
    crate::__pgrx_c_bindings::AggState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::AggState> as c::expression::OrdinaryField<
        Field_aggs,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).aggs) };
#[doc(hidden)]
pub struct Field_numaggs;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggState, numaggs) == 232);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numaggs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 232;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::AggState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::AggState> as c::expression::OrdinaryField<
        Field_numaggs,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numaggs) };
#[doc(hidden)]
pub struct Field_numtrans;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggState, numtrans) == 236);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numtrans>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 236;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::AggState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::AggState> as c::expression::OrdinaryField<
        Field_numtrans,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numtrans) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggState, aggstrategy) == 240);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aggstrategy>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggState>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__AggStrategy,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 240;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::AggState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::AggState> as c::expression::OrdinaryField<
        Field_aggstrategy,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).aggstrategy) };
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggState, aggsplit) == 244);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aggsplit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggState>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__AggSplit,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 244;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::AggState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::AggState> as c::expression::OrdinaryField<
        Field_aggsplit,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).aggsplit) };
#[doc(hidden)]
pub struct Field_numphases;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggState, numphases) == 256);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numphases>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 256;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::AggState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::AggState> as c::expression::OrdinaryField<
        Field_numphases,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numphases) };
#[doc(hidden)]
pub struct Field_current_phase;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggState, current_phase) == 260);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_current_phase>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 260;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::AggState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::AggState> as c::expression::OrdinaryField<
        Field_current_phase,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).current_phase) };
#[doc(hidden)]
pub struct Field_input_done;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggState, input_done) == 328);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_input_done>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 328;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::AggState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::AggState> as c::expression::OrdinaryField<
        Field_input_done,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).input_done) };
#[doc(hidden)]
pub struct Field_agg_done;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggState, agg_done) == 329);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_agg_done>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 329;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::AggState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::AggState> as c::expression::OrdinaryField<
        Field_agg_done,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).agg_done) };
#[doc(hidden)]
pub struct Field_projected_set;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggState, projected_set) == 332);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_projected_set>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 332;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::AggState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::AggState> as c::expression::OrdinaryField<
        Field_projected_set,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).projected_set) };
#[doc(hidden)]
pub struct Field_current_set;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggState, current_set) == 336);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_current_set>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 336;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::AggState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::AggState> as c::expression::OrdinaryField<
        Field_current_set,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).current_set) };
#[doc(hidden)]
pub struct Field_all_grouped_cols;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggState, all_grouped_cols) == 352);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_all_grouped_cols>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 352;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::AggState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::AggState> as c::expression::OrdinaryField<
        Field_all_grouped_cols,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).all_grouped_cols) };
#[doc(hidden)]
pub struct Field_max_colno_needed;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggState, max_colno_needed) == 368);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_max_colno_needed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 368;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::AggState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::AggState> as c::expression::OrdinaryField<
        Field_max_colno_needed,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).max_colno_needed) };
#[doc(hidden)]
pub struct Field_all_cols_needed;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggState, all_cols_needed) == 372);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_all_cols_needed>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 372;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::AggState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::AggState> as c::expression::OrdinaryField<
        Field_all_cols_needed,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).all_cols_needed) };
#[doc(hidden)]
pub struct Field_maxsets;
const _: () = assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggState, maxsets) == 376);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_maxsets>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 376;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::AggState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::AggState> as c::expression::OrdinaryField<
        Field_maxsets,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).maxsets) };
#[doc(hidden)]
pub struct Field_table_filled;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggState, table_filled) == 432);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_table_filled>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 432;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::AggState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::AggState> as c::expression::OrdinaryField<
        Field_table_filled,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).table_filled) };
#[doc(hidden)]
pub struct Field_num_hashes;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggState, num_hashes) == 436);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_num_hashes>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 436;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::AggState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::AggState> as c::expression::OrdinaryField<
        Field_num_hashes,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).num_hashes) };
#[doc(hidden)]
pub struct Field_hash_batches;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggState, hash_batches) == 480);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hash_batches>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggState>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 480;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::AggState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::AggState> as c::expression::OrdinaryField<
        Field_hash_batches,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hash_batches) };
#[doc(hidden)]
pub struct Field_hash_ever_spilled;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggState, hash_ever_spilled) == 488);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hash_ever_spilled>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 488;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::AggState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::AggState> as c::expression::OrdinaryField<
        Field_hash_ever_spilled,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hash_ever_spilled) };
#[doc(hidden)]
pub struct Field_hash_spill_mode;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggState, hash_spill_mode) == 489);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hash_spill_mode>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggState>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 489;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::AggState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::AggState> as c::expression::OrdinaryField<
        Field_hash_spill_mode,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hash_spill_mode) };
#[doc(hidden)]
pub struct Field_hash_mem_limit;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggState, hash_mem_limit) == 496);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hash_mem_limit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggState>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 496;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::AggState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::AggState> as c::expression::OrdinaryField<
        Field_hash_mem_limit,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hash_mem_limit) };
#[doc(hidden)]
pub struct Field_hash_ngroups_limit;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggState, hash_ngroups_limit) == 504);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hash_ngroups_limit>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggState>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 504;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::AggState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::AggState> as c::expression::OrdinaryField<
        Field_hash_ngroups_limit,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hash_ngroups_limit) };
#[doc(hidden)]
pub struct Field_hash_planned_partitions;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AggState, hash_planned_partitions) == 512
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hash_planned_partitions>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 512;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::AggState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::AggState> as c::expression::OrdinaryField<
        Field_hash_planned_partitions,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hash_planned_partitions) };
#[doc(hidden)]
pub struct Field_hash_mem_peak;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggState, hash_mem_peak) == 528);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hash_mem_peak>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggState>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, usize>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 528;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::AggState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::AggState> as c::expression::OrdinaryField<
        Field_hash_mem_peak,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hash_mem_peak) };
#[doc(hidden)]
pub struct Field_hash_ngroups_current;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AggState, hash_ngroups_current) == 536
);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hash_ngroups_current>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggState>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 536;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::AggState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::AggState> as c::expression::OrdinaryField<
        Field_hash_ngroups_current,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hash_ngroups_current) };
#[doc(hidden)]
pub struct Field_hash_disk_used;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggState, hash_disk_used) == 544);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hash_disk_used>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggState>
{
    type Member = c::CUnsignedLong;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 544;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::AggState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::AggState> as c::expression::OrdinaryField<
        Field_hash_disk_used,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hash_disk_used) };
#[doc(hidden)]
pub struct Field_hash_batches_used;
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggState, hash_batches_used) == 552);
#[cfg(any(feature = "pg15", feature = "pg16", feature = "pg17"))]
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_hash_batches_used>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggState>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 552;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<
    crate::__pgrx_c_bindings::AggState,
    <<c::expression::CRecord<crate::__pgrx_c_bindings::AggState> as c::expression::OrdinaryField<
        Field_hash_batches_used,
    >>::Member as c::expression::CType>::Storage,
> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).hash_batches_used) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AggStatePerAggData>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AggStatePerAggData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AggStatePerAggData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggStatePerAggData, transno) == 8);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_transno>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerAggData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggStatePerAggData, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerAggData> as c::expression::OrdinaryField<Field_transno>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).transno) };
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AggStatePerAggData, finalfn_oid) == 12
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_finalfn_oid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerAggData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggStatePerAggData, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerAggData> as c::expression::OrdinaryField<Field_finalfn_oid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).finalfn_oid) };
#[doc(hidden)]
pub struct Field_numFinalArgs;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AggStatePerAggData, numFinalArgs) == 64
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numFinalArgs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerAggData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 64;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggStatePerAggData, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerAggData> as c::expression::OrdinaryField<Field_numFinalArgs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numFinalArgs) };
#[doc(hidden)]
pub struct Field_aggdirectargs;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AggStatePerAggData, aggdirectargs) == 72
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aggdirectargs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerAggData>
{
    type Member = c::expression::CPointer<
        c::expression::CRecord<crate::__pgrx_c_bindings::List>,
        c::expression::ReadWrite,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 72;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggStatePerAggData, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerAggData> as c::expression::OrdinaryField<Field_aggdirectargs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).aggdirectargs) };
#[doc(hidden)]
pub struct Field_resulttypeLen;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AggStatePerAggData, resulttypeLen) == 80
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resulttypeLen>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerAggData>
{
    type Member = c::CShort;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 80;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggStatePerAggData, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerAggData> as c::expression::OrdinaryField<Field_resulttypeLen>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).resulttypeLen) };
#[doc(hidden)]
pub struct Field_resulttypeByVal;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AggStatePerAggData, resulttypeByVal) == 82
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_resulttypeByVal>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerAggData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 82;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggStatePerAggData, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerAggData> as c::expression::OrdinaryField<Field_resulttypeByVal>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).resulttypeByVal) };
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggStatePerAggData, shareable) == 83);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_shareable>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerAggData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 83;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggStatePerAggData, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerAggData> as c::expression::OrdinaryField<Field_shareable>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).shareable) };
#[doc(hidden)]
pub struct Field_transValue;
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AggStatePerGroupData>() == 16);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AggStatePerGroupData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AggStatePerGroupData {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AggStatePerGroupData, transValue) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_transValue>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerGroupData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedLong, crate::__pgrx_c_bindings::Datum>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggStatePerGroupData, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerGroupData> as c::expression::OrdinaryField<Field_transValue>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).transValue) };
#[doc(hidden)]
pub struct Field_transValueIsNull;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AggStatePerGroupData, transValueIsNull) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_transValueIsNull>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerGroupData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggStatePerGroupData, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerGroupData> as c::expression::OrdinaryField<Field_transValueIsNull>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).transValueIsNull) };
#[doc(hidden)]
pub struct Field_noTransValue;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AggStatePerGroupData, noTransValue) == 9
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_noTransValue>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerGroupData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 9;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggStatePerGroupData, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerGroupData> as c::expression::OrdinaryField<Field_noTransValue>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).noTransValue) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AggStatePerHashData>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AggStatePerHashData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AggStatePerHashData {}
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggStatePerHashData, numCols) == 48);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numCols>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerHashData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 48;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggStatePerHashData, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerHashData> as c::expression::OrdinaryField<Field_numCols>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numCols) };
#[doc(hidden)]
pub struct Field_numhashGrpCols;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AggStatePerHashData, numhashGrpCols) == 52
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numhashGrpCols>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerHashData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 52;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggStatePerHashData, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerHashData> as c::expression::OrdinaryField<Field_numhashGrpCols>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numhashGrpCols) };
#[doc(hidden)]
pub struct Field_largestGrpColIdx;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AggStatePerHashData, largestGrpColIdx) == 56
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_largestGrpColIdx>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerHashData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggStatePerHashData, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerHashData> as c::expression::OrdinaryField<Field_largestGrpColIdx>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).largestGrpColIdx) };
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AggStatePerPhaseData>() == 88);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AggStatePerPhaseData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AggStatePerPhaseData {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AggStatePerPhaseData, aggstrategy) == 0
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aggstrategy>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerPhaseData>
{
    type Member = c::expression::CEnumObject<
        crate::__pgrx_c_generated::EnumIdentity_enum__AggStrategy,
        c::CUnsignedInt,
        u32,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 0;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggStatePerPhaseData, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerPhaseData> as c::expression::OrdinaryField<Field_aggstrategy>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).aggstrategy) };
#[doc(hidden)]
pub struct Field_numsets;
const _: () =
    assert!(::core::mem::offset_of!(crate::__pgrx_c_bindings::AggStatePerPhaseData, numsets) == 4);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numsets>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerPhaseData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 4;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggStatePerPhaseData, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerPhaseData> as c::expression::OrdinaryField<Field_numsets>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numsets) };
#[doc(hidden)]
pub struct Field_evaltrans_cache;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AggStatePerPhaseData, evaltrans_cache) == 56
);
const _: () = {
    assert!(::core::mem::size_of::<[[*mut crate::__pgrx_c_bindings::ExprState; 2]; 2]>() == 32);
    assert!(::core::mem::align_of::<[[*mut crate::__pgrx_c_bindings::ExprState; 2]; 2]>() == 8);
};
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_evaltrans_cache>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerPhaseData>
{
    type Member = c::expression::CArray<
        c::expression::CArray<
            c::expression::CPointer<
                c::expression::CRecord<crate::__pgrx_c_bindings::ExprState>,
                c::expression::ReadWrite,
            >,
            2,
        >,
        2,
    >;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 56;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggStatePerPhaseData, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerPhaseData> as c::expression::OrdinaryField<Field_evaltrans_cache>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).evaltrans_cache) };
#[doc(hidden)]
pub struct Field_aggshared;
#[cfg(feature = "pg15")]
const _: () = {
    assert!(::core::mem::size_of::<crate::__pgrx_c_bindings::AggStatePerTransData>() == 360);
    assert!(::core::mem::align_of::<crate::__pgrx_c_bindings::AggStatePerTransData>() == 8);
};
impl c::expression::NativeRecord for crate::__pgrx_c_bindings::AggStatePerTransData {}
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AggStatePerTransData, aggshared) == 8
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aggshared>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerTransData>
{
    type Member = c::CBool;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 8;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggStatePerTransData, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerTransData> as c::expression::OrdinaryField<Field_aggshared>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).aggshared) };
#[doc(hidden)]
pub struct Field_numInputs;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AggStatePerTransData, numInputs) == 12
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numInputs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerTransData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 12;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggStatePerTransData, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerTransData> as c::expression::OrdinaryField<Field_numInputs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numInputs) };
#[doc(hidden)]
pub struct Field_numTransInputs;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AggStatePerTransData, numTransInputs) == 16
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numTransInputs>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerTransData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 16;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggStatePerTransData, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerTransData> as c::expression::OrdinaryField<Field_numTransInputs>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numTransInputs) };
#[doc(hidden)]
pub struct Field_transfn_oid;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AggStatePerTransData, transfn_oid) == 20
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_transfn_oid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerTransData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 20;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggStatePerTransData, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerTransData> as c::expression::OrdinaryField<Field_transfn_oid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).transfn_oid) };
#[doc(hidden)]
pub struct Field_serialfn_oid;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AggStatePerTransData, serialfn_oid) == 24
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_serialfn_oid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerTransData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 24;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggStatePerTransData, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerTransData> as c::expression::OrdinaryField<Field_serialfn_oid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).serialfn_oid) };
#[doc(hidden)]
pub struct Field_deserialfn_oid;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AggStatePerTransData, deserialfn_oid) == 28
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_deserialfn_oid>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerTransData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 28;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggStatePerTransData, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerTransData> as c::expression::OrdinaryField<Field_deserialfn_oid>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).deserialfn_oid) };
#[doc(hidden)]
pub struct Field_aggtranstype;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AggStatePerTransData, aggtranstype) == 32
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aggtranstype>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerTransData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 32;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggStatePerTransData, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerTransData> as c::expression::OrdinaryField<Field_aggtranstype>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).aggtranstype) };
#[doc(hidden)]
pub struct Field_aggCollation;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AggStatePerTransData, aggCollation) == 184
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_aggCollation>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerTransData>
{
    type Member = c::expression::CIntegerStorage<c::CUnsignedInt, crate::__pgrx_c_bindings::Oid>;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 184;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggStatePerTransData, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerTransData> as c::expression::OrdinaryField<Field_aggCollation>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).aggCollation) };
#[doc(hidden)]
pub struct Field_numSortCols;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AggStatePerTransData, numSortCols) == 188
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numSortCols>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerTransData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 188;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggStatePerTransData, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerTransData> as c::expression::OrdinaryField<Field_numSortCols>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numSortCols) };
#[doc(hidden)]
pub struct Field_numDistinctCols;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AggStatePerTransData, numDistinctCols) == 192
);
// SAFETY: Compiler-owned layout assertions and typed raw projection establish the field storage and offset. Write qualification only narrows.
unsafe impl c::expression::OrdinaryField<Field_numDistinctCols>
    for c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerTransData>
{
    type Member = c::CInt;
    type Declared = FieldReadWrite;
    const OFFSET: usize = 192;
}
// SAFETY: This uncalled witness checks the exact Rust field storage without accessing an allocation.
const _: Projection<crate::__pgrx_c_bindings::AggStatePerTransData, <<c::expression::CRecord<crate::__pgrx_c_bindings::AggStatePerTransData> as c::expression::OrdinaryField<Field_numDistinctCols>>::Member as c::expression::CType>::Storage> = |base| unsafe { ::core::ptr::addr_of_mut!((*base).numDistinctCols) };
#[doc(hidden)]
pub struct Field_initValue;
const _: () = assert!(
    ::core::mem::offset_of!(crate::__pgrx_c_bindings::AggStatePerTransData, initValue) == 288
);
